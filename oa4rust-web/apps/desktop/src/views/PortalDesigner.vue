<template>
  <div class="portal-designer">
    <div class="view-header glass-card">
      <h1>门户设计器</h1>
      <p class="subtitle">/jaxrs/portal/assemble/designer/* — 门户页面可视化设计</p>
      <button class="btn-create" @click="openCreate">+ 新建页面</button>
    </div>

    <div class="toolbar glass-card">
      <div class="tabs">
        <button :class="{active:tab==='pages'}" @click="tab='pages'">页面管理</button>
        <button :class="{active:tab==='widgets'}" @click="tab='widgets'">组件库</button>
        <button :class="{active:tab==='scripts'}" @click="tab='scripts'">脚本管理</button>
      </div>
    </div>

    <!-- Pages tab -->
    <div v-if="tab==='pages'" class="content-panel glass-card">
      <div v-if="pLoading" class="loading-state"><div class="sk" v-for="i in 5" :key="i"></div></div>
      <div v-else-if="pages.length===0" class="empty-state"><div class="ei">🏠</div><p>暂无门户页面</p></div>
      <div v-else class="page-grid">
        <div v-for="p in pages" :key="p.id" class="page-card glass-card" @click="editPage(p)">
          <div class="pc-top">
            <span class="pc-icon">🏠</span>
            <div class="pc-info">
              <div class="pc-title">{{ p.name || p.pageName || '未命名页面' }}</div>
              <div class="pc-flag">flag: {{ p.flag || p.pageFlag || p.id }}</div>
            </div>
          </div>
          <div class="pc-desc">{{ p.description || p.desc || '暂无描述' }}</div>
          <div class="pc-footer">
            <span class="pc-time">{{ fmtTime(p.updatedAt) }}</span>
            <div class="pc-actions">
              <button class="btn-preview" @click.stop="previewPage(p)">预览</button>
              <button class="btn-edit" @click.stop="editPage(p)">编辑</button>
              <button class="btn-delete" @click.stop="deletePage(p)">删除</button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Widgets tab -->
    <div v-if="tab==='widgets'" class="content-panel glass-card">
      <div class="widget-grid">
        <div v-for="w in widgets" :key="w.type" class="widget-card glass-card" @click="addWidget(w)">
          <div class="wc-icon">{{ w.icon }}</div>
          <div class="wc-name">{{ w.name }}</div>
          <div class="wc-desc">{{ w.desc }}</div>
        </div>
      </div>
    </div>

    <!-- Scripts tab -->
    <div v-if="tab==='scripts'" class="content-panel glass-card">
      <div v-if="sLoading" class="loading-state"><div class="sk" v-for="i in 4" :key="i"></div></div>
      <div v-else-if="scripts.length===0" class="empty-state"><div class="ei">⚡</div><p>暂无脚本</p></div>
      <div v-else class="script-list">
        <div v-for="s in scripts" :key="s.id" class="script-item glass-card">
          <div class="si-icon">⚡</div>
          <div class="si-info">
            <div class="si-name">{{ s.name || s.scriptName || '未命名脚本' }}</div>
            <div class="si-code font-mono">{{ (s.code || '').slice(0, 80) }}{{ (s.code || '').length > 80 ? '...' : '' }}</div>
          </div>
          <button class="btn-edit" @click="editScript(s)">编辑</button>
        </div>
      </div>
    </div>

    <!-- Page Edit Modal -->
    <div v-if="showModal" class="modal-overlay" @click.self="showModal=false">
      <div class="modal glass-card">
        <h3>{{ editingPage ? '编辑页面' : '新建页面' }}</h3>
        <div class="form-grid">
          <div class="form-group">
            <label>页面名称</label>
            <input v-model="form.name" class="form-input" placeholder="页面名称" />
          </div>
          <div class="form-group">
            <label>Flag</label>
            <input v-model="form.flag" class="form-input" placeholder="唯一标识" :disabled="!!editingPage" />
          </div>
          <div class="form-group full-width">
            <label>布局配置（JSON）</label>
            <textarea v-model="form.layout" class="form-textarea code-area" rows="8" placeholder='[{"type":"row","widgets":[...]}]'></textarea>
          </div>
          <div class="form-group full-width">
            <label>描述</label>
            <textarea v-model="form.desc" class="form-textarea" rows="2"></textarea>
          </div>
        </div>
        <div class="modal-actions">
          <button class="btn-cancel" @click="showModal=false">取消</button>
          <button class="btn-save" :disabled="saving" @click="savePage">{{ saving ? '保存中...' : '保存' }}</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { api } from '@oa4rust/sdk'

type PageDef = {
  id?: string
  name?: string
  pageName?: string
  flag?: string
  pageFlag?: string
  description?: string
  desc?: string
  layout?: string
  updatedAt?: string
}

type WidgetDef = { type: string; name: string; icon: string; desc: string }
type ScriptDef = { id?: string; name?: string; scriptName?: string; code?: string }

type Tab = 'pages' | 'widgets' | 'scripts'

const tab = ref<Tab>('pages')
const pLoading = ref(false)
const sLoading = ref(false)
const pages = ref<PageDef[]>([])
const scripts = ref<ScriptDef[]>([])
const showModal = ref(false)
const editingPage = ref<PageDef | null>(null)
const saving = ref(false)
const form = ref({ name: '', flag: '', layout: '', desc: '' })

const widgets: WidgetDef[] = [
  { type: 'text', name: '文本', icon: '📝', desc: '富文本展示' },
  { type: 'chart', name: '图表', icon: '📊', desc: '数据可视化' },
  { type: 'table', name: '表格', icon: '📋', desc: '数据列表' },
  { type: 'calendar', name: '日历', icon: '📅', desc: '日程展示' },
  { type: 'todo', name: '待办', icon: '✅', desc: '任务列表' },
  { type: 'news', name: '新闻', icon: '📰', desc: '资讯展示' },
  { type: 'map', name: '地图', icon: '🗺️', desc: '地理信息' },
  { type: 'counter', name: '计数器', icon: '🔢', desc: '数字统计' },
]

function fmtTime(t?: string) {
  return t ? new Date(t).toLocaleString('zh-CN', { month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit' }) : '-'
}

async function loadPages() {
  pLoading.value = true
  try {
    const r = await api.get('/jaxrs/portal/assemble/designer/page/list')
    pages.value = r.data ?? []
  } catch { pages.value = [] } finally { pLoading.value = false }
}

async function loadScripts() {
  sLoading.value = true
  try {
    const r = await api.get('/jaxrs/portal/assemble/designer/script/list')
    scripts.value = r.data ?? []
  } catch { scripts.value = [] } finally { sLoading.value = false }
}

function openCreate() {
  editingPage.value = null
  form.value = { name: '', flag: '', layout: '', desc: '' }
  showModal.value = true
}

function editPage(p: PageDef) {
  editingPage.value = p
  form.value = {
    name: p.name || p.pageName || '',
    flag: p.flag || p.pageFlag || '',
    layout: p.layout || '[{"type":"row","widgets":[]}]',
    desc: p.description || p.desc || '',
  }
  showModal.value = true
}

function previewPage(p: PageDef) {
  // Open in new tab or show preview
  if (p.flag) window.open(`/app/portal?page=${p.flag}`, '_blank')
}

function addWidget(w: WidgetDef) {
  alert(`已添加组件「${w.name}」到当前页面（需要页面编辑模式）`)
}

function editScript(s: ScriptDef) {
  alert(`编辑脚本「${s.name}」— 功能开发中`)
}

async function savePage() {
  if (!form.value.name.trim()) { alert('请输入页面名称'); return }
  saving.value = true
  try {
    const data = {
      name: form.value.name,
      flag: form.value.flag,
      layout: form.value.layout,
      description: form.value.desc,
    }
    if (editingPage.value?.id) {
      await api.put(`/jaxrs/portal/assemble/designer/page/${editingPage.value.id}`, data)
    } else {
      await api.post('/jaxrs/portal/assemble/designer/page', data)
    }
    showModal.value = false
    loadPages()
  } catch (e: any) { alert('保存失败: ' + (e?.message ?? '')) } finally { saving.value = false }
}

async function deletePage(p: PageDef) {
  if (!confirm(`删除页面「${p.name || p.flag}」？`)) return
  try {
    await api.delete(`/jaxrs/portal/assemble/designer/page/${p.id}`)
    pages.value = pages.value.filter(x => x.id !== p.id)
  } catch (e: any) { alert('删除失败: ' + (e?.message ?? '')) }
}

// Watch tab changes
import { watch } from 'vue'
watch(tab, (t) => {
  if (t === 'pages') loadPages()
  else if (t === 'scripts') loadScripts()
})

loadPages()
</script>

<style scoped>
.portal-designer { display: flex; flex-direction: column; gap: 16px; height: 100% }
.view-header { display: flex; align-items: center; justify-content: space-between; padding: 16px 24px }
.view-header h1 { font-family: 'Orbitron', sans-serif; font-size: 20px; color: var(--color-primary); margin: 0; text-shadow: 0 0 15px var(--color-primary-glow) }
.subtitle { font-size: 12px; color: var(--text-muted); margin: 4px 0 0; font-family: 'JetBrains Mono', monospace }
.btn-create { padding: 8px 20px; background: var(--color-accent); color: #fff; border: none; border-radius: var(--radius-md); font-size: 13px; cursor: pointer; font-weight: 600 }
.toolbar { padding: 12px 16px }
.tabs { display: flex; gap: 8px }
.tabs button { padding: 8px 20px; background: var(--bg-elevated); border: 1px solid var(--border-subtle); border-radius: var(--radius-md); color: var(--text-secondary); font-size: 13px; cursor: pointer }
.tabs button.active { background: var(--color-primary); color: #000; border-color: var(--color-primary); font-weight: 600 }
.content-panel { flex: 1; overflow-y: auto; padding: 16px }
.page-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(240px, 1fr)); gap: 12px }
.page-card { padding: 14px; cursor: pointer; transition: all var(--transition-fast); border: 1px solid var(--border-subtle); border-radius: var(--radius-lg); background: var(--bg-elevated) }
.page-card:hover { border-color: var(--color-primary); transform: translateY(-2px); box-shadow: var(--shadow-glow) }
.pc-top { display: flex; align-items: center; gap: 10px; margin-bottom: 8px }
.pc-icon { font-size: 22px }
.pc-info { flex: 1; min-width: 0 }
.pc-title { font-size: 14px; font-weight: 600; color: var(--text-primary); overflow: hidden; text-overflow: ellipsis; white-space: nowrap }
.pc-flag { font-size: 11px; color: var(--color-primary-deep); font-family: 'JetBrains Mono', monospace; margin-top: 2px }
.pc-desc { font-size: 12px; color: var(--text-muted); margin-bottom: 10px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap }
.pc-footer { display: flex; justify-content: space-between; align-items: center }
.pc-time { font-size: 11px; color: var(--text-muted) }
.pc-actions { display: flex; gap: 6px }
.btn-preview { padding: 3px 8px; background: transparent; border: 1px solid var(--color-success); color: var(--color-success); border-radius: var(--radius-sm); font-size: 11px; cursor: pointer }
.btn-preview:hover { background: var(--color-success); color: #000 }
.btn-edit { padding: 3px 8px; background: transparent; border: 1px solid var(--color-primary); color: var(--color-primary); border-radius: var(--radius-sm); font-size: 11px; cursor: pointer }
.btn-edit:hover { background: var(--color-primary); color: #000 }
.btn-delete { padding: 3px 8px; background: transparent; border: 1px solid var(--color-error); color: var(--color-error); border-radius: var(--radius-sm); font-size: 11px; cursor: pointer }
.btn-delete:hover { background: var(--color-error); color: #fff }
.widget-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(140px, 1fr)); gap: 12px }
.widget-card { padding: 16px; text-align: center; cursor: pointer; transition: all var(--transition-fast); border: 1px solid var(--border-subtle); border-radius: var(--radius-lg); background: var(--bg-elevated) }
.widget-card:hover { border-color: var(--color-primary); transform: translateY(-2px); box-shadow: var(--shadow-glow) }
.wc-icon { font-size: 32px; margin-bottom: 8px }
.wc-name { font-size: 14px; font-weight: 600; color: var(--text-primary) }
.wc-desc { font-size: 11px; color: var(--text-muted); margin-top: 4px }
.script-list { display: flex; flex-direction: column; gap: 8px }
.script-item { display: flex; align-items: center; gap: 12px; padding: 12px 16px; border: 1px solid var(--border-subtle); border-radius: var(--radius-md); background: var(--bg-elevated) }
.si-icon { font-size: 24px; flex-shrink: 0 }
.si-info { flex: 1; min-width: 0 }
.si-name { font-size: 14px; font-weight: 500; color: var(--text-primary) }
.si-code { font-size: 11px; color: var(--text-muted); margin-top: 2px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap }
.loading-state, .empty-state { display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 60px; color: var(--text-muted); gap: 12px }
.sk { height: 40px; border-radius: var(--radius-md); background: var(--bg-elevated); animation: pulse 1.2s ease-in-out infinite }
@keyframes pulse { 0%,100%{opacity:.4}50%{opacity:.8} }
.ei { font-size: 48px; opacity: 0.4 }
.modal-overlay { position: fixed; inset: 0; background: rgba(0,0,0,.7); display: flex; align-items: center; justify-content: center; z-index: 100 }
.modal { background: var(--bg-surface); border: 1px solid var(--border-subtle); border-radius: var(--radius-lg); padding: 24px; width: 560px; max-width: 90vw; max-height: 85vh; overflow: auto; display: flex; flex-direction: column; gap: 16px }
.modal h3 { font-family: 'Orbitron', sans-serif; color: var(--color-primary); margin: 0; font-size: 16px }
.form-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 12px }
.form-group { display: flex; flex-direction: column; gap: 6px }
.form-group.full-width { grid-column: span 2 }
.form-group label { font-size: 13px; color: var(--text-muted) }
.form-input, .form-textarea { background: var(--bg-elevated); border: 1px solid var(--border-subtle); border-radius: var(--radius-md); color: var(--text-primary); padding: 10px 12px; font-size: 14px }
.form-input:focus, .form-textarea:focus { outline: none; border-color: var(--color-primary) }
.form-textarea { resize: vertical }
.code-area { font-family: 'JetBrains Mono', monospace; font-size: 12px }
.modal-actions { display: flex; justify-content: flex-end; gap: 8px }
.btn-cancel { padding: 8px 20px; background: transparent; border: 1px solid var(--border-subtle); color: var(--text-secondary); border-radius: var(--radius-md); cursor: pointer }
.btn-save { padding: 8px 20px; background: var(--color-primary); color: #000; border: none; border-radius: var(--radius-md); font-size: 13px; cursor: pointer; font-weight: 600 }
.btn-save:disabled { opacity: 0.5; cursor: not-allowed }
.font-mono { font-family: 'JetBrains Mono', monospace }
@media(max-width:768px){.form-grid{grid-template-columns:1fr}.form-group.full-width{grid-column:span 1}}
</style>
