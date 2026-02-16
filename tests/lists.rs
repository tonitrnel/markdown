use markdown::parser::Parser;

#[test]
fn case_253() {
    let input = r#"A paragraph
with two lines.

    indented code

> A block quote."#;
    let output = r#"<p>A paragraph
with two lines.</p>
<pre><code>indented code
</code></pre>
<blockquote>
<p>A block quote.</p>
</blockquote>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_254() {
    let input = r#"1.  A paragraph
    with two lines.

        indented code

    > A block quote."#;
    let output = r#"<ol>
<li>
<p>A paragraph
with two lines.</p>
<pre><code>indented code
</code></pre>
<blockquote>
<p>A block quote.</p>
</blockquote>
</li>
</ol>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_255() {
    let input = r#"- one

 two"#;
    let output = r#"<ul>
<li>one</li>
</ul>
<p>two</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_256() {
    let input = r#"- one

  two"#;
    let output = r#"<ul>
<li>
<p>one</p>
<p>two</p>
</li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_257() {
    let input = r#" -    one

     two"#;
    let output = r#"<ul>
<li>one</li>
</ul>
<pre><code> two
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_258() {
    let input = r#" -    one

      two"#;
    let output = r#"<ul>
<li>
<p>one</p>
<p>two</p>
</li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_259() {
    let input = r#"   > > 1.  one
>>
>>     two"#;
    let output = r#"<blockquote>
<blockquote>
<ol>
<li>
<p>one</p>
<p>two</p>
</li>
</ol>
</blockquote>
</blockquote>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_260() {
    let input = r#">>- one
>>
  >  > two"#;
    let output = r#"<blockquote>
<blockquote>
<ul>
<li>one</li>
</ul>
<p>two</p>
</blockquote>
</blockquote>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_261() {
    let input = r#"-one

2.two"#;
    let output = r#"<p>-one</p>
<p>2.two</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_262() {
    let input = r#"- foo


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
fn case_263() {
    let input = r#"1.  foo

    ```
    bar
    ```

    baz

    > bam"#;
    let output = r#"<ol>
<li>
<p>foo</p>
<pre><code>bar
</code></pre>
<p>baz</p>
<blockquote>
<p>bam</p>
</blockquote>
</li>
</ol>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_264() {
    let input = r#"- Foo

      bar


      baz"#;
    let output = r#"<ul>
<li>
<p>Foo</p>
<pre><code>bar


baz
</code></pre>
</li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_265() {
    let input = r#"123456789. ok"#;
    let output = r#"<ol start="123456789">
<li>ok</li>
</ol>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_266() {
    let input = r#"1234567890. not ok"#;
    let output = r#"<p>1234567890. not ok</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_267() {
    let input = r#"0. ok"#;
    let output = r#"<ol start="0">
<li>ok</li>
</ol>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_268() {
    let input = r#"003. ok"#;
    let output = r#"<ol start="3">
<li>ok</li>
</ol>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_269() {
    let input = r#"-1. not ok"#;
    let output = r#"<p>-1. not ok</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_270() {
    let input = r#"- foo

      bar"#;
    let output = r#"<ul>
<li>
<p>foo</p>
<pre><code>bar
</code></pre>
</li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_271() {
    let input = r#"  10.  foo

           bar"#;
    let output = r#"<ol start="10">
<li>
<p>foo</p>
<pre><code>bar
</code></pre>
</li>
</ol>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_272() {
    let input = r#"    indented code

paragraph

    more code"#;
    let output = r#"<pre><code>indented code
</code></pre>
<p>paragraph</p>
<pre><code>more code
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_273() {
    let input = r#"1.     indented code

   paragraph

       more code"#;
    let output = r#"<ol>
<li>
<pre><code>indented code
</code></pre>
<p>paragraph</p>
<pre><code>more code
</code></pre>
</li>
</ol>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_274() {
    let input = r#"1.      indented code

   paragraph

       more code"#;
    let output = r#"<ol>
<li>
<pre><code> indented code
</code></pre>
<p>paragraph</p>
<pre><code>more code
</code></pre>
</li>
</ol>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_275() {
    let input = r#"   foo

bar"#;
    let output = r#"<p>foo</p>
<p>bar</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_276() {
    let input = r#"-    foo

  bar"#;
    let output = r#"<ul>
<li>foo</li>
</ul>
<p>bar</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_277() {
    let input = r#"-  foo

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
fn case_278() {
    let input = r#"-
  foo
-
  ```
  bar
  ```
-
      baz"#;
    let output = r#"<ul>
<li>foo</li>
<li>
<pre><code>bar
</code></pre>
</li>
<li>
<pre><code>baz
</code></pre>
</li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_279() {
    let input = r#"-   
  foo"#;
    let output = r#"<ul>
<li>foo</li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_280() {
    let input = r#"-

  foo"#;
    let output = r#"<ul>
<li></li>
</ul>
<p>foo</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_281() {
    let input = r#"- foo
-
- bar"#;
    let output = r#"<ul>
<li>foo</li>
<li></li>
<li>bar</li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_282() {
    let input = r#"- foo
-   
- bar"#;
    let output = r#"<ul>
<li>foo</li>
<li></li>
<li>bar</li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_283() {
    let input = r#"1. foo
2.
3. bar"#;
    let output = r#"<ol>
<li>foo</li>
<li></li>
<li>bar</li>
</ol>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_284() {
    let input = r#"*"#;
    let output = r#"<ul>
<li></li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_285() {
    let input = r#"foo
*

foo
1."#;
    let output = r#"<p>foo
*</p>
<p>foo
1.</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_286() {
    let input = r#" 1.  A paragraph
     with two lines.

         indented code

     > A block quote."#;
    let output = r#"<ol>
<li>
<p>A paragraph
with two lines.</p>
<pre><code>indented code
</code></pre>
<blockquote>
<p>A block quote.</p>
</blockquote>
</li>
</ol>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_287() {
    let input = r#"  1.  A paragraph
      with two lines.

          indented code

      > A block quote."#;
    let output = r#"<ol>
<li>
<p>A paragraph
with two lines.</p>
<pre><code>indented code
</code></pre>
<blockquote>
<p>A block quote.</p>
</blockquote>
</li>
</ol>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_288() {
    let input = r#"   1.  A paragraph
       with two lines.

           indented code

       > A block quote."#;
    let output = r#"<ol>
<li>
<p>A paragraph
with two lines.</p>
<pre><code>indented code
</code></pre>
<blockquote>
<p>A block quote.</p>
</blockquote>
</li>
</ol>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_289() {
    let input = r#"    1.  A paragraph
        with two lines.

            indented code

        > A block quote."#;
    let output = r#"<pre><code>1.  A paragraph
    with two lines.

        indented code

    &gt; A block quote.
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_290() {
    let input = r#"  1.  A paragraph
with two lines.

          indented code

      > A block quote."#;
    let output = r#"<ol>
<li>
<p>A paragraph
with two lines.</p>
<pre><code>indented code
</code></pre>
<blockquote>
<p>A block quote.</p>
</blockquote>
</li>
</ol>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_291() {
    let input = r#"  1.  A paragraph
    with two lines."#;
    let output = r#"<ol>
<li>A paragraph
with two lines.</li>
</ol>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_292() {
    let input = r#"> 1. > Blockquote
continued here."#;
    let output = r#"<blockquote>
<ol>
<li>
<blockquote>
<p>Blockquote
continued here.</p>
</blockquote>
</li>
</ol>
</blockquote>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_293() {
    let input = r#"> 1. > Blockquote
> continued here."#;
    let output = r#"<blockquote>
<ol>
<li>
<blockquote>
<p>Blockquote
continued here.</p>
</blockquote>
</li>
</ol>
</blockquote>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_294() {
    let input = r#"- foo
  - bar
    - baz
      - boo"#;
    let output = r#"<ul>
<li>foo<ul>
<li>bar<ul>
<li>baz<ul>
<li>boo</li>
</ul></li>
</ul></li>
</ul></li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_295() {
    let input = r#"- foo
 - bar
  - baz
   - boo"#;
    let output = r#"<ul>
<li>foo</li>
<li>bar</li>
<li>baz</li>
<li>boo</li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_296() {
    let input = r#"10) foo
    - bar"#;
    let output = r#"<ol start="10">
<li>foo<ul>
<li>bar</li>
</ul></li>
</ol>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_297() {
    let input = r#"10) foo
   - bar"#;
    let output = r#"<ol start="10">
<li>foo</li>
</ol>
<ul>
<li>bar</li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_298() {
    let input = r#"- - foo"#;
    let output = r#"<ul>
<li>
<ul>
<li>foo</li>
</ul>
</li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_299() {
    let input = r#"1. - 2. foo"#;
    let output = r#"<ol>
<li>
<ul>
<li>
<ol start="2">
<li>foo</li>
</ol>
</li>
</ul>
</li>
</ol>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_300() {
    let input = r#"- # Foo
- Bar
  ---
  baz"#;
    let output = r#"<ul>
<li>
<h1>Foo</h1>
</li>
<li>
<h2>Bar</h2>
baz
</li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_301() {
    let input = r#"- foo
- bar
+ baz"#;
    let output = r#"<ul>
<li>foo</li>
<li>bar</li>
</ul>
<ul>
<li>baz</li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_302() {
    let input = r#"1. foo
2. bar
3) baz"#;
    let output = r#"<ol>
<li>foo</li>
<li>bar</li>
</ol>
<ol start="3">
<li>baz</li>
</ol>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_303() {
    let input = r#"Foo
- bar
- baz"#;
    let output = r#"<p>Foo</p>
<ul>
<li>bar</li>
<li>baz</li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_304() {
    let input = r#"The number of windows in my house is
14.  The number of doors is 6."#;
    let output = r#"<p>The number of windows in my house is
14.  The number of doors is 6.</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_305() {
    let input = r#"The number of windows in my house is
1.  The number of doors is 6."#;
    let output = r#"<p>The number of windows in my house is</p>
<ol>
<li>The number of doors is 6.</li>
</ol>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_306() {
    let input = r#"- foo

- bar


- baz"#;
    let output = r#"<ul>
<li>
<p>foo</p>
</li>
<li>
<p>bar</p>
</li>
<li>
<p>baz</p>
</li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_307() {
    let input = r#"- foo
  - bar
    - baz


      bim"#;
    let output = r#"<ul>
<li>foo<ul>
<li>bar<ul>
<li>
<p>baz</p>
<p>bim</p>
</li>
</ul></li>
</ul></li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_308() {
    let input = r#"- foo
- bar

<!-- -->

- baz
- bim"#;
    let output = r#"<ul>
<li>foo</li>
<li>bar</li>
</ul>
<!-- -->
<ul>
<li>baz</li>
<li>bim</li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_309() {
    let input = r#"-   foo

    notcode

-   foo

<!-- -->

    code"#;
    let output = r#"<ul>
<li>
<p>foo</p>
<p>notcode</p>
</li>
<li>
<p>foo</p>
</li>
</ul>
<!-- -->
<pre><code>code
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_310() {
    let input = r#"- a
 - b
  - c
   - d
  - e
 - f
- g"#;
    let output = r#"<ul>
<li>a</li>
<li>b</li>
<li>c</li>
<li>d</li>
<li>e</li>
<li>f</li>
<li>g</li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_311() {
    let input = r#"1. a

  2. b

   3. c"#;
    let output = r#"<ol>
<li>
<p>a</p>
</li>
<li>
<p>b</p>
</li>
<li>
<p>c</p>
</li>
</ol>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_312() {
    let input = r#"- a
 - b
  - c
   - d
    - e"#;
    let output = r#"<ul>
<li>a</li>
<li>b</li>
<li>c</li>
<li>d
- e</li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_313() {
    let input = r#"1. a

  2. b

    3. c"#;
    let output = r#"<ol>
<li>
<p>a</p>
</li>
<li>
<p>b</p>
</li>
</ol>
<pre><code>3. c
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_314() {
    let input = r#"- a
- b

- c"#;
    let output = r#"<ul>
<li>
<p>a</p>
</li>
<li>
<p>b</p>
</li>
<li>
<p>c</p>
</li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_315() {
    let input = r#"* a
*

* c"#;
    let output = r#"<ul>
<li>
<p>a</p>
</li>
<li>

</li>
<li>
<p>c</p>
</li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_316() {
    let input = r#"- a
- b

  c
- d"#;
    let output = r#"<ul>
<li>
<p>a</p>
</li>
<li>
<p>b</p>
<p>c</p>
</li>
<li>
<p>d</p>
</li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_317() {
    let input = r#"- a
- b

  [ref]: /url
- d"#;
    let output = r#"<ul>
<li>
<p>a</p>
</li>
<li>
<p>b</p>
</li>
<li>
<p>d</p>
</li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_318() {
    let input = r#"- a
- ```
  b


  ```
- c"#;
    let output = r#"<ul>
<li>a</li>
<li>
<pre><code>b


</code></pre>
</li>
<li>c</li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_319() {
    let input = r#"- a
  - b

    c
- d"#;
    let output = r#"<ul>
<li>a<ul>
<li>
<p>b</p>
<p>c</p>
</li>
</ul></li>
<li>d</li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_320() {
    let input = r#"* a
  > b
  >
* c"#;
    let output = r#"<ul>
<li>a<blockquote>
<p>b</p>
</blockquote></li>
<li>c</li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_321() {
    let input = r#"- a
  > b
  ```
  c
  ```
- d"#;
    let output = r#"<ul>
<li>a<blockquote>
<p>b</p>
</blockquote>
<pre><code>c
</code></pre></li>
<li>d</li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_322() {
    let input = r#"- a"#;
    let output = r#"<ul>
<li>a</li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_323() {
    let input = r#"- a
  - b"#;
    let output = r#"<ul>
<li>a<ul>
<li>b</li>
</ul></li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_324() {
    let input = r#"1. ```
   foo
   ```

   bar"#;
    let output = r#"<ol>
<li>
<pre><code>foo
</code></pre>
<p>bar</p>
</li>
</ol>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_325() {
    let input = r#"* foo
  * bar

  baz"#;
    let output = r#"<ul>
<li>
<p>foo</p>
<ul>
<li>bar</li>
</ul>
<p>baz</p>
</li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_326() {
    let input = r#"- a
  - b
  - c

- d
  - e
  - f"#;
    let output = r#"<ul>
<li>
<p>a</p>
<ul>
<li>b</li>
<li>c</li>
</ul>
</li>
<li>
<p>d</p>
<ul>
<li>e</li>
<li>f</li>
</ul>
</li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
