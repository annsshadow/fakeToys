<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="crud-view modern-page ai-page-b5 ai-panel">
    <AdminPageHeader title="CMS 栏目管理" subtitle="实体栏目（x_cms_entity_column）" />
    <AdminToolbar
      v-model="search"
      search-placeholder="搜索栏目名称"
      create-label="+ 新建栏目"
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
        empty-text="暂无栏目，点击“新建栏目”创建"
        @retry="reload"
      >
      <template #cell-name="{ row }">
        {{ row.name || '—' }}
      </template>
      <template #cell-parentId="{ row }">
        {{ row.parentId || '（顶级）' }}
      </template>
      <template #cell-sortOrder="{ row }">
        {{ row.sortOrder ?? '—' }}
      </template>
      <template #cell-status="{ row }">
        {{ row.status || 'active' }}
      </template>
      <template #cell-updateTime="{ row }">
        {{ fmtTime(row.updateTime || row.createTime) }}
      </template>
      <template #cell-actions="{ row: item }">
        <button class="btn-sm" @click="editItem(item as any)">编辑</button>
          <button class="btn-sm btn-del" :disabled="deletingId === item.id" @click="deleteItem(item as any)">删除</button>
      </template>
    </AdminTable>
    </div>
<div v-if="showCreate || showEdit" class="modal-overlay" @click.self="closeModal">
      <div class="modal glass-card">
        <h3>{{ showEdit ? '编辑栏目' : '新建栏目' }}</h3>
        <div class="form-group"><label>名称</label><input v-model="form.name" placeholder="栏目名称" class="form-input" /></div>
        <div class="form-group"><label>上级栏目 ID（可选）</label><input v-model="form.parentId" placeholder="parent_id" class="form-input mono" /></div>
        <div class="form-group"><label>排序</label><input v-model="form.sortOrder" type="number" placeholder="sort_order" class="form-input" /></div>
        <div class="form-group"><label>描述</label><textarea v-model="form.description" rows="3" placeholder="描述" class="form-textarea"></textarea></div>
        <div class="modal-actions">
          <button class="btn-cancel" @click="closeModal">取消</button>
          <button class="btn-save" :disabled="!form.name?.trim() || saving" @click="saveItem">{{ saving ? '保存中…' : '保存' }}</button>
        </div>
      </div>
    </div>
  </div></template>
<script setup lang="ts">
const columns: AdminTableColumn[] = [
  { key: 'name', label: '名称' },
  { key: 'parentId', label: '上级' },
  { key: 'sortOrder', label: '排序' },
  { key: 'status', label: '状态' },
  { key: 'updateTime', label: '更新时间' },
  { key: 'actions', label: '操作' },
]

import { api } from '@oa4rust/sdk'
import { useMutation, useQuery, useQueryClient } from '@tanstack/vue-query'
import { computed, ref } from 'vue'
import { AdminPageHeader, AdminTable, type AdminTableColumn, AdminToolbar } from '../components/admin/'
import { formatServerDateTime } from '../utils/date'
import { confirmMsg, toast } from '../utils/toast'
// biome-ignore lint/correctness/noUnusedImports: Vue templates consume component imports.
import ListLoadError from './ListLoadError.vue'
import { useListError } from './listQuery'
import { liveQueryItems } from './liveQueryItems'
import { mutationErrorMessage } from './mutationError'

// 模板专用组件引用（避免 lint 误报未使用）
void AdminPageHeader
void AdminTable
void AdminToolbar

interface Item {
  id: string
  name?: string
  parentId?: string
  sortOrder?: number
  status?: string
  description?: string
  updateTime?: string
  createTime?: string
}

const listEp = '/api/cms/core/entity/column/list'
const createEp = '/api/cms/core/entity/column/create'
const qk = ['cms_Column', 'list']

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
      const rows = Array.isArray(r?.data) ? (r.data as Item[]) : []
      return rows
    } finally {
      loading.value = false
    }
  },
})
const { loadError, reload } = useListError({
  isError,
  error,
  queryKey: qk,
  label: 'CMS 栏目管理加载失败',
})

const filtered = computed(() =>
  search.value
    ? items.value.filter((i) => (i.name || '').toLowerCase().includes(search.value.toLowerCase()))
    : items.value,
)

function openCreate() {
  form.value = { name: '', parentId: '', sortOrder: 0, status: 'active', description: '' }
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
    if (editingId.value) return api.put(`/api/cms/core/entity/column/save/${editingId.value}`, payload)
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
  mutationFn: async (id: string) => api.post(`/api/cms/core/entity/column/delete/${id}`),
  onSuccess: () => {
    qc.invalidateQueries({ queryKey: qk })
  },
})
async function deleteItem(item: Item) {
  if (deletingId.value) return
  if (!(await confirmMsg('确定删除该栏目？'))) return
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
.mono{font-family:'Fira Code',monospace;font-size:12px;color:var(--color-secondary)}
.btn-sm{padding:4px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);cursor:pointer;font-size:12px}
.btn-del{border-color:var(--color-danger);color:var(--color-danger)}.btn-del:disabled{opacity:.5;cursor:not-allowed}
.btn-del:hover{background:var(--color-danger-soft)}
.loading-state,.empty-state{padding:40px;text-align:center;color:var(--text-muted)}
.empty-icon{font-size:32px;margin-bottom:8px}
.modal-overlay{position:fixed;inset:0;background:var(--bg-overlay);display:flex;align-items:center;justify-content:center;z-index:100}
.modal{padding:24px;width:480px;max-width:90vw}
.modal h3{font-size:16px;color:var(--color-primary);margin:0 0 16px}
.form-group{margin-bottom:12px}
.form-group label{display:block;font-size:12px;color:var(--text-muted);margin-bottom:4px}
.form-input,.form-textarea{width:100%;padding:8px 12px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);outline:none;box-sizing:border-box}
.modal-actions{display:flex;justify-content:flex-end;gap:8px;margin-top:16px}
.btn-cancel{padding:8px 16px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:transparent;color:var(--text-primary);cursor:pointer}
.btn-save{padding:8px 16px;border-radius:var(--radius-md);border:none;background:var(--color-primary);color:white;cursor:pointer;font-weight:600}
.btn-save:disabled{opacity:0.5;cursor:not-allowed}
.skel{height:16px;background:var(--bg-elevated);border-radius:4px;margin-bottom:8px;animation:pulse 1.5s infinite}
@keyframes pulse{0%,100%{opacity:1}50%{opacity:0.4}}
</style>
