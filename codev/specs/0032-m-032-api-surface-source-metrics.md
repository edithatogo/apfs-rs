# Spec 0032: M-032 Api Surface Source Metrics

Document version: 0.18.0  
Status: Implemented scaffold  
Codev phase: Specify

## Goal

Add cargoless CLI/API/source-metrics snapshots for review before Rust compilation.

## Safety

This slice is read-only with respect to APFS media. It must not mount, decrypt, repair, format, or write APFS sources.
