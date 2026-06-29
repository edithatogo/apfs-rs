# Spec 0016: M-015 Synthetic Filesystem Root Tree Records

Document version: 0.15.0  
Status: Implementing  
Codev phase: Specify

## Goal

Parse bounded synthetic filesystem-directory records from a mapped volume root-tree B-tree node.

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
