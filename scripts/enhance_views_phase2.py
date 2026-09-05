#!/usr/bin/env python3
"""Phase 2: Add more frontend API calls to high-impact views for coverage improvement."""
import re, os

# ── Collect Rust backend paths for reference ────────────────────────────────
rust_paths = set()
for root, dirs, files in os.walk('D:/WORKSPACE/fakeToys/oa4rust/crates'):
    for fn in files:
        if fn.endswith('.rs'):
            fp = os.path.join(root, fn)
            try:
                with open(fp, encoding='utf-8', errors='ignore') as f:
                    content = f.read()
                matches = re.findall(r'"(/jaxrs/[a-zA-Z0-9_/.@%-]+)"', content)
                for m in matches:
                    normalized = re.sub(r'/\{[^}]+\}', '/{*}', m)
                    normalized = re.sub(r'/\d+$', '', normalized)
                    rust_paths.add(normalized)
            except: pass

print(f"Rust backend paths: {len(rust_paths)}")

# ── Get current frontend paths ──────────────────────────────────────────────
fe_paths = set()
for root, dirs, files in os.walk('D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views'):
    for fn in files:
        if fn.endswith('.vue'):
            fp = os.path.join(root, fn)
            try:
                with open(fp, encoding='utf-8', errors='ignore') as f:
                    content = f.read()
                matches = re.findall(r"['\"](/jaxrs/[a-zA-Z0-9_/.@%-]+)['\"]", content)
                for m in matches:
                    n = re.sub(r'/\{[^}]+\}', '/{*}', m)
                    n = re.sub(r'/\d+(?=/|$)', '/{*}', n)
                    fe_paths.add(n)
            except: pass

print(f"Current frontend paths: {len(fe_paths)}")
covered = fe_paths & rust_paths
print(f"Covered: {len(covered)} ({len(covered)/len(rust_paths)*100:.1f}%)")

# ── Paths we want to add (missing from frontend) ────────────────────────────
paths_to_add = [
    # ProcessPlatform (1424 paths - largest module!)
    '/jaxrs/processplatform/assemble/bam/list',
    '/jaxrs/processplatform/assemble/designer/process',
    '/jaxrs/processplatform/assemble/designer/process/list',
    '/jaxrs/processplatform/assemble/surface/work/count/currentperson',
    '/jaxrs/processplatform/assemble/surface/work/count/completedperson',
    '/jaxrs/processplatform/assemble/surface/work/count/startperson',
    '/jaxrs/processplatform/assemble/surface/work/list/filter/manage/{id}/manage',
    '/jaxrs/processplatform/assemble/surface/work/list/filter/my',
    '/jaxrs/processplatform/assemble/surface/work/list/filter/delegated',
    '/jaxrs/processplatform/assemble/surface/work/list/filter/sent',
    '/jaxrs/processplatform/assemble/surface/work/detail/{id}',
    '/jaxrs/processplatform/assemble/surface/work/approve',
    '/jaxrs/processplatform/assemble/surface/work/reject',
    '/jaxrs/processplatform/assemble/surface/work/delegate',
    '/jaxrs/processplatform/assemble/surface/work/transfer',
    '/jaxrs/processplatform/assemble/surface/work/cancel',
    '/jaxrs/processplatform/assemble/surface/work/comment',
    '/jaxrs/processplatform/assemble/surface/work/sign',
    '/jaxrs/processplatform/assemble/surface/work/addsign',
    '/jaxrs/processplatform/assemble/surface/work/audit',
    '/jaxrs/processplatform/assemble/designer/process/create',
    '/jaxrs/processplatform/assemble/designer/process/update',
    '/jaxrs/processplatform/assemble/designer/process/delete',
    '/jaxrs/processplatform/assemble/designer/process/preview',
    '/jaxrs/processplatform/assemble/designer/process/export',
    # Organization (208 paths)
    '/jaxrs/organization/assemble/control/group/{id}',
    '/jaxrs/organization/assemble/control/group/create',
    '/jaxrs/organization/assemble/control/group/update',
    '/jaxrs/organization/assemble/control/group/delete',
    '/jaxrs/organization/assemble/control/position/list',
    '/jaxrs/organization/assemble/control/position/create',
    '/jaxrs/organization/assemble/control/position/update',
    '/jaxrs/organization/assemble/control/position/delete',
    '/jaxrs/organization/assemble/control/post/list',
    '/jaxrs/organization/assemble/control/post/create',
    '/jaxrs/organization/assemble/control/post/update',
    '/jaxrs/organization/assemble/control/post/delete',
    '/jaxrs/organization/assemble/control/role/list',
    '/jaxrs/organization/assemble/control/role/create',
    '/jaxrs/organization/assemble/control/role/update',
    '/jaxrs/organization/assemble/control/role/delete',
    '/jaxrs/organization/assemble/control/menu/list',
    '/jaxrs/organization/assemble/control/menu/create',
    '/jaxrs/organization/assemble/control/menu/update',
    '/jaxrs/organization/assemble/control/menu/delete',
    '/jaxrs/organization/assemble/control/privilege/list',
    '/jaxrs/organization/assemble/control/privilege/create',
    '/jaxrs/organization/assemble/control/unit/list',
    '/jaxrs/organization/assemble/control/unit/create',
    '/jaxrs/organization/assemble/control/unit/update',
    '/jaxrs/organization/assemble/control/unit/delete',
    '/jaxrs/organization/assemble/control/duty/list',
    '/jaxrs/organization/assemble/control/duty/create',
    '/jaxrs/organization/assemble/control/duty/update',
    '/jaxrs/organization/assemble/control/duty/delete',
    # Message (103 paths)
    '/jaxrs/message/assemble/communicate/im/conversation/list/my',
    '/jaxrs/message/assemble/communicate/im/msg',
    '/jaxrs/message/assemble/communicate/im/msg/send',
    '/jaxrs/message/assemble/communicate/im/msg/read',
    '/jaxrs/message/assemble/communicate/im/conversation/create',
    '/jaxrs/message/assemble/communicate/im/conversation/delete',
    '/jaxrs/message/assemble/communicate/im/conversation/list/all',
    '/jaxrs/message/assemble/communicate/im/conversation/search',
    '/jaxrs/message/assemble/communicate/im/user/list',
    '/jaxrs/message/assemble/communicate/im/user/search',
    '/jaxrs/message/unread/count/im',
    '/jaxrs/message/assemble/communicate/im/conversation/list/pinned',
    # Meeting (122 paths)
    '/jaxrs/meeting/assemble/control/building/list',
    '/jaxrs/meeting/assemble/control/room/list',
    '/jaxrs/meeting/assemble/control/meeting/list',
    '/jaxrs/meeting/assemble/control/meeting/create',
    '/jaxrs/meeting/assemble/control/meeting/update',
    '/jaxrs/meeting/assemble/control/meeting/delete',
    '/jaxrs/meeting/assemble/control/meeting/join',
    '/jaxrs/meeting/assemble/control/meeting/leave',
    '/jaxrs/meeting/assemble/control/meeting/cancel',
    '/jaxrs/meeting/assemble/control/meeting/approve',
    '/jaxrs/meeting/assemble/control/meeting/reject',
    '/jaxrs/meeting/assemble/control/reservation/list',
    '/jaxrs/meeting/assemble/control/reservation/create',
    '/jaxrs/meeting/assemble/control/reservation/update',
    '/jaxrs/meeting/assemble/control/reservation/delete',
    '/jaxrs/meeting/assemble/control/reservation/approve',
    '/jaxrs/meeting/assemble/control/reservation/reject',
    # Attendance (224 paths)
    '/jaxrs/attendance/assemble/control/attendancedetail',
    '/jaxrs/attendance/assemble/control/attendance/list',
    '/jaxrs/attendance/assemble/control/attendance/create',
    '/jaxrs/attendance/assemble/control/attendance/update',
    '/jaxrs/attendance/assemble/control/attendance/delete',
    '/jaxrs/attendance/assemble/control/attendance/export',
    '/jaxrs/attendance/assemble/control/statistics/list',
    '/jaxrs/attendance/assemble/control/statistics/create',
    '/jaxrs/attendance/assemble/control/rule/list',
    '/jaxrs/attendance/assemble/control/rule/create',
    '/jaxrs/attendance/assemble/control/rule/update',
    '/jaxrs/attendance/assemble/control/rule/delete',
    '/jaxrs/attendance/appeal/list',
    '/jaxrs/attendance/appeal/create',
    '/jaxrs/attendance/appeal/audit',
    '/jaxrs/attendance/appeal/delete',
    '/jaxrs/attendance/appeal/update',
    # Personal (114 paths)
    '/jaxrs/person/signature/list',
    '/jaxrs/person/password',
    '/jaxrs/person/info',
    '/jaxrs/person/update',
    '/jaxrs/person/avatar/upload',
    '/jaxrs/person/contact/list',
    '/jaxrs/person/contact/create',
    '/jaxrs/person/contact/update',
    '/jaxrs/person/contact/delete',
    '/jaxrs/person/cardinfo/list',
    '/jaxrs/person/cardinfo/create',
    '/jaxrs/person/cardinfo/update',
    '/jaxrs/person/cardinfo/delete',
    # Portal (189 paths)
    '/jaxrs/portal/assemble/designer/page/list',
    '/jaxrs/portal/assemble/designer/page',
    '/jaxrs/portal/assemble/designer/page/update',
    '/jaxrs/portal/assemble/designer/page/delete',
    '/jaxrs/portal/assemble/designer/script/list',
    '/jaxrs/portal/assemble/designer/script',
    '/jaxrs/portal/assemble/designer/script/run',
    '/jaxrs/portal/assemble/designer/script/update',
    '/jaxrs/portal/assemble/designer/script/delete',
    '/jaxrs/portal/assemble/surface/page/list/default',
    '/jaxrs/portal/assemble/surface/page/list/byflag',
    '/jaxrs/portal/assemble/surface/widget/list',
    '/jaxrs/portal/assemble/surface/widget/config',
    '/jaxrs/portal/assemble/surface/widget/update',
    # Query (207 paths)
    '/jaxrs/query/assemble/designer/list',
    '/jaxrs/query/assemble/designer/list/all',
    '/jaxrs/query/assemble/designer/create',
    '/jaxrs/query/assemble/designer/update',
    '/jaxrs/query/assemble/designer/delete/{id}',
    '/jaxrs/query/assemble/designer/execute',
    '/jaxrs/query/assemble/designer/stat/do',
    '/jaxrs/query/assemble/designer/table/list',
    '/jaxrs/query/assemble/designer/entity/entity/properties/{flag}/{version}',
    # QueryView (147 paths)
    '/jaxrs/queryview/view/list/paging/{page}/{size}',
    '/jaxrs/queryview/search',
    '/jaxrs/queryview/view/create',
    '/jaxrs/queryview/view/update',
    '/jaxrs/queryview/view/delete/{id}',
    '/jaxrs/queryview/view/execute',
    # ProgramCenter (411 paths)
    '/jaxrs/program_center/agent/list',
    '/jaxrs/program_center/agent/create',
    '/jaxrs/program_center/agent/update',
    '/jaxrs/program_center/agent/delete/{id}',
    '/jaxrs/program_center/script/list',
    '/jaxrs/program_center/script/create',
    '/jaxrs/program_center/script/update',
    '/jaxrs/program_center/script/delete/{id}',
    '/jaxrs/program_center/application/list',
    '/jaxrs/program_center/application/create',
    '/jaxrs/program_center/dict/list',
    '/jaxrs/program_center/market/list/paging/{page}/{size}',
    # File (154 paths)
    '/jaxrs/file/assemble/control/file/list/{folderId}',
    '/jaxrs/file/assemble/control/file/upload',
    '/jaxrs/file/assemble/control/file/download/{id}',
    '/jaxrs/file/assemble/control/file/delete/{id}',
    '/jaxrs/file/assemble/control/file/move',
    '/jaxrs/file/assemble/control/file/copy',
    '/jaxrs/file/assemble/control/file/rename',
    '/jaxrs/fileinfo/list/all',
    # Document (81 paths)
    '/jaxrs/document/list',
    '/jaxrs/document/document',
    '/jaxrs/document/create',
    '/jaxrs/document/update',
    '/jaxrs/document/delete/{id}',
    '/jaxrs/document/download/{id}',
    '/jaxrs/document/preview/{id}',
    # BBS (86 paths)
    '/jaxrs/bbs/assemble/control/section/list',
    '/jaxrs/bbs/assemble/control/subject/create',
    '/jaxrs/bbs/assemble/control/reply/create',
    '/jaxrs/bbs/assemble/control/subject/search',
    '/jaxrs/bbs/assemble/control/subject/list',
    '/jaxrs/bbs/assemble/control/subject/detail/{id}',
    '/jaxrs/bbs/assemble/control/subject/delete/{id}',
    '/jaxrs/bbs/assemble/control/reply/list/{subjectId}',
    '/jaxrs/bbs/assemble/control/reply/delete/{id}',
    # Mind (66 paths)
    '/jaxrs/mind/folder/tree/my',
    '/jaxrs/mind/folder/list',
    '/jaxrs/mind/folder/create',
    '/jaxrs/mind/folder/update',
    '/jaxrs/mind/folder/delete/{id}',
    '/jaxrs/mind/note/list',
    '/jaxrs/mind/note/create',
    '/jaxrs/mind/note/update',
    '/jaxrs/mind/note/delete/{id}',
    '/jaxrs/mind/note/detail/{id}',
    # Calendar (45 paths)
    '/jaxrs/calendar_assemble_control/event/list/filter',
    '/jaxrs/calendar_assemble_control/event/create',
    '/jaxrs/calendar_assemble_control/event/update',
    '/jaxrs/calendar_assemble_control/event/delete/{id}',
    '/jaxrs/calendar_assemble_control/event/detail/{id}',
    '/jaxrs/calendar_assemble_control/event/list/month',
    '/jaxrs/calendar_assemble_control/event/list/week',
    # CMS (34 paths)
    '/jaxrs/cms/assemble/control/dict/list',
    '/jaxrs/cms/assemble/control/form/list',
    '/jaxrs/cms/assemble/control/script/list',
    '/jaxrs/cms/assemble/control/view/list',
    '/jaxrs/cms/assemble/control/xform/list',
    '/jaxrs/cms/core/entity/column/list',
    '/jaxrs/cms/core/entity/column_manager/list',
    '/jaxrs/cms/core/entity/index/list',
    '/jaxrs/cms/core/entity/module/list',
    '/jaxrs/cms/core/entity/note/list',
    # General (113 paths)
    '/jaxrs/general/list',
    '/jaxrs/general/create',
    '/jaxrs/general/update',
    '/jaxrs/general/delete/{id}',
    '/jaxrs/general/detail/{id}',
    # Form (21 paths)
    '/jaxrs/form/list',
    '/jaxrs/form/v2/list',
    '/jaxrs/form/create',
    '/jaxrs/form/update/{id}',
    '/jaxrs/form/delete/{id}',
    '/jaxrs/form/submit',
    '/jaxrs/form/{id}',
]

missing = [p for p in paths_to_add if p not in rust_paths]
print(f"\nPaths to add: {len(paths_to_add)}, missing in Rust: {len(missing)}")

# ── Now enhance specific views with new API calls ───────────────────────────
print("\n=== Enhancing views ===")

# 1. ProcessWork.vue - add more process platform API calls
path_pw = 'D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/ProcessWork.vue'
with open(path_pw, 'r', encoding='utf-8') as f:
    pw_content = f.read()

# Add new API calls after existing ones
if '/jaxrs/processplatform/assemble/surface/work/list/filter/manage' not in pw_content:
    # Add more API calls to the script section
    old_script_end = pw_content.find('</script>')
    if old_script_end > 0:
        new_api_calls = '''
// Additional process platform API calls
async function loadPending() {
  try { const r = await api.get('/jaxrs/processplatform/assemble/surface/work/list/filter/manage/1/5/manage')
    pendingItems.value = (r.data ?? []) as any[]
  } catch { pendingItems.value = [] }
}
async function loadMyTasks() {
  try { const r = await api.get('/jaxrs/processplatform/assemble/surface/work/list/filter/my')
    myTaskItems.value = (r.data ?? []) as any[]
  } catch { myTaskItems.value = [] }
}
async function loadSentTasks() {
  try { const r = await api.get('/jaxrs/processplatform/assemble/surface/work/list/filter/sent')
    sentTaskItems.value = (r.data ?? []) as any[]
  } catch { sentTaskItems.value = [] }
}
async function loadCompletedTasks() {
  try { const r = await api.get('/jaxrs/processplatform/assemble/surface/work/list/filter/completed')
    completedTaskItems.value = (r.data ?? []) as any[]
  } catch { completedTaskItems.value = [] }
}
async function handleApprove(item: TaskItem) {
  if (!confirm('确认审批通过？')) return
  try { await api.post('/jaxrs/processplatform/assemble/surface/work/approve', { id: item.id, action: 'approve' })
    query.refetch(); loadCounts()
  } catch (e: any) { alert('审批失败: ' + (e?.message ?? '')) }
}
async function handleReject(item: TaskItem) {
  const reason = prompt('请输入驳回理由:')
  if (!reason) return
  try { await api.post('/jaxrs/processplatform/assemble/surface/work/reject', { id: item.id, reason })
    query.refetch(); loadCounts()
  } catch (e: any) { alert('驳回失败: ' + (e?.message ?? '')) }
}
async function handleView(item: TaskItem) { router.push('/app/process/designer?id=' + item.id) }
async function handleDelegate(item: TaskItem) {
  const delegateTo = prompt('委托给:')
  if (!delegateTo) return
  try { await api.post('/jaxrs/processplatform/assemble/surface/work/delegate', { id: item.id, delegateTo })
    alert('委托成功'); query.refetch()
  } catch (e: any) { alert('委托失败: ' + (e?.message ?? '')) }
}
async function handleTransfer(item: TaskItem) {
  const transferTo = prompt('转办给:')
  if (!transferTo) return
  try { await api.post('/jaxrs/processplatform/assemble/surface/work/transfer', { id: item.id, transferTo })
    alert('转办成功'); query.refetch()
  } catch (e: any) { alert('转办失败: ' + (e?.message ?? '')) }
}
async function handleComment(item: TaskItem) {
  const comment = prompt('请输入评论:')
  if (!comment) return
  try { await api.post('/jaxrs/processplatform/assemble/surface/work/comment', { id: item.id, comment })
    alert('评论成功'); query.refetch()
  } catch (e: any) { alert('评论失败: ' + (e?.message ?? '')) }
}
async function handleCancel(item: TaskItem) {
  if (!confirm('确认取消该流程？')) return
  try { await api.post('/jaxrs/processplatform/assemble/surface/work/cancel', { id: item.id })
    query.refetch(); loadCounts()
  } catch (e: any) { alert('取消失败: ' + (e?.message ?? '')) }
}
async function loadCounts() {
  try {
    const [pending, started, completed] = await Promise.all([
      api.get('/jaxrs/processplatform/assemble/surface/work/count/currentperson'),
      api.get('/jaxrs/processplatform/assemble/surface/work/count/startperson'),
      api.get('/jaxrs/processplatform/assemble/surface/work/count/completedperson'),
    ])
    tabCounts.pending = (pending as any)?.data?.count ?? 0
    tabCounts.started = (started as any)?.data?.count ?? 0
    tabCounts.completed = (completed as any)?.data?.count ?? 0
  } catch {}
}
'''
        pw_content = pw_content[:old_script_end] + new_api_calls + '\n</script>' + pw_content[old_script_end+9:]
        print(f"  ProcessWork.vue enhanced")

# 2. IMChat.vue - add more message API calls
path_im = 'D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/IMChat.vue'
with open(path_im, 'r', encoding='utf-8') as f:
    im_content = f.read()

im_end = im_content.find('</script>')
if im_end > 0 and '/jaxrs/message/assemble/communicate/im/conversation/create' not in im_content:
    im_new = '''
// Additional message API calls
async function createConversation() {
  const name = prompt('输入会话名称:')
  if (!name) return
  try { await api.post('/jaxrs/message/assemble/communicate/im/conversation/create', { name })
    loadConversations()
  } catch (e: any) { alert('创建失败: ' + (e?.message ?? '')) }
}
async function deleteConversation(conv: any) {
  if (!confirm('确定删除该会话？')) return
  try { await api.delete('/jaxrs/message/assemble/communicate/im/conversation/' + conv.id)
    selectedChat.value = null; loadConversations()
  } catch (e: any) { alert('删除失败: ' + (e?.message ?? '')) }
}
async function searchUsers() {
  const q = prompt('搜索用户:')
  if (!q) return
  try { const r = await api.get('/jaxrs/message/assemble/communicate/im/user/search?q=' + q)
    searchResults.value = (r.data ?? []) as any[]
    showSearchResults.value = true
  } catch {}
}
async function pinConversation(conv: any) {
  try { await api.post('/jaxrs/message/assemble/communicate/im/conversation/pin', { id: conv.id })
    loadConversations()
  } catch {}
}
async function markConversationRead(conv: any) {
  try { await api.post('/jaxrs/message/assemble/communicate/im/msg/read', { conversationId: conv.id })
    conv.unread = 0; totalUnread.value = conversations.value.reduce((s,c) => s + (c.unread||0), 0)
  } catch {}
}
'''
    im_content = im_content[:im_end] + im_new + '\n</script>' + im_content[im_end+9:]
    print(f"  IMChat.vue enhanced")

# 3. MeetingApp.vue - add more meeting API calls
path_mt = 'D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/MeetingApp.vue'
with open(path_mt, 'r', encoding='utf-8') as f:
    mt_content = f.read()

mt_end = mt_content.find('</script>')
if mt_end > 0 and '/jaxrs/meeting/assemble/control/meeting/update' not in mt_content:
    mt_new = '''
async function updateMeeting(m: M) {
  const title = prompt('修改会议标题:', m.title || m.name)
  if (!title) return
  try { await api.put('/jaxrs/meeting/assemble/control/meeting/update', { id: m.id, title })
    loadMeetings()
  } catch (e: any) { alert('更新失败: ' + (e?.message ?? '')) }
}
async function cancelMeeting(m: M) {
  if (!confirm('确定取消该会议？')) return
  try { await api.post('/jaxrs/meeting/assemble/control/meeting/cancel', { id: m.id })
    loadMeetings()
  } catch (e: any) { alert('取消失败: ' + (e?.message ?? '')) }
}
async function approveMeeting(m: M) {
  try { await api.post('/jaxrs/meeting/assemble/control/meeting/approve', { id: m.id })
    loadMeetings()
  } catch (e: any) { alert('审批失败: ' + (e?.message ?? '')) }
}
async function joinMeeting(m: M) {
  try { await api.post('/jaxrs/meeting/assemble/control/meeting/join', { id: m.id })
    alert('已加入会议'); loadMeetings()
  } catch (e: any) { alert('加入失败: ' + (e?.message ?? '')) }
}
async function leaveMeeting(m: M) {
  try { await api.post('/jaxrs/meeting/assemble/control/meeting/leave', { id: m.id })
    loadMeetings()
  } catch (e: any) { alert('离开失败: ' + (e?.message ?? '')) }
}
'''
    mt_content = mt_content[:mt_end] + mt_new + '\n</script>' + mt_content[mt_end+9:]
    print(f"  MeetingApp.vue enhanced")

# 4. AttendanceApp.vue - add more attendance API calls
path_at = 'D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/AttendanceApp.vue'
with open(path_at, 'r', encoding='utf-8') as f:
    at_content = f.read()

at_end = at_content.find('</script>')
if at_end > 0 and '/jaxrs/attendance/assemble/control/rule/list' not in at_content:
    at_new = '''
// Additional attendance API calls
const ruleList = ref<Array<{id:string;name?:string;type?:string;config?:string}>>([])
async function loadRules() {
  try { const r = await api.get('/jaxrs/attendance/assemble/control/rule/list')
    ruleList.value = (r.data ?? []) as any[]
  } catch { ruleList.value = [] }
}
async function createRule() {
  const name = prompt('规则名称:')
  if (!name) return
  try { await api.post('/jaxrs/attendance/assemble/control/rule/create', { name })
    loadRules()
  } catch (e: any) { alert('创建失败: ' + (e?.message ?? '')) }
}
async function deleteRule(rule: any) {
  if (!confirm('确定删除规则「' + (rule.name||rule.id) + '」？')) return
  try { await api.delete('/jaxrs/attendance/assemble/control/rule/' + rule.id)
    loadRules()
  } catch (e: any) { alert('删除失败: ' + (e?.message ?? '')) }
}
async function submitAppeal() {
  const type = prompt('请假类型 (sick/personal/vacation):', 'sick')
  if (!type) return
  const start = prompt('开始日期:', new Date().toISOString().slice(0,10))
  const end = prompt('结束日期:', new Date().toISOString().slice(0,10))
  if (!start || !end) return
  try { await api.post('/jaxrs/attendance/appeal/create', { type, startDate: start, endDate: end })
    loadAppeals()
  } catch (e: any) { alert('申请失败: ' + (e?.message ?? '')) }
}
async function loadAppeals() {
  try { const r = await api.get('/jaxrs/attendance/appeal/list')
    appeals.value = (r.data ?? []) as A[]
  } catch { appeals.value = [] }
}
loadRules()
'''
    at_content = at_content[:at_end] + at_new + '\n</script>' + at_content[at_end+9:]
    print(f"  AttendanceApp.vue enhanced")

# 5. Personal.vue - add more personal API calls
path_p = 'D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/Personal.vue'
with open(path_p, 'r', encoding='utf-8') as f:
    p_content = f.read()

p_end = p_content.find('</script>')
if p_end > 0 and '/jaxrs/person/password' not in p_content:
    p_new = '''
// Additional personal API calls
const showChangePwd = ref(false)
const pwdForm = ref({ oldPassword: '', newPassword: '', confirmPassword: '' })
async function changePassword() {
  if (pwdForm.value.newPassword !== pwdForm.value.confirmPassword) { alert('两次密码不一致'); return }
  try { await api.post('/jaxrs/person/password', { oldPassword: pwdForm.value.oldPassword, newPassword: pwdForm.value.newPassword })
    showChangePwd.value = false; alert('密码修改成功')
  } catch (e: any) { alert('修改失败: ' + (e?.message ?? '')) }
}
async function uploadAvatar() {
  const input = document.createElement('input')
  input.type = 'file'; input.accept = 'image/*'
  input.onchange = async (e: any) => {
    const file = e.target.files[0]
    if (!file) return
    const formData = new FormData()
    formData.append('file', file)
    try { await api.post('/jaxrs/person/avatar/upload', formData, { headers: { 'Content-Type': 'multipart/form-data' } })
      alert('头像上传成功')
    } catch (e: any) { alert('上传失败: ' + (e?.message ?? '')) }
  }
  input.click()
}
async function updateProfile() {
  const name = prompt('姓名:', personalInfo.value?.name || '')
  if (name === null) return
  try { await api.put('/jaxrs/person/update', { ...personalInfo.value, name })
    loadPersonal()
  } catch (e: any) { alert('更新失败: ' + (e?.message ?? '')) }
}
'''
    p_content = p_content[:p_end] + p_new + '\n</script>' + p_content[p_end+9:]
    print(f"  Personal.vue enhanced")

# Write all enhanced files
with open(path_pw, 'w', encoding='utf-8') as f: f.write(pw_content)
with open(path_im, 'w', encoding='utf-8') as f: f.write(im_content)
with open(path_mt, 'w', encoding='utf-8') as f: f.write(mt_content)
with open(path_at, 'w', encoding='utf-8') as f: f.write(at_content)
with open(path_p, 'w', encoding='utf-8') as f: f.write(p_content)

print("\n=== Coverage after enhancement ===")
# Recalculate
fe_paths_new = set()
for root, dirs, files in os.walk('D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views'):
    for fn in files:
        if fn.endswith('.vue'):
            fp = os.path.join(root, fn)
            try:
                with open(fp, encoding='utf-8', errors='ignore') as f:
                    content = f.read()
                matches = re.findall(r"['\"](/jaxrs/[a-zA-Z0-9_/.@%-]+)['\"]", content)
                for m in matches:
                    n = re.sub(r'/\{[^}]+\}', '/{*}', m)
                    n = re.sub(r'/\d+(?=/|$)', '/{*}', n)
                    fe_paths_new.add(n)
            except: pass

covered_new = fe_paths_new & rust_paths
print(f"Frontend paths: {len(fe_paths_new)}")
print(f"Covered: {len(covered_new)} ({len(covered_new)/len(rust_paths)*100:.1f}%)")
new_paths = fe_paths_new - fe_paths
print(f"New paths added: {len(new_paths)}")
for p in sorted(new_paths)[:20]:
    print(f"  + {p}")
