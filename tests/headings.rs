use markdown::parser::Parser;
#[test]
fn case_62() {
    let input = r#"# foo
## foo
### foo
#### foo
##### foo
###### foo"#;
    let output = r#"<h1>foo</h1>
<h2>foo</h2>
<h3>foo</h3>
<h4>foo</h4>
<h5>foo</h5>
<h6>foo</h6>"#;
    let ast = Parser::new(input).parse();
    // println!("AST:\n{ast:?}")
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_63() {
    let input = r#"####### foo"#;
    let output = r#"<p>####### foo</p>"#;
    let ast = Parser::new(input).parse();
    // println!("AST:\n{ast:?}")
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_64() {
    let input = r#"#5 bolt

#hashtag"#;
    let output = r#"<p>#5 bolt</p>
<p>#hashtag</p>"#;
    let ast = Parser::new(input).parse();
    // println!("AST:\n{ast:?}")
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_65() {
    let input = r#"\## foo"#;
    let output = r#"<p>## foo</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_66() {
    let input = r#"# foo *bar* \*baz\*"#;
    let output = r#"<h1>foo <em>bar</em> *baz*</h1>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_67() {
    let input = r#"#                  foo"#;
    let output = r#"<h1>foo</h1>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_68() {
    let input = r#" ### foo
  ## foo
   # foo"#;
    let output = r#"<h3>foo</h3>
<h2>foo</h2>
<h1>foo</h1>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_69() {
    let input = r#"    # foo"#;
    let output = r#"<pre><code># foo
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_70() {
    let input = r#"foo
    # bar"#;
    let output = r#"<p>foo
# bar</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_71() {
    let input = r#"## foo ##
  ###   bar    ###"#;
    let output = r#"<h2>foo</h2>
<h3>bar</h3>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_72() {
    let input = r#"# foo ##################################
##### foo ##"#;
    let output = r#"<h1>foo</h1>
<h5>foo</h5>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_73() {
    let input = r#"### foo ###"#;
    let output = r#"<h3>foo</h3>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_74() {
    let input = r#"### foo ### b"#;
    let output = r#"<h3>foo ### b</h3>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_75() {
    let input = r#"# foo#"#;
    let output = r#"<h1>foo#</h1>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_76() {
    let input = r#"### foo \###
## foo #\##
# foo \#"#;
    let output = r#"<h3>foo ###</h3>
<h2>foo ###</h2>
<h1>foo #</h1>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_77() {
    let input = r#"****
## foo
****"#;
    let output = r#"<hr />
<h2>foo</h2>
<hr />"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_78() {
    let input = r#"Foo bar
# baz
Bar foo"#;
    let output = r#"<p>Foo bar</p>
<h1>baz</h1>
<p>Bar foo</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_79() {
    let input = r#"## 
#
### ###"#;
    let output = r#"<h2></h2>
<h1></h1>
<h3></h3>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_80() {
    let input = r#"Foo *bar*
=========

Foo *bar*
---------"#;
    let output = r#"<h1>Foo <em>bar</em></h1>
<h2>Foo <em>bar</em></h2>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_81() {
    let input = r#"Foo *bar
baz*
===="#;
    let output = r#"<h1>Foo <em>bar
baz</em></h1>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_82() {
    let input = r#"  Foo *bar
baz*	
===="#;
    let output = r#"<h1>Foo <em>bar
baz</em></h1>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_83() {
    let input = r#"Foo
-------------------------

Foo
="#;
    let output = r#"<h2>Foo</h2>
<h1>Foo</h1>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_84() {
    let input = r#"   Foo
---

  Foo
-----

  Foo
  ==="#;
    let output = r#"<h2>Foo</h2>
<h2>Foo</h2>
<h1>Foo</h1>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_85() {
    let input = r#"    Foo
    ---

    Foo
---"#;
    let output = r#"<pre><code>Foo
---

Foo
</code></pre>
<hr />"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_86() {
    let input = r#"Foo
   ----"#;
    let output = r#"<h2>Foo</h2>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_87() {
    let input = r#"Foo
    ---"#;
    let output = r#"<p>Foo
---</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_88() {
    let input = r#"Foo
= =

Foo
--- -"#;
    let output = r#"<p>Foo
= =</p>
<p>Foo</p>
<hr />"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_89() {
    let input = r#"Foo  
-----"#;
    let output = r#"<h2>Foo</h2>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_90() {
    let input = r#"Foo\
----"#;
    let output = r#"<h2>Foo\</h2>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_91() {
    let input = r#"`Foo
----
`

<a title="a lot
---
of dashes"/>"#;
    let output = r#"<h2>`Foo</h2>
<p>`</p>
<h2>&lt;a title=&quot;a lot</h2>
<p>of dashes&quot;/&gt;</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_92() {
    let input = r#"> Foo
---"#;
    let output = r#"<blockquote>
<p>Foo</p>
</blockquote>
<hr />"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_93() {
    let input = r#"> foo
bar
==="#;
    let output = r#"<blockquote>
<p>foo
bar
===</p>
</blockquote>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_94() {
    let input = r#"- Foo
---"#;
    let output = r#"<ul>
<li>Foo</li>
</ul>
<hr />"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_95() {
    let input = r#"Foo
Bar
---"#;
    let output = r#"<h2>Foo
Bar</h2>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_96() {
    let input = r#"---
Foo
---
Bar
---
Baz"#;
    let output = r#"<hr />
<h2>Foo</h2>
<h2>Bar</h2>
<p>Baz</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_97() {
    let input = r#"
===="#;
    let output = r#"<p>====</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_98() {
    let input = r#"---
---"#;
    let output = r#"<hr />
<hr />"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_99() {
    let input = r#"- foo
-----"#;
    let output = r#"<ul>
<li>foo</li>
</ul>
<hr />"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_100() {
    let input = r#"    foo
---"#;
    let output = r#"<pre><code>foo
</code></pre>
<hr />"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_101() {
    let input = r#"> foo
-----"#;
    let output = r#"<blockquote>
<p>foo</p>
</blockquote>
<hr />"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_102() {
    let input = r#"\> foo
------"#;
    let output = r#"<h2>&gt; foo</h2>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_103() {
    let input = r#"Foo

bar
---
baz"#;
    let output = r#"<p>Foo</p>
<h2>bar</h2>
<p>baz</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_104() {
    let input = r#"Foo
bar

---

baz"#;
    let output = r#"<p>Foo
bar</p>
<hr />
<p>baz</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_105() {
    let input = r#"Foo
bar
* * *
baz"#;
    let output = r#"<p>Foo
bar</p>
<hr />
<p>baz</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_106() {
    let input = r#"Foo
bar
\---
baz"#;
    let output = r#"<p>Foo
bar
---
baz</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
