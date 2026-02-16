use crate::ast::MarkdownNode;
use crate::inlines::link;
use crate::inlines::link::scan_link_title;
use crate::parser::Parser;
use crate::span::Span;
use crate::utils;

pub(crate) fn process_link_reference(parser: &mut Parser, node_id: usize) {
    match &parser.tree[node_id].body {
        MarkdownNode::Paragraph => (),
        _ => return,
    };
    let mut line = match parser.inlines.get(&node_id).filter(|item| {
        item.first()
            .and_then(|it| it.get(0))
            .map(|it| it == b'[')
            .unwrap_or(false)
    }) {
        Some(_) => {
            let Some(spans) = parser.inlines.remove(&node_id) else {
                return;
            };
            match Span::merge(&spans) {
                Some(merged) => merged,
                None => return,
            }
        }
        _ => return,
    };
    loop {
        let snapshot = line.snapshot();
        match scan_link_reference(&mut line) {
            Some((ref_label, url, title)) => {
                parser
                    .link_refs
                    .entry(ref_label)
                    .or_insert((url, title.map(utils::entities::unescape_string)));
                continue;
            }
            _ => {
                line.resume(&snapshot);
            }
        }
        break;
    }
    if line.is_end() {
        parser.tree.remove(node_id);
    } else {
        parser.inlines.insert(node_id, vec![line]);
    }
}

pub(crate) fn process_setext_heading_link_reference(parser: &mut Parser, node_id: usize) {
    if !matches!(
        parser.tree[node_id].body,
        MarkdownNode::Heading(crate::ast::heading::Heading::SETEXT(_))
    ) {
        return;
    }
    let Some(spans) = parser.inlines.remove(&node_id) else {
        return;
    };
    let mut consumed = 0usize;
    for span in spans.iter() {
        let mut line = span.clone();
        let Some((ref_label, url, title)) = scan_link_reference(&mut line) else {
            break;
        };
        if !line.is_end() {
            break;
        }
        parser
            .link_refs
            .entry(ref_label)
            .or_insert((url, title.map(utils::entities::unescape_string)));
        consumed += 1;
    }
    if consumed == 0 {
        parser.inlines.insert(node_id, spans);
        return;
    }
    let remains = spans.into_iter().skip(consumed).collect::<Vec<_>>();
    if !remains.is_empty() {
        parser.inlines.insert(node_id, remains);
    }
}

pub(crate) fn is_link_reference_line(span: &Span) -> bool {
    let mut line = span.clone();
    scan_link_reference(&mut line).is_some() && line.is_end()
}

fn scan_link_reference(line: &mut Span) -> Option<(String, String, Option<String>)> {
    let ref_label = match link::scan_link_label(line) {
        Some((size, label)) => {
            line.skip(size);
            link::normalize_reference(label)
        }
        _ => return None,
    };
    if ref_label.is_empty() || !line.consume(b':') {
        return None;
    }
    link::skip_spaces(line);
    let url = match link::scan_link_url(line) {
        Some((size, url)) => {
            // Reference definitions must have an explicit destination token.
            // `size == 0` means nothing was consumed (e.g. `[foo]:`), which is invalid.
            if size == 0 {
                return None;
            }
            line.skip(size);
            utils::percent_encode::encode(
                link::backslash_unescape(&utils::unescape_string(url.to_string())),
                true,
            )
        }
        _ => return None,
    };
    let before_title_snapshot = line.snapshot();
    let mut title = None;
    let mut title_probe = false;
    if skip_spaces_tabs(line) > 0 {
        title_probe = true;
    }
    if skip_line_ending(line) {
        skip_spaces_tabs(line);
        title_probe = true;
    }
    if title_probe {
        match scan_link_title(line) {
            Some((size, _line)) => {
                line.skip(size);
                title = Some(link::backslash_unescape(&_line.to_string()));
            }
            None => {
                line.resume(&before_title_snapshot);
            }
        }
    }
    let at_line_end = if !only_space_to_eol(line) {
        if title.is_none() {
            false
        } else {
            title = None;
            line.resume(&before_title_snapshot);
            only_space_to_eol(line)
        }
    } else {
        true
    };
    if !at_line_end {
        return None;
    }
    skip_spaces_and_line_ending(line);
    Some((ref_label, url, title))
}

fn only_space_to_eol(line: &Span) -> bool {
    let mut i = 0;
    while let Some(b) = line.get(i) {
        match b {
            b' ' | b'\t' => i += 1,
            b'\n' | b'\r' => return true,
            _ => return false,
        }
    }
    true
}

fn skip_spaces_and_line_ending(line: &mut Span) {
    skip_spaces_tabs(line);
    skip_line_ending(line);
}

fn skip_spaces_tabs(line: &mut Span) -> usize {
    let mut count = 0usize;
    while matches!(line.get(0), Some(b' ' | b'\t')) {
        line.skip(1);
        count += 1;
    }
    count
}

fn skip_line_ending(line: &mut Span) -> bool {
    if line.consume(b'\r') {
        line.consume(b'\n');
        true
    } else {
        line.consume(b'\n')
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    #[test]
    fn case_192() {
        let p = Parser::new(
            r#"[foo]: /url "title"

[foo]"#,
        );
        let ast = p.parse();
        assert_eq!(
            ast.to_html(),
            r#"<p><a href="/url" title="title">foo</a></p>"#
        )
    }
    #[test]
    fn case_193() {
        let p = Parser::new(
            r#"   [foo]: 
      /url  
           'the title'  

[foo]"#,
        );
        let ast = p.parse();
        assert_eq!(
            ast.to_html(),
            r#"<p><a href="/url" title="the title">foo</a></p>"#
        )
    }
    #[test]
    fn case_194() {
        let p = Parser::new(
            r#"[Foo*bar\]]:my_(url) 'title (with parens)'

[Foo*bar\]]"#,
        );
        let ast = p.parse();
        assert_eq!(
            ast.to_html(),
            r#"<p><a href="my_(url)" title="title (with parens)">Foo*bar]</a></p>"#
        )
    }
    #[test]
    fn case_195() {
        let p = Parser::new(
            r#"[Foo bar]:
<my url>
'title'

[Foo bar]"#,
        );
        let ast = p.parse();
        assert_eq!(
            ast.to_html(),
            r#"<p><a href="my%20url" title="title">Foo bar</a></p>"#
        )
    }
    #[test]
    fn case_196() {
        let p = Parser::new(
            r#"[foo]: /url '
title
line1
line2
'

[foo]"#,
        );
        let ast = p.parse();
        assert_eq!(
            ast.to_html(),
            r#"<p><a href="/url" title="
title
line1
line2
">foo</a></p>"#
        )
    }
    #[test]
    fn case_197() {
        let p = Parser::new(
            r#"[foo]: /url 'title

with blank line'

[foo]"#,
        );
        let ast = p.parse();
        assert_eq!(
            ast.to_html(),
            r#"<p>[foo]: /url 'title</p>
<p>with blank line'</p>
<p>[foo]</p>"#
        )
    }
    #[test]
    fn case_217() {
        let p = Parser::new(
            r#"[foo]: /foo-url "foo"
[bar]: /bar-url
  "bar"
[baz]: /baz-url

[foo],
[bar],
[baz]"#,
        );
        let ast = p.parse();
        assert_eq!(
            ast.to_html(),
            r#"<p><a href="/foo-url" title="foo">foo</a>,
<a href="/bar-url" title="bar">bar</a>,
<a href="/baz-url">baz</a></p>"#
        )
    }
    #[test]
    fn case_44() {
        let p = Parser::new(
            r#"## 包装日期为2013年3月10日
超过日期请勿使用"#,
        );
        let ast = p.parse();
        println!("{:?}", ast.to_html());
    }
}
