use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

// 需要将 utils 模块暴露给 bench，通过 lib 的 pub(crate) 无法直接访问
// 所以我们直接内联测试逻辑，或者通过集成方式测试

// 由于 utils 是 pub(crate)，bench 无法直接访问。
// 我们通过完整的解析流程来间接测量，同时也写一个独立的微基准。

fn build_cjk_text(repeats: usize) -> String {
    let base = concat!(
        "在LeanCloud上，數據儲存是圍繞AVObject進行的。",
        "每個AVObject都包含了與JSON兼容的key-value對應的數據。",
        "數據是schema-free的，你不需要在每個AVObject上提前指定存在哪些键。",
        "今天出去買菜花了5000元。",
        "我家的光纖入屋寬頻有10Gbps，SSD一共有20TB。",
        "核磁共振成像(NMRI)是什麼原理都不知道?JFGI!",
        "使用GitHub登錄。我們的客戶有GitHub、Foursquare、Microsoft Corporation。",
        "她竟然對你說「喵」??!!太好了!!!",
        "推薦你閱讀erta---非常地有趣。Some dashes: em---em en--en。",
        "Ellipses...and...and....不要使用不道地的縮寫。",
    );
    base.repeat(repeats)
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("utils");

    for size in [1, 10, 100] {
        let text = build_cjk_text(size);
        let label = format!("{}chars", text.len());

        // 通过 Parser 完整流程测量（包含 CJK autocorrect）
        group.bench_with_input(
            BenchmarkId::new("parse_with_cjk", &label),
            &text,
            |b, text| {
                use markdown::{Parser, ParserOptions};
                b.iter(|| {
                    let parser = Parser::new_with_options(
                        text,
                        ParserOptions::default()
                            .enabled_ofm()
                            .enabled_cjk_autocorrect(),
                    );
                    let _ast = parser.parse();
                });
            },
        );

        // 不启用 CJK autocorrect 的对照组
        group.bench_with_input(
            BenchmarkId::new("parse_without_cjk", &label),
            &text,
            |b, text| {
                use markdown::{Parser, ParserOptions};
                b.iter(|| {
                    let parser =
                        Parser::new_with_options(text, ParserOptions::default().enabled_ofm());
                    let _ast = parser.parse();
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
