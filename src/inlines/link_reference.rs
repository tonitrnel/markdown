use crate::ast::MarkdownNode;
use crate::inlines::link;
use crate::inlines::link::scan_link_title;
use crate::line::Line;
use crate::parser::Parser;
use crate::tokenizer::Token;
use crate::utils;

pub(crate) fn process_link_reference(parser: &mut Parser, node_id: usize) {
    match &parser.tree[node_id].body {
        MarkdownNode::Paragraph => (),
        _ => return,
    };
    let mut line = match parser.inlines.get(&node_id).filter(|item| {
        item.first()
            .and_then(|it| it.get(0))
            .map(|it| matches!(it, Token::LBracket))
            .unwrap_or(false)
    }) {
        Some(_) => Line::extends(parser.inlines.remove(&node_id).unwrap()),
        _ => return,
    };
    // println!(
    //     "检测是否存在 reference #{node_id} => {:?}",
    //     parser.inlines.keys().collect::<Vec<_>>()
    // );
    // println!("{:?}", line)
    loop {
        let snapshot = line.snapshot();
        match scan_link_reference(&mut line) {
            Some((ref_label, url, title)) => {
                // println!("写入 reference {ref_label:?} ({url:?},{title:?})");
                parser
                    .link_refs
                    .entry(ref_label)
                    .or_insert((url, title.map(utils::entities::unescape_string)));
                continue;
            }
            _ => {
                line.resume(snapshot);
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
fn scan_link_reference(line: &mut Line) -> Option<(String, String, Option<String>)> {
    let ref_label = match link::scan_link_label(line) {
        Some((size, label)) => {
            line.skip(size);
            link::normalize_reference(label)
        }
        _ => return None,
    };
    // println!("scan_link_reference label={ref_label:?}")
    if ref_label.is_empty() || !line.consume(Token::Colon) {
        return None;
    }
    link::skip_spaces(line);
    let url = match link::scan_link_url(line) {
        Some((size, url)) => {
            line.skip(size);
            utils::percent_encode::encode(utils::unescape_string(url.to_string()), true)
        }
        _ => return None,
    };
    // println!("scan_link_reference url={url:?}")
    let before_title_snapshot = line.snapshot();
    let mut title = {
        let count = link::skip_spaces(line);
        if count > 0 {
            match scan_link_title(line) {
                Some((size, _line)) => {
                    line.skip(size);
                    Some(_line.to_string())
                }
                _ => None,
            }
        } else {
            None
        }
    };
    // println!("scan_link_reference title={title:?} line={line:?}")
    let at_line_end = if !line.only_space_to_end() {
        if title.is_none() {
            false
        } else {
            title = None;
            line.resume(before_title_snapshot);
            line.only_space_to_end()
        }
    } else {
        true
    };
    if !at_line_end {
        return None;
    }
    link::skip_spaces(line);
    Some((ref_label, url, title))
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
        // println!("{ast:?}")
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
        // println!("{ast:?}")
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
        // println!("{ast:?}")
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
        // println!("{ast:?}")
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
        // println!("{ast:?}")
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
        // println!("{ast:?}")
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
        // println!("{ast:?}")
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
