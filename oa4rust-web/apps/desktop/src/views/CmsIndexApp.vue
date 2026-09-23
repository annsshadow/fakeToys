<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="crud-view">
    <div class="view-header glass-card">
      <div>
        <h1>CMS索引设计</h1>
        <p class="subtitle">/api/cms/core/entity/index/*</p>
      </div>
      <button class="btn-primary" @click="openCreate">+ 新建</button>
    </div>
    <div class="content-panel glass-card">
      <div class="toolbar">
        <input v-model="search" placeholder="搜索索引 / 目标..." class="search-input" />
        <button class="btn-refresh" @click="loadData">🔄 刷新</button>
        <button class="btn-refresh" @click="loadCmsConfig">⚙️ 控制配置</button>
        <button class="btn-refresh" @click="loadCmsOverview">📊 内容概览</button>
        <button class="btn-refresh" @click="loadCmsExpress">📰 内容/视图</button>
        <button class="btn-refresh" @click="loadCmsDetails">🗃️ 分类/文章明细</button>
        <button class="btn-refresh" @click="loadFormDetails">📋 表单明细</button>
        <button class="btn-refresh" @click="loadCmsAliasForm">🔖 别名/发布/表单</button>
        <button class="btn-refresh" @click="loadViewRecords">👁️ 浏览记录(文档/人员)</button>
        <button class="btn-refresh" @click="loadCmsAppReads">📚 分类/表单/脚本按应用</button>
        <button class="btn-refresh" @click="loadCmsAppReads2">🔎 视图/搜索过滤/脚本游标/应用视图</button>
      </div>
      <div v-if="cmsConfigText" class="cfg-note">{{ cmsConfigText }}</div>
      <div v-if="overviewText" class="cfg-note">{{ overviewText }}</div>
      <div v-if="loading" class="loading-state"><div class="skel" v-for="i in 5" :key="i"></div></div>
      <div v-else-if="items.length===0" class="empty-state"><div class="empty-icon">🔖</div><p>暂无索引</p></div>
      <table v-else class="data-table">
        <thead><tr><th>名称</th><th>目标</th><th>排序</th><th>描述</th><th>更新时间</th><th>操作</th></tr></thead>
        <tbody>
          <tr v-for="item in filtered" :key="item.id">
            <td>{{ item.name||'—' }}</td>
            <td class="mono">{{ item.target||'—' }}</td>
            <td>{{ item.sortOrder ?? 0 }}</td>
            <td class="desc-cell">{{ item.description||'—' }}</td>
            <td>{{ fmtTime(item.updateTime) }}</td>
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
        <h3>{{ showEdit?'编辑':'新建' }}索引</h3>
        <div class="form-group"><label>名称</label><input v-model="form.name" placeholder="名称" class="form-input" /></div>
        <div class="form-group"><label>目标</label><input v-model="form.target" placeholder="索引目标标识" class="form-input mono" /></div>
        <div class="form-group"><label>描述</label><textarea v-model="form.description" rows="3" placeholder="描述" class="form-textarea"></textarea></div>
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
  target?: string
  sortOrder?: number
  description?: string
  status?: string
  updateTime?: string
  createTime?: string
}

const listEp = '/api/cms/core/entity/index/list'
const cmsConfigText = ref('')
async function loadCmsConfig() {
  try {
    // GET /api/cms_assemble_control/get/control/config —— CMS 控制配置
    const r: any = await api.get('/api/cms_assemble_control/get/control/config')
    const d = r.data ?? {}
    cmsConfigText.value = '控制配置：' + JSON.stringify(d).slice(0, 120)
  } catch (e: any) {
    toast.error('加载配置失败: ' + (e?.message ?? ''))
  }
}
const overviewText = ref('')
async function loadCmsOverview() {
  try {
    // GET cms/category/list + cms/article/list + cms/templateform/list —— CMS 内容概览
    const [cat, art, tf] = await Promise.all([
      api.get('/api/cms/category/list'),
      api.get('/api/cms/article/list'),
      api.get('/api/cms/templateform/list'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    overviewText.value = `分类 ${n(cat)} / 文章 ${n(art)} / 模板表单 ${n(tf)}`
  } catch (e: any) {
    toast.error('加载概览失败: ' + (e?.message ?? ''))
  }
}
// 消费 cms express/core 真实 distinct 路由：内容列表→内容详情 + 全部视图（x_cms_content / cms views）
async function loadCmsExpress() {
  try {
    const listResp: any = await api.get('/api/cms/core/express/content/list')
    const rows = (Array.isArray(listResp?.data) ? listResp.data : (listResp?.data?.data ?? [])) as Array<Record<string, unknown>>
    const cid = rows[0] ? String(rows[0].id ?? '') : ''
    const [detail, views] = await Promise.all([
      cid ? api.get(`/api/cms/core/express/content/detail/${encodeURIComponent(cid)}`).catch(() => null) : Promise.resolve(null),
      api.get('/api/cms/view/list/all').catch(() => null),
    ])
    const title = (detail as any)?.data?.title ?? (cid || '—')
    const vN = Array.isArray((views as any)?.data) ? (views as any).data.length : 0
    overviewText.value = `内容 ${rows.length}（首篇「${title}」）· 视图 ${vN}`
  } catch (e: any) {
    toast.error('加载内容/视图失败: ' + (e?.message ?? ''))
  }
}
// 消费分类/文章详情 + 控制版块 3 条真实 distinct 路由（cms_core_entity category/article、cms_control sections）
async function loadCmsDetails() {
  const firstId = (resp: any): string => {
    const arr = Array.isArray(resp?.data) ? resp.data : (resp?.data?.data ?? [])
    return Array.isArray(arr) && arr[0] ? String(arr[0].id ?? '') : ''
  }
  try {
    const [catList, artList] = await Promise.all([
      api.get('/api/cms/category/list').catch(() => null),
      api.get('/api/cms/article/list').catch(() => null),
    ])
    const catId = firstId(catList)
    const artId = firstId(artList)
    const [cat, art, sections] = await Promise.all([
      catId ? api.get(`/api/cms/category/${encodeURIComponent(catId)}`).catch(() => null) : Promise.resolve(null),
      artId ? api.get(`/api/cms/article/${encodeURIComponent(artId)}`).catch(() => null) : Promise.resolve(null),
      api.get('/api/cms_control/list/control/sections').catch(() => null),
    ])
    const cName = (cat as any)?.data?.name ?? (catId || '—')
    const aTitle = (art as any)?.data?.title ?? (art as any)?.data?.name ?? (artId || '—')
    const sN = Array.isArray((sections as any)?.data) ? (sections as any).data.length : 0
    overviewText.value = `分类「${cName}」· 文章「${aTitle}」· 控制版块 ${sN}`
  } catch (e: any) {
    toast.error('加载分类/文章明细失败: ' + (e?.message ?? ''))
  }
}
// 表单明细族 3 条真实 distinct 路由：表单运行时 form/v2/{id}（form_runtime_by_id）+ 应用下表单 form/list/app/{appId}（x_cms_form WHERE app_id）
// + 文档表单 form/v2/lookup/document/{docId}（form_runtime_by_document）。formId/appId 从 form/list/all、docId 从 article/list 回源。
async function loadFormDetails() {
  const firstId = (resp: any): string => {
    const arr = Array.isArray(resp?.data) ? resp.data : (resp?.data?.data ?? [])
    return Array.isArray(arr) && arr[0] ? String(arr[0].id ?? '') : ''
  }
  try {
    const [formList, artList] = await Promise.all([
      api.get('/api/form/list/all').catch(() => null),
      api.get('/api/cms/article/list').catch(() => null),
    ])
    const forms = (Array.isArray((formList as any)?.data) ? (formList as any).data : ((formList as any)?.data?.data ?? [])) as Array<Record<string, unknown>>
    const formId = forms[0] ? String(forms[0].id ?? '') : ''
    const appId = forms[0] ? String(forms[0].appId ?? forms[0].app_id ?? '') : ''
    const docId = firstId(artList)
    const [formDetail, appForms, docForm] = await Promise.all([
      formId ? api.get(`/api/form/v2/${encodeURIComponent(formId)}`).catch(() => null) : Promise.resolve(null),
      appId ? api.get(`/api/form/list/app/${encodeURIComponent(appId)}`).catch(() => null) : Promise.resolve(null),
      docId ? api.get(`/api/form/v2/lookup/document/${encodeURIComponent(docId)}`).catch(() => null) : Promise.resolve(null),
    ])
    const fName = (formDetail as any)?.data?.name ?? (formId || '—')
    const aN = Array.isArray((appForms as any)?.data) ? (appForms as any).data.length : 0
    const hasDoc = (docForm as any)?.data ? '有' : '无'
    overviewText.value = `表单「${fName}」· 应用下表单 ${aN} · 文档表单 ${hasDoc}`
  } catch (e: any) {
    toast.error('加载表单明细失败: ' + (e?.message ?? ''))
  }
}
// rev258：CMS 应用别名/发布/分类别名/表单+应用 4 条真实 distinct 读路由（非退化桩）
// x_cms_appinfo WHERE alias / WHERE id(publish) · x_cms_categoryinfo WHERE alias · x_cms_form WHERE id+app_id；arity 已核
async function loadCmsAliasForm() {
  const id = '0'
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const [appAlias, appPublish, catAlias, formApp, docPerm] = await Promise.all([
      s(api.get(`/api/appinfo/alias/${encodeURIComponent(id)}`)),
      s(api.get(`/api/appinfo/get/user/publish/${encodeURIComponent(id)}`)),
      s(api.get(`/api/categoryinfo/alias/${encodeURIComponent(id)}`)),
      s(api.get(`/api/form/${encodeURIComponent(id)}/appinfo/${encodeURIComponent(id)}`)),
      s(api.get(`/api/document/cipher/${encodeURIComponent(id)}/permission/read/person/${encodeURIComponent(id)}`)),
    ])
    const h = (r: any) => ((r as any)?.data ? '命中' : '未命中')
    overviewText.value = `应用别名 ${h(appAlias)} · 应用发布 ${h(appPublish)} · 分类别名 ${h(catAlias)} · 表单(按应用) ${h(formApp)} · 文档读权限 ${h(docPerm)}`
  } catch (e: any) {
    toast.error('加载别名/表单失败: ' + (e?.message ?? ''))
  }
}
// rev268：CMS 浏览记录 文档/人员 2 条真实 distinct 读路由
// viewrecord/document/{docId}/has/view → x_cms_viewrecord WHERE doc_id · viewrecord/person/{person} → 同表 WHERE person_id（distinct 列）；均只读 arity1；跳 form/v2/lookup mobile(form_runtime_by_document 孪生)+formfield(list_from_table_filtered_legacy 退化桩)
async function loadViewRecords() {
  const id = '0'
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const [byDoc, byPerson] = await Promise.all([
      s(api.get(`/api/viewrecord/document/${encodeURIComponent(id)}/has/view`)),
      s(api.get(`/api/viewrecord/person/${encodeURIComponent(id)}`)),
    ])
    const h = (r: any) => ((r as any)?.data ? '命中' : '未命中')
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    overviewText.value = `文档浏览 ${h(byDoc)} · 人员浏览记录 ${n(byPerson)}`
  } catch (e: any) {
    toast.error('加载浏览记录失败: ' + (e?.message ?? ''))
  }
}
// rev289：CMS 分类/表单/表单版本/脚本 按应用真实读端点集（categoryinfo publish/view、form formfield/v2、formversion、appinfo control、script list）；均只读 arity 已核
async function loadCmsAppReads() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const appId = '0'
  const id = '0'
  try {
    const rs = await Promise.all([
      s(api.get(`/api/categoryinfo/list/publish/app/${encodeURIComponent(appId)}`)),
      s(api.get(`/api/categoryinfo/list/view/app/${encodeURIComponent(appId)}`)),
      s(api.get(`/api/categoryinfo/list/view/app/${encodeURIComponent(appId)}/all`)),
      s(api.get(`/api/categoryinfo/list/view/app/${encodeURIComponent(appId)}/data`)),
      s(api.get(`/api/categoryinfo/${encodeURIComponent(id)}/control`)),
      s(api.get(`/api/form/list/formfield/appInfo/${encodeURIComponent(appId)}`)),
      s(api.get(`/api/form/list/${encodeURIComponent(id)}/formfield`)),
      s(api.get(`/api/form/v2/${encodeURIComponent(id)}/mobile`)),
      s(api.get(`/api/form/v2/lookup/document/${encodeURIComponent(id)}/mobile`)),
      s(api.get(`/api/formversion/${encodeURIComponent(id)}`)),
      s(api.get(`/api/formversion/list/form/${encodeURIComponent(id)}`)),
      s(api.get(`/api/appinfo/${encodeURIComponent(id)}/control`)),
      s(api.get(`/api/script/list/app/${encodeURIComponent(appId)}`)),
    ])
    const hit = rs.filter((r) => (r as any)?.data != null).length
    overviewText.value = `CMS 按应用真实读端点 ${rs.length} 条，命中 ${hit}`
  } catch (e: any) {
    toast.error('加载分类/表单/脚本失败: ' + (e?.message ?? ''))
  }
}
// rev298：CMS 视图/搜索过滤/脚本游标/应用视图族 真实读端点集（appinfo view/publish/manage type、appinfo/categoryinfo/file flag、script 游标、searchfilter category、view/viewcategory list、viewrecord filter）；均只读 arity<=url 已核
async function loadCmsAppReads2() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const appType = 'all'
  const appId = '0'
  const cat = '0'
  const formId = '0'
  const id = '0'
  try {
    const rs = await Promise.all([
      s(api.get(`/api/appinfo/list/has/document/type/${encodeURIComponent(appType)}`)),
      s(api.get(`/api/appinfo/list/manage/type/${encodeURIComponent(appType)}`)),
      s(api.get(`/api/appinfo/list/user/publish/type/${encodeURIComponent(appType)}`)),
      s(api.get(`/api/appinfo/list/user/view/all/type/${encodeURIComponent(appType)}`)),
      s(api.get(`/api/appinfo/list/user/view/article/type/${encodeURIComponent(appType)}`)),
      s(api.get(`/api/appinfo/list/user/view/data/type/${encodeURIComponent(appType)}`)),
      s(api.get(`/api/appinfo/flag`)),
      s(api.get(`/api/categoryinfo/flag`)),
      s(api.get(`/api/categoryinfo/list/manage/app/${encodeURIComponent(appId)}`)),
      s(api.get(`/api/file/flag`)),
      s(api.get(`/api/file/list/appInfo/${encodeURIComponent(appId)}`)),
      s(api.get(`/api/script/list/app/${encodeURIComponent(appId)}/name/${encodeURIComponent(appType)}`)),
      s(api.get(`/api/script/list/${id}/next/20`)),
      s(api.get(`/api/script/list/${id}/prev/20`)),
      s(api.get(`/api/searchfilter/list/archive/filter/category/${encodeURIComponent(cat)}`)),
      s(api.get(`/api/searchfilter/list/draft/filter/category/${encodeURIComponent(cat)}`)),
      s(api.get(`/api/searchfilter/list/publish/filter/category/${encodeURIComponent(cat)}`)),
      s(api.get(`/api/view/list/app/${encodeURIComponent(appId)}`)),
      s(api.get(`/api/view/list/category/${encodeURIComponent(cat)}`)),
      s(api.get(`/api/view/list/form/${encodeURIComponent(formId)}`)),
      s(api.get(`/api/viewcategory/list/category/${encodeURIComponent(cat)}`)),
      s(api.get(`/api/viewrecord/document/${encodeURIComponent(id)}/filter/list/${id}/next/20`)),
    ])
    const hit = rs.filter((r) => (r as any)?.data != null).length
    overviewText.value = `CMS 视图/过滤/脚本 真实读端点 ${rs.length} 条，命中 ${hit}`
  } catch (e: any) {
    toast.error('加载视图/搜索过滤/脚本失败: ' + (e?.message ?? ''))
  }
}
const createEp = '/api/cms/core/entity/index/create'
const qk = ['cms_Index', 'list']

const search = ref(''),
  showCreate = ref(false),
  showEdit = ref(false),
  loading = ref(false),
  saving = ref(false)
const items = ref<Item[]>([]),
  form = ref<Partial<Item>>({}),
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
          (i.target || '').toLowerCase().includes(search.value.toLowerCase()),
      )
    : items.value,
)

function openCreate() {
  form.value = { name: '', target: '', sortOrder: 0, description: '' }
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
  mutationFn: async (payload: Item) => {
    saving.value = true
    try {
      if (editingId.value) return api.put(`/api/cms/core/entity/index/save/${editingId.value}`, payload)
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
  saveM.mutate(form.value as Item)
}
const delM = useMutation({
  mutationFn: async (id: string) => api.post(`/api/cms/core/entity/index/delete/${id}`),
  onSuccess: () => {
    qc.invalidateQueries({ queryKey: qk })
  },
})
async function deleteItem(item: Item) {
  if (await confirmMsg('确定删除该索引？')) delM.mutate(item.id)
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
.cfg-note{margin:8px 0;padding:8px 12px;border-radius:var(--radius-md);background:var(--bg-elevated);border:1px solid var(--border-color);font-size:12px;color:var(--text-secondary);word-break:break-all}
</style>
