use markdown::parser::Parser;

#[test]
fn case_1() {
    let input = r#"	foo	baz		bim"#;
    let output = r#"<pre><code>foo	baz		bim
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_2() {
    let input = r#"  	foo	baz		bim"#;
    let output = r#"<pre><code>foo	baz		bim
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_3() {
    let input = r#"    a	a
    ὐ	a"#;
    let output = r#"<pre><code>a	a
ὐ	a
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_4() {
    let input = r#"  - foo

	bar"#;
    let output = r#"<ul>
<li>
<p>foo</p>
<p>bar</p>
</li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_5() {
    let input = r#"- foo

		bar"#;
    let output = r#"<ul>
<li>
<p>foo</p>
<pre><code>  bar
</code></pre>
</li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_6() {
    let input = r#">		foo"#;
    let output = r#"<blockquote>
<pre><code>  foo
</code></pre>
</blockquote>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_7() {
    let input = r#"-		foo"#;
    let output = r#"<ul>
<li>
<pre><code>  foo
</code></pre>
</li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_8() {
    let input = r#"    foo
	bar"#;
    let output = r#"<pre><code>foo
bar
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_9() {
    let input = r#" - foo
   - bar
	 - baz"#;
    let output = r#"<ul>
<li>foo<ul>
<li>bar<ul>
<li>baz</li>
</ul></li>
</ul></li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_10() {
    let input = r#"#	Foo"#;
    let output = r#"<h1>Foo</h1>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_11() {
    let input = r#"*	*	*	"#;
    let output = r#"<hr />"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
