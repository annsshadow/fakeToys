import sys, os
# replace only explicitly-listed .tmp files (paths passed as args)
for t in sys.argv[1:]:
    if not t.endswith('.tmp'):
        t = t + '.tmp'
    if os.path.exists(t):
        dst = t[:-4]
        os.replace(t, dst)
        print('replaced', dst)
    else:
        print('MISSING', t)
