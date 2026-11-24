<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="crud-view ai-page-b5" data-testid="cms-dict-page">
    <AdminPageHeader title="CMS 字典设计器" subtitle="设计字典（surface appdict）" />
    <p class="ai-gradient-text b5-title b5-subtitle">CMS 字典设计器</p>
    <AdminToolbar
      v-model="search"
      search-placeholder="搜索应用 / 字典标识 / 值"
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
        empty-text="暂无字典项"
        @retry="reload"
      >
      <template #cell-app_info_flag="{ row }">
        {{ row.app_info_flag||'—' }}
      </template>
      <template #cell-app_dict_flag="{ row }">
        {{ row.app_dict_flag||'—' }}
      </template>
      <template #cell-data_value="{ row }">
        {{ row.data_value||'—' }}
      </template>
      <template #cell-actions="{ row: item }">
        <button class="btn-sm" @click="editItem(item as any)">编辑</button>
          <button class="btn-sm btn-del" :disabled="deletingId === item.id" @click="deleteItem(item as any)">删除</button>
      </template>
    </AdminTable>
    </div>
<div v-if="showCreate||showEdit" class="modal-overlay" @click.self="closeModal">
      <div class="modal glass-card">
        <h3>{{ showEdit?'编辑':'新建' }}字典项</h3>
        <div class="form-group"><label>应用标识</label><input v-model="form.app_info_flag" placeholder="app_info_flag" class="form-input mono" /></div>
        <div class="form-group"><label>字典标识</label><input v-model="form.app_dict_flag" placeholder="app_dict_flag" class="form-input mono" /></div>
        <div class="form-group"><label>数据值</label><textarea v-model="form.data_value" rows="4" placeholder="字典数据（可为 JSON）" class="form-textarea mono"></textarea></div>
        <div class="modal-actions">
          <button class="btn-cancel" @click="closeModal">取消</button>
          <button class="btn-save" :disabled="!form.app_dict_flag?.trim()||saving" @click="saveItem">{{ saving?'保存中…':'保存' }}</button>
        </div>
      </div>
    </div>
  </div></template>
<script setup lang="ts">
const columns: AdminTableColumn[] = [
  { key: 'app_info_flag', label: '应用标识' },
  { key: 'app_dict_flag', label: '字典标识' },
  { key: 'data_value', label: '数据值' },
  { key: 'actions', label: '操作' },
]

import { api } from '@oa4rust/sdk'
import { useMutation, useQuery, useQueryClient } from '@tanstack/vue-query'
import { computed, ref } from 'vue'
import { AdminPageHeader, AdminTable, type AdminTableColumn, AdminToolbar } from '../components/admin/'
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
  app_info_flag?: string
  app_dict_flag?: string
  data_value?: string
}

const listEp = '/api/cms/assemble/control/dict/list'
const createEp = '/api/cms/assemble/control/dict/create'
const qk = ['CmsDictDesigner', 'list']

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
  label: 'CMS字典设计器加载失败',
})

const filtered = computed(() =>
  search.value
    ? items.value.filter(
        (i) =>
          (i.app_info_flag || '').toLowerCase().includes(search.value.toLowerCase()) ||
          (i.app_dict_flag || '').toLowerCase().includes(search.value.toLowerCase()) ||
          (i.data_value || '').toLowerCase().includes(search.value.toLowerCase()),
      )
    : items.value,
)

function openCreate() {
  form.value = { app_info_flag: '', app_dict_flag: '', data_value: '' }
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
    if (editingId.value) return api.post(`/api/cms/assemble/control/dict/save/${editingId.value}`, payload)
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
  mutationFn: async (id: string) => api.post(`/api/cms/assemble/control/dict/delete/${id}`),
  onSuccess: () => {
    qc.invalidateQueries({ queryKey: qk })
  },
})
async function deleteItem(item: Item) {
  if (deletingId.value) return
  if (!(await confirmMsg('确定删除该字典项？'))) return
  deletingId.value = item.id
  delM.mutate(item.id, { onSettled: () => (deletingId.value = '') })
}
function loadData() {
  reload()
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
.val-cell{max-width:280px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.mono{font-family:'Fira Code',monospace;font-size:12px;color:var(--color-secondary)}
.btn-sm{padding:4px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);cursor:pointer;font-size:12px}
.btn-del{border-color:var(--color-danger);color:var(--color-danger)}.btn-del:disabled{opacity:.5;cursor:not-allowed}
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
