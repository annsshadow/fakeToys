<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="crud-view ai-page-b5" data-testid="three-member-page">
    <div class="view-header glass-card ai-page-b5">
      <p class="ai-gradient-text b5-title b5-subtitle" aria-label="三方成员管理">三方成员管理</p>
      <button class="btn-primary" @click="openCreate">+ 新建</button>
    </div>
    <div class="content-panel glass-card">
      <ListLoadError v-if="items.length > 0 && loadError" mode="banner" :message="loadError" @retry="reload" />
      <div class="toolbar">
        <input v-model="search" placeholder="搜索姓名 / 手机 / 单位..." class="search-input" />
        <button class="btn-refresh" @click="loadData">🔄 刷新</button>
      </div>
      <div v-if="loading" class="loading-state"><div class="skel" v-for="i in 5" :key="i"></div></div>
      <ListLoadError v-else-if="items.length === 0 && loadError" mode="empty" :message="loadError" @retry="reload" />
      <div v-else-if="items.length === 0" class="empty-state"><div class="empty-icon">👥</div><p>暂无三方成员，点击下方「新建」添加第一位成员。</p><button class="btn-primary" data-testid="three-empty-create" @click="openCreate">+ 新建成员</button></div>
      <table v-else class="data-table">
        <thead><tr><th>姓名</th><th>手机</th><th>邮箱</th><th>单位</th><th>状态</th><th>操作</th></tr></thead>
        <tbody>
          <tr v-for="item in filtered" :key="item.id">
            <td>{{ item.name||'—' }}</td>
            <td class="mono">{{ item.mobile||'—' }}</td>
            <td>{{ item.email||'—' }}</td>
            <td data-testid="three-unit-cell">{{ unitLabel(item) }}</td>
            <td><span class="status-badge" :class="statusClass(item)" data-testid="three-status-badge">{{ statusLabel(item) }}</span></td>
            <td>
              <button class="btn-sm" @click="editItem(item)">编辑</button>
              <button class="btn-sm btn-del" :disabled="deletingId === item.id" @click="deleteItem(item)">删除</button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
    <div v-if="showCreate||showEdit" class="modal-overlay" @click.self="closeModal">
      <div class="modal glass-card">
        <h3>{{ showEdit?'编辑':'新建' }}三方成员</h3>
        <div class="form-group"><label>姓名<span class="required" aria-hidden="true">*</span></label><input v-model="form.name" placeholder="姓名" class="form-input" required /></div>
        <div class="form-group"><label>手机</label><input v-model="form.mobile" placeholder="手机号" class="form-input mono" /></div>
        <div class="form-group"><label>邮箱</label><input v-model="form.email" placeholder="邮箱" class="form-input mono" /></div>
        <div class="form-group"><label>所属单位</label><input v-model="form.unitId" data-testid="three-unit-input" placeholder="单位名称或编号" class="form-input" /></div>
        <div class="form-group"><label>状态</label><select v-model="form.status" data-testid="three-status-select" class="form-input"><option v-for="opt in statusOptions" :key="opt.value" :value="opt.value">{{ opt.label }}</option></select></div>
        <div class="modal-actions">
          <button class="btn-cancel" @click="closeModal">取消</button>
          <button class="btn-save" :disabled="!form.name?.trim()||saving" @click="saveItem">{{ saving?'保存中…':'保存' }}</button>
        </div>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { api } from '@oa4rust/sdk'
import { useMutation, useQuery, useQueryClient } from '@tanstack/vue-query'
import { computed, ref } from 'vue'
import { confirmMsg, toast } from '../utils/toast'
// biome-ignore lint/correctness/noUnusedImports: Vue templates consume component imports.
import ListLoadError from './ListLoadError.vue'
import { useListError } from './listQuery'
import { liveQueryItems } from './liveQueryItems'
import { mutationErrorMessage } from './mutationError'

interface Item {
  id: string
  name?: string
  mobile?: string
  email?: string
  unitId?: string
  status?: string
  statusDes?: string
  createTime?: string
}
interface MemberForm {
  name?: string
  mobile?: string
  email?: string
  unitId?: string
  status?: string
}

const statusOptions = [
  { value: 'active', label: '已启用' },
  { value: 'disabled', label: '已停用' },
  { value: 'pending', label: '待审核' },
]
const statusLabels: Record<string, string> = {
  active: '已启用', disabled: '已停用', pending: '待审核',
  normal: '正常', enabled: '已启用', valid: '有效',
  inactive: '已停用', cancel: '已停用', invalid: '已失效',
}
function statusLabel(item: Item): string {
  const raw = (item.statusDes || item.status || '').trim()
  if (!raw) return '—'
  return statusLabels[raw] || statusLabels[raw.toLowerCase()] || raw
}
function statusClass(item: Item): string {
  const raw = (item.status || '').toLowerCase()
  if (raw === 'active' || raw === 'normal' || raw === 'enabled') return 'status-active'
  if (raw === 'disabled' || raw === 'inactive' || raw === 'cancel') return 'status-disabled'
  return 'status-pending'
}
function unitLabel(item: Item): string {
  return (item as any).unitName || (item as any).departmentName || item.unitId || '—'
}

const listEp = '/api/organization/assemble/control/threemember/list'
const createEp = '/api/organization/assemble/control/threemember/create'
const qk = ['ThreeMember', 'list']

const search = ref(''),
  showCreate = ref(false),
  showEdit = ref(false),
  loading = ref(false),
  saving = ref(false)
const items = liveQueryItems<Item>(() => data.value),
  form = ref<MemberForm>({}),
  editingId = ref<string | null>(null)
const qc = useQueryClient()

const { data, isError, error } = useQuery({
  queryKey: qk,
  queryFn: async () => {
    loading.value = true
    try {
      const r = (await api.get(listEp)) as unknown as { data?: unknown }
      return Array.isArray(r?.data) ? (r.data as Item[]) : []
    } finally {
      loading.value = false
    }
  },
})
const { loadError, reload } = useListError({
  isError,
  error,
  queryKey: qk,
  label: '三方成员管理加载失败',
})

const filtered = computed(() =>
  search.value
    ? items.value.filter(
        (i) =>
          (i.name || '').toLowerCase().includes(search.value.toLowerCase()) ||
          (i.mobile || '').toLowerCase().includes(search.value.toLowerCase()) ||
          (i.unitId || '').toLowerCase().includes(search.value.toLowerCase()),
      )
    : items.value,
)

function openCreate() {
  form.value = { name: '', mobile: '', email: '', unitId: '', status: 'active' }
  editingId.value = null
  showCreate.value = true
}
function editItem(item: Item) {
  form.value = {
    name: item.name ?? '',
    mobile: item.mobile ?? '',
    email: item.email ?? '',
    unitId: item.unitId ?? '',
    status: item.status ?? '',
  }
  editingId.value = item.id
  showEdit.value = true
}
function closeModal() {
  showCreate.value = false
  showEdit.value = false
  form.value = {}
}
const saveM = useMutation({
  onError: (e) => toast.error(mutationErrorMessage(e)),
  mutationFn: async () => {
    const payload: Record<string, string> = {
      name: form.value.name ?? '',
      mobile: form.value.mobile ?? '',
      email: form.value.email ?? '',
      unitId: form.value.unitId ?? '',
      status: form.value.status ?? '',
    }
    if (editingId.value)
      return api.post(`/api/organization/assemble/control/threemember/save/${editingId.value}`, payload)
    return api.post(createEp, payload)
  },
  onSuccess: () => {
    qc.invalidateQueries({ queryKey: qk })
    closeModal()
  },
  onSettled: () => (saving.value = false),
})
function saveItem() {
  if (saving.value) return
  saving.value = true
  saveM.mutate()
}
const deletingId = ref('')
const delM = useMutation({
  onError: (e) => toast.error(mutationErrorMessage(e)),
  mutationFn: async (id: string) => api.post(`/api/organization/assemble/control/threemember/delete/${id}`),
  onSuccess: () => {
    qc.invalidateQueries({ queryKey: qk })
  },
})
async function deleteItem(item: Item) {
  if (deletingId.value) return
  if (!(await confirmMsg('确定删除该三方成员？'))) return
  deletingId.value = item.id
  delM.mutate(item.id, { onSettled: () => (deletingId.value = '') })
}
function loadData() {
  reload()
}
</script>
<style scoped>
.crud-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{display:flex;align-items:flex-start;justify-content:space-between;padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0}
.btn-primary{padding:8px 16px;border-radius:var(--radius-md);border:none;background:var(--color-primary);color:white;cursor:pointer;font-weight:600}
.content-panel{padding:16px}
.toolbar{display:flex;gap:8px;margin-bottom:16px}
.search-input{flex:1;padding:8px 12px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);outline:none}
.btn-refresh{padding:8px 12px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);cursor:pointer}
.data-table{width:100%;border-collapse:collapse}
.data-table th,.data-table td{padding:10px 12px;text-align:left;border-bottom:1px solid var(--border-color)}
.data-table th{color:var(--text-muted);font-weight:600;font-size:12px;text-transform:uppercase}
.data-table tr:hover{background:var(--bg-hover)}
.mono{font-family:'Fira Code',monospace;font-size:12px;color:var(--color-secondary)}
.btn-sm{padding:4px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);cursor:pointer;font-size:12px}
.btn-del{border-color:var(--color-danger);color:var(--color-danger)}.btn-del:disabled{opacity:.5;cursor:not-allowed}
.loading-state,.empty-state{padding:40px;text-align:center;color:var(--text-muted)}
.empty-icon{font-size:32px;margin-bottom:8px}
.status-badge{display:inline-flex;align-items:center;padding:2px 8px;border-radius:999px;font-size:12px;font-weight:600;background:var(--bg-elevated);color:var(--text-secondary)}
.status-active{background:var(--color-primary-soft);color:var(--color-primary)}
.status-disabled{background:var(--bg-elevated);color:var(--text-muted)}
.status-pending{background:var(--color-primary-soft);color:var(--color-primary)}
.modal-overlay{position:fixed;inset:0;background:var(--bg-overlay);display:flex;align-items:center;justify-content:center;z-index:100}
.modal{padding:24px;width:460px;max-width:90vw}
.modal h3{font-size:16px;color:var(--color-primary);margin:0 0 16px}
.form-group{margin-bottom:12px}
.form-group label{display:block;font-size:12px;color:var(--text-muted);margin-bottom:4px}
.required{color:var(--color-error)}
.form-input{width:100%;padding:8px 12px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);outline:none;box-sizing:border-box}
.modal-actions{display:flex;justify-content:flex-end;gap:8px;margin-top:16px}
.btn-cancel{padding:8px 16px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:transparent;color:var(--text-primary);cursor:pointer}
.btn-save{padding:8px 16px;border-radius:var(--radius-md);border:none;background:var(--color-primary);color:white;cursor:pointer;font-weight:600}
.btn-save:disabled{opacity:0.5;cursor:not-allowed}
.skel{height:16px;background:var(--bg-elevated);border-radius:4px;margin-bottom:8px;animation:pulse 1.5s infinite}
@keyframes pulse{0%,100%{opacity:1}50%{opacity:0.4}}
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