# Spec 0033: M-033 Safety Case Preflight

Document version: 0.18.0  
Status: Implemented scaffold  
Codev phase: Specify

## Goal

Add a safety case and cargoless checker for critical hazards, mitigations, evidence, and non-goals.

## Safety

This slice is read-only with respect to APFS media. It must not mount, decrypt, repair, format, or write APFS sources.
