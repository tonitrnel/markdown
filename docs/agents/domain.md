# Domain Docs

This repository uses a single-context domain documentation layout.

## Before exploring

Read these resources when they exist:

- `CONTEXT.md` at the repository root
- Relevant ADRs under `docs/adr/`

If they do not exist, proceed silently. The domain-modeling flows create them lazily when terminology or architectural decisions are resolved.

## Layout

```
/
├── CONTEXT.md
├── docs/
│   └── adr/
└── src/
```

## Vocabulary

When naming domain concepts in issues, proposals, hypotheses, and tests, use the terminology defined in `CONTEXT.md`.

If a required concept is absent, reconsider whether it belongs to the project vocabulary or note it for `/domain-modeling`.

## ADR conflicts

Explicitly flag output that conflicts with an existing ADR rather than silently overriding the recorded decision.
