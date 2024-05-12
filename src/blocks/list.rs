use crate::ast::{list, MarkdownNode};
use crate::blocks::{BeforeCtx, BlockMatching, BlockProcessing, BlockStrategy, ProcessCtx};
use crate::parser::Parser;
use crate::tokenizer::{Token, Whitespace};

impl BlockStrategy for list::ListItem {
    fn before(
        BeforeCtx {
            line,
            parser,
            container,
        }: BeforeCtx,
    ) -> BlockMatching {
        let location = line[0].location;
        // println!(
        //     "A is_indented = {} body {:?}",
        //     line.is_indented(),
        //     parser.tree[parser.tree.get_parent(container)].body
        // );
        if line.is_indented() && !matches!(parser.tree[container].body, MarkdownNode::List(..)) {
            return BlockMatching::Unmatched;
        }
        line.skip_indent();
        let mut cur_list = match line.next() {
            Some(Token::Hyphen) => list::List::Bullet(list::BulletList {
                marker: Token::Hyphen,
                padding: 1,
                marker_offset: line.indent_spaces(),
                tight: true,
            }),
            Some(Token::Plus) => list::List::Bullet(list::BulletList {
                marker: Token::Plus,
                padding: 1,
                marker_offset: line.indent_spaces(),
                tight: true,
            }),
            Some(Token::Asterisk) => list::List::Bullet(list::BulletList {
                marker: Token::Asterisk,
                padding: 1,
                marker_offset: line.indent_spaces(),
                tight: true,
            }),
            Some(Token::Digit(start)) if start.len() < 10 => match line.next() {
                Some(ch @ (Token::RParen | Token::Period))
                    if line.is_end() || line.validate(0, |t: &Token| t.is_space_or_tab()) =>
                {
                    list::List::Ordered(list::OrderedList {
                        start: start.parse::<u64>().unwrap(),
                        delimiter: if ch == Token::RParen { '(' } else { '.' },
                        padding: start.len() + 1,
                        marker_offset: line.indent_spaces(),
                        tight: true,
                    })
                }
                _ => return BlockMatching::Unmatched,
            },
            _ => return BlockMatching::Unmatched,
        };
        let spaces_after_marker = {
            let count = line.starts_count_matches(|it| it.is_space_or_tab());
            line.iter().take(count).fold(0, |a, b| match &b.token {
                Token::Whitespace(ws) => a + ws.len(),
                _ => a,
            })
        };
        // 必需后跟空格
        if spaces_after_marker == 0 && !line.is_end() {
            return BlockMatching::Unmatched;
        }
        // 计算 padding (W + space + rest spaces)
        line.skip_spaces(spaces_after_marker);
        if !(1..5).contains(&spaces_after_marker) || line.is_end() {
            cur_list.set_padding(cur_list.padding() + 1)
        } else {
            cur_list.set_padding(cur_list.padding() + spaces_after_marker)
        }
        let snapshot = line.snapshot();
        if matches!(cur_list, list::List::Bullet(..))
            && spaces_after_marker <= 4
            && line.consume(|it: &Token| it == &Token::LBracket)
        {
            // - [x] task item
            let padding = cur_list.padding() + 4;
            let task_list = match line.next() {
                Some(Token::RBracket) => None,
                Some(Token::Whitespace(Whitespace::Space)) => Some(list::TaskList {
                    task: Some(' '),
                    padding,
                    marker_offset: line.indent_spaces(),
                    tight: true,
                }),
                Some(token @ (Token::Text(str) | Token::Digit(str))) => {
                    if token.len() == 1 {
                        Some(list::TaskList {
                            task: str.chars().next(),
                            padding,
                            marker_offset: line.indent_spaces(),
                            tight: true,
                        })
                    } else {
                        None
                    }
                }
                Some(token) if token.len() == 1 && !token.is_control() => {
                    if let Ok(char) = char::try_from(&token) {
                        Some(list::TaskList {
                            task: Some(char),
                            padding,
                            marker_offset: line.indent_spaces(),
                            tight: true,
                        })
                    } else {
                        None
                    }
                }
                _ => None,
            };
            // 后跟 ] 和 空格
            if task_list.is_some()
                && line.consume(Token::RBracket)
                && line.consume(|it: &Token| it.is_space_or_tab())
            {
                cur_list = list::List::Task(task_list.unwrap())
            } else {
                line.resume(snapshot);
            }
        }
        // 空白列表不能打断段落
        if line.only_space_to_end()
            && matches!(parser.tree[container].body, MarkdownNode::Paragraph)
        {
            return BlockMatching::Unmatched;
        }
        let cur_item = match &cur_list {
            list::List::Ordered(list) => list::OrderedItem { start: list.start }.into(),
            list::List::Task(list) => list::TaskItem { task: list.task }.into(),
            _ => list::BulletItem {}.into(),
        };
        let cur_list_node = MarkdownNode::List(cur_list);
        parser.close_unmatched_blocks();
        if !matches!(parser.current_proc().body, MarkdownNode::List(..))
            || !match_list_node(&cur_list_node, &parser.tree[container].body)
        {
            parser.append_block(cur_list_node, location);
        }
        parser.append_block(MarkdownNode::ListItem(cur_item), location);
        BlockMatching::MatchedContainer
    }

    fn process<'input>(ProcessCtx { line, parser, id }: ProcessCtx) -> BlockProcessing {
        // 尝试提取处理当前节点的 List，如果不是 List 直接返回 Unprocessed
        let list_idx = parser.tree.get_parent(id);
        let list = if let MarkdownNode::List(list) = &parser.tree[list_idx].body {
            list
        } else {
            return BlockProcessing::Unprocessed;
        };
        // println!(
        //     "[A] indent = {}, padding = {}, marker_offset = {} is_blank = {}\n{:?}",
        //     line.indent_spaces(),
        //     list.padding(),
        //     list.marker_offset(),
        //     line.is_blank(),
        //     line
        // );
        return if line.is_blank() {
            // 如果当前容器存在子节点则跳过空白，否则返回 BlockProcessing::Unprocessed
            if parser.tree.get_first_child(list_idx).is_none() {
                return BlockProcessing::Unprocessed;
            }
            line.advance_next_nonspace();
            BlockProcessing::Further
        } else if line.indent_spaces() >= list.padding() + list.marker_offset() {
            line.skip_spaces(list.padding() + list.marker_offset());
            line.re_find_indent();
            BlockProcessing::Further
        } else {
            BlockProcessing::Unprocessed
        };
    }
}

impl BlockStrategy for list::List {
    fn before(_ctx: BeforeCtx) -> BlockMatching {
        BlockMatching::Unmatched
    }

    fn process<'input>(_ctx: ProcessCtx) -> BlockProcessing {
        BlockProcessing::Further
    }
    fn after(id: usize, parser: &mut Parser) {
        let mut tight = match &mut parser.tree[id].body {
            MarkdownNode::List(list) => list.tight(),
            _ => return,
        };
        // 定义一个闭包来检查tight条件
        // 检测 list 下面所有 item 是否行数相差大于 1
        // 检查 item 下面所有 node 是否行数相差大于 1
        let check_tight = |curr, next| -> bool {
            parser.tree[next]
                .start
                .line
                .saturating_sub(parser.tree[curr].end.line)
                <= 1
        };
        let mut item = parser.tree.get_first_child(id);
        while let Some(curr) = item {
            if !tight {
                break;
            }
            let next = parser.tree.get_next(curr);
            if next.is_some() && !check_tight(curr, next.unwrap()) {
                tight = false;
                break;
            }
            let mut sub_item = parser.tree.get_first_child(curr);
            while let Some(curr) = sub_item {
                let next = parser.tree.get_next(curr);
                if next.is_some() && !check_tight(curr, next.unwrap()) {
                    tight = false;
                    break;
                }
                sub_item = next
            }
            item = next;
        }
        if let MarkdownNode::List(list) = &mut parser.tree[id].body {
            list.set_tight(tight);
        }
    }
}

fn match_list_node(a: &MarkdownNode, b: &MarkdownNode) -> bool {
    match (a, b) {
        (MarkdownNode::List(a), MarkdownNode::List(b)) => a.like(b),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{list, MarkdownNode};
    use crate::parser::Parser;
    use crate::tokenizer::Location;

    #[test]
    fn case_1() {
        let text = r#"1.  A paragraph
    with two lines.

        indented code

    > A block quote."#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast[0].body, MarkdownNode::Document);
        assert_eq!(
            ast[1].body,
            MarkdownNode::List(list::List::Ordered(list::OrderedList {
                start: 1,
                delimiter: '.',
                padding: 4,
                marker_offset: 0,
                tight: false,
            }))
        );
        assert_eq!(ast[1].start, Location::new(1, 1));
        assert_eq!(ast[1].end, Location::new(6, 21));
        // println!("{:?}", ast)
    }
    #[test]
    fn case_2() {
        let text = r#"123456789. ok"#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast[0].body, MarkdownNode::Document);
        assert_eq!(
            ast[1].body,
            MarkdownNode::List(list::List::Ordered(list::OrderedList {
                start: 123456789,
                delimiter: '.',
                padding: 11,
                marker_offset: 0,
                tight: true,
            }))
        );
        let text = r#"1234567890. not ok"#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast[0].body, MarkdownNode::Document);
        assert_eq!(ast[1].body, MarkdownNode::Paragraph);
    }
    #[test]
    fn case_3() {
        let text = r#"1. foo
2.
3. bar"#;
        let ast = Parser::new(text).parse();
        println!("{ast:?}");
        assert_eq!(ast[0].body, MarkdownNode::Document);
        assert_eq!(
            ast[1].body,
            MarkdownNode::List(list::List::Ordered(list::OrderedList {
                start: 1,
                delimiter: '.',
                padding: 3,
                marker_offset: 0,
                tight: true
            }))
        );
        assert_eq!(
            ast[2].body,
            MarkdownNode::ListItem(list::OrderedItem { start: 1 }.into())
        );
        assert_eq!(ast[2].start, Location::new(1, 1));
        assert_eq!(ast[2].end, Location::new(1, 7));
        assert_eq!(ast[3].body, MarkdownNode::Paragraph);
        assert_eq!(ast[3].start, Location::new(1, 4));
        assert_eq!(ast[3].end, Location::new(1, 7));
        assert_eq!(
            ast[4].body,
            MarkdownNode::ListItem(list::OrderedItem { start: 2 }.into())
        );
        assert_eq!(ast[4].start, Location::new(2, 1));
        assert_eq!(ast[4].end, Location::new(2, 3));
        assert_eq!(
            ast[5].body,
            MarkdownNode::ListItem(list::OrderedItem { start: 3 }.into())
        );
        assert_eq!(ast[5].start, Location::new(3, 1));
        assert_eq!(ast[5].end, Location::new(3, 7));
        assert_eq!(ast[6].body, MarkdownNode::Paragraph);
        assert_eq!(ast[6].start, Location::new(3, 4));
        assert_eq!(ast[6].end, Location::new(3, 7));
    }

    #[test]
    fn case_4() {
        let text = r#"
- Headings:

    1. Heading 1 - Start a line with `#` followed by a space.

    2. Heading 2 - Start a line with `##` followed by a space.

    3. Heading 3 - Start a line with `###` followed by a space."#
            .trim();
        let ast = Parser::new(text).parse();
        assert_eq!(ast[0].body, MarkdownNode::Document);
        println!("{ast:?}")
    }
    #[test]
    fn case_5() {
        let text = r#"* foo
→bar"#
            .trim();
        let ast = Parser::new(text).parse();
        println!("AST:\n{ast:?}");
        assert_eq!(
            ast.to_html(),
            "<ul>
<li>foo
→bar</li>
</ul>"
        )
    }
}
