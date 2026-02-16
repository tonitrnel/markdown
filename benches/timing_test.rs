use markdown::{Parser, ParserOptions};
use std::fs;

fn main() {
    let text = fs::read_to_string("./benches/_data.md").unwrap();
    // 足够多次迭代让 samply 采样
    for _ in 0..500 {
        let parser = Parser::new_with_options(&text, ParserOptions::default().enabled_ofm());
        let _ast = parser.parse();
    }
}
