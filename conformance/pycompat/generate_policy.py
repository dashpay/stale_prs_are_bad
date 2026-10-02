"""What Python 3.12 does where the Rust port of pr_review/policy.py depends on
it, beyond what generate.py already records:

- conformance/pycompat/policy_regex.json: every regular expression policy.py
  uses, with what Python's `re` matched on curated inputs and seeded random
  ones; the engine's `policy::patterns` tests replay it;
- conformance/pycompat/object.json: `repr` of floats, `str()` of values,
  `==` and `<` between values, and the characters whose `str.upper()` is
  ASCII; the engine's `tests/pycompat/object.rs` replays it.

The patterns are taken from pr_review/policy.py itself where it names them,
and where it writes one inline the copy here must appear in its source
verbatim, so a pattern changed there and not here stops this script.

    uv run -q --python 3.12 --no-project python conformance/pycompat/generate_policy.py

The output is deterministic: a second run writes the same bytes.
"""

import json
import random
import re
import struct
import sys
from pathlib import Path

if sys.version_info[:2] != (3, 12):
    raise SystemExit(f'generate_policy.py must run on Python 3.12, not {sys.version.split()[0]}')

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT))
from pr_review import policy  # noqa: E402

PYTHON = sys.version.split()[0]
OUT = ROOT / 'conformance' / 'pycompat'
SOURCE = (ROOT / 'pr_review' / 'policy.py').read_text()


def in_source(text):
    """`text` as policy.py writes it; stops the script where it does not."""
    if text not in SOURCE:
        raise SystemExit(f'policy.py no longer contains {text!r}: update generate_policy.py and the port')
    return text


def inline(pattern):
    """A pattern policy.py writes inline as a raw string."""
    in_source(f"r'{pattern}'")
    return pattern


def write_json(name, cases, **body):
    """One case per line, so a regenerated file diffs line by line."""
    head = json.dumps({'python': PYTHON, **body}, ensure_ascii=True)[:-1]
    lines = [json.dumps(c, ensure_ascii=True, separators=(',', ':')) for c in cases]
    text = head + ',"cases":[\n' + ',\n'.join(lines) + '\n]}\n'
    json.loads(text)
    (OUT / name).write_text(text, encoding='ascii')


# --- regular expressions ------------------------------------------------------

HEX40 = '0123456789abcdef' * 2 + '01234567'
HEAD = 'a' * 40
# Characters where Python's `re` and the regex crate's own Unicode data, or
# a careless port, could disagree: whitespace `\s` does or does not hold,
# letters `(?i)` folds onto ASCII, digits `\d` holds, line breaks.
EDGE = ['\t', '\n', '\x0b', '\x0c', '\r', '\x1c', '\x1d', '\x1e', '\x1f', ' ', '\x85', '\xa0',
        '\u1680', '\u180e', '\u2000', '\u200b', '\u2028', '\u2029', '\u3000', '\ufeff',
        '\u0130', '\u0131', '\u017f', '\u212a', '\xdf', '\u1e9e', '\u0660', '\uff12', '\U0001d7ce',
        '\xb2', '\u2139', '\u2705', '|', '\\', '_', '*', '-', ':', '<', '>', '!', '"', '{', '}']
ALPHABET = list('ab|-:_*<>!{}" \n\\x') + EDGE


def mutate(rng, text):
    """`text` with a few characters inserted, deleted or replaced."""
    chars = list(text)
    for _ in range(rng.randint(1, 3)):
        op = rng.choice('idr')
        at = rng.randint(0, len(chars))
        if op == 'i' or not chars:
            chars.insert(at, rng.choice(ALPHABET))
        elif op == 'd':
            del chars[min(at, len(chars) - 1)]
        else:
            chars[min(at, len(chars) - 1)] = rng.choice(ALPHABET)
    return ''.join(chars)


def rabbit_body(**kw):
    """CodeRabbit's summary comment, in the shape test_policy.py builds it."""
    covered = json.dumps({'sourceCommitId': HEAD, 'coveredCommitId': HEAD, 'kind': 'reviewed'},
                         separators=(',', ':'))
    return ('<!-- This is an auto-generated comment: summarize by coderabbit.ai -->\n'
            '<!-- review_stack_entry_start -->\nbanner\n<!-- review_stack_entry_end -->\n'
            '<!-- recent_review_start -->\nNo actionable comments were generated.\n'
            '<details><summary>\u2699\ufe0f Run configuration</summary>\nRun ID: d5a7\n</details>\n'
            '<!-- recent_review_end -->\n'
            '| Layer / File(s) | Summary |\n| :--- | :--- |\n| `a.rs` | Adds a field. |\n'
            '<!-- final_review_risk_start -->\n**Merge Risk:** Minimal\n'
            f'<!-- final_review_risk_coverage:{covered} -->\n'
            + kw.get('finding', '') + '\n<!-- final_review_risk_end -->\n'
            '<summary>\U0001f6a5 Pre-merge checks | \u2705 Passed</summary>\n\n'
            '| Check name | Status | Explanation |\n| :---: | :--- | :--- |\n'
            '| Title check | \u2705 Passed | It reads well. |\n'
            '| Team sanity \\| \u2705 ok | \u26a0\ufe0f Warning | Do not merge. |\n'
            '- [ ] <!-- {"checkboxId":"585bb3f6"} --> Fix all pre-merge checks\n'
            '<!-- tips_start -->\n<details><summary>\U0001fab7 Tips</summary>\nChat.\n</details>\n'
            '<!-- tips_end -->\n')


RABBIT = rabbit_body()
SECTIONS = ([(f'<!-- {a}_start -->', f'<!-- {b}_end -->') for a, b in policy.VOLATILE]
            + list(policy.NOTICES))
in_source("[(f'<!-- {a}_start -->', f'<!-- {b}_end -->') for a, b in VOLATILE] + list(NOTICES)")


def section_inputs(start, end):
    return [
        f'{start}\nnoise\n{end}\nkept', f'x\n{start}  \t\nnoise\n{end} \nkept', f'a {start}\nb\n{end}\n',
        f'{start}\n{start}\nb\n{end}\n', f'{start}\nno end', f'{start}\r\nnoise\r\n{end}\r\n',
        f'{start}\u3000\nnoise\n{end}\n', f'{start}\nx{end}\n{end}\nkept', f'{start}{end}',
        f'{start}\n\n{end}\n{start}\n{end}', f'{start} x\n{end}\n', f'{start}\n{end}\x85\n',
    ]


def bookkeeping_inputs():
    names = ['Run configuration', 'RUN CONFIGURATION', 'Commits', 'commits (3)', 'Files selected for processing',
             'files selected for processing (17)', 'Recent review info', 'Commits with problems',
             'Nitpick comments (1)', 'Files selected and rejected', 'Run \u017fonfiguration',
             'Run configuratIon', 'Run conf\u0131guration', 'Recent revIew \u0130nfo', 'commits (\u0661\u0662)',
             'commits (\uff12)', 'commits ()', 'commits(3)', 'commits  \u3000(3) ', '\u212aommits']
    signs = ['', '\u2699\ufe0f ', '\u2139\ufe0f ', '\U0001f4e5 ', '<b>', '1. ', '\u0130 ', 'a ', '\u212a ', '\u017f']
    out = []
    for name in names:
        for sign in signs[:4] if name != names[0] else signs:
            out.append(f'<details><summary>{sign}{name}</summary>\nRun ID: 9\n</details>')
    out += ['<DETAILS>\n<SUMMARY>Commits</SUMMARY>x</DETAILS>', '<details>\x1c<summary>Commits\x1c</summary></details>',
            '<details><summary>Commits</summary>a</details>b</details>', '<details><summary>Commits</summary>never closed',
            '<deta\u0131ls><summary>Commits</summary></deta\u0131ls>', '<details> <summary> Commits </summary>\n</details>']
    return out


def table_inputs():
    return [
        '| a | b |\n|---|---|\n| c | d |\n', '|:--|--:|  \n', '| - |\x1c| x', '|\u3000-\u3000|\n|-|', '|-|\r\n|-|\n',
        '| Title check | \u2705 Passed | It reads well. |', '| Team sanity \\| \u2705 ok | \u26a0\ufe0f Warning | Do not merge. |',
        '| a | b\\|c | d', '|a|b|', '||', '|||', '| a |', '|a\\', '| x | \u2705 |\n| y | \u274c |\n', '| a |\u2028b|\n',
        'text\n| z |\n| a |\n\n| q |', '| a | \u2705 | x \\| y |\n', '|\\||\u2705|\n', '| :---: | :--- |\n| a | b |',
        '| a | b |\x85| c |\n',
    ]


def attestation_inputs():
    return [
        '/self-review', '/selfreview', '/self review', '/self-reviewed', '/SELF-REVIEW', '/\u017felf-review',
        '/self-rev\u0130ew', '/self-rev\u0131ew', '/self-review ', '/self-review\n', '/self-review ' + HEX40,
        '/self-review\u3000' + HEX40, '/self-review\x1c' + HEX40, '/self-review\u180e' + HEX40,
        '/self-review ' + HEX40.upper(), '/self-review ' + HEX40[:39], '/self-review ' + HEX40 + 'a',
        '/self-review\xa0' + HEX40, '/self-review\x85' + HEX40, '/self-review \uff10' + HEX40[1:],
        '/self\u2013review', '/self--review', '/self_review', '/selfreviewed', '/self reviewed ' + HEX40,
        '/sElF-ReViEwEd', '/\u212aelf-review', '/self-review \u212a' + HEX40[1:], '/self-review' + HEX40,
        '/self  review', '/SELFREVIEWED', '/self-review\x1f\x1e' + HEX40, '/self-review \u0661' + HEX40[1:],
        '/self-reviewedish', 'about to /self-review', '/review', 'self-reviewed',
    ]


PASTA_SUGGESTION = ('<!-- thepastaclaw-review v1 finding=1ae5c0d05709 dedupe=823d079e92be8910 -->\n'
                    '**\U0001f7e1 Suggestion: Add coverage for the ambiguous case**\n\nThe test is padded.')
PASTA_BLOCKING = ('<!-- thepastaclaw-review v1\nfinding=86cd dedupe=eaba -->\n'
                  '**\U0001f534 Blocking: Do not require a live runtime**\n\n**Why:** it panics.')
RABBIT_MINOR = ('_\U0001f3af Functional Correctness_ | _\U0001f7e1 Minor_ | _\u26a1 Quick win_\n\n'
                '**Reject an empty identifier.**\n\n_Note: this mirrors the parser above._\n<!-- cr-comment:v1:1 -->')
RABBIT_MAJOR = ('_\U0001f5c4\ufe0f Data Integrity & Integration_ | _\U0001f7e0 Major_ | _\U0001f3d7\ufe0f Heavy lift_\n\n'
                '**Support it.**')


def heading_inputs():
    return [
        PASTA_SUGGESTION, PASTA_BLOCKING, RABBIT_MINOR, RABBIT_MAJOR, RABBIT_MINOR.replace('\n', '\r\n'),
        '**Why:** not a heading', '**\U0001f7e3 Urgent: x**', '** \U0001f7e1 Suggestion: x**', '**\U0001f7e1 Sugg*estion: x**',
        '**\u00e9 note: x**', 'x **\U0001f7e1 Suggestion: x**', '**\U0001f7e1 Suggestion\n: x**',
        '_a_ | _b_', '_a_|_b_ \t\r', ' _a_ | _b_', '> _a_ | _b_', '_a_ | **b**', '_a_', '_a_ | _b_ x',
        '_a_ | _b_\n_c_ | _d_ | _e_\n', '_\U0001f9f9 Nitpick_ | _\U0001f535 Trivial_', '_a\n_ | _b_',
        '_a_ |\u3000_b_', '_a_ | _b_\x85', '<!--\n_a_ | _b_\n-->',
    ]


def coverage_inputs():
    covered = '{"kind":"reviewed","coveredCommitId":"' + HEAD + '"}'
    return [
        f'<!-- final_review_risk_coverage:{covered} -->', f'x <!-- final_review_risk_coverage:{covered} -->',
        f'a\n<!-- final_review_risk_coverage:   {covered} -->', f'<!-- final_review_risk_coverage:\u3000{covered}',
        f'<!-- final_review_risk_coverage:\n\x1c{covered}', '<!-- final_review_risk_coverage', '<!--  final_review_risk_coverage:',
        f'> <!-- final_review_risk_coverage:{covered} -->', '<!-- FINAL_REVIEW_RISK_COVERAGE:{}',
        f'<!-- final_review_risk_coverage:{covered}\n<!-- final_review_risk_coverage:{{}}',
    ]


def tail_inputs():
    return [' -->', ' -->\n', ' -->\r\n', ' --> \t\n', ' -->x', '\u3000-->\n', '\x1c-->', '-->\n\n', ' --> ',
            '--> \r', '-->\r', '\u180e-->', '\x85 -->\n', '-->\n', '-->', '--> x\n', '-->\r\r\n', '\n\n-->\t',
            '--->', '-- >', '-->\x0b', '-->\u2028']


def phase_inputs(heads):
    out = []
    folded = heads[0].replace('k', '\u212a').replace('s', '\u017f').replace('i', '\u0130')
    for head in heads + ['b' * 40, heads[0].upper(), heads[0][:39], folded]:
        base = f'<!-- thepastaclaw-review-phase v1 phase=final sha={head}'
        out += [base + ' -->', base + '-->', base + ' run=7 extra=x -->', base + '\n -->', base + ' a<b -->',
                base + ' a>b -->', base + '\u3000-->', base + ' x', 'x ' + base + ' -->', 'line\n' + base + ' -->',
                base.upper() + ' -->', base.replace('sha', '\u017fha') + ' -->', base.replace('pastaclaw', 'pa\u017ftaclaw') + ' -->',
                base.replace('final', 'f\u0131nal') + ' -->', base + '\r\n -->', base + ' a\nb -->', base + ' \x1c -->']
    return out


def target_cases():
    targets = ['v*-dev', 'develop', '*', 'release/*', 'a.b', 'v4.2-dev', '*-*', 'x+y', 'a$b', '\xe4*', '(a)', 'a|b', '']
    branches = ['v4.2-dev', 'v5.1-dev', 'feat/v5-dev', 'v5/x-dev', 'v5.1-dev-old', 'develop', 'developer',
                'release/1', 'release/1/2', 'a.b', 'axb', 'x+y', 'xxy', 'a$b', '\xe4z', '\xc4z', 'v-dev',
                'v4.2-dev\n', '-', 'a', '(a)', 'a|b', 'b', '']
    return [(t, branches) for t in targets]


def regex_goldens(rng):
    pool = list(dict.fromkeys(EDGE + [RABBIT, PASTA_SUGGESTION, RABBIT_MINOR, 'a  b\x1c\x1d c\u3000\u3000d']))
    pool += [mutate(rng, rng.choice([RABBIT, RABBIT_MINOR, PASTA_SUGGESTION, '| a | \u2705 |\n|---|\n']))
             for _ in range(24)]
    in_source(r"r'(?m)^<!-- ' + re.escape(RECEIPT_MARKER) + r':\s*'")
    coverage = r'(?m)^<!-- ' + re.escape(policy.RECEIPT_MARKER) + r':\s*'
    named = [
        ('handle', inline(r'[A-Za-z0-9](?:[A-Za-z0-9-]{0,37}[A-Za-z0-9])?'), 0, 'fullmatch', {},
         ['a', 'A-1', '-a', 'a-', 'a' * 39, 'a' * 40, 'ab_c', 'a\u0130', 'a\n', '\u212a', 'k', 'ab--c', '', 'a b',
          '\u0661', 'infraclaw-dash', 'a' * 38 + '-b', '\u212aelvin']),
        ('repository', inline(r'[\w.-]+/[\w.-]+'), 0, 'fullmatch', {},
         ['owner/repo', 'dash-pay/platform.js', '\xfc/\xdf', 'a\xb2/b', 'a_b/c', 'a/b/c', 'a b/c', '\xbd/x',
          '\u0663/\u0663', 'a\u200d/b', '/x', 'x/', 'a\u203f/b', 'a\u0345/b', 'x/y\n']),
        ('branch_syntax', inline(r'[?\[\]{}\\]|\*\*'), 0, 'search', {},
         ['v4.2-dev', 'v*-dev', 'v?-dev', 'v[45]-dev', 'release/**', 'v{4,5}-dev', 'v\\-dev', 'a*b*c', '*', '**',
          'x\n', ']', '}']),
        ('area_id', inline(r'[a-z0-9][a-z0-9-]*'), 0, 'fullmatch', {},
         ['drive', 'a-b', '-a', 'A', 'a_b', 'fallback', 'a\n', '\xe4', '0', '', 'swift-sdk', 'a\u0131']),
        ('prefix', inline(r'(?:[A-Za-z0-9_.-]+/)+'), 0, 'fullmatch', {},
         ['packages/drive/', 'a/', 'a', '/a/', 'a//b/', 'a/b/', '.github/', '../', 'a b/', '\xe4/', 'a/\n', '']),
        ('head_sha', inline(r'[0-9a-f]{40}'), 0, 'fullmatch', {},
         [HEAD, 'A' * 40, 'a' * 39, 'a' * 41, HEAD + '\n', 'g' * 40, '\u0663' * 40, HEX40]),
        ('risk_block', policy.RISK_BLOCK.pattern, policy.RISK_BLOCK.flags, 'search', {},
         [RABBIT, '<!-- final_review_risk_start -->\nA\n<!-- final_review_risk_end -->',
          '<!-- final_review_risk_start -->no end', '<!-- final_review_risk_end --><!-- final_review_risk_start -->',
          '<!-- final_review_risk_start --><!-- final_review_risk_start -->x<!-- final_review_risk_end -->']),
        ('bookkeeping', policy.BOOKKEEPING.pattern, policy.BOOKKEEPING.flags, 'finditer', {}, bookkeeping_inputs()),
        ('checkbox_item', policy.CHECKBOX_ITEM.pattern, policy.CHECKBOX_ITEM.flags, 'finditer', {},
         ['- [ ] <!-- {"checkboxId":"585bb3f6"} --> Fix all', '* [x] <!--{"checkboxId"--> a\nb', '-[X]<!--\x1c{"checkboxId"-->',
          'x - [ ] <!-- {"checkboxId":"1"} -->', '- [y] <!-- {"checkboxId":"1"} -->', '-\u3000[ ]\u3000<!--\u3000{"checkboxId"} -->',
          '- [ ] <!-- {"checkboxId":"1"} --> a\r\nb', '- [ ] <!-- {"checkboxId" > -->']),
        ('checkbox', policy.CHECKBOX.pattern, policy.CHECKBOX.flags, 'finditer', {},
         ['<!-- {"checkboxId": "x"} -->', '<!--\u3000{"checkboxId"-->', '<!--\x1c{"checkboxId"}-->',
          '<!--\u180e{"checkboxId"-->', 'x <!--\n\t{"checkboxId" > -->', '<!--{"checkboxId"--']),
        ('passed', policy.PASSED.pattern, policy.PASSED.flags, 'search', {}, ['\u2705', 'x \u2705 y', '\u2713', '']),
        ('separator', policy.SEPARATOR.pattern, policy.SEPARATOR.flags, 'finditer', {}, table_inputs()),
        ('table_row', policy.TABLE_ROW.pattern, policy.TABLE_ROW.flags, 'finditer', {}, table_inputs()),
        ('table', policy.TABLE.pattern, policy.TABLE.flags, 'finditer', {}, table_inputs()),
        ('spaces', policy.SPACES.pattern, policy.SPACES.flags, 'finditer', {}, []),
        ('attestation', policy.ATTESTATION.pattern, policy.ATTESTATION.flags, 'fullmatch', {}, attestation_inputs()),
        ('hidden_markup', policy.HIDDEN_MARKUP.pattern, policy.HIDDEN_MARKUP.flags, 'finditer', {},
         ['<!-- a -->x', '<!--\na\n-->', '<!-- unterminated', '<!----><!-- b -->', '<!-- a --> --> b']),
        ('pasta_heading', policy.PASTA_HEADING.pattern, policy.PASTA_HEADING.flags, 'finditer', {}, heading_inputs()),
        ('rabbit_heading', policy.RABBIT_HEADING.pattern, policy.RABBIT_HEADING.flags, 'finditer', {}, heading_inputs()),
        ('receipt_coverage', coverage, 0, 'finditer', {}, coverage_inputs()),
        ('receipt_tail', inline(r'\s*-->[ \t]*(?:\r?\n|$)'), 0, 'match', {}, tail_inputs()),
    ]
    cases = []
    for name, source, flags, method, params, inputs in named:
        compiled = re.compile(source, flags)
        pool_here = inputs + (pool if method in ('finditer', 'search') else [])
        cases.append(case(name, compiled, method, params, pool_here))
    in_source(r"r'(?m)^' + re.escape(start) + r'[ \t]*$'")
    in_source(r"r'(?ms)^' + re.escape(start) + r'[ \t]*$.*?^' + re.escape(end) + r'[ \t]*$'")
    for index, (start, end) in enumerate(SECTIONS):
        inputs = section_inputs(start, end) + [RABBIT]
        alone = re.compile(r'(?m)^' + re.escape(start) + r'[ \t]*$')
        whole = re.compile(r'(?ms)^' + re.escape(start) + r'[ \t]*$.*?^' + re.escape(end) + r'[ \t]*$')
        cases.append(case('section_alone', alone, 'count', {'section': index}, inputs))
        cases.append(case('section_whole', whole, 'finditer', {'section': index}, inputs))
    in_source("re.fullmatch('[^/]*'.join(map(re.escape, target.split('*'))), branch)")
    for target, branches in target_cases():
        compiled = re.compile('[^/]*'.join(map(re.escape, target.split('*'))))
        cases.append(case('branch_target', compiled, 'fullmatch', {'target': target}, branches))
    in_source(r"r'(?mi)^<!-- thepastaclaw-review-phase v1 phase=final sha=(' + '|'.join(re.escape(h) for h in sorted(heads))")
    in_source(r"+ r')(?:\s+[^<>]*?)?\s*-->'")
    for heads in [[HEAD], sorted([HEAD, 'f' * 40]), [HEX40], ['k' * 40], sorted(['kk', 'ss', 'ii'])]:
        compiled = re.compile(r'(?mi)^<!-- thepastaclaw-review-phase v1 phase=final sha=('
                              + '|'.join(re.escape(h) for h in sorted(heads)) + r')(?:\s+[^<>]*?)?\s*-->')
        cases.append(case('final_phase', compiled, 'search', {'heads': heads}, phase_inputs(heads)))
    write_json('policy_regex.json', cases)


def record(m):
    return None if m is None else [list(m.span()), list(m.groups())]


def case(name, compiled, method, params, inputs):
    results = []
    for s in dict.fromkeys(inputs):
        if method == 'finditer':
            answer = [record(m) for m in compiled.finditer(s)]
        elif method == 'count':
            answer = len(compiled.findall(s))
        else:
            answer = record(getattr(compiled, method)(s))
        results.append([s, answer])
    return {'name': name, 'method': method, 'params': params, 'results': results}


# --- the object model -----------------------------------------------------------

def float_bits(value):
    return struct.pack('>d', value).hex()


def tag(value):
    """A value the Rust side rebuilds exactly: ints and floats tagged, dicts as pairs."""
    if value is None or isinstance(value, bool):
        return value
    if isinstance(value, int):
        return {'int': str(value)}
    if isinstance(value, float):
        return {'float': float_bits(value)}
    if isinstance(value, str):
        return value
    if isinstance(value, list):
        return [tag(x) for x in value]
    return {'dict': [[k, tag(v)] for k, v in value.items()]}


def object_goldens(rng):
    floats = [0.0, -0.0, 1.0, -1.5, 0.1, 1e16, 1e15, 9999999999999998.0, 1e-4, 1e-5, 0.00012, 123456789012345678.0,
              1.5e300, 5e-324, 2.2250738585072014e-308, 1.7976931348623157e308, float('inf'), float('-inf'),
              float('nan'), 1 / 3, 2 / 3, 100.0, 1e22, 1e21, 0.5, 123.456, 1e100, 12345678901234567.0]
    floats += [struct.unpack('>d', rng.getrandbits(64).to_bytes(8, 'big'))[0] for _ in range(300)]
    floats += [rng.uniform(-1e6, 1e6) for _ in range(100)] + [float(rng.randint(-10**18, 10**18)) for _ in range(50)]
    reprs = [[float_bits(f), repr(f)] for f in floats]

    values = [None, True, False, 0, -7, 2**70, 1.5, -0.0, float('nan'), '', 'it\'s', 'say "hi"', 'both \' and "',
              'tab\tnew\nline\\', '\x00\x1f\x7f\x80\xa0\xad', '\u2028\u3000\ufeff', '\U0001f600 \U000e0001', [],
              [1, 'a', None, [True]], {}, {'k': 1.5, 'n': [None], 'x\'y': {'': False}}, 'caf\xe9', '\ud7ff']
    strs = [[tag(v), str(v)] for v in values]

    docs = ['null', 'true', 'false', '0', '1', '1.0', '1.5', '-0.0', '0.0', '"1"', '"a"', '[]', '{}', '[1]', '[1.0]',
            '[true]', '[NaN]', 'NaN', '[[NaN]]', '{"a":NaN}', '{"a":1}', '{"a":1.0}', '{"b":1,"a":2}', '{"a":2,"b":1}',
            '9007199254740993', '9007199254740992.0', '1e400', 'Infinity', '[1,[2,[3]]]', '[1,[2,[3.0]]]',
            '100000000000000000000', '1e20', '"\\u00e9"', '"\\u0065\\u0301"']
    eq = []
    for a in docs:
        for b in docs:
            eq.append([a, b, json.loads(a) == json.loads(b)])

    ordered = ['0', '1', '-1', 'true', 'false', '1.5', '-2.5', '2', '-3', '9007199254740993', '9007199254740992.0',
               '"a"', '"b"', '"A"', '"\\u00e9"', '"\\ud83d\\ude00"', '"\\uffff"', 'null', '[]', '[1]', '[1, 2]', '[2]',
               '["a"]', '[null]', '{}', '[1, "a"]', '[1, 1]', 'Infinity', '-Infinity', '1e400']
    order = []
    for a in ordered:
        for b in ordered:
            x, y = json.loads(a), json.loads(b)
            try:
                answer = {'lt': x < y, 'gt': y < x}
            except TypeError as error:
                answer = {'type_error': str(error)}
            order.append([a, b, answer])

    upper = [[c, chr(c).upper()] for c in range(0x110000)
             if not 0xD800 <= c <= 0xDFFF and not chr(c).isascii() and chr(c).upper().isascii()]
    write_json('object.json', [], float_repr=reprs, str=strs, eq=eq, order=order, upper_into_ascii=upper)


def main():
    rng = random.Random(20261003)
    regex_goldens(rng)
    object_goldens(rng)


if __name__ == '__main__':
    main()
