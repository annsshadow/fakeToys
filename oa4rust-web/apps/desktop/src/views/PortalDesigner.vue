<template>
  <div class="portal-designer">
    <div class="view-header glass-card">
      <h1>门户设计器</h1>
      <p class="subtitle">/jaxrs/portal/assemble/designer/* — 门户页面可视化设计</p>
      <button class="btn-create" @click="openCreate">+ 新建页面</button>
      <button class="btn-outline" @click="showImportExport=true">📤 导入导出</button>
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
              <button class="btn-preview" @click.stop="previewTargetPage=p;showPagePreview=true">预览</button>
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

    <!-- Script Editor Modal -->
    <div v-if="showScriptEditor" class="modal-overlay" @click.self="showScriptEditor=false">
      <div class="modal glass-card" style="width:640px">
        <h3>⚡ 脚本管理</h3>
        <div class="script-toolbar">
          <input v-model="scriptSearch" class="tmp-input" placeholder="搜索脚本..." />
          <button class="btn-sm" @click="openScriptEditor(null)">+ 新建脚本</button>
        </div>
        <div class="script-list-editor">
          <div v-for="(s,si) in filteredScripts" :key="s.id||si" class="script-editor-item">
            <div class="sei-head"><span class="sei-icon">⚡</span><span class="sei-name">{{s.name||s.scriptName||'未命名'}}</span><span class="sei-lang">{{s.language||'javascript'}}</span></div>
            <pre class="sei-code">{{(s.code||'').substring(0,100)}}{{(s.code||'').length>100?'...':''}}</pre>
            <div class="sei-actions">
              <button class="btn-sm" @click="editScript(s)">编辑</button>
              <button class="btn-sm" @click="runScript(s)">▶ 执行</button>
              <button class="btn-sm btn-del" @click="deleteScriptItem(si)">删除</button>
            </div>
          </div>
        </div>
        <div v-if="filteredScripts.length===0" class="tmpl-empty">暂无脚本</div>
      </div>
    </div>

    <!-- Script Item Editor -->
    <div v-if="showScriptItemEditor" class="modal-overlay" @click.self="showScriptItemEditor=false">
      <div class="modal glass-card">
        <h3>{{editingScript?'编辑脚本':'新建脚本'}}</h3>
        <div class="form-grid">
          <div class="form-group"><label>脚本名称</label><input v-model="scriptForm.name" class="form-input" placeholder="脚本名称" /></div>
          <div class="form-group"><label>语言</label>
            <select v-model="scriptForm.language" class="form-input">
              <option value="javascript">JavaScript</option><option value="typescript">TypeScript</option><option value="python">Python</option>
            </select>
          </div>
          <div class="form-group full-width"><label>代码</label>
            <textarea v-model="scriptForm.code" class="form-textarea code-area" rows="10" placeholder="// 脚本代码..."></textarea>
          </div>
          <div class="form-group full-width"><label>描述</label><input v-model="scriptForm.desc" class="form-input" placeholder="可选描述" /></div>
        </div>
        <div class="modal-actions">
          <button class="btn-cancel" @click="showScriptItemEditor=false">取消</button>
          <button class="btn-save" :disabled="!scriptForm.name" @click="saveScript">保存</button>
        </div>
      </div>
    </div>

    <!-- Widget Picker Modal -->
    <div v-if="showWidgetPicker" class="modal-overlay" @click.self="showWidgetPicker=false">
      <div class="modal glass-card" style="width:640px">
        <h3>🧩 组件库</h3>
        <div class="widget-picker-toolbar">
          <input v-model="widgetSearch" class="tmp-input" placeholder="搜索组件..." />
          <select v-model="widgetCat" class="tmp-select">
            <option value="all">全部分类</option>
            <option value="data">数据展示</option>
            <option value="chart">图表</option>
            <option value="form">表单</option>
            <option value="nav">导航</option>
            <option value="media">媒体</option>
          </select>
        </div>
        <div class="widget-picker-grid">
          <div v-for="(w,wi) in filteredWidgets" :key="w.type" class="widget-picker-card" @click="pickWidget(w)">
            <div class="wpc-icon">{{w.icon}}</div>
            <div class="wpc-name">{{w.name}}</div>
            <div class="wpc-cat">{{w.category}}</div>
            <div class="wpc-desc">{{w.desc}}</div>
          </div>
        </div>
      </div>
    </div>

    <!-- Import/Export Modal -->
    <div v-if="showImportExport" class="modal-overlay" @click.self="showImportExport=false">
      <div class="modal glass-card" style="width:480px">
        <h3>📤 导入/导出</h3>
        <div class="ie-tabs">
          <button :class="['ie-tab',{active:ieTab==='export'}]" @click="ieTab='export'">导出</button>
          <button :class="['ie-tab',{active:ieTab==='import'}]" @click="ieTab='import'">导入</button>
        </div>
        <div v-if="ieTab==='export'" class="ie-body">
          <div class="ie-option"><label>格式:</label>
            <select v-model="exportFmt" class="ie-select">
              <option value="json">JSON</option><option value="html">HTML</option>
            </select>
          </div>
          <div class="ie-count">{{pages.length}} 个页面待导出</div>
          <button class="bs" @click="doExport()">📥 导出</button>
        </div>
        <div v-if="ieTab==='import'" class="ie-body">
          <textarea v-model="importJson" class="ie-textarea" placeholder="粘贴JSON数据..."></textarea>
          <div v-if="importMsg" :class="['ie-msg',importMsg.ok?'ok':'err']">{{importMsg.txt}}</div>
          <button class="bs" @click="doImport()">📤 导入</button>
        </div>
        <div class="ie-footer"><button class="bc" @click="showImportExport=false">关闭</button></div>
      </div>
    </div>

    <!-- Page Preview Modal -->
    <div v-if="showPagePreview" class="modal-overlay" @click.self="showPagePreview=false">
      <div class="modal glass-card" style="width:80%;height:80%">
        <div class="preview-header"><span>👁 页面预览: {{previewTargetPage?.name||previewTargetPage?.pageName}}</span><button class="btn-close" @click="showPagePreview=false">✕</button></div>
        <div class="preview-content">
          <div v-if="previewTargetPage?.layout" class="preview-layout" v-html='renderLayout(previewTargetPage && previewTargetPage.layout ? previewTargetPage.layout : "")'></div>
          <div v-else class="preview-empty">暂无布局配置</div>
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

// --- Script Management ---
const showScriptEditor = ref(false)
const scriptSearch = ref("")
const filteredScripts = computed(() => {
  if (!scriptSearch.value.trim()) return scripts.value
  const q = scriptSearch.value.toLowerCase()
  return scripts.value.filter(s => (s.name||s.scriptName||'').toLowerCase().includes(q))
})
const showScriptItemEditor = ref(false)
const editingScript = ref<any>(null)
const scriptForm = ref({ name: '', language: 'javascript', code: '', desc: '' })
function openScriptEditor(s: any) {
  if (s) { editingScript.value = s; scriptForm.value = { name: s.name||s.scriptName||'', language: s.language||'javascript', code: s.code||'', desc: s.description||s.desc||'' } }
  else { editingScript.value = null; scriptForm.value = { name: '', language: 'javascript', code: '', desc: '' } }
  showScriptItemEditor.value = true
}
async function saveScript() {
  if (!scriptForm.value.name.trim()) { alert('请输入脚本名称'); return }
  try {
    const data = { name: scriptForm.value.name, language: scriptForm.value.language, code: scriptForm.value.code, description: scriptForm.value.desc }
    if (editingScript.value?.id) {
      await api.put(`/jaxrs/portal/assemble/designer/script/${editingScript.value.id}`, data)
    } else {
      await api.post('/jaxrs/portal/assemble/designer/script', data)
    }
    showScriptItemEditor.value = false
    loadScripts()
  } catch (e: any) { alert('保存失败: ' + (e?.message ?? '')) }
}
async function deleteScriptItem(idx: number) {
  if (!confirm('确定删除此脚本？')) return
  const s = scripts.value[idx]
  if (s?.id) { try { await api.delete(`/jaxrs/portal/assemble/designer/script/${s.id}`) } catch {} }
  scripts.value.splice(idx, 1)
}
async function runScript(s: any) {
  if (!s?.code) { alert('脚本无代码内容'); return }
  try {
    const r = await api.post('/jaxrs/portal/assemble/designer/script/run', { id: s.id, code: s.code })
    alert('执行结果: ' + JSON.stringify(r?.data ?? '未知'))
  } catch (e: any) { alert('执行失败: ' + (e?.message ?? '')) }
}

// --- Widget Picker ---
const showWidgetPicker = ref(false)
const widgetSearch = ref("")
const widgetCat = ref("all")
const allWidgets = ref<Array<{type:string;name:string;icon:string;desc:string;category:string}>>([
  {type:'text',name:'文本',icon:'📝',desc:'富文本展示',category:'data'},
  {type:'chart',name:'图表',icon:'📊',desc:'数据可视化',category:'chart'},
  {type:'table',name:'表格',icon:'📋',desc:'数据列表',category:'data'},
  {type:'calendar',name:'日历',icon:'📅',desc:'日程展示',category:'nav'},
  {type:'todo',name:'待办',icon:'✅',desc:'任务列表',category:'form'},
  {type:'news',name:'新闻',icon:'📰',desc:'资讯展示',category:'media'},
  {type:'map',name:'地图',icon:'🗺️',desc:'地理信息',category:'media'},
  {type:'counter',name:'计数器',icon:'🔢',desc:'数字统计',category:'data'},
  {type:'clock',name:'时钟',icon:'🕐',desc:'时间显示',category:'media'},
  {type:'weather',name:'天气',icon:'🌤️',desc:'天气信息',category:'media'},
  {type:'tree',name:'树形',icon:'🌲',desc:'层级结构',category:'data'},
  {type:'tabs',name:'标签页',icon:'📑',desc:'内容分组',category:'nav'},
  {type:'carousel',name:'轮播',icon:'🎠',desc:'图片轮播',category:'media'},
  {type:'form',name:'表单',icon:'📝',desc:'数据录入',category:'form'},
  {type:'upload',name:'上传',icon:'📤',desc:'文件上传',category:'form'},
  {type:'dialog',name:'对话框',icon:'💬',desc:'交互弹窗',category:'form'},
])
const filteredWidgets = computed(() => {
  let list = allWidgets.value
  if (widgetSearch.value) { const q = widgetSearch.value.toLowerCase(); list = list.filter(w => w.name.toLowerCase().includes(q) || w.desc.toLowerCase().includes(q)) }
  if (widgetCat.value !== 'all') list = list.filter(w => w.category === widgetCat.value)
  return list
})
function pickWidget(w: any) {
  if (editingPage.value) {
    alert(`已添加组件「${w.name}」到当前页面`)
  } else {
    alert('请先选择一个页面进行编辑')
  }
  showWidgetPicker.value = false
}

// --- Import/Export ---
const showImportExport = ref(false)
const ieTab = ref<'export'|'import'>('export')
const exportFmt = ref<'json'|'html'>('json')
const importJson = ref("")
const importMsg = ref<{ok:boolean;txt:string}|null>(null)
function doExport() {
  const data = pages.value.map(p => ({name:p.name||p.pageName,flag:p.flag||p.pageFlag,layout:p.layout,description:p.description||p.desc}))
  if (exportFmt.value==='json') {
    downloadBlob(new Blob([JSON.stringify(data,null,2)],{type:'application/json'}), 'portals_'+new Date().toISOString().slice(0,10)+'.json')
  } else {
    downloadBlob(new Blob([JSON.stringify(data,null,2)],{type:'text/html'}), 'portals_'+new Date().toISOString().slice(0,10)+'.json')
  }
  showImportExport.value = false
}
function downloadBlob(blob: Blob, filename: string) {
  const a = document.createElement('a'); a.href = URL.createObjectURL(blob); a.download = filename; a.click()
}
async function doImport() {
  if (!importJson.value.trim()) return
  try {
    const data = JSON.parse(importJson.value)
    if (!Array.isArray(data)) { importMsg.value={ok:false,txt:'格式错误'}; return }
    for (const p of data) {
      try { await api.post('/jaxrs/portal/assemble/designer/page', p) } catch {}
    }
    importMsg.value={ok:true,txt:`成功导入 ${data.length} 个页面`}
    loadPages()
    showImportExport.value = false
  } catch(e: any) { importMsg.value={ok:false,txt:'导入失败: '+e.message} }
}

// --- Page Preview ---
const showPagePreview = ref(false)
const previewTargetPage = ref<any>(null)
function renderLayout(layout: string): string { try { const nodes = JSON.parse(layout); return nodes.map((n: any) => `<div style="padding:12px;border:1px dashed var(--border-subtle);margin:4px;border-radius:var(--radius-sm)">${n.type||"block"}</div>`).join("") } catch { return "<div style=\"padding:12px;color:var(--text-muted)\">解析布局失败</div>" } }






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

/* Script management */
.script-toolbar{display:flex;gap:8px;padding:8px 12px;border-bottom:1px solid var(--border-subtle)}
.script-list-editor{padding:12px;max-height:300px;overflow-y:auto;display:flex;flex-direction:column;gap:8px}
.script-editor-item{background:rgba(255,255,255,0.02);border:1px solid var(--border-subtle);border-radius:var(--radius-md);overflow:hidden}
.sei-head{display:flex;align-items:center;gap:8px;padding:8px 12px;background:rgba(245,158,11,0.08);border-bottom:1px solid var(--border-subtle)}
.sei-icon{font-size:16px}.sei-name{flex:1;color:var(--text-primary);font-size:13px;font-weight:500}
.sei-lang{font-size:10px;padding:1px 6px;border-radius:10px;background:rgba(245,158,11,0.15);color:#f59e0b}
.sei-code{margin:0;padding:8px 12px;background:rgba(0,0,0,0.3);color:#7fdbca;font-size:11px;font-family:monospace;white-space:pre-wrap;word-break:break-all;max-height:60px;overflow-y:auto}
.sei-actions{display:flex;gap:6px;padding:8px 12px;border-top:1px solid var(--border-subtle)}
/* Script item editor */
.code-area{font-family:'JetBrains Mono',monospace;font-size:12px}
/* Widget picker */
.widget-picker-toolbar{display:flex;gap:8px;padding:8px 12px;border-bottom:1px solid var(--border-subtle)}
.widget-picker-grid{padding:12px;display:grid;grid-template-columns:repeat(auto-fill,minmax(120px,1fr));gap:8px;max-height:300px;overflow-y:auto}
.widget-picker-card{padding:12px;text-align:center;cursor:pointer;border:1px solid var(--border-subtle);border-radius:var(--radius-md);background:var(--bg-elevated);transition:all var(--transition-fast)}
.widget-picker-card:hover{border-color:var(--color-primary);transform:translateY(-2px);box-shadow:var(--shadow-glow)}
.wpc-icon{font-size:28px;margin-bottom:6px}.wpc-name{font-size:12px;font-weight:600;color:var(--text-primary)}.wpc-cat{font-size:10px;color:var(--color-primary);margin-top:2px}.wpc-desc{font-size:10px;color:var(--text-muted);margin-top:2px}
/* Import/Export */
.ie-tabs{display:flex;gap:4px;padding:8px 12px;border-bottom:1px solid var(--border-subtle)}.ie-tab{padding:4px 12px;border-radius:var(--radius-sm);border:1px solid var(--border-subtle);background:transparent;color:var(--text-muted);cursor:pointer;font-size:11px}.ie-tab.active{background:var(--color-primary);color:#000;border-color:var(--color-primary)}.ie-body{padding:12px;display:flex;flex-direction:column;gap:10px}.ie-option{display:flex;align-items:center;gap:8px;font-size:12px}.ie-select{padding:5px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-subtle);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}.ie-count{font-size:11px;color:var(--text-muted)}.ie-textarea{width:100%;height:120px;background:rgba(0,0,0,0.3);border:1px solid var(--border-subtle);color:#7fdbca;font-family:monospace;font-size:11px;padding:10px;resize:vertical;outline:none;border-radius:var(--radius-sm)}.ie-msg{padding:8px;border-radius:var(--radius-sm);font-size:12px}.ie-msg.ok{background:rgba(16,185,129,0.1);color:#10b981}.ie-msg.err{background:rgba(239,68,68,0.1);color:#ef4444}.ie-footer{display:flex;justify-content:flex-end;padding-top:8px;border-top:1px solid var(--border-subtle);margin-top:8px}
/* Page preview */
.preview-header{display:flex;align-items:center;justify-content:space-between;padding:12px 16px;border-bottom:1px solid var(--border-subtle)}
.preview-header span{font-size:14px;font-weight:600;color:var(--color-primary)}
.preview-content{flex:1;overflow:auto;padding:16px}
.preview-layout{display:flex;flex-direction:column;gap:8px}
.preview-empty{color:var(--text-muted);font-size:13px;text-align:center;padding:40px}
.btn-close{background:none;border:none;font-size:18px;cursor:pointer;color:var(--text-muted)}
.btn-close:hover{color:var(--color-primary)}
/* Button outline style */
.btn-outline{padding:8px 16px;background:transparent;border:1px solid var(--border-subtle);color:var(--text-secondary);border-radius:var(--radius-md);cursor:pointer;font-size:13px;margin-left:8px}
.btn-outline:hover{border-color:var(--color-primary);color:var(--color-primary)}
/* Toolbar responsive */
.toolbar{padding:12px 16px;flex-shrink:0}
.tabs{display:flex;gap:8px}
/* Content panel enhancement */
.content-panel{flex:1;overflow-y:auto;padding:16px}
/* Scrollbar polish */
.script-list-editor::-webkit-scrollbar,.widget-picker-grid::-webkit-scrollbar,.ie-body::-webkit-scrollbar{width:4px}
.script-list-editor::-webkit-scrollbar-thumb,.widget-picker-grid::-webkit-scrollbar-thumb,.ie-body::-webkit-scrollbar-thumb{background:var(--border-subtle);border-radius:2px}
/* Page card hover enhancement */
.page-card{transition:all var(--transition-fast)}
.page-card:hover{transform:translateY(-2px)}
/* Loading state */
.loading-state{display:flex;flex-direction:column;align-items:center;justify-content:center;padding:60px;color:var(--text-muted);gap:12px}
.sk{height:40px;border-radius:var(--radius-md);background:var(--bg-elevated);animation:pulse 1.2s ease-in-out infinite}
@keyframes pulse{0%,100%{opacity:.4}50%{opacity:.8}}
.empty-state{display:flex;flex-direction:column;align-items:center;justify-content:center;padding:60px;color:var(--text-muted);gap:12px}
.ei{font-size:48px;opacity:0.4}
</style>

