"""Optional Python Hypothesis-style fixture checks.

These tests are not part of the default cargoless smoke suite because the
Hypothesis package may not be installed in all environments. They are included
so local/CI environments can add Python property tests in addition to Rust
`proptest` property tests.
"""
from pathlib import Path
from hypothesis import given, strategies as st

ROOT = Path(__file__).resolve().parents[1]

def synthetic_extract_name_is_safe(name: str) -> bool:
    """Mirror the intended safe synthetic extraction-name policy."""
    normalized = name.replace("\\", "/")
    if not normalized or normalized.startswith("/"):
        return False
    parts = [part for part in normalized.split("/") if part]
    return len(parts) == 1 and parts[0] not in {".", ".."}

@given(st.sampled_from(list((ROOT / "fixtures").rglob("*.bin"))))
def test_binary_fixtures_are_nonempty(path: Path) -> None:
    assert path.stat().st_size > 0

@given(st.text(min_size=0, max_size=128))
def test_synthetic_extract_name_policy_rejects_path_traversal(name: str) -> None:
    normalized = name.replace("\\", "/")
    has_traversal = normalized.startswith("/") or "/" in normalized or normalized in {"", ".", ".."}
    if has_traversal:
        assert not synthetic_extract_name_is_safe(name)
    else:
        assert synthetic_extract_name_is_safe(name)
