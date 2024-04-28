use crate::ast::{heading, MarkdownNode};
use crate::blocks::{BeforeCtx, BlockMatching, BlockProcessing, BlockStrategy, Line, ProcessCtx};
use crate::tokenizer::Token;
use std::ops::Range;

#[derive(Debug)]
enum State {
    Initial,
    StartHashes(usize),
    Content,
    EndHashes(usize),
    End(usize),
}
impl heading::ATXHeading {
    fn try_match(line: &Line) -> Option<(usize, Range<usize>)> {
        if line.is_indented() {
            return None;
        }
        let mut state = State::Initial;
        let mut hash_count = 1;
        let mut start = 1;
        for (i, item) in line.iter().enumerate() {
            // println!("heading: {state:?} {:?}", item.token);
            match (&state, &item.token) {
                (State::Initial, Token::Crosshatch) => state = State::StartHashes(i),
                (State::StartHashes(_), Token::Crosshatch) => (),
                (State::StartHashes(s), t) if t.is_space_or_tab() => {
                    let count = i - *s;
                    if count > 6 {
                        return None;
                    };
                    hash_count = count;
                    start = i + 1;
                    state = State::End(i + 1);
                }
                (State::Content, t) if t.is_space_or_tab() => state = State::End(i),
                (State::Content, _) => {}
                (State::End(_), Token::Crosshatch) => {
                    state = State::EndHashes(i);
                }
                (State::End(_), t) if t.is_space_or_tab() => {}
                (State::End(_), _) => state = State::Content,
                (State::EndHashes(_), Token::Crosshatch) => {}
                (State::EndHashes(s), t) if t.is_space_or_tab() => state = State::End(*s),
                (State::EndHashes(_), _) => state = State::Content,
                _ => return None,
            }
        }
        let mut end = match state {
            State::EndHashes(s) | State::End(s) => s,
            _ => line.len(),
        };
        // trim start
        for i in start..end {
            if line[i].is_space_or_tab() {
                continue;
            }
            start = i;
            break;
        }
        // trim end
        for i in (start..end).rev() {
            if line[i].is_space_or_tab() {
                continue;
            }
            end = i + 1;
            break;
        }
        let range = Range { start, end };
        // println!(
        //     "hash_count = {hash_count} {:?}  => {:?}",
        //     line,
        //     line.slice(start, end)
        // );
        Some((hash_count, range))
    }
}
impl BlockStrategy for heading::ATXHeading {
    /// AXT headings
    ///
    /// ```text
    ///  # foo
    ///  ## foo
    ///  ### foo
    ///  #### foo
    ///  ##### foo
    ///  ###### foo
    ///  ## foo ## ## #
    /// ```
    fn before(BeforeCtx { line, parser, .. }: BeforeCtx) -> BlockMatching {
        let location = line[0].location;
        line.skip_indent();
        if let Some((hash_count, range)) = Self::try_match(line) {
            parser.close_unmatched_blocks();
            let idx = parser.append_block(
                MarkdownNode::Heading(heading::Heading::ATX(heading::ATXHeading {
                    level: heading::HeadingLevel::try_from(hash_count).unwrap(),
                })),
                location,
            );
            parser.append_inline(idx, line.slice(range.start, range.end));
            line.skip_to_end();
            BlockMatching::MatchedLeaf
        } else {
            BlockMatching::Unmatched
        }
    }
    fn process(_ctx: ProcessCtx) -> BlockProcessing {
        BlockProcessing::Unprocessed
    }
}

impl BlockStrategy for heading::SetextHeading {
    fn before(BeforeCtx { line, parser, .. }: BeforeCtx) -> BlockMatching {
        if !line.is_indented()
            && parser.current_proc().body == MarkdownNode::Paragraph
            && line
                .skip_indent()
                .starts_with_matches(|it| matches!(it, Token::Eq | Token::Hyphen), 1)
        {
            let level = if line[0].token == Token::Eq {
                line.skip_consecutive_tokens(&Token::Eq);
                heading::HeadingLevel::H1
            } else {
                line.skip_consecutive_tokens(&Token::Hyphen);
                heading::HeadingLevel::H2
            };
            if !line.only_space_to_end() {
                return BlockMatching::Unmatched;
            }
            parser.replace_block(
                MarkdownNode::Heading(heading::Heading::SETEXT(heading::SetextHeading { level })),
                line.end_location(),
            );
            return BlockMatching::MatchedLeaf;
        }
        BlockMatching::Unmatched
    }
    fn process(_ctx: ProcessCtx) -> BlockProcessing {
        BlockProcessing::Unprocessed
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{heading, MarkdownNode};
    use crate::parser::Parser;
    use crate::tokenizer::Location;

    #[test]
    fn test_atx_heading() {
        let text = r#"
# foo
## foo
### foo
#### foo
##### foo
###### foo #
####### foo ##
#hashtag
"#
        .trim();
        let ast = Parser::new(text).parse();
        assert_eq!(ast[0].body, MarkdownNode::Document);
        // 为每个标题定义预期的开始和结束位置
        let expected_locations = [
            (Location::new(1, 1), Location::new(1, 6)),
            (Location::new(2, 1), Location::new(2, 7)),
            (Location::new(3, 1), Location::new(3, 8)),
            (Location::new(4, 1), Location::new(4, 9)),
            (Location::new(5, 1), Location::new(5, 10)),
            (Location::new(6, 1), Location::new(6, 13)),
            // 注意：最后一行是段落，不是标题
            (Location::new(7, 1), Location::new(8, 9)),
        ];
        // 检查标题节点
        for (i, &(start, end)) in expected_locations.iter().enumerate().take(6) {
            match &ast[i + 1].body {
                MarkdownNode::Heading(heading::Heading::ATX(atx)) => {
                    assert_eq!(atx.level, heading::HeadingLevel::try_from(i + 1).unwrap());
                }
                _ => panic!("Expected heading, found {:?}", ast[i + 1].body),
            }
            assert_eq!(ast[i + 1].start, start);
            assert_eq!(ast[i + 1].end, end);
            assert_eq!(ast.get_next(i + 1), Some(i + 2));
        }
        // 检查最后一个段落节点
        assert_eq!(ast[7].body, MarkdownNode::Paragraph);
        let last = expected_locations.last().unwrap();
        assert_eq!(ast[7].start, last.0);
        assert_eq!(ast[7].end, last.1);
    }
    #[test]
    fn test_setext_heading() {
        let text = r#"
Foo *bar*
=========
Foo *bar*
---------
Foo *bar
baz*
====
"#
        .trim();
        let ast = Parser::new(text).parse();
        assert_eq!(ast[0].body, MarkdownNode::Document);
        // println!("{ast:?}")
        let expected_locations = [
            (Location::new(1, 1), Location::new(2, 10)),
            (Location::new(3, 1), Location::new(4, 10)),
            // 注意：最后一行是段落，不是标题
            (Location::new(5, 1), Location::new(7, 5)),
        ];
        for i in 1..3 {
            let (start, end) = expected_locations[i - 1];
            assert_eq!(
                ast[i].body,
                MarkdownNode::Heading(heading::Heading::SETEXT(heading::SetextHeading {
                    level: heading::HeadingLevel::try_from(i).unwrap(),
                }))
            );
            assert_eq!(ast[i].start, start);
            assert_eq!(ast[i].end, end);
            assert_eq!(ast.get_next(i), Some(i + 1));
        }
        assert_eq!(
            ast[3].body,
            MarkdownNode::Heading(heading::Heading::SETEXT(heading::SetextHeading {
                level: heading::HeadingLevel::H1,
            }))
        );

        let last = expected_locations.last().unwrap();
        assert_eq!(ast[3].start, last.0);
        assert_eq!(ast[3].end, last.1);
    }

    // #[test]
    // fn test_running() {
    //     let input = "# foo\t#\t";
    //     let output = r#"g"#;
    //     let ast = Parser::new(input).parse();
    //     println!("AST:\n{ast:?}");
    //     assert_eq!(ast.to_html(), output);
    // }
    #[test]
    fn case_62() {
        let input = r#"# foo
## foo
### foo
#### foo
##### foo
###### foo"#;
        let output = r#"<h1>foo</h1>
<h2>foo</h2>
<h3>foo</h3>
<h4>foo</h4>
<h5>foo</h5>
<h6>foo</h6>"#;
        let ast = Parser::new(input).parse();
        // println!("AST:\n{ast:?}")
        assert_eq!(ast.to_html(), output);
    }
    #[test]
    fn case_63() {
        let input = r#"####### foo"#;
        let output = r#"<p>####### foo</p>"#;
        let ast = Parser::new(input).parse();
        // println!("AST:\n{ast:?}")
        assert_eq!(ast.to_html(), output);
    }
    #[test]
    fn case_64() {
        let input = r#"#5 bolt

#hashtag"#;
        let output = r#"<p>#5 bolt</p>
<p>#hashtag</p>"#;
        let ast = Parser::new(input).parse();
        // println!("AST:\n{ast:?}")
        assert_eq!(ast.to_html(), output);
    }
    #[test]
    fn case_65() {
        let input = r#"\## foo"#;
        let output = r#"<p>## foo</p>"#;
        let ast = Parser::new(input).parse();
        println!("AST:\n{ast:?}");
        assert_eq!(ast.to_html(), output);
    }
    #[test]
    fn case_66() {
        let input = r#"# foo *bar* \*baz\*"#;
        let output = r#"<h1>foo <em>bar</em> *baz*</h1>"#;
        let ast = Parser::new(input).parse();
        println!("AST:\n{ast:?}");
        assert_eq!(ast.to_html(), output);
    }
    #[test]
    fn case_67() {
        let input = r#"#                  foo"#;
        let output = r#"<h1>foo</h1>"#;
        let ast = Parser::new(input).parse();
        println!("AST:\n{ast:?}");
        assert_eq!(ast.to_html(), output);
    }
    #[test]
    fn case_68() {
        let input = r#" ### foo
  ## foo
   # foo"#;
        let output = r#"<h3>foo</h3>
<h2>foo</h2>
<h1>foo</h1>"#;
        let ast = Parser::new(input).parse();
        println!("AST:\n{ast:?}");
        assert_eq!(ast.to_html(), output);
    }
    #[test]
    fn case_69() {
        let input = r#"    # foo"#;
        let output = r#"<pre><code># foo
</code></pre>"#;
        let ast = Parser::new(input).parse();
        println!("AST:\n{ast:?}");
        assert_eq!(ast.to_html(), output);
    }
    #[test]
    fn case_70() {
        let input = r#"foo
    # bar"#;
        let output = r#"<p>foo
# bar</p>"#;
        let ast = Parser::new(input).parse();
        println!("AST:\n{ast:?}");
        assert_eq!(ast.to_html(), output);
    }
    #[test]
    fn case_71() {
        let input = r#"## foo ##
  ###   bar    ###"#;
        let output = r#"<h2>foo</h2>
<h3>bar</h3>"#;
        let ast = Parser::new(input).parse();
        println!("AST:\n{ast:?}");
        assert_eq!(ast.to_html(), output);
    }
    #[test]
    fn case_72() {
        let input = r#"# foo ##################################
##### foo ##"#;
        let output = r#"<h1>foo</h1>
<h5>foo</h5>"#;
        let ast = Parser::new(input).parse();
        println!("AST:\n{ast:?}");
        assert_eq!(ast.to_html(), output);
    }
    #[test]
    fn case_73() {
        let input = r#"### foo ###"#;
        let output = r#"<h3>foo</h3>"#;
        let ast = Parser::new(input).parse();
        println!("AST:\n{ast:?}");
        assert_eq!(ast.to_html(), output);
    }
    #[test]
    fn case_74() {
        let input = r#"### foo ### b"#;
        let output = r#"<h3>foo ### b</h3>"#;
        let ast = Parser::new(input).parse();
        println!("AST:\n{ast:?}");
        assert_eq!(ast.to_html(), output);
    }
    #[test]
    fn case_75() {
        let input = r#"# foo#"#;
        let output = r#"<h1>foo#</h1>"#;
        let ast = Parser::new(input).parse();
        println!("AST:\n{ast:?}");
        assert_eq!(ast.to_html(), output);
    }
    #[test]
    fn case_76() {
        let input = r#"### foo \###
## foo #\##
# foo \#"#;
        let output = r#"<h3>foo ###</h3>
<h2>foo ###</h2>
<h1>foo #</h1>"#;
        let ast = Parser::new(input).parse();
        println!("AST:\n{ast:?}");
        assert_eq!(ast.to_html(), output);
    }
    #[test]
    fn case_77() {
        let input = r#"****
## foo
****"#;
        let output = r#"<hr />
<h2>foo</h2>
<hr />"#;
        let ast = Parser::new(input).parse();
        println!("AST:\n{ast:?}");
        assert_eq!(ast.to_html(), output);
    }
    #[test]
    fn case_78() {
        let input = r#"Foo bar
# baz
Bar foo"#;
        let output = r#"<p>Foo bar</p>
<h1>baz</h1>
<p>Bar foo</p>"#;
        let ast = Parser::new(input).parse();
        println!("AST:\n{ast:?}");
        assert_eq!(ast.to_html(), output);
    }
    #[test]
    fn case_79() {
        let input = r#"## 
#
### ###"#;
        let output = r#"<h2></h2>
<h1></h1>
<h3></h3>"#;
        let ast = Parser::new(input).parse();
        println!("AST:\n{ast:?}");
        assert_eq!(ast.to_html(), output);
    }
    #[test]
    fn case_80() {
        let input = r#"Foo *bar*
=========

Foo *bar*
---------"#;
        let output = r#"<h1>Foo <em>bar</em></h1>
<h2>Foo <em>bar</em></h2>"#;
        let ast = Parser::new(input).parse();
        println!("AST:\n{ast:?}");
        assert_eq!(ast.to_html(), output);
    }
    #[test]
    fn case_81() {
        let input = r#"Foo *bar
baz*
===="#;
        let output = r#"<h1>Foo <em>bar
baz</em></h1>"#;
        let ast = Parser::new(input).parse();
        println!("AST:\n{ast:?}");
        assert_eq!(ast.to_html(), output);
    }
    #[test]
    fn case_82() {
        let input = r#"  Foo *bar
baz*	
===="#;
        let output = r#"<h1>Foo <em>bar
baz</em></h1>"#;
        let ast = Parser::new(input).parse();
        println!("AST:\n{ast:?}");
        assert_eq!(ast.to_html(), output);
    }
}
