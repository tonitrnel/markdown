use markdown::parser::Parser;

#[test]
fn case_228() {
    let input = r#"> # Foo
> bar
> baz"#;
    let output = r#"<blockquote>
<h1>Foo</h1>
<p>bar
baz</p>
</blockquote>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_229() {
    let input = r#"># Foo
>bar
> baz"#;
    let output = r#"<blockquote>
<h1>Foo</h1>
<p>bar
baz</p>
</blockquote>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_230() {
    let input = r#"   > # Foo
   > bar
 > baz"#;
    let output = r#"<blockquote>
<h1>Foo</h1>
<p>bar
baz</p>
</blockquote>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_231() {
    let input = r#"    > # Foo
    > bar
    > baz"#;
    let output = r#"<pre><code>&gt; # Foo
&gt; bar
&gt; baz
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_232() {
    let input = r#"> # Foo
> bar
baz"#;
    let output = r#"<blockquote>
<h1>Foo</h1>
<p>bar
baz</p>
</blockquote>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_233() {
    let input = r#"> bar
baz
> foo"#;
    let output = r#"<blockquote>
<p>bar
baz
foo</p>
</blockquote>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_234() {
    let input = r#"> foo
---"#;
    let output = r#"<blockquote>
<p>foo</p>
</blockquote>
<hr />"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_235() {
    let input = r#"> - foo
- bar"#;
    let output = r#"<blockquote>
<ul>
<li>foo</li>
</ul>
</blockquote>
<ul>
<li>bar</li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_236() {
    let input = r#">     foo
    bar"#;
    let output = r#"<blockquote>
<pre><code>foo
</code></pre>
</blockquote>
<pre><code>bar
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_237() {
    let input = r#"> ```
foo
```"#;
    let output = r#"<blockquote>
<pre><code></code></pre>
</blockquote>
<p>foo</p>
<pre><code></code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_238() {
    let input = r#"> foo
    - bar"#;
    let output = r#"<blockquote>
<p>foo
- bar</p>
</blockquote>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_239() {
    let input = r#">"#;
    let output = r#"<blockquote>
</blockquote>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_240() {
    let input = r#">
>  
>"#;
    let output = r#"<blockquote>
</blockquote>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_241() {
    let input = r#">
> foo
>"#;
    let output = r#"<blockquote>
<p>foo</p>
</blockquote>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_242() {
    let input = r#"> foo

> bar"#;
    let output = r#"<blockquote>
<p>foo</p>
</blockquote>
<blockquote>
<p>bar</p>
</blockquote>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_243() {
    let input = r#"> foo
> bar"#;
    let output = r#"<blockquote>
<p>foo
bar</p>
</blockquote>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_244() {
    let input = r#"> foo
>
> bar"#;
    let output = r#"<blockquote>
<p>foo</p>
<p>bar</p>
</blockquote>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_245() {
    let input = r#"foo
> bar"#;
    let output = r#"<p>foo</p>
<blockquote>
<p>bar</p>
</blockquote>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_246() {
    let input = r#"> aaa
***
> bbb"#;
    let output = r#"<blockquote>
<p>aaa</p>
</blockquote>
<hr />
<blockquote>
<p>bbb</p>
</blockquote>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_247() {
    let input = r#"> bar
baz"#;
    let output = r#"<blockquote>
<p>bar
baz</p>
</blockquote>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_248() {
    let input = r#"> bar

baz"#;
    let output = r#"<blockquote>
<p>bar</p>
</blockquote>
<p>baz</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_249() {
    let input = r#"> bar
>
baz"#;
    let output = r#"<blockquote>
<p>bar</p>
</blockquote>
<p>baz</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_250() {
    let input = r#"> > > foo
bar"#;
    let output = r#"<blockquote>
<blockquote>
<blockquote>
<p>foo
bar</p>
</blockquote>
</blockquote>
</blockquote>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_251() {
    let input = r#">>> foo
> bar
>>baz"#;
    let output = r#"<blockquote>
<blockquote>
<blockquote>
<p>foo
bar
baz</p>
</blockquote>
</blockquote>
</blockquote>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_252() {
    let input = r#">     code

>    not code"#;
    let output = r#"<blockquote>
<pre><code>code
</code></pre>
</blockquote>
<blockquote>
<p>not code</p>
</blockquote>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
