use markdown::{Parser, ParserOptions};
use std::fs;

fn main() {
    let f_start = std::time::Instant::now();
    let text = fs::read_to_string("./benches/_data.md")
        .or_else(|_| fs::read_to_string("../../benches/_data.md"))
        .unwrap();
    let a = f_start.elapsed().as_micros() as f64 / 1000.0;
    let start = std::time::Instant::now();
    let parser = Parser::new_with_options(&text, ParserOptions::default().enabled_ofm());
    let b = start.elapsed().as_micros() as f64 / 1000.0;

    let start = std::time::Instant::now();
    let ast = parser.parse();
    let c = start.elapsed().as_micros() as f64 / 1000.0;

    let start = std::time::Instant::now();
    let _html = ast.to_html();
    let d = start.elapsed().as_micros() as f64 / 1000.0;
    let e = f_start.elapsed().as_micros() as f64 / 1000.0;
    println!("read file, elapsed: {a}ms",);
    println!("initial parser, elapsed: {b}ms",);
    println!("parse ast, elapsed: {c}ms",);
    println!("render html, elapsed: {d}ms",);
    println!("total elapsed: {e}ms",)
}
