use jieba_rs::{Jieba, KeywordExtract, TfIdf};
use std::collections::HashSet;
use std::time::Instant;

static TXT: &str = include_str!("story.txt");

fn main() {
    let start = Instant::now();
    let jieba = Jieba::new();
    let extractor = TfIdf::default();
    let mut set = HashSet::new();
    let mut top_k = vec![];
    for line in TXT.lines() {
        let keywords = extractor.extract_keywords(&jieba, line, 12, vec![]);
        for keyword in keywords {
            if set.contains(&keyword.keyword) {
                continue;
            }
            set.insert(keyword.keyword.clone());
            top_k.push(keyword);
        }
    }
    top_k.sort_by(|a, b| a.weight.total_cmp(&b.weight));
    println!(
        "finished, elapsed: {}ms\n({}){:?}",
        start.elapsed().as_millis(),
        top_k.len(),
        top_k
    );
}
