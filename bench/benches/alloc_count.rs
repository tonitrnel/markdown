use markdown::{Parser, ParserOptions};
use std::alloc::{GlobalAlloc, Layout, System};
use std::fs;
use std::sync::atomic::{AtomicU64, Ordering};

struct CountingAlloc;

static ALLOCS: AtomicU64 = AtomicU64::new(0);
static REALLOCS: AtomicU64 = AtomicU64::new(0);
static DEALLOCS: AtomicU64 = AtomicU64::new(0);
static ALLOC_BYTES: AtomicU64 = AtomicU64::new(0);
static REALLOC_BYTES: AtomicU64 = AtomicU64::new(0);

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
        REALLOC_BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
        unsafe { System.realloc(ptr, layout, new_size) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        DEALLOCS.fetch_add(1, Ordering::Relaxed);
        unsafe {
            System.dealloc(ptr, layout);
        }
    }
}

fn reset() {
    ALLOCS.store(0, Ordering::Relaxed);
    REALLOCS.store(0, Ordering::Relaxed);
    DEALLOCS.store(0, Ordering::Relaxed);
    ALLOC_BYTES.store(0, Ordering::Relaxed);
    REALLOC_BYTES.store(0, Ordering::Relaxed);
}

fn parse_once(text: &str) {
    let parser = Parser::new_with_options(text, ParserOptions::default().enabled_ofm());
    let ast = parser.parse();
    let _html = ast.to_html();
}

fn main() {
    let text = fs::read_to_string("./bench/benches/_data.md").unwrap();

    // 先热身一次，把文件读取、一次性初始化的噪音降下去
    parse_once(&text);

    reset();

    for _ in 0..500 {
        parse_once(&text);
    }

    eprintln!(
        "allocs={} reallocs={} deallocs={} alloc_bytes={} realloc_bytes={}",
        ALLOCS.load(Ordering::Relaxed),
        REALLOCS.load(Ordering::Relaxed),
        DEALLOCS.load(Ordering::Relaxed),
        ALLOC_BYTES.load(Ordering::Relaxed),
        REALLOC_BYTES.load(Ordering::Relaxed),
    );
}
