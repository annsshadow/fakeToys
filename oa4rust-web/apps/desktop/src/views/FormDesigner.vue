<template>
  <div class="designer-view">
    <div class="view-header glass-card">
      <h1>表单设计器</h1>
      <p class="subtitle">/jaxrs/form/* — 可视化表单构建</p>
      <button class="btn-create" @click="openCreate">+ 新建表单</button>
    </div>

    <div class="toolbar glass-card">
      <div class="tabs">
        <button :class="{active:tab==='v1'}" @click="tab='v1'">表单V1</button>
        <button :class="{active:tab==='v2'}" @click="tab='v2'">表单V2</button>
      </div>
      <div class="search-box">
        <span class="search-icon">⌕</span>
        <input v-model="keyword" placeholder="搜索表单..." class="search-input" @keyup.enter="loadForms" />
      </div>
      <button class="btn-refresh" @click="loadForms">刷新</button>
    </div>

    <div class="content-panel glass-card">
      <div v-if="loading" class="loading-state"><div class="sk" v-for="i in 6" :key="i"></div></div>
      <div v-else-if="forms.length === 0" class="empty-state">
        <div class="ei">📝</div><p>暂无表单</p>
      </div>
      <div v-else class="form-grid">
        <div v-for="f in forms" :key="f.id" class="form-card glass-card" @click="editForm(f)">
          <div class="fc-header">
            <span class="fc-icon">📝</span>
            <div class="fc-info">
              <div class="fc-title">{{ f.name || f.title || '未命名表单' }}</div>
              <div class="fc-flag">flag: {{ f.flag || f.formFlag || f.id }}</div>
            </div>
            <span class="fc-version">{{ f.version || 'v1' }}</span>
          </div>
          <div class="fc-desc">{{ f.desc || f.description || f.content || '暂无描述' }}</div>
          <div class="fc-footer">
            <span class="fc-time">{{ fmtTime(f.updatedAt || f.updateTime) }}</span>
            <div class="fc-actions">
              <button class="btn-edit" @click.stop="editForm(f)">编辑</button>
              <button class="btn-preview" @click.stop="previewForm(f)">预览</button>
              <button class="btn-delete" @click.stop="deleteForm(f)">删除</button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Edit Modal -->
    <div v-if="showModal" class="modal-overlay" @click.self="showModal=false">
      <div class="modal glass-card">
        <h3>{{ editingForm ? '编辑表单' : '新建表单' }}</h3>
        <div class="form-grid">
          <div class="form-group">
            <label>表单名称</label>
            <input v-model="form.name" class="form-input" placeholder="表单名称" />
          </div>
          <div class="form-group">
            <label>Flag</label>
            <input v-model="form.flag" class="form-input" placeholder="唯一标识" :disabled="!!editingForm" />
          </div>
          <div class="form-group full-width">
            <label>字段配置（JSON）</label>
            <textarea v-model="form.fields" class="form-textarea code-area" rows="10"
              placeholder='[{"type":"text","label":"姓名","key":"name"},{"type":"number","label":"年龄","key":"age"}]'></textarea>
          </div>
          <div class="form-group full-width">
            <label>描述</label>
            <textarea v-model="form.desc" class="form-textarea" rows="2"></textarea>
          </div>
        </div>
        <div class="modal-actions">
          <button class="btn-cancel" @click="showModal=false">取消</button>
          <button class="btn-save" :disabled="saving" @click="saveForm">{{ saving ? '保存中...' : '保存' }}</button>
        </div>
      </div>
    </div>

    <!-- Preview Modal -->
    <div v-if="previewFormRef" class="modal-overlay" @click.self="previewFormRef=null">
      <div class="modal glass-card" style="width:500px">
        <h3>{{ previewFormRef.name || '表单预览' }}</h3>
        <div class="preview-fields">
          <div v-for="(field, i) in parsedFields" :key="i" class="preview-field">
            <label>{{ field.label }}</label>
            <component :is="fieldComponent(field.type)" class="preview-input" />
          </div>
        </div>
        <button class="btn-close-modal" @click="previewFormRef=null">关闭</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { api } from '@oa4rust/sdk'

type FormDef = {
  id?: string
  name?: string
  title?: string
  flag?: string
  formFlag?: string
  desc?: string
  description?: string
  content?: string
  version?: string
  fields?: string
  updatedAt?: string
  updateTime?: string
}

type Tab = 'v1' | 'v2'

const tab = ref<Tab>('v1')
const keyword = ref('')
const loading = ref(false)
const forms = ref<FormDef[]>([])
const showModal = ref(false)
const editingForm = ref<FormDef | null>(null)
const previewFormRef = ref<FormDef | null>(null)
const saving = ref(false)
const form = ref({ name: '', flag: '', fields: '', desc: '' })

const parsedFields = computed(() => {
  if (!previewFormRef.value?.fields) return []
  try { return JSON.parse(previewFormRef.value.fields) } catch { return [] }
})

function fieldComponent(type: string) {
  const map: Record<string, string> = {
    text: 'input', number: 'input', textarea: 'textarea',
    select: 'select', date: 'input', checkbox: 'input',
  }
  return map[type] || 'input'
}

function fmtTime(t?: string) {
  return t ? new Date(t).toLocaleString('zh-CN', { month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit' }) : '-'
}

async function loadForms() {
  loading.value = true
  try {
    const url = tab.value === 'v2' ? '/jaxrs/form/v2/list' : '/jaxrs/form/list'
    const r = await api.get(url)
    forms.value = r.data ?? []
    if (keyword.value) {
      forms.value = forms.value.filter(f =>
        (f.name || f.title || '').toLowerCase().includes(keyword.value.toLowerCase())
      )
    }
  } catch { forms.value = [] } finally { loading.value = false }
}

function openCreate() {
  editingForm.value = null
  form.value = { name: '', flag: '', fields: '', desc: '' }
  showModal.value = true
}

function editForm(f: FormDef) {
  editingForm.value = f
  form.value = {
    name: f.name || f.title || '',
    flag: f.flag || f.formFlag || '',
    fields: f.fields || '[]',
    desc: f.desc || f.description || '',
  }
  showModal.value = true
}

function previewForm(f: FormDef) {
  previewFormRef.value = f
}

async function saveForm() {
  if (!form.value.name.trim()) { alert('请输入表单名称'); return }
  saving.value = true
  try {
    const data = {
      name: form.value.name,
      flag: form.value.flag,
      fields: form.value.fields,
      description: form.value.desc,
    }
    if (editingForm.value?.id) {
      await api.put(`/jaxrs/form/${editingForm.value.id}`, data)
    } else {
      await api.post('/jaxrs/form', data)
    }
    showModal.value = false
    loadForms()
  } catch (e: any) { alert('保存失败: ' + (e?.message ?? '')) } finally { saving.value = false }
}

async function deleteForm(f: FormDef) {
  if (!confirm(`删除表单「${f.name || f.flag}」？`)) return
  try {
    await api.delete(`/jaxrs/form/${f.id}`)
    forms.value = forms.value.filter(x => x.id !== f.id)
  } catch (e: any) { alert('删除失败: ' + (e?.message ?? '')) }
}

loadForms()
</script>

<style scoped>
.designer-view { display: flex; flex-direction: column; gap: 16px; height: 100% }
.view-header { display: flex; align-items: center; justify-content: space-between; padding: 16px 24px }
.view-header h1 { font-family: 'Orbitron', sans-serif; font-size: 20px; color: var(--color-primary); margin: 0; text-shadow: 0 0 15px var(--color-primary-glow) }
.subtitle { font-size: 12px; color: var(--text-muted); margin: 4px 0 0; font-family: 'JetBrains Mono', monospace }
.btn-create { padding: 8px 20px; background: var(--color-accent); color: #fff; border: none; border-radius: var(--radius-md); font-size: 13px; cursor: pointer; font-weight: 600 }
.toolbar { display: flex; align-items: center; gap: 12px; padding: 12px 16px }
.tabs { display: flex; gap: 8px }
.tabs button { padding: 6px 16px; background: var(--bg-elevated); border: 1px solid var(--border-subtle); border-radius: var(--radius-md); color: var(--text-secondary); font-size: 13px; cursor: pointer }
.tabs button.active { background: var(--color-primary); color: #000; border-color: var(--color-primary); font-weight: 600 }
.search-box { flex: 1; display: flex; align-items: center; gap: 8px; background: var(--bg-elevated); border: 1px solid var(--border-subtle); border-radius: var(--radius-md); padding: 6px 12px; max-width: 300px }
.search-icon { color: var(--text-muted) }
.search-input { background: none; border: none; outline: none; color: var(--text-primary); font-size: 14px; flex: 1 }
.btn-refresh { padding: 6px 16px; background: var(--bg-elevated); border: 1px solid var(--border-subtle); border-radius: var(--radius-md); color: var(--text-secondary); cursor: pointer }
.btn-refresh:hover { border-color: var(--color-primary); color: var(--color-primary) }
.content-panel { flex: 1; overflow-y: auto; padding: 16px }
.form-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(260px, 1fr)); gap: 12px }
.form-card { padding: 14px; cursor: pointer; transition: all var(--transition-fast); border: 1px solid var(--border-subtle); border-radius: var(--radius-lg); background: var(--bg-elevated) }
.form-card:hover { border-color: var(--color-primary); transform: translateY(-2px); box-shadow: var(--shadow-glow) }
.fc-header { display: flex; align-items: center; gap: 10px; margin-bottom: 6px }
.fc-icon { font-size: 22px }
.fc-info { flex: 1; min-width: 0 }
.fc-title { font-size: 14px; font-weight: 600; color: var(--text-primary); overflow: hidden; text-overflow: ellipsis; white-space: nowrap }
.fc-flag { font-size: 11px; color: var(--color-primary-deep); font-family: 'JetBrains Mono', monospace; margin-top: 2px }
.fc-version { font-size: 11px; padding: 2px 8px; background: var(--color-accent-soft); color: var(--color-accent); border-radius: var(--radius-sm) }
.fc-desc { font-size: 12px; color: var(--text-muted); margin-bottom: 10px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap }
.fc-footer { display: flex; justify-content: space-between; align-items: center }
.fc-time { font-size: 11px; color: var(--text-muted) }
.fc-actions { display: flex; gap: 6px }
.btn-edit { padding: 3px 8px; background: transparent; border: 1px solid var(--color-primary); color: var(--color-primary); border-radius: var(--radius-sm); font-size: 11px; cursor: pointer }
.btn-edit:hover { background: var(--color-primary); color: #000 }
.btn-preview { padding: 3px 8px; background: transparent; border: 1px solid var(--color-success); color: var(--color-success); border-radius: var(--radius-sm); font-size: 11px; cursor: pointer }
.btn-preview:hover { background: var(--color-success); color: #000 }
.btn-delete { padding: 3px 8px; background: transparent; border: 1px solid var(--color-error); color: var(--color-error); border-radius: var(--radius-sm); font-size: 11px; cursor: pointer }
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
.form-input, .form-textarea { background: var(--bg-elevated); border: 1px solid var(--border-subtle); border-radius: var(--radius-md); color: var(--text-primary); padding: 10px 12px; font-size: 14px }
.form-input:focus, .form-textarea:focus { outline: none; border-color: var(--color-primary) }
.form-textarea { resize: vertical; font-family: inherit }
.code-area { font-family: 'JetBrains Mono', monospace; font-size: 12px }
.modal-actions { display: flex; justify-content: flex-end; gap: 8px }
.btn-cancel { padding: 8px 20px; background: transparent; border: 1px solid var(--border-subtle); color: var(--text-secondary); border-radius: var(--radius-md); cursor: pointer }
.btn-save { padding: 8px 20px; background: var(--color-primary); color: #000; border: none; border-radius: var(--radius-md); font-size: 13px; cursor: pointer; font-weight: 600 }
.btn-save:disabled { opacity: 0.5; cursor: not-allowed }
.btn-close-modal { padding: 8px 20px; background: transparent; border: 1px solid var(--color-primary); color: var(--color-primary); border-radius: var(--radius-md); cursor: pointer; margin-top: 12px }
.preview-fields { display: flex; flex-direction: column; gap: 12px; max-height: 400px; overflow-y: auto }
.preview-field { display: flex; flex-direction: column; gap: 4px }
.preview-field label { font-size: 13px; font-weight: 500; color: var(--text-secondary) }
.preview-input { background: var(--bg-elevated); border: 1px solid var(--border-subtle); border-radius: var(--radius-md); color: var(--text-primary); padding: 8px 12px; font-size: 14px }
@media(max-width:768px){.form-grid{grid-template-columns:1fr}.form-group.full-width{grid-column:span 1}}
</style>
