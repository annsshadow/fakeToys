<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="crud-view">
    <div class="view-header glass-card">
      <div>
        <h1>流程应用管理</h1>
        <p class="subtitle">/api/program_center/application/*（x_applications）</p>
      </div>
      <button class="btn-primary" @click="openCreate">+ 新建</button>
    </div>
    <div class="content-panel glass-card">
      <div class="toolbar">
        <input v-model="search" placeholder="搜索名称 / 应用 ID..." class="search-input" />
        <button class="btn-refresh" @click="loadData">🔄 刷新</button>
        <button class="btn-refresh" @click="loadCategories">📁 分类</button>
      </div>
      <div v-if="categories.length" class="cat-chips">
        <span v-for="c in categories" :key="c.id || c.name" class="cat-chip">{{ c.name || c.id }}</span>
      </div>
      <div v-if="loading" class="loading-state"><div class="skel" v-for="i in 5" :key="i"></div></div>
      <div v-else-if="items.length===0" class="empty-state"><div class="empty-icon">🧩</div><p>暂无应用</p></div>
      <table v-else class="data-table">
        <thead><tr><th>名称</th><th>应用 ID</th><th>描述</th><th>禁用</th><th>创建时间</th><th>操作</th></tr></thead>
        <tbody>
          <tr v-for="item in filtered" :key="item.id">
            <td>{{ item.name||'—' }}</td>
            <td class="mono">{{ item.appId||'—' }}</td>
            <td class="desc-cell">{{ item.description||'—' }}</td>
            <td>{{ item.disable?'是':'否' }}</td>
            <td>{{ fmtTime(item.createTime) }}</td>
            <td>
              <button class="btn-sm" @click="editItem(item)">编辑</button>
              <button class="btn-sm btn-del" @click="deleteItem(item)">删除</button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
    <div v-if="showCreate||showEdit" class="modal-overlay" @click.self="closeModal">
      <div class="modal glass-card">
        <h3>{{ showEdit?'编辑':'新建' }}应用</h3>
        <div class="form-group"><label>名称</label><input v-model="form.name" placeholder="应用名称" class="form-input" /></div>
        <div class="form-group"><label>应用 ID</label><input v-model="form.appId" placeholder="appId" class="form-input mono" /></div>
        <div class="form-group"><label>描述</label><textarea v-model="form.description" rows="3" placeholder="描述" class="form-textarea"></textarea></div>
        <div class="form-group" v-if="showEdit">
          <label><input type="checkbox" v-model="form.disable" /> 禁用</label>
        </div>
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

interface Item {
  id: string
  name?: string
  appId?: string
  description?: string
  disable?: boolean
  creator?: string
  createTime?: string
}
interface AppForm {
  name?: string
  appId?: string
  description?: string
  disable?: boolean
}

const listEp = '/api/program_center/application/list'
const createEp = '/api/program_center/application/create'
const qk = ['ProcessApplication', 'list']

const search = ref(''),
  showCreate = ref(false),
  showEdit = ref(false),
  loading = ref(false),
  saving = ref(false)
const items = ref<Item[]>([]),
  form = ref<AppForm>({}),
  editingId = ref<string | null>(null)
const qc = useQueryClient()

const { data } = useQuery({
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
items.value = Array.isArray(data.value) ? (data.value as Item[]) : []

const filtered = computed(() =>
  search.value
    ? items.value.filter(
        (i) =>
          (i.name || '').toLowerCase().includes(search.value.toLowerCase()) ||
          (i.appId || '').toLowerCase().includes(search.value.toLowerCase()),
      )
    : items.value,
)

function openCreate() {
  form.value = { name: '', appId: '', description: '' }
  editingId.value = null
  showCreate.value = true
}
function editItem(item: Item) {
  form.value = {
    name: item.name ?? '',
    appId: item.appId ?? '',
    description: item.description ?? '',
    disable: Boolean(item.disable),
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
  mutationFn: async () => {
    saving.value = true
    try {
      // 后端 typed request 用 snake_case app_id（无 serde rename），payload 按此口径发送。
      const payload: Record<string, unknown> = {
        name: form.value.name ?? '',
        app_id: form.value.appId ?? '',
        description: form.value.description ?? '',
      }
      if (editingId.value) {
        payload.disable = Boolean(form.value.disable)
        return api.post(`/api/program_center/application/save/${editingId.value}`, payload)
      }
      return api.post(createEp, payload)
    } finally {
      saving.value = false
    }
  },
  onSuccess: () => {
    qc.invalidateQueries({ queryKey: qk })
    closeModal()
  },
})
function saveItem() {
  saveM.mutate()
}
const delM = useMutation({
  mutationFn: async (id: string) => api.post(`/api/program_center/application/delete/${id}`),
  onSuccess: () => {
    qc.invalidateQueries({ queryKey: qk })
  },
})
async function deleteItem(item: Item) {
  if (await confirmMsg('确定删除该应用？')) delM.mutate(item.id)
}
const categories = ref<Array<{ id?: string; name?: string }>>([])
async function loadCategories() {
  try {
    // GET processplatform/assemble/designer/applicationcategory/list —— 流程应用分类
    const r: any = await api.get('/api/processplatform/assemble/designer/applicationcategory/list')
    categories.value = (r.data ?? []) as Array<{ id?: string; name?: string }>
    if (categories.value.length === 0) toast.success('暂无分类')
  } catch (e: any) {
    toast.error('加载分类失败: ' + (e?.message ?? ''))
  }
}

function loadData() {
  qc.invalidateQueries({ queryKey: qk })
}
function fmtTime(t?: string) {
  if (!t) return ''
  try {
    return new Date(t).toLocaleString('zh-CN', { month: '2-digit', day: '2-digit', hour: '2-digit', minute: '2-digit' })
  } catch {
    return String(t)
  }
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
.desc-cell{max-width:220px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.mono{font-family:'Fira Code',monospace;font-size:12px;color:var(--color-secondary)}
.btn-sm{padding:4px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);cursor:pointer;font-size:12px}
.btn-del{border-color:var(--color-danger);color:var(--color-danger)}
.loading-state,.empty-state{padding:40px;text-align:center;color:var(--text-muted)}
.empty-icon{font-size:32px;margin-bottom:8px}
.modal-overlay{position:fixed;inset:0;background:rgba(0,0,0,0.6);display:flex;align-items:center;justify-content:center;z-index:100}
.modal{padding:24px;width:460px;max-width:90vw}
.modal h3{font-size:16px;color:var(--color-primary);margin:0 0 16px}
.form-group{margin-bottom:12px}
.form-group label{font-size:13px;color:var(--text-primary);display:flex;align-items:center;gap:8px}
.form-group label:first-child{display:block;font-size:12px;color:var(--text-muted)}
.form-input,.form-textarea{width:100%;padding:8px 12px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);outline:none;box-sizing:border-box}
.modal-actions{display:flex;justify-content:flex-end;gap:8px;margin-top:16px}
.btn-cancel{padding:8px 16px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:transparent;color:var(--text-primary);cursor:pointer}
.btn-save{padding:8px 16px;border-radius:var(--radius-md);border:none;background:var(--color-primary);color:white;cursor:pointer;font-weight:600}
.btn-save:disabled{opacity:0.5;cursor:not-allowed}
.skel{height:16px;background:var(--bg-elevated);border-radius:4px;margin-bottom:8px;animation:pulse 1.5s infinite}
@keyframes pulse{0%,100%{opacity:1}50%{opacity:0.4}}
.cat-chips{display:flex;flex-wrap:wrap;gap:6px;margin:8px 0}
.cat-chip{padding:2px 10px;border-radius:10px;background:var(--bg-elevated);border:1px solid var(--border-subtle);font-size:12px;color:var(--text-primary)}
</style>
