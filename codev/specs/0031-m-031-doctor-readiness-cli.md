# Spec 0031: M-031 Doctor Readiness Cli

Document version: 0.18.0  
Status: Implemented scaffold  
Codev phase: Specify

## Goal

Add a read-only doctor command that aggregates readiness and blockers from implemented inspect/resolver/directory surfaces.

## Safety

This slice is read-only with respect to APFS media. It must not mount, decrypt, repair, format, or write APFS sources.
