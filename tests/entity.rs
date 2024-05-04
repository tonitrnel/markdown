use markdown::parser::Parser;

#[test]
fn case_25() {
    let input = r#"&nbsp; &amp; &copy; &AElig; &Dcaron;
&frac34; &HilbertSpace; &DifferentialD;
&ClockwiseContourIntegral; &ngE;"#;
    let output = r#"<p>  &amp; © Æ Ď
¾ ℋ ⅆ
∲ ≧̸</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_26() {
    let input = r#"&#35; &#1234; &#992; &#0;"#;
    let output = r#"<p># Ӓ Ϡ �</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_27() {
    let input = r#"&#X22; &#XD06; &#xcab;"#;
    let output = r#"<p>&quot; ആ ಫ</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_28() {
    let input = r#"&nbsp &x; &#; &#x;
&#87654321;
&#abcdef0;
&ThisIsNotDefined; &hi?;"#;
    let output = r#"<p>&amp;nbsp &amp;x; &amp;#; &amp;#x;
&amp;#87654321;
&amp;#abcdef0;
&amp;ThisIsNotDefined; &amp;hi?;</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_29() {
    let input = r#"&copy"#;
    let output = r#"<p>&amp;copy</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_30() {
    let input = r#"&MadeUpEntity;"#;
    let output = r#"<p>&amp;MadeUpEntity;</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_31() {
    let input = r#"<a href="&ouml;&ouml;.html">"#;
    let output = r#"<a href="&ouml;&ouml;.html">"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_32() {
    let input = r#"[foo](/f&ouml;&ouml; "f&ouml;&ouml;")"#;
    let output = r#"<p><a href="/f%C3%B6%C3%B6" title="föö">foo</a></p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_33() {
    let input = r#"[foo]

[foo]: /f&ouml;&ouml; "f&ouml;&ouml;""#;
    let output = r#"<p><a href="/f%C3%B6%C3%B6" title="föö">foo</a></p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_34() {
    let input = r#"``` f&ouml;&ouml;
foo
```"#;
    let output = r#"<pre><code class="language-föö">foo
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_35() {
    let input = r#"`f&ouml;&ouml;`"#;
    let output = r#"<p><code>f&amp;ouml;&amp;ouml;</code></p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_36() {
    let input = r#"    f&ouml;f&ouml;"#;
    let output = r#"<pre><code>f&amp;ouml;f&amp;ouml;
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_37() {
    let input = r#"&#42;foo&#42;
*foo*"#;
    let output = r#"<p>*foo*
<em>foo</em></p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_38() {
    let input = r#"&#42; foo

* foo"#;
    let output = r#"<p>* foo</p>
<ul>
<li>foo</li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_39() {
    let input = r#"foo&#10;&#10;bar"#;
    let output = r#"<p>foo

bar</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_40() {
    let input = r#"&#9;foo"#;
    let output = r#"<p>	foo</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_41() {
    let input = r#"[a](url &quot;tit&quot;)"#;
    let output = r#"<p>[a](url &quot;tit&quot;)</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
