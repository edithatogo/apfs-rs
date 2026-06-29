# Documentation Automation

Version: 0.29.0

APFS-RS documentation uses the Astro 7 docs scaffold under `docs-site/` and is prepared for strict documentation checks.

## Required docs checks

- Astro build.
- Internal link audit.
- Documentation index audit.
- Mermaid diagram rendering check when available.
- API/CLI surface snapshots.
- Conductor/Codev history alignment.

The current environment can audit file structure and package declarations, but the Astro build itself requires npm install/build locally or in CI.
