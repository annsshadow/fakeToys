import pathlib,re,sys
D=pathlib.Path(sys.argv[1])
pats=[re.compile('eyJ[A-Za-z0-9_-]{10,}'),re.compile('Bearer\\s+[A-Za-z0-9._-]{10,}'),re.compile('postgres://[^\\s\\"\']+:[^\\s\\"\']+@'),re.compile('C:\\\\Users\\\\[^\\s\\\"]+')]
files=[p for p in D.rglob('*') if p.is_file() and p.suffix not in ('.png','.dump')]
hits=[];count=0
for p in files:
 try:text=p.read_text(encoding='utf-8')
 except Exception:continue
 count+=1
 for n,line in enumerate(text.splitlines(),1):
  if any(pat.search(line) for pat in pats):hits.append((str(p),n,line[:120]))
print('SECRET_SCAN_FILES='+str(count));print('SECRET_HITS='+str(len(hits)))
for h in hits[:10]:print(h)
