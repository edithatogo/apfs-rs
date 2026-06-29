# Future APFS-RS Windows read-only mount smoke test.
# Current status: scaffold only. It refuses to mount until apfs-win implements a WinFsp bridge.
param(
  [Parameter(Mandatory=$true)] [string]$Image,
  [Parameter(Mandatory=$true)] [string]$MountLetter
)

$ErrorActionPreference = "Stop"
if (-not (Test-Path $Image)) {
  throw "Image not found: $Image"
}
Write-Host "APFS-RS read-only Windows mount smoke scaffold"
Write-Host "Image: $Image"
Write-Host "Requested mount letter: $MountLetter"
Write-Host "No mount attempted: WinFsp FFI adapter is not implemented yet."
exit 0
