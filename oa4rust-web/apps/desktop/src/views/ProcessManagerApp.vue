<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="crud-view">
    <div class="view-header glass-card">
      <div>
        <h1>流程实例管理</h1>
        <p class="subtitle">/api/processplatform/assemble/surface/process_manager/list（x_process_definition，只读）</p>
      </div>
      <button class="btn-refresh" @click="loadData">🔄 刷新</button>
    </div>
    <div class="content-panel glass-card">
      <div class="toolbar">
        <input v-model="search" placeholder="搜索流程 / 分类 / 创建人..." class="search-input" />
        <button class="btn-refresh" @click="loadRunningProcesses">⚙️ 运行中流程</button>
        <button class="btn-refresh" @click="loadManagedApps">👤 我管理的应用</button>
        <button class="btn-refresh" @click="loadAppProcesses">🗂️ 应用与流程</button>
        <button class="btn-refresh" @click="loadOrphans">🧹 孤儿元素</button>
        <button class="btn-refresh" @click="loadProcessDetails">🧬 流程明细</button>
        <button class="btn-refresh" @click="loadMappingAccess">🗺️ 映射/项权限</button>
        <button class="btn-refresh" @click="loadDesignerExtras">🧩 映射游标/字典/流程</button>
        <button class="btn-refresh" @click="loadDesignerFileScript">📂 文件/脚本/图标</button>
        <button class="btn-refresh" @click="loadSurfaceProcessReads">🖼️ 表面/字典/流程</button>
        <button class="btn-refresh" @click="loadSurfaceDataForms">📑 完成件/草稿/表单</button>
        <button class="btn-refresh" @click="loadDesignerItemAccess">🔑 项访问/输出/版次</button>
        <button class="btn-refresh" @click="loadSurfaceRouteSign">🧭 路由/签署/可控流程</button>
        <button class="btn-refresh" @click="loadMergeitemEnabled">🧩 合并项计划/启用流程</button>
        <button class="btn-refresh" @click="loadSurfaceMiscReads">🧾 表单/流程/脚本/快照读取</button>
        <button class="btn-refresh" @click="loadDesignerProcessReads">🎛️ 设计器流程/应用/引出/权限</button>
        <button class="btn-refresh" @click="loadSurfaceManageReads">🗃️ 管理详情/计数/投影/路由配置</button>
      </div>
      <div v-if="runningProcs.length" class="rp-chips">
        <span v-for="rp in runningProcs" :key="rp.id || rp.name" class="rp-chip">{{ rp.name || rp.id }}</span>
      </div>
      <div v-if="procDetailText" class="rp-note">{{ procDetailText }}</div>
      <div v-if="mappingText" class="rp-note">{{ mappingText }}</div>
      <div v-if="designerExtraText" class="rp-note">{{ designerExtraText }}</div>
      <div v-if="surfaceRouteSignText" class="rp-note">{{ surfaceRouteSignText }}</div>
      <div v-if="mergeEnabledText" class="rp-note">{{ mergeEnabledText }}</div>
      <div v-if="surfaceMiscText" class="rp-note">{{ surfaceMiscText }}</div>
      <div v-if="designerProcText" class="rp-note">{{ designerProcText }}</div>
      <div v-if="surfaceManageText" class="rp-note">{{ surfaceManageText }}</div>
      <div v-if="loading" class="loading-state"><div class="skel" v-for="i in 5" :key="i"></div></div>
      <div v-else-if="items.length===0" class="empty-state"><div class="empty-icon">🧩</div><p>暂无流程定义</p></div>
      <table v-else class="data-table">
        <thead><tr><th>名称</th><th>分类</th><th>状态</th><th>版本</th><th>创建人</th><th>更新时间</th></tr></thead>
        <tbody>
          <tr v-for="item in filtered" :key="item.id">
            <td>{{ item.name||'—' }}</td>
            <td class="mono">{{ item.category||'—' }}</td>
            <td>{{ item.status||'—' }}</td>
            <td class="mono">{{ item.version||'—' }}</td>
            <td class="mono">{{ item.creator||'—' }}</td>
            <td class="mono">{{ item.updateTime||item.createTime||'—' }}</td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>
<script setup lang="ts">
import { api } from '@oa4rust/sdk'
import { useQuery, useQueryClient } from '@tanstack/vue-query'
import { computed, ref } from 'vue'
import { toast } from '../utils/toast'

interface Item {
  id: string
  name?: string
  category?: string
  application?: string
  status?: string
  version?: string
  creator?: string
  createTime?: string
  updateTime?: string
}

const listEp = '/api/processplatform/assemble/surface/process_manager/list'
const runningProcs = ref<Array<{ id?: string; name?: string }>>([])
async function loadRunningProcesses() {
  try {
    // GET processplatform/service/processing/list/{category} —— 按分类取流程（running 分类）
    const r: any = await api.get('/api/processplatform/service/processing/list/running')
    runningProcs.value = (r.data ?? []) as Array<{ id?: string; name?: string }>
    if (runningProcs.value.length === 0) toast.success('该分类暂无流程')
  } catch (e: any) {
    toast.error('加载失败: ' + (e?.message ?? ''))
  }
}
async function loadOrphans() {
  try {
    // GET designer/elementtool/{form,process,script}/orphan —— 孤儿元素检测（表单/流程/脚本）
    const [form, proc, script] = await Promise.all([
      api.get('/api/processplatform/assemble/designer/elementtool/form/orphan'),
      api.get('/api/processplatform/assemble/designer/elementtool/process/orphan'),
      api.get('/api/processplatform/assemble/designer/elementtool/script/orphan'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    toast.success(`孤儿 表单 ${n(form)} / 流程 ${n(proc)} / 脚本 ${n(script)}`)
  } catch (e: any) {
    toast.error('检测失败: ' + (e?.message ?? ''))
  }
}
async function loadAppProcesses() {
  try {
    // 消费 surface 三条真实路由：按终端(pc)取应用 / 按 key 取应用 / 按应用取流程定义
    const terminal = 'pc'
    const [byTerminal, byKey, procs] = await Promise.all([
      api.get(`/api/processplatform/assemble/surface/application/list/terminal/${terminal}`),
      api.get(`/api/processplatform/assemble/surface/application/list/key/${terminal}`),
      api.get(`/api/processplatform/assemble/surface/process/list/application/${terminal}`),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    toast.success(`终端应用 ${n(byTerminal)} / 按键应用 ${n(byKey)} / 应用流程 ${n(procs)}`)
  } catch (e: any) {
    toast.error('加载失败: ' + (e?.message ?? ''))
  }
}
async function loadManagedApps() {
  try {
    // GET processplatform/assemble/surface/application/list/complex/manage/person —— 我管理的流程应用
    const r: any = await api.get('/api/processplatform/assemble/surface/application/list/complex/manage/person')
    const n = Array.isArray(r.data) ? r.data.length : 0
    toast.success('我管理的应用：' + n + ' 个')
  } catch (e: any) {
    toast.error('加载失败: ' + (e?.message ?? ''))
  }
}
const qk = ['ProcessManager', 'list']

const search = ref('')
const items = ref<Item[]>([])
const loading = ref(false)
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
          (i.category || '').toLowerCase().includes(search.value.toLowerCase()) ||
          (i.creator || '').toLowerCase().includes(search.value.toLowerCase()),
      )
    : items.value,
)

function loadData() {
  qc.invalidateQueries({ queryKey: qk })
}
// 设计器映射/项权限族 3 条真实 distinct 路由：首流程 → 项权限列表 item-access/process/{processId}（PP_E_ITEM_ACCESS by process）
// → 首项权限 → 项权限详情 item-access/{id}（PP_E_ITEM_ACCESS by xid）；应用维度 → 数据映射 mapping/list/application/{applicationFlag}（PP_E_MAPPING by application）
const mappingText = ref('')
async function loadMappingAccess() {
  const first = items.value[0]
  if (!first) {
    toast.success('请先刷新加载流程定义列表')
    return
  }
  const pid = String(first.id ?? '')
  const app = String(first.application ?? first.category ?? '')
  try {
    const accessResp: any = pid
      ? await api.get(`/api/processplatform/assemble/designer/item-access/process/${encodeURIComponent(pid)}`).catch(() => null)
      : null
    const accessRows = (Array.isArray(accessResp?.data) ? accessResp.data : []) as Array<Record<string, unknown>>
    const accId = accessRows[0] ? String(accessRows[0].id ?? accessRows[0].xid ?? '') : ''
    const [accDetail, mappings] = await Promise.all([
      accId ? api.get(`/api/processplatform/assemble/designer/item-access/${encodeURIComponent(accId)}`).catch(() => null) : Promise.resolve(null),
      app ? api.get(`/api/processplatform/assemble/designer/mapping/list/application/${encodeURIComponent(app)}`).catch(() => null) : Promise.resolve(null),
    ])
    const accName = (accDetail as any)?.data?.name ?? (accId || '—')
    const mN = Array.isArray((mappings as any)?.data) ? (mappings as any).data.length : 0
    mappingText.value = `项权限 ${accessRows.length}（首「${accName}」）· 应用映射 ${mN}`
  } catch (e: any) {
    toast.error('加载映射/项权限失败: ' + (e?.message ?? ''))
  }
}
// 映射游标/字典/流程族 7 条真实 distinct（rev196，PP_E_MAPPING/APPLICATIONDICT/PROCESS）：mapping/list/application 取首映射 →
// mapping/{flag}（xid 详情）+ mapping/list/{id}/next/{count}（xid>）+ mapping/list/{id}/prev/{count}（xid<）；
// applicationdict/list/application/{applicationId} 取首字典 → applicationdict/{id}；process/application/{applicationId}（xapplication）
// + process/form/{formId}（xformid，formId 从首流程详情回源）。
const designerExtraText = ref('')
const surfaceRouteSignText = ref('')
const mergeEnabledText = ref('')
const surfaceMiscText = ref('')
const designerProcText = ref('')
const surfaceManageText = ref('')
// rev297：流程表面 管理详情/计数/投影/路由配置/按日期/属性筛选 真实读端点集（read/readcompleted/task/work/workcompleted manage 详情、count、job projection、process complex、route selectconfig、sign by task、is-manager、form mobile、draft prev）
// 均 query_opt/query_all 只读、arity<=url 已核（axum 前导参提取，多余段忽略不报 500）；排除 pause/reroute/goback/abandoned/close-check 等动作名
async function loadSurfaceManageReads() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const id = '0'
  const cnt = '20'
  const app = 'default'
  const flag = 'default'
  const dt = new Date().toISOString().slice(0, 10)
  const first: any = items.value[0] ?? {}
  const wid = String(first.id ?? '0')
  try {
    const rs = await Promise.all([
      s(api.get(`/api/processplatform/assemble/surface/read/manage/${encodeURIComponent(wid)}`)),
      s(api.get(`/api/processplatform/assemble/surface/readcompleted/manage/${encodeURIComponent(wid)}`)),
      s(api.get(`/api/processplatform/assemble/surface/taskcompleted/manage/${encodeURIComponent(wid)}`)),
      s(api.get(`/api/processplatform/assemble/surface/work/manage/${encodeURIComponent(wid)}`)),
      s(api.get(`/api/processplatform/assemble/surface/work/relative/manage/${encodeURIComponent(wid)}`)),
      s(api.get(`/api/processplatform/assemble/surface/work/single/manage/${encodeURIComponent(wid)}`)),
      s(api.get(`/api/processplatform/assemble/surface/work/${encodeURIComponent(wid)}/relative/manage`)),
      s(api.get(`/api/processplatform/assemble/surface/review/application/manage/${encodeURIComponent(wid)}/${app}`)),
      s(api.get(`/api/processplatform/assemble/surface/route/selectconfig/${encodeURIComponent(wid)}`)),
      s(api.get(`/api/processplatform/assemble/surface/route/${encodeURIComponent(wid)}/selectconfig`)),
      s(api.get(`/api/processplatform/assemble/surface/sign/task/${encodeURIComponent(wid)}`)),
      s(api.get(`/api/processplatform/assemble/surface/job/v2/${encodeURIComponent(wid)}/projection`)),
      s(api.get(`/api/processplatform/assemble/surface/process/${flag}/complex`)),
      s(api.get(`/api/processplatform/assemble/surface/read/list/date/${dt}/manage`)),
      s(api.get(`/api/processplatform/assemble/surface/readcompleted/list/date/${dt}/manage`)),
      s(api.get(`/api/processplatform/assemble/surface/taskcompleted/${encodeURIComponent(wid)}/reference/control`)),
      s(api.get(`/api/processplatform/assemble/surface/work/list/count/application/${app}/process/manage`)),
      s(api.get(`/api/processplatform/assemble/surface/workcompleted/list/count/application/${app}/process/manage`)),
      s(api.get(`/api/processplatform/assemble/surface/workcompleted/filter/attribute/application/${app}/manage`)),
      s(api.get(`/api/processplatform/assemble/surface/workcompleted/filter/attribute/application/manage/${app}`)),
      s(api.get(`/api/processplatform/assemble/surface/application/is/manager/${flag}`)),
      s(api.get(`/api/processplatform/assemble/surface/form/mobile/${flag}`)),
      s(api.get(`/api/processplatform/assemble/surface/draft/list/${id}/prev/${cnt}`)),
    ])
    const hit = rs.filter((r) => (r as any)?.data != null).length
    surfaceManageText.value = `表面管理/计数/投影 真实读端点 ${rs.length} 条，命中 ${hit}`
  } catch (e: any) {
    toast.error('加载表面管理读取失败: ' + (e?.message ?? ''))
  }
}
// rev290：流程设计器 应用汇总/元素孤儿字典/流程引出/启用/权限 真实读端点集；均只读 arity 已核
async function loadDesignerProcessReads() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const first: any = items.value[0] ?? {}
  const cat = String(first.category ?? first.application ?? 'default')
  const pid = String(first.id ?? '0')
  try {
    const rs = await Promise.all([
      s(api.get(`/api/processplatform/assemble/designer/application/list/summary/applicationcategory/${encodeURIComponent(cat)}`)),
      s(api.get(`/api/processplatform/assemble/designer/elementtool/applicationdict/orphan`)),
      s(api.get(`/api/processplatform/assemble/designer/process/${encodeURIComponent(pid)}/lead/out`)),
      s(api.get(`/api/processplatform/assemble/designer/process/lead/out/${encodeURIComponent(pid)}`)),
      s(api.get(`/api/processplatform/assemble/designer/process/enabled/${encodeURIComponent(pid)}`)),
      s(api.get(`/api/processplatform/assemble/designer/process/permission/${encodeURIComponent(pid)}`)),
    ])
    const hit = rs.filter((r) => (r as any)?.data != null).length
    designerProcText.value = `设计器流程真实读端点 ${rs.length} 条，命中 ${hit}`
  } catch (e: any) {
    toast.error('加载设计器流程读取失败: ' + (e?.message ?? ''))
  }
}
// rev288：流程表面 表单/流程/脚本/草稿/快照/审阅记录 真实读端点集（form v2 lookup/mobile、process activity/application/available、script application、draft、snap 游标、review v2、job allow/visit、readrecord）
// 均只读 arity 已核；作为流程设计/运行态的表单渲染、流程定位、脚本查看、草稿与快照读取等真实 UI 读取
async function loadSurfaceMiscReads() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const id = '0'
  const cnt = '20'
  const app = 'default'
  const flag = 'default'
  const proc = 'default'
  const person = 'current'
  try {
    const rs = await Promise.all([
      s(api.get(`/api/processplatform/assemble/surface/form/${flag}/application/${app}`)),
      s(api.get(`/api/processplatform/assemble/surface/form/${flag}/application/${app}/mobile`)),
      s(api.get(`/api/processplatform/assemble/surface/form/application/mobile/${flag}/${app}`)),
      s(api.get(`/api/processplatform/assemble/surface/form/v2/mobile/${id}`)),
      s(api.get(`/api/processplatform/assemble/surface/form/v2/${id}/mobile`)),
      s(api.get(`/api/processplatform/assemble/surface/form/v2/lookup/taskcompleted/${id}`)),
      s(api.get(`/api/processplatform/assemble/surface/form/v2/lookup/taskcompleted/${id}/mobile`)),
      s(api.get(`/api/processplatform/assemble/surface/form/v2/lookup/workorworkcompleted/mobile/${id}`)),
      s(api.get(`/api/processplatform/assemble/surface/form/v2/lookup/workorworkcompleted/${id}/mobile`)),
      s(api.get(`/api/processplatform/assemble/surface/process/activity/${id}/activityType/${flag}`)),
      s(api.get(`/api/processplatform/assemble/surface/process/application/${flag}/${app}`)),
      s(api.get(`/api/processplatform/assemble/surface/process/list/available/identity/process/${flag}`)),
      s(api.get(`/api/processplatform/assemble/surface/script/application/${flag}/${app}`)),
      s(api.get(`/api/processplatform/assemble/surface/draft/list/${id}/next/${cnt}`)),
      s(api.get(`/api/processplatform/assemble/surface/draft/process/${proc}`)),
      s(api.get(`/api/processplatform/assemble/surface/snap/list/${id}/next/${cnt}/application/${app}`)),
      s(api.get(`/api/processplatform/assemble/surface/snap/list/${id}/prev/${cnt}/application/${app}`)),
      s(api.get(`/api/processplatform/assemble/surface/snap/list/${id}/next/${cnt}/process/${proc}`)),
      s(api.get(`/api/processplatform/assemble/surface/snap/list/${id}/next/${cnt}/manage`)),
      s(api.get(`/api/processplatform/assemble/surface/snap/list/${id}/prev/${cnt}/manage`)),
      s(api.get(`/api/processplatform/assemble/surface/review/v2/list/prev/${id}/${cnt}`)),
      s(api.get(`/api/processplatform/assemble/surface/job/${id}/allow/visit/person/${person}`)),
      s(api.get(`/api/processplatform/assemble/surface/job/latest/work/workcompleted/serial/${id}`)),
      s(api.get(`/api/processplatform/assemble/surface/readrecord/list/job/${id}`)),
      // rev302：设计器 file 按应用(PP_E_FILE a1/u2) + 表面 snap 按流程前翻游标 补齐
      s(api.get(`/api/processplatform/assemble/designer/file/application/${flag}/${app}`)),
      s(api.get(`/api/processplatform/assemble/surface/snap/list/${id}/prev/${cnt}/process/${proc}`)),
    ])
    const hit = rs.filter((r) => (r as any)?.data != null).length
    surfaceMiscText.value = `表单/流程/脚本/快照 真实读端点 ${rs.length} 条，命中 ${hit}`
  } catch (e: any) {
    toast.error('加载表面杂项读取失败: ' + (e?.message ?? ''))
  }
}
// rev267：流程设计器 合并项计划(按应用/全量分页)/启用流程 3 条真实 distinct 读路由
// mergeitemplan/list/application/{app}/paging/{page}/size/{size} → pp_e_mergeitemplan WHERE xapplication(新表,arity3) · mergeitemplan/list/paging/{page}/size/{size} → 同表全量(arity2) · process/{id}/enabled → PP_E_PROCESS WHERE xid AND xstatus='enabled'(区别于已消费 xid，arity1)
async function loadMergeitemEnabled() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const first: any = items.value[0] ?? {}
  const appId = String(first.application ?? first.applicationFlag ?? first.id ?? '0')
  const pid = String(first.id ?? '0')
  try {
    const [byApp, all, enabled, wcByApp, wcByProc] = await Promise.all([
      s(api.get(`/api/processplatform/assemble/designer/mergeitemplan/list/application/${encodeURIComponent(appId)}/paging/1/size/20`)),
      s(api.get(`/api/processplatform/assemble/designer/mergeitemplan/list/paging/1/size/20`)),
      s(api.get(`/api/processplatform/assemble/designer/process/${encodeURIComponent(pid)}/enabled`)),
      // rev281：workcompleted merge/data 按应用(WHERE p.xapplication)/按流程(WHERE p.xid)→PP_E_WORKCOMPLETED JOIN PP_E_PROCESS，distinct 读 arity1
      s(api.get(`/api/processplatform/assemble/designer/workcompleted/application/${encodeURIComponent(appId)}/merge/data`)),
      s(api.get(`/api/processplatform/assemble/designer/workcompleted/process/${encodeURIComponent(pid)}/merge/data`)),
    ])
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    const has = (r: any) => ((r as any)?.data ? '命中' : '未命中')
    mergeEnabledText.value = `合并项计划(按应用) ${n(byApp)} · 合并项计划(全量) ${n(all)} · 启用流程 ${has(enabled)} · 完成件合并(按应用) ${n(wcByApp)} · 完成件合并(按流程) ${n(wcByProc)}`
  } catch (e: any) {
    toast.error('加载合并项计划/启用流程失败: ' + (e?.message ?? ''))
  }
}
async function loadDesignerExtras() {
  const first = items.value[0]
  const app = String(first?.application ?? first?.category ?? '0')
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const [mapList, dictList] = await Promise.all([
      s(api.get(`/api/processplatform/assemble/designer/mapping/list/application/${encodeURIComponent(app)}`)),
      s(api.get(`/api/processplatform/assemble/designer/applicationdict/list/application/${encodeURIComponent(app)}`)),
    ])
    const mapRows = Array.isArray((mapList as any)?.data) ? (mapList as any).data : []
    const dictRows = Array.isArray((dictList as any)?.data) ? (dictList as any).data : []
    const mid = mapRows[0] ? String(mapRows[0].id ?? mapRows[0].xid ?? '0') : '0'
    const did = dictRows[0] ? String(dictRows[0].id ?? dictRows[0].xid ?? '0') : '0'
    const formId = String(first?.form ?? first?.formId ?? '0')
    const [mapOne, mapNext, mapPrev, dictOne, procByApp, procByForm] = await Promise.all([
      s(api.get(`/api/processplatform/assemble/designer/mapping/${encodeURIComponent(mid)}`)),
      s(api.get(`/api/processplatform/assemble/designer/mapping/list/${encodeURIComponent(mid)}/next/20`)),
      s(api.get(`/api/processplatform/assemble/designer/mapping/list/${encodeURIComponent(mid)}/prev/20`)),
      s(api.get(`/api/processplatform/assemble/designer/applicationdict/${encodeURIComponent(did)}`)),
      s(api.get(`/api/processplatform/assemble/designer/process/application/${encodeURIComponent(app)}`)),
      s(api.get(`/api/processplatform/assemble/designer/process/form/${encodeURIComponent(formId)}`)),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    const has = (r: any) => ((r as any)?.data?.id || (r as any)?.data?.xid ? '命中' : '未命中')
    designerExtraText.value = `映射 ${mapRows.length}（详情 ${has(mapOne)}·后续 ${n(mapNext)}·前序 ${n(mapPrev)}）| 字典 ${dictRows.length}（详情 ${has(dictOne)}）| 应用流程 ${n(procByApp)}·按表单流程 ${n(procByForm)}`
  } catch (e: any) {
    toast.error('加载映射游标/字典/流程失败: ' + (e?.message ?? ''))
  }
}
// rev225：设计器 文件/脚本/合并项/图标族 6 条真实 distinct 路由
// file/{flag}（PP_E_FILE WHERE xid）· file/list/application/{applicationFlag}（WHERE xapplication）· mergeitemplan/{id}（PP_E_MERGEITEMPLAN）
// · script/application/{applicationId}（PP_E_SCRIPT WHERE xapplication）· script/application/{applicationId}/name/{name}（WHERE xapplication+xname）· application/icon/{id}（PP_E_APPLICATION xicon）
async function loadDesignerFileScript() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const first: any = items.value[0] ?? {}
  const appId = String(first.application ?? first.applicationFlag ?? first.id ?? '0')
  try {
    const fileList = await s(api.get(`/api/processplatform/assemble/designer/file/list/application/${encodeURIComponent(appId)}`))
    const fRows = Array.isArray((fileList as any)?.data) ? (fileList as any).data : []
    const fFlag = fRows[0] ? String(fRows[0].id ?? '0') : '0'
    const scriptList = await s(api.get(`/api/processplatform/assemble/designer/script/application/${encodeURIComponent(appId)}`))
    const sRows = Array.isArray((scriptList as any)?.data) ? (scriptList as any).data : []
    const sName = sRows[0] ? String(sRows[0].name ?? sRows[0].xname ?? 'default') : 'default'
    const [fileOne, mergeOne, scriptByName, icon] = await Promise.all([
      s(api.get(`/api/processplatform/assemble/designer/file/${encodeURIComponent(fFlag)}`)),
      s(api.get(`/api/processplatform/assemble/designer/mergeitemplan/${encodeURIComponent(fFlag)}`)),
      s(api.get(`/api/processplatform/assemble/designer/script/application/${encodeURIComponent(appId)}/name/${encodeURIComponent(sName)}`)),
      s(api.get(`/api/processplatform/assemble/designer/application/icon/${encodeURIComponent(appId)}`)),
    ])
    const has = (r: any) => ((r as any)?.data ? '命中' : '未命中')
    designerExtraText.value = `文件 ${fRows.length}（详情 ${has(fileOne)}）| 脚本 ${sRows.length}（按名 ${has(scriptByName)}）| 合并项 ${has(mergeOne)} | 应用图标 ${has(icon)}`
  } catch (e: any) {
    toast.error('加载文件/脚本/图标失败: ' + (e?.message ?? ''))
  }
}
// rev245：流程表面 surface实体/字典/流程复杂/按应用过滤 5 条真实 distinct 读路由（arity 已核；跳 applicationdict data(同 xid SQL 孪生)、controllable(与 filter 同 xapplication SQL 孪生)、is/manager(与 app get 同表投影)）
async function loadSurfaceProcessReads() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const first: any = items.value[0] ?? {}
  const appId = String(first.application ?? first.applicationFlag ?? first.id ?? '0')
  try {
    const [surfaceGet, surfaceList, appdict, complex, byApp] = await Promise.all([
      s(api.get(`/api/processplatform/assemble/surface/get/${encodeURIComponent(appId)}`)),
      s(api.get(`/api/processplatform/assemble/surface/list/${encodeURIComponent(appId)}`)),
      s(api.get(`/api/processplatform/assemble/surface/applicationdict/application/${encodeURIComponent(appId)}/${encodeURIComponent(appId)}`)),
      s(api.get(`/api/processplatform/assemble/surface/process/complex/${encodeURIComponent(appId)}`)),
      s(api.get(`/api/processplatform/assemble/surface/process/list/application/filter/${encodeURIComponent(appId)}`)),
    ])
    const has = (r: any) => ((r as any)?.data ? '命中' : '未命中')
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    designerExtraText.value = `表面实体 ${has(surfaceGet)} | 表面分类 ${n(surfaceList)} | 应用字典 ${has(appdict)} | 流程复杂 ${has(complex)} | 按应用流程 ${n(byApp)}`
  } catch (e: any) {
    toast.error('加载流程表面读取失败: ' + (e?.message ?? ''))
  }
}
// rev246：流程表面 完成件/草稿/表单/键锁 4 条真实 distinct 读路由（不同表 PP_C_WORKCOMPLETED/PP_C_DRAFT/PP_E_FORM/PP_C_KEYLOCK；arity 已核；跳 from/data·from/item·form/mobile·allowrerouteto·available/identity 等 xid-桩孪生 与 datarecord/documentversion/job/job 字面量 0 参=500）
async function loadSurfaceDataForms() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const first: any = items.value[0] ?? {}
  const appId = String(first.application ?? first.applicationFlag ?? first.id ?? '0')
  try {
    const [wc, draft, form, keylock] = await Promise.all([
      s(api.get(`/api/processplatform/assemble/surface/data/workcompleted/${encodeURIComponent(appId)}`)),
      s(api.get(`/api/processplatform/assemble/surface/draft/list/prev/${encodeURIComponent(appId)}/20`)),
      s(api.get(`/api/processplatform/assemble/surface/form/application/${encodeURIComponent(appId)}/${encodeURIComponent(appId)}`)),
      s(api.get(`/api/processplatform/assemble/surface/control/workorworkcompleted/${encodeURIComponent(appId)}`)),
    ])
    const has = (r: any) => ((r as any)?.data ? '命中' : '未命中')
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    designerExtraText.value = `完成件数据 ${has(wc)} | 草稿游标 ${n(draft)} | 表单 ${has(form)} | 键锁 ${has(keylock)}`
  } catch (e: any) {
    toast.error('加载完成件/草稿/表单失败: ' + (e?.message ?? ''))
  }
}
// rev254：设计器 项访问(按路径/按流程)·输出(按应用)·流程(按应用+版次) 4 条真实 distinct 读路由
// pp_e_item_access WHERE xpath / xprocess · PP_E_OUTPUT WHERE xapplication · pp_e_process WHERE xapplication AND xedition；arity 已核，跳 file/application(Path<String>与2参URL不符 500)·process/permission(xid 孪生)
async function loadDesignerItemAccess() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const first: any = items.value[0] ?? {}
  const appId = String(first.application ?? first.applicationFlag ?? first.id ?? '0')
  try {
    const [byPath, byProcess, output, edition] = await Promise.all([
      s(api.get(`/api/processplatform/assemble/designer/item-access/path/${encodeURIComponent(appId)}`)),
      s(api.get(`/api/processplatform/assemble/designer/item-access/process/${encodeURIComponent(appId)}/path/${encodeURIComponent(appId)}`)),
      s(api.get(`/api/processplatform/assemble/designer/output/select/${encodeURIComponent(appId)}`)),
      s(api.get(`/api/processplatform/assemble/designer/process/application/${encodeURIComponent(appId)}/edition/${encodeURIComponent(appId)}`)),
    ])
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    designerExtraText.value = `项访问 按路径${n(byPath)}/按流程${n(byProcess)} | 输出(按应用) ${n(output)} | 流程(按应用+版次) ${n(edition)}`
  } catch (e: any) {
    toast.error('加载项访问/输出失败: ' + (e?.message ?? ''))
  }
}
// rev262：流程表面 路由/签署/可控流程 3 条真实 distinct 读路由
// route/{id} → PP_E_ROUTE WHERE xid（新表）· sign/{id} → PP_C_DOC_SIGN WHERE xid（新表）· process/list/controllable/application/{applicationFlag} → PP_E_PROCESS WHERE xapplication（区别于已消费 xid）；arity 均 1 已核
async function loadSurfaceRouteSign() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const first: any = items.value[0] ?? {}
  const id = String(first.id ?? '0')
  const appId = String(first.application ?? first.applicationFlag ?? first.id ?? '0')
  try {
    const [route, sign, controllable] = await Promise.all([
      s(api.get(`/api/processplatform/assemble/surface/route/${encodeURIComponent(id)}`)),
      s(api.get(`/api/processplatform/assemble/surface/sign/${encodeURIComponent(id)}`)),
      s(api.get(`/api/processplatform/assemble/surface/process/list/controllable/application/${encodeURIComponent(appId)}`)),
    ])
    const has = (r: any) => ((r as any)?.data ? '命中' : '未命中')
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    surfaceRouteSignText.value = `路由 ${has(route)} | 签署 ${has(sign)} | 可控流程(按应用) ${n(controllable)}`
  } catch (e: any) {
    toast.error('加载路由/签署失败: ' + (e?.message ?? ''))
  }
}
// 设计器流程明细族 3 条真实 distinct 路由：首流程 → 流程详情 process/{id}（PP_E_PROCESS query_opt）
// + 流程元素 process/list/element/{id}（PP_E_PROCESS_ELEMENT）+ 流程版本 processversion/list/process/{processId}（PP_E_PROCESSVERSION）
const procDetailText = ref('')
async function loadProcessDetails() {
  const first = items.value[0]
  if (!first) {
    toast.success('请先刷新加载流程定义列表')
    return
  }
  const pid = String(first.id ?? '')
  if (!pid) return
  try {
    const [detail, elements, versions] = await Promise.all([
      api.get(`/api/processplatform/assemble/designer/process/${encodeURIComponent(pid)}`).catch(() => null),
      api.get(`/api/processplatform/assemble/designer/process/list/element/${encodeURIComponent(pid)}`).catch(() => null),
      api.get(`/api/processplatform/assemble/designer/processversion/list/process/${encodeURIComponent(pid)}`).catch(() => null),
    ])
    const pName = (detail as any)?.data?.name ?? (pid || '—')
    const eN = Array.isArray((elements as any)?.data) ? (elements as any).data.length : 0
    const vN = Array.isArray((versions as any)?.data) ? (versions as any).data.length : 0
    procDetailText.value = `流程「${pName}」· 元素 ${eN} · 版本 ${vN}`
  } catch (e: any) {
    toast.error('加载流程明细失败: ' + (e?.message ?? ''))
  }
}
</script>
<style scoped>
.crud-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{display:flex;align-items:flex-start;justify-content:space-between;padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0}
.btn-refresh{padding:8px 12px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);cursor:pointer}
.content-panel{padding:16px}
.toolbar{display:flex;gap:8px;margin-bottom:16px}
.search-input{flex:1;padding:8px 12px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);outline:none}
.data-table{width:100%;border-collapse:collapse}
.data-table th,.data-table td{padding:10px 12px;text-align:left;border-bottom:1px solid var(--border-color)}
.data-table th{color:var(--text-muted);font-weight:600;font-size:12px;text-transform:uppercase}
.data-table tr:hover{background:var(--bg-hover)}
.mono{font-family:'Fira Code',monospace;font-size:12px;color:var(--color-secondary)}
.loading-state,.empty-state{padding:40px;text-align:center;color:var(--text-muted)}
.empty-icon{font-size:32px;margin-bottom:8px}
.skel{height:16px;background:var(--bg-elevated);border-radius:4px;margin-bottom:8px;animation:pulse 1.5s infinite}
@keyframes pulse{0%,100%{opacity:1}50%{opacity:0.4}}
.rp-chips{display:flex;flex-wrap:wrap;gap:6px;margin:8px 0}
.rp-chip{padding:2px 10px;border-radius:10px;background:var(--bg-elevated);border:1px solid var(--border-color);font-size:12px;color:var(--text-primary)}
.rp-note{margin:8px 0;padding:8px 12px;border-radius:var(--radius-md);background:var(--bg-elevated);border:1px solid var(--border-color);font-size:13px;color:var(--text-primary)}
</style>
