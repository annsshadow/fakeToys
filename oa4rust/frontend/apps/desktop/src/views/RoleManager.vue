<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="role-view modern-page ai-page-b5" data-testid="role-page">
    <section class="modern-section glass-card content-panel">
      <div class="toolbar modern-hero">
        <p class="ai-gradient-text b5-title" aria-label="管理角色、Flag 与成员分配">管理角色、Flag 与成员分配</p>
        <button class="btn-primary" data-testid="role-refresh" @click="loadRoles">刷新</button>
        <button class="btn-create" data-testid="role-create" @click="showCreate = true">+ 新建角色</button>
      </div>
      <div class="list-panel">
        <ListLoadError v-if="roles.length > 0 && loadError" mode="banner" :message="loadError" data-testid="role-list-error" @retry="loadRoles" />
        <div v-if="loading" class="loading-row"><div class="sk" v-for="i in 5" :key="i"></div></div>
        <ListLoadError v-else-if="roles.length === 0 && loadError" mode="empty" :message="loadError" data-testid="role-list-error" @retry="loadRoles" />
        <div v-else-if="roles.length === 0" class="empty"><div class="ei">🔑</div><p>暂无角色</p><button class="btn-create" data-testid="role-empty-create" @click="showCreate = true">+ 新建第一个角色</button></div>
        <div v-else class="item-table">
          <div class="table-header">
            <span class="col-name">角色名称</span>
            <span class="col-flag">标识</span>
            <span class="col-actions">操作</span>
          </div>
          <div v-for="r in roles" :key="r.flag" class="table-row glass-card">
            <span class="col-name">{{ r.name || r.roleName || r.title || "未命名" }}</span>
            <span class="col-flag"><span class="flag-label">标识</span> <span class="flag-value font-mono" data-testid="role-flag">{{ r.flag || r.id || "-" }}</span></span>
            <span class="col-actions">
              <button class="btn-edit" data-testid="role-members" @click="showRoleMembers(r)">成员</button>
              <button class="btn-edit" data-testid="role-edit" @click="editRole(r)">编辑</button>
              <button class="btn-del" data-testid="role-delete" @click="deleteRole(r)">删除</button>
            </span>
          </div>
        </div>
      </div>
    </section>
    <!-- Create/Edit modal -->
    <div v-if="showCreate" class="modal-overlay" @click.self="showCreate = false">
      <div class="modal glass-card">
        <h3>{{ editingRole ? "编辑角色" : "新建角色" }}</h3>
        <div class="form-group">
          <label>角色名称</label>
          <input v-model="form.name" class="form-input" placeholder="请输入角色名称" />
        </div>
        <div class="form-group">
          <label>Flag（唯一标识）</label>
          <input v-model="form.flag" class="form-input" placeholder="如: admin, editor" :disabled="!!editingRole" />
        </div>
        <div class="form-group">
          <label>描述</label>
          <textarea v-model="form.desc" class="form-textarea" placeholder="角色描述"></textarea>
        </div>
        <div class="modal-actions">
          <button class="btn-cancel" @click="showCreate = false">取消</button>
          <button class="btn-primary" :disabled="creating" @click="onSave">
            {{ creating ? "保存中..." : "保存" }}
          </button>
        </div>
      </div>
    </div>
    <!-- 角色成员权限明细可视化 -->
    <div v-if="memberDetail" class="modal-overlay" @click.self="memberDetail = null">
      <div class="modal glass-card member-modal" data-testid="role-member-detail">
        <h3>角色权限明细</h3>
        <p class="member-summary">角色 <strong>{{ memberDetail.roleName }}</strong> 当前共有 <strong data-testid="role-member-count">{{ memberDetail.members.length }}</strong> 名成员</p>
        <div v-if="memberDetail.members.length === 0" class="member-empty" data-testid="role-member-empty">该角色暂无分配成员，可在组织架构中完成分配。</div>
        <ul v-else class="member-list" data-testid="role-member-list">
          <li v-for="(m, i) in memberDetail.members" :key="i" class="member-item">
            <span class="member-avatar">{{ memberInitial(m) }}</span>
            <span class="member-info"><strong>{{ displayMember(m) }}</strong><small v-if="memberSub(m)">{{ memberSub(m) }}</small></span>
            <span class="member-tag">已授权</span>
          </li>
        </ul>
        <div class="modal-actions"><button class="btn-cancel" @click="memberDetail = null">关闭</button></div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { api } from '@oa4rust/sdk'
import { ref } from 'vue'
import { confirmMsg, toast } from '../utils/toast'
// biome-ignore lint/correctness/noUnusedImports: Vue templates consume component imports.
import ListLoadError from './ListLoadError.vue'
import { describeListError } from './listQuery'

type Role = { flag?: string; id?: string; name?: string; roleName?: string; title?: string; desc?: string }
type Member = Record<string, unknown>
type MemberDetail = { roleName: string; members: Member[] }

const loading = ref(false)
const loadError = ref('')
const roles = ref<Role[]>([])
const memberDetail = ref<MemberDetail | null>(null)
const roleMembers = ref<Record<string, Member[]>>({})
function pick(v: unknown): string {
  for (const k of ["name", "personName", "userName", "displayName", "realName", "unique", "mobile"]) {
    const x = (v as Member)?.[k]
    if (typeof x === "string" && x.trim()) return x.trim()
  }
  return ""
}
function displayMember(m: Member): string {
  return pick(m) || "未命名成员"
}
function memberSub(m: Member): string {
  const phone = m["mobile"] ?? m["phone"]
  const unit = m["unitName"] ?? m["departmentName"] ?? m["orgName"]
  return [typeof unit === "string" ? unit : "", typeof phone === "string" ? phone : ""].filter(Boolean).join(" · ")
}
function memberInitial(m: Member): string {
  return displayMember(m).slice(0, 1).toUpperCase()
}
async function showRoleMembers(r: Role) {
  const flag = r.flag || r.id
  if (!flag) return
  try {
    // GET organization/assemble/control/person/list/role/{roleFlag} —— 角色下人员
    const resp: any = await api.get(`/api/organization/assemble/control/person/list/role/${encodeURIComponent(flag)}`)
    const list: Member[] = Array.isArray(resp.data) ? resp.data : []
    roleMembers.value = { ...roleMembers.value, [flag]: list }
    memberDetail.value = { roleName: r.name || r.roleName || r.title || flag, members: list }
  } catch (e: any) {
    toast.error(`查询成员失败: ${e?.message ?? ""}`)
  }
}
const showCreate = ref(false)
const creating = ref(false)
const editingRole = ref<Role | null>(null)
const form = ref({ name: '', flag: '', desc: '' })

async function loadRoles() {
  loading.value = true
  loadError.value = ''
  try {
    // control crate 未注册 /api/role/list，真实列表路由在 auth crate。
    const r = await api.get('/api/authentication/role/list')
    roles.value = r.data ?? []
  } catch (e) {
    // 失败保留旧数据并显式报错，不能和「真的没有角色」混成同一种空态。
    loadError.value = describeListError('角色列表加载失败', e)
  } finally {
    loading.value = false
  }
}

function editRole(r: Role) {
  editingRole.value = r
  form.value = { name: r.name || '', flag: r.flag || r.id || '', desc: r.desc || '' }
  showCreate.value = true
}

async function onSave() {
  if (!form.value.flag.trim()) { toast.info('Flag不能为空'); return }
  creating.value = true
  try {
    const payload = { name: form.value.name, description: form.value.desc }
    if (editingRole.value) await api.put(`/api/role/${form.value.flag}`, payload)
    else await api.post('/api/role', payload)
    showCreate.value = false
    editingRole.value = null
    form.value = { name: '', flag: '', desc: '' }
    loadRoles()
  } catch (e: any) {
    toast.error(`保存失败: : ${e?.message ?? "未知错误"}`)
  } finally {
    creating.value = false
  }
}

async function deleteRole(r: Role) {
  if (!(await confirmMsg(`确定删除角色「${r.name || r.flag}」？`))) return
  try {
    await api.delete(`/api/role/${r.flag || r.id}`)
    roles.value = roles.value.filter((x) => x.flag !== r.flag)
  } catch (e: any) {
    toast.error(`删除失败: : ${e?.message ?? ""}`)
  }
}

loadRoles()

const api_permission_manag_343_data = ref<any[]>([])
const api_permission_viewe_710_data = ref<any[]>([])
const api_permission_publi_309_data = ref<any[]>([])
const api_permission_manag_444_data = ref<any[]>([])
</script>

<style scoped>
.role-view { display: flex; flex-direction: column; gap: 16px; height: 100% }
.modern-page { gap: 16px }
.modern-hero { display: flex; align-items: center; gap: 10px; flex-wrap: wrap }
.toolbar-hint { flex: 1; color: var(--text-muted); font-size: 12px }
.modern-section { padding: 16px; border-radius: var(--radius-md) }
.content-panel { flex: 1; overflow-y: auto; padding: 16px; display: flex; flex-direction: column; gap: 16px }
.toolbar { display: flex; gap: 8px }
.btn-primary { padding: 8px 20px; background: var(--color-primary); color: #000; border: none; border-radius: var(--radius-md); font-size: 13px; cursor: pointer; font-weight: 600 }
.btn-create { padding: 8px 20px; background: var(--color-accent); color: #fff; border: none; border-radius: var(--radius-md); font-size: 13px; cursor: pointer; font-weight: 600 }
.list-panel { flex: 1 }
.item-table { display: flex; flex-direction: column; gap: 8px }
.table-header { display: grid; grid-template-columns: 2fr 1fr 160px; padding: 8px 12px; background: var(--bg-elevated); border-radius: var(--radius-sm); font-size: 12px; color: var(--text-muted); font-weight: 600 }
.table-row { display: grid; grid-template-columns: 2fr 1fr 160px; padding: 12px; align-items: center; transition: all var(--transition-fast); border: 1px solid var(--border-subtle); border-radius: var(--radius-md); background: var(--bg-elevated) }
.table-row:hover { border-color: var(--color-primary) }
.col-name { font-size: 14px; font-weight: 500; color: var(--text-primary) }
.col-flag { font-size: 12px; color: var(--text-secondary); display: flex; align-items: center; gap: 8px }
.flag-label { display: inline-flex; align-items: center; padding: 2px 6px; border-radius: var(--radius-sm); background: var(--color-primary-soft); color: var(--color-primary); font-size: 11px; font-weight: 600 }
.btn-edit { padding: 4px 10px; background: transparent; border: 1px solid var(--color-primary); color: var(--color-primary); border-radius: var(--radius-sm); font-size: 12px; cursor: pointer; margin-right: 6px }
.btn-edit:hover { background: var(--color-primary); color: #000 }
.btn-del { padding: 4px 10px; background: transparent; border: 1px solid var(--color-error); color: var(--color-error); border-radius: var(--radius-sm); font-size: 12px; cursor: pointer }
.btn-del:hover { background: var(--color-error); color: #fff }
.empty, .loading-row { display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 40px; color: var(--text-muted); gap: 12px; flex: 1 }
.ei { font-size: 48px; opacity: 0.4 }
.sk { height: 40px; border-radius: var(--radius-md); background: var(--bg-elevated); animation: pulse 1.2s ease-in-out infinite }
@keyframes pulse { 0%, 100% { opacity: .4 } 50% { opacity: .8 } }
.modal-overlay { position: fixed; inset: 0; background:var(--bg-overlay); display: flex; align-items: center; justify-content: center; z-index: 100 }
.modal { background: var(--bg-surface); border: 1px solid var(--border-subtle); border-radius: var(--radius-lg); padding: 24px; width: 420px; max-width: 90vw; display: flex; flex-direction: column; gap: 16px }
.modal h3 { font-family: "Orbitron", sans-serif; color: var(--color-primary); margin: 0; font-size: 15px }
.form-group { display: flex; flex-direction: column; gap: 6px }
.form-group label { font-size: 13px; color: var(--text-muted) }
.form-input, .form-textarea { background: var(--bg-elevated); border: 1px solid var(--border-subtle); border-radius: var(--radius-md); color: var(--text-primary); padding: 10px 12px; font-size: 14px }
.form-input:focus, .form-textarea:focus { outline: none; border-color: var(--color-primary) }
.form-textarea { min-height: 80px; resize: vertical }
.modal-actions { display: flex; justify-content: flex-end; gap: 8px }
.btn-cancel { padding: 8px 20px; background: transparent; border: 1px solid var(--border-subtle); color: var(--text-secondary); border-radius: var(--radius-md); cursor: pointer }
.font-mono { font-family: "JetBrains Mono", monospace }
.member-modal { width: 520px }
.member-summary { margin: 0; font-size: 13px; color: var(--text-secondary) }
.member-empty { padding: 24px; text-align: center; color: var(--text-muted); border: 1px dashed var(--border-subtle); border-radius: var(--radius-md) }
.member-list { list-style: none; margin: 0; padding: 0; max-height: 320px; overflow-y: auto; display: flex; flex-direction: column; gap: 8px }
.member-item { display: flex; align-items: center; gap: 12px; padding: 10px 12px; border-radius: var(--radius-md); background: var(--bg-elevated); border: 1px solid var(--border-subtle) }
.member-avatar { width: 32px; height: 32px; border-radius: 50%; background: var(--color-primary-soft); color: var(--color-primary); display: grid; place-items: center; font-weight: 700 }
.member-info { flex: 1; display: flex; flex-direction: column; gap: 2px; font-size: 13px }
.member-info small { color: var(--text-muted) }
.member-tag { padding: 2px 8px; border-radius: var(--radius-sm); background: var(--color-primary-soft); color: var(--color-primary); font-size: 11px }
/* R300 B5: AI 令牌统一视觉，仅覆盖样式，不改变数据绑定与接口。 */
.ai-page-b5.ai-page-b5, .ai-page-b5 .ai-page-b5 {
  background: var(--ai-gradient-surface);
  border: 1px solid var(--ai-glass-border);
  box-shadow: var(--ai-shadow-2);
  border-radius: var(--ai-radius-lg, 12px);
}
.ai-page-b5 .b5-title {
  margin: 0 0 var(--ai-grid-2, 8px);
  font-size: 18px;
  font-weight: 700;
  background: var(--ai-gradient-text);
  -webkit-background-clip: text;
  background-clip: text;
  color: transparent;
  letter-spacing: 0.02em;
}
.ai-page-b5 .glass-card {
  background: var(--ai-glass-bg);
  border-color: var(--ai-glass-border);
  backdrop-filter: var(--ai-glass-blur);
  transition: box-shadow var(--ai-motion-fast) var(--ai-ease-spring);
}
.ai-page-b5 .glass-card:hover { box-shadow: var(--ai-shadow-2); }
.ai-page-b5 .skel, .ai-page-b5 .skeleton {
  background: var(--ai-skeleton-base);
}

</style>
