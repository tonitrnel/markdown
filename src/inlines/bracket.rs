use crate::ast::{self, MarkdownNode};
use crate::inlines::ProcessCtx;
use crate::utils;

#[derive(Clone)]
pub(crate) enum BracketVariant {
    Link,
    Image,
}
/// bracket 工作区条目（P3）：以 `Parser::bracket_store` 中的索引成链
/// （`prev` 单向），取代逐 bracket 的 `Rc<RefCell<_>>` 堆分配；容量随
/// delimiter 工作区同一 base/truncate 协议复用与隔离。
pub(crate) struct Bracket {
    pub(crate) node: usize,
    pub(crate) prev: Option<usize>,
    pub(crate) bracket_after: bool,
    pub(crate) index: usize,
    pub(crate) active: bool,
    pub(crate) variant: BracketVariant,
    pub(crate) image_size: Option<(u32, Option<u32>)>,
}

impl Bracket {
    pub(crate) fn is_image(&self) -> bool {
        matches!(self.variant, BracketVariant::Image)
    }
}

pub(super) fn parse_image_size_suffix(text: &str) -> Option<(usize, (u32, Option<u32>))> {
    let (alt, size) = text.rsplit_once('|')?;
    if size.is_empty() {
        return None;
    }
    let mut parts = size.split('x');
    let width = parts.next()?.parse::<u32>().ok()?;
    let height = match parts.next() {
        Some(value) if !value.is_empty() => Some(value.parse::<u32>().ok()?),
        Some(_) => return None,
        None => None,
    };
    if parts.next().is_some() {
        return None;
    }
    Some((alt.len(), (width, height)))
}

pub(super) fn before(
    ProcessCtx {
        line,
        parser,
        id,
        brackets,
        ..
    }: &mut ProcessCtx,
    is_image: bool,
) -> bool {
    let start = line.cursor();
    // P1b：`[` / `![` 标记保存源码区间，不分配 String
    let (variant, text, locations) = if is_image {
        // '!' + '['
        let start_loc = line.cursor_or_end() as u32;
        line.next_byte(); // '!'
        let end_loc = (line.cursor() + 1) as u32;
        line.next_byte(); // '['
        (
            BracketVariant::Image,
            crate::ast::text::TextRef::Source(crate::ast::text::SourceSpan::new(
                start as u32,
                (start + 2) as u32,
            )),
            (start_loc, end_loc),
        )
    } else {
        // '['
        let start_loc = line.cursor_or_end() as u32;
        let end_loc = (line.cursor() + 1) as u32;
        line.next_byte();
        (
            BracketVariant::Link,
            crate::ast::text::TextRef::Source(crate::ast::text::SourceSpan::new(
                start as u32,
                (start + 1) as u32,
            )),
            (start_loc, end_loc),
        )
    };
    let node = parser.append_to(*id, MarkdownNode::Text(text), locations);
    parser.mark_as_processed(node);
    let store = &mut parser.bracket_store;
    if let Some(prev_idx) = *brackets {
        store[prev_idx].bracket_after = true;
    }
    let idx = store.len();
    store.push(Bracket {
        node,
        prev: *brackets,
        index: start,
        active: true,
        variant,
        bracket_after: false,
        image_size: None,
    });
    *brackets = Some(idx);
    true
}

pub(super) fn process(ctx: &mut ProcessCtx) -> bool {
    let ProcessCtx {
        line,
        parser,
        brackets,
        ..
    } = ctx;
    let opener_idx = match *brackets {
        Some(idx) => idx,
        _ => return false,
    };
    if !parser.bracket_store[opener_idx].active {
        remove_brackets(&parser.bracket_store, brackets);
        return false;
    }
    line.next_byte(); // skip ']'
    let current_span = match line.current_span_mut() {
        Some(span) => span,
        None => return false,
    };
    let is_image = parser.bracket_store[opener_idx].is_image();
    // OFM/GFM 语义：bracket 自身内容是已定义脚注标签时优先解析为脚注引用，
    // 不再把它当作后随 `[...]`/`(...)` 形式的链接文本（`[^a][^b]` 是两个引用，
    // `[^a](t)` 是引用 + 字面括号）。未定义标签保持原有引用链接回退。
    let scanned = if is_image {
        None
    } else {
        let open_index = parser.bracket_store[opener_idx].index;
        own_footnote_label(open_index, current_span, &parser.footnotes)
            .map(|label| super::link::ScannedLink::Footnote { label })
    };
    let scanned = scanned.or_else(|| {
        super::link::scan_link_or_image(
            current_span,
            &parser.bracket_store[opener_idx],
            &parser.link_refs,
            &parser.footnotes,
        )
    });
    return if let Some(scanned) = scanned {
        let opener_inl = parser.bracket_store[opener_idx].node;
        let start_location = parser.tree[opener_inl].span.start;
        // 与旧行为保持一致：image 分支优先（`![^x][^y]` 边缘按图片处理）
        let node = if is_image {
            let (url, title) = match scanned {
                super::link::ScannedLink::Resource { url, title } => (url, title),
                super::link::ScannedLink::Footnote { label } => {
                    (crate::ast::text::TextRef::Owned(label), None)
                }
            };
            let size = parser.bracket_store[opener_idx].image_size.take();
            parser.append_free_node(
                MarkdownNode::Image(Box::new(ast::image::Image { url, title, size })),
                start_location,
            )
        } else if let super::link::ScannedLink::Footnote { label } = &scanned {
            let label = label.clone();
            // index/ref_count 此处为临时值；parse_footnote_list 会按源码位置统一最终化
            let (index, ref_count) = parser
                .footnote_refs
                .get(&label)
                .map(|(a, b)| (*a, b + 1))
                .unwrap_or((parser.footnote_refs.len() + 1, 1));
            parser
                .footnote_refs
                .entry(label.clone())
                .and_modify(|it| it.1 += 1)
                .or_insert((index, ref_count));
            let node = parser.append_free_node(
                MarkdownNode::Link(Box::new(ast::link::Link::Footnote(
                    ast::link::FootnoteLink {
                        footnote_label: utils::percent_encode::encode(&label, true),
                        index,
                        ref_count,
                    },
                ))),
                start_location,
            );
            parser.footnote_ref_nodes.push((node, label));
            node
        } else {
            let super::link::ScannedLink::Resource { url, title } = scanned else {
                unreachable!()
            };
            parser.append_free_node(
                MarkdownNode::Link(Box::new(ast::link::Link::Default(ast::link::DefaultLink {
                    url,
                    title,
                }))),
                start_location,
            )
        };
        let mut temp = parser.tree.get_next(opener_inl);
        while let Some(item) = temp {
            let next = parser.tree.get_next(item);
            parser.tree.unlink(item);
            parser.tree.set_parent(item, node);
            temp = next;
        }
        parser.tree[node].span.end = line.cursor_or_end() as u32;
        parser
            .tree
            .set_parent(node, parser.tree.get_parent(opener_inl));
        remove_brackets(&ctx.parser.bracket_store, &mut ctx.brackets);
        ctx.parser.tree.remove(opener_inl);
        if !is_image {
            let store = &mut ctx.parser.bracket_store;
            let mut cur = ctx.brackets;
            while let Some(idx) = cur {
                if !store[idx].is_image() {
                    store[idx].active = false;
                }
                cur = store[idx].prev;
            }
        }
        true
    } else {
        remove_brackets(&parser.bracket_store, brackets);
        false
    };
}

/// bracket 自身内容（`[` 与已消费的 `]` 之间）为 `^` + 已定义脚注标签时返回该标签。
fn own_footnote_label(
    open_index: usize, // '[' 的绝对字节偏移
    span: &crate::span::Span,
    footnotes: &rustc_hash::FxHashMap<String, usize>,
) -> Option<String> {
    let close_index = span.cursor().checked_sub(1)?; // 刚跳过的 ']' 位置
    let source = span.source_slice();
    let content = source.get(open_index + 1..close_index)?;
    let rest = content.strip_prefix(b"^")?;
    if rest.is_empty()
        || rest
            .iter()
            .any(|b| matches!(b, b'\n' | b'\r' | b'[' | b']' | b'\\'))
    {
        return None;
    }
    let label = std::str::from_utf8(rest).ok()?;
    footnotes.contains_key(label).then(|| label.to_string())
}

pub(crate) fn remove_brackets(store: &[Bracket], slot: &mut Option<usize>) {
    let Some(idx) = *slot else {
        return;
    };
    *slot = store[idx].prev;
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;
    // links
    #[test]
    fn case_482() {
        let text = r#"[link](/uri "title")"#;
        let ast = Parser::new(text).parse();
        assert_eq!(
            ast.to_html(),
            r#"<p><a href="/uri" title="title">link</a></p>"#
        )
    }
    #[test]
    fn case_483() {
        let text = r#"[link](/uri)"#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast.to_html(), r#"<p><a href="/uri">link</a></p>"#)
    }
    #[test]
    fn case_484() {
        let text = r#"[](./target.md)"#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast.to_html(), r#"<p><a href="./target.md"></a></p>"#)
    }
    #[test]
    fn case_487() {
        let text = r#"[]()"#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast.to_html(), r#"<p><a href=""></a></p>"#)
    }
    #[test]
    fn case_488() {
        let text = r#"[link](/my uri)"#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast.to_html(), r#"<p>[link](/my uri)</p>"#)
    }
    #[test]
    fn case_489() {
        let text = r#"[link](</my uri>)"#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast.to_html(), r#"<p><a href="/my%20uri">link</a></p>"#)
    }
    #[test]
    fn case_490() {
        let text = r#"[link](foo
bar)"#;
        let ast = Parser::new(text).parse();
        assert_eq!(
            ast.to_html(),
            r#"<p>[link](foo
bar)</p>"#
        )
    }
    #[test]
    fn case_496() {
        let text = r#"[link](foo(and(bar)))"#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast.to_html(), r#"<p><a href="foo(and(bar))">link</a></p>"#)
    }
    #[test]
    fn case_500() {
        let text = r#"[link](foo\)\:)"#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast.to_html(), r#"<p><a href="foo):">link</a></p>"#)
    }
    #[test]
    fn case_501() {
        let text = r#"[link](#fragment)

[link](https://example.com#fragment)

[link](https://example.com?foo=3#frag)"#;
        let ast = Parser::new(text).parse();
        assert_eq!(
            ast.to_html(),
            r##"<p><a href="#fragment">link</a></p>
<p><a href="https://example.com#fragment">link</a></p>
<p><a href="https://example.com?foo=3#frag">link</a></p>"##
        )
    }
    #[test]
    fn case_502() {
        let text = r#"[link](foo\bar)"#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast.to_html(), r#"<p><a href="foo%5Cbar">link</a></p>"#)
    }
    #[test]
    fn case_503() {
        let text = r#"[link](foo%20b&auml;)"#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast.to_html(), r#"<p><a href="foo%20b%C3%A4">link</a></p>"#)
    }
    #[test]
    fn case_504() {
        let text = r#"[link]("title")"#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast.to_html(), r#"<p><a href="%22title%22">link</a></p>"#)
    }

    // images
    #[test]
    fn case_572() {
        let text = r#"![foo](/url "title")"#;
        let ast = Parser::new(text).parse();
        assert_eq!(
            ast.to_html(),
            r#"<p><img src="/url" alt="foo" title="title" /></p>"#
        )
    }
    #[test]
    fn case_574() {
        let text = r#"![foo ![bar](/url)](/url2)"#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast.to_html(), r#"<p><img src="/url2" alt="foo bar" /></p>"#)
    }
    #[test]
    fn case_575() {
        let text = r#"![foo [bar](/url)](/url2)"#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast.to_html(), r#"<p><img src="/url2" alt="foo bar" /></p>"#)
    }
    #[test]
    fn case_579() {
        let input = r#"My ![foo bar](/path/to/train.jpg  "title"   )"#;
        let output = r#"<p>My <img src="/path/to/train.jpg" alt="foo bar" title="title" /></p>"#;
        let ast = Parser::new(input).parse();
        println!("AST:\n{ast:?}");
        assert_eq!(ast.to_html(), output);
    }
}
