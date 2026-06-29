# APFS-RS Windows adapter readiness check.
# This script performs file-presence checks only. It does not mount or write anything.
param(
  [string]$RepoRoot = "."
)

$ErrorActionPreference = "Stop"
$required = @(
  "crates/apfs-vfs/Cargo.toml",
  "crates/apfs-vfs/src/lib.rs",
  "crates/apfs-win/Cargo.toml",
  "crates/apfs-win/src/lib.rs",
  "tools/windows/README.md"
)
foreach ($path in $required) {
  $full = Join-Path $RepoRoot $path
  if (-not (Test-Path $full)) {
    throw "Missing required Windows readiness file: $path"
  }
}
Write-Host "apfs-rs Windows readiness scaffold is present. No mount attempted."
