import json, os, sys, importlib.util
p = r'D:/WORKSPACE/linux-7.1.3/docs/系统文档/.translate_tools.py'
spec = importlib.util.spec_from_file_location('tt', p)
t = importlib.util.module_from_spec(spec); spec.loader.exec_module(t)

d = json.load(open(r'D:/WORKSPACE/linux-7.1.3/docs/系统文档/.translate_dispatch_cold.json', encoding='utf-8'))
c = d['chunks'][int(sys.argv[1]) if len(sys.argv) > 1 else 2]
for f in c:
    a = t.analyze(f)
    print('|'.join([os.path.basename(f), a.get('decision',''), 'fe='+str(a.get('fence_even')),
                    'ch='+str(a.get('chinese')), 'en='+str(a.get('english')),
                    'r='+str(a.get('ratio')), a.get('reason','')]))
