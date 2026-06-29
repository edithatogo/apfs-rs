# APFS-RS Safety Case

Document version: 0.18.0

## Claim

APFS-RS is currently safe to develop and test against synthetic image fixtures because every implemented path follows a read-only default with respect to APFS media, every user-visible write goes only to an explicit host output directory, and unsupported APFS states are reported as diagnostic or refusal states.

## Scope

This safety case covers the current synthetic/read-only implementation through M-030. It does not claim production APFS compatibility, Windows mount safety, encryption support, repair safety, format safety, or write safety.

## Critical hazards and mitigations

| Hazard | Risk | Current mitigation |
|---|---|---|
| raw physical-device write | Catastrophic data loss | No raw physical-device access or write APIs are implemented; safety gates and static checks scan for suspicious terms. |
| metadata corruption | Data loss if write support exists | Write support is not implemented; future write support requires image-only lab evidence first. |
| secret leakage | Disclosure of passwords, keys, or filenames | Encryption support is not implemented; diagnostics export is redacted and excludes raw APFS blocks, file contents, passwords, and keys. |
| path traversal | Host filesystem overwrite during extraction | Synthetic extract requires a single file name with no path separators, rejects absolute paths and `..`, and writes only to an explicit host destination. |
| unsupported encryption | False access expectations or unsafe bypass | Unsupported encryption remains diagnostic-only; no password recovery, bypass, or key extraction exists. |
| panic on corrupt input | Tool crash or denial of service on malicious images | Parser paths use bounds checks and typed diagnostics; precompile checks and future fuzzing target corrupt inputs. |
| unsupported feature misrepresentation | User trusts unsupported APFS features | CLI doctor, compatibility report, requirements, and remaining-element ledger explicitly mark unsupported features. |
| agent context drift | Coding agents modify unsafe areas without history | Codev and Conductor track history must be updated for every slice; conductor-check verifies the historical ledger. |

## Safety boundaries

```text
Allowed now:
  synthetic image fixture reads
  read-only inspection
  synthetic resolver lookup
  synthetic directory/file preview
  host-side redacted diagnostics export
  host-side synthetic extract preview

Not allowed now:
  APFS media writes
  raw physical-device access
  Windows mount adapter writes
  encryption bypass
  repair
  format
```

## Evidence available before Rust compilation

- `tools/precompile_static_check.py`
- `tools/synthetic_fixture_oracle.py`
- `tools/cli_contract_snapshot.py`
- `tools/api_surface_snapshot.py`
- `tools/source_metrics.py`
- `tools/safety_case_check.py`
- `tools/release_preflight.py`
- Codev capability and safety registries
- Conductor historical tracks
- Fixture manifests, fixture evidence, and SHA-256 manifest

## Evidence required before MVP release

1. Rust compilation and tests on Linux, macOS, and Windows.
2. First real macOS APFS fixture and oracle manifest.
3. Parser corrections from the real-fixture feedback loop.
4. Full checkpoint and production object-map traversal evidence.
5. Production directory and file extraction evidence.
6. Windows WinFsp read-only smoke tests.
7. Fuzz/property/mutation/coverage evidence.
8. Release provenance, SBOM, and signed artifacts.

## Codev and Conductor linkage

Every safety-sensitive feature must update:

- Codev spec, plan, and review.
- Conductor track metadata, spec, and plan.
- `conductor/history.md` and `conductor/tracks.md`.
- `REQUIREMENTS.md`, `DESIGN.md`, and `REMAINING_ELEMENTS.yaml`.

Codev captures implementation intent and review evidence. Conductor captures persistent context for agents and future contributors.
