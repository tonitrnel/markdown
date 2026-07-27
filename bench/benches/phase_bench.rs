use criterion::{Criterion, black_box, criterion_group, criterion_main};
use markdown::{Parser, ParserOptions};
use std::fs;

fn bench_parse_only(c: &mut Criterion) {
    let text = fs::read_to_string("./bench/fixtures/curated/_data.md").unwrap();
    c.bench_function("parse_ast_only", |b| {
        b.iter(|| {
            let parser = Parser::new_with_options(&text, ParserOptions::default().enabled_ofm());
            black_box(parser.parse());
        })
    });
}

fn bench_html_only(c: &mut Criterion) {
    let text = fs::read_to_string("./bench/fixtures/curated/_data.md").unwrap();
    // 预先解析一次
    let parser = Parser::new_with_options(&text, ParserOptions::default().enabled_ofm());
    let ast = parser.parse();
    c.bench_function("html_render_only", |b| {
        b.iter(|| {
            black_box(ast.to_html());
        })
    });
}

fn bench_full(c: &mut Criterion) {
    let text = fs::read_to_string("./bench/fixtures/curated/_data.md").unwrap();
    c.bench_function("full_parse_and_html", |b| {
        b.iter(|| {
            let parser = Parser::new_with_options(&text, ParserOptions::default().enabled_ofm());
            let ast = black_box(parser.parse());
            black_box(ast.to_html());
        })
    });
}

fn bench_selective_session(c: &mut Criterion) {
    use markdown::selective::VisitControl;
    let data = fs::read_to_string("./bench/fixtures/curated/_data.md").unwrap();
    let corpus = fs::read_to_string("./bench/fixtures/corpora/markdown-it-corpus.md").unwrap();
    c.bench_function("block_only/_data", |b| {
        b.iter(|| {
            let phase = Parser::new_with_options(&data, ParserOptions::default().enabled_ofm())
                .parse_blocks_with(|_| true, |_| VisitControl::Continue);
            black_box(phase);
        })
    });
    c.bench_function("session_prepare/_data", |b| {
        b.iter(|| {
            let phase = Parser::new_with_options(&data, ParserOptions::default().enabled_ofm())
                .parse_blocks_with(|_| true, |_| VisitControl::Continue)
                .prepare_semantic_targets();
            black_box(phase);
        })
    });
    c.bench_function("session_prepare/corpus", |b| {
        b.iter(|| {
            let phase = Parser::new_with_options(&corpus, ParserOptions::default().enabled_ofm())
                .parse_blocks_with(|_| true, |_| VisitControl::Continue)
                .prepare_semantic_targets();
            black_box(phase);
        })
    });
}

criterion_group!(
    benches,
    bench_parse_only,
    bench_html_only,
    bench_full,
    bench_selective_session
);
criterion_main!(benches);
