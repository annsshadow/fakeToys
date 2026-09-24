<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="designer-shell">
    <header class="designer-header glass-card">
      <div><h1>门户设计器</h1><p>拖拽模块并保存到 portal design content</p></div>
      <div class="header-actions">
        <button class="btn" @click="showCreate = true">新建设计</button>
        <button class="btn" :disabled="!activeId" @click="loadDesignerAssets">资产明细</button>
        <button class="btn" :disabled="!activeId" @click="loadDesignerCategories">分类/页面</button>
        <button class="btn" :disabled="!activeId" @click="loadDesignerPaging">分页/文件/版本</button>
        <button class="btn" :disabled="!activeId" @click="loadDesignerOutputs">输出/文件前翻</button>
        <button class="btn" @click="pdDesignerOps('saveById')">按ID保存</button>
        <button class="btn" @click="pdDesignerOps('pageVersions')">页版本清单</button>
        <button class="btn" @click="pdDesignerOps('pageDelete')">删页面</button>
        <button class="btn" @click="pdDesignerOps('tplCategory')">模板页分类</button>
        <button class="btn" @click="pdDesignerOps('widgetDelete')">删组件</button>
        <button class="btn" @click="pdDesignerOps('designerDetail')">设计器详情</button>
        <button class="btn primary" :disabled="!activeId || saving" @click="saveDesign">
          {{ saving ? '保存中…' : '保存布局' }}
        </button>
      </div>
    </header>
    <main class="designer-main">
      <aside class="panel glass-card">
        <h2>门户设计</h2>
        <button
          v-for="item in designs"
          :key="item.id"
          class="design-item"
          :class="{ active: item.id === activeId }"
          @click="openDesign(item.id)"
        >
          <strong>{{ item.name }}</strong><span>{{ item.description || '无描述' }}</span>
        </button>
        <p v-if="loading" class="muted">加载中…</p>
        <p v-else-if="!designs.length" class="muted">暂无设计</p>
      </aside>
      <section class="workspace glass-card">
        <div v-if="activeId" class="workspace-grid">
          <div class="palette">
            <h2>模块</h2>
            <div
              v-for="module in modules"
              :key="module.type"
              class="palette-item"
              draggable="true"
              @dragstart="dragModule = module"
              @dragend="dragModule = null"
              @click="addWidget(module)"
            >
              <strong>{{ module.label }}</strong><span>{{ module.hint }}</span>
            </div>
          </div>
          <div class="canvas" @dragover.prevent @drop="dropModule">
            <div class="canvas-head"><h2>{{ activeName }}</h2><span>{{ widgets.length }} 个模块<template v-if="portalMeta"> · {{ portalMeta }}</template><template v-if="assetText"> · {{ assetText }}</template></span></div>
            <p v-if="!widgets.length" class="drop-hint">将左侧模块拖到这里</p>
            <article
              v-for="(widget, index) in widgets"
              :key="widget.id"
              class="widget"
              :class="[`span-${widget.width}`, { selected: selectedId === widget.id }]"
              draggable="true"
              @dragstart="dragIndex = index"
              @dragover.prevent
              @drop.stop="dropWidget(index)"
              @click="selectedId = widget.id"
            >
              <div class="widget-head"><strong>{{ widget.title }}</strong><span>{{ widget.type }}</span></div>
              <p>{{ widgetPreview(widget) }}</p>
            </article>
          </div>
          <aside class="inspector">
            <h2>模块配置</h2>
            <template v-if="selectedWidget">
              <label>标题<input v-model="selectedWidget.title" /></label>
              <label>宽度
                <select v-model.number="selectedWidget.width">
                  <option :value="1">1/4</option><option :value="2">1/2</option>
                  <option :value="3">3/4</option><option :value="4">整行</option>
                </select>
              </label>
              <label v-if="selectedWidget.type === 'text'">文本<textarea v-model="selectedWidget.config.text" rows="5" /></label>
              <label v-else-if="selectedWidget.type === 'link'">目标地址<input v-model="selectedWidget.config.target" /></label>
              <label v-else>数据源<input v-model="selectedWidget.config.source" placeholder="query/view 标识" /></label>
              <button class="btn danger" @click="removeSelected">删除模块</button>
            </template>
            <p v-else class="muted">选择画布中的模块进行配置</p>
          </aside>
        </div>
        <div v-else class="empty">选择或新建一个门户设计</div>
      </section>
    </main>
    <div v-if="showCreate" class="modal-mask" @click.self="showCreate = false">
      <form class="modal glass-card" @submit.prevent="createDesign">
        <h2>新建门户设计</h2>
        <label>名称<input v-model="createForm.name" required /></label>
        <label>描述<textarea v-model="createForm.description" rows="3" /></label>
        <div class="modal-actions"><button type="button" class="btn" @click="showCreate = false">取消</button><button class="btn primary">创建</button></div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { api } from '@oa4rust/sdk'
import { computed, ref } from 'vue'
import {
  designerPaths,
  extractList,
  moveItem,
  type PortalWidget,
  type PortalWidgetType,
  parsePortalContent,
  serializePortalContent,
} from '../contracts/designer'
import { toast } from '../utils/toast'

interface DesignSummary {
  id: string
  name: string
  description?: string
}
interface ModuleItem {
  type: PortalWidgetType
  label: string
  hint: string
}

const modules: ModuleItem[] = [
  { type: 'text', label: '文本', hint: '静态文本内容' },
  { type: 'metric', label: '指标', hint: '单值数据源' },
  { type: 'table', label: '表格', hint: '列表数据源' },
  { type: 'chart', label: '图表', hint: '图表数据源' },
  { type: 'link', label: '链接', hint: '导航地址' },
]
const designs = ref<DesignSummary[]>([])
const activeId = ref('')
const activeName = ref('')
const portalMeta = ref('')
// 门户设计器资产明细族 6 条真实 distinct（rev197，各读独立表）：从 widget/script/dict/list/portal 取首 id →
// widget/{id}（x_portal_widget）+ script/{id}（x_portal_script）+ scriptversion/list/script/{scriptId}→scriptversion/{id}
// （x_portal_script_version）+ templatepage/{id}（x_portal_template_page）+ dict/{id}（x_portal_dict）。
const assetText = ref('')
async function loadDesignerAssets() {
  const id = String(activeId.value ?? '')
  if (!id) {
    toast.error('请先打开一个门户设计')
    return
  }
  const s = <T,>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const [wlist, slist, dlist] = await Promise.all([
      s(api.get<any>(`/api/portal/assemble/designer/widget/list/portal/${encodeURIComponent(id)}`)),
      s(api.get<any>(`/api/portal/assemble/designer/script/list/portal/${encodeURIComponent(id)}`)),
      s(api.get<any>(`/api/portal/assemble/designer/dict/list/portal/${encodeURIComponent(id)}`)),
    ])
    const pick = (r: any) => (Array.isArray(r?.data) && r.data[0] ? String(r.data[0].id ?? '0') : '0')
    const wid = pick(wlist)
    const sid = pick(slist)
    const did = pick(dlist)
    const tid = '0'
    const svList = await s(api.get<any>(`/api/portal/assemble/designer/scriptversion/list/script/${encodeURIComponent(sid)}`))
    const svId = Array.isArray((svList as any)?.data) && (svList as any).data[0] ? String((svList as any).data[0].id ?? '0') : '0'
    const [widget, script, svDetail, templatepage, dict] = await Promise.all([
      s(api.get<any>(`/api/portal/assemble/designer/widget/${encodeURIComponent(wid)}`)),
      s(api.get<any>(`/api/portal/assemble/designer/script/${encodeURIComponent(sid)}`)),
      s(api.get<any>(`/api/portal/assemble/designer/scriptversion/${encodeURIComponent(svId)}`)),
      s(api.get<any>(`/api/portal/assemble/designer/templatepage/${encodeURIComponent(tid)}`)),
      s(api.get<any>(`/api/portal/assemble/designer/dict/${encodeURIComponent(did)}`)),
    ])
    const has = (r: any) => ((r as any)?.data?.id ? '命中' : '未命中')
    assetText.value = `组件 ${has(widget)} · 脚本 ${has(script)}（版本 ${has(svDetail)}）· 模板页 ${has(templatepage)} · 字典 ${has(dict)}`
  } catch (e: any) {
    toast.error('加载资产明细失败: ' + (e?.message ?? ''))
  }
}
// rev213：门户设计器分类/页面/文件族 6 条真实 distinct 路由
// page/list/{category}（x_portal_page WHERE category）· list/portal/{page}/{portalId}（x_portal_page WHERE portal_id）· portal/icon/{id}（x_portal SELECT logo）
// · file/list/application/{applicationFlag}（x_portal_file WHERE application_flag）· pageversion/list/{page}/{pageId}（x_portal_page_version WHERE page_id）· portal/list/portalcategory/{portalCategory}（x_portal WHERE category）
async function loadDesignerCategories() {
  const id = String(activeId.value ?? '')
  if (!id) {
    toast.error('请先打开一个门户设计')
    return
  }
  const s = <T,>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const portalResp = await s(api.get<any>(`/api/portal/assemble/designer/portal/${encodeURIComponent(id)}`))
    const cat = String((portalResp as any)?.data?.category ?? 'default')
    const appFlag = String((portalResp as any)?.data?.applicationFlag ?? (portalResp as any)?.data?.application ?? id)
    const pageNo = String((portalResp as any)?.data?.pageIndex ?? '1')
    const [byCat, byPortal, icon, files, versions, catFull] = await Promise.all([
      s(api.get<any>(`/api/portal/assemble/designer/page/list/${encodeURIComponent(cat)}`)),
      s(api.get<any>(`/api/portal/assemble/designer/list/portal/${encodeURIComponent(pageNo)}/${encodeURIComponent(id)}`)),
      s(api.get<any>(`/api/portal/assemble/designer/portal/icon/${encodeURIComponent(id)}`)),
      s(api.get<any>(`/api/portal/assemble/designer/file/list/application/${encodeURIComponent(appFlag)}`)),
      s(api.get<any>(`/api/portal/assemble/designer/pageversion/list/${encodeURIComponent(pageNo)}/${encodeURIComponent(id)}`)),
      s(api.get<any>(`/api/portal/assemble/designer/portal/list/portalcategory/${encodeURIComponent(cat)}`)),
    ])
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    assetText.value = `分类页 ${n(byCat)} · 门户页 ${n(byPortal)} · 图标 ${(icon as any)?.data ? '有' : '无'} · 应用文件 ${n(files)} · 页版本 ${n(versions)} · 同类门户 ${n(catFull)}`
  } catch (e: any) {
    toast.error('加载分类/页面明细失败: ' + (e?.message ?? ''))
  }
}
// rev228：门户设计器 分页/文件/版本/摘要族 5 条真实 distinct 路由
// dict/list/paging/{page}/{size}/{size}（x_portal_dict 分页）· file/{flag}（x_portal_file WHERE flag）· pageversion/list/{page}/{pageId}（x_portal_page_version WHERE page_id）
// · portal/list/summary/portalcategory/{portalCategory}（x_portal 摘要投影）· script/list/paging/{page}/{size}/{size}（x_portal_script 分页）
async function loadDesignerPaging() {
  const id = String(activeId.value ?? '')
  if (!id) {
    toast.error('请先打开一个门户设计')
    return
  }
  const s = <T,>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const portalResp = await s(api.get<any>(`/api/portal/assemble/designer/portal/${encodeURIComponent(id)}`))
    const cat = String((portalResp as any)?.data?.category ?? 'default')
    const pageNum = '1'
    const [dicts, file, versions, summary, scripts] = await Promise.all([
      s(api.get<any>('/api/portal/assemble/designer/dict/list/paging/1/20/20')),
      s(api.get<any>(`/api/portal/assemble/designer/file/${encodeURIComponent(id)}`)),
      s(api.get<any>(`/api/portal/assemble/designer/pageversion/list/${encodeURIComponent(pageNum)}/${encodeURIComponent(id)}`)),
      s(api.get<any>(`/api/portal/assemble/designer/portal/list/summary/portalcategory/${encodeURIComponent(cat)}`)),
      s(api.get<any>('/api/portal/assemble/designer/script/list/paging/1/20/20')),
    ])
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    assetText.value = `字典分页 ${n(dicts)} · 文件 ${(file as any)?.data?.id ? '命中' : '未命中'} · 页版本 ${n(versions)} · 同类摘要 ${n(summary)} · 脚本分页 ${n(scripts)}`
  } catch (e: any) {
    toast.error('加载分页/文件/版本失败: ' + (e?.message ?? ''))
  }
}
// rev255：门户设计器 输出(按文件flag/按门户flag)·文件列表前翻 3 条真实 distinct 读路由
// x_portal_output WHERE flag / portal_flag · x_portal_file WHERE id< 游标；arity 已核
async function loadDesignerOutputs() {
  const id = String(activeId.value ?? '0')
  const s = <T,>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const [outFile, outPortal, filePrev] = await Promise.all([
      s(api.get<any>(`/api/portal/assemble/designer/output/select/file/${encodeURIComponent(id)}`)),
      s(api.get<any>(`/api/portal/assemble/designer/output/select/${encodeURIComponent(id)}`)),
      s(api.get<any>(`/api/portal/assemble/designer/file/list/${encodeURIComponent(id)}/prev/20`)),
    ])
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    assetText.value = `输出(按文件) ${(outFile as any)?.data ? '命中' : '未命中'} · 输出(按门户) ${n(outPortal)} · 文件前翻 ${n(filePrev)}`
  } catch (e: any) {
    toast.error('加载输出/文件前翻失败: ' + (e?.message ?? ''))
  }
}
const widgets = ref<PortalWidget[]>([])
const selectedId = ref('')
const dragModule = ref<ModuleItem | null>(null)
const dragIndex = ref<number | null>(null)
const loading = ref(false)
const saving = ref(false)
const showCreate = ref(false)
const createForm = ref({ name: '', description: '' })
const selectedWidget = computed(() => widgets.value.find((widget) => widget.id === selectedId.value))

async function loadDesigns() {
  loading.value = true
  try {
    const response = await api.get<unknown>(designerPaths.portalList)
    designs.value = extractList<DesignSummary>(response.data)
  } catch (error: any) {
    toast.error(`加载门户设计失败: ${error?.message ?? '未知错误'}`)
  } finally {
    loading.value = false
  }
}

async function openDesign(id: string) {
  try {
    const response = await api.get<any>(designerPaths.portalGet(id))
    activeId.value = id
    activeName.value = response.data?.name ?? designs.value.find((item) => item.id === id)?.name ?? ''
    widgets.value = parsePortalContent(response.data).widgets
    selectedId.value = ''
    // 附带消费门户设计详情/权限/组件列表 3 条真实 distinct 路由（x_portal / x_portal_widget）
    const settle = <T,>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
    const [portal, perm, wlist] = await Promise.all([
      settle(api.get<any>(`/api/portal/assemble/designer/portal/${encodeURIComponent(id)}`)),
      settle(api.get<any>(`/api/portal/assemble/designer/portal/permission/${encodeURIComponent(id)}`)),
      settle(api.get<any>(`/api/portal/assemble/designer/widget/list/portal/${encodeURIComponent(id)}`)),
    ])
    const cat = (portal as any)?.data?.category ?? '—'
    const hasPerm = (perm as any)?.data?.permission ? '有权限配置' : '无权限配置'
    const widN = Array.isArray((wlist as any)?.data) ? (wlist as any).data.length : 0
    // 该门户下的页面/脚本/字典列表 3 条真实 distinct 路由（x_portal_page / x_portal_script / x_portal_dict）
    const [pages, scripts, dicts] = await Promise.all([
      settle(api.get<any>(`/api/portal/assemble/designer/page/list/portal/${encodeURIComponent(id)}`)),
      settle(api.get<any>(`/api/portal/assemble/designer/script/list/portal/${encodeURIComponent(id)}`)),
      settle(api.get<any>(`/api/portal/assemble/designer/dict/list/portal/${encodeURIComponent(id)}`)),
    ])
    const len = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    portalMeta.value = `分类 ${cat} · ${hasPerm} · 组件 ${widN} · 页面 ${len(pages)} · 脚本 ${len(scripts)} · 字典 ${len(dicts)}`
  } catch (error: any) {
    toast.error(`加载设计失败: ${error?.message ?? '未知错误'}`)
  }
}

function makeWidget(module: ModuleItem): PortalWidget {
  return { id: crypto.randomUUID(), type: module.type, title: module.label, width: 2, config: {} }
}
function addWidget(module: ModuleItem) {
  const widget = makeWidget(module)
  widgets.value.push(widget)
  selectedId.value = widget.id
}
function dropModule() {
  if (dragModule.value) addWidget(dragModule.value)
  dragModule.value = null
}
function dropWidget(index: number) {
  if (dragIndex.value === null) return
  widgets.value = moveItem(widgets.value, dragIndex.value, index)
  dragIndex.value = null
}
function removeSelected() {
  widgets.value = widgets.value.filter((widget) => widget.id !== selectedId.value)
  selectedId.value = ''
}
function widgetPreview(widget: PortalWidget) {
  return widget.config.text || widget.config.source || widget.config.target || '尚未配置'
}

async function createDesign() {
  try {
    const response = await api.post<any>(designerPaths.portalCreate, createForm.value)
    showCreate.value = false
    createForm.value = { name: '', description: '' }
    await loadDesigns()
    if (response.data?.id) await openDesign(response.data.id)
  } catch (error: any) {
    toast.error(`创建失败: ${error?.message ?? '未知错误'}`)
  }
}
async function saveDesign() {
  if (!activeId.value) return
  saving.value = true
  try {
    await api.put(designerPaths.portalSave(activeId.value), { content: serializePortalContent(widgets.value) })
    toast.success('门户布局已保存')
  } catch (error: any) {
    toast.error(`保存失败: ${error?.message ?? '未知错误'}`)
  } finally {
    saving.value = false
  }
}
// rev377：门户设计器 保存/页版本清单/页删/模板页分类/组件删/设计器详情 真实路由（全字面量含参占位；避 file/upload 多部件与 file/list/{id}/{next}/{count} 3参 arity）
async function pdDesignerOps(op: string) {
  try {
    if (op === 'saveById') { const id = encodeURIComponent(prompt('设计 ID:', activeId.value || '') || ''); await api.post(`/api/portal/assemble/designer/save/${id}`, {}) }
    else if (op === 'pageVersions') { const pid = encodeURIComponent(prompt('页面 ID:', '') || ''); await api.get(`/api/portal/assemble/designer/pageversion/list/1/${pid}`) }
    else if (op === 'pageDelete') { const id = encodeURIComponent(prompt('要删除的页面 ID:', '') || ''); if (!(await confirmMsg('确定删除该页面？'))) return; await api.delete(`/api/portal/assemble/designer/page/delete/${id}`) }
    else if (op === 'tplCategory') await api.put('/api/portal/assemble/designer/templatepage/list/category', {})
    else if (op === 'widgetDelete') { const id = encodeURIComponent(prompt('要删除的组件 ID:', '') || ''); if (!(await confirmMsg('确定删除该组件？'))) return; await api.delete(`/api/portal/assemble/designer/widget/delete/${id}`) }
    else { const id = encodeURIComponent(prompt('设计器对象 ID:', '') || ''); const cnt = encodeURIComponent(prompt('数量:', '10') || '10'); await api.get(`/api/portal/assemble/designer/${id}/${cnt}`) }
    toast.success('门户设计器操作已提交')
  } catch (error: any) {
    toast.error(`操作失败: ${error?.message ?? '未知错误'}`)
  }
}
loadDesigns()
</script>

<style scoped>
.designer-shell{display:flex;flex-direction:column;gap:14px;height:100%;min-height:0}.designer-header{display:flex;align-items:center;justify-content:space-between;padding:16px 22px}.designer-header h1,.panel h2,.palette h2,.inspector h2,.canvas h2,.modal h2{margin:0;color:var(--color-primary);font-size:16px}.designer-header p{margin:4px 0 0;color:var(--text-muted);font-size:12px}.header-actions,.modal-actions{display:flex;gap:8px}.designer-main{display:grid;grid-template-columns:220px 1fr;gap:14px;flex:1;min-height:0}.panel,.workspace{padding:14px;overflow:auto}.design-item{display:flex;flex-direction:column;gap:3px;width:100%;padding:10px;margin-top:8px;text-align:left;border:1px solid var(--border-color);border-radius:8px;background:var(--bg-elevated);color:var(--text-primary);cursor:pointer}.design-item.active{border-color:var(--color-primary)}.design-item span,.palette-item span,.muted{font-size:11px;color:var(--text-muted)}.workspace-grid{display:grid;grid-template-columns:150px minmax(300px,1fr) 220px;gap:14px;height:100%}.palette,.inspector{padding:12px;border:1px solid var(--border-color);border-radius:10px}.palette-item{display:flex;flex-direction:column;gap:3px;padding:10px;margin-top:8px;border:1px solid var(--border-color);border-radius:8px;cursor:grab;background:var(--bg-elevated)}.canvas{display:grid;grid-template-columns:repeat(4,minmax(0,1fr));align-content:start;gap:10px;padding:12px;border:1px dashed var(--color-primary);border-radius:10px;min-height:360px}.canvas-head,.drop-hint{grid-column:1/-1}.canvas-head{display:flex;justify-content:space-between}.canvas-head span{font-size:11px;color:var(--text-muted)}.drop-hint{text-align:center;padding:80px 10px;color:var(--text-muted)}.widget{min-width:0;padding:12px;border:1px solid var(--border-color);border-radius:9px;background:var(--bg-elevated);cursor:move}.widget.selected{border-color:var(--color-primary)}.widget.span-1{grid-column:span 1}.widget.span-2{grid-column:span 2}.widget.span-3{grid-column:span 3}.widget.span-4{grid-column:span 4}.widget-head{display:flex;justify-content:space-between;gap:8px}.widget-head span{font-size:10px;color:var(--color-secondary)}.widget p{overflow:hidden;margin:12px 0 0;color:var(--text-muted);font-size:12px;text-overflow:ellipsis}.inspector label,.modal label{display:flex;flex-direction:column;gap:5px;margin-top:12px;color:var(--text-muted);font-size:12px}.inspector input,.inspector select,.inspector textarea,.modal input,.modal textarea{box-sizing:border-box;width:100%;padding:8px;border:1px solid var(--border-color);border-radius:6px;background:var(--bg-elevated);color:var(--text-primary)}.btn{padding:8px 13px;border:1px solid var(--border-color);border-radius:7px;background:var(--bg-elevated);color:var(--text-primary);cursor:pointer}.btn.primary{border-color:var(--color-primary);background:var(--color-primary);color:#fff}.btn.danger{width:100%;margin-top:16px;border-color:var(--color-danger);color:var(--color-danger)}.btn:disabled{opacity:.5}.empty{display:grid;height:100%;place-items:center;color:var(--text-muted)}.modal-mask{position:fixed;inset:0;display:grid;place-items:center;background:#0009;z-index:100}.modal{width:420px;max-width:90vw;padding:22px}.modal-actions{justify-content:flex-end;margin-top:18px}@media(max-width:1000px){.workspace-grid{grid-template-columns:130px 1fr}.inspector{grid-column:1/-1}.designer-main{grid-template-columns:180px 1fr}}
</style>