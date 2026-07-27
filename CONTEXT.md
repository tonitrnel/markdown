# Markdown Parser Context

## Glossary

- **Source document**: the immutable Markdown input from which parsing derives blocks, inline nodes, locations, and source-backed text.
- **Top-level Block**: a direct child of the `Document` node after its Block structure has been finalized. Frontmatter and the `Document` node itself are not Top-level Blocks.
- **Semantic target**: a node addressable because it is a Heading, has an OFM BlockId, or has both. A Heading denotes only itself, not the following section.
- **Pending Inline**: an Inline-capable node whose logical source segments have been recorded but whose ordinary Inline AST has not yet been materialized.
- **Selective document**: a result with complete Block structure but Inline AST materialized only for selected targets, their required descendants, and required footnote definitions.
