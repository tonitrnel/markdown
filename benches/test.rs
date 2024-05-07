use criterion::{criterion_group, criterion_main, Criterion};

use markdown::{Parser, ParserOptions};
use std::fs;

fn parse(text: &str) {
    let parser = Parser::new_with_options(text, ParserOptions::default().enabled_ofm());
    let ast = parser.parse();
    let _html = ast.to_html();
}

fn criterion_benchmark(c: &mut Criterion) {
    let text = fs::read_to_string("./benches/_data.md").unwrap();
    c.bench_function("markdown", |b| b.iter(|| parse(&text)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
