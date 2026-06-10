/// 看纯文本连续扫描成本
use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use markdown::{Parser, ParserOptions};

fn parse_only(text: &str) {
    let parser = Parser::new_with_options(text, ParserOptions::default().enabled_ofm());
    let _ast = parser.parse();
}

fn case_plain_ascii() -> String {
    "The quick brown fox jumps over the lazy dog. ".repeat(256)
}

fn case_many_flushes_dense_inline() -> String {
    let mut s = String::with_capacity(16 * 1024);
    for _ in 0..1024 {
        s.push_str("a *b* c _d_ e [f](g) ");
    }
    s
}

fn case_multiline_blockquote_dense() -> String {
    let mut s = String::with_capacity(16 * 1024);
    for _ in 0..512 {
        s.push_str("> abcdefghijklmnopqrstuvwxyz\n");
    }
    s
}

fn hotspots(c: &mut Criterion) {
    let cases = [
        ("plain_ascii_4k", case_plain_ascii()),
        (
            "many_flushes_dense_inline",
            case_many_flushes_dense_inline(),
        ),
        (
            "multiline_blockquote_dense",
            case_multiline_blockquote_dense(),
        ),
    ];

    let mut group = c.benchmark_group("parse_hotspots");
    for (name, text) in &cases {
        group.throughput(Throughput::Bytes(text.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(name), text, |b, text| {
            b.iter(|| parse_only(black_box(text)));
        });
    }
    group.finish();
}

criterion_group!(benches, hotspots);
criterion_main!(benches);
