# Borrowed source-backed Text

**Type:** grilling
**Status:** resolved
**Blocked by:** None - historical decision recorded while charting the map.

## Question

What source-lifetime contract permits source-backed Text to remove ordinary text allocations without forcing all Rust callers to copy the complete Markdown input?

## Answer

The default Rust path borrows its source for the lifetime of `Document<'source>`. An explicit owned-source path transfers a String into the completed document for WASM and callers that need the document to outlive the input. Text stores source ranges or transformed owned text, while HTML, Serde, and WASM expose resolved strings. The owned path must not create a self-referential parser.

## Comments

- This decision is also recorded in `docs/adr/0001-source-backed-text-lifetime.md`.
