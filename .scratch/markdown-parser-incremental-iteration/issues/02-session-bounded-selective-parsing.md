# Session-bounded selective parsing

**Type:** grilling
**Status:** resolved
**Blocked by:** None - historical decision recorded while charting the map.

## Question

What identity, interruption, and result guarantees may the first selective parsing capability make without turning a Markdown parser into a durable processing system?

## Answer

Top-level Block callbacks expose only a callback-scoped read-only view; they do not promise a saveable NodeId. After semantic preparation, IDs are valid only within the current in-memory session. `Stop` accepts the current Block prefix and is terminal: no serialization, disk checkpoint, cross-process recovery, source-change recovery, or resumed Block scan. A `SelectiveParseOutput`-style wrapper makes partial Inline materialization explicit instead of presenting it as an ordinary complete Document.

## Comments

- Resolved from the maintainer discussion on 2026-07-26.
