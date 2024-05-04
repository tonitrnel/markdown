use markdown::parser::Parser;

#[test]
fn case_43() {
    let input = r#"***
---
___"#;
    let output = r#"<hr />
<hr />
<hr />"#;
    let ast = Parser::new(input).parse();
    // println!("AST:\n{ast:?}")
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_44() {
    let input = r#"+++"#;
    let output = r#"<p>+++</p>"#;
    let ast = Parser::new(input).parse();
    // println!("AST:\n{ast:?}")
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_45() {
    let input = r#"==="#;
    let output = r#"<p>===</p>"#;
    let ast = Parser::new(input).parse();
    // println!("AST:\n{ast:?}")
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_46() {
    let input = r#"--
**
__"#;
    let output = r#"<p>--
**
__</p>"#;
    let ast = Parser::new(input).parse();
    // println!("AST:\n{ast:?}")
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_47() {
    let input = r#" ***
  ***
   ***"#;
    let output = r#"<hr />
<hr />
<hr />"#;
    let ast = Parser::new(input).parse();
    // println!("AST:\n{ast:?}")
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_48() {
    let input = r#"    ***"#;
    let output = r#"<pre><code>***
</code></pre>"#;
    let ast = Parser::new(input).parse();
    // println!("AST:\n{ast:?}")
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_49() {
    let input = r#"Foo
    ***"#;
    let output = r#"<p>Foo
***</p>"#;
    let ast = Parser::new(input).parse();
    // println!("AST:\n{ast:?}")
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_50() {
    let input = r#"_____________________________________"#;
    let output = r#"<hr />"#;
    let ast = Parser::new(input).parse();
    // println!("AST:\n{ast:?}")
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_51() {
    let input = r#" - - -"#;
    let output = r#"<hr />"#;
    let ast = Parser::new(input).parse();
    // println!("AST:\n{ast:?}")
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_52() {
    let input = r#" **  * ** * ** * **"#;
    let output = r#"<hr />"#;
    let ast = Parser::new(input).parse();
    // println!("AST:\n{ast:?}")
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_53() {
    let input = r#"-     -      -      -"#;
    let output = r#"<hr />"#;
    let ast = Parser::new(input).parse();
    // println!("AST:\n{ast:?}")
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_54() {
    let input = r#"- - - -"#;
    let output = r#"<hr />"#;
    let ast = Parser::new(input).parse();
    // println!("AST:\n{ast:?}")
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_55() {
    let input = r#"_ _ _ _ a

a------

---a---"#;
    let output = r#"<p>_ _ _ _ a</p>
<p>a------</p>
<p>---a---</p>"#;
    let ast = Parser::new(input).parse();
    // println!("AST:\n{ast:?}")
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_56() {
    let input = r#" *-*"#;
    let output = r#"<p><em>-</em></p>"#;
    let ast = Parser::new(input).parse();
    // println!("AST:\n{ast:?}")
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_57() {
    let input = r#"- foo
***
- bar"#;
    let output = r#"<ul>
<li>foo</li>
</ul>
<hr />
<ul>
<li>bar</li>
</ul>"#;
    let ast = Parser::new(input).parse();
    // println!("AST:\n{ast:?}")
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_58() {
    let input = r#"Foo
***
bar"#;
    let output = r#"<p>Foo</p>
<hr />
<p>bar</p>"#;
    let ast = Parser::new(input).parse();
    // println!("AST:\n{ast:?}")
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_59() {
    let input = r#"Foo
---
bar"#;
    let output = r#"<h2>Foo</h2>
<p>bar</p>"#;
    let ast = Parser::new(input).parse();
    // println!("AST:\n{ast:?}")
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_60() {
    let input = r#"* Foo
* * *
* Bar"#;
    let output = r#"<ul>
<li>Foo</li>
</ul>
<hr />
<ul>
<li>Bar</li>
</ul>"#;
    let ast = Parser::new(input).parse();
    // println!("AST:\n{ast:?}")
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_61() {
    let input = r#"- Foo
- * * *"#;
    let output = r#"<ul>
<li>Foo</li>
<li>
<hr />
</li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
