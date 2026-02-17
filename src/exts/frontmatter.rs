use crate::exts::yaml::{YamlMap, parse_yaml};
use crate::parser::Parser;

/// 跳过 \r\n 或 \n 换行符
#[inline]
fn skip_newline(scanner: &mut crate::scanner::Scanner) {
    if scanner.peek() == Some(b'\r') {
        scanner.advance();
    }
    if scanner.peek() == Some(b'\n') {
        scanner.advance();
    }
}

/// 检查是否是有效的 frontmatter 标记行（--- 或 +++）
#[inline]
fn is_marker_line(scanner: &mut crate::scanner::Scanner, marker: u8) -> bool {
    // 快速路径：检查第一个字节
    if scanner.peek() != Some(marker) {
        return false;
    }

    // 检查是否有连续 3 个标记符
    if scanner.count_consecutive(marker) < 3 {
        return false;
    }

    // 临时前进 3 个字节检查后面是否只有空白
    let pos = scanner.pos();
    scanner.advance_by(3);
    scanner.skip_spaces();

    let is_valid = match scanner.peek() {
        None | Some(b'\n') | Some(b'\r') => true,
        _ => false,
    };

    // 恢复位置
    scanner.set_pos(pos);
    is_valid
}

pub fn parse(parser: &mut Parser) -> Option<YamlMap> {
    let snapshot = parser.scanner.snapshot();

    // 快速检查：第一个字节必须是 - 或 +
    let first_byte = parser.scanner.peek()?;
    let marker = match first_byte {
        b'-' => b'-',
        b'+' => b'+',
        _ => return None,
    };

    // 检查开始标记行
    if !is_marker_line(&mut parser.scanner, marker) {
        return None;
    }

    // 跳过开始标记行
    parser.scanner.advance_by(3);
    parser.scanner.skip_spaces();
    skip_newline(&mut parser.scanner);

    // 收集 frontmatter 内容
    let content_start = parser.scanner.pos();

    // 查找结束标记
    loop {
        let line_start = parser.scanner.pos();

        // 快速检查：如果第一个字节是标记符，才进行完整检查
        if parser.scanner.peek() == Some(marker) && is_marker_line(&mut parser.scanner, marker) {
            // 找到结束标记
            let content_end = line_start;

            // 跳过结束标记行
            parser.scanner.advance_by(3);
            parser.scanner.skip_spaces();
            skip_newline(&mut parser.scanner);

            // 提取并解析 YAML。若不是有效 YAML frontmatter，回滚并按普通 Markdown 处理。
            let yaml_text = parser.scanner.slice(content_start, content_end);
            return match parse_yaml(yaml_text) {
                Some(map) => Some(map),
                None => {
                    parser.scanner.resume(&snapshot);
                    None
                }
            };
        }

        // 跳到下一行
        parser.scanner.skip_to_eol();

        // 检查是否到达文件末尾
        if parser.scanner.peek().is_none() {
            // 没有找到结束标记，恢复并返回 None
            parser.scanner.resume(&snapshot);
            return None;
        }

        // 跳过换行符
        skip_newline(&mut parser.scanner);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MarkdownNode;
    use crate::exts::yaml::YamlValue;
    use crate::parser::ParserOptions;

    #[test]
    fn it_works() {
        let mut parser = Parser::new(
            r#"
--- 
external: false
draft: true
title: Hello World
description: It's a beautiful world out there.
date: 2022-11-05
---   
Hello world        "#
                .trim_start(),
        );
        parser.parse_frontmatter().unwrap();
        let frontmatter = if let MarkdownNode::FrontMatter(frontmatter) = &parser.tree[1].body {
            frontmatter
        } else {
            panic!("Failed to get frontmatter node")
        };
        assert_eq!(frontmatter.get("external"), Some(&YamlValue::Bool(false)));
        assert_eq!(frontmatter.get("draft"), Some(&YamlValue::Bool(true)));
        assert_eq!(
            frontmatter.get("description"),
            Some(&YamlValue::String(
                "It's a beautiful world out there.".to_string()
            ))
        );
        assert_eq!(
            frontmatter.get("date"),
            Some(&YamlValue::String("2022-11-05".to_string()))
        );
        let ast = parser.parse();
        println!("AST:\n{ast:?}");
        assert_eq!(ast.to_html(), "<p>Hello world</p>")
    }

    #[test]
    fn test_no_frontmatter() {
        let mut parser = Parser::new("# Hello\n\nWorld");
        parser.parse_frontmatter().unwrap();
        let ast = parser.into_ast();
        assert!(ast.is_empty())
    }

    #[test]
    fn test_incomplete_frontmatter() {
        // 没有结束标记
        let mut parser = Parser::new("---\ntitle: Test\n\nContent");
        parser.parse_frontmatter().unwrap();
        let ast = parser.into_ast();
        println!("AST:\n{ast:?}");
        assert!(ast.is_empty());
    }

    #[test]
    fn test_frontmatter_with_dashes_in_content() {
        // 内容中包含 -- 但不是标记行
        let mut parser = Parser::new(
            r#"---
title: Test
note: -- not a marker
---
Content"#,
        );
        parser.parse_frontmatter().unwrap();
        let frontmatter = if let MarkdownNode::FrontMatter(frontmatter) = &parser.tree[1].body {
            frontmatter
        } else {
            panic!("Failed to get frontmatter node")
        };
        assert_eq!(
            frontmatter.get("title"),
            Some(&YamlValue::String("Test".to_string()))
        );
        assert_eq!(
            frontmatter.get("note"),
            Some(&YamlValue::String("-- not a marker".to_string()))
        );
    }

    #[test]
    fn test_plus_marker() {
        let mut parser = Parser::new(
            r#"+++
title: Test
+++
Content"#,
        );
        parser.parse_frontmatter().unwrap();
        let frontmatter = if let MarkdownNode::FrontMatter(frontmatter) = &parser.tree[1].body {
            frontmatter
        } else {
            panic!("Failed to get frontmatter node")
        };
        assert_eq!(
            frontmatter.get("title"),
            Some(&YamlValue::String("Test".to_string()))
        );
    }

    #[test]
    fn basic_usage() {}

    #[test]
    fn cjk_nouns_from_options_and_frontmatter_are_merged() {
        let input = r#"---
nouns:
  - 豆瓣FM
---
我在用GitHub和豆瓣FM"#;
        let ast = crate::parser::Parser::new_with_options(
            input,
            ParserOptions::default()
                .enabled_ofm()
                .enabled_cjk_autocorrect()
                .with_cjk_nouns(["GitHub"])
                .with_cjk_nouns_from_frontmatter("nouns"),
        )
        .parse();
        assert_eq!(ast.to_html(), "<p>我在用GitHub和豆瓣FM</p>");
    }
}
