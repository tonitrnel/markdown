use comrak;
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use markdown_local::{Parser, ParserOptions};
use pulldown_cmark::{Options as PulldownOptions, Parser as PulldownParser};
use rushdown;
use rushdown::parser::ParserExtension;

#[derive(Clone, Copy)]
enum Lane {
    CommonMark,
    SharedGfm,
    OfmProduct,
}

impl Lane {
    const ALL: [Self; 3] = [Self::CommonMark, Self::SharedGfm, Self::OfmProduct];

    const fn name(self) -> &'static str {
        match self {
            Self::CommonMark => "commonmark",
            Self::SharedGfm => "shared_gfm",
            Self::OfmProduct => "ofm_product",
        }
    }
}

struct Dataset {
    name: &'static str,
    text: &'static str,
}

fn datasets() -> [Dataset; 2] {
    [
        Dataset {
            name: "curated",
            text: include_str!("../../../fixtures/curated/_data.md"),
        },
        Dataset {
            name: "markdown_it_corpus",
            text: include_str!("../../../fixtures/corpora/markdown-it-corpus.md"),
        },
    ]
}

fn local_options(lane: Lane) -> ParserOptions {
    match lane {
        Lane::CommonMark => ParserOptions::default(),
        Lane::SharedGfm => ParserOptions::default().enabled_gfm(),
        Lane::OfmProduct => ParserOptions::default().enabled_ofm(),
    }
}

fn comrak_options(lane: Lane) -> Option<comrak::Options<'static>> {
    match lane {
        Lane::CommonMark => Some(comrak::Options::default()),
        Lane::SharedGfm => {
            let mut options = comrak::Options::default();
            options.extension.table = true;
            options.extension.strikethrough = true;
            options.extension.tasklist = true;
            Some(options)
        }
        Lane::OfmProduct => None,
    }
}

fn pulldown_options(lane: Lane) -> Option<PulldownOptions> {
    match lane {
        Lane::CommonMark => Some(PulldownOptions::empty()),
        Lane::SharedGfm => Some(
            PulldownOptions::ENABLE_TABLES
                | PulldownOptions::ENABLE_STRIKETHROUGH
                | PulldownOptions::ENABLE_TASKLISTS,
        ),
        Lane::OfmProduct => None,
    }
}

fn bench_parse_only(c: &mut Criterion) {
    for lane in Lane::ALL {
        let mut group = c.benchmark_group(format!("parse_only/{}", lane.name()));
        for dataset in datasets() {
            let options = local_options(lane);
            group.bench_with_input(
                BenchmarkId::new("markdown", dataset.name),
                &dataset.text,
                |b, input| {
                    b.iter(|| {
                        let parser = Parser::new_with_options(input, options.clone());
                        black_box(parser.parse().unwrap());
                    })
                },
            );

            if let Some(options) = pulldown_options(lane) {
                group.bench_with_input(
                    BenchmarkId::new("pulldown_cmark_events", dataset.name),
                    &dataset.text,
                    |b, input| {
                        b.iter(|| {
                            for event in PulldownParser::new_ext(input, options) {
                                black_box(event);
                            }
                        })
                    },
                );
            }

            // comrak：构建完整 AST（arena），与本地完整 AST 工作量等价
            if let Some(options) = comrak_options(lane) {
                group.bench_with_input(
                    BenchmarkId::new("comrak_ast", dataset.name),
                    &dataset.text,
                    |b, input| {
                        b.iter(|| {
                            let arena = comrak::Arena::new();
                            black_box(comrak::parse_document(&arena, input, &options));
                        })
                    },
                );
            }

            // rushdown：完整 AST（Arena + NodeRef），只比较 CommonMark 和共享 GFM。
            if matches!(lane, Lane::CommonMark | Lane::SharedGfm) {
                let parser = match lane {
                    Lane::CommonMark => {
                        rushdown::parser::Parser::with_options(rushdown::parser::Options::default())
                    }
                    Lane::SharedGfm => rushdown::parser::Parser::with_extensions(
                        rushdown::parser::Options::default(),
                        rushdown::parser::gfm_table()
                            .and(rushdown::parser::gfm_strikethrough())
                            .and(rushdown::parser::gfm_task_list_item()),
                    ),
                    Lane::OfmProduct => unreachable!(),
                };
                group.bench_with_input(
                    BenchmarkId::new("rushdown_ast", dataset.name),
                    &dataset.text,
                    |b, input| {
                        b.iter(|| {
                            let mut reader = rushdown::text::BasicReader::new(input);
                            black_box(parser.parse(&mut reader));
                        })
                    },
                );
            }
        }
        group.finish();
    }
}

fn bench_parse_and_html(c: &mut Criterion) {
    for lane in Lane::ALL {
        let mut group = c.benchmark_group(format!("parse_and_html/{}", lane.name()));
        for dataset in datasets() {
            let options = local_options(lane);
            group.bench_with_input(
                BenchmarkId::new("markdown", dataset.name),
                &dataset.text,
                |b, input| {
                    b.iter(|| {
                        let parser = Parser::new_with_options(input, options.clone());
                        let document = black_box(parser.parse().unwrap());
                        black_box(document.to_html());
                    })
                },
            );

            if let Some(options) = pulldown_options(lane) {
                group.bench_with_input(
                    BenchmarkId::new("pulldown_cmark_html", dataset.name),
                    &dataset.text,
                    |b, input| {
                        b.iter(|| {
                            let mut html = String::new();
                            pulldown_cmark::html::push_html(
                                &mut html,
                                PulldownParser::new_ext(input, options),
                            );
                            black_box(html);
                        })
                    },
                );
            }

            // rushdown：CommonMark 通道；parser/renderer 构造移出计时循环
            if matches!(lane, Lane::CommonMark) {
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
                group.bench_with_input(
                    BenchmarkId::new("rushdown_html", dataset.name),
                    &dataset.text,
                    |b, input| {
                        b.iter(|| {
                            let mut output = String::new();
                            markdown_to_html(&mut output, input).unwrap();
                            black_box(output);
                        });
                    },
                );
            }
            if matches!(lane, Lane::SharedGfm) {
                let markdown_to_html = rushdown::new_markdown_to_html(
                    rushdown::parser::Options::default(),
                    rushdown::renderer::html::Options {
                        allows_unsafe: true,
                        xhtml: true,
                        ..rushdown::renderer::html::Options::default()
                    },
                    rushdown::parser::gfm_table()
                        .and(rushdown::parser::gfm_strikethrough())
                        .and(rushdown::parser::gfm_task_list_item()),
                    rushdown::renderer::html::NO_EXTENSIONS,
                );
                group.bench_with_input(
                    BenchmarkId::new("rushdown_html", dataset.name),
                    &dataset.text,
                    |b, input| {
                        b.iter(|| {
                            let mut output = String::new();
                            markdown_to_html(&mut output, input).unwrap();
                            black_box(output);
                        });
                    },
                );
            }
            if let Some(options) = comrak_options(lane) {
                group.bench_with_input(
                    BenchmarkId::new("comrak_html", dataset.name),
                    &dataset.text,
                    |b, input| {
                        b.iter(|| {
                            black_box(comrak::markdown_to_html(input, &options));
                        })
                    },
                );
            }
        }
        group.finish();
    }
}

criterion_group!(benches, bench_parse_only, bench_parse_and_html);
criterion_main!(benches);
