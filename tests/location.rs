use markdown::ast::{MarkdownNode, math};
use markdown::parser::{Parser, ParserOptions};
use markdown::tree::Tree;
use markdown::{Document, Location};

fn loc_le(a: Location, b: Location) -> bool {
    (a.line, a.column) <= (b.line, b.column)
}

fn assert_location_invariants(doc: &Document, idx: usize) {
    let tree = &doc.tree;
    let node = &tree[idx];
    let node_start = doc.location_at(node.span.start as usize);
    let node_end = doc.location_at(node.span.end as usize);
    assert!(
        loc_le(node_start, node_end),
        "node {:?} has invalid range: {:?}..{:?}",
        node.body,
        node_start,
        node_end
    );

    let mut prev: Option<usize> = None;
    let mut cur = tree.get_first_child(idx);
    while let Some(child_idx) = cur {
        let child = &tree[child_idx];
        let child_start = doc.location_at(child.span.start as usize);
        let child_end = doc.location_at(child.span.end as usize);
        assert!(
            loc_le(node_start, child_start),
            "child {:?} starts before parent {:?}: {:?} < {:?}",
            child.body,
            node.body,
            child_start,
            node_start
        );
        assert!(
            loc_le(child_end, node_end),
            "child {:?} ends after parent {:?}: {:?} > {:?}",
            child.body,
            node.body,
            child_end,
            node_end
        );
        if let Some(prev_idx) = prev {
            let prev_node = &tree[prev_idx];
            assert!(
                loc_le(doc.location_at(prev_node.span.start as usize), child_start),
                "siblings out of order: prev {:?} at {:?}, curr {:?} at {:?}",
                prev_node.body,
                doc.location_at(prev_node.span.start as usize),
                child.body,
                child_start
            );
        }

        assert_location_invariants(doc, child_idx);
        prev = Some(child_idx);
        cur = tree.get_next(child_idx);
    }
}

fn find_first_math_node(tree: &Tree<markdown::Node>) -> Option<usize> {
    let mut stack = vec![0usize];
    while let Some(idx) = stack.pop() {
        if matches!(tree[idx].body, MarkdownNode::Math(..)) {
            return Some(idx);
        }
        let mut child = tree.get_first_child(idx);
        while let Some(c) = child {
            stack.push(c);
            child = tree.get_next(c);
        }
    }
    None
}

#[test]
fn location_invariants_hold_for_mixed_document() {
    let input = r#"# H1

Paragraph with $x$ and **bold**.

> quote line
> $$\begin{align}
> x &= a + b
> \end{align}$$

1. item
2. item
"#;
    let doc = Parser::new_with_options(input, ParserOptions::default().enabled_gfm().enabled_ofm())
        .parse();

    assert_location_invariants(&doc, 0);
}

#[test]
fn inline_math_location_is_precise() {
    let input = "A $x$ B";
    let doc = Parser::new_with_options(input, ParserOptions::default().enabled_gfm()).parse();
    let math_idx = find_first_math_node(&doc.tree).expect("math node not found");
    let math_node = &doc.tree[math_idx];
    assert!(matches!(
        math_node.body,
        MarkdownNode::Math(ref m) if matches!(m.as_ref(), math::Math::Inline(..))
    ));
    assert_eq!(
        doc.location_at(math_node.span.start as usize),
        Location::new(1, 3)
    );
    assert_eq!(
        doc.location_at(math_node.span.end as usize),
        Location::new(1, 6)
    );

    let text_idx = doc
        .tree
        .get_first_child(math_idx)
        .expect("inline math text child missing");
    let text_node = &doc.tree[text_idx];
    assert_eq!(
        doc.location_at(text_node.span.start as usize),
        Location::new(1, 4)
    );
    assert_eq!(
        doc.location_at(text_node.span.end as usize),
        Location::new(1, 5)
    );
}

#[test]
fn multiline_display_math_location_is_consistent() {
    let input = "$$\\begin{vmatrix}a & b\\\\\nc & d\n\\end{vmatrix}=ad-bc$$";
    let doc = Parser::new_with_options(input, ParserOptions::default().enabled_gfm()).parse();
    let math_idx = find_first_math_node(&doc.tree).expect("math node not found");
    let math_node = &doc.tree[math_idx];
    assert!(matches!(
        math_node.body,
        MarkdownNode::Math(ref m) if matches!(m.as_ref(), math::Math::Block(..))
    ));
    assert_eq!(
        doc.location_at(math_node.span.start as usize),
        Location::new(1, 1)
    );
    let math_end = doc.location_at(math_node.span.end as usize);
    assert_eq!(math_end.line, 3);

    let text_idx = doc
        .tree
        .get_first_child(math_idx)
        .expect("display math text child missing");
    let text_node = &doc.tree[text_idx];
    assert_eq!(
        doc.location_at(text_node.span.start as usize),
        Location::new(1, 3)
    );
    let text_end = doc.location_at(text_node.span.end as usize);
    assert_eq!(text_end.line, 3);
    assert_eq!(math_end.column, text_end.column + 2);
}

// ---------------------------------------------------------------------------
// M2 位置模型矩阵（ticket 20）：在 LF/CRLF/孤立 CR/tab/CJK/emoji/末尾空行等样本上
// 断言 `Document::location_at` 与独立朴素实现逐字符边界相等；golden 钉住全树节点
// 位置的现值，作为位置表示切换阶段的回归锚。
// ---------------------------------------------------------------------------

const MATRIX: &[(&str, &str)] = &[
    ("lf", "# h\né **b** c\n"),
    ("crlf", "# t\r\npara **x**\r\n\r\n- a\r\nend\r\n"),
    ("lone_cr", "a\rb\r"),
    ("tabs", "A\tB\n\tcode\n- x\t y\n"),
    ("cjk", "# 标题\n中文**段落**测试\n> 引用行\n"),
    ("emoji", "a 👩‍👩‍👧‍👦 b\n**🎉x🎉**\n"),
    ("trailing_blank", "x\n\n\n"),
    ("no_trailing_newline", "end **b**"),
    ("empty", ""),
    ("mixed_eol", "α\r\nβ\nγ\rδ"),
];

fn matrix_parse(source: &str) -> markdown::Document<'_> {
    Parser::new_with_options(source, ParserOptions::default().enabled_gfm().enabled_ofm()).parse()
}

fn naive_location(source: &str, offset: usize) -> Location {
    let offset = offset.min(source.len());
    let bytes = source.as_bytes();
    let line = 1 + bytes[..offset].iter().filter(|&&b| b == b'\n').count() as u64;
    let line_start = bytes[..offset]
        .iter()
        .rposition(|&b| b == b'\n')
        .map_or(0, |i| i + 1);
    let column = 1 + source[line_start..offset].chars().count() as u64;
    Location::new(line, column)
}

#[test]
fn location_at_matches_naive_reference_on_matrix() {
    for (name, source) in MATRIX {
        let doc = matrix_parse(source);
        for offset in (0..=source.len()).filter(|&o| source.is_char_boundary(o)) {
            assert_eq!(
                doc.location_at(offset),
                naive_location(source, offset),
                "{name}: offset {offset}"
            );
        }
        assert_eq!(
            doc.location_at(source.len() + 7),
            naive_location(source, source.len()),
            "{name}: clamp"
        );
    }
}

fn dump_positions(doc: &Document, idx: usize, depth: usize, out: &mut String) {
    let node = &doc.tree[idx];
    let start = doc.location_at(node.span.start as usize);
    let end = doc.location_at(node.span.end as usize);
    out.push_str(&format!(
        "d{depth} {},{}-{},{}\n",
        start.line, start.column, end.line, end.column
    ));
    let mut child = doc.tree.get_first_child(idx);
    while let Some(c) = child {
        dump_positions(doc, c, depth + 1, out);
        child = doc.tree.get_next(c);
    }
}

#[test]
fn node_positions_match_golden_on_matrix() {
    let mut actual = String::new();
    for (name, source) in MATRIX {
        let doc = matrix_parse(source);
        actual.push_str(&format!("== {name}\n"));
        dump_positions(&doc, 0, 0, &mut actual);
    }
    if actual != GOLDEN_MATRIX_POSITIONS {
        eprintln!("--- actual matrix positions ---\n{actual}\n--- end ---");
        panic!("matrix node positions drifted from golden");
    }
}

// byte-span 主表示落地时（M2 phase b）经设计评审接受的两处偏差，其余逐字节不变：
// 1. SoftBreak/HardBreak 的 end 列：急切实现为「行内容末列+1」的幻影列（指向行终止符，
//    纯偏移模型不可表示）；现为行内容终点列。
// 2. 孤立 \r（classic Mac 行尾）后的列：急切实现只累计已消费的终止符（skip_to_eol 不更新列，
//    数值不自洽）；现为距行起点的字符数。
const GOLDEN_MATRIX_POSITIONS: &str = "\
== lf\n\
d0 1,1-2,10\n\
d1 1,1-1,4\n\
d2 1,3-1,4\n\
d1 2,1-2,10\n\
d2 2,1-2,3\n\
d2 2,3-2,8\n\
d3 2,5-2,6\n\
d2 2,8-2,10\n\
== crlf\n\
d0 1,1-5,4\n\
d1 1,1-1,4\n\
d2 1,3-1,4\n\
d1 2,1-2,11\n\
d2 2,1-2,6\n\
d2 2,6-2,11\n\
d3 2,8-2,9\n\
d1 4,1-5,4\n\
d2 4,1-5,4\n\
d3 4,3-5,4\n\
d4 4,3-4,4\n\
d4 4,4-4,4\n\
d4 5,1-5,4\n\
== lone_cr\n\
d0 1,1-1,4\n\
d1 1,1-1,4\n\
d2 1,1-1,2\n\
d2 1,2-1,2\n\
d2 1,3-1,4\n\
== tabs\n\
d0 1,1-3,7\n\
d1 1,1-3,1\n\
d2 1,1-1,4\n\
d2 1,4-1,4\n\
d2 2,2-2,6\n\
d1 3,1-3,7\n\
d2 3,1-3,7\n\
d3 3,3-3,7\n\
d4 3,3-3,7\n\
== cjk\n\
d0 1,1-3,6\n\
d1 1,1-1,5\n\
d2 1,3-1,5\n\
d1 2,1-3,1\n\
d2 2,1-2,3\n\
d2 2,3-2,9\n\
d3 2,5-2,7\n\
d2 2,9-2,11\n\
d1 3,1-3,6\n\
d2 3,3-3,6\n\
d3 3,3-3,6\n\
== emoji\n\
d0 1,1-2,8\n\
d1 1,1-2,8\n\
d2 1,1-1,12\n\
d2 1,12-1,12\n\
d2 2,1-2,8\n\
d3 2,3-2,6\n\
== trailing_blank\n\
d0 1,1-1,2\n\
d1 1,1-1,2\n\
d2 1,1-1,2\n\
== no_trailing_newline\n\
d0 1,1-1,10\n\
d1 1,1-1,10\n\
d2 1,1-1,5\n\
d2 1,5-1,10\n\
d3 1,7-1,8\n\
== empty\n\
d0 1,1-1,1\n\
== mixed_eol\n\
d0 1,1-3,4\n\
d1 1,1-3,4\n\
d2 1,1-1,2\n\
d2 1,2-1,2\n\
d2 2,1-2,2\n\
d2 2,2-2,2\n\
d2 3,1-3,2\n\
d2 3,2-3,2\n\
d2 3,3-3,4\n\
";
