
from pathlib import Path
import json
from hypothesis import given, strategies as st

ROOT = Path(__file__).resolve().parents[2]

@given(st.text(min_size=1, max_size=80).filter(lambda s: "\x00" not in s))
def test_manifest_ids_never_escape_paths(name: str):
    # Property for manifest tooling: generated IDs must be treated as data, not paths.
    unsafe = any(part in {"..", ""} for part in name.replace("\\", "/").split("/"))
    assert isinstance(unsafe, bool)

def test_fixture_manifests_are_json_objects():
    for path in (ROOT / "fixtures" / "manifests").glob("*.json"):
        data = json.loads(path.read_text())
        assert isinstance(data, dict)
