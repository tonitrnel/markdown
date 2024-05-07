use crate::ast::MarkdownNode;
use crate::inlines::ProcessCtx;
use crate::line::Line;
use crate::tokenizer::{Token, Whitespace};
use std::cell::{Ref, RefCell, RefMut};
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

#[derive(Clone)]
pub(super) struct Delimiter<'input> {
    pub(super) token: Token<'input>,
    pub(super) can_open: bool,
    pub(super) can_close: bool,
    pub(super) length: usize,
    pub(super) prev: Option<DelimiterChain<'input>>,
    pub(super) next: Option<DelimiterChain<'input>>,
    pub(super) position: usize,
    pub(super) node: usize,
}
impl PartialEq for Delimiter<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
    }
}

#[derive(Clone, PartialEq)]
pub(super) struct DelimiterChain<'input>(Rc<RefCell<Delimiter<'input>>>);
impl<'a, 'input> DelimiterChain<'input> {
    pub(super) fn new(delimiter: Delimiter<'input>) -> Self {
        Self(Rc::new(RefCell::new(delimiter)))
    }
    pub(super) fn borrow(&'a self) -> Ref<'a, Delimiter<'input>> {
        self.0.borrow()
    }
    pub(super) fn borrow_mut(&'a self) -> RefMut<'a, Delimiter<'input>> {
        self.0.borrow_mut()
    }
}

impl Debug for DelimiterChain<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut count = 0;
        {
            let cur = self.borrow();
            writeln!(
                f,
                "  {count}. [{}]({},{})@{}#{}",
                cur.token, cur.can_open, cur.can_close, cur.length, cur.node
            )?;
        }
        let mut prev = self.borrow().prev.clone();
        while let Some(prev_delimiter) = prev {
            count += 1;
            {
                let prev = prev_delimiter.borrow();
                writeln!(
                    f,
                    "  {count}. [{}]({},{})@{}#{}",
                    prev.token, prev.can_open, prev.can_close, prev.length, prev.node
                )?;
            }
            let cloned = prev_delimiter.borrow().prev.clone();
            prev = cloned;
        }
        Ok(())
    }
}

pub(super) fn scan_delimiters<'input>(
    line: &mut Line<'input>,
) -> (Token<'input>, usize, bool, bool) {
    let mut length = 0;
    let start = line.start_offset;
    let snapshot = line.snapshot();
    let initial_token = line.next().unwrap();
    if matches!(initial_token, Token::SingleQuote | Token::DoubleQuote) {
        length += 1;
    } else {
        let rep = line.starts_count(&initial_token);
        length = 1 + rep;
        line.skip(rep);
    }
    let before_token = {
        let default = Token::Whitespace(Whitespace::NewLine("\n"));
        if start == 0 {
            default
        } else {
            line.get_raw(start - 1).cloned().unwrap_or(default)
        }
    };
    let after_token = line
        .next()
        .unwrap_or(Token::Whitespace(Whitespace::NewLine("\n")));
    let after_is_whitespace = after_token.is_anything_space();
    let after_is_punctuation = !after_is_whitespace && after_token.is_punctuation();
    let before_is_whitespace = before_token.is_anything_space();
    let before_is_punctuation = !before_is_whitespace && before_token.is_punctuation();
    let left_flanking = {
        !after_is_whitespace
            && (!after_is_punctuation || before_is_whitespace || before_is_punctuation)
    };
    let right_flanking = {
        length > 0
            && !before_is_whitespace
            && (!before_is_punctuation || after_is_whitespace || after_is_punctuation)
    };
    let (left, right) = match initial_token {
        Token::Underscore | Token::Tilde | Token::Eq => (
            left_flanking && (!right_flanking || before_is_punctuation),
            right_flanking && (!left_flanking || after_is_punctuation),
        ),
        Token::SingleQuote | Token::DoubleQuote => {
            (left_flanking && !right_flanking, right_flanking)
        }
        _ => (left_flanking, right_flanking),
    };
    line.resume(snapshot);
    (initial_token, length, left, right)
}
pub(super) fn before(
    ProcessCtx {
        line,
        parser,
        id,
        delimiters,
        ..
    }: &mut ProcessCtx,
    enabled_gfm_strikethrough: bool,
    enabled_ofm_highlight: bool,
) -> bool {
    let scan_result = scan_delimiters(line);
    let start = line.start_offset;
    let (text, locations) = {
        match scan_result.0 {
            Token::SingleQuote => (
                "\u{2019}".to_string(),
                (line.start_location(), line.end_location()),
            ),
            Token::DoubleQuote => (
                "\u{201C}".to_string(),
                (line.start_location(), line.end_location()),
            ),
            _ => {
                line.skip(scan_result.1);
                let end = line.start_offset;
                let line = line.slice_raw(start, end);
                (
                    line.to_string(),
                    (line.start_location(), line.last_token_end_location()),
                )
            }
        }
    };
    let node = parser.append_to(*id, MarkdownNode::Text(text), locations);
    parser.mark_as_processed(node);
    // println!("开始预处理 {scan_result:?}")
    if (scan_result.2 || scan_result.3)
        && (parser.options.smart_punctuation || !matches!(scan_result.0, Token::SingleQuote | Token::DoubleQuote))
        // strikethrough 只允许单个或两个 `~` 符号
        && (!enabled_gfm_strikethrough || scan_result.1 == 1 || scan_result.1 == 2)
        // highlight 只允许两个 `=` 符号
        && (!enabled_ofm_highlight || scan_result.1 == 2)
    {
        // println!(
        //     "写入 delimiter ({}, {})#{node} parent = {id}",
        //     scan_result.2, scan_result.3
        // );
        *delimiters = Some(DelimiterChain::new(Delimiter {
            token: scan_result.0,
            can_open: scan_result.2,
            can_close: scan_result.3,
            length: scan_result.1,
            prev: delimiters.clone(),
            next: None,
            position: start,
            node,
        }));
        if let Some(delimiters) = delimiters {
            let cloned = delimiters.clone();
            if let Some(previous) = &delimiters.borrow().prev {
                previous.borrow_mut().next = Some(cloned);
            }
        }
    }
    true
}
pub(super) fn process(
    ProcessCtx {
        parser, delimiters, ..
    }: &mut ProcessCtx,
    stack_bottom: usize,
) {
    // println!("AST: \n{:?}", parser.tree);
    // if let Some(dc) = delimiters.as_ref() {
    //     println!("Delimiter chain: \n{dc:?}");
    // }
    let mut openers_bottom = [stack_bottom; 21];
    let mut candidate = delimiters.clone();
    let mut closer = None;
    // 找到第一个相关分隔符。
    while let Some(candidate_delimiter) = candidate
        .as_ref()
        .filter(|it| it.borrow().position >= stack_bottom)
    {
        closer = Some(candidate_delimiter.clone());
        let cloned_previous = candidate_delimiter.borrow().prev.clone();
        candidate = cloned_previous;
    }
    // 现在向前推进，寻找接近者，并处理每个
    while let Some(closer_delimiter) = closer.as_ref() {
        if closer_delimiter.borrow().can_close {
            let (mut opener, openers_bottom_index) = {
                let closer_delimiter = closer_delimiter.borrow();
                let openers_bottom_index = match closer_delimiter.token {
                    Token::DoubleQuote => 0,
                    Token::SingleQuote => 1,
                    Token::Underscore => {
                        2 + (if closer_delimiter.can_open { 3 } else { 0 })
                            + (closer_delimiter.length % 3)
                    }
                    Token::Asterisk => {
                        8 + (if closer_delimiter.can_open { 3 } else { 0 })
                            + (closer_delimiter.length % 3)
                    }
                    // Strikethrough
                    Token::Tilde => {
                        14 + if closer_delimiter.can_open { 2 } else { 0 } + closer_delimiter.length
                    }
                    // Highlight
                    Token::Eq => 19 + if closer_delimiter.can_open { 1 } else { 0 },
                    _ => panic!("Invalid Token {}", closer_delimiter.token),
                };
                (closer_delimiter.prev.clone(), openers_bottom_index)
            };
            // 现在向后看第一个匹配的 opener：
            let mut opener_found = false;
            {
                let closer_delimiter = closer_delimiter.borrow();
                while let Some(opener_delimiter) = opener
                    .as_ref()
                    .filter(|it| it.borrow().position >= openers_bottom[openers_bottom_index])
                {
                    {
                        let opener_delimiter = opener_delimiter.borrow();
                        // 长度为 2 的 closer 无法与长度为 1 的 opener 匹配，或者长度为 1 的 opener 无法与长度为 2 的 closer 匹配
                        let odd_match = (closer_delimiter.can_open || opener_delimiter.can_close)
                            && closer_delimiter.length % 3 != 0
                            && (opener_delimiter.length + closer_delimiter.length) % 3 == 0;
                        if opener_delimiter.can_open
                            && opener_delimiter.token == closer_delimiter.token
                            && !odd_match
                        {
                            opener_found = true;
                            break;
                        }
                    }
                    {
                        let cloned_previous = opener_delimiter.borrow().prev.clone();
                        opener = cloned_previous;
                    }
                }
            }
            // println!("⌈---\nopener = {opener:?} \ncloser = {closer:?} \nopener_found = {opener_found}\n⌊---");
            let mut old_closer = closer.clone();
            let closer_token = closer_delimiter.borrow().token;
            match closer_token {
                Token::Asterisk | Token::Underscore | Token::Tilde | Token::Eq => {
                    if let Some(opener_delimiter) = opener.as_ref().filter(|_| opener_found) {
                        let opener_inl = opener_delimiter.borrow().node;
                        let closer_inl = closer_delimiter.borrow().node;
                        let mut opener_char_nums =
                            if let MarkdownNode::Text(t) = &parser.tree[opener_inl].body {
                                t.len()
                            } else {
                                0
                            };
                        let mut closer_char_nums =
                            if let MarkdownNode::Text(t) = &parser.tree[closer_inl].body {
                                t.len()
                            } else {
                                0
                            };
                        // 计算出实际使用的分隔符数量。
                        let used_delimiter_nums = if closer_char_nums >= 2 && opener_char_nums >= 2
                        {
                            2
                        } else {
                            1
                        };
                        if let MarkdownNode::Text(text) = &mut parser.tree[opener_inl].body {
                            *text = text[0..text.len() - used_delimiter_nums].to_string();
                            opener_char_nums = text.len();
                            parser.tree[opener_inl].end.column -= used_delimiter_nums as u64;
                        }
                        if let MarkdownNode::Text(text) = &mut parser.tree[closer_inl].body {
                            *text = text[0..text.len() - used_delimiter_nums].to_string();
                            closer_char_nums = text.len();
                            parser.tree[closer_inl].end.column -= used_delimiter_nums as u64;
                        }
                        let start_location = parser.tree[opener_inl].end;
                        let node = match closer_token {
                            Token::Asterisk | Token::Underscore => {
                                if used_delimiter_nums == 1 {
                                    parser.append_free_node(MarkdownNode::Emphasis, start_location)
                                } else {
                                    parser.append_free_node(MarkdownNode::Strong, start_location)
                                }
                            }
                            Token::Tilde => {
                                parser.append_free_node(MarkdownNode::Strikethrough, start_location)
                            }
                            Token::Eq => {
                                parser.append_free_node(MarkdownNode::Highlighting, start_location)
                            }
                            _ => panic!("Invalid Token {}", closer_token),
                        };
                        parser.tree[node].end = {
                            let mut loc = parser.tree[closer_inl].end;
                            loc.column += used_delimiter_nums as u64;
                            loc
                        };
                        let mut temp = parser.tree.get_next(opener_inl);
                        // 将 opener_inl 和 closer_inl 之间的节点全部压入 node 形成 opener_inl, node, closer_inl 平行的结构
                        while let Some(item) = temp.filter(|it| it != &closer_inl) {
                            let next = parser.tree.get_next(item);
                            parser.tree.unlink(item);
                            // println!(
                            //     "将 {:?}#{item} 插入到 {:?}#{node} ",
                            //     parser.tree[item], parser.tree[node]
                            // );
                            parser.tree.set_parent(item, node);
                            temp = next;
                        }
                        // println!("喵喵喵 opener_inl = {opener_inl} closer_inl = {closer_inl} node = {node}");

                        // parser
                        //     .tree
                        //     .print_link_info("A", parser.tree.get_parent(opener_inl));
                        parser
                            .tree
                            .set_parent(node, parser.tree.get_parent(opener_inl));
                        // parser
                        //     .tree
                        //     .print_link_info("B", parser.tree.get_parent(opener_inl));
                        assert_eq!(
                            parser.tree.get_parent(opener_inl),
                            parser.tree.get_parent(closer_inl),
                            "Unexpected error: opener #{opener_inl} and closer #{closer_inl} do not have the same parent."
                        );
                        parser.tree.set_next(opener_inl, node);
                        parser.tree.set_prev(closer_inl, node);
                        // parser
                        //     .tree
                        //     .print_link_info("C", parser.tree.get_parent(opener_inl));

                        // open_inl 和 closer_inl 之间的 delimiter 已经被作为 Text 压入 node 了，
                        // 因此如果存在则需要删除防止出现不匹配的情况
                        if opener_delimiter.borrow().next.as_ref() != Some(closer_delimiter) {
                            opener_delimiter.borrow_mut().next = closer.clone();
                            closer_delimiter.borrow_mut().prev = opener.clone();
                        }
                        if opener_char_nums == 0 {
                            parser.tree.remove(opener_inl);
                            remove_delimiter(&mut opener)
                        }
                        if closer_char_nums == 0 {
                            parser.tree.remove(closer_inl);
                            let cloned_next = closer_delimiter.borrow().next.clone();
                            remove_delimiter(&mut closer);
                            closer = cloned_next
                        }
                    } else {
                        let cloned_next = closer_delimiter.borrow().next.clone();
                        closer = cloned_next;
                    }
                }
                Token::SingleQuote => {
                    let node = &mut parser.tree[closer_delimiter.borrow().node];
                    node.body = if opener_found {
                        MarkdownNode::Text("\u{2018}".into())
                    } else {
                        MarkdownNode::Text("\u{2019}".into())
                    };
                    let cloned_next = closer_delimiter.borrow().next.clone();
                    closer = cloned_next;
                }
                Token::DoubleQuote => {
                    let node = &mut parser.tree[closer_delimiter.borrow().node];
                    node.body = if opener_found {
                        MarkdownNode::Text("\u{201C}".into())
                    } else {
                        MarkdownNode::Text("\u{201D}".into())
                    };
                    let cloned_next = closer_delimiter.borrow().next.clone();
                    closer = cloned_next;
                }
                _ => (),
            };
            if let Some(old_closer_delimiter) = old_closer.as_ref().filter(|_| !opener_found) {
                // 为今后搜索 openers 设定下限：
                openers_bottom[openers_bottom_index] = old_closer_delimiter.borrow().position;
                if !old_closer_delimiter.borrow().can_open {
                    // 如果没有匹配的开始，就可以删除一个没有对应 opener 的 closer：
                    remove_delimiter(&mut old_closer)
                }
            }
        } else {
            let cloned_next = closer_delimiter.borrow().next.clone();
            closer = cloned_next;
        }
    }
    // 释放列表中的所有分隔符，直到栈底：
    while delimiters
        .as_ref()
        .filter(|it| it.borrow().position >= stack_bottom)
        .is_some()
    {
        remove_delimiter(delimiters)
    }
}
fn remove_delimiter(delimiter_chain: &mut Option<DelimiterChain>) {
    let delimiter = match delimiter_chain.as_ref() {
        Some(d) => d,
        None => return,
    };
    // println!("移除 delimiter #{}", delimiter.borrow().node);
    // try change previous pointer
    if let Some(previous) = delimiter.borrow().prev.as_ref() {
        previous.borrow_mut().next = delimiter.borrow().next.clone();
    }
    // try change next pointer
    if let Some(next) = delimiter.borrow().next.as_ref() {
        next.borrow_mut().prev = delimiter.borrow().prev.clone();
        return;
    }
    // top of stack
    let cloned_previous = delimiter.borrow().prev.clone();
    *delimiter_chain = cloned_previous
}

#[cfg(test)]
mod tests {
    use crate::parser::{Parser, ParserOptions};

    #[test]
    fn case_350() {
        let text = r#"*foo bar*"#;
        let ast = Parser::new(text).parse();
        // println!("{ast:?}")
        assert_eq!(ast.to_html(), "<p><em>foo bar</em></p>")
    }
    #[test]
    fn case_351() {
        let text = r#"a * foo bar*"#;
        let ast = Parser::new(text).parse();
        // println!("{ast:?}")
        assert_eq!(ast.to_html(), "<p>a * foo bar*</p>")
    }
    #[test]
    fn case_357() {
        let text = r#"_foo bar_"#;
        let ast = Parser::new(text).parse();
        // println!("{ast:?}")
        assert_eq!(ast.to_html(), "<p><em>foo bar</em></p>")
    }
    #[test]
    fn case_378() {
        let text = r#"**foo bar**"#;
        let ast = Parser::new(text).parse();
        // println!("{ast:?}")
        assert_eq!(ast.to_html(), "<p><strong>foo bar</strong></p>")
    }
    #[test]
    fn case_409() {
        let text = r#"*foo *bar**"#;
        let ast = Parser::new(text).parse();
        // println!("{ast:?}")
        assert_eq!(ast.to_html(), "<p><em>foo <em>bar</em></em></p>")
    }
    #[test]
    fn case_411() {
        let text = r#"*foo**bar**baz*"#;
        let ast = Parser::new(text).parse();
        // println!("{ast:?}")
        assert_eq!(ast.to_html(), "<p><em>foo<strong>bar</strong>baz</em></p>")
    }
    #[test]
    fn case_412() {
        let text = r#"*foo**bar*"#;
        let ast = Parser::new(text).parse();
        // println!("{ast:?}")
        assert_eq!(ast.to_html(), "<p><em>foo**bar</em></p>")
    }
    #[test]
    fn case_413() {
        let text = r#"***foo** bar*"#;
        let ast = Parser::new(text).parse();
        // println!("{ast:?}")
        assert_eq!(ast.to_html(), "<p><em><strong>foo</strong> bar</em></p>")
    }
    #[test]
    fn case_416() {
        let text = r#"foo***bar***baz"#;
        let ast = Parser::new(text).parse();
        // println!("{ast:?}")
        assert_eq!(ast.to_html(), "<p>foo<em><strong>bar</strong></em>baz</p>")
    }
    #[test]
    fn case_417() {
        let text = r#"foo******bar*********baz"#;
        let ast = Parser::new(text).parse();
        // println!("{ast:?}")
        assert_eq!(
            ast.to_html(),
            "<p>foo<strong><strong><strong>bar</strong></strong></strong>***baz</p>"
        )
    }
    #[test]
    fn case_418() {
        let text = r#"*foo **bar *baz* bim** bop*"#;
        let ast = Parser::new(text).parse();
        // println!("{ast:?}")
        assert_eq!(
            ast.to_html(),
            "<p><em>foo <strong>bar <em>baz</em> bim</strong> bop</em></p>"
        )
    }
    #[test]
    fn case_420() {
        let text = r#"** is not an empty emphasis"#;
        let ast = Parser::new(text).parse();
        // println!("{ast:?}")
        assert_eq!(ast.to_html(), "<p>** is not an empty emphasis</p>")
    }
    #[test]
    fn case_425() {
        let text = r#"__foo __bar__ baz__"#;
        let ast = Parser::new(text).parse();
        // println!("{ast:?}")
        assert_eq!(
            ast.to_html(),
            "<p><strong>foo <strong>bar</strong> baz</strong></p>"
        )
    }
    #[test]
    fn case_442() {
        let text = r#"**foo*"#;
        let ast = Parser::new(text).parse();
        // println!("{ast:?}")
        assert_eq!(ast.to_html(), "<p>*<em>foo</em></p>")
    }
    #[test]
    fn case_443() {
        let text = r#"*foo**"#;
        let ast = Parser::new(text).parse();
        // println!("{ast:?}")
        assert_eq!(ast.to_html(), "<p><em>foo</em>*</p>")
    }
    #[test]
    fn case_444() {
        let text = r#"***foo**"#;
        let ast = Parser::new(text).parse();
        // println!("{ast:?}")
        assert_eq!(ast.to_html(), "<p>*<strong>foo</strong></p>")
    }
    #[test]
    fn case_445() {
        let text = r#"****foo*"#;
        let ast = Parser::new(text).parse();
        // println!("{ast:?}")
        assert_eq!(ast.to_html(), "<p>***<em>foo</em></p>")
    }
    #[test]
    fn case_449() {
        let text = r#"foo _\__"#;
        let ast = Parser::new(text).parse();
        // println!("{ast:?}")
        assert_eq!(ast.to_html(), "<p>foo <em>_</em></p>")
    }

    #[test]
    fn gfm_case_491() {
        let text = r#"~~Hi~~ Hello, ~there~ world!"#;
        let ast = Parser::new_with_options(text, ParserOptions::default().enabled_gfm()).parse();
        // println!("{ast:?}")
        assert_eq!(
            ast.to_html(),
            "<p><del>Hi</del> Hello, <del>there</del> world!</p>"
        )
    }
    #[test]
    fn gfm_case_492() {
        let text = r#"This ~~has a

new paragraph~~."#;
        let ast = Parser::new_with_options(text, ParserOptions::default().enabled_gfm()).parse();
        // println!("{ast:?}")
        assert_eq!(
            ast.to_html(),
            r#"<p>This ~~has a</p>
<p>new paragraph~~.</p>"#
        )
    }
    #[test]
    fn gfm_case_493() {
        let text = r#"This will ~~~not~~~ strike."#;
        let ast = Parser::new_with_options(text, ParserOptions::default().enabled_gfm()).parse();
        // println!("{ast:?}")
        assert_eq!(ast.to_html(), "<p>This will ~~~not~~~ strike.</p>")
    }

    #[test]
    fn ofm_case_1() {
        let text = r#"==Highlighted text=="#;
        let ast = Parser::new_with_options(text, ParserOptions::default().enabled_ofm()).parse();
        // println!("{ast:?}")
        assert_eq!(ast.to_html(), "<p><mark>Highlighted text</mark></p>")
    }

    #[test]
    fn test_crisscross() {
        let text = r#"**bold _italic** ending_"#;
        let ast = Parser::new(text).parse();
        println!("{ast:?}");
        assert_eq!(
            ast.to_html(),
            "<p><strong>bold _italic</strong> ending_</p>"
        );
    }
}
