# APFS-RS Context and Control Surface

Version: 0.25.0

This handoff pack is controlled by three aligned context systems:

- **Codev**: specs, plans, reviews, capability registry, safety gates.
- **Conductor**: product, workflow, historical tracks, and context-management skills.
- **Cargoless validators**: Python/static checks that run before Rust/Cargo are available.

## Current-environment control stack

Run:

```bash
python3 tools/cargoless_smoke_suite.py
python3 tools/precompile_static_check.py
python3 tools/context_integrity_check.py
python3 tools/markdown_link_audit.py
python3 tools/shell_script_static_check.py
python3 tools/documentation_index_audit.py
python3 tools/fixture_coverage_report.py
python3 tools/current_env_completion_report.py
```

These checks do not replace `cargo test`; they keep the repository coherent until the package moves to a Rust-enabled machine.
