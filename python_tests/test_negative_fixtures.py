"""Hypothesis-style checks for synthetic negative parser fixtures."""
from pathlib import Path
from hypothesis import given, strategies as st

ROOT = Path(__file__).resolve().parents[1]
NEGATIVE = list((ROOT / "fixtures" / "negative").glob("*.bin"))

@given(st.sampled_from(NEGATIVE))
def test_negative_fixtures_are_present_and_small(path: Path) -> None:
    assert path.exists()
    assert 0 < path.stat().st_size <= 8192

@given(st.sampled_from(NEGATIVE))
def test_negative_fixture_manifests_exist(path: Path) -> None:
    manifest = ROOT / "fixtures" / "manifests" / f"{path.stem}.json"
    assert manifest.exists()
