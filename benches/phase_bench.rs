use criterion::{Criterion, criterion_group, criterion_main};
use markdown::{Parser, ParserOptions};
use std::fs;

fn bench_parse_only(c: &mut Criterion) {
    let text = fs::read_to_string("./benches/_data.md").unwrap();
    c.bench_function("parse_ast_only", |b| {
        b.iter(|| {
            let parser = Parser::new_with_options(&text, ParserOptions::default().enabled_ofm());
            let _ast = parser.parse();
        })
    });
}

fn bench_html_only(c: &mut Criterion) {
    let text = fs::read_to_string("./benches/_data.md").unwrap();
    // 预先解析一次
    let parser = Parser::new_with_options(&text, ParserOptions::default().enabled_ofm());
    let ast = parser.parse();
    c.bench_function("html_render_only", |b| {
        b.iter(|| {
            let _html = ast.to_html();
        })
    });
}

fn bench_full(c: &mut Criterion) {
    let text = fs::read_to_string("./benches/_data.md").unwrap();
    c.bench_function("full_parse_and_html", |b| {
        b.iter(|| {
            let parser = Parser::new_with_options(&text, ParserOptions::default().enabled_ofm());
            let ast = parser.parse();
            let _html = ast.to_html();
        })
    });
}

criterion_group!(benches, bench_parse_only, bench_html_only, bench_full);
criterion_main!(benches);
