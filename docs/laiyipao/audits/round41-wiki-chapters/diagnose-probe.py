import json,sys
p=sys.argv[1]
s=open(p,encoding='utf-8').read()
pos=s.find('(')
print('AUTH_PREFIX='+s[:pos+1])
print('AUTH_SUFFIX='+s[-12:].replace(s[s.rfind(',')+1:-2],'<MASK>'))
print('AUTH_OPEN_PARENS='+str(s.count('('))+' CLOSE='+str(s.count(')')))
q=sys.argv[2]
v=json.load(open(q,encoding='utf-8'))
if isinstance(v,str): v=json.loads(v)
print('PROBE_KEYS='+','.join(sorted(v.keys())))
for k in ('rows','headers','sectionVisible','activeChapter','hash','error'):
 print(k+'='+repr(v.get(k))[:600])
