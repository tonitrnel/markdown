use markdown::parser::Parser;

// ========================== indent code =================================
#[test]
fn case_107() {
    let input = r#"    a simple
      indented code block"#;
    let output = r#"<pre><code>a simple
  indented code block
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_108() {
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
fn case_109() {
    let input = r#"1.  foo

    - bar"#;
    let output = r#"<ol>
<li>
<p>foo</p>
<ul>
<li>bar</li>
</ul>
</li>
</ol>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_110() {
    let input = r#"    <a/>
    *hi*

    - one"#;
    let output = r#"<pre><code>&lt;a/&gt;
*hi*

- one
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_111() {
    let input = r#"    chunk1

    chunk2
  
 
 
    chunk3"#;
    let output = r#"<pre><code>chunk1

chunk2



chunk3
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_112() {
    let input = r#"    chunk1
      
      chunk2"#;
    let output = r#"<pre><code>chunk1
  
  chunk2
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_113() {
    let input = r#"Foo
    bar"#;
    let output = r#"<p>Foo
bar</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_114() {
    let input = r#"    foo
bar"#;
    let output = r#"<pre><code>foo
</code></pre>
<p>bar</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_115() {
    let input = r#"# Heading
    foo
Heading
------
    foo
----"#;
    let output = r#"<h1>Heading</h1>
<pre><code>foo
</code></pre>
<h2>Heading</h2>
<pre><code>foo
</code></pre>
<hr />"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_116() {
    let input = r#"        foo
    bar"#;
    let output = r#"<pre><code>    foo
bar
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_117() {
    let input = r#"
    
    foo"#;
    let output = r#"<pre><code>foo
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_118() {
    let input = r#"    foo  "#;
    let output = r#"<pre><code>foo  
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
// ========================== fence code =================================
#[test]
fn case_119() {
    let input = r#"```
<
 >
```"#;
    let output = r#"<pre><code>&lt;
 &gt;
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_120() {
    let input = r#"~~~
<
 >
~~~"#;
    let output = r#"<pre><code>&lt;
 &gt;
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_121() {
    let input = r#"``
foo
``"#;
    let output = r#"<p><code>foo</code></p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_122() {
    let input = r#"```
aaa
~~~
```"#;
    let output = r#"<pre><code>aaa
~~~
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_123() {
    let input = r#"~~~
aaa
```
~~~"#;
    let output = r#"<pre><code>aaa
```
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_124() {
    let input = r#"````
aaa
```
``````"#;
    let output = r#"<pre><code>aaa
```
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_125() {
    let input = r#"~~~~
aaa
~~~
~~~~"#;
    let output = r#"<pre><code>aaa
~~~
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_126() {
    let input = r#"```"#;
    let output = r#"<pre><code></code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_127() {
    let input = r#"`````

```
aaa"#;
    let output = r#"<pre><code>
```
aaa
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_128() {
    let input = r#"> ```
> aaa

bbb"#;
    let output = r#"<blockquote>
<pre><code>aaa
</code></pre>
</blockquote>
<p>bbb</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_129() {
    let input = r#"```

  
```"#;
    let output = r#"<pre><code>
  
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_130() {
    let input = r#"```
```"#;
    let output = r#"<pre><code></code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_131() {
    let input = r#" ```
 aaa
aaa
```"#;
    let output = r#"<pre><code>aaa
aaa
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_132() {
    let input = r#"  ```
aaa
  aaa
aaa
  ```"#;
    let output = r#"<pre><code>aaa
aaa
aaa
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_133() {
    let input = r#"   ```
   aaa
    aaa
  aaa
   ```"#;
    let output = r#"<pre><code>aaa
 aaa
aaa
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_134() {
    let input = r#"    ```
    aaa
    ```"#;
    let output = r#"<pre><code>```
aaa
```
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_135() {
    let input = r#"```
aaa
  ```"#;
    let output = r#"<pre><code>aaa
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_136() {
    let input = r#"   ```
aaa
  ```"#;
    let output = r#"<pre><code>aaa
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_137() {
    let input = r#"```
aaa
    ```"#;
    let output = r#"<pre><code>aaa
    ```
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_138() {
    let input = r#"``` ```
aaa"#;
    let output = r#"<p><code> </code>
aaa</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_139() {
    let input = r#"~~~~~~
aaa
~~~ ~~"#;
    let output = r#"<pre><code>aaa
~~~ ~~
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_140() {
    let input = r#"foo
```
bar
```
baz"#;
    let output = r#"<p>foo</p>
<pre><code>bar
</code></pre>
<p>baz</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_141() {
    let input = r#"foo
---
~~~
bar
~~~
# baz"#;
    let output = r#"<h2>foo</h2>
<pre><code>bar
</code></pre>
<h1>baz</h1>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_142() {
    let input = r#"```ruby
def foo(x)
  return 3
end
```"#;
    let output = r#"<pre><code class="language-ruby">def foo(x)
  return 3
end
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_143() {
    let input = r#"~~~~    ruby startline=3 $%@#$
def foo(x)
  return 3
end
~~~~~~~"#;
    let output = r#"<pre><code class="language-ruby">def foo(x)
  return 3
end
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_144() {
    let input = r#"````;
````"#;
    let output = r#"<pre><code class="language-;"></code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_145() {
    let input = r#"``` aa ```
foo"#;
    let output = r#"<p><code>aa</code>
foo</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_146() {
    let input = r#"~~~ aa ``` ~~~
foo
~~~"#;
    let output = r#"<pre><code class="language-aa">foo
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_147() {
    let input = r#"```
``` aaa
```"#;
    let output = r#"<pre><code>``` aaa
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
