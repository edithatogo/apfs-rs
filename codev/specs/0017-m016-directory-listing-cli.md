# Spec 0017: M-016 Synthetic Directory Listing CLI

Document version: 0.15.0  
Status: Implementing  
Codev phase: Specify

## Goal

Expose the synthetic root-directory report through `apfs ls --json`.

## Non-goals

- Production APFS filesystem record decoding.
- Recursive directory traversal.
- Real APFS file extent resolution.
- Windows mounting.
- Write support.

## Acceptance

- The slice is read-only.
- Synthetic fixtures exercise the code path.
- JSON output includes warnings that the path is synthetic.
- Conductor track and capability registry are updated.
