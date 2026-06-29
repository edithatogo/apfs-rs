# Astro 7 Documentation Plan

Document version: 0.27.0

## Decision

Use a static Astro 7 docs site scaffold under `docs-site/`, pinned to `astro@7.0.2` for this handoff.

## Why Astro

- Static documentation site is simple to publish.
- Minimal browser JavaScript by default.
- Good fit for generated docs, Codev/Conductor summaries, and handoff reports.

## Local commands

```bash
cd docs-site
npm install
npm run build
```

## Guardrails

- The docs site must not claim production APFS support until the production-claim guard permits it.
- Generated reports remain the source of truth for status/counts.
