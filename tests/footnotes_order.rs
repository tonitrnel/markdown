use ptdgrp_markdown::{Parser, ParserOptions};

/// 回归：脚注引用分布在多个 Block 时，编号与脚注列表必须按文档顺序。
/// 见 .scratch/footnote-numbering-order/issues/01-footnote-numbering-follows-hashmap-order.md
#[test]
fn footnote_numbering_follows_document_order_across_blocks() {
    let mut src = String::new();
    for i in 1..=20 {
        src.push_str(&format!("Paragraph {i} refs[^n{i}].\n\n"));
    }
    for i in 1..=20 {
        src.push_str(&format!("[^n{i}]: note {i}\n"));
    }
    let html = Parser::new_with_options(&src, ParserOptions::default().enabled_ofm())
        .parse()
        .unwrap()
        .to_html();

    // 每个引用标记按文档顺序取得编号 1..=20
    for i in 1..=20 {
        let marker = format!(r##"<a href="#cont-fn-n{i}" id="cont-fn-ref-n{i}">[{i}]</a>"##);
        assert!(
            html.contains(&marker),
            "第 {i} 个引用应编号为 [{i}]，未找到 {marker}\nhtml:\n{html}"
        );
    }

    // 底部脚注列表按文档顺序排列（<li id="..."> 出现位置严格递增）
    let mut last_pos = 0;
    for i in 1..=20 {
        let li = format!(r#"<li id="cont-fn-n{i}">"#);
        let pos = html
            .find(&li)
            .unwrap_or_else(|| panic!("脚注列表缺少 {li}\nhtml:\n{html}"));
        assert!(
            pos > last_pos || i == 1,
            "脚注定义 n{i} 未按文档顺序出现在列表中"
        );
        last_pos = pos;
    }
}

/// 相邻脚注引用各自独立渲染（Obsidian 语义）。
/// 见 .scratch/footnote-adjacent-refs/issues/01-adjacent-footnote-refs-swallowed.md
#[test]
fn adjacent_footnote_references_render_separately() {
    let src = "A[^x][^y].\n\n[^x]: X\n[^y]: Y\n";
    let html = Parser::new_with_options(src, ParserOptions::default().enabled_ofm())
        .parse()
        .unwrap()
        .to_html();
    assert!(
        html.contains(
            r##"A<a href="#cont-fn-x" id="cont-fn-ref-x">[1]</a><a href="#cont-fn-y" id="cont-fn-ref-y">[2]</a>."##
        ),
        "两个相邻引用都应渲染:\n{html}"
    );
    assert!(html.contains(r#"<li id="cont-fn-x">"#));
    assert!(html.contains(r#"<li id="cont-fn-y">"#));
}

/// 脚注引用优先于内联链接形式：`[^x](t)` 是脚注 + 字面括号内容。
#[test]
fn footnote_reference_takes_precedence_over_inline_link_form() {
    let src = "A[^x](not-a-url).\n\n[^x]: X\n";
    let html = Parser::new_with_options(src, ParserOptions::default().enabled_ofm())
        .parse()
        .unwrap()
        .to_html();
    assert!(
        html.contains(r##">[1]</a>(not-a-url).</p>"##),
        "括号内容应保持字面:\n{html}"
    );
}

/// 同一 Block 内多个引用的既有顺序行为保持不变。
#[test]
fn footnote_numbering_within_single_block_unchanged() {
    let src = "A[^x] then B[^y].\n\n[^x]: note x\n[^y]: note y\n";
    let html = Parser::new_with_options(src, ParserOptions::default().enabled_ofm())
        .parse()
        .unwrap()
        .to_html();
    assert!(html.contains(r##"<a href="#cont-fn-x" id="cont-fn-ref-x">[1]</a>"##));
    assert!(html.contains(r##"<a href="#cont-fn-y" id="cont-fn-ref-y">[2]</a>"##));
    let x = html.find(r#"<li id="cont-fn-x">"#).expect("missing x li");
    let y = html.find(r#"<li id="cont-fn-y">"#).expect("missing y li");
    assert!(x < y);
}
