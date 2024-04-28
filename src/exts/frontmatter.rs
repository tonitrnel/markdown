use fmt::Write;
use std::fmt;

use crate::line::{Line, TokenIteratorGuard};
use crate::parser::Parser;
use crate::tokenizer::Token;

pub fn parse(parser: &mut Parser) -> Option<serde_yaml::Value> {
    let mut guard = TokenIteratorGuard::new(&mut parser.tokens);
    let mut line = match guard.line() {
        Some(line) => line,
        None => return None,
    };
    let marker = match line[0].token {
        Token::Hyphen => Token::Hyphen,
        Token::Plus => Token::Plus,
        _ => return None,
    };
    if !line.starts_with(&marker, 3) || !line.skip(3).only_space_to_end() {
        return None;
    }
    let mut lines = Vec::<Line>::new();
    while let Some(mut line) = guard.line() {
        if line.starts_with(&marker, 3) && line.skip(3).only_space_to_end() {
            let text = lines.iter().fold(String::new(), |mut acc, it| {
                writeln!(acc, "{}", it).unwrap();
                acc
            });
            guard.commit();
            return serde_yaml::from_str::<serde_yaml::Value>(&text).ok();
        }
        line.reset();
        lines.push(line)
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let frontmatter = parser.parse_frontmatter().unwrap();
        assert_eq!(
            frontmatter.get("external"),
            Some(&serde_yaml::Value::Bool(false))
        );
        assert_eq!(
            frontmatter.get("draft"),
            Some(&serde_yaml::Value::Bool(true))
        );
        assert_eq!(
            frontmatter.get("description"),
            Some(&serde_yaml::Value::String(
                "It's a beautiful world out there.".to_string()
            ))
        );
        assert_eq!(
            frontmatter.get("date"),
            Some(&serde_yaml::Value::String("2022-11-05".to_string()))
        );
        let ast = parser.parse();
        println!("AST:\n{ast:?}");
        assert_eq!(ast.to_html(), "<p>Hello world</p>")
    }
    #[test]
    fn basic_usage() {}
}
