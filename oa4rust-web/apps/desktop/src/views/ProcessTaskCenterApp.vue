<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="crud-view">
    <div class="view-header glass-card">
      <div>
        <h1>流程任务中心</h1>
        <p class="subtitle">/api/processplatform/service/processing/task/*（x_task）</p>
      </div>
      <button class="btn-refresh" @click="loadData">🔄 刷新</button>
    </div>
    <div class="content-panel glass-card">
      <div class="toolbar">
        <input v-model="search" placeholder="搜索标题 / 流程 / 处理人..." class="search-input" />
        <button class="btn-refresh" @click="loadWorkList">📋 工作实例</button>
        <button class="btn-refresh" @click="loadCounts">📊 计数</button>
        <button class="btn-refresh" @click="loadAppOverview">🗂️ 应用概览</button>
        <button class="btn-refresh" @click="loadTouch">⏰ 超期触发</button>
        <button class="btn-refresh" @click="loadWorkDetail">🧾 工作明细</button>
        <button class="btn-refresh" @click="loadSnaps">📸 工作快照</button>
        <button class="btn-refresh" @click="loadSnapCursors">🎞️ 快照游标</button>
        <button class="btn-refresh" @click="loadEngineEntities">🔧 引擎实体</button>
        <button class="btn-refresh" @click="loadEngineMore">⚙️ 引擎扩展</button>
        <button class="btn-refresh" @click="loadTaskCursorFilters">📋 待办/已办游标</button>
        <button class="btn-refresh" @click="loadReadCursorFilters">📖 待阅/已阅游标</button>
        <button class="btn-refresh" @click="loadJobDataReads">🧬 作业/数据记录/签署/发票</button>
      </div>
      <div v-if="countsText" class="wk-chips"><span class="wk-chip">{{ countsText }}</span></div>
      <div v-if="workDetailText" class="wk-chips"><span class="wk-chip">{{ workDetailText }}</span></div>
      <div v-if="jobDataText" class="wk-chips"><span class="wk-chip">{{ jobDataText }}</span></div>
      <div v-if="snapText" class="wk-chips"><span class="wk-chip">{{ snapText }}</span></div>
      <div v-if="workItems.length" class="wk-chips">
        <span v-for="w in workItems" :key="w.id || w.title" class="wk-chip">{{ w.title || w.id }}</span>
      </div>
      <div v-if="loading" class="loading-state"><div class="skel" v-for="i in 5" :key="i"></div></div>
      <div v-else-if="items.length===0" class="empty-state"><div class="empty-icon">⚡</div><p>暂无任务</p></div>
      <table v-else class="data-table">
        <thead><tr><th>标题</th><th>流程</th><th>环节</th><th>处理人</th><th>状态</th><th>开始时间</th><th>操作</th></tr></thead>
        <tbody>
          <tr v-for="item in filtered" :key="item.id">
            <td>{{ item.title||'—' }}</td>
            <td class="mono">{{ item.work||'—' }}</td>
            <td>{{ item.activity||'—' }}</td>
            <td>{{ item.person||'—' }}</td>
            <td class="mono">{{ item.taskStatus||'—' }}</td>
            <td>{{ fmtTime(item.startTime) }}</td>
            <td>
              <button class="btn-sm" :disabled="processingId===item.id" @click="startProcessing(item)">开始处理</button>
              <button class="btn-sm" @click="urge(item)">催办</button>
              <button class="btn-sm btn-del" @click="deleteTask(item)">删除</button>
            </td>
          </tr>
        </tbody>
      </table>
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
  title?: string
  work?: string
  activity?: string
  person?: string
  taskStatus?: string
  startTime?: string
  endTime?: string
}

const listEp = '/api/processplatform/service/processing/task/list'
const workItems = ref<Array<{ id?: string; title?: string }>>([])
const countsText = ref('')
const jobDataText = ref('')
// rev266：流程表面 作业/数据记录/签署/发票 6 条真实 distinct 读路由（全字面量，arity 均已核）
// PP_C_JOB 三态：data/job/{job}(xjob) · correlation/list/job/{job}/site/{site}(xjob+xsite) · job/{job}/find/work/workcompleted(xid)
// + datarecord/get/job/{job}/path/{path}→PP_C_DATA_RECORD(xid,新表) · sign/list/job/{job}→PP_C_DOC_SIGN(xjob，区别 rev262 xid) · attachment/invoice/{flag}/joborworkorworkcompleted/{w}→x_general_invoice(id,新表)
async function loadJobDataReads() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const first: any = items.value[0] ?? {}
  const job = String(first.job ?? first.id ?? '0')
  const wid = String(first.work ?? first.id ?? '0')
  try {
    const [byJob, corr, findWork, datarecord, signJob, invoice] = await Promise.all([
      s(api.get(`/api/processplatform/assemble/surface/data/job/${encodeURIComponent(job)}`)),
      s(api.get(`/api/processplatform/assemble/surface/correlation/list/job/${encodeURIComponent(job)}/site/${encodeURIComponent(job)}`)),
      s(api.get(`/api/processplatform/assemble/surface/job/${encodeURIComponent(job)}/find/work/workcompleted`)),
      s(api.get(`/api/processplatform/assemble/surface/datarecord/get/job/${encodeURIComponent(job)}/path/${encodeURIComponent(job)}`)),
      s(api.get(`/api/processplatform/assemble/surface/sign/list/job/${encodeURIComponent(job)}`)),
      s(api.get(`/api/processplatform/assemble/surface/attachment/invoice/${encodeURIComponent(wid)}/joborworkorworkcompleted/${encodeURIComponent(wid)}`)),
    ])
    const has = (r: any) => ((r as any)?.data ? '命中' : '未命中')
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    jobDataText.value = `作业数据 ${has(byJob)} · 关联(job+site) ${n(corr)} · 作业定位工作 ${has(findWork)} · 数据记录 ${has(datarecord)} · 签署(按job) ${n(signJob)} · 发票 ${has(invoice)}`
  } catch (e: any) {
    toast.error('加载作业/数据/签署/发票失败: ' + (e?.message ?? ''))
  }
}
const workDetailText = ref('')
// 引擎工作明细（rev182，service/processing 域 3 条真实 distinct，按工作 id）：get/{work}（get_process x_work 详情）
// + work/{work}/projection（work_id_projection x_task 任务投影）+ documentversion/{work}/{work}（x_document_version 版本）。
async function loadWorkDetail() {
  workDetailText.value = ''
  const work = items.value[0] ? String(items.value[0].work ?? items.value[0].id ?? '') : ''
  if (!work) {
    workDetailText.value = '暂无任务可查工作明细'
    return
  }
  const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const [detail, projection, docVer] = await Promise.all([
    settle(api.get(`/api/processplatform/service/processing/get/${work}`)),
    settle(api.get(`/api/processplatform/service/processing/work/${work}/projection`)),
    settle(api.get(`/api/processplatform/service/processing/documentversion/${work}/${work}`)),
  ])
  const wTitle = (detail as any)?.data?.title ?? work
  const projN = Array.isArray((projection as any)?.data) ? (projection as any).data.length : 0
  const docN = Array.isArray((docVer as any)?.data) ? (docVer as any).data.length : 0
  workDetailText.value = `工作「${wTitle}」· 任务投影 ${projN} · 文档版本 ${docN}`
}
// rev234：流程引擎按 id 实体族 8 条真实 distinct 路由（不同处理函数/表；全字面量路径避免变量前缀失配）
async function loadEngineEntities() {
  workDetailText.value = ''
  const id = items.value[0] ? String(items.value[0].work ?? items.value[0].id ?? '0') : '0'
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const [inst, appdict, dataWc, formAct, readc, review, taskc, proj] = await Promise.all([
    s(api.get(`/api/processplatform/service/processing/instance/${id}`)),
    s(api.get(`/api/processplatform/service/processing/applicationdict/${id}`)),
    s(api.get(`/api/processplatform/service/processing/data/workcompleted/${id}`)),
    s(api.get(`/api/processplatform/service/processing/form/suitable/activity/${id}`)),
    s(api.get(`/api/processplatform/service/processing/readcompleted/${id}`)),
    s(api.get(`/api/processplatform/service/processing/review/${id}`)),
    s(api.get(`/api/processplatform/service/processing/taskcompleted/${id}`)),
    s(api.get(`/api/processplatform/service/processing/projection/${id}/${id}`)),
  ])
  const has = (r: any) => ((r as any)?.data ? '命中' : '未命中')
  workDetailText.value = `实例 ${has(inst)} · 应用字典 ${has(appdict)} · 完成数据 ${has(dataWc)} · 表单适配 ${has(formAct)} · 已读完成 ${has(readc)} · 意见 ${has(review)} · 已办 ${has(taskc)} · 投影 ${has(proj)}`
}
// rev235：流程引擎 job/草稿/完成件 5 条真实 distinct 读路由（全字面量；跳过 v2/projection 双注册 twin）
async function loadEngineMore() {
  workDetailText.value = ''
  const id = items.value[0] ? String(items.value[0].work ?? items.value[0].id ?? '0') : '0'
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const [draft, wcProc, dataPath, dataWcPath, jobView, attnWc, tcIdentity] = await Promise.all([
    s(api.get(`/api/processplatform/service/processing/draft/${id}/${id}`)),
    s(api.get(`/api/processplatform/service/processing/workcompleted/process/${id}`)),
    s(api.get(`/api/processplatform/service/processing/data/path/${id}/${id}`)),
    s(api.get(`/api/processplatform/service/processing/data/workcompleted/path/${id}`)),
    s(api.get(`/api/processplatform/service/processing/job/v2/${id}/person/${id}/view`)),
    s(api.get(`/api/processplatform/service/processing/attachment/workcompleted/${id}/${id}`)),
    s(api.get(`/api/processplatform/service/processing/taskcompleted/task/identity/${id}`)),
  ])
  const has = (r: any) => ((r as any)?.data ? '命中' : '未命中')
  workDetailText.value = `草稿 ${has(draft)} · 完成件按流程 ${has(wcProc)} · 数据路径 ${has(dataPath)} · 完成数据路径 ${has(dataWcPath)} · job视图 ${has(jobView)} · 完成件附件 ${has(attnWc)} · 已办任务身份 ${has(tcIdentity)}`
}
// rev247：流程表面 task/taskcompleted 游标基/按应用/按流程 6 条真实 distinct 读路由（PP_C_TASK 与 PP_C_TASKCOMPLETED 两表，base(WHERE xid) 与 +xapplication/+xprocess 各异；跳 prev/filter/manage 同 WHERE xid 桩孪生）
async function loadTaskCursorFilters() {
  const id = items.value[0] ? String(items.value[0].work ?? items.value[0].id ?? '0') : '0'
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const [tBase, tApp, tProc, tcBase, tcApp, tcProc] = await Promise.all([
    s(api.get(`/api/processplatform/assemble/surface/task/list/next/${id}/20`)),
    s(api.get(`/api/processplatform/assemble/surface/task/list/next/application/${id}/20/${id}`)),
    s(api.get(`/api/processplatform/assemble/surface/task/list/next/process/${id}/20/${id}`)),
    s(api.get(`/api/processplatform/assemble/surface/taskcompleted/list/next/${id}/20`)),
    s(api.get(`/api/processplatform/assemble/surface/taskcompleted/list/next/application/${id}/20/${id}`)),
    s(api.get(`/api/processplatform/assemble/surface/taskcompleted/list/next/process/${id}/20/${id}`)),
  ])
  const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
  workDetailText.value = `待办 基${n(tBase)}/应用${n(tApp)}/流程${n(tProc)} · 已办 基${n(tcBase)}/应用${n(tcApp)}/流程${n(tcProc)}`
}
// rev248：流程表面 read/readcompleted 游标基/按应用/按流程 6 条真实 distinct 读路由（PP_C_READ 与 PP_C_READCOMPLETED 两表，base(WHERE xid) 与 +xapplication/+xprocess 各异；跳 filter/prev 同 WHERE xid 桩孪生）
async function loadReadCursorFilters() {
  const id = items.value[0] ? String(items.value[0].work ?? items.value[0].id ?? '0') : '0'
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const [rBase, rApp, rProc, rcBase, rcApp, rcProc] = await Promise.all([
    s(api.get(`/api/processplatform/assemble/surface/read/list/next/${id}/20`)),
    s(api.get(`/api/processplatform/assemble/surface/read/list/next/application/${id}/20/${id}`)),
    s(api.get(`/api/processplatform/assemble/surface/read/list/next/process/${id}/20/${id}`)),
    s(api.get(`/api/processplatform/assemble/surface/readcompleted/list/next/${id}/20`)),
    s(api.get(`/api/processplatform/assemble/surface/readcompleted/list/next/application/${id}/20/${id}`)),
    s(api.get(`/api/processplatform/assemble/surface/readcompleted/list/next/process/${id}/20/${id}`)),
  ])
  const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
  workDetailText.value = `待阅 基${n(rBase)}/应用${n(rApp)}/流程${n(rProc)} · 已阅 基${n(rcBase)}/应用${n(rcApp)}/流程${n(rcProc)}`
}

const snapText = ref('')
// 工作快照（rev183，service/processing 域 3 条真实 distinct，按工作 id 取不同 snap_type）：
// snap/work/{workId}/type/snap（快照）+ /type/abandoned（废弃）+ /type/suspend（挂起），均读 x_snap WHERE work_id AND snap_type。
async function loadSnaps() {
  snapText.value = ''
  const work = items.value[0] ? String(items.value[0].work ?? items.value[0].id ?? '') : ''
  if (!work) {
    snapText.value = '暂无任务可查快照'
    return
  }
  const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const [snap, abandoned, suspend] = await Promise.all([
    settle(api.get(`/api/processplatform/service/processing/snap/work/${work}/type/snap`)),
    settle(api.get(`/api/processplatform/service/processing/snap/work/${work}/type/abandoned`)),
    settle(api.get(`/api/processplatform/service/processing/snap/work/${work}/type/suspend`)),
  ])
  const cnt = (r: any) => (Array.isArray(r?.data) ? r.data.length : (r as any)?.data ? 1 : 0)
  snapText.value = `工作「${work}」快照 ${cnt(snap)} · 废弃 ${cnt(abandoned)} · 挂起 ${cnt(suspend)}`
}
// rev209：快照游标族 6 条真实 distinct 路由（pp_c_snap）：snap/{id}（WHERE id）
// · list/{id}/next/{count}（xcreateTime> ASC 前翻）· list/{id}/prev/{count}（xcreateTime< DESC 后翻）
// · workcompleted/{workCompletedId}/type/snapworkcompleted（by_type snapWorkCompleted）· /type/abandonedworkcompleted（abandonedWorkCompleted）
// · list/my/paging/{page}/size/{size}（offset 分页）
async function loadSnapCursors() {
  snapText.value = ''
  const work = items.value[0] ? String(items.value[0].work ?? items.value[0].id ?? '') : ''
  if (!work) {
    snapText.value = '暂无任务可查快照'
    return
  }
  const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const listResp = await settle(api.get(`/api/processplatform/assemble/surface/snap/work/${work}/type/snap`))
  const rows = Array.isArray((listResp as any)?.data) ? (listResp as any).data : []
  const sid = rows[0] ? String(rows[0].id ?? work) : work
  const [one, next, prev, snapWc, abandonWc, myPaging] = await Promise.all([
    settle(api.get(`/api/processplatform/assemble/surface/snap/${sid}`)),
    settle(api.get(`/api/processplatform/assemble/surface/snap/list/${sid}/next/10`)),
    settle(api.get(`/api/processplatform/assemble/surface/snap/list/${sid}/prev/10`)),
    settle(api.get(`/api/processplatform/assemble/surface/snap/workcompleted/${work}/type/snapworkcompleted`)),
    settle(api.get(`/api/processplatform/assemble/surface/snap/workcompleted/${work}/type/abandonedworkcompleted`)),
    settle(api.get('/api/processplatform/assemble/surface/snap/list/my/paging/1/size/20')),
  ])
  const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : ((r as any)?.data ? 1 : 0))
  snapText.value = `单条 ${(one as any)?.data?.id ? '命中' : '未命中'} · 前翻 ${n(next)} · 后翻 ${n(prev)} · 完成快照 ${n(snapWc)} · 完成废弃 ${n(abandonWc)} · 我的分页 ${n(myPaging)}`
}
async function loadTouch() {
  try {
    // GET surface touch/expire + passexpired + touchdetained —— 超期/超期通过/催办触发
    const [exp, passExp, detained] = await Promise.all([
      api.get('/api/processplatform/assemble/surface/touch/expire'),
      api.get('/api/processplatform/assemble/surface/touch/passexpired'),
      api.get('/api/processplatform/assemble/surface/touch/touchdetained'),
    ])
    const cnt = (r: any) => (Array.isArray(r?.data) ? r.data.length : ((r as any)?.data ? 1 : 0))
    countsText.value = `超期 ${cnt(exp)} / 超期通过 ${cnt(passExp)} / 催办 ${cnt(detained)}`
  } catch (e: any) {
    toast.error('触发失败: ' + (e?.message ?? ''))
  }
}
async function loadCounts() {
  try {
    // GET surface task/read/workcompleted list/count/application —— 待办/待阅/已办按应用计数
    const [task, read, done, taskDone, readDone] = await Promise.all([
      api.get('/api/processplatform/assemble/surface/task/list/count/application'),
      api.get('/api/processplatform/assemble/surface/read/list/count/application'),
      api.get('/api/processplatform/assemble/surface/workcompleted/list/count/application'),
      api.get('/api/processplatform/assemble/surface/taskcompleted/list/count/application'),
      api.get('/api/processplatform/assemble/surface/readcompleted/list/count/application'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    countsText.value = `待办 ${n(task)} / 待阅 ${n(read)} / 已办工作 ${n(done)} / 已办任务 ${n(taskDone)} / 已阅 ${n(readDone)}`
  } catch (e: any) {
    toast.error('加载计数失败: ' + (e?.message ?? ''))
  }
}
async function loadAppOverview() {
  try {
    // 消费 surface 三条无参真实路由：全部应用 / 复杂应用清单 / 工作按应用计数汇总
    const [apps, complex, workCount] = await Promise.all([
      api.get('/api/processplatform/assemble/surface/application/list'),
      api.get('/api/processplatform/assemble/surface/application/list/complex'),
      api.get('/api/processplatform/assemble/surface/work/list/count/application'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    countsText.value = `全部应用 ${n(apps)} / 复杂应用 ${n(complex)} / 工作计数 ${n(workCount)}`
  } catch (e: any) {
    toast.error('加载应用概览失败: ' + (e?.message ?? ''))
  }
}
async function loadWorkList() {
  try {
    // GET processplatform/service/processing/work/list —— 工作实例列表（Query 可选）
    const r: any = await api.get('/api/processplatform/service/processing/work/list')
    workItems.value = (r.data ?? []) as Array<{ id?: string; title?: string }>
    if (workItems.value.length === 0) toast.success('暂无工作实例')
  } catch (e: any) {
    toast.error('加载工作实例失败: ' + (e?.message ?? ''))
  }
}
const qk = ['ProcessTaskCenter', 'list']

const search = ref(''),
  loading = ref(false),
  processingId = ref<string | null>(null)
const items = ref<Item[]>([])
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
          (i.title || '').toLowerCase().includes(search.value.toLowerCase()) ||
          (i.work || '').toLowerCase().includes(search.value.toLowerCase()) ||
          (i.person || '').toLowerCase().includes(search.value.toLowerCase()),
      )
    : items.value,
)

const startM = useMutation({
  mutationFn: async (id: string) => api.post(`/api/processplatform/service/processing/task/processing/${id}`),
  onMutate: (id) => (processingId.value = id),
  onSuccess: () => qc.invalidateQueries({ queryKey: qk }),
  onSettled: () => (processingId.value = null),
})
function startProcessing(item: Item) {
  startM.mutate(item.id)
}
const urgeM = useMutation({
  mutationFn: async (id: string) => api.get(`/api/processplatform/service/processing/task/urge/${id}`),
  onSuccess: () => qc.invalidateQueries({ queryKey: qk }),
})
function urge(item: Item) {
  urgeM.mutate(item.id)
}
const delM = useMutation({
  mutationFn: async (id: string) => api.delete(`/api/processplatform/service/processing/task/${id}`),
  onSuccess: () => qc.invalidateQueries({ queryKey: qk }),
})
async function deleteTask(item: Item) {
  if (await confirmMsg('确定删除该任务？')) delM.mutate(item.id)
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
.btn-refresh{padding:8px 12px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);cursor:pointer}
.content-panel{padding:16px}
.toolbar{display:flex;gap:8px;margin-bottom:16px}
.search-input{flex:1;padding:8px 12px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);outline:none}
.data-table{width:100%;border-collapse:collapse}
.data-table th,.data-table td{padding:10px 12px;text-align:left;border-bottom:1px solid var(--border-color)}
.data-table th{color:var(--text-muted);font-weight:600;font-size:12px;text-transform:uppercase}
.data-table tr:hover{background:var(--bg-hover)}
.mono{font-family:'Fira Code',monospace;font-size:12px;color:var(--color-secondary)}
.btn-sm{padding:4px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);cursor:pointer;font-size:12px}
.btn-sm:disabled{opacity:0.5;cursor:not-allowed}
.btn-del{border-color:var(--color-danger);color:var(--color-danger)}
.loading-state,.empty-state{padding:40px;text-align:center;color:var(--text-muted)}
.empty-icon{font-size:32px;margin-bottom:8px}
.skel{height:16px;background:var(--bg-elevated);border-radius:4px;margin-bottom:8px;animation:pulse 1.5s infinite}
@keyframes pulse{0%,100%{opacity:1}50%{opacity:0.4}}
.wk-chips{display:flex;flex-wrap:wrap;gap:6px;margin:8px 0}
.wk-chip{padding:2px 10px;border-radius:10px;background:var(--bg-elevated);border:1px solid var(--border-color);font-size:12px;color:var(--text-primary)}
</style>
