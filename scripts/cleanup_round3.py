"""Round 3: Bulk stub removal from A-class views (have useQuery)."""
import re, os

VIEWS = '/d/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views'
FILES = [
    'FileManager.vue', 'AttendanceApp.vue', 'BBSForum.vue',
    'CommonApp.vue', 'IMChat.vue', 'CalendarApp.vue',
    'MeetingApp.vue', 'CmsModuleApp.vue', 'QueryManagerDeep.vue',
    'Dashboard.vue', 'TemplateApp.vue',
]

STUB_RE = re.compile(r'async function (api_|call_)\w+\(\) \{ try \{ await api\.(get|post|put|delete)\("[^"]+"\) \} catch \{\} \}\n?')
DOUBLE_COLON = ["'失败: : '", "'更新失败: : '", "'取消失败: : '", "'审批失败: : '", "'加入失败: : '", "'离开失败: : '", "'创建失败: : '", "'保存失败: : '"]

total_removed = 0
total_typos = 0

for fname in FILES:
    fpath = os.path.join(VIEWS, fname)
    with open(fpath, encoding='utf-8') as f:
        content = f.read()
    original = content

    # Remove stubs
    matches = STUB_RE.findall(content)
    stub_count = len(matches)
    content = STUB_RE.sub('', content)
    content = re.sub(r'\n{3,}', '\n\n', content)

    # Fix double-colon typos
    typo_count = 0
    for old in DOUBLE_COLON:
        c = content.count(old)
        if c:
            typo_count += c
            content = content.replace(old, old.replace(': : ', ': '))

    if content != original:
        with open(fpath, 'w', encoding='utf-8') as f:
            f.write(content)

    total_removed += stub_count
    total_typos += typo_count
    print(f"  {fname}: {stub_count} stubs removed, {typo_count} typos fixed")

print(f"\nRound 3 total: {total_removed} stubs removed, {total_typos} typos fixed")
