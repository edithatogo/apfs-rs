# Spec M-026: Read-only VFS facade crate

Document version: 0.18.0  
Status: Implemented scaffold  
Codev phase: Specify

## Goal

Read-only VFS facade crate.

## Safety

Read-only and diagnostics/tooling-only. No physical-device writes, repair, format, encryption bypass, or production mount lifecycle.
