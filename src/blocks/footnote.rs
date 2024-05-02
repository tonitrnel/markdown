use crate::ast::{footnote, MarkdownNode};
use crate::blocks::{BeforeCtx, BlockMatching, BlockProcessing, BlockStrategy, ProcessCtx};
use crate::parser::Parser;
use crate::tokenizer::Token;
use crate::utils;

impl BlockStrategy for footnote::Footnote {
    fn before(BeforeCtx { line, parser, .. }: BeforeCtx) -> BlockMatching {
        let location = line.start_location();
        if !line.is_indented()
            && line
                .advance_next_nonspace()
                .starts_with(&Token::LBracket, 1)
        {
            line.next();
            if !line.consume(Token::Caret) {
                return BlockMatching::Unmatched;
            }
            let mut end = 0;
            for (i, item) in line.iter().enumerate() {
                match &item.token {
                    Token::Text(..) | Token::Digit(..) => continue,
                    Token::RBracket | Token::Escaped(']') if line.validate(i + 1, Token::Colon) => {
                        end = i;
                        break;
                    }
                    Token::LBracket | Token::Whitespace(..) => return BlockMatching::Unmatched,
                    _ => continue,
                }
            }
            let label = line.slice(0, end).to_string();
            line.skip(end + 2); // `]` and `:`
            parser.close_unmatched_blocks();
            let idx = parser.append_block(
                MarkdownNode::Footnote(footnote::Footnote {
                    label: utils::percent_encode::encode(&label, true),
                    ref_count: 0,
                }),
                location,
            );
            parser.footnotes.entry(label).or_insert(idx);
            return BlockMatching::MatchedContainer;
        }
        BlockMatching::Unmatched
    }
    fn process(ProcessCtx { line, .. }: ProcessCtx) -> BlockProcessing {
        if line.is_indented() {
            line.skip(line.indent_len());
            line.re_find_indent();
            BlockProcessing::Further
        } else if line.is_blank() {
            BlockProcessing::Further
        } else {
            BlockProcessing::Unprocessed
        }
    }
    fn after(id: usize, parser: &mut Parser) {
        // 将 Footnote 转为游离节点，在使用后插入至 FootnoteList
        parser.tree.unlink(id);
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    #[test]
    fn gfm_case_1() {
        let parser = Parser::new("[^1]: This is the first footnote.");
        let ast = parser.parse();
        assert_eq!(ast.to_html(), "");
    }
    #[test]
    fn gfm_case_2() {
        let parser = Parser::new(
            r#"
[^bignote]: Here's one with multiple paragraphs and code.

    Indent paragraphs to include them in the footnote.

    `{ my code }`

    Add as many paragraphs as you like.

    end"#,
        );
        let ast = parser.parse();
        assert_eq!(ast.to_html(), "");
    }
    #[test]
    fn gfm_case_3() {
        let parser = Parser::new(
            r#"Here's a simple footnote,[^1] and here's a longer one.[^bignote]

[^1]: This is the first footnote.

[^bignote]: Here's one with multiple paragraphs and code.

    Indent paragraphs to include them in the footnote.

    `{ my code }`

    Add as many paragraphs as you like."#,
        );
        let ast = parser.parse();
        assert_eq!(
            ast.to_html(),
            r##"<p>Here's a simple footnote,<a href="#cont-fn-1" id="cont-fn-ref-1">[1]</a> and here's a longer one.<a href="#cont-fn-bignote" id="cont-fn-ref-bignote">[2]</a></p>
<section>
<h2>Footnotes</h2>
<ol>
<li id="cont-fn-1">
<p>This is the first footnote.<a href="#cont-fn-ref-1">↩</a></p>
</li>
<li id="cont-fn-bignote">
<p>Here's one with multiple paragraphs and code.</p>
<p>Indent paragraphs to include them in the footnote.</p>
<p><code>{ my code }</code></p>
<p>Add as many paragraphs as you like.<a href="#cont-fn-ref-bignote">↩</a></p>
</li>
</ol>
</section>"##
        );
    }
    #[test]
    fn gfm_case_4() {
        let parser = Parser::new(
            r#"and here's a longer one.[^bignote] and more.[^bignote] Here's a simple footnote,[^1] 

[^1]: This is the first footnote.
48854 
[^bignote]: Here's one with multiple paragraphs and code.

    Indent paragraphs to include them in the footnote.

    `{ my code }`

    Add as many paragraphs as you like.

    end"#,
        );
        let ast = parser.parse();
        println!("{}", ast.to_html())
    }
}
