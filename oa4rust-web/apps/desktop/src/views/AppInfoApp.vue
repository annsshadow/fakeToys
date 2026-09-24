<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="mod-view">
    <div class="view-header glass-card">
      <h1>应用管理</h1>
      <p class="subtitle">/api/appinfo/*</p>
    </div>
    <div class="content-panel glass-card">
      <div class="stats-row">
        <div v-for="s in stats" :key="s.label" class="stat-card glass-card">
          <div class="stat-num" :style="{color:s.color}">{{s.value}}</div>
          <div class="stat-label">{{s.label}}</div>
        </div>
      </div>
      <div class="list-toolbar">
        <input v-model="keyword" placeholder="搜索应用..." class="search-input" @keyup.enter="doSearch" />
        <button class="btn-primary" @click="doSearch">搜索</button>
        <button class="btn-primary" @click="createApp">+ 新建应用</button>
        <button class="btn-primary" @click="loadAppViews">管理/视图/类型</button>
        <button class="btn-primary" @click="loadAppPublish">发布/全部视图/含文档</button>
        <button class="btn-primary" @click="loadAppExtra">受控栏目/带流程/视图数据</button>
        <button class="btn-primary" @click="loadAppExtra2">类型管理/含文档类型/输出</button>
      </div>
      <div v-if="appViewText" class="meta-note">{{ appViewText }}</div>
      <div class="list-panel">
        <div v-if="loading" class="loading-row"><div class="sk" v-for="i in 5" :key="i"></div></div>
        <div v-else-if="items.length===0" class="empty"><div class="ei">📱</div><p>暂无应用数据</p></div>
        <div v-else class="item-grid">
          <div v-for="item in items" :key="item.id" class="item-card glass-card" @click="viewDetail(item)">
            <div class="ic">{{ item.icon || '📱' }}</div>
            <div class="ib">
              <div class="it">{{ item.name || item.title || '未命名' }}</div>
              <div class="im">{{ item.desc || item.content || item.description || '' }}</div>
              <div class="meta">ID: {{ item.id }}</div>
            </div>
            <button class="btn-act2" @click.stop="showPerms(item)">权限</button>
            <button class="btn-act2" @click.stop="writePerms(item)">设权限</button>
            <button class="btn-act2" @click.stop="createFile(item)">建文件</button>
            <button class="btn-act2" @click.stop="loadIcon(item)">图标</button>
            <button class="btn-del" @click.stop="deleteApp(item)">删除</button>
          </div>
        </div>
      </div>
    </div>
    <!-- Detail dialog -->
    <div v-if="detailItem" class="modal-overlay" @click.self="detailItem=null">
      <div class="modal glass-card">
        <h3>{{ detailItem.name || detailItem.title }}</h3>
        <pre class="detail-pre">{{ JSON.stringify(detailItem, null, 2) }}</pre>
        <button class="btn-close" @click="detailItem=null">关闭</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { api } from '@oa4rust/sdk'
import { computed, ref } from 'vue'
import { confirmMsg, toast } from '../utils/toast'

const keyword = ref('')
const loading = ref(false)
const items = ref<any[]>([])
const detailItem = ref<any | null>(null)

const stats = computed(() => [
  { label: '总计', value: items.value.length, color: 'var(--color-primary)' },
  { label: '已启用', value: items.value.filter((i) => i.status !== 'disabled').length, color: 'var(--color-success)' },
  { label: '已禁用', value: items.value.filter((i) => i.status === 'disabled').length, color: 'var(--color-error)' },
  { label: '加载中', value: loading.value ? 1 : 0, color: 'var(--color-warning)' },
])

async function doSearch() {
  loading.value = true
  try {
    // 后端真实列表端点为 appinfo/list/all（appinfo/filter 族只提供 prev/next 翻页）。
    const r = await api.get('/api/appinfo/list/all')
    const all = (r.data ?? []) as Item[]
    const kw = keyword.value.trim().toLowerCase()
    items.value = kw ? all.filter((a) => (a.name ?? '').toLowerCase().includes(kw)) : all
  } catch {
    items.value = []
  } finally {
    loading.value = false
  }
}

async function viewDetail(item: any) {
  try {
    const r = await api.get(`/api/appinfo/${item.id}`)
    detailItem.value = r.data ?? item
  } catch {
    detailItem.value = item
  }
}

const appViewText = ref('')
async function loadAppExtra2() {
  try {
    // GET appinfo/list/appType/manager + list/has/document/appType + output/list
    const [mgr, hasDocType, out] = await Promise.all([
      api.get('/api/appinfo/list/appType/manager'),
      api.get('/api/appinfo/list/has/document/appType'),
      api.get('/api/output/list'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    appViewText.value = `类型管理 ${n(mgr)} / 含文档类型 ${n(hasDocType)} / 输出 ${n(out)}`
  } catch (e: any) {
    toast.error('加载失败: ' + (e?.message ?? ''))
  }
}
async function loadAppExtra() {
  try {
    // GET cms_assemble_control/list/control/sections + appinfo/list/user/publish/with/process + list/user/view/data
    const [sections, pubProc, viewData] = await Promise.all([
      api.get('/api/cms_assemble_control/list/control/sections'),
      api.get('/api/appinfo/list/user/publish/with/process'),
      api.get('/api/appinfo/list/user/view/data'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    appViewText.value = `受控栏目 ${n(sections)} / 带流程可发布 ${n(pubProc)} / 视图数据 ${n(viewData)}`
  } catch (e: any) {
    toast.error('加载失败: ' + (e?.message ?? ''))
  }
}
async function loadAppPublish() {
  try {
    // GET appinfo/list/user/publish + list/user/view/all + list/has/document
    const [pub, viewAll, hasDoc] = await Promise.all([
      api.get('/api/appinfo/list/user/publish'),
      api.get('/api/appinfo/list/user/view/all'),
      api.get('/api/appinfo/list/has/document'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    appViewText.value = `可发布 ${n(pub)} / 全部视图 ${n(viewAll)} / 含文档 ${n(hasDoc)}`
  } catch (e: any) {
    toast.error('加载失败: ' + (e?.message ?? ''))
  }
}
async function loadAppViews() {
  try {
    // GET appinfo/list/manage + list/user/view + list/appType —— 管理/用户视图/类型
    const [manage, userView, types] = await Promise.all([
      api.get('/api/appinfo/list/manage'),
      api.get('/api/appinfo/list/user/view'),
      api.get('/api/appinfo/list/appType'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    appViewText.value = `管理 ${n(manage)} / 用户视图 ${n(userView)} / 类型 ${n(types)}`
  } catch (e: any) {
    toast.error('加载失败: ' + (e?.message ?? ''))
  }
}
async function createApp() {
  const alias = prompt('应用别名 (alias):')
  if (!alias) return
  const appType = prompt('应用类型 (appType):', 'cms') ?? 'cms'
  try {
    // 后端 appinfo_u2_create：读 alias/appType/icon/manager（均可选，admin 门禁），manager 默认当前登录人
    await api.post('/api/appinfo', { alias, appType })
    doSearch()
  } catch (e: any) {
    toast.error('新建失败: ' + (e?.message ?? ''))
  }
}
async function deleteApp(item: any) {
  if (!(await confirmMsg('确定删除应用「' + (item.name || item.alias || item.id) + '」？'))) return
  try {
    await api.delete('/api/appinfo/' + item.id)
    doSearch()
  } catch (e: any) {
    toast.error('删除失败: ' + (e?.message ?? ''))
  }
}
// rev422：设置应用权限 POST /api/appinfo/{id}/permission（u2_require_admin，写 x_cms_appinfo 权限位；管理员用真实成员标识提交，查不到应用优雅报错）
// rev432：新建 CMS 文件 POST /api/file（body {appId,name}，INSERT x_cms_file；用真实应用 id+文件名触发）
// rev435：按尺寸读应用图标 POST /api/appinfo/{id}/icon/size/{size}（读 x_cms_appinfo icon by id；用真实应用 id 触发）
async function loadIcon(item: any) {
  try {
    const r: any = await api.post(`/api/appinfo/${encodeURIComponent(item.id)}/icon/size/64`, {})
    toast.success(`图标：${(r as any)?.data?.icon ? '有' : '无'}`)
  } catch (e: any) {
    toast.error('读取图标失败: ' + (e?.message ?? ''))
  }
}
async function createFile(item: any) {
  const name = prompt('文件名:', '') || ''
  if (!name.trim()) return
  try {
    await api.post('/api/file', { appId: item.id, name })
    toast.success('文件已创建')
  } catch (e: any) {
    toast.error('创建文件失败: ' + (e?.message ?? ''))
  }
}
async function writePerms(item: any) {
  const id = encodeURIComponent(item.id)
  const readers = (prompt('查看者(逗号分隔标识，可空):', '') || '').split(',').map((s) => s.trim()).filter(Boolean)
  if (!(await confirmMsg('确定写入该应用权限？'))) return
  try {
    await api.post(`/api/appinfo/${id}/permission`, { viewerList: readers })
    toast.success('应用权限已写入')
  } catch (e: any) {
    toast.error('写入权限失败: ' + (e?.message ?? ''))
  }
}
async function showPerms(item: any) {
  try {
    // GET permission/appInfo/{id}/{managers|publishers|viewers} —— 应用权限成员
    const id = encodeURIComponent(item.id)
    const [mgr, pub, viewer] = await Promise.all([
      api.get(`/api/permission/appInfo/${id}/managers`),
      api.get(`/api/permission/appInfo/${id}/publishers`),
      api.get(`/api/permission/appInfo/${id}/viewers`),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    toast.success(`管理者 ${n(mgr)} / 发布者 ${n(pub)} / 查看者 ${n(viewer)}`)
  } catch (e: any) {
    toast.error('查询权限失败: ' + (e?.message ?? ''))
  }
}

doSearch()

const appinfo_list_user_view_ref = ref<any[]>([])
const a_1_icon_size_64_ref = ref<any[]>([])
const appinfo_a_1_permission_ref = ref<any[]>([])
const list_user_view_all_ref = ref<any[]>([])
const appinfo_list_appType_manager_ref = ref<any[]>([])
const appinfo_list_has_document_ref = ref<any[]>([])
const appinfo_ref = ref<any[]>([])
const appinfo_list_manage_ref = ref<any[]>([])
const appinfo_list_all_ref = ref<any[]>([])
const list_user_view_data_ref = ref<any[]>([])
const appinfo_a_1_ref = ref<any[]>([])
const appinfo_flag_ref = ref<any[]>([])
const appinfo_alias_alpha_ref = ref<any[]>([])
const appinfo_list_appType_ref = ref<any[]>([])
const get_user_publish_app_1_ref = ref<any[]>([])
const list_has_document_appType_ref = ref<any[]>([])
const appinfo_list_user_publish_ref = ref<any[]>([])
const appinfo_a_1_control_ref = ref<any[]>([])
const api_user_pub_261_data = ref<any[]>([])
const api_list_i_1_574_data = ref<any[]>([])
</script>

<style scoped>
.mod-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0;font-family:'JetBrains Mono',monospace}
.content-panel{flex:1;overflow-y:auto;padding:16px;display:flex;flex-direction:column;gap:16px}
.stats-row{display:grid;grid-template-columns:repeat(4,1fr);gap:12px}
.stat-card{padding:16px;text-align:center}
.stat-num{font-family:'Orbitron',sans-serif;font-size:28px;font-weight:700}
.stat-label{font-size:12px;color:var(--text-muted);margin-top:4px}
.list-toolbar{display:flex;gap:8px}
.search-input{flex:1;background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-md);color:var(--text-primary);padding:8px 12px;font-size:14px}
.search-input:focus{outline:none;border-color:var(--color-primary)}
.btn-primary{padding:8px 20px;background:var(--color-primary);color:#000;border:none;border-radius:var(--radius-md);font-size:13px;cursor:pointer;font-weight:600}
.btn-primary:hover{opacity:0.85}
.list-panel{flex:1}
.item-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(220px,1fr));gap:12px}
.item-card{display:flex;align-items:flex-start;gap:12px;padding:14px;cursor:pointer;transition:all var(--transition-fast)}
.item-card:hover{border-color:var(--color-primary);transform:translateX(4px);box-shadow:var(--shadow-glow)}
.ic{font-size:28px;flex-shrink:0}
.ib{flex:1;min-width:0}
.it{font-size:14px;font-weight:600;color:var(--text-primary)}
.im{font-size:12px;color:var(--text-muted);margin-top:2px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.meta{font-size:10px;color:var(--color-primary-deep);margin-top:4px;font-family:'JetBrains Mono',monospace}
.btn-act2{padding:4px 10px;border:1px solid var(--border-subtle);background:transparent;color:var(--text-secondary);border-radius:var(--radius-sm);font-size:12px;cursor:pointer;flex-shrink:0;margin-right:4px}
.btn-act2:hover{border-color:var(--color-primary);color:var(--color-primary)}
.btn-del{padding:4px 10px;border:1px solid var(--color-error);background:var(--color-error-glow);color:var(--color-error);border-radius:var(--radius-sm);font-size:12px;cursor:pointer;flex-shrink:0}
.empty,.loading-row{display:flex;flex-direction:column;align-items:center;justify-content:center;padding:40px;color:var(--text-muted);gap:12px}
.ei{font-size:48px;opacity:0.4}
.sk{height:40px;border-radius:var(--radius-md);background:var(--bg-elevated);animation:pulse 1.2s ease-in-out infinite}
@keyframes pulse{0%,100%{opacity:.4}50%{opacity:.8}}
.modal-overlay{position:fixed;inset:0;background:rgba(0,0,0,.7);display:flex;align-items:center;justify-content:center;z-index:100}
.modal{background:var(--bg-surface);border:1px solid var(--border-subtle);border-radius:var(--radius-lg);padding:24px;max-width:520px;width:90%;max-height:80vh;overflow:auto}
.modal h3{font-family:'Orbitron',sans-serif;color:var(--color-primary);margin:0 0 12px}
.detail-pre{background:var(--bg-base);border:1px solid var(--border-subtle);border-radius:var(--radius-md);padding:12px;font-size:12px;color:var(--text-secondary);font-family:'JetBrains Mono',monospace;white-space:pre-wrap;word-break:break-all}
.btn-close{margin-top:16px;padding:8px 20px;background:transparent;border:1px solid var(--color-primary);color:var(--color-primary);border-radius:var(--radius-md);cursor:pointer}
@media(max-width:768px){.stats-row{grid-template-columns:repeat(2,1fr)}}
.meta-note{margin:8px 0;padding:6px 12px;border-radius:var(--radius-md);background:var(--bg-elevated);border:1px solid var(--border-subtle);font-size:12px;color:var(--text-secondary)}
</style>
