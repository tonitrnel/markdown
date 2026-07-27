//! 脚注编号/标签/回链的行为锁定测试。
//!
//! 期望值捕获自 F2 步骤 a（与调度无关的脚注最终化）重构之前的输出，
//! 该重构必须保持这些输出逐字节不变；heading_and_body 一例已随
//! footnote-adjacent-refs 修复（Obsidian 语义：相邻引用各自独立）更新期望。

use markdown::{Parser, ParserOptions};

fn html(src: &str) -> String {
    Parser::new_with_options(src, ParserOptions::default().enabled_ofm())
        .parse()
        .to_html()
}

#[test]
fn multi_occurrence_numbering_and_backrefs() {
    let out = html("A[^x]. B[^y] again[^x].\n\nC[^y].\n\n[^x]: X\n[^y]: Y\n");
    assert_eq!(
        out,
        "<p>A<a href=\"#cont-fn-x\" id=\"cont-fn-ref-x\">[1]</a>. B<a href=\"#cont-fn-y\" id=\"cont-fn-ref-y\">[2]</a> again<a href=\"#cont-fn-x\" id=\"cont-fn-ref-x-2\">[1]</a>.</p>\n<p>C<a href=\"#cont-fn-y\" id=\"cont-fn-ref-y-2\">[2]</a>.</p>\n<section>\n<h2>Footnotes</h2>\n<ol>\n<li id=\"cont-fn-x\">\n<p>X<a href=\"#cont-fn-ref-x\">\u{21a9}</a><a href=\"#cont-fn-ref-x-2\">\u{21a9}<sup>2</sup></a></p>\n</li>\n<li id=\"cont-fn-y\">\n<p>Y<a href=\"#cont-fn-ref-y\">\u{21a9}</a><a href=\"#cont-fn-ref-y-2\">\u{21a9}<sup>2</sup></a></p>\n</li>\n</ol>\n</section>"
    );
}

#[test]
fn inline_footnotes_keep_arrival_style_labels() {
    let out = html("P1 ^[first inline].\n\nP2[^named] and ^[second inline].\n\n[^named]: N\n");
    assert_eq!(
        out,
        "<p>P1 <a href=\"#cont-fn-inline-footnote-1\" id=\"cont-fn-ref-inline-footnote-1\">[1]</a>.</p>\n<p>P2<a href=\"#cont-fn-named\" id=\"cont-fn-ref-named\">[2]</a> and <a href=\"#cont-fn-inline-footnote-3\" id=\"cont-fn-ref-inline-footnote-3\">[3]</a>.</p>\n<section>\n<h2>Footnotes</h2>\n<ol>\n<li id=\"cont-fn-inline-footnote-1\">\n<p>first inline<a href=\"#cont-fn-ref-inline-footnote-1\">\u{21a9}</a></p>\n</li>\n<li id=\"cont-fn-named\">\n<p>N<a href=\"#cont-fn-ref-named\">\u{21a9}</a></p>\n</li>\n<li id=\"cont-fn-inline-footnote-3\">\n<p>second inline<a href=\"#cont-fn-ref-inline-footnote-3\">\u{21a9}</a></p>\n</li>\n</ol>\n</section>"
    );
}

#[test]
fn heading_and_body_references_document_order() {
    // Tail[^a][^b] 是两个相邻引用（Obsidian 语义，见 footnote-adjacent-refs 票）
    let out = html("Intro[^a].\n\n## Head[^b]\n\nTail[^a][^b].\n\n[^a]: A\n[^b]: B\n");
    assert_eq!(
        out,
        "<p>Intro<a href=\"#cont-fn-a\" id=\"cont-fn-ref-a\">[1]</a>.</p>\n<h2>Head<a href=\"#cont-fn-b\" id=\"cont-fn-ref-b\">[2]</a></h2>\n<p>Tail<a href=\"#cont-fn-a\" id=\"cont-fn-ref-a-2\">[1]</a><a href=\"#cont-fn-b\" id=\"cont-fn-ref-b-2\">[2]</a>.</p>\n<section>\n<h2>Footnotes</h2>\n<ol>\n<li id=\"cont-fn-a\">\n<p>A<a href=\"#cont-fn-ref-a\">\u{21a9}</a><a href=\"#cont-fn-ref-a-2\">\u{21a9}<sup>2</sup></a></p>\n</li>\n<li id=\"cont-fn-b\">\n<p>B<a href=\"#cont-fn-ref-b\">\u{21a9}</a><a href=\"#cont-fn-ref-b-2\">\u{21a9}<sup>2</sup></a></p>\n</li>\n</ol>\n</section>"
    );
}

#[test]
fn reference_inside_definition_numbers_by_position() {
    let out = html("X[^o].\n\n[^o]: outer refs[^i]\n\n[^i]: inner\n");
    assert_eq!(
        out,
        "<p>X<a href=\"#cont-fn-o\" id=\"cont-fn-ref-o\">[1]</a>.</p>\n<section>\n<h2>Footnotes</h2>\n<ol>\n<li id=\"cont-fn-o\">\n<p>outer refs<a href=\"#cont-fn-i\" id=\"cont-fn-ref-i\">[2]</a><a href=\"#cont-fn-ref-o\">\u{21a9}</a></p>\n</li>\n<li id=\"cont-fn-i\">\n<p>inner<a href=\"#cont-fn-ref-i\">\u{21a9}</a></p>\n</li>\n</ol>\n</section>"
    );
}

#[test]
fn unreferenced_definition_is_dropped() {
    let out = html("Y[^u].\n\n[^u]: used\n\n[^nope]: unused\n");
    assert_eq!(
        out,
        "<p>Y<a href=\"#cont-fn-u\" id=\"cont-fn-ref-u\">[1]</a>.</p>\n<section>\n<h2>Footnotes</h2>\n<ol>\n<li id=\"cont-fn-u\">\n<p>used<a href=\"#cont-fn-ref-u\">\u{21a9}</a></p>\n</li>\n</ol>\n</section>"
    );
}
