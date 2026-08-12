import re, os, glob
CRATES = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), 'crates')
for name in sorted(os.listdir(CRATES)):
    src = os.path.join(CRATES, name, 'src')
    if not os.path.isdir(src):
        continue
    text = ''
    for f in glob.glob(os.path.join(src, '**', '*.rs'), recursive=True):
        try:
            text += open(f, encoding='utf-8').read()
        except:
            pass
    stub = text.count('stub_')
    null = len(re.findall(r'ActionResult::success\(Value::Null\)', text))
    handlers = len(re.findall(r'->\s*Result<Json<ActionResult<Value>>,\s*AppError>', text))
    routes = len(re.findall(r'\.route\(\s*"', text))
    if handlers == 0:
        print(f"TODO   {name:50} routes={routes}")
    elif stub > 0 or null > 0:
        print(f"DOING  {name:50} routes={routes} handlers={handlers} stub={stub} null={null}")
