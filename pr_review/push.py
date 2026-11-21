"""Hand a run's verdicts to the workflow step that posts them to the dashboard.

The status on each head is the gate; the dashboard only shows it. So this
makes no network call, runs after every status is written, and any problem it
meets is a warning: it can never change what a run publishes or how it ends.
"""

import json
import os
from pathlib import Path
import sys

FILE = 'verdicts.json'


def payload(repository, read_at, open_prs, rows):
    """What the service overlays on its snapshot.

    Each verdict is the row `report --format json` prints, unchanged. `open`
    is every open pull request, evaluated or not, so the board can hide the
    ones closed since its snapshot. `read_at` must be the time before the run
    read anything: a push is newer than a snapshot only if its reads are.
    """
    return {'repo': repository, 'read_at': read_at,
            'open': sorted({pr['number'] for pr in open_prs}),
            'verdicts': [{'number': row['number'], 'head': row['head'], 'verdict': row} for row in rows]}


def write(repository, read_at, open_prs, rows):
    """Leave the payload in the runner's temporary folder; outside Actions, do nothing."""
    folder = os.environ.get('RUNNER_TEMP')
    if not folder:
        return
    try:
        Path(folder, FILE).write_text(json.dumps(payload(repository, read_at, open_prs, rows)))
    except Exception as error:
        print(f'::warning::Verdicts not handed to the dashboard ({type(error).__name__}: {error}); '
              'it keeps showing its last snapshot', file=sys.stderr)
