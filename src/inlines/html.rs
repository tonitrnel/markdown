use crate::ast::{MarkdownNode, html};
use crate::blocks::html::scan_html_type;
use crate::inlines::ProcessCtx;

fn scan_processing_instruction_len(ctx: &ProcessCtx) -> Option<usize> {
    if !ctx.line.validate(0, b'<') || !ctx.line.validate(1, b'?') {
        return None;
    }
    let mut i = 2usize;
    while let Some(b) = ctx.line.get(i) {
        if b == b'?' && ctx.line.get(i + 1) == Some(b'>') {
            return Some(i + 2);
        }
        i += 1;
    }
    None
}

fn scan_cdata_len(ctx: &ProcessCtx) -> Option<usize> {
    if !ctx.line.validate(0, b'<')
        || !ctx.line.validate(1, b'!')
        || !ctx.line.validate(2, b'[')
        || !ctx.line.validate(3, b'C')
        || !ctx.line.validate(4, b'D')
        || !ctx.line.validate(5, b'A')
        || !ctx.line.validate(6, b'T')
        || !ctx.line.validate(7, b'A')
        || !ctx.line.validate(8, b'[')
    {
        return None;
    }
    let mut i = 9usize;
    while let Some(b) = ctx.line.get(i) {
        if b == b']' && ctx.line.get(i + 1) == Some(b']') && ctx.line.get(i + 2) == Some(b'>') {
            return Some(i + 3);
        }
        i += 1;
    }
    None
}

fn scan_html_comment_len(ctx: &ProcessCtx) -> Option<usize> {
    if !ctx.line.validate(0, b'<')
        || !ctx.line.validate(1, b'!')
        || !ctx.line.validate(2, b'-')
        || !ctx.line.validate(3, b'-')
    {
        return None;
    }
    if !ctx.parser.options.github_flavored {
        if ctx.line.get(4) == Some(b'>') {
            return Some(5);
        }
        if ctx.line.get(4) == Some(b'-') && ctx.line.get(5) == Some(b'>') {
            return Some(6);
        }
        let mut i = 4usize;
        while let Some(b) = ctx.line.get(i) {
            if b == b'-' && ctx.line.get(i + 1) == Some(b'-') && ctx.line.get(i + 2) == Some(b'>') {
                return Some(i + 3);
            }
            i += 1;
        }
        return None;
    }

    // GFM comment matching: reject `<!-->`, `<!--->`, and `--` inside body.
    if ctx.line.get(4) == Some(b'>')
        || (ctx.line.get(4) == Some(b'-') && ctx.line.get(5) == Some(b'>'))
    {
        return None;
    }
    let mut i = 4usize;
    while let Some(b) = ctx.line.get(i) {
        if b == b'-' && ctx.line.get(i + 1) == Some(b'-') {
            if ctx.line.get(i + 2) == Some(b'>') {
                return Some(i + 3);
            }
            return None;
        }
        i += 1;
    }
    None
}

fn collect_merged_text(ctx: &ProcessCtx, len: usize) -> Option<String> {
    let mut buf = Vec::with_capacity(len);
    for i in 0..len {
        buf.push(ctx.line.get(i)?);
    }
    String::from_utf8(buf).ok()
}

fn scan_js_comment(ctx: &ProcessCtx) -> Option<(usize, String)> {
    if !ctx.line.validate(0, b'{') || !ctx.line.validate(1, b'/') || !ctx.line.validate(2, b'*') {
        return None;
    }
    let mut i = 3usize;
    while let Some(b) = ctx.line.get(i) {
        if b == b'*' && ctx.line.get(i + 1) == Some(b'/') && ctx.line.get(i + 2) == Some(b'}') {
            let end = i + 3;
            let raw = collect_merged_text(ctx, end)?;
            let value = raw
                .strip_prefix("{/*")
                .and_then(|s| s.strip_suffix("*/}"))
                .unwrap_or("")
                .to_string();
            return Some((end, value));
        }
        i += 1;
    }
    None
}

#[derive(Clone, Copy)]
enum JsScanState {
    Normal,
    SingleQuote,
    DoubleQuote,
    TemplateQuote,
    LineComment,
    BlockComment,
}

fn scan_js_expression(ctx: &ProcessCtx) -> Option<(usize, String)> {
    if !ctx.line.validate(0, b'{') {
        return None;
    }
    let mut i = 1usize;
    let mut depth = 1usize;
    let mut state = JsScanState::Normal;
    let mut escaped = false;

    while let Some(b) = ctx.line.get(i) {
        match state {
            JsScanState::Normal => match b {
                b'\'' => state = JsScanState::SingleQuote,
                b'"' => state = JsScanState::DoubleQuote,
                b'`' => state = JsScanState::TemplateQuote,
                b'/' if ctx.line.get(i + 1) == Some(b'/') => {
                    state = JsScanState::LineComment;
                    i += 1;
                }
                b'/' if ctx.line.get(i + 1) == Some(b'*') => {
                    state = JsScanState::BlockComment;
                    i += 1;
                }
                b'{' => depth += 1,
                b'}' => {
                    depth = depth.saturating_sub(1);
                    if depth == 0 {
                        let end = i + 1;
                        let raw = collect_merged_text(ctx, end)?;
                        let value = raw
                            .strip_prefix('{')
                            .and_then(|s| s.strip_suffix('}'))
                            .map(str::trim)
                            .unwrap_or("")
                            .to_string();
                        return Some((end, value));
                    }
                }
                _ => {}
            },
            JsScanState::SingleQuote => {
                if escaped {
                    escaped = false;
                } else if b == b'\\' {
                    escaped = true;
                } else if b == b'\'' {
                    state = JsScanState::Normal;
                }
            }
            JsScanState::DoubleQuote => {
                if escaped {
                    escaped = false;
                } else if b == b'\\' {
                    escaped = true;
                } else if b == b'"' {
                    state = JsScanState::Normal;
                }
            }
            JsScanState::TemplateQuote => {
                if escaped {
                    escaped = false;
                } else if b == b'\\' {
                    escaped = true;
                } else if b == b'`' {
                    state = JsScanState::Normal;
                }
            }
            JsScanState::LineComment => {
                if matches!(b, b'\n' | b'\r') {
                    state = JsScanState::Normal;
                }
            }
            JsScanState::BlockComment => {
                if b == b'*' && ctx.line.get(i + 1) == Some(b'/') {
                    state = JsScanState::Normal;
                    i += 1;
                }
            }
        }
        i += 1;
    }
    None
}

pub(super) fn process(ctx: &mut ProcessCtx) -> bool {
    if ctx.parser.options.mdx_component {
        if let Some((len, value)) = scan_js_comment(ctx) {
            let start_location = ctx.line.start_location();
            ctx.line.skip(len);
            let end_location = ctx.line.end_location();
            ctx.parser.append_to(
                ctx.id,
                MarkdownNode::Html(Box::new(html::Html::Inline(html::HtmlType::JSComment(
                    value,
                )))),
                (start_location, end_location),
            );
            return true;
        }
        if let Some((len, value)) = scan_js_expression(ctx) {
            let start_location = ctx.line.start_location();
            ctx.line.skip(len);
            let end_location = ctx.line.end_location();
            ctx.parser.append_to(
                ctx.id,
                MarkdownNode::Html(Box::new(html::Html::Inline(html::HtmlType::JSExpression(
                    value,
                )))),
                (start_location, end_location),
            );
            return true;
        }
    }

    if let Some(len) = scan_html_comment_len(ctx) {
        let Some(raw) = collect_merged_text(ctx, len) else {
            return false;
        };
        let start_location = ctx.line.start_location();
        ctx.line.skip(len);
        let end_location = ctx.line.end_location();
        let idx = ctx.parser.append_to(
            ctx.id,
            MarkdownNode::Html(Box::new(html::Html::Inline(html::HtmlType::HtmlComment))),
            (start_location, end_location),
        );
        ctx.parser
            .append_text_to_owned(idx, raw, (start_location, end_location));
        return true;
    }

    // `<? ... ?>` 允许跨行；MergedSpan 需要跨 Span 扫描闭合符。
    if let Some(len) = scan_processing_instruction_len(ctx) {
        let Some(raw) = collect_merged_text(ctx, len) else {
            return false;
        };
        let start_location = ctx.line.start_location();
        ctx.line.skip(len);
        let end_location = ctx.line.end_location();
        let idx = ctx.parser.append_to(
            ctx.id,
            MarkdownNode::Html(Box::new(html::Html::Inline(
                html::HtmlType::ProcessingInstruction,
            ))),
            (start_location, end_location),
        );
        ctx.parser
            .append_text_to_owned(idx, raw, (start_location, end_location));
        return true;
    }
    if let Some(len) = scan_cdata_len(ctx) {
        let Some(raw) = collect_merged_text(ctx, len) else {
            return false;
        };
        let start_location = ctx.line.start_location();
        ctx.line.skip(len);
        let end_location = ctx.line.end_location();
        let idx = ctx.parser.append_to(
            ctx.id,
            MarkdownNode::Html(Box::new(html::Html::Inline(html::HtmlType::CDataSection))),
            (start_location, end_location),
        );
        ctx.parser
            .append_text_to_owned(idx, raw, (start_location, end_location));
        return true;
    }

    // Fallback: parse tags that span multiple merged lines, e.g. `<foo\nbar>`.
    if ctx.line.validate(0, b'<') {
        let mut probe_len = 0usize;
        let mut found_gt = false;
        let mut quote: Option<u8> = None;
        while let Some(b) = ctx.line.get(probe_len) {
            probe_len += 1;
            match quote {
                Some(q) if b == q => quote = None,
                None if b == b'\'' || b == b'"' => quote = Some(b),
                None if b == b'>' => {
                    found_gt = true;
                    break;
                }
                _ => {}
            }
            if b == b'>' && quote.is_none() {
                found_gt = true;
                break;
            }
            if probe_len > 4096 {
                break;
            }
        }
        if found_gt {
            if let Some(raw) = collect_merged_text(ctx, probe_len) {
                if !raw.as_bytes().iter().any(|&b| b == b'\n' || b == b'\r') {
                    // Single-line tags should continue using the normal path below.
                    // This fallback only targets merged multi-line tag spans.
                    // (e.g. `<foo\nbar>`)
                    // Continue to regular scanner.
                    //
                    // We cannot `continue` here because this is not in a loop.
                } else {
                    let probe_raw = raw.replace(['\n', '\r'], " ");
                    let mut scanner = crate::scanner::Scanner::new(&probe_raw);
                    if let Some(mut probe_span) = crate::span::Span::extract(&mut scanner) {
                        if let Some((_, len, _)) =
                            scan_html_type(&mut probe_span, true, ctx.parser.options.mdx_component)
                        {
                            if len == probe_raw.len() {
                                let start_location = ctx.line.start_location();
                                ctx.line.skip(probe_len);
                                let end_location = ctx.line.end_location();
                                let idx = ctx.parser.append_to(
                                    ctx.id,
                                    MarkdownNode::Html(Box::new(html::Html::Inline(
                                        html::HtmlType::Declaration,
                                    ))),
                                    (start_location, end_location),
                                );
                                // Preserve original raw tag text (including merged newlines).
                                ctx.parser.append_text_to_owned(
                                    idx,
                                    raw,
                                    (start_location, end_location),
                                );
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }

    let ProcessCtx {
        id, line, parser, ..
    } = ctx;
    let current_span = match line.current_span_mut() {
        Some(span) => span,
        None => return false,
    };
    let start_location = current_span.start_location();
    let (_, len, html_type) =
        if let Some(html_type) = scan_html_type(current_span, true, parser.options.mdx_component) {
            html_type
        } else {
            return false;
        };
    // Inline comments should only be accepted when fully matched by
    // `scan_html_comment_len`; avoid treating bare `<!--` as raw HTML.
    if matches!(html_type, html::HtmlType::HtmlComment) && len <= 4 {
        return false;
    }
    if matches!(html_type, html::HtmlType::CDataSection) && len <= 9 {
        return false;
    }
    let end_location = if len == 0 {
        current_span.last_token_end_location()
    } else {
        current_span.location_at_byte(current_span.cursor() + len)
    };
    let raw_tag_text = current_span.slice(0, len).to_string();
    let end_tag_with_space_before_gt = matches!(
        &html_type,
        html::HtmlType::RawTextContainer(.., html::Flag::End)
            | html::HtmlType::CanonicalBlockTag(.., html::Flag::End)
            | html::HtmlType::GenericTag(.., html::Flag::End)
            | html::HtmlType::Component(.., html::Flag::End)
    ) && raw_tag_text.as_bytes().len() >= 2
        && raw_tag_text.as_bytes()[raw_tag_text.as_bytes().len() - 2] == b' ';
    let raw_inline = match &html_type {
        html::HtmlType::HtmlComment
        | html::HtmlType::ProcessingInstruction
        | html::HtmlType::Declaration
        | html::HtmlType::CDataSection
        | html::HtmlType::RawTextContainer(.., html::Flag::SelfClose)
        | html::HtmlType::CanonicalBlockTag(.., html::Flag::SelfClose)
        | html::HtmlType::GenericTag(.., html::Flag::SelfClose) => Some(raw_tag_text.clone()),
        html::HtmlType::RawTextContainer(.., html::Flag::End)
        | html::HtmlType::CanonicalBlockTag(.., html::Flag::End)
        | html::HtmlType::GenericTag(.., html::Flag::End)
        | html::HtmlType::Component(.., html::Flag::End)
            if end_tag_with_space_before_gt =>
        {
            Some(raw_tag_text.clone())
        }
        _ => None,
    };
    current_span.skip(len);
    let (start, end, html) = match &html_type {
        html::HtmlType::HtmlComment
        | html::HtmlType::JSComment(..)
        | html::HtmlType::JSExpression(..) => (0, 0, html::Html::Inline(html_type)),
        html::HtmlType::ProcessingInstruction
        | html::HtmlType::Declaration
        | html::HtmlType::CDataSection
        | html::HtmlType::RawTextContainer(.., html::Flag::SelfClose)
        | html::HtmlType::CanonicalBlockTag(.., html::Flag::SelfClose)
        | html::HtmlType::GenericTag(.., html::Flag::SelfClose) => {
            (0, 0, html::Html::Inline(html::HtmlType::Declaration))
        }
        html::HtmlType::Component(.., html::Flag::SelfClose) => {
            (0, 0, html::Html::Inline(html_type))
        }
        html::HtmlType::RawTextContainer(.., html::Flag::End)
        | html::HtmlType::CanonicalBlockTag(.., html::Flag::End)
        | html::HtmlType::GenericTag(.., html::Flag::End)
        | html::HtmlType::Component(.., html::Flag::End)
            if end_tag_with_space_before_gt =>
        {
            (0, 0, html::Html::Inline(html::HtmlType::Declaration))
        }
        html::HtmlType::RawTextContainer(.., html::Flag::End)
        | html::HtmlType::CanonicalBlockTag(.., html::Flag::End)
        | html::HtmlType::GenericTag(.., html::Flag::End)
        | html::HtmlType::Component(.., html::Flag::End) => (0, 0, html::Html::Inline(html_type)),
        html::HtmlType::RawTextContainer(..)
        | html::HtmlType::CanonicalBlockTag(..)
        | html::HtmlType::GenericTag(..)
        | html::HtmlType::Component(..) => {
            let mut html = html::Html::Inline(html_type);
            if let Some((start, end)) = html.scan_end_span(current_span) {
                html.set_flag_is_full();
                (start, end, html)
            } else {
                (0, 0, html)
            }
        }
    };
    let idx = parser.append_to(
        *id,
        MarkdownNode::Html(Box::new(html)),
        (start_location, end_location),
    );
    if let Some(raw) = raw_inline {
        parser.append_text_to_owned(idx, raw, (start_location, end_location));
    }
    if start > 0 {
        let text_line = current_span.slice(0, start);
        if matches!(
            &parser.tree[idx].body,
            MarkdownNode::Html(h)
                if matches!(
                    h.as_ref(),
                    html::Html::Inline(
                        html::HtmlType::CanonicalBlockTag(_, html::Flag::Full)
                            | html::HtmlType::GenericTag(_, html::Flag::Full)
                            | html::HtmlType::Component(_, html::Flag::Full)
                    )
                )
        ) {
            super::process(idx, parser, vec![text_line]);
        } else {
            parser.append_text_to_owned(
                idx,
                text_line.to_string(),
                (text_line.start_location(), text_line.end_location()),
            );
        }
    }
    if end > 0 {
        current_span.skip(end);
    };
    true
}

#[cfg(test)]
mod tests {
    use crate::parser::{Parser, ParserOptions};

    #[test]
    fn case_626_comment_with_inner_double_dash() {
        let text = "foo <!-- this is a --\ncomment - with hyphens -->";
        let ast = Parser::new(text).parse();
        assert_eq!(
            ast.to_html(),
            "<p>foo <!-- this is a --\ncomment - with hyphens --></p>"
        );
    }

    #[test]
    fn case_627_short_comment_openers() {
        let text = "foo <!--> foo -->\n\nfoo <!---> foo -->";
        let ast = Parser::new(text).parse();
        assert_eq!(
            ast.to_html(),
            "<p>foo <!--> foo --&gt;</p>\n<p>foo <!---> foo --&gt;</p>"
        );
    }

    #[test]
    fn gfm_reject_invalid_comment_forms() {
        let text =
            "foo <!-- not a comment -- two hyphens -->\n\nfoo <!--> foo -->\n\nfoo <!-- foo--->";
        let ast = Parser::new_with_options(text, ParserOptions::default().enabled_gfm()).parse();
        assert_eq!(
            ast.to_html(),
            "<p>foo &lt;!-- not a comment -- two hyphens --&gt;</p>\n<p>foo &lt;!--&gt; foo --&gt;</p>\n<p>foo &lt;!-- foo---&gt;</p>"
        );
    }
}
