mod support;

use markdown::{Parser, ParserOptions};
use support::semantic::semantic_digest;

#[test]
fn semantic_digest_is_deterministic_for_product_syntax() {
    let source =
        "---\nzeta: last\nalpha: first\n---\n# Heading *one*\n\nTag #beta and :dog: $x$\n^block-id";
    let options = ParserOptions::default().enabled_ofm();

    let first = Parser::new_with_options(source, options.clone())
        .parse()
        .unwrap();
    let second = Parser::new_with_options(source, options).parse().unwrap();

    assert_eq!(semantic_digest(&first), semantic_digest(&second));
}

#[test]
fn semantic_digest_observes_structure_and_payload_changes() {
    let first = Parser::new("# title\n\nbody").parse().unwrap();
    let second = Parser::new("# title\n\nchanged").parse().unwrap();

    assert_ne!(semantic_digest(&first), semantic_digest(&second));

    let list = Parser::new("# title\n\n- body").parse().unwrap();
    assert_ne!(semantic_digest(&first), semantic_digest(&list));
}

#[test]
fn semantic_digest_observes_position_changes() {
    let first = semantic_digest(&Parser::new("# title\n\nbody").parse().unwrap());
    let second = semantic_digest(&Parser::new("\n# title\n\nbody").parse().unwrap());

    assert_eq!(
        first
            .nodes
            .iter()
            .map(|node| (
                &node.kind,
                &node.parent_preorder,
                &node.block_id,
                &node.payload
            ))
            .collect::<Vec<_>>(),
        second
            .nodes
            .iter()
            .map(|node| (
                &node.kind,
                &node.parent_preorder,
                &node.block_id,
                &node.payload
            ))
            .collect::<Vec<_>>(),
    );
    assert_ne!(
        first
            .nodes
            .iter()
            .map(|node| (node.start, node.end))
            .collect::<Vec<_>>(),
        second
            .nodes
            .iter()
            .map(|node| (node.start, node.end))
            .collect::<Vec<_>>(),
    );
}

#[test]
fn semantic_digest_observes_metadata_and_sorts_tags() {
    let options = ParserOptions::default().enabled_ofm();
    let first = semantic_digest(
        &Parser::new_with_options("# title\n\n#zeta #alpha\n^first-id", options.clone())
            .parse()
            .unwrap(),
    );
    let second = semantic_digest(
        &Parser::new_with_options("# title\n\n#zeta #alpha\n^second-id", options)
            .parse()
            .unwrap(),
    );

    assert_eq!(first.tags, ["alpha", "zeta"]);
    assert!(
        first
            .nodes
            .iter()
            .any(|node| node.block_id.as_deref() == Some("first-id"))
    );
    assert_ne!(first, second);
}

#[test]
fn semantic_digest_observes_html_changes() {
    let first = semantic_digest(&Parser::new("<div>first</div>").parse().unwrap());
    let second = semantic_digest(&Parser::new("<div>second</div>").parse().unwrap());

    assert_ne!(first.html, second.html);
    assert_ne!(first, second);
}
