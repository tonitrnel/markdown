//! 热点 lane 的合成 fixture 生成器，供 `hotspots.rs` 与 `alloc_count.rs` 共用。
//! lane 名称被 `bench/results/incremental-iteration.md` 与渐进式迭代计划的
//! 验收门槛引用，重命名前必须同步更新两处。

pub fn all() -> Vec<(&'static str, String)> {
    vec![
        ("plain_ascii_4k", case_plain_ascii()),
        (
            "many_flushes_dense_inline",
            case_many_flushes_dense_inline(),
        ),
        (
            "multiline_blockquote_dense",
            case_multiline_blockquote_dense(),
        ),
        ("link_dense_flat", case_link_dense_flat()),
        ("nested_brackets", case_nested_brackets()),
        ("many_short_paragraphs", case_many_short_paragraphs()),
        ("reference_heavy", case_reference_heavy()),
        ("cjk_dense", case_cjk_dense()),
    ]
}

/// P1 门槛 lane：纯 ASCII 文本连续扫描。
fn case_plain_ascii() -> String {
    "The quick brown fox jumps over the lazy dog. ".repeat(256)
}

/// P2 门槛 lane：密集强调/链接内联，触发多次 flush。
fn case_many_flushes_dense_inline() -> String {
    let mut s = String::with_capacity(32 * 1024);
    for _ in 0..1024 {
        s.push_str("a *b* c _d_ e [f](g) ");
    }
    s
}

/// P5 回归 lane：多行密集块引用。
fn case_multiline_blockquote_dense() -> String {
    let mut s = String::with_capacity(16 * 1024);
    for _ in 0..512 {
        s.push_str("> abcdefghijklmnopqrstuvwxyz\n");
    }
    s
}

/// P3 门槛 lane：扁平链接密集正文。
fn case_link_dense_flat() -> String {
    let mut s = String::with_capacity(48 * 1024);
    for _ in 0..512 {
        s.push_str("See [alpha](https://example.com/a) and [beta](https://example.com/b) here. ");
    }
    s
}

/// P3 门槛 lane：嵌套括号（链接内图片 + 未匹配方括号）。
fn case_nested_brackets() -> String {
    let mut s = String::with_capacity(64 * 1024);
    for _ in 0..512 {
        s.push_str(
            "[![alt](https://example.com/i.png)](https://example.com) and [outer [inner] tail](https://example.com/o) ",
        );
    }
    s
}

/// P4 门槛 lane：大量短段落，每段一个 pending inline 条目。
fn case_many_short_paragraphs() -> String {
    let mut s = String::with_capacity(40 * 1024);
    for _ in 0..1024 {
        s.push_str("A short paragraph of plain text.\n\n");
    }
    s
}

/// P4 回归 lane：引用定义密集（正文引用 + 文末定义）。
fn case_reference_heavy() -> String {
    let mut s = String::with_capacity(48 * 1024);
    for i in 0..256 {
        s.push_str(&format!("Body uses [link {i}][r{i}] once.\n\n"));
    }
    for i in 0..256 {
        s.push_str(&format!("[r{i}]: https://example.com/{i}\n"));
    }
    s
}

/// P7 位置模型证据 lane：CJK 密集段落（非 ASCII 列计数、宽窄混排、内联标记）。
fn case_cjk_dense() -> String {
    let mut s = String::with_capacity(96 * 1024);
    for _ in 0..256 {
        s.push_str(
            "中文排版與English夾雜時需要處理邊界，數字如42與破折號—還有標點。**強調**與[連結](https://example.com/路径)混排。\n\n",
        );
    }
    s
}
