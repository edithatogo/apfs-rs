# Spec 0013: M-012 Real-Fixture Feedback Loop

Document version: 0.12.0  
Status: Implemented scaffold  
Codev phase: Specify

## Goal

Add a structured way to compare current `apfs inspect --json` output against a macOS-generated APFS fixture manifest and convert mismatches into actionable implementation tasks.

## Why this matters

Synthetic fixtures are useful for parser development, but they are not production APFS evidence. The next step is to run the parser against a real macOS-generated APFS sparse image and feed any mismatches back into Codev and Conductor.

## Non-goals

- Claiming real APFS support before running the loop.
- Mounting.
- File extraction.
- Write support.
- Raw-device access.
- Parsing personal disks.

## Requirements

- Accept an `inspect.json` file and a fixture manifest.
- Produce a machine-readable feedback report.
- Identify matched fields, mismatches, missing fields, and next tasks.
- Generate a Markdown task packet suitable for Codev/Conductor follow-up.
- Preserve redaction and no-personal-data rules.

## Acceptance

- `tools/real_fixture_feedback.py` exists.
- `cargo xtask real-fixture-feedback <inspect.json> <manifest.json> <out-dir>` exists.
- The output directory contains JSON and Markdown feedback artifacts.
- Conductor track `0012-real-fixture-feedback-loop` records this work.
