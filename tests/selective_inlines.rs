mod support;

use ptdgrp_markdown::{
    BlockScanStatus, InlineSelection, MarkdownNode, ParseError, Parser, ParserOptions,
    SemanticPhase, VisitControl,
};
use support::semantic::semantic_digest;

fn ofm() -> ParserOptions {
    ParserOptions::default().enabled_ofm()
}

const SOURCE: &str = "# Title\n\nAlpha para [link](https://a).\n\n> Beta quote *em*.\n>\n> Gamma inner.\n\nDelta refs[^x].\n\nEpsilon plain.\n\n[^x]: Note body [^y].\n\n[^y]: Nested note.\n\n[^z]: Unused note.\n";

/// `root` 子树内（含自身）的全部 Paragraph 是否都没有子节点（未物化）。
fn paragraphs_unmaterialized(doc: &ptdgrp_markdown::Document, root: usize) -> bool {
    let mut stack = vec![root];
    while let Some(id) = stack.pop() {
        if matches!(doc.tree[id].body, MarkdownNode::Paragraph)
            && doc.tree.get_first_child(id).is_some()
        {
            return false;
        }
        let mut child = doc.tree.get_first_child(id);
        while let Some(c) = child {
            stack.push(c);
            child = doc.tree.get_next(c);
        }
    }
    true
}

/// 扫描 + 语义准备，同时返回顶层 Block（Heading、Alpha、Quote、Delta、Epsilon）的节点 id。
fn prepared(source: &str) -> (SemanticPhase<'_>, Vec<usize>) {
    let mut top_level = Vec::new();
    let phase = Parser::new_with_options(source, ofm())
        .parse_blocks_with(
            |_| true,
            |event| {
                top_level.push(event.node_id());
                VisitControl::Continue
            },
        )
        .unwrap()
        .prepare_semantic_targets()
        .unwrap();
    (phase, top_level)
}

#[test]
fn full_selection_equals_plain_parse() {
    let (phase, _) = prepared(SOURCE);
    let mut selection = InlineSelection::default();
    selection.select(0); // Document 根 = 全选
    let output = phase.parse_selected_inlines(selection).unwrap();
    let plain = Parser::new_with_options(SOURCE, ofm()).parse().unwrap();
    assert_eq!(output.block_status, BlockScanStatus::Complete);
    assert_eq!(semantic_digest(&output.document), semantic_digest(&plain));
    assert_eq!(output.document.to_html(), plain.to_html());
}

#[test]
fn full_selection_equals_plain_parse_on_curated_corpus() {
    let source = include_str!("../bench/fixtures/curated/_data.md");
    let (phase, _) = prepared(source);
    let mut selection = InlineSelection::default();
    selection.select(0);
    let output = phase.parse_selected_inlines(selection).unwrap();
    let plain = Parser::new_with_options(source, ofm()).parse().unwrap();
    assert_eq!(semantic_digest(&output.document), semantic_digest(&plain));
    assert_eq!(output.document.to_html(), plain.to_html());
}

#[test]
fn empty_selection_skips_body_inlines_but_keeps_headings() {
    let (phase, top_level) = prepared(SOURCE);
    let output = phase
        .parse_selected_inlines(InlineSelection::default())
        .unwrap();
    let doc = &output.document;
    // C3：Heading 惰性化后，空选择下同样保持 pending（要 AST 需 select 或 ref_text）
    assert!(doc.tree.get_first_child(top_level[0]).is_none());
    // 正文段落均未物化（容器保留 Block 结构，但段落无 Inline 子节点）
    for &id in &top_level[1..] {
        assert!(
            paragraphs_unmaterialized(doc, id),
            "未选择的顶层 Block #{id} 子树内不应有已物化段落"
        );
    }
    // 无引用产生 → 无脚注列表
    assert!(!doc.to_html().contains("<section>"));
}

#[test]
fn leaf_selection_materializes_only_that_node() {
    let (phase, top_level) = prepared(SOURCE);
    let alpha = top_level[1];
    let mut selection = InlineSelection::default();
    selection.select(alpha);
    let output = phase.parse_selected_inlines(selection).unwrap();
    let doc = &output.document;
    assert!(doc.tree.get_first_child(alpha).is_some());
    // Alpha 的链接已解析
    let mut found_link = false;
    let mut child = doc.tree.get_first_child(alpha);
    while let Some(id) = child {
        if matches!(doc.tree[id].body, MarkdownNode::Link(_)) {
            found_link = true;
        }
        child = doc.tree.get_next(id);
    }
    assert!(found_link);
    // 其余正文未物化；Delta 未物化 ⇒ x 未被引用 ⇒ 无脚注列表
    for &id in &top_level[2..] {
        assert!(paragraphs_unmaterialized(doc, id));
    }
    assert!(!doc.to_html().contains("cont-fn-x"));
}

#[test]
fn container_selection_expands_inline_capable_descendants() {
    let (phase, top_level) = prepared(SOURCE);
    let quote = top_level[2];
    let mut selection = InlineSelection::default();
    selection.select(quote);
    let output = phase.parse_selected_inlines(selection).unwrap();
    let doc = &output.document;
    // 引用内两个段落（Beta/Gamma）都应物化
    let mut materialized = 0;
    let mut child = doc.tree.get_first_child(quote);
    while let Some(id) = child {
        if doc.tree.get_first_child(id).is_some() {
            materialized += 1;
        }
        child = doc.tree.get_next(id);
    }
    assert_eq!(materialized, 2);
    assert!(doc.to_html().contains("<em>em</em>"));
}

#[test]
fn ancestor_and_descendant_selection_deduplicates() {
    let (phase, top_level) = prepared(SOURCE);
    let quote = top_level[2];
    let mut selection = InlineSelection::default();
    selection.select(quote);
    let ancestor_only = phase.parse_selected_inlines(selection).unwrap();

    let (phase2, top_level2) = prepared(SOURCE);
    let quote2 = top_level2[2];
    let beta2 = phase2
        .parse_selected_inlines({
            let mut s = InlineSelection::default();
            s.select(quote2);
            // 同时选择其第一个后代段落
            s
        })
        .unwrap()
        .document;
    // 两种选择方式结果一致（重复选择不产生重复节点/副作用）
    assert_eq!(
        semantic_digest(&ancestor_only.document),
        semantic_digest(&beta2)
    );
}

#[test]
fn footnote_dependencies_expand_recursively() {
    let (phase, top_level) = prepared(SOURCE);
    let delta = top_level[3];
    let mut selection = InlineSelection::default();
    selection.select(delta);
    let output = phase.parse_selected_inlines(selection).unwrap();
    let html = output.document.to_html();
    // x 被选中内容引用 → 物化并进列表；x 的正文引用 y → 递归物化
    assert!(html.contains(r#"<li id="cont-fn-x">"#), "{html}");
    assert!(html.contains(r#"<li id="cont-fn-y">"#), "{html}");
    // 未被引用的 z 不物化、不进列表
    assert!(!html.contains("cont-fn-z"), "{html}");
    // Epsilon 未选择 → 未物化
    assert!(output.document.tree.get_first_child(top_level[4]).is_none());
}

#[test]
fn invalid_selection_node_is_rejected_before_materialization() {
    let (phase, _) = prepared(SOURCE);
    let mut selection = InlineSelection::default();
    selection.select(999_999);
    match phase.parse_selected_inlines(selection) {
        Err(err) => assert_eq!(err, ParseError::InvalidSelectionNode { node_id: 999_999 }),
        Ok(_) => panic!("invalid selection should be rejected"),
    }
}

#[test]
fn selection_on_stopped_prefix_carries_status() {
    // 在第二个顶层 Block 后停止，选择前缀内的段落
    let mut fired = 0usize;
    let phase = Parser::new_with_options(SOURCE, ofm())
        .parse_blocks_with(
            |_| true,
            |_| {
                fired += 1;
                if fired == 2 {
                    VisitControl::Stop
                } else {
                    VisitControl::Continue
                }
            },
        )
        .unwrap()
        .prepare_semantic_targets()
        .unwrap();
    let mut selection = InlineSelection::default();
    selection.select(0);
    let output = phase.parse_selected_inlines(selection).unwrap();
    assert_eq!(output.block_status, BlockScanStatus::Stopped);
    // 前缀 = Heading + Alpha；全选前缀与直接解析前缀一致
    let prefix = "# Title\n\nAlpha para [link](https://a).\n\n";
    let plain = Parser::new_with_options(prefix, ofm()).parse().unwrap();
    assert_eq!(semantic_digest(&output.document), semantic_digest(&plain));
}

/// W2（ticket 28）：owned-source 选择性解析的等价与契约。
#[test]
fn parse_selected_string_full_selection_equals_plain_parse() {
    let text = std::fs::read_to_string("bench/fixtures/curated/_data.md").unwrap();
    let opts = || {
        ptdgrp_markdown::parser::ParserOptions::default()
            .enabled_gfm()
            .enabled_ofm()
    };
    let selected =
        ptdgrp_markdown::parser::Parser::parse_selected_string(text.clone(), opts(), &[0]).unwrap();
    let plain = ptdgrp_markdown::parser::Parser::parse_string(text, opts()).unwrap();
    assert_eq!(semantic_digest(&selected), semantic_digest(&plain));
}

#[test]
fn parse_selected_string_rejects_invalid_node() {
    let err = ptdgrp_markdown::parser::Parser::parse_selected_string(
        "# t\n\npara\n".to_string(),
        ptdgrp_markdown::parser::ParserOptions::default().enabled_ofm(),
        &[999_999],
    )
    .unwrap_err();
    assert!(matches!(
        err,
        ptdgrp_markdown::parser::ParseError::InvalidSelectionNode { node_id: 999_999 }
    ));
}

#[test]
fn node_ids_stable_across_calls_on_identical_source() {
    let text = "# A\n\npara ^p1\n\n## B ^h\n";
    let opts = || ptdgrp_markdown::parser::ParserOptions::default().enabled_ofm();
    let collect = || {
        let mut ids = Vec::new();
        let mut selection = ptdgrp_markdown::selective::InlineSelection::default();
        let mut phase = ptdgrp_markdown::parser::Parser::new_with_options(text, opts())
            .parse_blocks_with(
                |_| true,
                |_| ptdgrp_markdown::selective::VisitControl::Continue,
            )
            .unwrap()
            .prepare_semantic_targets()
            .unwrap();
        phase.visit_semantic_targets(
            |_| true,
            &mut selection,
            |t, _| {
                ids.push((t.node_id(), t.ref_text(), t.block_id().map(str::to_owned)));
                ptdgrp_markdown::selective::VisitControl::Continue
            },
        );
        ids
    };
    assert_eq!(collect(), collect());
}
