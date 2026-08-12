import re, glob

text = ''
for f in glob.glob('crates/processplatform_service_processing/src/**/*.rs', recursive=True):
    text += open(f, encoding='utf-8').read()

matches = list(re.finditer(r'ActionResult::success\(Value::Null\)', text))
print(f"Found {len(matches)} matches:")
for i, m in enumerate(matches):
    start = max(0, m.start() - 100)
    end = min(len(text), m.end() + 50)
    print(f"\n--- Match {i+1} ---")
    print(text[start:end])
