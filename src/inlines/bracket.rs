use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use crate::ast::{self, MarkdownNode};
use crate::inlines::{DelimiterChain, ProcessCtx};
use crate::utils;

#[derive(Clone)]
pub(super) enum BracketVariant {
    Link,
    Image,
}
#[derive(Clone)]
pub(super) struct Bracket<'input> {
    pub(super) node: usize,
    pub(super) prev: Option<BracketChain<'input>>,
    pub(super) prev_delimiter: Option<DelimiterChain<'input>>,
    pub(super) bracket_after: bool,
    pub(super) index: usize,
    pub(super) active: bool,
    pub(super) variant: BracketVariant,
}
#[derive(Clone)]
pub(super) struct BracketChain<'input>(Rc<RefCell<Bracket<'input>>>);

impl<'a, 'input> BracketChain<'input> {
    pub(super) fn new(bracket: Bracket<'input>) -> Self {
        Self(Rc::new(RefCell::new(bracket)))
    }
    pub(super) fn borrow(&'a self) -> Ref<'a, Bracket<'input>> {
        self.0.borrow()
    }
    pub(super) fn borrow_mut(&'a self) -> RefMut<'a, Bracket<'input>> {
        self.0.borrow_mut()
    }
    pub(super) fn is_image(&self) -> bool {
        matches!(self.borrow().variant, BracketVariant::Image)
    }
    // pub(super) fn is_link(&self) -> bool {
    //     matches!(self.borrow().variant, BracketVariant::Link)
    // }
}

pub(super) fn before(
    ProcessCtx {
        line,
        parser,
        id,
        brackets,
        delimiters,
        ..
    }: &mut ProcessCtx,
    is_image: bool,
) -> bool {
    let start = line.start_offset;
    let (variant, text, locations) = if is_image {
        let (first, last) = (
            line.next_with_location().unwrap(),
            line.next_with_location().unwrap(),
        );
        (
            BracketVariant::Image,
            format!("{first}{last}"),
            (first.start_location(), last.end_location()),
        )
    } else {
        let item = line.next_with_location().unwrap();
        (
            BracketVariant::Link,
            item.to_string(),
            (item.start_location(), item.end_location()),
        )
    };
    let node = parser.append_to(*id, MarkdownNode::Text(text), locations);
    parser.mark_as_processed(node);
    if let Some(brackets) = brackets.as_ref() {
        brackets.borrow_mut().bracket_after = true;
    }
    *brackets = Some(BracketChain::new(Bracket {
        node,
        prev: brackets.clone(),
        prev_delimiter: delimiters.clone(),
        index: start,
        active: true,
        variant,
        bracket_after: false,
    }));
    true
}

pub(super) fn process(ctx: &mut ProcessCtx) -> bool {
    let ProcessCtx {
        line,
        parser,
        brackets,
        ..
    } = ctx;
    // println!("AST: \n{:?}", parser.tree);
    let opener = match brackets.as_ref() {
        Some(b) => b,
        _ => return false,
    };
    if !opener.borrow().active {
        remove_brackets(brackets);
        return false;
    }
    line.next();
    return if let Some((url, title, is_footnote_link)) =
        super::link::scan_link_or_image(line, opener, &parser.link_refs, &parser.footnotes)
    {
        let is_image = opener.is_image();
        let opener_inl = opener.borrow().node;
        let start_location = parser.tree[opener_inl].start;
        let node = if is_image {
            parser.append_free_node(
                MarkdownNode::Image(ast::image::Image { url, title }),
                start_location,
            )
        } else if is_footnote_link {
            let (index, ref_count) = parser
                .footnote_refs
                .get(&url)
                .map(|(a, b)| (*a, b + 1))
                .unwrap_or((parser.footnote_refs.len() + 1, 1));
            parser
                .footnote_refs
                .entry(url.clone())
                .and_modify(|it| it.1 += 1)
                .or_insert((index, ref_count));
            parser.append_free_node(
                MarkdownNode::Link(ast::link::Link::Footnote(ast::link::FootnoteLink {
                    footnote_label: utils::percent_encode::encode(url, true),
                    index,
                    ref_count,
                })),
                start_location,
            )
        } else {
            parser.append_free_node(
                MarkdownNode::Link(ast::link::Link::Default(ast::link::DefaultLink {
                    url,
                    title,
                })),
                start_location,
            )
        };
        let mut temp = parser.tree.get_next(opener_inl);
        // println!("opener_inl = {opener_inl}, next = {temp:?}")
        while let Some(item) = temp {
            let next = parser.tree.get_next(item);
            parser.tree.unlink(item);
            // println!(
            //     "将 {:?}#{item} 插入到 {:?}#{node} ",
            //     parser.tree[item], parser.tree[node]
            // );
            parser.tree.set_parent(item, node);
            temp = next;
        }
        parser.tree[node].end = line.start_location();
        parser
            .tree
            .set_parent(node, parser.tree.get_parent(opener_inl));
        let prev_delimiter_pos = opener
            .borrow()
            .prev_delimiter
            .as_ref()
            .map(|it| it.borrow().position);
        if let Some(pos) = prev_delimiter_pos {
            super::delimiter::process(ctx, pos);
        }
        remove_brackets(&mut ctx.brackets);
        ctx.parser.tree.remove(opener_inl);
        // 链接不能包含连接
        if !is_image {
            let mut opener = ctx.brackets.clone();
            while let Some(bc) = opener.as_ref() {
                if !bc.is_image() {
                    bc.borrow_mut().active = false;
                }
                let cloned_previous = bc.borrow().prev.clone();
                opener = cloned_previous;
            }
        }
        true
    } else {
        remove_brackets(brackets);
        false
    };
}

pub fn remove_brackets(bracket_chain: &mut Option<BracketChain>) {
    let bracket = match bracket_chain.as_ref() {
        Some(b) => b,
        _ => return,
    };
    let cloned_previous = bracket.borrow().prev.clone();
    *bracket_chain = cloned_previous;
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;
    // links
    #[test]
    fn case_482() {
        let text = r#"[link](/uri "title")"#;
        let ast = Parser::new(text).parse();
        // println!("{ast:?}")
        assert_eq!(
            ast.to_html(),
            r#"<p><a href="/uri" title="title">link</a></p>"#
        )
    }
    #[test]
    fn case_483() {
        let text = r#"[link](/uri)"#;
        let ast = Parser::new(text).parse();
        // println!("{ast:?}")
        assert_eq!(ast.to_html(), r#"<p><a href="/uri">link</a></p>"#)
    }
    #[test]
    fn case_484() {
        let text = r#"[](./target.md)"#;
        let ast = Parser::new(text).parse();
        // println!("{ast:?}")
        assert_eq!(ast.to_html(), r#"<p><a href="./target.md"></a></p>"#)
    }
    #[test]
    fn case_487() {
        let text = r#"[]()"#;
        let ast = Parser::new(text).parse();
        // println!("{ast:?}")
        assert_eq!(ast.to_html(), r#"<p><a href=""></a></p>"#)
    }
    #[test]
    fn case_488() {
        let text = r#"[link](/my uri)"#;
        let ast = Parser::new(text).parse();
        // println!("{ast:?}")
        assert_eq!(ast.to_html(), r#"<p>[link](/my uri)</p>"#)
    }
    #[test]
    fn case_489() {
        let text = r#"[link](</my uri>)"#;
        let ast = Parser::new(text).parse();
        // println!("{ast:?}")
        assert_eq!(ast.to_html(), r#"<p><a href="/my%20uri">link</a></p>"#)
    }
    #[test]
    fn case_490() {
        let text = r#"[link](foo
bar)"#;
        let ast = Parser::new(text).parse();
        // println!("{ast:?}")
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
        // println!("{ast:?}")
        assert_eq!(ast.to_html(), r#"<p><a href="foo(and(bar))">link</a></p>"#)
    }
    #[test]
    fn case_500() {
        let text = r#"[link](foo\)\:)"#;
        let ast = Parser::new(text).parse();
        // println!("{ast:?}")
        assert_eq!(ast.to_html(), r#"<p><a href="foo):">link</a></p>"#)
    }
    #[test]
    fn case_501() {
        let text = r#"[link](#fragment)

[link](https://example.com#fragment)

[link](https://example.com?foo=3#frag)"#;
        let ast = Parser::new(text).parse();
        // println!("{ast:?}")
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
        // println!("{ast:?}")
        assert_eq!(ast.to_html(), r#"<p><a href="foo%5Cbar">link</a></p>"#)
    }
    #[test]
    fn case_503() {
        let text = r#"[link](foo%20b&auml;)"#;
        let ast = Parser::new(text).parse();
        // println!("{ast:?}")
        assert_eq!(ast.to_html(), r#"<p><a href="foo%20b%C3%A4">link</a></p>"#)
    }
    #[test]
    fn case_504() {
        let text = r#"[link]("title")"#;
        let ast = Parser::new(text).parse();
        // println!("{ast:?}")
        assert_eq!(ast.to_html(), r#"<p><a href="%22title%22">link</a></p>"#)
    }

    // images
    #[test]
    fn case_572() {
        let text = r#"![foo](/url "title")"#;
        let ast = Parser::new(text).parse();
        // println!("{ast:?}")
        assert_eq!(
            ast.to_html(),
            r#"<p><img src="/url" alt="foo" title="title" /></p>"#
        )
    }
    #[test]
    fn case_574() {
        let text = r#"![foo ![bar](/url)](/url2)"#;
        let ast = Parser::new(text).parse();
        // println!("{ast:?}")
        assert_eq!(ast.to_html(), r#"<p><img src="/url2" alt="foo bar" /></p>"#)
    }
    #[test]
    fn case_575() {
        let text = r#"![foo [bar](/url)](/url2)"#;
        let ast = Parser::new(text).parse();
        // println!("{ast:?}")
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
