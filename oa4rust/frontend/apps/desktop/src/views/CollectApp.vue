<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="crud-view">
    <AdminPageHeader title="收集管理" subtitle="收集记录（x_program_collect）" />
    <AdminToolbar
      v-model="search"
      search-placeholder="搜索标题 / 链接"
      create-label="+ 新建"
      @search="loadData"
      @refresh="loadData"
      @create="openCreate"
    >
    </AdminToolbar>
    <div class="content-panel glass-card">
      <ListLoadError v-if="items.length > 0 && loadError" mode="banner" :message="loadError" @retry="reload" />
      <div v-if="loading" class="loading-state"><div class="skel" v-for="i in 5" :key="i"></div></div>
      <ListLoadError v-else-if="items.length === 0 && loadError" mode="empty" :message="loadError" @retry="reload" />
      <div v-else-if="items.length === 0" class="empty-state"><div class="empty-icon">📐</div><p>暂无数据</p></div>
      <AdminTable v-else
        :columns="columns"
        :rows="filtered as any[]"
        :loading="loading"
        :error="loadError"
        empty-text="暂无收集记录"
        @retry="reload"
      >
      <template #cell-title="{ row }">
        {{ row.title||'—' }}
      </template>
      <template #cell-url="{ row }">
        {{ row.url||'—' }}
      </template>
      <template #cell-personId="{ row }">
        {{ row.personId||'—' }}
      </template>
      <template #cell-createTime="{ row }">
        {{ fmtTime(row.createTime) }}
      </template>
      <template #cell-actions="{ row: item }">
        <button class="btn-sm" @click="editItem(item as any)">编辑</button>
          <button class="btn-sm btn-del" :disabled="deletingId === item.id" @click="deleteItem(item as any)">删除</button>
      </template>
    </AdminTable>
    </div>
<div v-if="showCreate||showEdit" class="modal-overlay" @click.self="closeModal">
      <div class="modal glass-card">
        <h3>{{ showEdit?'编辑':'新建' }}收集记录</h3>
        <div class="form-group"><label>成员 ID</label><input v-model="form.personId" placeholder="personId" class="form-input mono" /></div>
        <div class="form-group"><label>标题</label><input v-model="form.title" placeholder="标题" class="form-input" /></div>
        <div class="form-group"><label>链接</label><input v-model="form.url" placeholder="https://..." class="form-input mono" /></div>
        <div class="modal-actions">
          <button class="btn-cancel" @click="closeModal">取消</button>
          <button class="btn-save" :disabled="!form.title?.trim()||saving" @click="saveItem">{{ saving?'保存中…':'保存' }}</button>
        </div>
      </div>
    </div>
  </div></template>
<script setup lang="ts">
const columns: AdminTableColumn[] = [
  { key: 'title', label: '标题' },
  { key: 'url', label: '链接' },
  { key: 'personId', label: '成员' },
  { key: 'createTime', label: '创建时间' },
  { key: 'actions', label: '操作' },
]

import { api, useSession } from '@oa4rust/sdk'
import { useMutation, useQuery, useQueryClient } from '@tanstack/vue-query'
import { computed, ref } from 'vue'
import { AdminPageHeader, AdminTable, type AdminTableColumn, AdminToolbar } from '../components/admin/'
import { formatServerDateTime } from '../utils/date'
import { confirmMsg, toast } from '../utils/toast'
import { collectMutationErrorMessage } from './collectError'
// biome-ignore lint/correctness/noUnusedImports: Vue templates consume component imports.
import ListLoadError from './ListLoadError.vue'
import { useListError } from './listQuery'
import { liveQueryItems } from './liveQueryItems'

// 模板专用组件引用（避免 lint 误报未使用）
void AdminPageHeader
void AdminTable
void AdminToolbar

interface Item {
  id: string
  personId?: string
  title?: string
  url?: string
  creator?: string
  createTime?: string
}
interface CollectForm {
  personId?: string
  title?: string
  url?: string
}

const listEp = '/api/program_center/collect/list'
const createEp = '/api/program_center/collect/create'
const qk = ['Collect', 'list']

const search = ref(''),
  showCreate = ref(false),
  showEdit = ref(false),
  loading = ref(false),
  saving = ref(false)
const items = liveQueryItems<Item>(() => data.value),
  form = ref<CollectForm>({}),
  editingId = ref<string | null>(null)
const qc = useQueryClient()
const session = useSession()

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
  label: '收集管理加载失败',
})

const filtered = computed(() =>
  search.value
    ? items.value.filter(
        (i) =>
          (i.title || '').toLowerCase().includes(search.value.toLowerCase()) ||
          (i.url || '').toLowerCase().includes(search.value.toLowerCase()),
      )
    : items.value,
)

function openCreate() {
  // 新建默认归属当前登录人（此前需手填 person UUID，几乎不可用）。
  form.value = { personId: session.user?.unique ?? '', title: '', url: '' }
  editingId.value = null
  showCreate.value = true
}
function editItem(item: Item) {
  form.value = { personId: item.personId ?? '', title: item.title ?? '', url: item.url ?? '' }
  editingId.value = item.id
  showEdit.value = true
}
function closeModal() {
  showCreate.value = false
  showEdit.value = false
  form.value = {}
}
const saveM = useMutation({
  mutationFn: async () => {
    const payload: Record<string, string> = {
      personId: form.value.personId ?? '',
      title: form.value.title ?? '',
      url: form.value.url ?? '',
    }
    if (editingId.value) return api.post(`/api/program_center/collect/save/${editingId.value}`, payload)
    return api.post(createEp, payload)
  },
  onSuccess: () => {
    qc.invalidateQueries({ queryKey: qk })
    closeModal()
  },
  onError: (e) => toast.error(collectMutationErrorMessage(e)),
  onSettled: () => (saving.value = false),
})
function saveItem() {
  if (saving.value) return
  saving.value = true
  saveM.mutate()
}
const deletingId = ref('')
const delM = useMutation({
  mutationFn: async (id: string) => api.post(`/api/program_center/collect/delete/${id}`),
  onSuccess: () => {
    qc.invalidateQueries({ queryKey: qk })
  },
  onError: (e) => toast.error(collectMutationErrorMessage(e)),
})
async function deleteItem(item: Item) {
  if (deletingId.value) return
  if (!(await confirmMsg('确定删除该收集记录？'))) return
  deletingId.value = item.id
  delM.mutate(item.id, { onSettled: () => (deletingId.value = '') })
}
function loadData() {
  reload()
}
function fmtTime(t?: string) {
  return t ? formatServerDateTime(t) : ''
}
// __TEMPLATE_IMPORT_REFS__
void AdminPageHeader
void AdminTable
void AdminToolbar
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
.url-cell{max-width:260px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.mono{font-family:'Fira Code',monospace;font-size:12px;color:var(--color-secondary)}
.btn-sm{padding:4px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);cursor:pointer;font-size:12px}
.btn-del{border-color:var(--color-danger);color:var(--color-danger)}
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
