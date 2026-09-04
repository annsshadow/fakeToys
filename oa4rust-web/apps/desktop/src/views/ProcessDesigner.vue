<template>
  <div class="designer-view">
    <div class="view-header glass-card">
      <h1>流程设计器</h1>
      <p class="subtitle">/jaxrs/processplatform/assemble/designer/*</p>
      <button class="btn-create" @click="openCreate">+ 新建流程</button>
    </div>

    <div class="toolbar glass-card">
      <div class="search-box">
        <span class="search-icon">⌕</span>
        <input v-model="keyword" placeholder="搜索流程..." class="search-input" @keyup.enter="doSearch" />
      </div>
      <button class="btn-refresh" @click="loadProcesses">刷新</button>
    </div>

    <div class="content-panel glass-card">
      <div v-if="loading" class="loading-state">
        <div class="sk" v-for="i in 6" :key="i"></div>
      </div>
      <div v-else-if="processes.length === 0" class="empty-state">
        <div class="ei">⚙️</div>
        <p>暂无流程定义</p>
      </div>
      <div v-else class="process-grid">
        <div v-for="p in processes" :key="p.id" class="process-card glass-card" @click="editProcess(p)">
          <div class="pc-header">
            <span class="pc-icon">⚙️</span>
            <div class="pc-info">
              <div class="pc-title">{{ p.name || p.processName || '未命名流程' }}</div>
              <div class="pc-flag">{{ p.flag || p.id }}</div>
            </div>
            <span class="pc-status" :class="p.status || 'active'">{{ statusLabel(p) }}</span>
          </div>
          <div class="pc-desc">{{ p.description || p.desc || '暂无描述' }}</div>
          <div class="pc-footer">
            <span class="pc-time">{{ fmtTime(p.updatedAt || p.updateTime) }}</span>
            <div class="pc-actions">
              <button class="btn-edit" @click.stop="editProcess(p)">编辑</button>
              <button class="btn-delete" @click.stop="deleteProcess(p)">删除</button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Create/Edit Modal -->
    <div v-if="showModal" class="modal-overlay" @click.self="showModal = false">
      <div class="modal glass-card">
        <h3>{{ editingProcess ? '编辑流程' : '新建流程' }}</h3>
        <div class="form-grid">
          <div class="form-group">
            <label>流程名称</label>
            <input v-model="form.name" class="form-input" placeholder="请输入流程名称" />
          </div>
          <div class="form-group">
            <label>Flag（唯一标识）</label>
            <input v-model="form.flag" class="form-input" placeholder="如: leave_approval" :disabled="!!editingProcess" />
          </div>
          <div class="form-group full-width">
            <label>描述</label>
            <textarea v-model="form.desc" class="form-textarea" placeholder="流程描述"></textarea>
          </div>
          <div class="form-group full-width">
            <label>流程配置（JSON）</label>
            <textarea v-model="form.config" class="form-textarea code-area" rows="8" placeholder='{"nodes":[...],"edges":[...]}'></textarea>
          </div>
        </div>
        <div class="modal-actions">
          <button class="btn-cancel" @click="showModal = false">取消</button>
          <button class="btn-save" :disabled="saving" @click="saveProcess">
            {{ saving ? '保存中...' : '保存' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { api } from '@oa4rust/sdk'

type ProcessDef = {
  id?: string
  name?: string
  processName?: string
  flag?: string
  description?: string
  desc?: string
  status?: string
  updatedAt?: string
  updateTime?: string
  config?: Record<string, unknown>
}

const keyword = ref('')
const loading = ref(false)
const processes = ref<ProcessDef[]>([])
const showModal = ref(false)
const editingProcess = ref<ProcessDef | null>(null)
const saving = ref(false)
const form = ref({ name: '', flag: '', desc: '', config: '' })

const stats = computed(() => [
  { label: '总流程', value: processes.value.length, color: 'var(--color-primary)' },
  { label: '已启用', value: processes.value.filter(p => p.status !== 'disabled').length, color: 'var(--color-success)' },
  { label: '已禁用', value: processes.value.filter(p => p.status === 'disabled').length, color: 'var(--color-error)' },
])

function statusLabel(p: ProcessDef) {
  if (p.status === 'disabled') return '禁用'
  if (p.status === 'draft') return '草稿'
  return '启用'
}

function fmtTime(t?: string) {
  if (!t) return '-'
  return new Date(t).toLocaleString('zh-CN', { month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit' })
}

async function doSearch() {
  loading.value = true
  try {
    const r = await api.post(`/jaxrs/processplatform/assemble/designer/process/list/paging/1/20`, {})
    processes.value = r.data?.list ?? r.data ?? []
    if (keyword.value) {
      processes.value = processes.value.filter(p =>
        (p.name || p.processName || '').toLowerCase().includes(keyword.value.toLowerCase())
      )
    }
  } catch { processes.value = [] } finally { loading.value = false }
}

async function loadProcesses() {
  await doSearch()
}

function openCreate() {
  editingProcess.value = null
  form.value = { name: '', flag: '', desc: '', config: '' }
  showModal.value = true
}

function editProcess(p: ProcessDef) {
  editingProcess.value = p
  form.value = {
    name: p.name || p.processName || '',
    flag: p.flag || '',
    desc: p.description || p.desc || '',
    config: p.config ? JSON.stringify(p.config, null, 2) : '',
  }
  showModal.value = true
}

async function saveProcess() {
  if (!form.value.name.trim()) { alert('请输入流程名称'); return }
  saving.value = true
  try {
    const data = {
      name: form.value.name,
      flag: form.value.flag,
      description: form.value.desc,
      ...(form.value.config ? { config: JSON.parse(form.value.config) } : {}),
    }
    if (editingProcess.value?.id) {
      await api.put(`/jaxrs/processplatform/assemble/designer/process/${editingProcess.value.id}`, data)
    } else {
      await api.post('/jaxrs/processplatform/assemble/designer/process', data)
    }
    showModal.value = false
    loadProcesses()
  } catch (e: any) {
    alert('保存失败: ' + (e?.message ?? '未知错误'))
  } finally { saving.value = false }
}

async function deleteProcess(p: ProcessDef) {
  if (!confirm(`确定删除流程「${p.name || p.flag}」？`)) return
  try {
    await api.delete(`/jaxrs/processplatform/assemble/designer/process/${p.id}`)
    processes.value = processes.value.filter(x => x.id !== p.id)
  } catch (e: any) { alert('删除失败: ' + (e?.message ?? '')) }
}

loadProcesses()
</script>

<style scoped>
.designer-view { display: flex; flex-direction: column; gap: 16px; height: 100% }
.view-header { display: flex; align-items: center; justify-content: space-between; padding: 16px 24px }
.view-header h1 { font-family: 'Orbitron', sans-serif; font-size: 20px; color: var(--color-primary); margin: 0; text-shadow: 0 0 15px var(--color-primary-glow) }
.subtitle { font-size: 12px; color: var(--text-muted); margin: 4px 0 0; font-family: 'JetBrains Mono', monospace }
.btn-create { padding: 8px 20px; background: var(--color-primary); color: #000; border: none; border-radius: var(--radius-md); font-size: 13px; cursor: pointer; font-weight: 600 }
.toolbar { display: flex; gap: 8px; padding: 12px 16px }
.search-box { flex: 1; display: flex; align-items: center; gap: 8px; background: var(--bg-elevated); border: 1px solid var(--border-subtle); border-radius: var(--radius-md); padding: 6px 12px }
.search-icon { color: var(--text-muted); font-size: 16px }
.search-input { background: none; border: none; outline: none; color: var(--text-primary); font-size: 14px; flex: 1 }
.btn-refresh { padding: 6px 16px; background: var(--bg-elevated); border: 1px solid var(--border-subtle); border-radius: var(--radius-md); color: var(--text-secondary); font-size: 13px; cursor: pointer }
.btn-refresh:hover { border-color: var(--color-primary); color: var(--color-primary) }
.content-panel { flex: 1; overflow-y: auto; padding: 16px }
.process-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: 12px }
.process-card { padding: 16px; cursor: pointer; transition: all var(--transition-fast); border: 1px solid var(--border-subtle); border-radius: var(--radius-lg); background: var(--bg-elevated) }
.process-card:hover { border-color: var(--color-primary); transform: translateY(-2px); box-shadow: var(--shadow-glow) }
.pc-header { display: flex; align-items: center; gap: 12px; margin-bottom: 8px }
.pc-icon { font-size: 24px }
.pc-info { flex: 1; min-width: 0 }
.pc-title { font-size: 14px; font-weight: 600; color: var(--text-primary); overflow: hidden; text-overflow: ellipsis; white-space: nowrap }
.pc-flag { font-size: 11px; color: var(--color-primary-deep); font-family: 'JetBrains Mono', monospace; margin-top: 2px }
.pc-status { font-size: 11px; padding: 2px 8px; border-radius: var(--radius-sm); font-weight: 600 }
.pc-status.active { background: rgba(16,185,129,.15); color: var(--color-success) }
.pc-status.disabled { background: rgba(239,68,68,.15); color: var(--color-error) }
.pc-status.draft { background: rgba(245,158,11,.15); color: var(--color-warning) }
.pc-desc { font-size: 12px; color: var(--text-muted); margin-bottom: 12px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap }
.pc-footer { display: flex; justify-content: space-between; align-items: center }
.pc-time { font-size: 11px; color: var(--text-muted) }
.pc-actions { display: flex; gap: 6px }
.btn-edit { padding: 4px 10px; background: transparent; border: 1px solid var(--color-primary); color: var(--color-primary); border-radius: var(--radius-sm); font-size: 12px; cursor: pointer }
.btn-edit:hover { background: var(--color-primary); color: #000 }
.btn-delete { padding: 4px 10px; background: transparent; border: 1px solid var(--color-error); color: var(--color-error); border-radius: var(--radius-sm); font-size: 12px; cursor: pointer }
.btn-delete:hover { background: var(--color-error); color: #fff }
.loading-state, .empty-state { display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 60px; color: var(--text-muted); gap: 12px }
.sk { height: 40px; border-radius: var(--radius-md); background: var(--bg-elevated); animation: pulse 1.2s ease-in-out infinite }
@keyframes pulse { 0%,100%{opacity:.4}50%{opacity:.8} }
.ei { font-size: 48px; opacity: 0.4 }
.modal-overlay { position: fixed; inset: 0; background: rgba(0,0,0,.7); display: flex; align-items: center; justify-content: center; z-index: 100 }
.modal { background: var(--bg-surface); border: 1px solid var(--border-subtle); border-radius: var(--radius-lg); padding: 24px; width: 600px; max-width: 90vw; max-height: 85vh; overflow: auto; display: flex; flex-direction: column; gap: 16px }
.modal h3 { font-family: 'Orbitron', sans-serif; color: var(--color-primary); margin: 0; font-size: 16px }
.form-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 12px }
.form-group { display: flex; flex-direction: column; gap: 6px }
.form-group.full-width { grid-column: span 2 }
.form-group label { font-size: 13px; color: var(--text-muted) }
.form-input, .form-textarea { background: var(--bg-elevated); border: 1px solid var(--border-subtle); border-radius: var(--radius-md); color: var(--text-primary); padding: 10px 12px; font-size: 14px; font-family: inherit }
.form-input:focus, .form-textarea:focus { outline: none; border-color: var(--color-primary) }
.form-textarea { resize: vertical; min-height: 60px }
.code-area { font-family: 'JetBrains Mono', monospace; font-size: 12px }
.modal-actions { display: flex; justify-content: flex-end; gap: 8px }
.btn-cancel { padding: 8px 20px; background: transparent; border: 1px solid var(--border-subtle); color: var(--text-secondary); border-radius: var(--radius-md); cursor: pointer }
.btn-save { padding: 8px 20px; background: var(--color-primary); color: #000; border: none; border-radius: var(--radius-md); font-size: 13px; cursor: pointer; font-weight: 600 }
.btn-save:disabled { opacity: 0.5; cursor: not-allowed }
@media(max-width:768px){.form-grid{grid-template-columns:1fr}.form-group.full-width{grid-column:span 1}}
</style>
