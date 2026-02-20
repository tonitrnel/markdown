use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use markdown_local::{Parser, ParserOptions};
use markdown_rs::{Options as MdRsOptions, ParseOptions as MdRsParseOptions};
use pulldown_cmark::{Options, Parser as PdParser, html};

fn dataset_cases() -> Vec<(&'static str, &'static str)> {
    vec![
        ("default_data", include_str!("../data/data.md")),
        (
            "markdown_it_corpus",
            include_str!("../../../data/markdown-it-corpus.md"),
        ),
    ]
}

fn bench_parse_only(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_only");
    for (case, text) in dataset_cases() {
        group.bench_with_input(BenchmarkId::new("markdown", case), &text, |b, input| {
            b.iter(|| {
                let parser =
                    Parser::new_with_options(input, ParserOptions::default().enabled_ofm());
                let _ast = parser.parse();
            })
        });

        group.bench_with_input(BenchmarkId::new("pulldown_cmark", case), &text, |b, input| {
            b.iter(|| {
                let parser = PdParser::new_ext(input, Options::all());
                for _ in parser {}
            })
        });

        group.bench_with_input(BenchmarkId::new("markdown_rs", case), &text, |b, input| {
            b.iter(|| {
                let _ast = markdown_rs::to_mdast(input, &MdRsParseOptions::gfm()).unwrap();
            })
        });
    }
    group.finish();
}

fn bench_parse_and_html(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_and_html");
    for (case, text) in dataset_cases() {
        group.bench_with_input(BenchmarkId::new("markdown", case), &text, |b, input| {
            b.iter(|| {
                let parser =
                    Parser::new_with_options(input, ParserOptions::default().enabled_ofm());
                let ast = parser.parse();
                let _html = ast.to_html();
            })
        });

        group.bench_with_input(BenchmarkId::new("pulldown_cmark", case), &text, |b, input| {
            b.iter(|| {
                let parser = PdParser::new_ext(input, Options::all());
                let mut output = String::new();
                html::push_html(&mut output, parser);
            })
        });

        group.bench_with_input(BenchmarkId::new("markdown_rs", case), &text, |b, input| {
            b.iter(|| {
                let _html = markdown_rs::to_html_with_options(input, &MdRsOptions::gfm()).unwrap();
            })
        });
    }
    group.finish();
}

criterion_group!(benches, bench_parse_only, bench_parse_and_html);
criterion_main!(benches);
