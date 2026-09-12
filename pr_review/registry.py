"""The repository registry: which repositories are governed and where each policy lives."""

import json
from pathlib import Path, PurePosixPath
import re

ROOT = Path(__file__).resolve().parents[1]
POLICIES = ROOT / 'policies'
CENTRAL_REPOSITORY = 'dashpay/stale_prs_are_bad'


def validate_registry(registry):
    if (not isinstance(registry, dict) or set(registry) != {'version', 'repositories'}
            or type(registry.get('version')) is not int or registry['version'] != 1
            or not isinstance(registry.get('repositories'), list) or not registry['repositories']):
        raise ValueError('Invalid review repository registry')
    seen = set()
    for entry in registry['repositories']:
        if not isinstance(entry, dict) or set(entry) != {'repository', 'policy', 'mode'}:
            raise ValueError('Invalid repository registry entry schema')
        repo = entry.get('repository', '')
        path = entry.get('policy', '')
        if (not isinstance(repo, str) or not re.fullmatch(r'[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+', repo)
                or repo.lower() in seen or entry.get('mode') not in {'preview', 'active'}
                or not isinstance(path, str) or not path or PurePosixPath(path).is_absolute()
                or len(PurePosixPath(path).parts) != 1 or '\\' in path):
            raise ValueError('Invalid or duplicate repository registry entry')
        seen.add(repo.lower())


def load_registry(policies_root=POLICIES):
    registry = json.loads((Path(policies_root) / 'repositories.json').read_text())
    validate_registry(registry)
    return registry


def entry_for(registry, repository):
    entry = next((e for e in registry['repositories'] if e['repository'] == repository), None)
    if entry is None:
        raise ValueError('Repository is not in the configured registry')
    return entry


def policy_path(policies_root, entry):
    path = (Path(policies_root) / entry['policy']).resolve()
    if path.parent != Path(policies_root).resolve():
        raise ValueError('Policy path escapes the policies directory')
    return path


def repositories(policies_root=POLICIES):
    return {entry['repository'] for entry in load_registry(policies_root)['repositories']}
