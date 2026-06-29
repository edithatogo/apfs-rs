# Spec 0025: Loop dashboard and backlog counter

Document version: 0.17.0  
Status: Implemented scaffold  
Codev phase: Specify

## Capability

`M-024`

## Goal

Add a cargoless development-quality slice that can run in constrained environments before Rust/Cargo is available.

## Safety

This slice reads repository files, manifests, generated JSON, or synthetic fixture bytes only. It does not mount, decrypt, repair, format, or write APFS media.
