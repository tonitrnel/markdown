use markdown::parser::Parser;

// ========================== html block =================================
#[test]
fn case_148() {
    let input = r#"<table><tr><td>
<pre>
**Hello**,

_world_.
</pre>
</td></tr></table>"#;
    let output = r#"<table><tr><td>
<pre>
**Hello**,
<p><em>world</em>.
</pre></p>
</td></tr></table>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_149() {
    let input = r#"<table>
  <tr>
    <td>
           hi
    </td>
  </tr>
</table>

okay."#;
    let output = r#"<table>
  <tr>
    <td>
           hi
    </td>
  </tr>
</table>
<p>okay.</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_150() {
    let input = r#" <div>
  *hello*
         <foo><a>"#;
    let output = r#" <div>
  *hello*
         <foo><a>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_151() {
    let input = r#"</div>
*foo*"#;
    let output = r#"</div>
*foo*"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_152() {
    let input = r#"<DIV CLASS="foo">

*Markdown*

</DIV>"#;
    let output = r#"<DIV CLASS="foo">
<p><em>Markdown</em></p>
</DIV>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_153() {
    let input = r#"<div id="foo"
  class="bar">
</div>"#;
    let output = r#"<div id="foo"
  class="bar">
</div>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_154() {
    let input = r#"<div id="foo" class="bar
  baz">
</div>"#;
    let output = r#"<div id="foo" class="bar
  baz">
</div>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_155() {
    let input = r#"<div>
*foo*

*bar*"#;
    let output = r#"<div>
*foo*
<p><em>bar</em></p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_156() {
    let input = r#"<div id="foo"
*hi*"#;
    let output = r#"<div id="foo"
*hi*"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_157() {
    let input = r#"<div class
foo"#;
    let output = r#"<div class
foo"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_158() {
    let input = r#"<div *???-&&&-<---
*foo*"#;
    let output = r#"<div *???-&&&-<---
*foo*"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_159() {
    let input = r#"<div><a href="bar">*foo*</a></div>"#;
    let output = r#"<div><a href="bar">*foo*</a></div>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_160() {
    let input = r#"<table><tr><td>
foo
</td></tr></table>"#;
    let output = r#"<table><tr><td>
foo
</td></tr></table>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_161() {
    let input = r#"<div></div>
``` c
int x = 33;
```"#;
    let output = r#"<div></div>
``` c
int x = 33;
```"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_162() {
    let input = r#"<a href="foo">
*bar*
</a>"#;
    let output = r#"<a href="foo">
*bar*
</a>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_163() {
    let input = r#"<Warning>
*bar*
</Warning>"#;
    let output = r#"<Warning>
*bar*
</Warning>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_164() {
    let input = r#"<i class="foo">
*bar*
</i>"#;
    let output = r#"<i class="foo">
*bar*
</i>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_165() {
    let input = r#"</ins>
*bar*"#;
    let output = r#"</ins>
*bar*"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_166() {
    let input = r#"<del>
*foo*
</del>"#;
    let output = r#"<del>
*foo*
</del>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_167() {
    let input = r#"<del>

*foo*

</del>"#;
    let output = r#"<del>
<p><em>foo</em></p>
</del>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_168() {
    let input = r#"<del>*foo*</del>"#;
    let output = r#"<p><del><em>foo</em></del></p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_169() {
    let input = r#"<pre language="haskell"><code>
import Text.HTML.TagSoup

main :: IO ()
main = print $ parseTags tags
</code></pre>
okay"#;
    let output = r#"<pre language="haskell"><code>
import Text.HTML.TagSoup

main :: IO ()
main = print $ parseTags tags
</code></pre>
<p>okay</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_170() {
    let input = r#"<script type="text/javascript">
// JavaScript example

document.getElementById("demo").innerHTML = "Hello JavaScript!";
</script>
okay"#;
    let output = r#"<script type="text/javascript">
// JavaScript example

document.getElementById("demo").innerHTML = "Hello JavaScript!";
</script>
<p>okay</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_171() {
    let input = r#"<textarea>

*foo*

_bar_

</textarea>"#;
    let output = r#"<textarea>

*foo*

_bar_

</textarea>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_172() {
    let input = r#"<style
  type="text/css">
h1 {color:red;}

p {color:blue;}
</style>
okay"#;
    let output = r#"<style
  type="text/css">
h1 {color:red;}

p {color:blue;}
</style>
<p>okay</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_173() {
    let input = r#"<style
  type="text/css">

foo"#;
    let output = r#"<style
  type="text/css">

foo"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_174() {
    let input = r#"> <div>
> foo

bar"#;
    let output = r#"<blockquote>
<div>
foo
</blockquote>
<p>bar</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_175() {
    let input = r#"- <div>
- foo"#;
    let output = r#"<ul>
<li>
<div>
</li>
<li>foo</li>
</ul>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_176() {
    let input = r#"<style>p{color:red;}</style>
*foo*"#;
    let output = r#"<style>p{color:red;}</style>
<p><em>foo</em></p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_177() {
    let input = r#"<!-- foo -->*bar*
*baz*"#;
    let output = r#"<!-- foo -->*bar*
<p><em>baz</em></p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_178() {
    let input = r#"<script>
foo
</script>1. *bar*"#;
    let output = r#"<script>
foo
</script>1. *bar*"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_179() {
    let input = r#"<!-- Foo

bar
   baz -->
okay"#;
    let output = r#"<!-- Foo

bar
   baz -->
<p>okay</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_180() {
    let input = r#"<?php

  echo '>';

?>
okay"#;
    let output = r#"<?php

  echo '>';

?>
<p>okay</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_181() {
    let input = r#"<!DOCTYPE html>"#;
    let output = r#"<!DOCTYPE html>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_182() {
    let input = r#"<![CDATA[
function matchwo(a,b)
{
  if (a < b && a < 0) then {
    return 1;

  } else {

    return 0;
  }
}
]]>
okay"#;
    let output = r#"<![CDATA[
function matchwo(a,b)
{
  if (a < b && a < 0) then {
    return 1;

  } else {

    return 0;
  }
}
]]>
<p>okay</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_183() {
    let input = r#"  <!-- foo -->

    <!-- foo -->"#;
    let output = r#"  <!-- foo -->
<pre><code>&lt;!-- foo --&gt;
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_184() {
    let input = r#"  <div>

    <div>"#;
    let output = r#"  <div>
<pre><code>&lt;div&gt;
</code></pre>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_185() {
    let input = r#"Foo
<div>
bar
</div>"#;
    let output = r#"<p>Foo</p>
<div>
bar
</div>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_186() {
    let input = r#"<div>
bar
</div>
*foo*"#;
    let output = r#"<div>
bar
</div>
*foo*"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_187() {
    let input = r#"Foo
<a href="bar">
baz"#;
    let output = r#"<p>Foo
<a href="bar">
baz</p>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_188() {
    let input = r#"<div>

*Emphasized* text.

</div>"#;
    let output = r#"<div>
<p><em>Emphasized</em> text.</p>
</div>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_189() {
    let input = r#"<div>
*Emphasized* text.
</div>"#;
    let output = r#"<div>
*Emphasized* text.
</div>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_190() {
    let input = r#"<table>

<tr>

<td>
Hi
</td>

</tr>

</table>"#;
    let output = r#"<table>
<tr>
<td>
Hi
</td>
</tr>
</table>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}
#[test]
fn case_191() {
    let input = r#"<table>

  <tr>

    <td>
      Hi
    </td>

  </tr>

</table>"#;
    let output = r#"<table>
  <tr>
<pre><code>&lt;td&gt;
  Hi
&lt;/td&gt;
</code></pre>
  </tr>
</table>"#;
    let ast = Parser::new(input).parse();
    println!("AST:\n{ast:?}");
    assert_eq!(ast.to_html(), output);
}

// ========================== html inline =================================
