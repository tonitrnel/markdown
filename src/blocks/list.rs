use crate::ast::{list, MarkdownNode};
use crate::blocks::{BlockMatching, BlockProcessing, BlockStrategy, Line};
use crate::parser::Parser;
use crate::tokenizer::{Token, Whitespace};

impl BlockStrategy for list::ListItem {
    fn before<'input>(parser: &mut Parser<'input>, line: &mut Line<'input>) -> BlockMatching {
        let location = line[0].location;
        if line.is_indented() && !matches!(parser.current_container().body, MarkdownNode::List(..))
        {
            return BlockMatching::Unmatched;
        }
        line.skip_indent();
        let mut need_detect_task_list = false;
        let mut cur_list = match line.next() {
            Some(Token::Hyphen) => {
                need_detect_task_list = true;
                list::List::Bullet(list::BulletList {
                    marker: Token::Hyphen,
                })
            }
            Some(Token::Plus) => list::List::Bullet(list::BulletList {
                marker: Token::Plus,
            }),
            Some(Token::Asterisk) => list::List::Bullet(list::BulletList {
                marker: Token::Asterisk,
            }),
            Some(Token::Ordered(ordered, d)) => list::List::Ordered(list::OrderedList {
                start: ordered,
                delimiter: d,
            }),
            _ => return BlockMatching::Unmatched,
        };
        if !line.consume(|it: &Token| it.is_space_or_tab()) {
            return BlockMatching::Unmatched;
        }
        let sn = line.snapshot();
        if need_detect_task_list && line.consume(|it: &Token| it == &Token::LBracket) {
            let task_list = match line.next() {
                Some(Token::Whitespace(Whitespace::Space)) => Some(list::TaskList {
                    checked: false,
                    quested: false,
                }),
                Some(Token::Text("?")) => Some(list::TaskList {
                    checked: false,
                    quested: true,
                }),
                Some(Token::Text(s)) if s.chars().count() == 1 => Some(list::TaskList {
                    checked: true,
                    quested: false,
                }),
                _ => None,
            };
            if !line.consume(Token::RBracket) || task_list.is_none() {
                line.resume(sn);
            } else {
                cur_list = list::List::Task(task_list.unwrap())
            }
        }
        // 空白列表不能打断段落
        if line.only_spaces_to_end()
            && matches!(parser.current_container().body, MarkdownNode::Paragraph)
        {
            return BlockMatching::Unmatched;
        }
        let cur_item = match &cur_list {
            list::List::Ordered(list) => list::ListItem {
                order: Some(list.start),
                checked: None,
                quested: None,
            },
            list::List::Task(list) => list::ListItem {
                order: None,
                checked: Some(list.checked),
                quested: Some(list.quested),
            },
            _ => list::ListItem {
                order: None,
                checked: None,
                quested: None,
            },
        };
        let cur_list_node = MarkdownNode::List(cur_list);
        if !matches!(parser.current_proc().body, MarkdownNode::List(..))
            || match_list_node(&cur_list_node, &parser.current_container().body)
        {
            parser.append_block(cur_list_node, location);
        }
        parser.append_block(MarkdownNode::ListItem(cur_item), location);
        BlockMatching::MatchedContainer
    }

    fn process<'input>(parser: &mut Parser<'input>, line: &mut Line<'input>) -> BlockProcessing {
        BlockProcessing::Unprocessed
    }
}

impl BlockStrategy for list::List {
    fn before<'input>(_parser: &mut Parser<'input>, _line: &mut Line<'input>) -> BlockMatching {
        BlockMatching::Unmatched
    }

    fn process<'input>(_parser: &mut Parser<'input>, _line: &mut Line<'input>) -> BlockProcessing {
        BlockProcessing::Further
    }
    fn after(_id: usize, _parser: &mut Parser) {
        todo!()
    }
}

fn match_list_node(a: &MarkdownNode, b: &MarkdownNode) -> bool {
    match (a, b) {
        (MarkdownNode::List(a), MarkdownNode::List(b)) => a.like(b),
        _ => false,
    }
}
