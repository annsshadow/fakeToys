import json,sys
p,o=sys.argv[1:]
d=json.load(open(p,encoding='utf-8'))
t=d.get('access_token') or d.get('token')
if not t and isinstance(d.get('data'),dict): t=d['data'].get('access_token') or d['data'].get('token')
if not t: raise SystemExit('NO_TOKEN')
line='localStorage.setItem('+json.dumps('lyp_admin_token')+', '+json.dumps(t)+');'
open(o,'w',encoding='utf-8').write(line)
print('INIT_WRITTEN')