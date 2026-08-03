mod support;

use markdown::{
    BlockScanStatus, InlineSelection, MarkdownNode, Parser, ParserOptions, VisitControl,
};
use support::semantic::semantic_digest;

fn ofm() -> ParserOptions {
    ParserOptions::default().enabled_ofm()
}

const SOURCE: &str = "# H1\n\nPara with id. ^p1\n\n> ## Nested H2\n>\n> quoted content ^q1\n\n- item one ^li1\n- item two\n\n## H2 title ^h2id\n\n^standalone\n";

#[test]
fn targets_visited_in_preorder_with_ids() {
    let mut seen: Vec<(bool, Option<String>)> = Vec::new();
    let mut selection = InlineSelection::default();
    let mut phase = Parser::new_with_options(SOURCE, ofm())
        .parse_blocks_with(|_| true, |_| VisitControl::Continue)
        .unwrap()
        .prepare_semantic_targets()
        .unwrap();
    phase.visit_semantic_targets(
        |_| true,
        &mut selection,
        |target, _| {
            seen.push((
                target.heading().is_some(),
                target.block_id().map(str::to_owned),
            ));
            VisitControl::Continue
        },
    );
    assert_eq!(
        seen,
        vec![
            (true, None),                       // # H1
            (false, Some("p1".into())),         // 段落尾缀 id
            (true, None),                       // 引用内嵌套 H2
            (false, Some("q1".into())),         // 引用内段落 id
            (false, Some("li1".into())),        // 列表项段落 id
            (true, Some("h2id".into())),        // Heading + BlockId 单目标
            (false, Some("standalone".into())), // 独立 id 段落
        ],
    );
}

#[test]
fn heading_inline_materializes_lazily_on_ref_text() {
    let source = "## See [docs][d] *now* `x`\n\nBody.\n\n[d]: https://example.com/d\n";
    let mut selection = InlineSelection::default();
    let mut kinds: Vec<String> = Vec::new();
    let mut ref_texts: Vec<String> = Vec::new();
    let mut phase = Parser::new_with_options(source, ofm())
        .parse_blocks_with(|_| true, |_| VisitControl::Continue)
        .unwrap()
        .prepare_semantic_targets()
        .unwrap();
    phase.visit_semantic_targets(
        |target| target.heading().is_some(),
        &mut selection,
        |target, _| {
            // C3：准备期不物化——首个 heading 目标此时应无 Inline 子节点
            assert!(
                target.tree().get_first_child(target.node_id()).is_none(),
                "heading should stay pending before ref_text"
            );
            ref_texts.push(target.ref_text());
            let tree = target.tree();
            let mut child = tree.get_first_child(target.node_id());
            while let Some(id) = child {
                kinds.push(match &tree[id].body {
                    MarkdownNode::Text(_) => "text".to_string(),
                    MarkdownNode::Emphasis => "emphasis".to_string(),
                    MarkdownNode::Code(_) => "code".to_string(),
                    MarkdownNode::Link(link) => format!("link:{}", {
                        match link.as_ref() {
                            // 引用链接的 URL 来自定义表，恒为 Owned
                            markdown::ast::link::Link::Default(d) => match &d.url {
                                markdown::ast::text::TextRef::Owned(url) => url.clone(),
                                markdown::ast::text::TextRef::Source(_) => "<source>".to_string(),
                            },
                            _ => "other".to_string(),
                        }
                    }),
                    other => format!("{other:?}"),
                });
                child = tree.get_next(id);
            }
            VisitControl::Continue
        },
    );
    assert_eq!(ref_texts, ["See docs now x"]);
    assert_eq!(
        kinds,
        [
            "text",
            "link:https://example.com/d",
            "text",
            "emphasis",
            "text",
            "code"
        ],
    );
}

/// C3 等价兜底：会话 `ref_text` 与完整解析该 heading Inline 子树的
/// 纯文本投影逐字节一致（真实引擎物化，全 curated 语料）。
#[test]
fn ref_text_matches_full_parse_projection_on_curated() {
    fn project(doc: &markdown::Document, root: usize, out: &mut String) {
        let mut stack: Vec<usize> = Vec::new();
        let mut node = doc.tree.get_first_child(root);
        while let Some(id) = node {
            if let MarkdownNode::Text(t) = &doc.tree[id].body {
                out.push_str(doc.text(t));
            }
            if let Some(child) = doc.tree.get_first_child(id) {
                stack.push(id);
                node = Some(child);
            } else {
                node = doc.tree.get_next(id);
                while node.is_none() {
                    match stack.pop() {
                        Some(parent) => node = doc.tree.get_next(parent),
                        None => break,
                    }
                }
            }
        }
    }
    let text = std::fs::read_to_string("bench/fixtures/curated/_data.md").unwrap();
    let full = Parser::new_with_options(&text, ofm()).parse().unwrap();
    let mut expected: Vec<String> = Vec::new();
    {
        let mut stack: Vec<usize> = Vec::new();
        let mut node = full.tree.get_first_child(0);
        while let Some(id) = node {
            if matches!(full.tree[id].body, MarkdownNode::Heading(_)) {
                let mut s = String::new();
                project(&full, id, &mut s);
                expected.push(s);
            }
            if let Some(child) = full.tree.get_first_child(id) {
                stack.push(id);
                node = Some(child);
            } else {
                node = full.tree.get_next(id);
                while node.is_none() {
                    match stack.pop() {
                        Some(parent) => node = full.tree.get_next(parent),
                        None => break,
                    }
                }
            }
        }
    }
    let mut actual: Vec<String> = Vec::new();
    let mut selection = InlineSelection::default();
    let mut phase = Parser::new_with_options(&text, ofm())
        .parse_blocks_with(|_| true, |_| VisitControl::Continue)
        .unwrap()
        .prepare_semantic_targets()
        .unwrap();
    phase.visit_semantic_targets(
        |t| t.heading().is_some(),
        &mut selection,
        |t, _| {
            actual.push(t.ref_text());
            VisitControl::Continue
        },
    );
    assert_eq!(actual.len(), expected.len());
    assert_eq!(actual, expected);
}

#[test]
fn prepare_then_finish_equals_plain_parse_on_synthetic_source() {
    let document = Parser::new_with_options(SOURCE, ofm())
        .parse_blocks_with(|_| true, |_| VisitControl::Continue)
        .unwrap()
        .prepare_semantic_targets()
        .unwrap()
        .finish()
        .unwrap();
    let plain = Parser::new_with_options(SOURCE, ofm()).parse().unwrap();
    assert_eq!(semantic_digest(&document), semantic_digest(&plain));
    assert_eq!(document.to_html(), plain.to_html());
}

#[test]
fn prepare_then_finish_keeps_footnote_numbering_with_heading_refs() {
    // Heading 预物化会先处理 [^b]；位置化的脚注最终化必须保持与 parse() 一致
    let source = "Intro[^a].\n\n## Head[^b]\n\nTail[^b] again[^a].\n\n[^a]: A\n[^b]: B\n";
    let document = Parser::new_with_options(source, ofm())
        .parse_blocks_with(|_| true, |_| VisitControl::Continue)
        .unwrap()
        .prepare_semantic_targets()
        .unwrap()
        .finish()
        .unwrap();
    let plain = Parser::new_with_options(source, ofm()).parse().unwrap();
    assert_eq!(document.to_html(), plain.to_html());
    assert_eq!(semantic_digest(&document), semantic_digest(&plain));
    // 编号确实是文档顺序：a=1（正文首现先于 heading 的 b）
    assert!(
        document
            .to_html()
            .contains(r##"id="cont-fn-ref-a">[1]</a>"##)
    );
    assert!(
        document
            .to_html()
            .contains(r##"id="cont-fn-ref-b">[2]</a>"##)
    );
}

#[test]
fn prepare_then_finish_equals_plain_parse_with_inline_footnotes() {
    let source = "P1 ^[first inline].\n\n## H ^[second inline]\n\nP2[^named].\n\n[^named]: N\n";
    let document = Parser::new_with_options(source, ofm())
        .parse_blocks_with(|_| true, |_| VisitControl::Continue)
        .unwrap()
        .prepare_semantic_targets()
        .unwrap()
        .finish()
        .unwrap();
    let plain = Parser::new_with_options(source, ofm()).parse().unwrap();
    assert_eq!(document.to_html(), plain.to_html());
    assert_eq!(semantic_digest(&document), semantic_digest(&plain));
}

#[test]
fn prepare_then_finish_equals_plain_parse_on_curated_corpus() {
    let source = include_str!("../bench/fixtures/curated/_data.md");
    let document = Parser::new_with_options(source, ofm())
        .parse_blocks_with(|_| true, |_| VisitControl::Continue)
        .unwrap()
        .prepare_semantic_targets()
        .unwrap()
        .finish()
        .unwrap();
    let plain = Parser::new_with_options(source, ofm()).parse().unwrap();
    assert_eq!(semantic_digest(&document), semantic_digest(&plain));
    assert_eq!(document.to_html(), plain.to_html());
}

#[test]
fn semantic_stop_only_stops_traversal() {
    let mut selection = InlineSelection::default();
    let mut visited = 0usize;
    let mut phase = Parser::new_with_options(SOURCE, ofm())
        .parse_blocks_with(|_| true, |_| VisitControl::Continue)
        .unwrap()
        .prepare_semantic_targets()
        .unwrap();
    phase.visit_semantic_targets(
        |_| true,
        &mut selection,
        |_, _| {
            visited += 1;
            VisitControl::Stop
        },
    );
    assert_eq!(visited, 1);
    // Stop 不截断树：finish 仍是完整文档
    let document = phase.finish().unwrap();
    let plain = Parser::new_with_options(SOURCE, ofm()).parse().unwrap();
    assert_eq!(semantic_digest(&document), semantic_digest(&plain));
}

#[test]
fn filter_rejection_continues_traversal() {
    let mut selection = InlineSelection::default();
    let mut visited = Vec::new();
    let mut phase = Parser::new_with_options(SOURCE, ofm())
        .parse_blocks_with(|_| true, |_| VisitControl::Continue)
        .unwrap()
        .prepare_semantic_targets()
        .unwrap();
    phase.visit_semantic_targets(
        |target| target.heading().is_some(),
        &mut selection,
        |target, selection| {
            selection.select(target.node_id());
            visited.push(target.node_id());
            VisitControl::Continue
        },
    );
    assert_eq!(visited.len(), 3); // H1、嵌套 H2、H2^h2id
    assert!(!selection.is_empty());
}

#[test]
fn duplicate_block_ids_produce_distinct_targets() {
    let source = "First. ^dup\n\nSecond. ^dup\n";
    let mut selection = InlineSelection::default();
    let mut ids = Vec::new();
    let mut nodes = Vec::new();
    let mut phase = Parser::new_with_options(source, ofm())
        .parse_blocks_with(|_| true, |_| VisitControl::Continue)
        .unwrap()
        .prepare_semantic_targets()
        .unwrap();
    phase.visit_semantic_targets(
        |_| true,
        &mut selection,
        |target, _| {
            ids.push(target.block_id().unwrap().to_owned());
            nodes.push(target.node_id());
            VisitControl::Continue
        },
    );
    assert_eq!(ids, ["dup", "dup"]);
    assert_ne!(nodes[0], nodes[1]);
}

#[test]
fn stopped_block_scan_status_survives_preparation() {
    let phase = Parser::new_with_options(SOURCE, ofm())
        .parse_blocks_with(|_| true, |_| VisitControl::Stop)
        .unwrap()
        .prepare_semantic_targets()
        .unwrap();
    assert_eq!(phase.block_status(), BlockScanStatus::Stopped);
}
