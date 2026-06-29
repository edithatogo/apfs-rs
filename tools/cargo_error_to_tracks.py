#!/usr/bin/env python3
"""Promote Cargo compiler/test output into reviewable Codev/Conductor task stubs."""
from __future__ import annotations
import argparse, hashlib, json, re
from pathlib import Path

def slugify(text: str) -> str:
    text = re.sub(r'[^a-zA-Z0-9]+', '-', text.lower()).strip('-')
    return text[:60] or 'cargo-issue'

def parse_errors(text: str) -> list[dict[str, str]]:
    issues: list[dict[str, str]] = []
    lines = text.splitlines()
    for i, line in enumerate(lines):
        m = re.match(r'error(?:\[([E0-9]+)\])?:\s*(.*)', line)
        if not m:
            continue
        code = m.group(1) or 'cargo-error'
        message = m.group(2).strip() or 'Cargo error'
        location = ''
        for look in lines[i+1:i+8]:
            loc = re.search(r'-->\s+([^:]+):(\d+):(\d+)', look)
            if loc:
                location = f'{loc.group(1)}:{loc.group(2)}:{loc.group(3)}'
                break
        digest = hashlib.sha1((code + message + location).encode()).hexdigest()[:10]
        issues.append({'id': f'CARGO-{digest}', 'code': code, 'message': message, 'location': location})
    if not issues and text.strip():
        digest = hashlib.sha1(text.encode()).hexdigest()[:10]
        issues.append({'id': f'CARGO-{digest}', 'code': 'cargo-output', 'message': 'Cargo command produced output but no Rust error pattern was detected.', 'location': ''})
    return issues

def write_task_bundle(issues: list[dict[str, str]], out: Path) -> None:
    out.mkdir(parents=True, exist_ok=True)
    (out / 'cargo-triage.json').write_text(json.dumps({'schema_version':'0.20.0','issues':issues}, indent=2)+'\n', encoding='utf-8')
    lines = ['# Cargo Triage', '', f'- Issues detected: {len(issues)}', '']
    for issue in issues:
        lines.append(f"## {issue['id']}: {issue['code']}")
        lines.append('')
        lines.append(f"- Message: {issue['message']}")
        lines.append(f"- Location: {issue['location'] or 'unknown'}")
        lines.append('')
    (out / 'cargo-triage.md').write_text('\n'.join(lines), encoding='utf-8')
    for idx, issue in enumerate(issues, start=1):
        slug = slugify(issue['code'] + '-' + issue['message'])
        track_id = f'cargo-{idx:03d}-{slug}'
        codev_spec = out / 'codev' / 'specs' / f'{track_id}.md'
        codev_plan = out / 'codev' / 'plans' / f'{track_id}.md'
        conductor = out / 'conductor' / 'tracks' / track_id
        codev_spec.parent.mkdir(parents=True, exist_ok=True)
        codev_plan.parent.mkdir(parents=True, exist_ok=True)
        conductor.mkdir(parents=True, exist_ok=True)
        body = f"""# {track_id}: Cargo triage task\n\n## Error\n\n- Code: `{issue['code']}`\n- Message: {issue['message']}\n- Location: `{issue['location'] or 'unknown'}`\n\n## Safety\n\nFix compile/test issues without adding APFS media writes, unsafe code, or dependency changes unless separately reviewed.\n"""
        codev_spec.write_text(body, encoding='utf-8')
        codev_plan.write_text(body + '\n## Plan\n\n1. Reproduce locally.\n2. Apply minimal fix.\n3. Run cargo test and cargoless checks.\n', encoding='utf-8')
        (conductor / 'metadata.json').write_text(json.dumps({'track_id': track_id, 'status': 'generated', 'source': 'cargo_error_to_tracks'}, indent=2)+'\n', encoding='utf-8')
        (conductor / 'spec.md').write_text(body, encoding='utf-8')
        (conductor / 'plan.md').write_text('1. Reproduce the Cargo failure.\n2. Fix the smallest scope.\n3. Run required checks.\n', encoding='utf-8')

def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument('cargo_log', type=Path)
    parser.add_argument('out_dir', type=Path)
    args = parser.parse_args()
    text = args.cargo_log.read_text(encoding='utf-8', errors='replace')
    issues = parse_errors(text)
    write_task_bundle(issues, args.out_dir)
    print(f'cargo-error-to-tracks: wrote {len(issues)} issues to {args.out_dir}')
    return 0

if __name__ == '__main__':
    raise SystemExit(main())
