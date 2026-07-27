# Markdown Parser Incremental Iteration Map

## Destination

Reach an approved, implementation-ready spec and blocker-ordered tracer tickets for an incremental parser programme: v2A is the first evidence-based challenge, v2B is the final challenge, and neither is reached through a whole-parser rewrite.

## Notes

- This map resolves decisions only. It does not implement parser behaviour or performance changes.
- Start each ticket from the current parser, Tree, tests, and benchmark evidence. Do not recover code from the discarded v2 experiment.
- Consult `CONTEXT.md`, `docs/adr/0001-source-backed-text-lifetime.md`, `docs/specs/2026-07-19-selective-inline-events-design.md`, and `docs/specs/2026-07-20-markdown-parser-performance-architecture-design.md`.
- Preserve one formal parsing path. A module refactor is allowed; parallel ParserV2/DocumentV2 engines, durable checkpoints, and whole-parser migration are out of scope.
- `/grilling` and `/domain-modeling` govern human decisions. `/prototype` is used only to make a specific interface or state model concrete.

## Decisions so far

- [v2A and v2B challenge framing](issues/01-v2a-and-v2b-challenge-framing.md) — v2A is the first challenge, v2B the final challenge; neither target is an unconditional delivery promise.
- [Session-bounded selective parsing](issues/02-session-bounded-selective-parsing.md) — top-level callbacks expose no durable identity, `Stop` is terminal, and selective output is explicitly partial.
- [Borrowed source-backed Text](issues/03-borrowed-source-backed-text.md) — Rust borrows source by default; an explicit owned path supports WASM and independent documents.
- [Trustworthy performance baseline](issues/04-trustworthy-performance-baseline.md) — P0 records reproducible allocation, phase, CommonMark, shared-GFM, and OFM-product lanes plus a deterministic semantic digest; it is the evidence input to ticket 05, not a module-order decision.
- [Selective parsing session interface](issues/06-selective-parsing-session-interface.md) — type-state phases (`Parser → BlockPhase → SemanticPhase → SelectiveParseOutput`) with callback-scoped event borrows, terminal Stop, full parse as the complete-selection path; implementable on the current Tree and pending map.
- [Source-backed text interface](issues/07-source-backed-text-interface.md) — `Document<'source>` with `SourceText::Borrowed/Owned`, `MarkdownNode::Text(TextRef)` resolved through the single `Document::text` seam, owned path via `Parser::parse_string` with no self-reference; HTML/Serde/WASM output byte-identical.
- [Evidence-based P7 verdict](issues/18-p7-evidence-based-redecision.md) — v2A closes with the 2x gate met and same-class full-AST parity; the maintainer directs v2B at Parse+AST only (rendering excluded); module order revised by the gate-lane profile: M1 = link/escape text lowering (tickets 19), M2 = line extraction + location (20), M3 = OFM owned-text / bounded inline commit (21).
- [v2B entry and final challenge](issues/08-define-v2b-entry-and-final-challenge.md) — v2B targets same-class superiority (not worse than comrak/rushdown on both datasets), OFM overhead <=5% over local CommonMark, and the allocation -50% tail; one module at a time with separate design reviews.
- [v2A module gates and order](issues/05-choose-v2a-module-gates-and-order.md) — F1→F2→F3→P1→P2→P3→P4→P5→(P6 evidence-gated)→P7, each slice with a named lane, no-regression rule, and semantic digest seam; first performance module is source-backed Text per the 2026-07-26 profile; deferral-to-v2B conditions named in the ticket. Converged into [spec.md](spec.md) and tracer tickets 09–18.

## Not yet specified

- (none — all current decisions resolved)

## Decisions so far (v2C)

- [v2C block phase and selective session](issues/22-v2c-block-phase-and-selective-session.md) — sessions cost ~50% of a full parse; lazy heading ref-text (Obsidian-style addressing, limited strip pass) replaces eager materialization; goals: _data session <=500us, corpus session <=1,100us, full parse no regression; order C2+C5 (23) -> C3 (24) -> C4 (25) -> C1 (26) with resampling gates. **v2C closed 2026-07-27**: corpus session goal met (1,875 -> 1,012us), _data session improved 728 -> 596us but the <=500us goal is honestly unmet (remainder is container-descent/line-loop, out of scope without a new block state machine — a separate map decision if ever pursued); full parse net improved. Tickets 22-26 all resolved.

## Decisions so far (WASM delivery)

- [WASM delivery-surface evidence](issues/27-wasm-delivery-surface-evidence.md) — SIMD local parse leads the compared JS/TS libraries, while full-tree JS-object serialization dominates end-to-end cost; prioritise selective session access, then boundary serialization, then SIMD build hardening.
- [W2 selective session over WASM](issues/28-w2-selective-session-over-wasm-binding.md) — expose stateless semantic-target queries and selected parsing over the binding, retaining stable node IDs only for byte-identical input; the target-query path avoids whole-tree transfer.
- [W1 full-tree boundary strategy](issues/29-w1-full-tree-boundary-strategy.md) — retain `.tree` for compatibility and add compact direct JSON serialization plus `JSON.parse`; it halves full-tree transfer and beats markdown-it on the large corpus, while the small-fixture <=8 ms target remains narrowly unmet.
- [W3 SIMD build hardening](issues/30-w3-simd-build-hardening.md) — wasm32 builds enable `+simd128` by default and document the runtime compatibility requirement.

## Out of scope

- Restoring or merging the discarded all-at-once v2 experiment.
- Parser state serialization, disk checkpointing, cross-process recovery, recovery after source changes, and durable NodeId contracts.
- Direct streaming HTML, a new Block state machine, generic bump allocation, and a second complete AST before a separately approved decision map changes scope.
