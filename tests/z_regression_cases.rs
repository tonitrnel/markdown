use markdown::{Parser, ParserOptions};

#[test]
fn parse_obsidian_reference_document() {
    let input = r##"

![plumeria|300x300](https://interactive-examples.mdn.mozilla.net/media/examples/plumeria.jpg)

### 块 Block

使用             两次回车新建一个块，即两个`\n`换行符

### 前言 Frontmatter

一个位于文件头，可选的，以Yaml为语法记录元数据的方式

	4442
	55476
	22
### 内部链接 InternalLink

```markdown
链接到文件
Link to a page: [[Internal Links]]
使用 # 链接到笔记中的标题
Link to a page: [[Internal Links#Heading 2]]
使用 #^ 链接到笔记中的块
Link to a page: [[Windows 下 CLion 开发 Rust 的 Debugger 配置#^9b8041]]
使用 | 显示自定义文本
Link to a page: [[Internal Links|custom display text]]
```
此链接无需输入路径，将自动匹配资源库下的内容
此处块部分语法是：`^9b8041`，包含上面的一个段落，`9b8041`是段落ID，一般由 Obsidian 生成，解析时需要考虑
^365d428

查看：[Internal Links](https://help.obsidian.md/Linking+notes+and+files/Internal+links) 

### 内部嵌入 Embeds

```markdown
嵌入笔记
![[Internal links]]
使用 # 嵌入笔记的标题部分
![[Internal links#Heading 2]]
使用 #^ 嵌入笔记的块部分
![[Internal links#^365d428]]
嵌入图像文件
![[exmaple.jpg]]
使用 | 指定嵌入图像的宽高
![[exmaple.jpg|200]]
![[exmaple.jpg|200x200]]
(考虑实现) 指定嵌入图像的属性
![[exmaple.jpg#width=200&height=300&fit=cover]]
嵌入音频文件
![[川井宪次-_かわい-けんじ_-孤独な巡礼-_孤独的巡礼_.mp3]]
嵌入PDF文件并指定页码
![[document.pdf#page=3]]
```

查看：[Embedding Files](https://help.obsidian.md/Linking+notes+and+files/Embedding+files)

### 标题 Heading

```md
# This is a heading 1

also use the alternative

This is a Heading 1
===

## This is a heading 2

This is a Heading 2
---

### This is a heading 3

#### This is a heading 4

##### This is a heading 5

###### This is a heading 6
```

# This is a heading 1

also use the alternative

This is a Heading 1
===

## This is a heading 2

This is a Heading 2
---

### This is a heading 3

#### This is a heading 4

##### This is a heading 5

###### This is a heading 6

### 重点 Emphasis

italic
```markdown
*This text will be italic*
_This will also be italic_
```

*This text will be italic*
_This will also be italic_

bold
```markdown
**This text will be bold**
__This will also be bold__
```

**This text will be bold**
__This will also be bold__

combine them
```markdown
_You **can** combine them_
```

_You **can** combine them_

### 列表 Lists

Unnumbered lists
```markdown
- Item 1
- Item 2
	- Item 2a
	- Item 2b
```

- Item 1
- Item 2
	- Item 2a
	- Item 2b

Numbered lists
```markdown
1. Item 1
2. Item 2
3. Item 3
	1. Item 3a
	2. Item 3b
```

1. Item 1
2. Item 2
3. Item 3
	1. Item 3a
	2. Item 3b

### 图像 Images

```md
![grapefruit](https://interactive-examples.mdn.mozilla.net/media/cc0-images/grapefruit-slice-332-332.jpg)
使用 | 指定图像的大小
![grapefruit|100](https://interactive-examples.mdn.mozilla.net/media/cc0-images/grapefruit-slice-332-332.jpg)
```


![grapefruit](https://interactive-examples.mdn.mozilla.net/media/cc0-images/grapefruit-slice-332-332.jpg)

![grapefruit|100](https://interactive-examples.mdn.mozilla.net/media/cc0-images/grapefruit-slice-332-332.jpg)

### 链接 Links

```markdown
[Mozilla Developer Network](https://developer.mozilla.org/en-US/)
```

[Mozilla Developer Network](https://developer.mozilla.org/en-US/)

如果未给定协议，则进行本地匹配，支持：`https?`、`obsidian` 等

链接地址中的空格等敏感字符需要转义，或者使用下面的语法
```markdown
[Slides Demo](<Slides Demo>)
```

[Slides Demo](<Slides Demo>)

### 块引用 Blockquotes

```markdown
> 无人爱苦，亦无人寻之欲之，乃因其苦...
```

> 无人爱苦，亦无人寻之欲之，乃因其苦...

### 代码 Code

Inline code
```markdown
Text inside `backticks` on a line will be formatted like code.
```

Text inside `backticks` on a line will be formatted like code.

Code blocks
~~~markdown
```rust
fn main(){
	pritntf!("Hello World")
}
```
~~~

```rust
fn main(){
	pritntf!("Hello World")
}
```

Escape code block

```markdown
~~~rust
fn main(){
	pritntf!("Hello World")
}
~~~
```

~~~rust
fn main(){
	pritntf!("Hello World")
}
~~~

Escape inline code
~~~markdown
single backtick: `` ` ``
triple ticks: `` ``` ``
~~~

single backtick: `` ` ``
triple ticks: `` ``` ``

### 任务列表 Task list

```markdown
- [x] #tags, [links](), **formatting** supported
- [x] list syntax required (any unordered or ordered list supported)
- [x] this is a complete item
 -[?] this is also a complete item (works with every character)
- [ ] this is an incomplete item
- [ ] tasks can be clicked in Preview to be checked off
```

- [x] #tags, [links](), **formatting** supported
- [x] list syntax required (any unordered or ordered list supported)
- [x] this is a complete item
 -[?] this is also a complete item (works with every character)
- [ ] this is an incomplete item
- [ ] tasks can be clicked in Preview to be checked off

### 表格 Tables

```markdown
|First Header | Second Header|
|------------ | ------------|
|Content from cell 1 | Content from cell 2|
|Content in the first column | Content in the second column|
```

|First Header | Second Header|
|------------ | ------------|
|Content from cell 1 | Content from cell 2|
|Content in the first column | Content in the second column|

The vertical bars at the start and end of a line are optional.

```markdown
First Header | Second Header
------------ | ------------
Content from cell 1 | Content from cell 2
Content in the first column | Content in the second column
```

First Header | Second Header
------------ | ------------
Content from cell 1 | Content from cell 2
Content in the first column | Content in the second column

Tables can be justified with a colon

```markdown
First Header  | Second Header | Third Header
:------------ | ------------: | :------------:
Content from cell 1 | Content from cell 2 | Content from cell 3
Content in the first column | Content in the second column | Content in the third column
```

First Header  | Second Header | Third Header
:------------ | ------------: | :------------:
Content from cell 1 | Content from cell 2 | Content from cell 3
Content in the first column | Content in the second column | Content in the third column

### 删除线 Strikethrough

```markdown
Any word wrapped with two tildes (like ~~this~~) will appear crossed out.
```

Any word wrapped with two tildes (like ~~this~~) will appear crossed out.

### 高亮 Highlighting

```markdown
Use two equal signs to ==highlight text==.
```

Use two equal signs to ==highlight text==.

### 水平线 Horizontal Bar

```markdown
Use three stars ***, hyphens ---, or underscores ___ in a new line to produce an horizontal bar.
```

Use three stars ***, hyphens 

---

, or underscores 

___ 

in a new line to produce an horizontal bar.

---



### 脚注 Fontnotes

```Markdown
Here's a simple footnote,[^1] and here's a longer one.[^bignote]

[^1]: meaningful!

[^bignote]: Here's one with multiple paragraphs and code.

    Indent paragraphs to include them in the footnote.

    `{ my code }`

    Add as many paragraphs as you like.
```

Here's a simple footnote,[^1] and here's a longer one.[^bignote]

[^1]: meaningful!

[^bignote]: Here's one with multiple paragraphs and code.

    Indent paragraphs to include them in the footnote.

    `{ my code }`

    Add as many paragraphs as you like.

### 数学 Math

Block

```Markdown
$$\begin{vmatrix}a & b\\
c & d
\end{vmatrix}=ad-bc$$
```

$$\begin{vmatrix}a & b\\
c & d
\end{vmatrix}=ad-bc$$

Inline

```Markdown
You can also do inline math like $e^{2i\pi} = 1$
```

You can also do inline math like $e^{2i\pi} = 1$

### 注释 Comments

```Markdown
Here is some inline comments: %%You can't see this text%% (Can't see it in Reading mode)

Here is a block comment: (can't see it in Reading mode either)
%%
It can span
multiple lines
%%
```

Here is some inline comments: %%You can't see this text%% (Can't see it in Reading mode)

Here is a block comment: (can't see it in Reading mode either)
%%
It can span
multiple lines
%%

### 标注 Callouts

```Markdown
> [!INFO]
> Here's a callout block.
> It supports **markdown**
```

> [!INFO]
> Here's a callout block.
> It supports **markdown**


补充：[Github Flavored Markdown(GFM)](https://github.github.com/gfm/)

> `Hammerspoon` 是一个 macos 上的自动化工具"##;
    let ast = Parser::new(input).parse();
    println!("{:?}", ast)
}

#[test]
fn parse_obsidian_advanced_blocks() {
    let input = r##"
### 脚注 Fontnotes

```Markdown
Here's a simple footnote,[^1] and here's a longer one.[^bignote]

[^1]: meaningful!

[^bignote]: Here's one with multiple paragraphs and code.

    Indent paragraphs to include them in the footnote.

    `{ my code }`

    Add as many paragraphs as you like.
```

Here's a simple footnote,[^1] and here's a longer one.[^bignote]

[^1]: meaningful!

[^bignote]: Here's one with multiple paragraphs and code.

    Indent paragraphs to include them in the footnote.

    `{ my code }`

    Add as many paragraphs as you like.

### 数学 Math

Block

```Markdown
$$\begin{vmatrix}a & b\\
c & d
\end{vmatrix}=ad-bc$$
```

$$\begin{vmatrix}a & b\\
c & d
\end{vmatrix}=ad-bc$$

Inline

```Markdown
You can also do inline math like $e^{2i\pi} = 1$
```

You can also do inline math like $e^{2i\pi} = 1$

### 注释 Comments

```Markdown
Here is some inline comments: %%You can't see this text%% (Can't see it in Reading mode)

Here is a block comment: (can't see it in Reading mode either)
%%
It can span
multiple lines
%%
```

Here is some inline comments: %%You can't see this text%% (Can't see it in Reading mode)

Here is a block comment: (can't see it in Reading mode either)
%%
It can span
multiple lines
%%

### 标注 Callouts

```Markdown
> [!INFO]
> Here's a callout block.
> It supports **markdown**
```

> [!INFO]
> Here's a callout block.
> It supports **markdown**


补充：[Github Flavored Markdown(GFM)](https://github.github.com/gfm/)

> `Hammerspoon` 是一个 macos 上的自动化工具"##;
    let ast = Parser::new(input).parse();
    println!("{:?}", ast)
}

#[test]
fn parse_ofm_task_list_items() {
    let input = r##"
- [x] #tags, [links](), **formatting** supported
- [x] list syntax required (any unordered or ordered list supported)
- [x] this is a complete item
- [?] this is also a complete item (works with every character)
- [ ] this is an incomplete item
- [ ] tasks can be clicked in Preview to be checked off"##;
    let ast = Parser::new_with_options(input, ParserOptions::default().enabled_ofm()).parse();
    println!("{:?}", ast)
}

#[test]
fn parse_ofm_nested_ordered_list() {
    let input = r##"
3. Item 3
	1. Item 3a
	2. Item 3b"##;
    let ast = Parser::new_with_options(input, ParserOptions::default().enabled_ofm()).parse();
    println!("{:?}", ast)
}
#[test]
fn parse_ofm_embed_and_cjk_paragraphs() {
    let input = r#"![[7c564948ca509ce4f46a5d0a97e1ef581db5de47200defc6d58f08e49aaa98eb.jpg]]

博客已经死了 3 年了，丢在 github 上也不怕丢失。

之前使用 Gatesby 生成的静态页面，build 一次很慢，要几分钟，一段时间不使用后，基于 Gatesby 写的那个 elegant 那个项目都运行不起来了😂，干脆放弃了。

后面使用 Obsidian 记录一些内容，因为其有一些独特的语法和功能，于是产生写一个解析 Markdown 为 AST 的工具。

用 Rust 断断续续写了几个月终于是完成，测试了下，性能没有其他 Rust / Go 写的快，特别是用 Rust 写的 pulldown-cmark 这个库，同样一个文件，这个库能达到 500μs，我的要 10ms...  
![[image_20240517193219.png]]

不过这个速度也将就够用了。

目前本博客支持大部分的 Obsidian 的  Markdown 功能显示，所有的附件文件都是基于相对路径匹配，不再像之前那样易出现找不到文件了。"#;

    let ast = Parser::new_with_options(input, ParserOptions::default().enabled_ofm()).parse();
    println!("{:?}", ast)
}

#[test]
fn parse_ofm_html_iframe_block() {
    let input = r#"喵喵喵
    
<iframe src="https://codesandbox.io/embed/react-function-component-gets-state-in-timer-vyv6g?autoresize=1&fontsize=14&hidenavigation=1&initialpath=%2Freact-shiyong-hanshu-shi-zujian-he-liangci-xuanran-wenti&module=%2Fsrc%2Fcomponents%2Fcounter.tsx&theme=light&view=preview"
     style="width:100%; height:500px; border:0; border-radius: 4px; overflow:hidden;"
     title="React function component gets state in timer"
     allow="geolocation; microphone; camera; midi; vr; accelerometer; gyroscope; payment; ambient-light-sensor; encrypted-media; usb"
     sandbox="allow-modals allow-forms allow-popups allow-scripts allow-same-origin"
   />"#;
    let ast = Parser::new_with_options(input, ParserOptions::default().enabled_ofm()).parse();
    println!("{:?}", ast)
}
