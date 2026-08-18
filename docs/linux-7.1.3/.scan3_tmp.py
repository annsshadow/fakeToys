import json, re, os

BASE = "D:/WORKSPACE/linux-7.1.3/docs/系统文档"
d = json.load(open(os.path.join(BASE, ".translate_dispatch_cold2.json")))
ch = [p.replace("\\","/") for p in d["chunks"][0]]
zh_re = re.compile(r'[\u3400-\u4dbf\u4e00-\u9fff]')
en_re = re.compile(r'[A-Za-z]+')

# For each file, estimate English "prose sentences" = lines (outside fences) that are
# natural-language (>=6 english words) and are NOT purely a link/identifier/table-row.
def fence_split(text):
    out=[]; inf=False; buf=[]
    for line in text.splitlines():
        if line.strip().startswith("```"):
            if inf: out.append(("\n".join(buf), True)); buf=[]
            inf=not inf; continue
        if inf: buf.append(line); continue
        out.append((line, False))
    return out

for p in ch:
    text=open(p,encoding="utf-8",errors="replace").read()
    segs=fence_split(text)
    en_prose_lines=0
    en_prose_words=0
    for seg,inf in segs:
        if inf: continue
        for line in seg.splitlines():
            line=line.strip()
            if not line: continue
            # skip headings/list bullets that are mostly identifiers
            words=en_re.findall(line)
            if len(words)>=6:
                # ignore pure link/table/identifier lines
                en_prose_lines+=1
                en_prose_words+=len(words)
    zh=len(zh_re.findall(text))
    name=p.split("/")[-1]
    flag = "CHECK" if en_prose_lines>=3 else "ok"
    print(f"{flag:5} enLines={en_prose_lines:3} enWords={en_prose_words:4} zh={zh:4}  {name}")
