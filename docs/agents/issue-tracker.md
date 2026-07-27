# Issue tracker: Local Markdown

Issues and specs for this repo live as Markdown files in `.scratch/`.

## Conventions

- One feature per directory: `.scratch/<feature-slug>/`
- The spec is `.scratch/<feature-slug>/spec.md`
- Implementation issues are stored at `.scratch/<feature-slug>/issues/<NN>-<slug>.md`
- Ticket numbering starts at `01`
- Triage state is recorded as a `Status:` line near the top
- Comments are appended under a `## Comments` heading

## Publishing and fetching

When a skill says "publish to the issue tracker", create a file under `.scratch/<feature-slug>/`.

When a skill says "fetch the relevant ticket", read the referenced file.

## Wayfinding operations

- Map: `.scratch/<effort>/map.md`
- Child ticket: `.scratch/<effort>/issues/NN-<slug>.md`
- Type: `research`, `prototype`, `grilling`, or `task`
- Status: `claimed` or `resolved`
- Blocking: record dependencies as `Blocked by: NN, NN`
- Frontier: open, unblocked, unclaimed tickets; lowest number wins
- Claim: set `Status: claimed` before starting work
- Resolve: append an `## Answer`, set `Status: resolved`, and add a context pointer to the map's `Decisions so far`
