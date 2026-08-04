//! A high-performance, AST-first Markdown parser.
//!
//! PTDGRP Markdown parses CommonMark, GitHub Flavored Markdown (GFM), and
//! Obsidian Flavored Markdown (OFM) into an arena-backed syntax tree. Every AST
//! node has a source byte span, and unchanged text can remain as a zero-copy
//! reference into the original input.
//!
//! # Quick start
//!
//! ```
//! use ptdgrp_markdown::{MarkdownNode, Parser, ParserOptions};
//!
//! let document = Parser::new_with_options(
//!     "# Hello\n\nThis is **Markdown**.",
//!     ParserOptions::default().enabled_gfm(),
//! )
//! .parse()?;
//!
//! assert!(matches!(document.tree[0].body, MarkdownNode::Document));
//! assert!(document.tree.len() > 1);
//! # Ok::<(), ptdgrp_markdown::ParseError>(())
//! ```
//!
//! Node ID `0` is always the document root. Use [`Tree::get_first_child`],
//! [`Tree::get_next`], [`Tree::get_parent`], and the related methods to navigate
//! the AST.
//!
//! # Resolving text
//!
//! Text nodes may contain [`ast::text::TextRef::Source`], which stores a byte
//! range instead of allocating a new string. Resolve it through the owning
//! [`Document`]:
//!
//! ```
//! use ptdgrp_markdown::{MarkdownNode, Parser};
//!
//! let document = Parser::new("hello").parse()?;
//! let paragraph = document.tree.get_first_child(0).unwrap();
//! let text_node = document.tree.get_first_child(paragraph).unwrap();
//!
//! if let MarkdownNode::Text(text) = &document.tree[text_node].body {
//!     assert_eq!(document.text(text), "hello");
//! }
//! # Ok::<(), ptdgrp_markdown::ParseError>(())
//! ```
//!
//! # Owned input
//!
//! [`Parser::new`] borrows the input. Use [`Parser::parse_string`] when the
//! returned document must own its source, such as across a WASM or FFI boundary.
//!
//! # Parser extensions
//!
//! Configure syntax and text processing with [`ParserOptions`]. Common choices
//! include [`ParserOptions::enabled_gfm`], [`ParserOptions::enabled_ofm`],
//! [`ParserOptions::enabled_smart_punctuation`], and
//! [`ParserOptions::enabled_cjk_autocorrect`]. Resource limits for untrusted
//! input are available through [`ParserOptions::with_max_input_bytes`] and
//! [`ParserOptions::with_max_nodes`].
//!
//! # Block-only and selective parsing
//!
//! [`Parser::parse_blocks`] returns a [`BlockDocument`] whose block tree is
//! complete but whose inline nodes have not been materialized. Call
//! [`BlockDocument::materialize_all`] for a full document, or
//! [`BlockDocument::prepare_semantics`] to inspect headings and OFM block IDs
//! before selectively materializing inline content. See the [`selective`]
//! module for the complete workflow.
//!
//! ## Selecting a block by Block ID
//!
//! Semantic preparation discovers OFM block IDs before inline materialization:
//!
//! ```
//! use ptdgrp_markdown::{InlineSelection, Parser, ParserOptions, VisitControl};
//!
//! let source = "Ignore.\n\nParse **this**. ^chosen\n\nSkip.";
//! let mut phase = Parser::new_with_options(
//!     source,
//!     ParserOptions::default().enabled_ofm(),
//! )
//! .parse_blocks()?
//! .prepare_semantics()?;
//!
//! let mut selection = InlineSelection::default();
//! let mut chosen = None;
//! phase.visit_semantic_targets(
//!     |target| target.block_id() == Some("chosen"),
//!     &mut selection,
//!     |target, selection| {
//!         chosen = Some(target.node_id());
//!         selection.select(target.node_id());
//!         VisitControl::Stop
//!     },
//! );
//!
//! let output = phase.parse_selected_inlines(selection)?;
//!
//! assert!(output
//!     .document
//!     .tree
//!     .get_first_child(chosen.expect("block ID not found"))
//!     .is_some());
//! # Ok::<(), ptdgrp_markdown::ParseError>(())
//! ```
//!
//! To parse a heading section, find the heading through [`SemanticTarget`], then
//! select the heading and its following siblings until the next heading of the
//! same or a higher level. The [`selective`] module contains a complete example.
//!
//! # HTML
//!
//! With the default `html` feature enabled, [`Document::to_html`] renders the
//! parsed document. HTML rendering is a convenience and integration surface;
//! the primary output of this crate is the AST.

pub mod ast;
mod blocks;
mod document;
mod exts;
mod inlines;
mod location;
mod node;
pub mod parser;
mod pending;
mod render;
pub(crate) mod scanner;
pub mod selective;
mod semantic;
pub(crate) mod span;
pub mod tree;
mod utils;

pub use ast::*;
pub use document::*;
pub use location::*;
pub use node::*;
pub use parser::*;
pub use selective::*;
pub use tree::*;
