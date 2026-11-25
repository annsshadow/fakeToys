import json, sys, urllib.request
# Reads BOOTSTRAP_ADMIN_PASS from environment only; never prints credentials/tokens.
out = sys.argv[1]
pw = __import__('os').environ.get('BOOTSTRAP_ADMIN_PASS','')
if not pw: raise SystemExit('NO_BOOTSTRAP_PASS')
body = json.dumps({'username':'admin','password':pw}).encode('utf-8')
req = urllib.request.Request('http://127.0.0.1:8080/api/v1/admin/login', data=body, headers={'Content-Type':'application/json'}, method='POST')
try:
    resp = urllib.request.urlopen(req, timeout=10)
    d = json.loads(resp.read())
except Exception as e:
    raise SystemExit('LOGIN_FAILED '+type(e).__name__+' '+str(getattr(e,'code','')))
t = d.get('access_token') or d.get('token') or (d.get('data') or {}).get('access_token')
if not t: raise SystemExit('NO_TOKEN')
# validate token
req2 = urllib.request.Request('http://127.0.0.1:8080/api/v1/admin/me', headers={'Authorization':'Bearer '+t})
try:
    me = json.loads(urllib.request.urlopen(req2, timeout=10).read())
except Exception as e:
    raise SystemExit('ME_FAILED '+type(e).__name__)
role = (me.get('admin') or {}).get('role','?')
open(out,'w',encoding='utf-8').write("localStorage.setItem('lyp_admin_token', "+json.dumps(t)+");\n")
print('INIT_WRITTEN role='+role+' bytes='+str(len(t)))
