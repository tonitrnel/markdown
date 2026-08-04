/// 各优化阶段验收门槛所引用的热点 lane；fixture 生成器与 alloc_count 共用。
use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use ptdgrp_markdown::{InlineSelection, Parser, ParserOptions, VisitControl};

mod hotspot_cases;

fn parse_only(text: &str) {
    let parser = Parser::new_with_options(text, ParserOptions::default().enabled_ofm());
    let _ast = parser.parse();
}

/// 选择性路径：扫描 + 语义准备 + 选择约 10% 顶层 Block 后物化（F3 记录 lane）。
fn parse_selective_10pct(text: &str) {
    let mut top_level = Vec::new();
    let phase = Parser::new_with_options(text, ParserOptions::default().enabled_ofm())
        .parse_blocks_with(
            |_| true,
            |event| {
                top_level.push(event.node_id());
                VisitControl::Continue
            },
        )
        .unwrap()
        .prepare_semantic_targets()
        .unwrap();
    let mut selection = InlineSelection::default();
    for id in top_level.iter().step_by(10) {
        selection.select(*id);
    }
    let _output = phase.parse_selected_inlines(selection);
}

fn hotspots(c: &mut Criterion) {
    let cases = hotspot_cases::all();

    let mut group = c.benchmark_group("parse_hotspots");
    for (name, text) in &cases {
        group.throughput(Throughput::Bytes(text.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(name), text, |b, text| {
            b.iter(|| parse_only(black_box(text)));
        });
    }
    group.finish();

    // 选择性 vs 完整解析：同一 many_short_paragraphs fixture，只记录不设门槛
    let selective_fixture = hotspot_cases::all()
        .into_iter()
        .find(|(name, _)| *name == "many_short_paragraphs")
        .map(|(_, text)| text)
        .expect("many_short_paragraphs fixture exists");
    let mut group = c.benchmark_group("selective_parse");
    group.throughput(Throughput::Bytes(selective_fixture.len() as u64));
    group.bench_with_input(
        BenchmarkId::from_parameter("full"),
        &selective_fixture,
        |b, text| {
            b.iter(|| parse_only(black_box(text)));
        },
    );
    group.bench_with_input(
        BenchmarkId::from_parameter("select_10pct"),
        &selective_fixture,
        |b, text| {
            b.iter(|| parse_selective_10pct(black_box(text)));
        },
    );
    group.finish();
}

criterion_group!(benches, hotspots);
criterion_main!(benches);
