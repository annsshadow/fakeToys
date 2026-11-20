<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="crud-view modern-page ai-panel ai-page-b5">
    <div class="view-header glass-card modern-hero ai-panel">
      <div class="hero-copy">
        <h2 class="ai-gradient-text" data-testid="deployment-page-title">部署管理</h2>
        <h2 class="subtitle">维护应用部署记录与运行状态</h2>
      </div>
      <button class="btn-primary" data-testid="deployment-create" @click="openCreate">+ 新建</button>
    </div>
    <div class="content-panel glass-card">
      <ListLoadError v-if="items.length > 0 && loadError" mode="banner" :message="loadError" @retry="reload" />
      <div class="toolbar">
        <input v-model="search" placeholder="搜索应用 / 分类..." class="search-input" />
        <button class="btn-refresh" @click="loadData">🔄 刷新</button>
      </div>
      <div v-if="loading" class="loading-state"><div class="skel" v-for="i in 5" :key="i"></div></div>
      <ListLoadError v-else-if="items.length === 0 && loadError" mode="empty" :message="loadError" @retry="reload" />
      <div v-else-if="items.length === 0" class="empty-state"><div class="empty-icon">🚀</div><p>暂无部署记录</p></div>
      <table v-else class="data-table">
        <thead><tr><th>应用 ID</th><th>应用名</th><th>程序</th><th>分类</th><th>禁用</th><th>创建时间</th><th>操作</th></tr></thead>
        <tbody>
          <tr v-for="item in filtered" :key="item.id">
            <td>{{ item.appId||'—' }}</td>
            <td>{{ item.appName||'—' }}</td>
            <td>{{ item.application||'—' }}</td>
            <td>{{ item.category||'—' }}</td>
            <td>{{ item.disable==='true'?'是':'否' }}</td>
            <td>{{ fmtTime(item.createTime) }}</td>
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
        <h3>{{ showEdit?'编辑':'新建' }}部署记录</h3>
        <div class="form-group"><label>应用 ID</label><input v-model="form.appId" placeholder="appId" class="form-input mono" /></div>
        <div class="form-group"><label>应用名</label><input v-model="form.appName" placeholder="应用名称" class="form-input" /></div>
        <div class="form-group"><label>程序</label><input v-model="form.application" placeholder="如 o2server / oa4rust" class="form-input mono" /></div>
        <div class="form-group"><label>分类</label><input v-model="form.category" placeholder="如 backend / frontend" class="form-input mono" /></div>
        <div class="form-group"><label>禁用</label><input v-model="form.disable" placeholder="true / false" class="form-input mono" /></div>
        <div class="modal-actions">
          <button class="btn-cancel" @click="closeModal">取消</button>
          <button class="btn-save" :disabled="!form.appId?.trim()||saving" @click="saveItem">{{ saving?'保存中…':'保存' }}</button>
        </div>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { api } from '@oa4rust/sdk'
import { useMutation, useQuery, useQueryClient } from '@tanstack/vue-query'
import { computed, ref } from 'vue'
import { formatServerDateTime } from '../utils/date'
import { confirmMsg, toast } from '../utils/toast'
// biome-ignore lint/correctness/noUnusedImports: Vue templates consume component imports.
import ListLoadError from './ListLoadError.vue'
import { useListError } from './listQuery'
import { liveQueryItems } from './liveQueryItems'
import { mutationErrorMessage } from './mutationError'

interface Item {
  id: string
  appId?: string
  appName?: string
  application?: string
  category?: string
  disable?: string
  createTime?: string
}

const listEp = '/api/server/deploy/list'
const createEp = '/api/server/deploy/create'
const qk = ['Deployment', 'list']

const search = ref(''),
  showCreate = ref(false),
  showEdit = ref(false),
  loading = ref(false),
  saving = ref(false)
const items = liveQueryItems<Item>(() => data.value),
  form = ref<Partial<Item>>({}),
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
  label: '部署管理加载失败',
})

const filtered = computed(() =>
  search.value
    ? items.value.filter(
        (i) =>
          (i.appId || '').toLowerCase().includes(search.value.toLowerCase()) ||
          (i.appName || '').toLowerCase().includes(search.value.toLowerCase()),
      )
    : items.value,
)

function openCreate() {
  form.value = { appId: '', appName: '', application: '', category: '', disable: 'false' }
  editingId.value = null
  showCreate.value = true
}
function editItem(item: Item) {
  form.value = { ...item }
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
  mutationFn: async (payload: Item) => {
    if (editingId.value) return api.post(`/api/server/deploy/save/${editingId.value}`, payload)
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
  saveM.mutate(form.value as Item)
}
const deletingId = ref('')
const delM = useMutation({
  onError: (e) => toast.error(mutationErrorMessage(e)),
  mutationFn: async (id: string) => api.post(`/api/server/deploy/delete/${id}`),
  onSuccess: () => {
    qc.invalidateQueries({ queryKey: qk })
  },
})
async function deleteItem(item: Item) {
  if (deletingId.value) return
  if (!(await confirmMsg('确定删除该部署记录？'))) return
  deletingId.value = item.id
  delM.mutate(item.id, { onSettled: () => (deletingId.value = '') })
}
function loadData() {
  reload()
}
function fmtTime(t?: string) {
  return t ? formatServerDateTime(t) : ''
}
</script>
<style scoped>
.crud-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{display:flex;align-items:flex-start;justify-content:space-between;padding:16px 24px}
.view-header h2.ai-gradient-text{margin:0 0 4px;background:var(--ai-gradient-text);-webkit-background-clip:text;background-clip:text;color:transparent;font-size:20px;font-weight:700}
.view-header h2{margin:0;font-size:12px;font-weight:500;color:var(--text-muted)}
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
.modal-overlay{position:fixed;inset:0;background:var(--bg-overlay);display:flex;align-items:center;justify-content:center;z-index:100}
.modal{padding:24px;width:460px;max-width:90vw}
.modal h3{font-size:16px;color:var(--color-primary);margin:0 0 16px}
.form-group{margin-bottom:12px}
.form-group label{display:block;font-size:12px;color:var(--text-muted);margin-bottom:4px}
.form-input{width:100%;padding:8px 12px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);outline:none;box-sizing:border-box}
.modal-actions{display:flex;justify-content:flex-end;gap:8px;margin-top:16px}
.btn-cancel{padding:8px 16px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:transparent;color:var(--text-primary);cursor:pointer}
.btn-save{padding:8px 16px;border-radius:var(--radius-md);border:none;background:var(--color-primary);color:white;cursor:pointer;font-weight:600}
.btn-save:disabled{opacity:0.5;cursor:not-allowed}
.skel{height:16px;background:var(--bg-elevated);border-radius:4px;margin-bottom:8px;animation:pulse 1.5s infinite}
@keyframes pulse{0%,100%{opacity:1}50%{opacity:0.4}}
</style>