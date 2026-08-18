import json, os, re

BASE = "D:/WORKSPACE/linux-7.1.3/docs/系统文档"
cjk = re.compile(r'[\u4e00-\u9fff]')
W = json.load(open(os.path.join(BASE, ".translate_workers.json"), encoding="utf-8"))["workers"]
N = len(W)

def fstat(path):
    try:
        d = open(path, encoding="utf-8", errors="ignore").read()
    except Exception:
        return (0.0, 0, 0, "missing")
    lines = d.split("\n")
    # Real Markdown fences: line whose stripped form starts with ``` (ignores
    # inline literal ```` ``` ````, which never starts a line).
    fn = sum(1 for l in lines if l.strip().startswith("```"))
    # Strip fenced code blocks via toggle (handles odd parity gracefully).
    depth = 0
    prose_parts = []
    for l in lines:
        if l.strip().startswith("```"):
            depth = 1 - depth
            continue
        if depth == 0:
            prose_parts.append(l)
    prose = "\n".join(prose_parts)
    prose = re.sub(r'`[^`]*`', "", prose)
    if not prose.strip():
        return (1.0, fn, len(d.encode()), "empty")
    pr = len(cjk.findall(prose)) / len(prose)
    return (pr, fn, len(d.encode()), "ok")

done, partial, english = [], [], []
detail = {}
for w in range(N):
    files = W[w]
    ok = 0
    bad = []
    for f in files:
        pr, fn, sz, st = fstat(f)
        # Translated if prose is substantially Chinese (fence parity handled
        # separately by _fix_fences.py; do NOT let odd fences cause false ENGLISH).
        good = (st == "empty") or (pr > 0.03)
        if good:
            ok += 1
        else:
            bad.append((os.path.basename(f), round(pr, 3), fn, st))
    detail[w] = {"files": len(files), "ok": ok, "bad": bad}
    if ok == len(files):
        done.append(w)
    elif ok == 0:
        english.append(w)
    else:
        partial.append(w)

out = {
    "total": N,
    "done": done,
    "partial": partial,
    "english": english,
    "detail": {str(w): detail[w] for w in partial + english},
}
json.dump(out, open(os.path.join(BASE, ".translate_fullscan.json"), "w", encoding="utf-8"),
          ensure_ascii=False, indent=1)

print("TOTAL workers:", N)
print("DONE (all files translated):", len(done))
print("PARTIAL:", len(partial))
print("ENGLISH (0 done):", len(english))
print("Pending total:", len(partial) + len(english))
print("DONE list:", done)
print("ENGLISH list:", english)
print("PARTIAL list:", partial)
