# ADR 0001: Source-Backed Text Borrows by Default

**Status:** Accepted

## Context

`MarkdownNode::Text(String)` allocates one owned string for every ordinary text node. On the curated corpus, the final `Document` alone retains 8,817 heap allocations out of 20,571 allocations per parse. Eliminating only temporary allocations therefore cannot reliably reach the performance challenge.

The parser already consumes a caller-provided `&str`. Rust callers commonly retain that source for the entire lifetime of a parsed document, while WASM needs an owned source because JavaScript passes an owned string across the binding boundary.

## Decision

The Text module will introduce source-backed text in the existing Tree:

- `Parser::new(&str)` produces a `Document<'source>` that borrows its Source document.
- An explicit owned-source construction path transfers a `String` into the resulting document for WASM and callers that need the document to outlive their input buffer.
- Text payloads store either a source range or transformed owned text. A document-bound text view resolves both forms.
- The existing Tree, Block algorithm, and non-Text payloads remain the formal AST during this change.
- HTML, Serde output, and WASM output continue to expose resolved text, not source ranges.

The owned-source path must avoid a self-referential parser: it parses through a temporary borrow, drops scanner and pending state, then moves the source into the final document.

## Consequences

- Rust AST users receive a targeted breaking change from direct `String` text access to a document-bound text view or equivalent accessor.
- Borrowed Rust parsing does not copy the full source document.
- The parser obtains a migration path to reduce ordinary Text allocations without a parallel AST or a whole-parser rewrite.
- Converted, generated, or logically non-contiguous text remains owned until separately proven worth optimizing.
