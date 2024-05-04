use markdown::parser::Parser;

#[test]
fn case_227() {
    let input = r#"  

aaa
  

# aaa"#;
    let output = r#"<p>aaa</p>
<h1>aaa</h1>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
