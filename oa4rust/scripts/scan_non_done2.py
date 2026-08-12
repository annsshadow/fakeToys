import re, os, glob

CRATES = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), 'crates')

# Check processplatform_service_processing null matches
src = os.path.join(CRATES, 'processplatform_service_processing', 'src')
text = ''
for f in glob.glob(os.path.join(src, '**', '*.rs'), recursive=True):
    try:
        text += open(f, encoding='utf-8').read()
    except:
        pass

matches = re.findall(r'ActionResult::success\(Value::Null\)', text)
print(f"null matches: {len(matches)}")
# Show context
for i, m in enumerate(matches):
    idx = text.find(m)
    print(f"  Match {i}: ...{text[max(0,idx-40):idx+50]}...")

# Also check calendar/empower handler patterns
print("\n=== calendar handler signatures ===")
src = os.path.join(CRATES, 'calendar', 'src')
text = ''
for f in glob.glob(os.path.join(src, '**', '*.rs'), recursive=True):
    try:
        text += open(f, encoding='utf-8').read()
    except:
        pass
for line in text.split('\n'):
    if 'pub async fn' in line or 'pub fn ' in line:
        print(f"  {line.strip()}")

print("\n=== empower handler signatures ===")
src = os.path.join(CRATES, 'empower', 'src')
text = ''
for f in glob.glob(os.path.join(src, '**', '*.rs'), recursive=True):
    try:
        text += open(f, encoding='utf-8').read()
    except:
        pass
for line in text.split('\n'):
    if 'pub async fn' in line or 'pub fn ' in line:
        print(f"  {line.strip()}")

print("\n=== process_express handler signatures ===")
src = os.path.join(CRATES, 'process_express', 'src')
text = ''
for f in glob.glob(os.path.join(src, '**', '*.rs'), recursive=True):
    try:
        text += open(f, encoding='utf-8').read()
    except:
        pass
for line in text.split('\n'):
    if 'pub async fn' in line or 'pub fn ' in line:
        print(f"  {line.strip()}")
