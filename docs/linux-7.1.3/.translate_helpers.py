import sys, os, re

def main():
    for tmp in sys.argv[1:]:
        target = tmp[:-4] if tmp.endswith('.tmp') else tmp
        data = open(tmp, encoding='utf-8').read()
        # fence count even?
        fences = data.count('```')
        # Chinese ratio of prose
        # strip fenced code blocks and inline code
        lines = data.split('\n')
        in_fence = False
        prose = []
        for ln in lines:
            if ln.lstrip().startswith('```'):
                in_fence = not in_fence
                continue
            if in_fence:
                continue
            prose.append(ln)
        prose_text = '\n'.join(prose)
        # remove inline code spans
        prose_text = re.sub(r'`[^`]*`', '', prose_text)
        # remove link targets/urls
        prose_text = re.sub(r'\[[^\]]*\]\([^)]*\)', lambda m: m.group(0), prose_text)
        # count cjk chars and letters in remaining prose (exclude link text? keep simple)
        cjk = len(re.findall(r'[一-鿿]', prose_text))
        letters = len(re.findall(r'[A-Za-z]', prose_text))
        ratio = cjk / (cjk + letters) if (cjk + letters) > 0 else 0
        ok = True
        note = []
        if fences % 2 != 0:
            ok = False
            note.append('ODD_FENCES=%d' % fences)
        if ratio < 0.03:
            ok = False
            note.append('LOW_ZH=%.3f' % ratio)
        if not ok:
            print('CHECK_FAIL', target, ','.join(note))
            # do not replace, leave tmp for inspection
            continue
        os.replace(tmp, target)
        print('REPLACED', target, 'zh_ratio=%.3f' % ratio)

if __name__ == '__main__':
    main()
