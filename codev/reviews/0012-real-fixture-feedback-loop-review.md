# Review 0012: Real-Fixture Feedback Loop

Document version: 0.12.0  
Status: Implemented scaffold review  
Codev phase: Review

## What changed

- Added repository-root `REQUIREMENTS.md` with MoSCoW priorities.
- Added repository-root `DESIGN.md` with Mermaid diagrams.
- Added `REMAINING_ELEMENTS.md` with a quantified remaining-work ledger.
- Added `tools/real_fixture_feedback.py`.
- Added `cargo xtask real-fixture-feedback` command scaffold.
- Added Conductor track `0012-real-fixture-feedback-loop`.
- Updated Conductor history/tracks to include the full development history through M-012.

## Safety result

The feedback loop reads JSON files only and writes feedback artifacts. It does not mount, decrypt, extract, repair, format, or write APFS media.

## Remaining issue

The loop still needs to be run on a Rust/macOS-capable computer after `tools/macos/create_real_apfs_fixture.sh` creates the first real APFS image and manifest.
