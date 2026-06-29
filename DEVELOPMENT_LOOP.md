# Batched Development Loop

Version: 0.21.0

Use this loop for future local or agent-assisted development.

```mermaid
flowchart TD
    A[Read REMAINING_ELEMENTS.yaml] --> B[Select several safe slices]
    B --> C[Update Codev spec/plan]
    C --> D[Implement code/tools/docs]
    D --> E[Update Conductor track]
    E --> F[Run cargoless checks]
    F --> G[Run cargo checks when available]
    G --> H[Promote failures to tracks]
    H --> I[Update review and remaining ledger]
```

## Loop rules

- Batch related safe slices together.
- Keep APFS media read-only.
- Promote failures to Codev/Conductor instead of hiding them.
- Keep `REQUIREMENTS.md`, `DESIGN.md`, `REMAINING_ELEMENTS.yaml`, Codev, and Conductor in sync.
- Do not erase history; add new tracks.
