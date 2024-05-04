use markdown::parser::Parser;
use regex::Regex;
use std::fs;
use std::path::Path;
use std::sync::OnceLock;

static RE: OnceLock<Regex> = OnceLock::new();

struct Example {
    markdown: String,
    html: String,
    section: String,
    number: usize,
}

fn extract_space_tests(testfile: &Path) -> Vec<Example> {
    let regex = RE.get_or_init(|| {
        Regex::new(r"(?m)^`{32} example\n(?<markdown>[\s\S]*?)^\.\n(?<html>[\s\S]*?)^`{32}$|^#{1,6} *(?<section>.*)$").unwrap()
    });
    let text = fs::read_to_string(testfile).unwrap().replace("\r\n", "\n");
    // 结果将是包含字符串中每个匹配项的开始和结束索引的元组的迭代器
    let result = regex.captures_iter(&text);
    let mut current_section = String::new();
    let mut examples = Vec::new();
    for captures in result {
        if let Some(section) = captures.name("section") {
            current_section = section.as_str().to_owned();
            continue;
        }
        let markdown = captures
            .name("markdown")
            .unwrap()
            .as_str()
            .trim_end()
            .replace('→', "\t");
        let html = captures
            .name("html")
            .unwrap()
            .as_str()
            .trim_end()
            .replace('→', "\t");
        examples.push(Example {
            section: current_section.to_owned(),
            markdown,
            html,
            number: examples.len(),
        })
    }
    examples
}

fn spec_test(testcase: &Example) -> bool {
    let ast = Parser::new(&testcase.markdown).parse();
    let html = ast.to_html();
    if html.replace('\n', "") != testcase.html.replace('\n', "") {
        eprintln!("⌈------------------------DEBUG INFO--------------------------");
        eprintln!("[AST]:\n {:?}", ast);
        eprintln!("[RAW]:\n {:?}", testcase.markdown);
        eprintln!("⁞------------------------ASSERT INFO-------------------------");
        eprintln!(" left: {:?}", html);
        eprintln!("right: {:?}", testcase.html);
        eprintln!("⌊------------------------------------------------------------");
        false
    } else {
        true
    }
}
fn spec_tests(testfile: &str) {
    let testcases = extract_space_tests(Path::new(&format!("tests/{testfile}")));
    println!("running {} tests", testcases.len());
    let mut passed = 0;
    let mut failed = 0;
    let start = std::time::Instant::now();
    for testcase in testcases.iter() {
        let panics = std::panic::catch_unwind(|| spec_test(&testcase));
        if let Ok(true) = panics {
            println!("test {}:{} ... ok", testcase.section, testcase.number);
            passed += 1;
        } else {
            eprintln!("test {}:{} ... failed", testcase.section, testcase.number);
            failed += 1;
        }
    }
    if failed > 0 {
        panic!(
            "tests result: {passed:?} passed; {failed:?} failed; finished in {}ms",
            start.elapsed().as_millis()
        )
    } else {
        println!(
            "tests result: {passed:?} passed; {failed:?} failed; finished in {}ms",
            start.elapsed().as_millis()
        )
    }
}

#[test]
fn setup_spec() {
    spec_tests("commonmark/spec.txt");
}
#[test]
fn setup_smart_punct() {
    spec_tests("commonmark/smart_punct.txt");
}
#[test]
fn setup_regression() {
    spec_tests("commonmark/regression.txt");
}

#[ignore]
#[test]
fn fix() {
    let input = r#"```
abc
```"#;
    let output = r#"<pre><code>abc
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
