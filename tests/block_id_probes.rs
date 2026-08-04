//! OFM BlockId 各形态的行为锁定测试。
//!
//! 期望值捕获自 F2 步骤 b（共享 BlockId 发现扫描器）之前的输出；
//! 该步骤必须保持全部输出逐字节不变（扫描器只写 `node.id`，
//! 标记剥离仍由 inline 引擎完成）。

use ptdgrp_markdown::{Parser, ParserOptions};

fn html(src: &str) -> String {
    Parser::new_with_options(src, ParserOptions::default().enabled_ofm())
        .parse()
        .unwrap()
        .to_html()
}

#[test]
fn sameline_trailing_id_keeps_preceding_space() {
    assert_eq!(
        html("This para ^same-id\n\nNext.\n"),
        "<p id=\"same-id\">This para </p>\n<p>Next.</p>"
    );
}

#[test]
fn sameline_id_on_heading() {
    assert_eq!(
        html("## Head text ^hid\n\nBody.\n"),
        "<h2 id=\"hid\">Head text </h2>\n<p>Body.</p>"
    );
}

#[test]
fn sameline_id_on_list_item() {
    assert_eq!(
        html("- item one ^item-id\n- item two\n"),
        "<ul>\n<li>item one </li>\n<li>item two</li>\n</ul>"
    );
}

#[test]
fn nextline_id_in_blockquote() {
    assert_eq!(
        html("> quoted text\n> ^qid\n"),
        "<blockquote>\n<p id=\"qid\">quoted text</p>\n</blockquote>"
    );
}

#[test]
fn midblock_lineend_id_still_recognized_by_engine() {
    assert_eq!(
        html("first ^mid\nsecond line\n"),
        "<p id=\"mid\">first\nsecond line</p>"
    );
}

#[test]
fn caret_inside_code_span_is_not_an_id() {
    assert_eq!(
        html("para `code ^fake` end\n"),
        "<p>para <code>code ^fake</code> end</p>"
    );
    assert_eq!(
        html("para `code ^fake`\n"),
        "<p>para <code>code ^fake</code></p>"
    );
    assert_eq!(
        html("a `x ^trap\ny` tail\n"),
        "<p>a <code>x ^trap y</code> tail</p>"
    );
}

#[test]
fn id_only_line_allows_trailing_whitespace() {
    assert_eq!(
        html("content here\n^ws-id   \n"),
        "<p id=\"ws-id\">content here</p>"
    );
}

#[test]
fn later_id_overwrites_earlier_one() {
    assert_eq!(
        html("text ^first\nmore ^second\n"),
        "<p id=\"second\">text\nmore </p>"
    );
}
