<template>
  <div class="crud-view">
    <div class="view-header glass-card">
      <div>
        <h1>公共组件库</h1>
        <p class="subtitle">/api/general/assemble/control/*（x_general_assemble_control_config）</p>
      </div>
      <button class="btn-primary" @click="openCreate">+ 新建</button>
    </div>
    <div class="content-panel glass-card">
      <div class="toolbar">
        <input v-model="search" placeholder="搜索系统名称 / 版本..." class="search-input" />
        <button class="btn-refresh" @click="loadData">🔄 刷新</button>
      </div>
      <div v-if="loading" class="loading-state"><div class="skel" v-for="i in 5" :key="i"></div></div>
      <div v-else-if="items.length===0" class="empty-state"><div class="empty-icon">🧰</div><p>暂无公共组件配置</p></div>
      <table v-else class="data-table">
        <thead><tr><th>系统</th><th>维护模式</th><th>开放注册</th><th>版本</th><th>创建时间</th><th>操作</th></tr></thead>
        <tbody>
          <tr v-for="item in filtered" :key="item.id">
            <td>{{ item.systemName||'—' }}</td>
            <td>{{ item.maintenanceMode?'是':'否' }}</td>
            <td>{{ item.allowRegistration?'是':'否' }}</td>
            <td class="mono">{{ item.version||'—' }}</td>
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
        <h3>{{ showEdit?'编辑':'新建' }}公共组件配置</h3>
        <div class="form-group"><label>系统名称</label><input v-model="form.systemName" placeholder="如 o2server" class="form-input mono" /></div>
        <div class="form-group"><label>版本</label><input v-model="form.version" placeholder="如 2.x" class="form-input mono" /></div>
        <div class="form-group">
          <label><input type="checkbox" v-model="form.maintenanceMode" /> 维护模式</label>
        </div>
        <div class="form-group">
          <label><input type="checkbox" v-model="form.allowRegistration" /> 开放注册</label>
        </div>
        <div class="modal-actions">
          <button class="btn-cancel" @click="closeModal">取消</button>
          <button class="btn-save" :disabled="!form.systemName?.trim()||saving" @click="saveItem">{{ saving?'保存中…':'保存' }}</button>
        </div>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { api } from '@oa4rust/sdk'
import { useMutation, useQuery, useQueryClient } from '@tanstack/vue-query'
import { computed, ref } from 'vue'
import { confirmMsg } from '../utils/toast'

interface Item {
  id: string
  systemName?: string
  maintenanceMode?: boolean
  allowRegistration?: boolean
  version?: string
  createTime?: string
}
interface GeneralForm {
  systemName?: string
  version?: string
  maintenanceMode?: boolean
  allowRegistration?: boolean
}

const listEp = '/api/general/assemble/control/list'
const createEp = '/api/general/assemble/control/create'
const qk = ['Common', 'list']

const search = ref(''),
  showCreate = ref(false),
  showEdit = ref(false),
  loading = ref(false),
  saving = ref(false)
const items = ref<Item[]>([]),
  form = ref<GeneralForm>({}),
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
          (i.systemName || '').toLowerCase().includes(search.value.toLowerCase()) ||
          (i.version || '').toLowerCase().includes(search.value.toLowerCase()),
      )
    : items.value,
)

function openCreate() {
  form.value = { systemName: '', version: '', maintenanceMode: false, allowRegistration: false }
  editingId.value = null
  showCreate.value = true
}
function editItem(item: Item) {
  form.value = {
    systemName: item.systemName ?? '',
    version: item.version ?? '',
    maintenanceMode: Boolean(item.maintenanceMode),
    allowRegistration: Boolean(item.allowRegistration),
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
      // 后端 spec 按 TEXT 列写入：布尔以 'true'/'false' 字符串传递，与列表 bool 读取兼容（PG text→bool）。
      const payload: Record<string, string> = {
        systemName: form.value.systemName ?? '',
        version: form.value.version ?? '',
        maintenanceMode: String(Boolean(form.value.maintenanceMode)),
        allowRegistration: String(Boolean(form.value.allowRegistration)),
      }
      if (editingId.value) return api.post(`/api/general/assemble/control/save/${editingId.value}`, payload)
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
  mutationFn: async (id: string) => api.post(`/api/general/assemble/control/delete/${id}`),
  onSuccess: () => {
    qc.invalidateQueries({ queryKey: qk })
  },
})
async function deleteItem(item: Item) {
  if (await confirmMsg('确定删除该配置？')) delM.mutate(item.id)
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
.mono{font-family:'Fira Code',monospace;font-size:12px;color:var(--color-secondary)}
.btn-sm{padding:4px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);cursor:pointer;font-size:12px}
.btn-del{border-color:var(--color-danger);color:var(--color-danger)}
.loading-state,.empty-state{padding:40px;text-align:center;color:var(--text-muted)}
.empty-icon{font-size:32px;margin-bottom:8px}
.modal-overlay{position:fixed;inset:0;background:rgba(0,0,0,0.6);display:flex;align-items:center;justify-content:center;z-index:100}
.modal{padding:24px;width:440px;max-width:90vw}
.modal h3{font-size:16px;color:var(--color-primary);margin:0 0 16px}
.form-group{margin-bottom:12px}
.form-group label{font-size:13px;color:var(--text-primary);display:flex;align-items:center;gap:8px}
.form-group label:first-child{display:block;font-size:12px;color:var(--text-muted)}
.form-input{width:100%;padding:8px 12px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);outline:none;box-sizing:border-box}
.modal-actions{display:flex;justify-content:flex-end;gap:8px;margin-top:16px}
.btn-cancel{padding:8px 16px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:transparent;color:var(--text-primary);cursor:pointer}
.btn-save{padding:8px 16px;border-radius:var(--radius-md);border:none;background:var(--color-primary);color:white;cursor:pointer;font-weight:600}
.btn-save:disabled{opacity:0.5;cursor:not-allowed}
.skel{height:16px;background:var(--bg-elevated);border-radius:4px;margin-bottom:8px;animation:pulse 1.5s infinite}
@keyframes pulse{0%,100%{opacity:1}50%{opacity:0.4}}
</style>
