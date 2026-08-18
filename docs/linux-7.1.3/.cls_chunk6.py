import json, re, os

with open('.translate_dispatch_final.json','r',encoding='utf-8') as f:
    d = json.load(f)
paths = d['chunks'][6]

def strip_code(text):
    lines = text.split('\n')
    out = []
    in_block = False
    for ln in lines:
        if ln.lstrip().startswith('```'):
            in_block = not in_block
            continue
        if in_block:
            continue
        # remove inline code
        ln = re.sub(r'`[^`]*`', '', ln)
        out.append(ln)
    return '\n'.join(out)

def is_heading(ln): return re.match(r'^\s*#+\s', ln) is not None
def is_link_line(ln): return re.match(r'^\s*[-*]?\s*\[[^\]]*\]\([^)]*\)\s*$', ln) is not None
def is_rst_role(ln): return re.match(r'^\s*:\w+:', ln) is not None
def is_directive(ln): return re.match(r'^\s*\.\.\s', ln) is not None

def classify(path):
    try:
        text = open(path, encoding='utf-8', errors='ignore').read()
    except Exception as e:
        return {'path':path,'error':str(e)}
    prose = strip_code(text)
    cjk = len(re.findall(r'[\u4e00-\u9fff]', prose))
    total = len(prose)
    pr = cjk/total if total else 0.0
    # english words after removing headings/links/rst/directive
    lines = prose.split('\n')
    eng_lines = []
    for ln in lines:
        if is_heading(ln): continue
        if is_link_line(ln): continue
        if is_rst_role(ln): continue
        if is_directive(ln): continue
        eng_lines.append(ln)
    eng_text = '\n'.join(eng_lines)
    eng_words = len(re.findall(r'[A-Za-z]{2,}', eng_text))
    if pr > 0.03:
        mode = 'done'
    elif eng_words >= 8:
        mode = 'full'
    else:
        mode = 'stub'
    return {'path':path,'pr':round(pr,4),'eng_words':eng_words,'mode':mode,'size':os.path.getsize(path)}

res = [classify(p) for p in paths]
for r in res:
    print(f"{r.get('mode'):5} pr={r.get('pr')} ew={r.get('eng_words')} size={r.get('size')} :: {os.path.basename(r.get('path'))}")
json.dump(res, open('.cls_chunk6.json','w',encoding='utf-8'), ensure_ascii=False, indent=1)
