#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE_ID="${FIXTURE_ID:-macos-minimal-apfs-001}"
OUT_DIR="${OUT_DIR:-$ROOT_DIR/fixtures/real/$FIXTURE_ID}"
SIZE="${SIZE:-128m}"
VOLNAME="${VOLNAME:-APFSRSFixture}"
IMAGE="$OUT_DIR/$FIXTURE_ID.sparseimage"
MANIFEST="$OUT_DIR/manifest.json"
HASHES="$OUT_DIR/file-hashes.sha256"
ORACLE="$OUT_DIR/macos-oracle.redacted.txt"

mkdir -p "$OUT_DIR"
rm -f "$IMAGE" "$HASHES" "$ORACLE" "$MANIFEST"

command -v hdiutil >/dev/null || { echo "hdiutil not found; run on macOS" >&2; exit 1; }
command -v diskutil >/dev/null || { echo "diskutil not found; run on macOS" >&2; exit 1; }
command -v shasum >/dev/null || { echo "shasum not found" >&2; exit 1; }

hdiutil create -size "$SIZE" -type SPARSE -fs APFS -volname "$VOLNAME" "$IMAGE" >/dev/null

ATTACH_OUTPUT="$(hdiutil attach "$IMAGE" -readwrite -nobrowse)"
DEVICE="$(printf '%s\n' "$ATTACH_OUTPUT" | awk '/Apple_APFS/ {print $1; exit}')"
MOUNT_POINT="$(printf '%s\n' "$ATTACH_OUTPUT" | awk -v vol="/$VOLNAME" '$0 ~ vol {print $NF; exit}')"

if [[ -z "${MOUNT_POINT:-}" || ! -d "$MOUNT_POINT" ]]; then
  echo "Unable to determine APFS mount point" >&2
  printf '%s\n' "$ATTACH_OUTPUT" >&2
  exit 1
fi

cleanup() {
  sync || true
  hdiutil detach "$MOUNT_POINT" >/dev/null 2>&1 || true
}
trap cleanup EXIT

mkdir -p "$MOUNT_POINT/fixture-root/dir-a" "$MOUNT_POINT/fixture-root/dir-b"
printf 'APFS-RS synthetic fixture\n' > "$MOUNT_POINT/fixture-root/hello.txt"
printf 'Nested deterministic content\n' > "$MOUNT_POINT/fixture-root/dir-a/nested.txt"
python3 - <<PY
from pathlib import Path
p = Path('$MOUNT_POINT/fixture-root/dir-b/pattern.bin')
p.write_bytes(bytes([i % 251 for i in range(4096)]))
PY
ln -s ../hello.txt "$MOUNT_POINT/fixture-root/dir-a/link-to-hello" || true
sync

(
  cd "$MOUNT_POINT"
  find fixture-root -type f -print0 | sort -z | xargs -0 shasum -a 256
) > "$HASHES"

{
  echo '# diskutil apfs list -plist'
  diskutil apfs list -plist || true
  echo
  echo '# hdiutil imageinfo'
  hdiutil imageinfo "$IMAGE" || true
  echo
  echo '# find tree'
  (cd "$MOUNT_POINT" && find fixture-root -print | sort) || true
} > "$ORACLE"

IMAGE_SHA256="$(shasum -a 256 "$IMAGE" | awk '{print $1}')"

cat > "$MANIFEST" <<JSON
{
  "schema_version": "0.13.0",
  "fixture_id": "$FIXTURE_ID",
  "source_type": "macos_generated_apfs_sparseimage",
  "created_with": {
    "tool": "tools/macos/create_real_apfs_fixture.sh",
    "size": "$SIZE",
    "volume_name": "$VOLNAME"
  },
  "apfs_features": {
    "encrypted": false,
    "compressed": false,
    "snapshots": false,
    "case_sensitive": false,
    "volume_group": false,
    "fusion": false
  },
  "expected_artifacts": {
    "image": "$FIXTURE_ID.sparseimage",
    "file_hashes": "file-hashes.sha256",
    "macos_oracle_redacted": "macos-oracle.redacted.txt"
  },
  "image_sha256": "$IMAGE_SHA256",
  "capability_ids": ["M-001", "M-002", "M-003", "M-004", "M-011", "M-012"],
  "redaction": {
    "contains_personal_data": false,
    "contains_secret_material": false
  },
  "expected_inspect_fields": {
    "status": "apfs_container_detected",
    "safety.read_only": true,
    "safety.physical_write_supported": false
  },
  "safe_to_commit": "manifest_and_scripts_only_by_default"
}
JSON

cleanup
trap - EXIT

echo "Created $IMAGE"
echo "Manifest: $MANIFEST"
echo "Hashes: $HASHES"
echo "Oracle: $ORACLE"
