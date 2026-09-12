"""Fail when a policy's required_bots contradicts the repository's own CodeRabbit setting."""

import json
import pathlib
import sys

import yaml

policies = pathlib.Path('proposed/policies')
registry = json.loads((policies / 'repositories.json').read_text())
problems = []
for entry in registry['repositories']:
    name = entry['repository'].split('/')[1]
    policy = json.loads((policies / entry['policy']).read_text())
    required = 'coderabbitai' in policy.get('required_bots', ['thepastaclaw', 'coderabbitai'])
    tree = pathlib.Path('trees') / name
    config = next((tree / candidate for candidate in ('.coderabbit.yaml', '.coderabbit.yml')
                   if (tree / candidate).is_file()), None)
    disabled = False
    if config is not None:
        settings = yaml.safe_load(config.read_text()) or {}
        disabled = (settings.get('reviews') or {}).get('auto_review', {}).get('enabled') is False
    if disabled and required:
        problems.append(f'{entry["repository"]}: {config.name} disables automatic CodeRabbit reviews, '
                        'so requiring coderabbitai would never resolve; drop it from required_bots')
    elif not disabled and not required:
        print(f'::warning::{entry["repository"]}: required_bots omits coderabbitai but nothing in the '
              'repository disables automatic reviews; confirm this is deliberate')
for problem in problems:
    print(f'::error::{problem}')
sys.exit(1 if problems else 0)
