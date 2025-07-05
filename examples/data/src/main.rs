use markdown::{Parser, ParserOptions};
use std::fs;

fn main() {
    let f_start = std::time::Instant::now();
    let text = fs::read_to_string("../../benches/_data.md").unwrap();
    println!(
        "read file, elapsed: {}ms",
        f_start.elapsed().as_micros() as f64 / 1000.0
    );
    let start = std::time::Instant::now();
    let parser = Parser::new_with_options(&text, ParserOptions::default().enabled_ofm());
    println!(
        "initial parser, elapsed: {}ms",
        start.elapsed().as_micros() as f64 / 1000.0
    );
    let start = std::time::Instant::now();
    let ast = parser.parse();
    println!(
        "parse ast, elapsed: {}ms",
        start.elapsed().as_micros() as f64 / 1000.0
    );
    let start = std::time::Instant::now();
    let _html = ast.to_html();
    println!(
        "render html, elapsed: {}ms",
        start.elapsed().as_micros() as f64 / 1000.0
    );
    println!(
        "total elapsed: {}ms",
        f_start.elapsed().as_micros() as f64 / 1000.0
    )
}
