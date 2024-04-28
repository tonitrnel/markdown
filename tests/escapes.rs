use markdown::parser::Parser;

#[test]
fn case_12() {
    let input = r#"\!\"\#\$\%\&\'\(\)\*\+\,\-\.\/\:\;\<\=\>\?\@\[\\\]\^\_\`\{\|\}\~"#;
    let output = r#"<p>!&quot;#$%&amp;'()*+,-./:;&lt;=&gt;?@[\]^_`{|}~</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_13() {
    let input = r#"\	\A\a\ \3\φ\«"#;
    let output = r#"<p>\	\A\a\ \3\φ\«</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_14() {
    let input = r#"\*not emphasized*
\<br/> not a tag
\[not a link](/foo)
\`not code`
1\. not a list
\* not a list
\# not a heading
\[foo]: /url "not a reference"
\&ouml; not a character entity"#;
    let output = r#"<p>*not emphasized*
&lt;br/&gt; not a tag
[not a link](/foo)
`not code`
1. not a list
* not a list
# not a heading
[foo]: /url &quot;not a reference&quot;
&amp;ouml; not a character entity</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_15() {
    let input = r#"\\*emphasis*"#;
    let output = r#"<p>\<em>emphasis</em></p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_16() {
    let input = r#"foo\
bar"#;
    let output = r#"<p>foo<br />
bar</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_17() {
    let input = r#"`` \[\` ``"#;
    let output = r#"<p><code>\[\`</code></p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_19() {
    let input = r#"~~~
\[\]
~~~"#;
    let output = r#"<pre><code>\[\]
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_20() {
    let input = r#"<https://example.com?find=\*>"#;
    let output =
        r#"<p><a href="https://example.com?find=%5C*">https://example.com?find=\*</a></p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_21() {
    let input = r#"<a href="/bar\/)">"#;
    let output = r#"<a href="/bar\/)">"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_22() {
    let input = r#"[foo](/bar\* "ti\*tle")"#;
    let output = r#"<p><a href="/bar*" title="ti*tle">foo</a></p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_23() {
    let input = r#"[foo]

[foo]: /bar\* "ti\*tle""#;
    let output = r#"<p><a href="/bar*" title="ti*tle">foo</a></p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_24() {
    let input = r#"``` foo\+bar
foo
```"#;
    let output = r#"<pre><code class="language-foo+bar">foo
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
