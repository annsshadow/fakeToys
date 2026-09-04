<template>
  <div class="fd">
    <!-- Header -->
    <div class="fd-header glass-card">
      <div class="fd-title">
        <h1>表单设计器</h1>
        <p class="subtitle">/jaxrs/form/* — 可视化表单构建器</p>
      </div>
      <div class="fd-actions">
        <button class="btn" @click="resetForm" title="新建表单">📄 新建</button>
        <button class="btn btn-outline" @click="loadForms" title="刷新列表">🔄 刷新</button>
        <button class="btn btn-outline" :class="{ active: mode === 'preview' }" @click="togglePreview">👁 预览</button>
        <button class="btn btn-primary" :disabled="!currentForm || !currentForm.name" @click="saveForm">💾 保存</button>
      </div>
    </div>

    <div class="fd-body">
      <!-- Left: Form List -->
      <aside class="fd-list-panel glass-card">
        <div class="list-header"><span>📋 表单列表</span><button class="btn-sm" @click="loadForms">刷新</button></div>
        <div class="list-search"><input v-model="listFilter" placeholder="搜索..." class="list-input" /></div>
        <div class="list-items">
          <div v-if="formsLoading" class="list-loading">加载中...</div>
          <div v-else-if="filteredForms.length===0" class="list-empty">暂无表单</div>
          <div v-for="f in filteredForms" :key="f.id" class="list-item" :class="{active:currentForm?.id===f.id}" @click="loadForm(f)">
            <div class="li-name">{{ f.name||f.title||'未命名' }}</div>
            <div class="li-meta">{{ f.flag||f.id }}</div>
          </div>
        </div>
      </aside>

      <!-- Palette + Canvas + Props -->
      <div class="fd-center">
        <!-- Left: Field Palette -->
        <aside class="fd-palette glass-card" v-if="mode==='edit'">
          <div class="palette-title">字段组件</div>
          <div class="palette-grid">
            <div v-for="ft in fieldTypes" :key="ft.type" class="palette-item" draggable="true"
              @dragstart="onDragStart($event, ft)" @click="addField(ft)">
              <span class="pi-icon">{{ ft.icon }}</span>
              <span class="pi-label">{{ ft.label }}</span>
            </div>
          </div>
          <div class="palette-sep"></div>
          <div class="palette-title">布局组件</div>
          <div class="palette-grid">
            <div v-for="ft in layoutTypes" :key="ft.type" class="palette-item" draggable="true"
              @dragstart="onDragStart($event, ft)" @click="addField(ft)">
              <span class="pi-icon">{{ ft.icon }}</span>
              <span class="pi-label">{{ ft.label }}</span>
            </div>
          </div>
        </aside>

        <!-- Center: Canvas -->
        <main class="fd-canvas glass-card" @dragover.prevent @drop="onDrop">
          <div v-if="!currentForm" class="canvas-empty">
            <div class="ce-icon">📋</div>
            <p>选择或新建表单开始设计</p>
            <button class="btn btn-primary" @click="resetForm">+ 新建表单</button>
          </div>
          <div v-else class="canvas-form">
            <div class="cf-header">
              <input v-model="currentForm.name" placeholder="表单名称" class="cf-name" />
              <input v-model="currentForm.flag" placeholder="唯一标识(Flag)" class="cf-flag" :disabled="!!currentForm.id" />
            </div>
            <div class="cf-fields">
              <div v-if="currentForm.fields.length===0" class="fields-empty">拖拽字段到此处，或点击上方组件添加</div>
              <div v-for="(field, index) in currentForm.fields" :key="field.id" class="field-row"
                :class="{ selected: selectedField?.id===field.id }" @click="selectField(field)">
                <div class="fr-handle">⠿</div>
                <div class="fr-icon">{{ getFieldIcon(field.type) }}</div>
                <div class="fr-info">
                  <div class="fr-label">{{ field.label || field.placeholder || '未命名字段' }}</div>
                  <div class="fr-meta">{{ field.type }} · {{ field.key || '—' }}</div>
                </div>
                <div class="fr-actions">
                  <button class="fa-btn" @click.stop="moveField(index,-1)" :disabled="index===0" title="上移">↑</button>
                  <button class="fa-btn" @click.stop="moveField(index,1)" :disabled="index===currentForm.fields.length-1" title="下移">↓</button>
                  <button class="fa-btn fa-del" @click.stop="removeField(index)" title="删除">✕</button>
                </div>
              </div>
            </div>
          </div>
        </main>

        <!-- Right: Property Panel -->
        <aside class="fd-props glass-card" v-if="mode==='edit' && selectedField">
          <div class="props-title"><span>属性面板</span><span class="props-type">{{ selectedField.type }}</span></div>
          <div class="props-body">
            <div class="prop-group"><label>显示名称</label><input v-model="selectedField.label" class="prop-input" placeholder="字段标签" /></div>
            <div class="prop-group"><label>字段标识 (Key)</label><input v-model="selectedField.key" class="prop-input" placeholder="唯一标识" /></div>
            <div class="prop-group"><label>占位提示</label><input v-model="selectedField.placeholder" class="prop-input" placeholder="输入提示文字" /></div>
            <div class="prop-group"><label>默认值</label><input v-model="selectedField.defaultValue" class="prop-input" placeholder="默认值" /></div>
            <div class="prop-group prop-row">
              <label class="checkbox-label"><input type="checkbox" v-model="selectedField.required" />必填</label>
              <label class="checkbox-label"><input type="checkbox" v-model="selectedField.disabled" />只读</label>
            </div>
            <div v-if="['select','checkbox_group','radio'].includes(selectedField.type)" class="prop-group">
              <label>选项配置（每行：值|显示名）</label>
              <textarea v-model="selectedField.optionsStr" class="prop-textarea" placeholder="male|男&#10;female|女"></textarea>
            </div>
            <div v-if="selectedField.type==='textarea'" class="prop-group">
              <label>行数</label><input v-model.number="selectedField.rows" type="number" class="prop-input" min="2" max="20" />
            </div>
            <div v-if="selectedField.type==='number'" class="prop-group">
              <label>范围</label>
              <div class="prop-row2">
                <input v-model.number="selectedField.min" type="number" class="prop-input" placeholder="最小" />
                <input v-model.number="selectedField.max" type="number" class="prop-input" placeholder="最大" />
              </div>
            </div>
          </div>
        </aside>
      </div>
    </div>

    <!-- Preview Modal -->
    <div v-if="mode==='preview' && currentForm" class="preview-overlay" @click.self="mode='edit'">
      <div class="preview-modal glass-card">
        <div class="preview-header">
          <h2>{{ currentForm.name || '表单预览' }}</h2>
          <button class="btn-close" @click="mode='edit'">✕</button>
        </div>
        <div class="preview-body">
          <form @submit.prevent>
            <div v-for="(field, i) in currentForm.fields" :key="i" class="pv-field">
              <label v-if="!['row_start','row_end'].includes(field.type)">
                {{ field.label }}<span v-if="field.required" class="req">*</span>
              </label>
              <!-- Text -->
              <input v-if="field.type==='text'" :placeholder="field.placeholder" :value="field.defaultValue" :disabled="field.disabled" class="pv-input" />
              <!-- Textarea -->
              <textarea v-else-if="field.type==='textarea'" :placeholder="field.placeholder" :rows="field.rows||4" :disabled="field.disabled" class="pv-input"></textarea>
              <!-- Number -->
              <input v-else-if="field.type==='number'" type="number" :placeholder="field.placeholder" :disabled="field.disabled" class="pv-input" />
              <!-- Date -->
              <input v-else-if="field.type==='date'" type="date" :disabled="field.disabled" class="pv-input" />
              <!-- Select -->
              <select v-else-if="field.type==='select'" :disabled="field.disabled" class="pv-input">
                <option value="">请选择</option>
                <option v-for="opt in parseOptions(field.optionsStr)" :key="opt.value" :value="opt.value">{{ opt.label }}</option>
              </select>
              <!-- Checkbox -->
              <label v-else-if="field.type==='checkbox'" class="pv-cb-label">
                <input type="checkbox" :disabled="field.disabled" class="pv-cb" />{{ field.label }}
              </label>
              <!-- Checkbox group -->
              <div v-else-if="field.type==='checkbox_group'" class="pv-cb-group">
                <label v-for="opt in parseOptions(field.optionsStr)" :key="opt.value" class="pv-cb-label">
                  <input type="checkbox" :value="opt.value" :disabled="field.disabled" class="pv-cb" />{{ opt.label }}
                </label>
              </div>
              <!-- File -->
              <input v-else-if="field.type==='file'" type="file" class="pv-input" />
              <!-- Email -->
              <input v-else-if="field.type==='email'" type="email" :placeholder="field.placeholder" :disabled="field.disabled" class="pv-input" />
              <!-- Phone -->
              <input v-else-if="field.type==='phone'" type="tel" :placeholder="field.placeholder" :disabled="field.disabled" class="pv-input" />
              <!-- Section -->
              <fieldset v-else-if="field.type==='section'" class="pv-fieldset"><legend class="pv-legend">{{ field.label }}</legend></fieldset>
            </div>
            <button type="submit" class="btn btn-primary pv-submit">提交</button>
          </form>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { api } from '@oa4rust/sdk'

interface FormField {
  id: string; type: string; label: string; key: string
  placeholder?: string; defaultValue?: string; required?: boolean; disabled?: boolean
  rows?: number; min?: number; max?: number; optionsStr?: string
}
interface FormDef {
  id?: string; name: string; flag: string; desc?: string
  fields: FormField[]; updatedAt?: string
}

const fieldTypes = [
  { type: 'text',     label: '文本',   icon: '📝' },
  { type: 'textarea', label: '多行文本', icon: '📄' },
  { type: 'number',   label: '数字',   icon: '🔢' },
  { type: 'date',     label: '日期',   icon: '📅' },
  { type: 'select',   label: '下拉',   icon: '📋' },
  { type: 'checkbox', label: '单选',   icon: '◉' },
  { type: 'checkbox_group', label: '多选', icon: '☑' },
  { type: 'file',     label: '文件',   icon: '📎' },
  { type: 'email',    label: '邮箱',   icon: '✉' },
  { type: 'phone',    label: '手机',   icon: '📱' },
]
const layoutTypes = [
  { type: 'section',  label: '分组', icon: '📁' },
  { type: 'row_start', label: '开始行', icon: '↔️' },
  { type: 'row_end',   label: '结束行', icon: '↩️' },
]

const mode = ref<'edit'|'preview'>('edit')
const forms = ref<FormDef[]>([])
const formsLoading = ref(false)
const listFilter = ref('')
const currentForm = ref<FormDef|null>(null)
const selectedField = ref<FormField|null>(null)
const draggedType = ref<string|null>(null)

const filteredForms = computed(() =>
  listFilter.value
    ? forms.value.filter(f => (f.name||'').toLowerCase().includes(listFilter.value.toLowerCase()) || (f.flag||'').toLowerCase().includes(listFilter.value.toLowerCase()))
    : forms.value
)

function onDragStart(e: DragEvent, ft: { type: string }) {
  draggedType.value = ft.type
  e.dataTransfer?.setData('text/plain', ft.type)
}
function onDrop(e: DragEvent) {
  e.preventDefault()
  const type = e.dataTransfer?.getData('text/plain') || draggedType.value
  if (type) addField({ type, label: '' })
  draggedType.value = null
}

function genId() { return 'f_' + Date.now() + '_' + Math.random().toString(36).slice(2,6) }

function makeField(type: string): FormField {
  const d: Record<string, Partial<FormField>> = {
    text: { label: '文本字段', key: 'text_field', placeholder: '请输入' },
    textarea: { label: '多行文本', key: 'textarea_field', placeholder: '请输入内容', rows: 4 },
    number: { label: '数字', key: 'number_field', placeholder: '请输入数字' },
    date: { label: '日期', key: 'date_field' },
    select: { label: '下拉选择', key: 'select_field', optionsStr: 'option1|选项1\noption2|选项2' },
    checkbox: { label: '单选框', key: 'checkbox_field' },
    checkbox_group: { label: '多选框', key: 'checkbox_group_field', optionsStr: 'a|A\nb|B\nc|C' },
    file: { label: '文件上传', key: 'file_field' },
    email: { label: '邮箱', key: 'email_field', placeholder: 'example@domain.com' },
    phone: { label: '手机号', key: 'phone_field', placeholder: '请输入手机号' },
    section: { label: '分组标题', key: '' },
  }
  return { id: genId(), type, ...d[type], required: false, disabled: false } as FormField
}

function addField(ft: { type: string; label?: string }) {
  if (!currentForm.value) { resetForm(); return }
  const field = makeField(ft.type)
  if (ft.label) field.label = ft.label
  currentForm.value.fields.push(field)
  selectedField.value = field
}
function removeField(i: number) {
  if (!currentForm.value) return
  currentForm.value.fields.splice(i, 1)
  if (selectedField.value) selectedField.value = null
}
function moveField(i: number, d: number) {
  if (!currentForm.value) return
  const n = i + d
  if (n < 0 || n >= currentForm.value.fields.length) return
  const fs = currentForm.value.fields
  ;[fs[i], fs[n]] = [fs[n], fs[i]]
}
function selectField(f: FormField) { selectedField.value = f }
function getFieldIcon(t: string) {
  return [...fieldTypes, ...layoutTypes].find(f => f.type === t)?.icon ?? '⬜'
}
function parseOptions(s?: string) {
  if (!s?.trim()) return []
  return s.split('\n').filter(Boolean).map(l => { const [v, lb = v] = l.split('|'); return { value: v.trim(), label: lb.trim() } })
}

async function loadForms() {
  formsLoading.value = true
  try { const r: any = await api.get('/jaxrs/form/list'); forms.value = r?.data ?? [] }
  catch { forms.value = [] } finally { formsLoading.value = false }
}
async function loadForm(f: FormDef) {
  try {
    const r: any = await api.get(`/jaxrs/form/${f.id}`)
    const data = r?.data ?? f
    currentForm.value = {
      id: data.id || f.id, name: data.name||data.title||'', flag: data.flag||data.formFlag||'',
      desc: data.description||data.desc||'',
      fields: Array.isArray(data.fields) ? data.fields.map((ff: any) => ({
        id: ff.id || genId(), type: ff.type||'text', label: ff.label||ff.name||'',
        key: ff.key||ff.fieldKey||'', placeholder: ff.placeholder||'',
        defaultValue: ff.defaultValue||ff.default||'', required: ff.required||false,
        disabled: ff.disabled||false, rows: ff.rows||4, min: ff.min, max: ff.max,
        optionsStr: ff.optionsStr || (Array.isArray(ff.options) ? ff.options.map((o:any)=>`${o.value}|${o.label}`).join('\n') : ''),
      })) : [],
      updatedAt: data.updatedAt||f.updatedAt,
    }
    selectedField.value = null
  } catch { currentForm.value = { ...f, fields: [] }; selectedField.value = null }
}
function resetForm() { currentForm.value = { name: '', flag: '', fields: [] }; selectedField.value = null }

async function saveForm() {
  if (!currentForm.value?.name.trim()) { alert('请输入表单名称'); return }
  try {
    const payload = {
      name: currentForm.value.name, flag: currentForm.value.flag,
      description: currentForm.value.desc,
      fields: currentForm.value.fields.map(f => ({
        type: f.type, label: f.label, key: f.key, placeholder: f.placeholder,
        defaultValue: f.defaultValue, required: f.required, disabled: f.disabled,
        rows: f.rows, min: f.min, max: f.max,
        options: parseOptions(f.optionsStr),
      })),
    }
    if (currentForm.value.id) await api.put(`/jaxrs/form/${currentForm.value.id}`, payload)
    else await api.post('/jaxrs/form', payload)
    await loadForms(); alert('保存成功')
  } catch (e: any) { alert('保存失败: ' + (e?.message ?? '')) }
}

function togglePreview() { mode.value = mode.value === 'preview' ? 'edit' : 'preview' }
onMounted(loadForms)
</script>

<style scoped>
.fd { display: flex; flex-direction: column; gap: 0; height: 100% }
.fd-header { display: flex; align-items: center; justify-content: space-between; padding: 12px 20px; flex-shrink: 0 }
.fd-title h1 { font-family: 'Orbitron', sans-serif; font-size: 18px; color: var(--color-primary); margin: 0 0 2px; text-shadow: 0 0 15px var(--color-primary-glow) }
.subtitle { font-size: 11px; color: var(--text-muted); margin: 0; font-family: 'JetBrains Mono', monospace }
.fd-actions { display: flex; gap: 8px }
.btn { padding: 6px 14px; border-radius: var(--radius-md); border: 1px solid var(--border-color); background: var(--bg-elevated); color: var(--text-primary); cursor: pointer; font-size: 13px }
.btn:hover { border-color: var(--color-primary); color: var(--color-primary) }
.btn:disabled { opacity: 0.4; cursor: not-allowed }
.btn-primary { background: var(--color-primary); color: #000; border-color: var(--color-primary); font-weight: 600 }
.btn-primary:hover { background: var(--color-primary-light) }
.btn-outline.active { background: var(--color-primary-soft); border-color: var(--color-primary); color: var(--color-primary) }
.fd-body { display: flex; flex: 1; gap: 0; min-height: 0; overflow: hidden }
/* List panel */
.fd-list-panel { width: 200px; flex-shrink: 0; display: flex; flex-direction: column; border-right: 1px solid var(--border-color) }
.list-header { display: flex; align-items: center; justify-content: space-between; padding: 10px 12px; border-bottom: 1px solid var(--border-color); font-size: 13px; font-weight: 600; color: var(--color-primary) }
.btn-sm { padding: 3px 8px; border-radius: var(--radius-sm); border: 1px solid var(--border-color); background: transparent; color: var(--text-muted); cursor: pointer; font-size: 11px }
.list-search { padding: 8px }
.list-input { width: 100%; padding: 5px 8px; border-radius: var(--radius-sm); border: 1px solid var(--border-color); background: var(--bg-elevated); color: var(--text-primary); font-size: 12px; outline: none; box-sizing: border-box }
.list-items { flex: 1; overflow-y: auto; padding: 4px }
.list-loading, .list-empty { padding: 16px; text-align: center; color: var(--text-muted); font-size: 12px }
.list-item { padding: 8px 10px; border-radius: var(--radius-sm); cursor: pointer; margin-bottom: 2px }
.list-item:hover { background: var(--bg-hover) }
.list-item.active { background: var(--color-primary-soft); border-left: 3px solid var(--color-primary) }
.li-name { font-size: 13px; color: var(--text-primary); overflow: hidden; text-overflow: ellipsis; white-space: nowrap }
.li-meta { font-size: 10px; color: var(--text-muted); font-family: 'JetBrains Mono', monospace; margin-top: 2px }
/* Center area */
.fd-center { display: flex; flex: 1; min-width: 0; overflow: hidden }
/* Palette */
.fd-palette { width: 180px; flex-shrink: 0; padding: 12px; border-right: 1px solid var(--border-color); overflow-y: auto }
.palette-title { font-size: 11px; color: var(--text-muted); text-transform: uppercase; letter-spacing: 1px; margin: 8px 0 6px; font-weight: 600 }
.palette-sep { height: 1px; background: var(--border-color); margin: 8px 0 }
.palette-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 6px }
.palette-item { display: flex; flex-direction: column; align-items: center; padding: 10px 4px; border-radius: var(--radius-md); border: 1px solid var(--border-color); cursor: grab; background: var(--bg-elevated); transition: all var(--transition-fast); user-select: none }
.palette-item:hover { border-color: var(--color-primary); background: var(--color-primary-soft); transform: translateY(-1px) }
.pi-icon { font-size: 20px }
.pi-label { font-size: 10px; color: var(--text-muted); margin-top: 4px; text-align: center }
/* Canvas */
.fd-canvas { flex: 1; padding: 16px; overflow-y: auto; min-width: 0 }
.canvas-empty { display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100%; gap: 16px; color: var(--text-muted) }
.ce-icon { font-size: 64px; opacity: 0.3 }
.canvas-form { display: flex; flex-direction: column; gap: 12px; max-width: 700px; margin: 0 auto; width: 100% }
.cf-header { display: flex; gap: 12px; padding-bottom: 12px; border-bottom: 1px solid var(--border-color) }
.cf-name, .cf-flag { padding: 8px 12px; border-radius: var(--radius-md); border: 1px solid var(--border-color); background: var(--bg-elevated); color: var(--text-primary); font-size: 14px; outline: none }
.cf-name { flex: 2; font-weight: 600 }
.cf-flag { flex: 1; font-family: 'JetBrains Mono', monospace; font-size: 12px; color: var(--color-secondary) }
.cf-flag:disabled { opacity: 0.5 }
.cf-fields { display: flex; flex-direction: column; gap: 4px; min-height: 100px; padding: 8px; border: 2px dashed var(--border-color); border-radius: var(--radius-md); transition: border-color 0.2s }
.fields-empty { padding: 24px; text-align: center; color: var(--text-muted); font-size: 13px }
.field-row { display: flex; align-items: center; gap: 8px; padding: 8px 12px; border-radius: var(--radius-md); border: 1px solid transparent; cursor: pointer; transition: all 0.15s; background: var(--bg-elevated) }
.field-row:hover { border-color: var(--border-color) }
.field-row.selected { border-color: var(--color-primary); background: var(--color-primary-soft) }
.fr-handle { color: var(--text-muted); cursor: grab; font-size: 14px; flex-shrink: 0 }
.fr-icon { font-size: 18px; flex-shrink: 0 }
.fr-info { flex: 1; min-width: 0 }
.fr-label { font-size: 13px; color: var(--text-primary); font-weight: 500; overflow: hidden; text-overflow: ellipsis; white-space: nowrap }
.fr-meta { font-size: 10px; color: var(--text-muted); font-family: 'JetBrains Mono', monospace; margin-top: 1px }
.fr-actions { display: flex; gap: 2px; flex-shrink: 0 }
.fa-btn { padding: 2px 6px; border-radius: var(--radius-sm); border: 1px solid var(--border-color); background: transparent; color: var(--text-muted); cursor: pointer; font-size: 11px }
.fa-btn:hover { border-color: var(--color-primary); color: var(--color-primary) }
.fa-btn:disabled { opacity: 0.3; cursor: not-allowed }
.fa-del:hover { border-color: var(--color-danger); color: var(--color-danger) }
/* Properties */
.fd-props { width: 260px; flex-shrink: 0; padding: 12px; border-left: 1px solid var(--border-color); overflow-y: auto }
.props-title { display: flex; align-items: center; justify-content: space-between; padding-bottom: 10px; border-bottom: 1px solid var(--border-color); margin-bottom: 12px }
.props-title span:first-child { font-size: 12px; font-weight: 600; color: var(--text-muted); text-transform: uppercase; letter-spacing: 1px }
.props-type { font-size: 11px; padding: 2px 8px; border-radius: var(--radius-sm); background: var(--color-primary-soft); color: var(--color-primary); font-family: 'JetBrains Mono', monospace }
.prop-group { margin-bottom: 12px }
.prop-group label { display: block; font-size: 11px; color: var(--text-muted); margin-bottom: 4px; text-transform: uppercase; letter-spacing: 0.5px }
.prop-input { width: 100%; padding: 7px 10px; border-radius: var(--radius-md); border: 1px solid var(--border-color); background: var(--bg-elevated); color: var(--text-primary); font-size: 13px; outline: none; box-sizing: border-box }
.prop-input:focus { border-color: var(--color-primary) }
.prop-textarea { width: 100%; padding: 7px 10px; border-radius: var(--radius-md); border: 1px solid var(--border-color); background: var(--bg-elevated); color: var(--text-primary); font-size: 12px; outline: none; resize: vertical; box-sizing: border-box; font-family: 'JetBrains Mono', monospace; min-height: 60px }
.prop-row { display: flex; gap: 16px }
.checkbox-label { display: flex; align-items: center; gap: 6px; font-size: 13px; color: var(--text-primary); text-transform: none; cursor: pointer }
.prop-row2 { display: flex; gap: 8px }
.prop-row2 .prop-input { flex: 1 }
/* Preview */
.preview-overlay { position: fixed; inset: 0; background: rgba(0,0,0,0.7); display: flex; align-items: center; justify-content: center; z-index: 200 }
.preview-modal { width: 560px; max-width: 90vw; max-height: 85vh; overflow: auto; padding: 24px; display: flex; flex-direction: column; gap: 16px }
.preview-header { display: flex; align-items: center; justify-content: space-between }
.preview-header h2 { font-size: 18px; color: var(--color-primary); margin: 0 }
.btn-close { padding: 4px 10px; border-radius: var(--radius-sm); border: 1px solid var(--border-color); background: transparent; color: var(--text-muted); cursor: pointer }
.pv-field { display: flex; flex-direction: column; gap: 4px; margin-bottom: 14px }
.pv-field label { font-size: 13px; color: var(--text-secondary); font-weight: 500 }
.req { color: var(--color-danger) }
.pv-input { width: 100%; padding: 9px 12px; border-radius: var(--radius-md); border: 1px solid var(--border-color); background: var(--bg-elevated); color: var(--text-primary); font-size: 14px; outline: none; box-sizing: border-box }
.pv-input:focus { border-color: var(--color-primary) }
.pv-input:disabled { opacity: 0.5 }
.pv-cb { margin-right: 6px }
.pv-cb-group { display: flex; flex-direction: column; gap: 6px }
.pv-cb-label { display: flex; align-items: center; font-size: 14px; color: var(--text-primary); cursor: pointer }
.pv-fieldset { border: 1px solid var(--border-color); border-radius: var(--radius-md); padding: 12px; margin-bottom: 14px }
.pv-legend { font-size: 13px; font-weight: 600; color: var(--color-primary); padding: 0 8px }
.pv-submit { margin-top: 8px; padding: 10px 24px; font-size: 14px }
</style>
