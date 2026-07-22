use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use markdown_local::{Parser, ParserOptions};
use markdown_rs::Options as MdRsOptions;
use pulldown_cmark::{Options, Parser as PdParser};
use rushdown;

fn dataset_cases() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "default_data",
            include_str!("../../../fixtures/curated/_data.md"),
        ),
        (
            "markdown_it_corpus",
            include_str!("../../../fixtures/corpora/markdown-it-corpus.md"),
        ),
    ]
}

// fn bench_parse_only(c: &mut Criterion) {
//     let mut group = c.benchmark_group("parse_only");
//     for (case, text) in dataset_cases() {
//         group.bench_with_input(BenchmarkId::new("markdown", case), &text, |b, input| {
//             b.iter(|| {
//                 let parser =
//                     Parser::new_with_options(input, ParserOptions::default().enabled_ofm());
//                 let _ast = parser.parse();
//             })
//         });

//         group.bench_with_input(
//             BenchmarkId::new("pulldown_cmark", case),
//             &text,
//             |b, input| {
//                 b.iter(|| {
//                     let parser = PdParser::new_ext(input, Options::all());
//                     for _ in parser {}
//                 })
//             },
//         );

//         group.bench_with_input(BenchmarkId::new("markdown_rs", case), &text, |b, input| {
//             b.iter(|| {
//                 let _ast = markdown_rs::to_mdast(input, &MdRsParseOptions::gfm()).unwrap();
//             })
//         });
//     }
//     group.finish();
// }

fn bench_compare(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare");
    for (case, text) in dataset_cases() {
        group.bench_with_input(BenchmarkId::new("self", case), &text, |b, input| {
            b.iter(|| {
                let parser =
                    Parser::new_with_options(input, ParserOptions::default().enabled_ofm());
                let ast = parser.parse();
                let _html = ast.to_html();
            })
        });

        group.bench_with_input(
            BenchmarkId::new("pulldown_cmark", case),
            &text,
            |b, input| {
                b.iter(|| {
                    let parser = PdParser::new_ext(input, Options::all());
                    let mut output = String::new();
                    pulldown_cmark::html::push_html(&mut output, parser);
                })
            },
        );

        group.bench_with_input(BenchmarkId::new("markdown_rs", case), &text, |b, input| {
            b.iter(|| {
                let _html = markdown_rs::to_html_with_options(input, &MdRsOptions::gfm()).unwrap();
            })
        });
        group.bench_with_input(BenchmarkId::new("rushdown", case), &text, |b, input| {
            b.iter(|| {
                let markdown_to_html = rushdown::new_markdown_to_html(
                    rushdown::parser::Options::default(),
                    rushdown::renderer::html::Options {
                        allows_unsafe: true,
                        xhtml: true,
                        ..rushdown::renderer::html::Options::default()
                    },
                    rushdown::parser::NO_EXTENSIONS,
                    rushdown::renderer::html::NO_EXTENSIONS,
                );
                let mut output = String::new();
                markdown_to_html(&mut output, input).unwrap();
            })
        });
        group.bench_with_input(BenchmarkId::new("comark", case), &text, |b, input| {
            b.iter(|| {
                let _ = comrak::markdown_to_html(input, &comrak::Options::default());
            })
        });
    }
    group.finish();
}

criterion_group!(benches, bench_compare);
criterion_main!(benches);
