# Track generated-real-fixture-001-issue-0001 Spec

## Source

Generated from real-fixture feedback issue `RF-001`.

## Field

`issue-0001`

## Feedback

```text
expected `status` to be "apfs_container_detected", observed Some(String("not_apfs"))
```

## Requirement

Resolve this mismatch through read-only parser work, fixture manifest correction, or explicit unsupported-state documentation.

## Safety

This track must not introduce write, mount, repair, format, decryption, password recovery, or raw physical-device access.
