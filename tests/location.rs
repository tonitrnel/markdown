use markdown::ast::{MarkdownNode, math};
use markdown::parser::{Location, Parser, ParserOptions};
use markdown::tree::Tree;

fn loc_le(a: Location, b: Location) -> bool {
    (a.line, a.column) <= (b.line, b.column)
}

fn assert_location_invariants(tree: &Tree<markdown::Node>, idx: usize) {
    let node = &tree[idx];
    assert!(
        loc_le(node.start, node.end),
        "node {:?} has invalid range: {:?}..{:?}",
        node.body,
        node.start,
        node.end
    );

    let mut prev: Option<usize> = None;
    let mut cur = tree.get_first_child(idx);
    while let Some(child_idx) = cur {
        let child = &tree[child_idx];
        assert!(
            loc_le(node.start, child.start),
            "child {:?} starts before parent {:?}: {:?} < {:?}",
            child.body,
            node.body,
            child.start,
            node.start
        );
        assert!(
            loc_le(child.end, node.end),
            "child {:?} ends after parent {:?}: {:?} > {:?}",
            child.body,
            node.body,
            child.end,
            node.end
        );
        if let Some(prev_idx) = prev {
            let prev_node = &tree[prev_idx];
            assert!(
                loc_le(prev_node.start, child.start),
                "siblings out of order: prev {:?} at {:?}, curr {:?} at {:?}",
                prev_node.body,
                prev_node.start,
                child.body,
                child.start
            );
        }

        assert_location_invariants(tree, child_idx);
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
    let doc = Parser::new_with_options(
        input,
        ParserOptions::default().enabled_gfm().enabled_ofm(),
    )
    .parse();

    assert_location_invariants(&doc.tree, 0);
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
    assert_eq!(math_node.start, Location::new(1, 3));
    assert_eq!(math_node.end, Location::new(1, 6));

    let text_idx = doc
        .tree
        .get_first_child(math_idx)
        .expect("inline math text child missing");
    let text_node = &doc.tree[text_idx];
    assert_eq!(text_node.start, Location::new(1, 4));
    assert_eq!(text_node.end, Location::new(1, 5));
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
    assert_eq!(math_node.start, Location::new(1, 1));
    assert_eq!(math_node.end.line, 3);

    let text_idx = doc
        .tree
        .get_first_child(math_idx)
        .expect("display math text child missing");
    let text_node = &doc.tree[text_idx];
    assert_eq!(text_node.start, Location::new(1, 3));
    assert_eq!(text_node.end.line, 3);
    assert_eq!(math_node.end.column, text_node.end.column + 2);
}
