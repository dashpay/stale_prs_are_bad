"""What Python 3.12 does with text, JSON, patterns and timestamps, written
down for the engine's Rust port (engine/, `pr-hygiene-engine`).

The Python engine runs CPython 3.12, whose `json`, `datetime` and `re` are C
modules with their own rules and whose Unicode data is version 15.0. This
script asks that Python and writes the answers:

- engine/src/pycompat/tables.rs: character tables found by enumerating every
  code point: `str.isspace` (which is also `re`'s `\\s`), `splitlines`' line
  breaks, `str.isprintable`, `str.lower` and the two case properties its
  final-sigma rule reads, and `re`'s `\\w`, `\\d` and case-insensitive sets as
  Rust `regex` classes;
- conformance/pycompat/*.json: curated inputs and seeded random ones, each
  with what Python made of it, which engine/tests/pycompat replays.

Run it with the Python the engine runs on, and again only when that changes:

    uv run -q --python 3.12 --no-project python conformance/pycompat/generate.py

The output is deterministic: a second run writes the same bytes.
"""

import _pydatetime
import datetime
import json
import json.decoder
import json.scanner
import random
import re
import struct
import sys
import unicodedata
from pathlib import Path

if sys.version_info[:2] != (3, 12):
    raise SystemExit(f'generate.py must run on Python 3.12, not {sys.version.split()[0]}')
# The engine runs the C implementations; their answers are the contract.
if (datetime.datetime is _pydatetime.datetime or json.scanner.c_make_scanner is None
        or json.decoder.c_scanstring is None):
    raise SystemExit('generate.py needs the C datetime and json modules')

PYTHON = sys.version.split()[0]
UNICODE = unicodedata.unidata_version
ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'conformance' / 'pycompat'
TABLES = ROOT / 'engine' / 'src' / 'pycompat' / 'tables.rs'
REGENERATE = 'uv run -q --python 3.12 --no-project python conformance/pycompat/generate.py'

# Every Unicode scalar value: what a Rust `char` can hold. Python strings can
# also hold lone surrogates; Rust strings never do, so they are left out.
SCALARS = [c for c in range(0x110000) if not 0xD800 <= c <= 0xDFFF]
ALL = ''.join(map(chr, SCALARS))


# --- helpers ----------------------------------------------------------------

def ranges(code_points):
    """Sorted code points as inclusive (first, last) runs."""
    out = []
    for c in code_points:
        if out and out[-1][1] == c - 1:
            out[-1][1] = c
        else:
            out.append([c, c])
    return [tuple(r) for r in out]


def fnv(values):
    """FNV-1a (64-bit) over 32-bit little-endian values; the Rust tests
    compute the same over their own answers."""
    h = 0xcbf29ce484222325
    for v in values:
        for b in v.to_bytes(4, 'little'):
            h ^= b
            h = (h * 0x100000001b3) & 0xFFFFFFFFFFFFFFFF
    return f'{h:016x}'


def set_summary(code_points):
    return {'count': len(code_points), 'fnv': fnv(code_points)}


def class_char(c):
    return chr(c) if chr(c).isascii() and chr(c).isalnum() else f'\\x{{{c:x}}}'


def class_body(code_points):
    return ''.join(class_char(a) if a == b else f'{class_char(a)}-{class_char(b)}'
                   for a, b in ranges(code_points))


def class_string(code_points):
    """A complete Rust `regex` class matching exactly these scalar values,
    negated when that is shorter."""
    inside = set(code_points)
    outside = [c for c in SCALARS if c not in inside]
    positive, negative = class_body(code_points), class_body(outside)
    return f'[{positive}]' if len(positive) <= len(negative) else f'[^{negative}]'


def has_surrogate(value):
    if isinstance(value, str):
        return any(0xD800 <= ord(ch) <= 0xDFFF for ch in value)
    if isinstance(value, list):
        return any(map(has_surrogate, value))
    if isinstance(value, dict):
        return any(has_surrogate(k) or has_surrogate(v) for k, v in value.items())
    return False


def float_bits(value):
    return struct.pack('>d', value).hex()


def tag(value):
    """A JSON value standing for a Python value, readable without the
    reader under test: ints and floats are tagged so that nothing is lost."""
    if value is None or isinstance(value, bool):
        return value
    if isinstance(value, int):
        return {'int': str(value)}
    if isinstance(value, float):
        return {'float': float_bits(value), 'repr': repr(value)}
    if isinstance(value, str):
        return value
    if isinstance(value, list):
        return [tag(x) for x in value]
    if isinstance(value, dict):
        return {'dict': [[k, tag(v)] for k, v in value.items()]}
    raise TypeError(type(value))


def write_json(name, body, cases):
    """One case per line, so a regenerated file diffs line by line."""
    head = json.dumps({'python': PYTHON, 'unicode': UNICODE, **body}, ensure_ascii=True)[:-1]
    lines = [json.dumps(c, ensure_ascii=True, separators=(',', ':')) for c in cases]
    text = head + ',"cases":[\n' + ',\n'.join(lines) + '\n]}\n'
    json.loads(text)
    (OUT / name).write_text(text, encoding='ascii')


# Characters where Python and a newer or differently defined Unicode table
# disagree, or that the engine's own rules single out.
EDGE = [
    '\t', '\n', '\x0b', '\x0c', '\r', '\x1c', '\x1d', '\x1e', '\x1f', ' ', '\x85', '\xa0',
    ' ', '᠎', ' ', ' ', ' ', '​', '‌', '‍', ' ',
    ' ', ' ', ' ', '⁠', '　', '﻿', '\x00', '\x01', '\x7f', '\x80',
    'İ', 'ı', 'ſ', 'K', 'Å', 'Ω', 'ẞ', '\xdf', 'ǅ',
    'ΐ', 'Σ', 'σ', 'ς', 'ͅ', '́', '̇', "'", '.', ':', '­',
    '\xb2', '\xbd', '٣', '①', 'Ⅻ', '２', '\U0001d7ce', '๑',
    'Ᲊ', 'Ɤ', 'Ƛ', '\U00010d50', '\U00010400', '\U0001e900', '\U0001f600',
    '‿', '⁀', '\U0010ffff', '￿', '�',
]
ASCII_SAMPLE = list('aAzZiIsSkKeE019 _-./<>"\\|*:,')


# --- character tables -------------------------------------------------------

def tables():
    space = [c for c in SCALARS if chr(c).isspace()]
    re_space = sorted(ord(ch) for ch in re.findall(r'\s', ALL))
    if space != re_space:
        raise SystemExit('str.isspace and re \\s differ; the tables assume they do not')
    line_break = [c for c in SCALARS if len(('a' + chr(c) + 'b').splitlines()) == 2]
    printable = [c for c in SCALARS if chr(c).isprintable()]
    lower = [(c, chr(c).lower()) for c in SCALARS if chr(c).lower() != chr(c)]
    # The final-sigma rule of str.lower reads two properties Python does not
    # expose. "X + Σ" ends in ς when X is cased and not case-ignorable;
    # "A + X + Σ" ends in ς as well when X is case-ignorable (skipped back
    # to the cased A).
    final_after = [c for c in SCALARS if (chr(c) + 'Σ').lower()[-1] == 'ς']
    through = [c for c in SCALARS if ('A' + chr(c) + 'Σ').lower()[-1] == 'ς']
    final_set = set(final_after)
    case_ignorable = [c for c in through if c not in final_set]
    cased = final_after
    word = sorted(ord(ch) for ch in re.findall(r'\w', ALL))
    digit = sorted(ord(ch) for ch in re.findall(r'\d', ALL))
    folds = {}
    for letter in 'abcdefghijklmnopqrstuvwxyz':
        low = sorted(ord(ch) for ch in re.findall('(?i)' + letter, ALL))
        up = sorted(ord(ch) for ch in re.findall('(?i)' + letter.upper(), ALL))
        if low != up:
            raise SystemExit(f'(?i){letter} and (?i){letter.upper()} match different sets')
        folds[letter] = low
    ignorecase = {
        'RE_IGNORECASE_HEX': ('[0-9a-fA-F]', sorted(ord(ch) for ch in re.findall('(?i)[0-9a-fA-F]', ALL))),
        'RE_IGNORECASE_NOT_LETTER_OR_LT': (
            '[^A-Za-z<]', sorted(ord(ch) for ch in re.findall('(?i)[^A-Za-z<]', ALL))),
    }
    return {
        'space': space, 'line_break': line_break, 'printable': printable, 'lower': lower,
        'case_ignorable': case_ignorable, 'cased': cased, 'word': word, 'digit': digit,
        'folds': folds, 'ignorecase': ignorecase,
    }


def rust_ranges(name, doc, code_points):
    rows = [f'(0x{a:04x}, 0x{b:04x})' for a, b in ranges(code_points)]
    lines = ['    ' + ', '.join(rows[i:i + 6]) + ',' for i in range(0, len(rows), 6)]
    return f'/// {doc}\npub static {name}: &[(u32, u32)] = &[\n' + '\n'.join(lines) + '\n];\n'


def rust_string(value):
    out = []
    for ch in value:
        if ch == '\\':
            out.append('\\\\')
        elif ch == '"':
            out.append('\\"')
        elif ' ' <= ch <= '~':
            out.append(ch)
        else:
            out.append(f'\\u{{{ord(ch):x}}}')
    return '"' + ''.join(out) + '"'


def write_tables(t):
    single = [(c, ord(m)) for c, m in t['lower'] if len(m) == 1]
    expanding = [(c, m) for c, m in t['lower'] if len(m) != 1]
    rows = [f'(0x{a:04x}, 0x{b:04x})' for a, b in single]
    lower_lines = ['    ' + ', '.join(rows[i:i + 6]) + ',' for i in range(0, len(rows), 6)]
    folds = ',\n'.join(f'    r"[{class_body(t["folds"][letter])}]"' for letter in sorted(t['folds']))
    parts = [
        f'// Generated by conformance/pycompat/generate.py from Python {PYTHON}\n'
        f'// (Unicode {UNICODE}). Do not edit by hand; to regenerate, run\n'
        f'//     {REGENERATE}\n'
        '//\n'
        '// Every table was found by asking Python about every Unicode scalar value.\n\n',
        f'/// The Python these tables were read from.\npub const PYTHON_VERSION: &str = "{PYTHON}";\n',
        f'/// The Unicode data of that Python.\npub const UNICODE_VERSION: &str = "{UNICODE}";\n\n',
        rust_ranges('SPACE', '`str.isspace()`, which is also what `re` matches with `\\s`.', t['space']),
        '\n' + rust_ranges('LINE_BREAK', 'Where `str.splitlines()` breaks a line (`\\r\\n` counts as one break).',
                           t['line_break']),
        '\n' + rust_ranges('PRINTABLE', '`str.isprintable()`: what `repr` shows as it is.', t['printable']),
        '\n' + rust_ranges('CASE_IGNORABLE', 'Case-ignorable, as the final-sigma rule of `str.lower` reads it.',
                           t['case_ignorable']),
        '\n' + rust_ranges('CASED', 'Cased and not case-ignorable, as the final-sigma rule reads it.',
                           t['cased']),
        '\n/// `str.lower()` of each character that lowers to one other character\n'
        '/// (U+03A3 lowers to σ here; in a string, the final-sigma rule decides).\n'
        'pub static LOWER: &[(u32, u32)] = &[\n' + '\n'.join(lower_lines) + '\n];\n',
        '\n/// `str.lower()` of each character that lowers to more than one.\n'
        'pub static LOWER_EXPANDING: &[(u32, &str)] = &[\n'
        + ''.join(f'    (0x{c:04x}, {rust_string(m)}),\n' for c, m in expanding) + '];\n',
        '\n/// `re`\'s `\\s` as the inside of a Rust `regex` class.\n'
        f'pub const RE_SPACE: &str = r"{class_body(t["space"])}";\n',
        '\n/// `re`\'s `\\w` as the inside of a Rust `regex` class.\n'
        f'pub const RE_WORD: &str = r"{class_body(t["word"])}";\n',
        '\n/// `re`\'s `\\d` as the inside of a Rust `regex` class.\n'
        f'pub const RE_DIGIT: &str = r"{class_body(t["digit"])}";\n',
        '\n/// What each ASCII letter matches under `re.IGNORECASE`, `a` to `z`, each a\n'
        '/// complete class; a letter and its capital match the same set.\n'
        'pub static RE_FOLD: [&str; 26] = [\n' + folds + ',\n];\n',
    ]
    for name, (source, members) in t['ignorecase'].items():
        parts.append(f'\n/// `re`\'s `{source}` under `re.IGNORECASE`, as a complete class.\n'
                     f'pub const {name}: &str = r"{class_string(members)}";\n')
    TABLES.write_text(''.join(parts), encoding='utf-8')


def table_goldens(t):
    lower_pairs = []
    for c, m in t['lower']:
        lower_pairs += [c, *map(ord, m), 0xFFFFFFFF]
    body = {
        'sets': {
            'space': set_summary(t['space']),
            'line_break': set_summary(t['line_break']),
            'printable': set_summary(t['printable']),
            'case_ignorable': set_summary(t['case_ignorable']),
            'cased': set_summary(t['cased']),
            're_word': set_summary(t['word']),
            're_digit': set_summary(t['digit']),
            **{name: set_summary(members) for name, (_, members) in t['ignorecase'].items()},
        },
        'lower': {'count': len(t['lower']), 'fnv': fnv(lower_pairs)},
        'folds': {letter: members for letter, members in sorted(t['folds'].items())},
    }
    write_json('tables.json', body, [])


# --- text -------------------------------------------------------------------

TEXT_CURATED = [
    '', ' ', '\t\n', 'a b', '  a  b  ', '\x1c\x1d\x1e\x1fa\x1f', '\x85a\x85', '᠎a᠎',
    '\xa0a\xa0', '　a　', '​a​', '﻿a', 'a\r\nb', 'a\n\rb', 'a\rb\nc\r\n',
    'a\x0bb\x0cc', 'a\x1cb\x1dc\x1ed\x1fe', 'a\x85b c d', '\n', '\n\n', 'a\n', '\r', '\r\r\n',
    'x\r\n\n\ry', ' /skip-bots ', ' /self-review\x1c',
    'ΑΣ', 'ΑΣ ', 'ΑΣΑ', 'Σ', 'ΣΣ', 'ΆΣ',
    'ΑΣ́', "Α'Σ", 'ΑΣ́Α', 'AΣ', '1Σ', 'Α.Σ',
    'İ', 'İstanbul', 'İ', 'ı', 'ſ', 'K', 'Å', 'ẞ', 'ǅ',
    'Ᲊ', 'Ɤ', 'Ƛ', '\U00010d50', '\U00010400', '\U0001e900', 'ABC', 'Ünïcödé', '\xb2\xbd',
    'x́', 'Ⅻ', 'ǅ', 'ΣΑΣ ΣΑΣ', 'thepastaClaw', 'CodeRabbitAI[bot]', '\U0001f600‍\U0001f600',
    "it's", '"quoted"', '\'"both"\'', 'back\\slash', '\x00\x01\x7f\x80\xad͸\U000e0001',
]
SLICES = [(None, 7), (2, 5), (-3, None), (5, 2), (-100, 100)]


def random_strings(rng, count, max_len, pool):
    return [''.join(rng.choice(pool) for _ in range(rng.randint(0, max_len))) for _ in range(count)]


def text_goldens(rng):
    pool = EDGE + ASCII_SAMPLE + [chr(rng.choice(SCALARS)) for _ in range(60)]
    strings = list(dict.fromkeys(TEXT_CURATED + random_strings(rng, 600, 10, pool)))
    cases = []
    for s in strings:
        cases.append({
            's': s, 'len': len(s), 'isspace': s.isspace(), 'strip': s.strip(), 'lstrip': s.lstrip(),
            'rstrip': s.rstrip(), 'split': s.split(), 'splitlines': s.splitlines(),
            'splitlines_keepends': s.splitlines(keepends=True), 'lower': s.lower(), 'repr': repr(s),
            'slices': [[a, b, s[a:b]] for a, b in SLICES],
        })
    write_json('text.json', {}, cases)
    return strings


# --- JSON -------------------------------------------------------------------

def outcome(call):
    try:
        value = call()
    except json.JSONDecodeError as error:
        return {'error': {'type': 'JSONDecodeError', 'message': str(error), 'pos': error.pos}}
    except RecursionError as error:
        return {'error': {'type': 'RecursionError', 'message': str(error)}}
    except ValueError as error:
        return {'error': {'type': 'ValueError', 'message': str(error)}}
    if isinstance(value, tuple):
        value, end = value
        if has_surrogate(value):
            return {'lone_surrogate': True}
        return {'ok': tag(value), 'end': end}
    if has_surrogate(value):
        return {'lone_surrogate': True}
    return {'ok': tag(value)}


LOADS_CURATED = [
    '', ' ', '1', ' 1', '1 ', ' \t\n\r1\r\n\t ', '\x0c1', '\x0b1', '\xa01', '　1', '1　', '1\x1c',
    '﻿1', '﻿', ' ﻿1', '1﻿',
    'null', 'true', 'false', 'nul', 'tru', 'fals', 'NULL', 'True', 'None', 'NaN', 'Infinity', '-Infinity',
    '-NaN', '+Infinity', 'nan', 'inf', 'Infinit', '-Infinit', 'NaNa', 'nullx', 'truefalse', 'n', '-I',
    '[NaN, Infinity, -Infinity]', '{"a": NaN}',
    '0', '-0', '00', '01', '-01', '1.', '.5', '1.5', '-1.5', '1e5', '1E5', '1e+5', '1e-5', '1e', '1e+', '1E-',
    '1e-a', '1.5e3', '0e0', '-', '--1', '+1', '1.e5', '1.0e', '123abc', '1_000', '-0.0', '0.0', '1.5.6',
    '9223372036854775807', '9223372036854775808', '-9223372036854775808', '-9223372036854775809',
    '18446744073709551616', '1' * 50, '-' + '1' * 50, '1' * 4300, '1' * 4301, '-' + '1' * 4300,
    '-' + '1' * 4301, '[' + '1' * 4301 + ']', '1' * 4301 + '.5', '1' * 4301 + 'e1',
    '1e400', '-1e400', '1e-400', '4.9e-324', '2.4703282292062327e-324', '2.2250738585072014e-308',
    '1.7976931348623157e308', '1.7976931348623159e308', '0.1', '0.30000000000000004',
    '1e99999999999999999999', '1e-99999999999999999999', '1' * 400 + '.5', '0.' + '0' * 400 + '1',
    '123456789012345678901234567890e-10', '9007199254740993', '9007199254740993.0', '٣', '１',
    '""', '"a"', '"\\"\\\\\\/\\b\\f\\n\\r\\t"', '"\\u0041"', '"\\u00e9"', '"\\u00E9"', '"é"', '"\\ud83d\\ude00"',
    '"\\uD83D\\uDE00"', '"\\ud83d"', '"\\ude00"', '"\\ude00\\ud83d"', '"\\ud83d\\u0041"', '"\\ud83dx"',
    '"\\ud83d\\"', '"\\ud83d\\u12"', '"\\ud83d\\uzzzz"', '"\\ud83d\\ude0"', '"\\ud83d\\ude0"x',
    '"\\ud83d\\ud83d\\ude00"', '"\\u12"', '"\\u12', '"\\u0041', '"\\u004g"', '"\\x41"', '"\\a"', '"\\',
    '"abc', '"abc\\"', '"\t"', '"\x00"', '"\x1f"', '"\x7f"', '"\x85"', '" "', '"\\u0000"', '"😀"',
    '"\\U0001F600"', '"a\nb"', '"a\rb"', '"\\u00"', '"\\u"', '"\\u0041\\u"', '"é\\q"', '"éé', '"\\ud800\\u00e9"',
    '"\\ud800\\uéééé"', '"\\uéééé"', '"\\ud800\\uéé"', '"\\ud800\\u12é"', '"\\u12é"',
    '[]', '[ ]', '[', ']', '[1', '[1,', '[1,]', '[,1]', '[1 2]', '[1,,2]', '[[[]]]', '[[]', '[]]', '[\n]',
    '{}', '{ }', '{', '}', '{"a"}', '{"a":}', '{"a":1', '{"a":1,}', '{"a" 1}', '{a:1}', "{'a':1}",
    '{"a":1 "b":2}', '{"a":1,"a":2}', '{"a":1,"b":2,"a":3}', '{"b":1,"a":2}', '{"":1}',
    '{"\\u00e9":1, "é": 2}', '{1:1}', '[1]x', '[1] [2]', '1 2', '"a" "b"', '{"a":[1,{"b":null}]}',
    '{"a" : 1 , "b" : [ 1 , 2 ] }', '{"a":1}}', '{"a":', '{"a', '{"a"', '{"a":1,', '{"a":1,"', '{,}',
    '{\n  "a": 1,\n  "b": \n}', '[\n1,\n2,\n]', '\n\n\nx', 'é\nx', '"é" x', '\n"é"\n\nx', '["é", x]',
    '["😀", x]', '{"😀": 1, }', '{"é\nx": 1 x}', '\r\n x',
    '["\\ud800", }', '["\\ud800"] x', '{"\\ud800": 1}', '"\\udfff"', '["\\ud800", 1e]', '[["\\udc00"]]',
    '{"kind": "reviewed", "coveredCommitId": "abc"} -->', '{"a": 1}\n-->', '"é" -->', '[1]]', '{"a":1}{"b":2}',
    '  {"a":1}', '\t[1]', '{"state": "ready-for-human", "receipts": {"z": 1, "a": 2}}',
]


def random_value(rng, depth, pool):
    kind = rng.randrange(9 if depth < 3 else 6)
    if kind == 0:
        return None
    if kind == 1:
        return rng.random() < 0.5
    if kind == 2:
        return rng.choice([0, -1, 7, 2**63, -2**63 - 1, 10**25, rng.randint(-10**6, 10**6)])
    if kind == 3:
        return rng.choice([0.5, -0.0, 1e300, 1e-300, float('nan'), float('inf'), float('-inf'), rng.random()])
    if kind in (4, 5):
        return ''.join(rng.choice(pool) for _ in range(rng.randint(0, 6)))
    if kind in (6, 7):
        return [random_value(rng, depth + 1, pool) for _ in range(rng.randint(0, 3))]
    return {''.join(rng.choice(pool) for _ in range(rng.randint(0, 3))): random_value(rng, depth + 1, pool)
            for _ in range(rng.randint(0, 3))}


MUTATIONS = list('{}[]",:\\ \n0-e.') + ['\\u', '\\ud800', 'é', '\x00', 'NaN', 'true']


def mutate(rng, text, alphabet):
    chars = list(text)
    for _ in range(rng.randint(0, 2)):
        op = rng.randrange(3)
        at = rng.randint(0, len(chars))
        if op == 0 and chars:
            del chars[min(at, len(chars) - 1)]
        elif op == 1:
            chars.insert(at, rng.choice(alphabet))
        elif chars:
            chars[min(at, len(chars) - 1)] = rng.choice(alphabet)
    return ''.join(chars)


def json_goldens(rng):
    pool = EDGE + ASCII_SAMPLE
    fuzz = []
    for _ in range(500):
        value = random_value(rng, 0, pool)
        options = rng.choice([{}, {'separators': (',', ':')}, {'indent': 1}, {'ensure_ascii': False},
                              {'ensure_ascii': False, 'indent': 2}])
        fuzz.append(mutate(rng, json.dumps(value, **options), MUTATIONS))
    docs = list(dict.fromkeys(LOADS_CURATED + fuzz))
    decoder = json.JSONDecoder()
    cases = [{'doc': doc, 'loads': outcome(lambda: json.loads(doc)),
              'raw_decode': outcome(lambda: decoder.raw_decode(doc))} for doc in docs]
    write_json('json_loads.json', {}, cases)

    values = [
        None, True, False, 0, -1, 1, 2**63 - 1, 2**63, -2**63 - 1, 10**40, -10**40, '', 'a',
        '\x00\x01\x1f\x7f\x80\xff', '"\\/', '\b\f\n\r\t', 'é', '  ', '￿', '\U0001f600',
        '\U0010ffff', 'mixed é 😀 \x00 "q" \\', [], {}, [[]], [{}], {'a': []}, {'a': {}}, [1, [2, [3, []]]],
        {'b': 1, 'a': 2, 'é': 3, 'Z': 4, '': 5, '￿': 6, '\U0001f600': 7, 'aa': 8, 'a\x00': 9, 'A': 10},
        {'state': 'ready-for-human', 'number': 42, 'admitted_at': None, 'heads': ['ab' * 20],
         'receipts': {'z': '2026-10-01T00:00:00Z', 'a': '2026-09-30T00:00:00Z'}, 'ok': True},
        [{'path': 'src/é.rs', 'additions': 3}, {'path': 'a', 'additions': 0, 'deletions': 2**70}],
        {'nested': {'deeper': {'deepest': [None, False, {'k': 'v'}]}}, 'list': [[], {}, '']},
        1.5, [0.0], {'a': float('nan')},
    ]
    shapes = {
        'compact_sorted': {'separators': (',', ':'), 'sort_keys': True},
        'compact': {'separators': (',', ':')},
        'default_sorted': {'sort_keys': True},
        'default': {},
        'indent2': {'indent': 2},
    }
    cases = []
    for value in values:
        cases.append({'value': tag(value),
                      'out': {name: json.dumps(value, **options) for name, options in shapes.items()}})
    write_json('json_dumps.json', {}, cases)


def json_limits():
    def deepest(opening, inner, closing):
        lo, hi = 1, 100000
        while lo < hi:
            mid = (lo + hi + 1) // 2
            try:
                json.loads(opening * mid + inner + closing * mid)
                lo = mid
            except RecursionError:
                hi = mid - 1
        return lo

    depth = deepest('[', '', ']')
    if deepest('{"a":', '1', '}') != depth:
        raise SystemExit('arrays and objects nest to different depths')
    messages = {}
    for kind, (opening, closing) in {'array': ('[', ']'), 'object': ('{"a":', '1}')}.items():
        try:
            json.loads(opening * (depth + 1) + closing * (depth + 1))
        except RecursionError as error:
            messages[kind] = str(error)
    return {'max_depth': depth, 'recursion_messages': messages,
            'int_max_str_digits': sys.get_int_max_str_digits()}


# --- regular expressions ----------------------------------------------------

ATTESTATION = re.compile(r'/self[- ]?review(?:ed)?(?:\s+(?P<head>[0-9a-fA-F]{40}))?', re.IGNORECASE)
HEX40 = '0123456789abcdef' * 2 + '01234567'
PATTERNS = [
    ('repository', r'[\w.-]+/[\w.-]+', 'fullmatch', [
        'owner/repo', 'dash-pay/platform.js', 'ü/ß', 'a\xb2/b', 'á/b', 'a_b/c', 'a/b/c', 'a b/c',
        '\xbd/x', '٣/٣', 'a‍/b', 'ǅ/x', '/x', 'x/', 'a‿/b', 'Ⅰ/x', 'x\U0001d7ce/y',
        '\U0001e900/x', 'Ᲊ/x', 'a­/b', 'aͅ/b']),
    ('timestamp', r'\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z', 'fullmatch', [
        '2024-01-01T00:00:00Z', '٢٠٢٤-01-01T00:00:00Z', '２０２４-01-01T00:00:00Z',
        '\U0001d7d0\U0001d7ce\U0001d7d0\U0001d7d2-01-01T00:00:00Z', '2024-01-01T00:00:00Z\n',
        '\xb2⁰\xb2⁴-01-01T00:00:00Z', '2024-01-01T00:00:00+00:00', '2024-01-01T00:00:0๑Z']),
    ('spaces', r'\s+', 'finditer', []),
    ('non_spaces', r'\S+', 'finditer', []),
    ('words', r'\w+', 'finditer', []),
    ('non_words', r'\W+', 'finditer', []),
    ('digits', r'\d+', 'finditer', []),
    ('non_digits', r'\D+', 'finditer', []),
    ('space_or_digit', r'[\s\d]+', 'finditer', []),
    ('neither_space_nor_word', r'[^\s\w]+', 'finditer', []),
    ('non_word_or_digit', r'[\W\d]+', 'finditer', []),
    ('not_non_digit', r'[^\D]+', 'finditer', []),
    ('bracket_first', r'[]\s]+', 'finditer', ['a] b]]\x1c]']),
    ('checkbox', r'<!--\s*\{"checkboxId"[^>]*-->', 'search', [
        '<!-- {"checkboxId": "x"} -->', '<!--　{"checkboxId"-->', '<!--\x1c{"checkboxId"}-->',
        '<!--᠎{"checkboxId"-->', 'x <!--\n\t{"checkboxId" > -->', '<!--{"checkboxId"--']),
    ('receipt_tail', r'\s*-->[ \t]*(?:\r?\n|\Z)', 'match', [
        ' -->', ' -->\n', ' -->\r\n', ' --> \t\n', ' -->x', '　-->\n', '\x1c-->', '-->\n\n', ' --> ',
        '--> \r', '-->\r', '᠎-->', '\x85 -->\n']),
    ('table_separator', r'(?m)^\|(?:\s*:?-+:?\s*\|)+[ \t]*$\n?', 'finditer', [
        '|---|---|\n| a | b |', '|:--|--:|  \n', '| - |\x1c| x', '|　-　|\n|-|', '|-|\r\n|-|\n']),
    ('attestation', ATTESTATION.pattern, 'fullmatch', [
        '/self-review', '/selfreview', '/self review', '/self-reviewed', '/SELF-REVIEW', '/ſelf-review',
        '/self-revİew', '/self-revıew', '/self-review ', '/self-review\n', '/self-review ' + HEX40,
        '/self-review　' + HEX40, '/self-review\x1c' + HEX40, '/self-review᠎' + HEX40,
        '/self-review ' + HEX40.upper(), '/self-review ' + HEX40[:39], '/self-review ' + HEX40 + 'a',
        '/self-review\xa0' + HEX40, '/self-review\x85' + HEX40, '/self-review ０' + HEX40[1:],
        '/self–review', '/self--review', '/self_review', '/selfreviewed', '/self reviewed ' + HEX40,
        '/sElF-ReViEwEd', '/self-revieẈ', '/Kelf-review', '/self-review K' + HEX40[1:],
        '/SELF-REVİEWED \t\n' + HEX40, '/self-review' + HEX40, '/self  review', '/SELFREVIEWED',
        '/self-review ' + HEX40, '/self-review\x1f\x1e' + HEX40, '/self-review ١' + HEX40[1:]]),
]


def match_record(m):
    if m is None:
        return None
    return {'span': list(m.span()), 'groups': m.groupdict()}


def regex_goldens(rng, strings):
    shared = strings[:len(TEXT_CURATED)] + rng.sample(strings[len(TEXT_CURATED):], 120)
    cases = []
    for name, source, method, inputs in PATTERNS:
        flags = re.IGNORECASE if name == 'attestation' else 0
        pattern = re.compile(source, flags)
        pool = list(dict.fromkeys(inputs + (shared if method == 'finditer' else [])))
        results = []
        for s in pool:
            if method == 'finditer':
                results.append([s, [list(m.span()) for m in pattern.finditer(s)]])
            else:
                results.append([s, match_record(getattr(pattern, method)(s))])
        cases.append({'name': name, 'pattern': source, 'ignorecase': bool(flags), 'method': method,
                      'results': results})
    write_json('regex.json', {}, cases)


# --- datetime ---------------------------------------------------------------

DATETIME_CURATED = [
    '2024-01-01', '2024-01-01T00:00:00', '2024-01-01T12:34:56Z', '2024-01-01T12:34:56+00:00',
    '2026-10-01T00:00:00Z', '2026-10-01T00:00:00.123Z', '2024-01-01T12:34:56.1', '2024-01-01T12:34:56.12',
    '2024-01-01T12:34:56.123', '2024-01-01T12:34:56.1234', '2024-01-01T12:34:56.12345',
    '2024-01-01T12:34:56.123456', '2024-01-01T12:34:56.1234567', '2024-01-01T12:34:56.123456789',
    '2024-01-01T12:34:56,5', '2024-01-01T12:34:56.', '2024-01-01T12:34:56.5x', '2024-01-01T12.5',
    '2024-01-01T1230.5', '2024-01-01T12:30.5', '2024-01-01T12:30:45:123', '2024-01-01T12304567',
    '2024-01-01T1230456', '2024-01-01T123', '2024-01-01T12:3', '2024-01-01T12:3045', '2024-01-01T1230:45',
    '2024-01-01T12', '2024-01-01T1234', '2024-01-01T123456', '20240101T123456', '20240101T1234Z',
    '2024-01-01T12:34:56+05:30', '2024-01-01T12:34:56-05:30', '2024-01-01T12:34:56+0530',
    '2024-01-01T12:34:56+05', '2024-01-01T12:34:56+5', '2024-01-01T12:34:56+053015',
    '2024-01-01T12:34:56+05:30:15', '2024-01-01T12:34:56+05:30:15.5', '2024-01-01T12:34:56-05:30:15.123456',
    '2024-01-01T12:34:56+05:30:15.1234567', '2024-01-01T12:34:56+00:00:00.5', '2024-01-01T12:34:56-00:00',
    '2024-01-01T12:34:56+00:00:00.000001', '2024-01-01T12:34:56-00:00:00.000001', '2024-01-01T12:30+05:99',
    '2024-01-01T12:34:56+23:59:59.999999', '2024-01-01T12:34:56-23:59:59.999999', '2024-01-01T12:34:56+24:00',
    '2024-01-01T12:34:56-24:00', '2024-01-01T12:34:56+99:99', '2024-01-01T12:34:56-99:99:99.999999',
    '2024-01-01T12:34:56+', '2024-01-01T12:34:56+05:', '2024-01-01T12:34:56+05:3',
    '2024-01-01T12:34:56Z+05:00', '2024-01-01T12:34:56+05:00Z', '2024-01-01T12:34:56ZZ', '2024-01-01T-05:00',
    '2024-01-01T12-05', '2024-01-01T12:00:00.123+05:00', '2024-01-01T12:00:00,5Z', '2024-01-01TZ',
    '2024-01-01T12Z', '2024-01-01T24:00', '2024-01-01T23:60', '2024-01-01T23:59:60', '2024-01-01T25:00:00',
    '2024-02-29', '2023-02-29', '2024-02-30', '2024-13-01', '2024-00-01', '2024-01-00', '2024-01-32',
    '0000-01-01', '0001-01-01', '9999-12-31T23:59:59.999999', '0001-01-01T00:00:00+01:00',
    '9999-12-31T23:59:59-01:00', '2024-01-01 12:34:56', '2024-01-01t12:34:56', '2024-01-01x12:34:56',
    '2024-01-01012:00', '2024-01-01é12:00', '2024-01-01　12:00', '2024-01-01\U0001f40d12:00',
    '2024-01-01\x0012:00', '2024-01-01T12:30:45\x00', '2024-01-01T12:30\x00', '2024-01-01T12\x00:30',
    '2024-01-01T', '2024-01-01é', '2024-01-01 ', '2024-01-01T12:00 ', ' 2024-01-01', '2024-01-01\n',
    '2024-W01', '2024W01', '2024-W01-1', '2024W011', '2024-W01-7', '2024-W01-0', '2024-W01-8', '2020-W53',
    '2020-W53-7', '2021-W53', '2024-W00', '2024-W54', '2024-W1', '2024-W01-', '2024-W01-1T00:00',
    '2024W011T00:00', '2024W01T00:00', '2024-W01T00', '2020-W01-0000', '2020-W01-1000', '2024W0112',
    '2024W01123', '2024W011234', '0000-W01-1', '9999-W52-5', '9999-W52-6', '2024-W01-1Z', '2024-W01x',
    '2024-123', '2024123', '2024-123T00:00', '2024-01', '202401', '2024', '2024-1-01', '2024-01-1',
    '2024-1 -01', '2024-01-01T1 :00', '２０２４-01-01', '2024-0١-01',
    '2024/01/01', '2024-01/01', '2024-0101', '202401-01', '20240101', '2024-01-01T12:34:56.000000',
    '2024-01-01T00:00:00.000001-00:00:00.000001', '2024-01-01T00:00:00+00:00:01',
]
DATETIME_TEMPLATES = [
    '2024-01-01T12:34:56.789012+05:30', '20240101T123456Z', '2024-W01-1T00:00', '2020-W53-7 23:59:59,5-00:00:30.5',
    '2024-02-29', '2024W017', '0001-01-01T00:00:00+01:00', '2026-10-01T00:00:00Z', '2024-01-01T1234',
]
DATETIME_ALPHABET = list('0123456789-:TW.,+Z ') + ['é', '\x00', 't', '　']


def datetime_result(s):
    try:
        v = datetime.datetime.fromisoformat(s)
    except ValueError as error:
        result = {'error': {'type': 'ValueError', 'message': str(error)}}
        python_side = ('error', str(error))
    else:
        offset = v.utcoffset()
        result = {'ok': {
            'fields': [v.year, v.month, v.day, v.hour, v.minute, v.second, v.microsecond],
            'offset_us': None if offset is None else offset // datetime.timedelta(microseconds=1),
            'isoformat': v.isoformat(), 'isoformat_seconds': v.isoformat(timespec='seconds'),
            'timestamp': None if offset is None else float_bits(v.timestamp()),
        }}
        python_side = ('ok', v.isoformat())
    # The engine runs the C module; the pure-Python one is only the readable
    # reference, and differs on some of these. Kept to show where.
    try:
        pure = ('ok', _pydatetime.datetime.fromisoformat(s).isoformat())
    except Exception as error:  # noqa: BLE001 - any failure is a difference worth showing
        pure = ('error', str(error))
    result['pure_python_differs'] = pure != python_side
    return result


def datetime_goldens(rng):
    fuzz = []
    for _ in range(900):
        fuzz.append(mutate(rng, rng.choice(DATETIME_TEMPLATES), DATETIME_ALPHABET))
    inputs = [s for s in dict.fromkeys(DATETIME_CURATED + fuzz)]
    cases = [{'s': s, **datetime_result(s)} for s in inputs]

    compared = [
        '2024-01-01T00:00:00', '2024-01-01T00:00:00.000001', '2024-01-01T05:30:00', '2024-01-01T00:00:00Z',
        '2024-01-01T05:30:00+05:30', '2023-12-31T18:30:00-05:30', '2024-01-01T00:00:00.000001+00:00',
        '2024-01-01T00:00:00+00:00:00.000001', '2024-01-01T00:00:00-00:00:00.5', '2024-01-01T00:00:00+23:59',
        '0001-01-01T00:00:00+01:00', '0001-01-01T00:00:00', '0001-01-01T00:00:00Z', '9999-12-31T23:59:59.999999',
        '9999-12-31T23:59:59.999999-23:59:59.999999', '1970-01-01T00:00:00Z', '1969-12-31T23:59:59.999999Z',
        '2026-10-01T00:00:00Z', '2026-10-01T02:00:00+02:00', '2026-10-01T00:00:00',
    ]
    pairs = []
    for a in compared:
        for b in compared:
            x, y = datetime.datetime.fromisoformat(a), datetime.datetime.fromisoformat(b)
            try:
                lt = x < y
            except TypeError as error:
                lt = {'type_error': str(error)}
            try:
                delta = x - y
                sub = {'micros': delta // datetime.timedelta(microseconds=1),
                       'total_seconds': float_bits(delta.total_seconds())}
            except TypeError as error:
                sub = {'type_error': str(error)}
            pairs.append({'a': a, 'b': b, 'eq': x == y, 'lt': lt, 'sub': sub})
    write_json('datetime.json', {'pairs': pairs}, cases)


# --- values and ties ----------------------------------------------------------

def value_goldens():
    ints = [0, 1, -1, 42, -42, 2**63 - 1, 2**63, -2**63, -2**63 - 1, 2**64, -2**64, 10**18, 10**19, -10**19,
            10**30, -10**30, 10**30 + 1, 123456789012345678901234567890, -123456789012345678901234567890,
            99999999999999999999, 100000000000000000000, int('9' * 4300), -int('9' * 4300)]
    texts = [str(i) for i in ints]
    order = [[(a > b) - (a < b) for b in ints] for a in ints]
    truthy = [None, True, False, 0, 1, -1, 2**70, 0.0, -0.0, float('nan'), float('inf'), 0.5, '', ' ', '0',
              [], [0], [[]], {}, {'': None}]
    write_json('values.json', {
        'ints': texts, 'compare': order,
        'truthiness': [[tag(v), bool(v)] for v in truthy],
    }, [])


def tie_goldens(rng):
    cases = [[[1, 'a'], [3, 'b'], [3, 'c'], [2, 'd']], [[0, 'x']], [[5, 'b'], [5, 'a']],
             [[2, 'z'], [1, 'y'], [1, 'x'], [2, 'w']]]
    for _ in range(40):
        cases.append([[rng.randint(0, 3), rng.choice('ab')] for _ in range(rng.randint(1, 6))])
    out = []
    for keys in cases:
        # max(key=...) and min(key=...) over the positions, keyed by the
        # first number only: ties are what this pins.
        out.append({'keys': keys,
                    'max': max(range(len(keys)), key=lambda i: keys[i][0]),
                    'min': min(range(len(keys)), key=lambda i: keys[i][0]),
                    'max_tuple': max(range(len(keys)), key=lambda i: tuple(keys[i])),
                    'min_tuple': min(range(len(keys)), key=lambda i: tuple(keys[i]))})
    write_json('ties.json', {}, out)


def main():
    rng = random.Random(20261002)
    t = tables()
    write_tables(t)
    table_goldens(t)
    strings = text_goldens(rng)
    json_goldens(rng)
    regex_goldens(rng, strings)
    datetime_goldens(rng)
    value_goldens()
    tie_goldens(rng)
    meta = {'python': PYTHON, 'unicode': UNICODE, 'implementation': sys.implementation.name,
            'generator': 'conformance/pycompat/generate.py', 'regenerate': REGENERATE, 'json': json_limits()}
    (OUT / 'meta.json').write_text(json.dumps(meta, indent=2) + '\n', encoding='ascii')


if __name__ == '__main__':
    main()
