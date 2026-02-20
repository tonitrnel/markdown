use markdown_local::{Parser, ParserOptions};
use std::hint::black_box;
use std::path::PathBuf;
use std::time::Instant;

fn bench<F>(name: &str, iterations: usize, mut f: F)
where
    F: FnMut(),
{
    for _ in 0..20 {
        f();
    }

    let start = Instant::now();
    for _ in 0..iterations {
        f();
    }
    let elapsed = start.elapsed();
    let ms_per_op = (elapsed.as_secs_f64() * 1000.0) / iterations as f64;
    println!("{name},{ms_per_op:.6}");
}

fn main() {
    let label = std::env::var("POLYGLOT_DATA_LABEL").unwrap_or_else(|_| "default_data".to_string());
    let data_path = std::env::var("POLYGLOT_DATA_FILE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("bench/third_party/polyglot/data/data.md"));
    let text = std::fs::read_to_string(&data_path).expect("failed to read input data");
    let iterations: usize = std::env::var("POLYGLOT_ITERS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(800);

    bench(&format!("rust_markdown_parse_only__{label}"), iterations, || {
        let parser = Parser::new_with_options(&text, ParserOptions::default().enabled_ofm());
        let ast = parser.parse();
        black_box(ast);
    });

    bench(&format!("rust_markdown_parse_and_html__{label}"), iterations, || {
        let parser = Parser::new_with_options(&text, ParserOptions::default().enabled_ofm());
        let ast = parser.parse();
        let html = ast.to_html();
        black_box(html);
    });
}
