<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="crud-view">
    <div class="view-header glass-card">
      <div>
        <h1>查询定义</h1>
        <p class="subtitle">/api/query/assemble/designer/*（x_query_design）</p>
      </div>
      <button class="btn-primary" @click="openCreate">+ 新建</button>
    </div>
    <div class="content-panel glass-card">
      <div class="toolbar">
        <input v-model="search" placeholder="搜索名称 / 分类..." class="search-input" />
        <button class="btn-refresh" @click="loadData">🔄 刷新</button>
      </div>
      <div v-if="loading" class="loading-state"><div class="skel" v-for="i in 5" :key="i"></div></div>
      <div v-else-if="items.length===0" class="empty-state"><div class="empty-icon">🔍</div><p>暂无查询定义</p></div>
      <table v-else class="data-table">
        <thead><tr><th>名称</th><th>分类</th><th>定义 ID</th><th>操作</th></tr></thead>
        <tbody>
          <tr v-for="item in filtered" :key="item.id">
            <td>{{ item.name||'—' }}</td>
            <td>{{ item.category||'—' }}</td>
            <td class="mono">{{ item.id||'—' }}</td>
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
        <h3>{{ showEdit?'编辑':'新建' }}查询定义</h3>
        <div class="form-group"><label>名称</label><input v-model="form.name" placeholder="查询名称" class="form-input" /></div>
        <div class="form-group"><label>分类</label><input v-model="form.category" placeholder="如 default / report" class="form-input mono" /></div>
        <div class="form-group"><label>查询定义</label><textarea v-model="form.query" rows="4" placeholder="SQL / 查询表达式" class="form-textarea mono"></textarea></div>
        <div class="modal-actions">
          <button class="btn-cancel" @click="closeModal">取消</button>
          <button class="btn-save" :disabled="!form.name?.trim()||saving" @click="saveItem">{{ saving?'保存中…':'保存' }}</button>
        </div>
      </div>
    </div>

    <!-- 表面查询（x_query_surface）——rev106 -->
    <div class="content-panel glass-card">
      <div class="toolbar">
        <h3 class="sf-title">表面查询（surface）</h3>
        <input v-model="sfCategory" placeholder="分类（默认 default）" class="search-input mono" @keydown.enter="loadSurfaces" />
        <button class="btn-refresh" @click="loadSurfaces">🔄 加载</button>
        <button class="btn-sm" @click="openSurfaceCreate">+ 新建表面</button>
      </div>
      <div v-if="surfaces.length===0" class="empty-state"><p>该分类暂无表面查询</p></div>
      <table v-else class="data-table">
        <thead><tr><th>名称</th><th>分类</th><th>ID</th><th>操作</th></tr></thead>
        <tbody>
          <tr v-for="s in surfaces" :key="s.id">
            <td>{{ s.name||'—' }}</td>
            <td>{{ s.category||'—' }}</td>
            <td class="mono">{{ s.id }}</td>
            <td>
              <button class="btn-sm" @click="viewSurface(s)">详情</button>
              <button class="btn-sm" @click="editSurface(s)">编辑</button>
              <button class="btn-sm btn-del" @click="deleteSurface(s)">删除</button>
            </td>
          </tr>
        </tbody>
      </table>
      <div v-if="sfDetailText" class="sf-detail mono">{{ sfDetailText }}</div>
    </div>

    <div v-if="showSurfaceModal" class="modal-overlay" @click.self="showSurfaceModal=false">
      <div class="modal glass-card">
        <h3>{{ sfEditingId?'编辑':'新建' }}表面查询</h3>
        <div class="form-group"><label>名称</label><input v-model="sfForm.name" class="form-input" placeholder="表面名称" /></div>
        <div class="form-group"><label>分类</label><input v-model="sfForm.category" class="form-input mono" placeholder="default" /></div>
        <div class="form-group"><label>内容</label><textarea v-model="sfForm.content" rows="4" class="form-textarea mono" placeholder="表面定义（JSON/表达式）"></textarea></div>
        <div class="modal-actions">
          <button class="btn-cancel" @click="showSurfaceModal=false">取消</button>
          <button class="btn-save" :disabled="!sfForm.name?.trim()||sfSaving" @click="saveSurface">{{ sfSaving?'保存中…':'保存' }}</button>
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
  category?: string
}

const listEp = '/api/query/assemble/designer/list'
const saveEp = '/api/query/assemble/designer/save'
const deleteEp = '/api/query/assemble/designer/delete'
const qk = ['QueryQuery', 'list']

const search = ref(''),
  showCreate = ref(false),
  showEdit = ref(false),
  loading = ref(false),
  saving = ref(false)
const items = ref<Item[]>([]),
  form = ref<Partial<Item & { query?: string }>>({}),
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
          (i.category || '').toLowerCase().includes(search.value.toLowerCase()),
      )
    : items.value,
)

function openCreate() {
  form.value = { name: '', category: '', query: '' }
  editingId.value = null
  showCreate.value = true
}
function editItem(item: Item) {
  form.value = { ...item, query: '' }
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
      const payload: Record<string, string> = {
        name: form.value.name ?? '',
        category: form.value.category ?? '',
        query: form.value.query ?? '',
      }
      if (editingId.value) payload.id = editingId.value
      return api.post(saveEp, payload)
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
  mutationFn: async (id: string) => api.post(deleteEp, { id }),
  onSuccess: () => {
    qc.invalidateQueries({ queryKey: qk })
  },
})
async function deleteItem(item: Item) {
  if (await confirmMsg('确定删除该查询定义？')) delM.mutate(item.id)
}
function loadData() {
  qc.invalidateQueries({ queryKey: qk })
}

// ── 表面查询 CRUD（x_query_surface，rev106）─────────────────────
interface Surface {
  id: string
  name?: string
  category?: string
  content?: string
}
const sfCategory = ref('default')
const surfaces = ref<Surface[]>([])
const sfDetailText = ref('')
const showSurfaceModal = ref(false)
const sfSaving = ref(false)
const sfEditingId = ref<string | null>(null)
const sfForm = ref<Partial<Surface>>({})

async function loadSurfaces(): Promise<void> {
  const cat = (sfCategory.value || 'default').trim()
  try {
    // GET surface/list/{category} —— 按分类列出表面查询
    const r: any = await api.get(`/api/query/assemble/surface/list/${encodeURIComponent(cat)}`)
    surfaces.value = (Array.isArray(r?.data) ? r.data : []) as Surface[]
  } catch {
    surfaces.value = []
  }
}
async function viewSurface(s: Surface): Promise<void> {
  try {
    // GET surface/get/{id} —— 表面详情
    const r: any = await api.get(`/api/query/assemble/surface/get/${s.id}`)
    const d = r?.data
    sfDetailText.value = d ? JSON.stringify(d).slice(0, 200) : '（空）'
  } catch (e: any) {
    toast.error('加载表面详情失败: ' + (e?.message ?? ''))
  }
}
function openSurfaceCreate(): void {
  sfForm.value = { name: '', category: sfCategory.value || 'default', content: '' }
  sfEditingId.value = null
  showSurfaceModal.value = true
}
function editSurface(s: Surface): void {
  sfForm.value = { name: s.name, category: s.category, content: s.content }
  sfEditingId.value = s.id
  showSurfaceModal.value = true
}
async function saveSurface(): Promise<void> {
  if (!sfForm.value.name?.trim() || sfSaving.value) return
  sfSaving.value = true
  const payload = {
    name: sfForm.value.name ?? '',
    category: sfForm.value.category ?? 'default',
    content: sfForm.value.content ?? '',
  }
  try {
    if (sfEditingId.value) {
      // POST surface/save/{id} —— 更新
      await api.post(`/api/query/assemble/surface/save/${sfEditingId.value}`, payload)
    } else {
      // POST surface/create —— 新建
      await api.post('/api/query/assemble/surface/create', payload)
    }
    toast.success('表面已保存')
    showSurfaceModal.value = false
    await loadSurfaces()
  } catch (e: any) {
    toast.error('保存表面失败: ' + (e?.message ?? ''))
  } finally {
    sfSaving.value = false
  }
}
async function deleteSurface(s: Surface): Promise<void> {
  if (!(await confirmMsg('确定删除该表面查询？'))) return
  try {
    // POST surface/delete/{id} —— 删除
    await api.post(`/api/query/assemble/surface/delete/${s.id}`)
    surfaces.value = surfaces.value.filter((x) => x.id !== s.id)
    toast.success('表面已删除')
  } catch (e: any) {
    toast.error('删除表面失败: ' + (e?.message ?? ''))
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
.mono{font-family:'Fira Code',monospace;font-size:12px;color:var(--color-secondary)}
.btn-sm{padding:4px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);cursor:pointer;font-size:12px}
.btn-del{border-color:var(--color-danger);color:var(--color-danger)}
.loading-state,.empty-state{padding:40px;text-align:center;color:var(--text-muted)}
.empty-icon{font-size:32px;margin-bottom:8px}
.modal-overlay{position:fixed;inset:0;background:rgba(0,0,0,0.6);display:flex;align-items:center;justify-content:center;z-index:100}
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
.sf-title{font-size:14px;color:var(--color-primary);margin:0 12px 0 0}
.sf-detail{margin-top:12px;padding:10px 12px;background:var(--bg-elevated);border-radius:var(--radius-md);font-size:12px;color:var(--text-secondary);word-break:break-all}
@keyframes pulse{0%,100%{opacity:1}50%{opacity:0.4}}
</style>
