use crate::ast::MarkdownNode;
use crate::inlines::ProcessCtx;
use crate::inlines::link;

fn skip_leading_continuation_ws(
    ProcessCtx {
        id, parser, line, ..
    }: &mut ProcessCtx,
) {
    if !matches!(
        parser.tree[*id].body,
        MarkdownNode::Paragraph | MarkdownNode::Heading(..)
    ) {
        return;
    }
    while matches!(line.peek(), Some(b' ' | b'\t')) {
        line.next_byte();
    }
}

pub(super) fn process(ctx: &mut ProcessCtx) -> bool {
    if let Some((child_idx, MarkdownNode::Text(text))) = ctx
        .parser
        .tree
        .get_last_child(ctx.id)
        .map(|idx| (idx, &mut ctx.parser.tree[idx].body))
    {
        if text.ends_with(' ') {
            let node = if text.ends_with("  ") {
                MarkdownNode::HardBreak
            } else {
                MarkdownNode::SoftBreak
            };
            let trimmed = text.trim_end().to_string();
            let offset = text.len() - trimmed.len();
            *text = trimmed;
            ctx.parser.tree[child_idx].end.column -= offset as u64;
            ctx.parser.append_to(
                ctx.id,
                node,
                (ctx.line.start_location(), ctx.line.end_location()),
            );
            ctx.line.next_byte();
            skip_leading_continuation_ws(ctx);
            return true;
        }
    }

    // OFM: `^block-id` on the next line should annotate current block instead of creating SoftBreak.
    if ctx.parser.options.obsidian_flavored && ctx.line.validate(1, b'^') {
        let snapshot = ctx.line.snapshot();
        ctx.line.next_byte(); // skip '\n' / '\r'
        if link::process_block_id(ctx) {
            return true;
        }
        ctx.line.resume(&snapshot);
    }

    ctx.parser.append_to(
        ctx.id,
        MarkdownNode::SoftBreak,
        (ctx.line.start_location(), ctx.line.end_location()),
    );
    ctx.line.next_byte();
    skip_leading_continuation_ws(ctx);
    true
}

pub(super) fn process_backslash(
    ProcessCtx {
        line, parser, id, ..
    }: &mut ProcessCtx,
) -> bool {
    // 检查 backslash 后面是否是换行符
    if line.validate_with(1, |b| b == b'\n' || b == b'\r') {
        let end_location = line.location_at_byte(line.cursor() + 2);
        parser.append_to(
            *id,
            MarkdownNode::HardBreak,
            (line.start_location(), end_location),
        );
        line.skip(2);
        skip_leading_continuation_ws(&mut ProcessCtx {
            id: *id,
            parser,
            line,
            brackets: None,
            delimiters: None,
        });
        return true;
    }
    // 检查 backslash 后面是否是 ASCII 标点字符（反斜杠转义）
    if let Some(next) = line.get(1) {
        if next.is_ascii_punctuation() {
            let start_loc = line.start_location();
            line.next_byte(); // skip '\'
            let end_loc = line.location_at_byte(line.cursor() + 1);
            let Some(ch) = line.next_byte() else {
                return false;
            };
            parser.append_text_char_to(*id, ch as char, (start_loc, end_loc));
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    #[test]
    fn case_633() {
        let text = r#"foo  
baz"#;
        let ast = Parser::new(text).parse();
        assert_eq!(
            ast.to_html(),
            "<p>foo<br />
baz</p>"
        )
    }

    #[test]
    fn case_634() {
        let text = r#"foo\
baz"#;
        let ast = Parser::new(text).parse();
        assert_eq!(
            ast.to_html(),
            "<p>foo<br />
baz</p>"
        )
    }

    #[test]
    fn case_backslash_escapes_294() {
        let text = r#"\*not emphasized*
\<br/> not a tag
\[not a link](/foo)
\`not code`
1\. not a list
\* not a list
\# not a heading
\[foo]: /url "not a reference""#;
        let ast =
            Parser::new_with_options(text, crate::parser::ParserOptions::default().enabled_gfm())
                .parse();
        assert_eq!(
            ast.to_html(),
            "<p>*not emphasized*\n&lt;br/&gt; not a tag\n[not a link](/foo)\n`not code`\n1. not a list\n* not a list\n# not a heading\n[foo]: /url &quot;not a reference&quot;</p>"
        );
    }
}
