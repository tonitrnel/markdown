use markdown::parser::Parser;

#[test]
fn case_192() {
    let input = r#"[foo]: /url "title"

[foo]"#;
    let output = r#"<p><a href="/url" title="title">foo</a></p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_193() {
    let input = r#"   [foo]: 
      /url  
           'the title'  

[foo]"#;
    let output = r#"<p><a href="/url" title="the title">foo</a></p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_194() {
    let input = r#"[Foo*bar\]]:my_(url) 'title (with parens)'

[Foo*bar\]]"#;
    let output = r#"<p><a href="my_(url)" title="title (with parens)">Foo*bar]</a></p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_195() {
    let input = r#"[Foo bar]:
<my url>
'title'

[Foo bar]"#;
    let output = r#"<p><a href="my%20url" title="title">Foo bar</a></p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_196() {
    let input = r#"[foo]: /url '
title
line1
line2
'

[foo]"#;
    let output = r#"<p><a href="/url" title="
title
line1
line2
">foo</a></p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_197() {
    let input = r#"[foo]: /url 'title

with blank line'

[foo]"#;
    let output = r#"<p>[foo]: /url 'title</p>
<p>with blank line'</p>
<p>[foo]</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_198() {
    let input = r#"[foo]:
/url

[foo]"#;
    let output = r#"<p><a href="/url">foo</a></p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_199() {
    let input = r#"[foo]:

[foo]"#;
    let output = r#"<p>[foo]:</p>
<p>[foo]</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_200() {
    let input = r#"[foo]: <>

[foo]"#;
    let output = r#"<p><a href="">foo</a></p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_201() {
    let input = r#"[foo]: <bar>(baz)

[foo]"#;
    let output = r#"<p>[foo]: <bar>(baz)</p>
<p>[foo]</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_202() {
    let input = r#"[foo]: /url\bar\*baz "foo\"bar\baz"

[foo]"#;
    let output = r#"<p><a href="/url%5Cbar*baz" title="foo&quot;bar\baz">foo</a></p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_203() {
    let input = r#"[foo]

[foo]: url"#;
    let output = r#"<p><a href="url">foo</a></p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_204() {
    let input = r#"[foo]

[foo]: first
[foo]: second"#;
    let output = r#"<p><a href="first">foo</a></p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_205() {
    let input = r#"[FOO]: /url

[Foo]"#;
    let output = r#"<p><a href="/url">Foo</a></p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_206() {
    let input = r#"[ΑΓΩ]: /φου

[αγω]"#;
    let output = r#"<p><a href="/%CF%86%CE%BF%CF%85">αγω</a></p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_207() {
    let input = r#"[foo]: /url"#;
    let output = r#""#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_208() {
    let input = r#"[
foo
]: /url
bar"#;
    let output = r#"<p>bar</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_209() {
    let input = r#"[foo]: /url "title" ok"#;
    let output = r#"<p>[foo]: /url &quot;title&quot; ok</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_210() {
    let input = r#"[foo]: /url
"title" ok"#;
    let output = r#"<p>&quot;title&quot; ok</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_211() {
    let input = r#"    [foo]: /url "title"

[foo]"#;
    let output = r#"<pre><code>[foo]: /url &quot;title&quot;
</code></pre>
<p>[foo]</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_212() {
    let input = r#"```
[foo]: /url
```

[foo]"#;
    let output = r#"<pre><code>[foo]: /url
</code></pre>
<p>[foo]</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_213() {
    let input = r#"Foo
[bar]: /baz

[bar]"#;
    let output = r#"<p>Foo
[bar]: /baz</p>
<p>[bar]</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_214() {
    let input = r#"# [Foo]
[foo]: /url
> bar"#;
    let output = r#"<h1><a href="/url">Foo</a></h1>
<blockquote>
<p>bar</p>
</blockquote>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_215() {
    let input = r#"[foo]: /url
bar
===
[foo]"#;
    let output = r#"<h1>bar</h1>
<p><a href="/url">foo</a></p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_216() {
    let input = r#"[foo]: /url
===
[foo]"#;
    let output = r#"<p>===
<a href="/url">foo</a></p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_217() {
    let input = r#"[foo]: /foo-url "foo"
[bar]: /bar-url
  "bar"
[baz]: /baz-url

[foo],
[bar],
[baz]"#;
    let output = r#"<p><a href="/foo-url" title="foo">foo</a>,
<a href="/bar-url" title="bar">bar</a>,
<a href="/baz-url">baz</a></p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_218() {
    let input = r#"[foo]

> [foo]: /url"#;
    let output = r#"<p><a href="/url">foo</a></p>
<blockquote>
</blockquote>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
