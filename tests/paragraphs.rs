use markdown::parser::Parser;

#[test]
fn case_219() {
    let input = r#"aaa

bbb"#;
    let output = r#"<p>aaa</p>
<p>bbb</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_220() {
    let input = r#"aaa
bbb

ccc
ddd"#;
    let output = r#"<p>aaa
bbb</p>
<p>ccc
ddd</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_221() {
    let input = r#"aaa


bbb"#;
    let output = r#"<p>aaa</p>
<p>bbb</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_222() {
    let input = r#"  aaa
 bbb"#;
    let output = r#"<p>aaa
bbb</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_223() {
    let input = r#"aaa
             bbb
                                       ccc"#;
    let output = r#"<p>aaa
bbb
ccc</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_224() {
    let input = r#"   aaa
bbb"#;
    let output = r#"<p>aaa
bbb</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_225() {
    let input = r#"    aaa
bbb"#;
    let output = r#"<pre><code>aaa
</code></pre>
<p>bbb</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_226() {
    let input = r#"aaa     
bbb"#;
    let output = r#"<p>aaa<br />
bbb</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
