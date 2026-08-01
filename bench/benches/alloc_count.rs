use markdown::{Parser, ParserOptions};
use std::alloc::{GlobalAlloc, Layout, System};
use std::hint::black_box;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

mod hotspot_cases;

const PARSES: usize = 500;
const FIXTURE: &str = include_str!("../fixtures/curated/_data.md");

struct CountingAlloc;

static ALLOCS: AtomicU64 = AtomicU64::new(0);
static REALLOCS: AtomicU64 = AtomicU64::new(0);
static DEALLOCS: AtomicU64 = AtomicU64::new(0);
static ALLOC_BYTES: AtomicU64 = AtomicU64::new(0);
static REALLOC_BYTES: AtomicU64 = AtomicU64::new(0);
static DEALLOC_BYTES: AtomicU64 = AtomicU64::new(0);

#[global_allocator]
static GLOBAL: CountingAlloc = CountingAlloc;

unsafe impl GlobalAlloc for CountingAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Ordering::Relaxed);
        ALLOC_BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
        unsafe { System.alloc(layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        REALLOCS.fetch_add(1, Ordering::Relaxed);
        REALLOC_BYTES.fetch_add(new_size as u64, Ordering::Relaxed);
        unsafe { System.realloc(ptr, layout, new_size) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        DEALLOCS.fetch_add(1, Ordering::Relaxed);
        DEALLOC_BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
        unsafe { System.dealloc(ptr, layout) }
    }
}

fn reset_counts() {
    ALLOCS.store(0, Ordering::Relaxed);
    REALLOCS.store(0, Ordering::Relaxed);
    DEALLOCS.store(0, Ordering::Relaxed);
    ALLOC_BYTES.store(0, Ordering::Relaxed);
    REALLOC_BYTES.store(0, Ordering::Relaxed);
    DEALLOC_BYTES.store(0, Ordering::Relaxed);
}

fn parse_once(text: &str) -> u64 {
    let started = Instant::now();
    let parser = Parser::new_with_options(text, ParserOptions::default().enabled_ofm());
    black_box(parser.parse());
    started.elapsed().as_nanos() as u64
}

fn block_only_once(text: &str) -> u64 {
    use markdown::selective::VisitControl;
    let started = Instant::now();
    let phase = Parser::new_with_options(text, ParserOptions::default().enabled_ofm())
        .parse_blocks_with(|_| true, |_| VisitControl::Continue);
    black_box(&phase);
    drop(phase);
    started.elapsed().as_nanos() as u64
}

fn session_prepare_once(text: &str) -> u64 {
    use markdown::selective::VisitControl;
    let started = Instant::now();
    let phase = Parser::new_with_options(text, ParserOptions::default().enabled_ofm())
        .parse_blocks_with(|_| true, |_| VisitControl::Continue)
        .prepare_semantic_targets();
    black_box(&phase);
    drop(phase);
    started.elapsed().as_nanos() as u64
}

fn per_parse(value: u64) -> f64 {
    value as f64 / PARSES as f64
}

fn median_ns(samples: &mut [u64]) -> u64 {
    samples.sort_unstable();
    let upper = samples.len() / 2;
    let lower = upper - 1;
    (samples[lower] + samples[upper]) / 2
}

fn measure_with(fixture_label: Option<&str>, text: &str, run: fn(&str) -> u64) {
    // Warm up parser code and allocator state before measurement.
    run(text);

    let mut elapsed_ns = Vec::with_capacity(PARSES);
    reset_counts();
    for _ in 0..PARSES {
        elapsed_ns.push(run(text));
    }

    // 先把计数器读入局部变量，再做任何格式化分配，避免输出自身污染读数。
    let allocs = per_parse(ALLOCS.load(Ordering::Relaxed));
    let reallocs = per_parse(REALLOCS.load(Ordering::Relaxed));
    let deallocs = per_parse(DEALLOCS.load(Ordering::Relaxed));
    let alloc_bytes = per_parse(ALLOC_BYTES.load(Ordering::Relaxed));
    let realloc_bytes = per_parse(REALLOC_BYTES.load(Ordering::Relaxed));
    let dealloc_bytes = per_parse(DEALLOC_BYTES.load(Ordering::Relaxed));
    let median_us = median_ns(&mut elapsed_ns) as f64 / 1_000.0;
    let prefix = match fixture_label {
        Some(name) => format!("fixture={name} "),
        None => String::new(),
    };
    println!(
        "{prefix}parses={PARSES} allocs_per_parse={allocs:.2} reallocs_per_parse={reallocs:.2} \
         deallocs_per_parse={deallocs:.2} alloc_bytes_per_parse={alloc_bytes:.2} \
         realloc_bytes_per_parse={realloc_bytes:.2} dealloc_bytes_per_parse={dealloc_bytes:.2} median_us={median_us:.2}",
    );
}

fn measure(fixture_label: Option<&str>, text: &str) {
    measure_with(fixture_label, text, parse_once);
}

fn measure_render(text: &str) {
    let document = Parser::new_with_options(text, ParserOptions::default().enabled_ofm()).parse();
    let warmup = document.to_html();
    let output_len = warmup.len();
    let output_capacity = warmup.capacity();

    let mut elapsed_ns = Vec::with_capacity(PARSES);
    reset_counts();
    for _ in 0..PARSES {
        let started = Instant::now();
        black_box(document.to_html());
        elapsed_ns.push(started.elapsed().as_nanos() as u64);
    }

    let allocs = per_parse(ALLOCS.load(Ordering::Relaxed));
    let reallocs = per_parse(REALLOCS.load(Ordering::Relaxed));
    let alloc_bytes = per_parse(ALLOC_BYTES.load(Ordering::Relaxed));
    let realloc_bytes = per_parse(REALLOC_BYTES.load(Ordering::Relaxed));
    let median_us = median_ns(&mut elapsed_ns) as f64 / 1_000.0;
    println!(
        "[DEBUG-render-a4f2] allocs_per_render={allocs:.2} reallocs_per_render={reallocs:.2} \
         alloc_bytes_per_render={alloc_bytes:.2} realloc_bytes_per_render={realloc_bytes:.2} \
         output_len={output_len} output_capacity={output_capacity} median_us={median_us:.2}",
    );
}

fn main() {
    // 首行保持既有无前缀格式（既有记录兼容）；合成 lane 逐行追加。
    measure(None, FIXTURE);
    measure_render(FIXTURE);
    // v2C 会话变体（C4）
    measure_with(Some("session_block_only/_data"), FIXTURE, block_only_once);
    measure_with(Some("session_prepare/_data"), FIXTURE, session_prepare_once);
    for (name, text) in hotspot_cases::all() {
        measure(Some(name), &text);
    }
}
