# Batched Loop Policy

Version: 0.22.0

The repository is now maintained with a batched implementation loop while Rust/Cargo are unavailable in this environment.

## Loop

1. Read `REMAINING_ELEMENTS.yaml`, `NEXT_LOOP_PLAN.md`, `PRODUCTION_GAP_REPORT.md`, and `HANDOFF_STATUS.md`.
2. Choose several safe, local-only slices that do not require APFS media writes, live mounts, encryption bypass, repair, or format.
3. Implement code, tools, documentation, and fixtures that can be statically validated here.
4. Update Codev specs/plans/reviews.
5. Update Conductor tracks/history/skills.
6. Regenerate requirements, design, traceability, dashboards, manifests, and safety reports.
7. Run the cargoless validation stack.
8. Package the next source ZIP.

## Stop condition for this environment

Stop adding scaffolding when the only remaining high-value work requires:

- `cargo` / `rustc` compile and test feedback;
- macOS `hdiutil` / APFS fixture generation;
- Windows + WinFsp live mount validation.

## Safety rule

Every loop must preserve read-only defaults and must not add raw physical-device writes, APFS media mutation, encryption bypass, repair, format, or live write mounts.

## v0.23.0 current-environment hardening

Added current environment inventory, remaining-work classifier, dependency graph, synthetic negative fixtures, test/control matrix, and archive audit.
