import os, re, sys

def fence_even(text):
    depth = 0
    for ln in text.split('\n'):
        s = ln.lstrip()
        if s.startswith('```') or s.startswith('~~~'):
            depth ^= 1
    return depth == 0

def strip_prose(text):
    """Return prose with code blocks, inline code, headings, link lines removed."""
    lines = text.split('\n')
    out = []
    in_code = False
    in_html = False
    for ln in lines:
        s = ln.lstrip()
        if s.startswith('```') or s.startswith('~~~'):
            in_code = not in_code
            continue
        if in_code:
            continue
        if s.startswith('<') and ('>' in s):
            # crude html block skip
            in_html = True
            continue
        if in_html:
            if '>' in s:
                in_html = False
            continue
        if not ln.strip():
            continue
        if re.match(r'^#{1,6}\s', ln):  # heading line -> keep text? we remove for ratio of body
            # keep heading text for translation but exclude from skip-ratio counting
            continue
        if re.match(r'^\s*\[[^\]]+\]:', ln):  # link reference def
            continue
        if re.match(r'^\s*!?\[.*\]\(.*\)\s*$', ln):  # pure link line
            continue
        out.append(ln)
    return '\n'.join(out)

# remove inline code spans for ratio counting
CODE_RE = re.compile(r'`[^`]*`')
def count_prose_stats(text):
    prose = strip_prose(text)
    # remove inline code
    prose = CODE_RE.sub(' ', prose)
    # remove url-ish tokens
    prose = re.sub(r'https?://\S+', ' ', prose)
    prose = re.sub(r'<[^>]+>', ' ', prose)
    chinese = len(re.findall(r'[一-鿿]', prose))
    # english words: runs of latin letters length>=2
    eng = re.findall(r'[A-Za-z][A-Za-z\'-]{1,}', prose)
    eng_words = len(eng)
    total = chinese + eng_words
    ratio = (chinese / total) if total > 0 else 0.0
    return chinese, eng_words, ratio

def analyze(path):
    try:
        text = open(path, encoding='utf-8', errors='ignore').read()
    except Exception as e:
        return {'error': str(e)}
    fe = fence_even(text)
    ch, en, ratio = count_prose_stats(text)
    decision = 'translate'
    reason = ''
    if ch >= 1 and ratio >= 0.10 and en < 20:
        decision = 'skip'; reason = 'already translated (zh>=10%% and en<20)'
    elif en < 8:
        decision = 'skip'; reason = 'near-empty/structural (en<8)'
    elif ch == 0 and en == 0:
        decision = 'skip'; reason = 'empty prose'
    return {'fence_even': fe, 'chinese': ch, 'english': en, 'ratio': round(ratio,3),
            'decision': decision, 'reason': reason}

def atomic_replace(tmp, target):
    os.replace(tmp, target)

if __name__ == '__main__':
    cmd = sys.argv[1]
    if cmd == 'analyze':
        for p in sys.argv[2:]:
            print(p)
            print('  ', analyze(p))
    elif cmd == 'replace':
        atomic_replace(sys.argv[2], sys.argv[3])
