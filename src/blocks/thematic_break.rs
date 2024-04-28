use crate::ast::{thematic_break, MarkdownNode};
use crate::blocks::{BeforeCtx, BlockMatching, BlockProcessing, BlockStrategy, ProcessCtx};
use crate::tokenizer::Token;

impl BlockStrategy for thematic_break::ThematicBreak {
    fn before(BeforeCtx { line, parser, .. }: BeforeCtx) -> BlockMatching {
        if line.is_indented() {
            return BlockMatching::Unmatched;
        }
        let location = line.start_location();
        let marker = match line.skip_indent().next() {
            Some(Token::Hyphen) => Token::Hyphen,
            Some(Token::Underscore) => Token::Underscore,
            Some(Token::Asterisk) => Token::Asterisk,
            _ => return BlockMatching::Unmatched,
        };
        let mut len = 1;
        while let Some(next) = line.next() {
            if next == marker {
                len += 1;
                continue;
            } else if next.is_space_or_tab() {
                continue;
            }
            return BlockMatching::Unmatched;
        }
        if len < 3 {
            return BlockMatching::Unmatched;
        }
        parser.close_unmatched_blocks();
        parser.append_block(MarkdownNode::ThematicBreak, location);
        BlockMatching::MatchedLeaf
    }

    fn process(_ctx: ProcessCtx) -> BlockProcessing {
        BlockProcessing::Unprocessed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;

    #[test]
    fn it_works() {
        let text = r#"
***
---
___
 **  * ** * ** * **
"#
        .trim();
        let ast = Parser::new(text).parse();
        assert_eq!(ast[0].body, MarkdownNode::Document);
        assert_eq!(ast[1].body, MarkdownNode::ThematicBreak);
        assert_eq!(ast[2].body, MarkdownNode::ThematicBreak);
        assert_eq!(ast[3].body, MarkdownNode::ThematicBreak);
        assert_eq!(ast[4].body, MarkdownNode::ThematicBreak);
        assert_eq!(ast.get_next(1), Some(2));
        assert_eq!(ast.get_next(2), Some(3));
        assert_eq!(ast.get_next(3), Some(4));
        assert_eq!(ast.get_next(4), None);
    }
    #[test]
    fn case_43() {
        let input = r#"***
---
___"#;
        let output = r#"<hr />
<hr />
<hr />"#;
        let ast = Parser::new(input).parse();
        // println!("AST:\n{ast:?}")
        assert_eq!(ast.to_html(), output);
    }
    #[test]
    fn case_44() {
        let input = r#"+++"#;
        let output = r#"<p>+++</p>"#;
        let ast = Parser::new(input).parse();
        // println!("AST:\n{ast:?}")
        assert_eq!(ast.to_html(), output);
    }
    #[test]
    fn case_45() {
        let input = r#"==="#;
        let output = r#"<p>===</p>"#;
        let ast = Parser::new(input).parse();
        // println!("AST:\n{ast:?}")
        assert_eq!(ast.to_html(), output);
    }
    #[test]
    fn case_46() {
        let input = r#"--
**
__"#;
        let output = r#"<p>--
**
__</p>"#;
        let ast = Parser::new(input).parse();
        // println!("AST:\n{ast:?}")
        assert_eq!(ast.to_html(), output);
    }
    #[test]
    fn case_47() {
        let input = r#" ***
  ***
   ***"#;
        let output = r#"<hr />
<hr />
<hr />"#;
        let ast = Parser::new(input).parse();
        // println!("AST:\n{ast:?}")
        assert_eq!(ast.to_html(), output);
    }
    #[test]
    fn case_48() {
        let input = r#"    ***"#;
        let output = r#"<pre><code>***
</code></pre>"#;
        let ast = Parser::new(input).parse();
        // println!("AST:\n{ast:?}")
        assert_eq!(ast.to_html(), output);
    }
    #[test]
    fn case_49() {
        let input = r#"Foo
    ***"#;
        let output = r#"<p>Foo
***</p>"#;
        let ast = Parser::new(input).parse();
        // println!("AST:\n{ast:?}")
        assert_eq!(ast.to_html(), output);
    }
    #[test]
    fn case_50() {
        let input = r#"_____________________________________"#;
        let output = r#"<hr />"#;
        let ast = Parser::new(input).parse();
        // println!("AST:\n{ast:?}")
        assert_eq!(ast.to_html(), output);
    }
    #[test]
    fn case_51() {
        let input = r#" - - -"#;
        let output = r#"<hr />"#;
        let ast = Parser::new(input).parse();
        // println!("AST:\n{ast:?}")
        assert_eq!(ast.to_html(), output);
    }
    #[test]
    fn case_52() {
        let input = r#" **  * ** * ** * **"#;
        let output = r#"<hr />"#;
        let ast = Parser::new(input).parse();
        // println!("AST:\n{ast:?}")
        assert_eq!(ast.to_html(), output);
    }
    #[test]
    fn case_53() {
        let input = r#"-     -      -      -"#;
        let output = r#"<hr />"#;
        let ast = Parser::new(input).parse();
        // println!("AST:\n{ast:?}")
        assert_eq!(ast.to_html(), output);
    }
    #[test]
    fn case_54() {
        let input = r#"- - - -"#;
        let output = r#"<hr />"#;
        let ast = Parser::new(input).parse();
        // println!("AST:\n{ast:?}")
        assert_eq!(ast.to_html(), output);
    }
    #[test]
    fn case_55() {
        let input = r#"_ _ _ _ a

a------

---a---"#;
        let output = r#"<p>_ _ _ _ a</p>
<p>a------</p>
<p>---a---</p>"#;
        let ast = Parser::new(input).parse();
        // println!("AST:\n{ast:?}")
        assert_eq!(ast.to_html(), output);
    }
    #[test]
    fn case_56() {
        let input = r#" *-*"#;
        let output = r#"<p><em>-</em></p>"#;
        let ast = Parser::new(input).parse();
        // println!("AST:\n{ast:?}")
        assert_eq!(ast.to_html(), output);
    }
    #[test]
    fn case_57() {
        let input = r#"- foo
***
- bar"#;
        let output = r#"<ul>
<li>foo</li>
</ul>
<hr />
<ul>
<li>bar</li>
</ul>"#;
        let ast = Parser::new(input).parse();
        // println!("AST:\n{ast:?}")
        assert_eq!(ast.to_html(), output);
    }
    #[test]
    fn case_58() {
        let input = r#"Foo
***
bar"#;
        let output = r#"<p>Foo</p>
<hr />
<p>bar</p>"#;
        let ast = Parser::new(input).parse();
        // println!("AST:\n{ast:?}")
        assert_eq!(ast.to_html(), output);
    }
    #[test]
    fn case_59() {
        let input = r#"Foo
---
bar"#;
        let output = r#"<h2>Foo</h2>
<p>bar</p>"#;
        let ast = Parser::new(input).parse();
        // println!("AST:\n{ast:?}")
        assert_eq!(ast.to_html(), output);
    }
    #[test]
    fn case_60() {
        let input = r#"* Foo
* * *
* Bar"#;
        let output = r#"<ul>
<li>Foo</li>
</ul>
<hr />
<ul>
<li>Bar</li>
</ul>"#;
        let ast = Parser::new(input).parse();
        // println!("AST:\n{ast:?}")
        assert_eq!(ast.to_html(), output);
    }
    #[test]
    fn case_61() {
        let input = r#"- Foo
- * * *"#;
        let output = r#"<ul>
<li>Foo</li>
<li>
<hr />
</li>
</ul>"#;
        let ast = Parser::new(input).parse();
        println!("AST:\n{ast:?}");
        assert_eq!(ast.to_html(), output);
    }
}
