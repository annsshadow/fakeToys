import sys
p=sys.argv[1]
s=open(p,encoding='utf-8').read()
ok=s.startswith('localStorage.setItem(') and s.endswith(');') and s.count('(')==1 and s.count(')')==1 and 'lyp_admin_token' in s
print('AUTH_SHAPE=' + ('YES' if ok else 'NO'))
print('AUTH_BYTES='+str(len(s.encode('utf-8'))))
