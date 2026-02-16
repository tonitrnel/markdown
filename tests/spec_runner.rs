use colored::*;
use markdown::parser::{Parser, ParserOptions};
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

static RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)^`{32} example\n(?<markdown>[\s\S]*?)^\.\n(?<html>[\s\S]*?)^`{32}$|^#{1,6} *(?<section>.*)$").unwrap()
});

struct Example {
    markdown: String,
    html: String,
    section: String,
    number: usize,
    markdown_line: usize,
    html_line: usize,
}

fn trim_single_trailing_newline(s: &str) -> &str {
    s.strip_suffix('\n').unwrap_or(s)
}

fn list_txt_files(root: &str) -> Vec<PathBuf> {
    let mut files = fs::read_dir(Path::new(root))
        .unwrap_or_else(|e| panic!("failed to read dir {root}: {e}"))
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "txt") {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    files.sort();
    files
}

fn extract_spec_tests(testfile: &Path) -> Vec<Example> {
    let text = fs::read_to_string(testfile)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", testfile.display()))
        .replace("\r\n", "\n");
    let result = RE.captures_iter(&text);
    let mut current_section = String::new();
    let mut examples = Vec::new();
    for captures in result {
        if let Some(section) = captures.name("section") {
            current_section = section.as_str().to_owned();
            continue;
        }
        let markdown = trim_single_trailing_newline(captures.name("markdown").unwrap().as_str())
            .replace('→', "\t");
        let html = trim_single_trailing_newline(captures.name("html").unwrap().as_str())
            .replace('→', "\t");
        let markdown_match = captures.name("markdown").unwrap();
        let html_match = captures.name("html").unwrap();
        let markdown_line = 1 + text[..markdown_match.start()]
            .bytes()
            .filter(|b| *b == b'\n')
            .count();
        let html_line = 1 + text[..html_match.start()]
            .bytes()
            .filter(|b| *b == b'\n')
            .count();
        examples.push(Example {
            section: current_section.to_owned(),
            markdown,
            html,
            number: examples.len(),
            markdown_line,
            html_line,
        });
    }
    examples
}

fn run_example(testfile: &Path, testcase: &Example, options: ParserOptions) -> Result<(), String> {
    let ast = Parser::new_with_options(&testcase.markdown, options).parse();
    let html = ast.to_html();
    if html.replace('\n', "") != testcase.html.replace('\n', "") {
        Err(format!(
            "{}\n[FILE]: {}:{}\n[EXPECT]: {}:{}\n[AST]:\n {:?}\n[RAW]:\n {:?}\n{}\n left: {:?}\nright: {:?}\n{}",
            "⌈------------------------DEBUG INFO--------------------------".bright_black(),
            testfile.display(),
            testcase.markdown_line,
            testfile.display(),
            testcase.html_line,
            ast,
            testcase.markdown,
            "⁞------------------------ASSERT INFO-------------------------".bright_black(),
            html,
            testcase.html,
            "⌊------------------------------------------------------------".bright_black()
        ))
    } else {
        Ok(())
    }
}

pub fn spec_suite(root: &str, options: ParserOptions, fail_fast: bool) {
    run_suite(root, fail_fast, move |_| options.clone());
}

pub fn spec_suite_with_flavor(root: &str, flavor: Flavor, fail_fast: bool) {
    run_suite(root, fail_fast, move |file| options_for(flavor, file));
}

fn run_suite<F>(root: &str, fail_fast: bool, options_for_file: F)
where
    F: Fn(&Path) -> ParserOptions,
{
    let files = list_txt_files(root);
    let start = std::time::Instant::now();
    let mut passed = 0usize;
    let mut failed = 0usize;
    for file in files {
        let cases = extract_spec_tests(&file);
        let options = options_for_file(&file);
        println!("running {} tests from {}", cases.len(), file.display());
        for testcase in &cases {
            let panics = std::panic::catch_unwind(|| run_example(&file, testcase, options.clone()));
            match panics {
                Ok(Ok(())) => {
                    println!(
                        "test {}:{}:{} ... {}",
                        file.display(),
                        testcase.section,
                        testcase.number,
                        "ok".green()
                    );
                    passed += 1;
                }
                Ok(Err(debug_info)) => {
                    eprintln!(
                        "test {}:{}:{} ... {}",
                        file.display(),
                        testcase.section,
                        testcase.number,
                        "FAILED".red()
                    );
                    eprintln!("{debug_info}");
                    failed += 1;
                    if fail_fast {
                        panic!(
                            "fail-fast: first failure at {}:{}:{}",
                            file.display(),
                            testcase.section,
                            testcase.number
                        );
                    }
                }
                Err(_) => {
                    eprintln!(
                        "test {}:{}:{} ... {}",
                        file.display(),
                        testcase.section,
                        testcase.number,
                        "FAILED (panic)".red()
                    );
                    failed += 1;
                    if fail_fast {
                        panic!(
                            "fail-fast: first failure at {}:{}:{}",
                            file.display(),
                            testcase.section,
                            testcase.number
                        );
                    }
                }
            }
        }
    }
    if failed > 0 {
        panic!(
            "tests result: {passed:?} {}; {failed:?} {}; finished in {}ms",
            "passed",
            "failed".red(),
            start.elapsed().as_millis()
        );
    } else {
        println!(
            "tests result: {passed:?} {}; {failed:?} {}; finished in {}ms",
            "passed".green(),
            "failed",
            start.elapsed().as_millis()
        );
    }
}

pub fn fail_fast_from_env() -> bool {
    std::env::var("SPEC_FAIL_FAST")
        .map(|v| {
            let v = v.trim();
            v == "1"
                || v.eq_ignore_ascii_case("true")
                || v.eq_ignore_ascii_case("yes")
                || v.eq_ignore_ascii_case("on")
        })
        .unwrap_or(false)
}

#[derive(Clone, Copy)]
pub enum Flavor {
    CommonMark,
    Github,
    Obsidian,
}

fn options_for(flavor: Flavor, testfile: &Path) -> ParserOptions {
    let base_options = match flavor {
        Flavor::CommonMark => ParserOptions::default(),
        Flavor::Github => ParserOptions::default().enabled_gfm(),
        Flavor::Obsidian => ParserOptions::default().enabled_ofm(),
    };

    // Enable smart punctuation for smart_punct.txt test files
    if testfile
        .file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|n| n.contains("smart_punct"))
    {
        base_options.enabled_smart_punctuation()
    } else {
        base_options
    }
}
