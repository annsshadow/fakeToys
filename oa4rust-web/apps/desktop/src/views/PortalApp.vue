<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="portal-view">
    <div class="view-header glass-card">
      <h1>门户管理</h1>
      <p class="subtitle">接入 /api/portal/* — 页面设计与发布</p>
      <span class="hdr-actions">
        <button class="new-page-btn ghost" @click="loadPortalList">门户列表</button>
        <button class="new-page-btn ghost" @click="loadPortalSurface">表面/移动</button>
        <button class="new-page-btn ghost" @click="loadPortalResources">门户资源</button>
        <button class="new-page-btn ghost" @click="loadPortalDetail">门户明细</button>
        <button class="new-page-btn ghost" @click="loadPortalSurfaceEntities">表面实体</button>
        <button class="new-page-btn ghost" @click="loadPortalMobileFacets">移动/字典/角标</button>
        <button class="new-page-btn" @click="showEditor = true">+ 新建页面</button>
      </span>
    </div>
    <div v-if="portalListText" class="portal-note">{{ portalListText }}</div>
    <div class="page-grid glass-card">
      <div v-if="pages.length === 0" class="empty-state">
        <div class="empty-icon">📄</div>
        <p>暂无门户页面</p>
      </div>
      <div v-for="page in pages" :key="page.id" class="page-card" @click="editPage(page)">
        <div class="page-preview">{{ page.icon || '📄' }}</div>
        <div class="page-info">
          <div class="page-name">{{ page.name || page.title || '未命名页面' }}</div>
          <div class="page-meta">{{ page.createTime || '未知时间' }}</div>
        </div>
        <div class="page-actions">
          <button class="icon-btn" @click.stop="publishPage(page)" title="发布">📤</button>
          <button class="icon-btn danger" @click.stop="deletePage(page.id)" title="删除">🗑</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { api } from '@oa4rust/sdk'
import { useMutation, useQuery, useQueryClient } from '@tanstack/vue-query'
import { ref } from 'vue'
import { confirmMsg, toast } from '../utils/toast'

interface PortalPage {
  id: string
  name?: string
  title?: string
  icon?: string
  createTime?: string
  appId?: string
}

const portalListText = ref('')
async function loadPortalSurface() {
  try {
    // GET portal/surface/list + portal/assemble/surface/portal/list/mobile —— 门户表面/移动门户
    const [surface, mobile] = await Promise.all([
      api.get('/api/portal/surface/list'),
      api.get('/api/portal/assemble/surface/portal/list/mobile'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    portalListText.value = `表面门户 ${n(surface)} / 移动门户 ${n(mobile)}`
  } catch (e: any) {
    toast.error('加载门户表面失败: ' + (e?.message ?? ''))
  }
}
async function loadPortalList() {
  try {
    // GET /api/portal/list + /api/portalcategory/list —— 门户与门户分类列表
    const [portals, cats] = await Promise.all([
      api.get('/api/portal/list'),
      api.get('/api/portalcategory/list'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    portalListText.value = `门户 ${n(portals)} 个 / 分类 ${n(cats)} 个`
  } catch (e: any) {
    toast.error('加载门户列表失败: ' + (e?.message ?? ''))
  }
}
// 门户资源（default 门户）：字典 + 文件 + 页面——三条 distinct 真实路由（portalFlag/page 参数匹配）
async function loadPortalResources() {
  try {
    const [dict, files, portalPages] = await Promise.all([
      api.get('/api/portal/assemble/surface/dict/list/portal/default'),
      api.get('/api/portal/assemble/surface/file/list/portal/default'),
      api.get('/api/portal/assemble/surface/list/portal/portal/default'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    portalListText.value = `字典 ${n(dict)} / 文件 ${n(files)} / 页面 ${n(portalPages)}`
  } catch (e: any) {
    toast.error('加载门户资源失败: ' + (e?.message ?? ''))
  }
}
// 门户明细族 3 条真实 distinct 路由：从 portal/list 首项取 flag → 门户详情 portal/{flag}（x_portal 全列）
// + 角标 portal/corner/mark/{flag}（x_portal corner_mark）+ 门户文件详情 file/{flag}（x_portal_file by flag）
async function loadPortalDetail() {
  try {
    const listResp: any = await api.get('/api/portal/list').catch(() => null)
    const portals = (Array.isArray(listResp?.data) ? listResp.data : []) as Array<Record<string, unknown>>
    const flag = portals[0] ? String(portals[0].flag ?? portals[0].id ?? '') : ''
    if (!flag) {
      portalListText.value = '暂无门户（无可抽样项）'
      return
    }
    // 门户文件 flag 从 file/list/portal/default（已消费列表）首项回源
    const fileListResp: any = await api.get('/api/portal/assemble/surface/file/list/portal/default').catch(() => null)
    const files = (Array.isArray(fileListResp?.data) ? fileListResp.data : []) as Array<Record<string, unknown>>
    const fileFlag = files[0] ? String(files[0].flag ?? files[0].id ?? '') : flag
    const [detail, corner, file] = await Promise.all([
      api.get(`/api/portal/assemble/surface/portal/${encodeURIComponent(flag)}`).catch(() => null),
      api.get(`/api/portal/assemble/surface/portal/corner/mark/${encodeURIComponent(flag)}`).catch(() => null),
      api.get(`/api/portal/assemble/surface/file/${encodeURIComponent(fileFlag)}`).catch(() => null),
    ])
    const name = (detail as any)?.data?.name ?? flag
    const mark = (corner as any)?.data?.corner_mark ?? '—'
    const fName = (file as any)?.data?.name ?? '—'
    portalListText.value = `门户「${name}」· 角标 ${mark} · 文件「${fName}」`
  } catch (e: any) {
    toast.error('加载门户明细失败: ' + (e?.message ?? ''))
  }
}
// rev214：门户表面组件/脚本/字典/页面族 7 条真实 distinct 路由
// get/{id}（x_portal_surface WHERE id）· list/{category}（WHERE category）· widget/{id}（x_portal_widget WHERE id）
// · widget/portal/{flag}/{portalFlag}（WHERE flag+portal_id）· script/{id}（x_portal_script WHERE id）· page/{id}/mobile（x_portal_page mobile_content）· dict/portal/{dictFlag}/{portalFlag}（x_portal_dict WHERE flag+portal_flag）
async function loadPortalSurfaceEntities() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const listResp: any = await api.get('/api/portal/list').catch(() => null)
    const portals = (Array.isArray(listResp?.data) ? listResp.data : []) as Array<Record<string, unknown>>
    const flag = portals[0] ? String(portals[0].flag ?? portals[0].id ?? '0') : '0'
    const wlist: any = await s(api.get('/api/portal/assemble/surface/widget/list/portal/portal'))
    const wRows = Array.isArray(wlist?.data) ? wlist.data : []
    const wid = wRows[0] ? String(wRows[0].id ?? '0') : '0'
    const wflag = wRows[0] ? String(wRows[0].flag ?? wid) : wid
    const [surf, byCat, widget, widgetByFlag, script, pageMobile, dict] = await Promise.all([
      s(api.get(`/api/portal/assemble/surface/get/${encodeURIComponent(flag)}`)),
      s(api.get(`/api/portal/assemble/surface/list/${encodeURIComponent(flag)}`)),
      s(api.get(`/api/portal/assemble/surface/widget/${encodeURIComponent(wid)}`)),
      s(api.get(`/api/portal/assemble/surface/widget/portal/${encodeURIComponent(wflag)}/${encodeURIComponent(flag)}`)),
      s(api.get(`/api/portal/assemble/surface/script/${encodeURIComponent(wid)}`)),
      s(api.get(`/api/portal/assemble/surface/page/${encodeURIComponent(flag)}/mobile`)),
      s(api.get(`/api/portal/assemble/surface/dict/portal/${encodeURIComponent(wflag)}/${encodeURIComponent(flag)}`)),
    ])
    const hit = (r: any) => ((r as any)?.data?.id ? '命中' : '未命中')
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    portalListText.value = `表面 ${hit(surf)} · 分类列表 ${n(byCat)} · 组件 ${hit(widget)}（按flag ${hit(widgetByFlag)}）· 脚本 ${hit(script)} · 移动页 ${hit(pageMobile)} · 字典 ${(dict as any)?.data ? '有' : '无'}`
  } catch (e: any) {
    toast.error('加载门户表面实体失败: ' + (e?.message ?? ''))
  }
}
// rev244：门户表面 字典/角标/组件移动 4 条真实 distinct 读路由（arity 已核；跳 get/layout·list/layouts·script/list/portal/portal·mobile/{page}/{id}·v2/{page}/{id} 等 handler Path 元数与 URL 参数数不符=运行时 500，及 dict/{}/portal/{}/data 双注册孪生）
async function loadPortalMobileFacets() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const flag = '0'
    const [dictData, corner, widgetMobile, widgetByFlag] = await Promise.all([
      s(api.get(`/api/portal/assemble/surface/dict/portal/data/${encodeURIComponent(flag)}/${encodeURIComponent(flag)}`)),
      s(api.get(`/api/portal/assemble/surface/portal/${encodeURIComponent(flag)}/corner/mark`)),
      s(api.get(`/api/portal/assemble/surface/widget/${encodeURIComponent(flag)}/mobile`)),
      s(api.get(`/api/portal/assemble/surface/widget/portal/mobile/${encodeURIComponent(flag)}/${encodeURIComponent(flag)}`)),
    ])
    const hit = (r: any) => ((r as any)?.data ? '命中' : '未命中')
    portalListText.value = `字典数据 ${hit(dictData)} · 角标 ${hit(corner)} · 组件移动(按id) ${hit(widgetMobile)} · 组件移动(按flag) ${hit(widgetByFlag)}`
  } catch (e: any) {
    toast.error('加载门户移动族失败: ' + (e?.message ?? ''))
  }
}
const pages = ref<PortalPage[]>([])
const showEditor = ref(false)
const queryClient = useQueryClient()

const { data } = useQuery({
  queryKey: ['portal', 'pages'],
  queryFn: async () => {
    const resp = await api.get('/api/portal/assemble/surface/page/list/portal/default')
    return ((resp as any)?.data ?? []) as PortalPage[]
  },
  staleTime: 60_000,
})
pages.value = data.value ?? []

const deleteMutation = useMutation({
  mutationFn: (id: string) => api.delete(`/api/portal/assemble/surface/page/${id}`),
  onSuccess: () => {
    pages.value = pages.value.filter((p) => p.id !== id)
    queryClient.invalidateQueries({ queryKey: ['portal', 'pages'] })
  },
})

function editPage(_page: PortalPage): void {
  // Navigate to portal designer (future)
}

function publishPage(_page: PortalPage): void {}

async function deletePage(id: string): void {
  if (await confirmMsg('确定删除此页面？')) deleteMutation.mutate(id)
}
</script>

<style scoped>
.portal-view { display: flex; flex-direction: column; gap: 16px; height: 100%; }
.view-header { display: flex; align-items: center; justify-content: space-between; padding: 16px 24px; }
.view-header h1 { font-family: 'Orbitron', sans-serif; font-size: 20px; color: var(--color-primary); margin: 0; }
.subtitle { font-size: 12px; color: var(--text-muted); margin: 4px 0 0; font-family: 'JetBrains Mono', monospace; }
.new-page-btn { padding: 8px 16px; border-radius: var(--radius-md); border: none; background: var(--color-primary); color: white; cursor: pointer; font-weight: 600; }
.page-grid { flex: 1; overflow-y: auto; padding: 16px; display: grid; grid-template-columns: repeat(auto-fill, minmax(200px, 1fr)); gap: 16px; }
.page-card { padding: 16px; background: var(--bg-elevated); border: 1px solid var(--border-subtle); border-radius: var(--radius-lg); cursor: pointer; transition: all var(--transition-fast); position: relative; }
.page-card:hover { border-color: var(--border-active); transform: translateY(-2px); box-shadow: var(--shadow-glow); }
.page-preview { font-size: 40px; margin-bottom: 8px; }
.page-info { }
.page-name { font-size: 14px; font-weight: 600; color: var(--text-primary); }
.page-meta { font-size: 11px; color: var(--text-muted); margin-top: 4px; }
.page-actions { position: absolute; top: 8px; right: 8px; display: flex; gap: 4px; opacity: 0; transition: opacity var(--transition-fast); }
.page-card:hover .page-actions { opacity: 1; }
.icon-btn { background: var(--bg-elevated); border: 1px solid var(--border-subtle); width: 28px; height: 28px; border-radius: var(--radius-sm); cursor: pointer; font-size: 14px; display: flex; align-items: center; justify-content: center; }
.icon-btn:hover { border-color: var(--color-primary); }
.icon-btn.danger:hover { border-color: var(--color-error); }
.empty-state { grid-column: 1/-1; display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 60px; color: var(--text-muted); gap: 12px; }
.empty-icon { font-size: 48px; opacity: 0.4; }
.hdr-actions{display:flex;gap:8px;align-items:center}
.new-page-btn.ghost{background:transparent;border:1px solid var(--border-subtle);color:var(--text-secondary)}
.portal-note{margin:8px 0;padding:8px 12px;border-radius:var(--radius-md);background:var(--bg-elevated);border:1px solid var(--border-subtle);font-size:12px;color:var(--text-secondary)}
</style>
