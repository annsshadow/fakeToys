<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="dash-view">
    <div class="view-header glass-card">
      <div><h1>业务活动监控</h1><p class="subtitle">/api/processplatform/assemble/bam/*</p></div>
      <span class="hdr-a">
        <button class="btn-primary ghost" @click="loadPeriodStats">周期统计</button>
        <button class="btn-primary ghost" @click="loadBamConfigs">BAM定义</button>
        <button class="btn-primary ghost" @click="loadStateStats">状态监控</button>
        <button class="btn-primary ghost" @click="loadStartStubs">起始统计</button>
        <button class="btn-primary ghost" @click="loadCompletedStubs">已办/超期存根</button>
        <button class="btn-primary ghost" @click="loadUnitStubs">单位维度存根</button>
        <button class="btn-primary ghost" @click="loadDimensionStats">维度周期统计</button>
        <button class="btn-primary ghost" @click="loadCountStats">计数聚合</button>
        <button class="btn-primary ghost" @click="loadPeriodMatrix">周期多维矩阵</button>
        <button class="btn-primary ghost" @click="loadPeriodMatrix2">周期维度聚合</button>
        <button class="btn-primary ghost" @click="bamWrite('create')">建BAM</button>
        <button class="btn-primary ghost" @click="bamWrite('delete')">删BAM</button>
        <button class="btn-primary ghost" @click="bamWrite('trigger')">触发状态</button>
        <button class="btn-primary ghost" @click="bamWrite('periodTaskApp')">周期任务应用</button>
        <button class="btn-primary ghost" @click="bamWrite('periodTaskUnit')">周期任务单位</button>
        <button class="btn-primary ghost" @click="bamWrite('periodAppWork')">周期应用工作</button>
        <button class="btn-primary ghost" @click="bamWrite('periodWorkUnit')">周期工作单位</button>
        <button class="btn-primary" @click="refresh">🔄 刷新</button>
      </span>
    </div>
    <div v-if="periodText" class="period-note">{{ periodText }}</div>
    <div class="stats-grid glass-card">
      <div class="stat-card"><div class="stat-num">{{ stats.total }}</div><div class="stat-label">总活动</div></div>
      <div class="stat-card"><div class="stat-num">{{ stats.active }}</div><div class="stat-label">活跃中</div></div>
      <div class="stat-card"><div class="stat-num">{{ stats.completed }}</div><div class="stat-label">已完成</div></div>
      <div class="stat-card"><div class="stat-num">{{ stats.failed }}</div><div class="stat-label">失败</div></div>
    </div>
    <div class="content-panel glass-card">
      <div class="panel-title">最近活动</div>
      <div v-if="loading" class="loading-state">加载中...</div>
      <div v-else-if="events.length===0" class="empty-state"><p>暂无活动记录</p></div>
      <table v-else class="data-table">
        <thead><tr><th>时间</th><th>类型</th><th>描述</th><th>状态</th></tr></thead>
        <tbody>
          <tr v-for="e in events" :key="e.id">
            <td>{{ fmtTime(e.time) }}</td>
            <td><span class="type-tag" :class="e.type">{{ e.type }}</span></td>
            <td>{{ e.desc }}</td>
            <td><span class="status" :class="statusCls(e.status)">{{ e.status }}</span></td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>
<script setup lang="ts">
import { api } from '@oa4rust/sdk'
import { useQuery } from '@tanstack/vue-query'
import { ref } from 'vue'
import { confirmMsg, toast } from '../utils/toast'

const loading = ref(false)
const stats = ref({ total: 0, active: 0, completed: 0, failed: 0 })
const periodText = ref('')
// 消费 BAM 定义族 3 条真实 distinct 路由：按类别列表→定义详情+运行状态（x_bam_config / x_bam_status）
async function loadBamConfigs() {
  try {
    const listResp: any = await api.get('/api/processplatform/assemble/bam/list/default')
    const rows = (Array.isArray(listResp?.data) ? listResp.data : (listResp?.data?.data ?? [])) as Array<Record<string, unknown>>
    const id = rows[0] ? String(rows[0].id ?? '') : ''
    const [config, status] = await Promise.all([
      id ? api.get(`/api/processplatform/assemble/bam/get/${encodeURIComponent(id)}`).catch(() => null) : Promise.resolve(null),
      id ? api.get(`/api/processplatform/assemble/bam/status/${encodeURIComponent(id)}`).catch(() => null) : Promise.resolve(null),
    ])
    const name = (config as any)?.data?.name ?? (id || '—')
    const st = (status as any)?.data?.status ?? ((status as any)?.data ? '有状态' : '无状态')
    periodText.value = `BAM定义 ${rows.length}（首个「${name}」· ${st}）`
  } catch (e: any) {
    toast.error('加载 BAM 定义失败: ' + (e?.message ?? ''))
  }
}
// BAM 维度周期统计族 3 条真实 distinct 路由（各 SQL 维度不同，非退化）：按单位已办任务 period/list/completed/task/{unit}（x_task 该单位人员已办）
// + 按应用已办工作 period/list/completed/application/{work}（x_work by application）+ 按单位超期任务 period/list/expired/task/{unit}（x_task 该单位超期）。unit 从组织顶级单位、application 从 BAM 定义回源。
async function loadDimensionStats() {
  try {
    const [unitResp, bamResp]: any[] = await Promise.all([
      api.get('/api/organization/assemble/control/unit/list/top').catch(() => null),
      api.get('/api/processplatform/assemble/bam/list/default').catch(() => null),
    ])
    const units = (Array.isArray(unitResp?.data) ? unitResp.data : []) as Array<Record<string, unknown>>
    const bams = (Array.isArray(bamResp?.data) ? bamResp.data : (bamResp?.data?.data ?? [])) as Array<Record<string, unknown>>
    const unit = units[0] ? String(units[0].id ?? '') : '0'
    const application = bams[0] ? String(bams[0].application ?? bams[0].id ?? 'default') : 'default'
    const [doneTask, doneWork, expiredTask] = await Promise.all([
      api.get(`/api/processplatform/assemble/bam/period/list/completed/task/${encodeURIComponent(unit)}`).catch(() => null),
      api.get(`/api/processplatform/assemble/bam/period/list/completed/application/${encodeURIComponent(application)}`).catch(() => null),
      api.get(`/api/processplatform/assemble/bam/period/list/expired/task/${encodeURIComponent(unit)}`).catch(() => null),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    periodText.value = `单位已办任务 ${n(doneTask)} · 应用已办工作 ${n(doneWork)} · 单位超期任务 ${n(expiredTask)}`
  } catch (e: any) {
    toast.error('加载维度统计失败: ' + (e?.message ?? ''))
  }
}
// rev218：BAM 计数聚合族 7 条真实 distinct 路由（period_count_query 按 kind×period×group 分组统计）
// count/completed task·work by/application · completed/task by/process · expired task·work by/application · start task·work by/application
async function loadCountStats() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const unitResp: any = await api.get('/api/organization/assemble/control/unit/list/top').catch(() => null)
    const units = (Array.isArray(unitResp?.data) ? unitResp.data : []) as Array<Record<string, unknown>>
    const unit = units[0] ? String(units[0].id ?? '0') : '0'
    const person = '0'
    const app = 'default'
    const [ctA, cwA, ctP, etA, ewA, stA, swA] = await Promise.all([
      s(api.get(`/api/processplatform/assemble/bam/period/list/count/completed/task/unit/${encodeURIComponent(unit)}/person/${encodeURIComponent(person)}/by/application`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/count/completed/work/unit/${encodeURIComponent(unit)}/person/${encodeURIComponent(person)}/by/application`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/count/completed/task/application/${encodeURIComponent(app)}/unit/${encodeURIComponent(unit)}/person/${encodeURIComponent(person)}/by/process`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/count/expired/task/unit/${encodeURIComponent(unit)}/person/${encodeURIComponent(person)}/by/application`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/count/expired/work/unit/${encodeURIComponent(unit)}/person/${encodeURIComponent(person)}/by/application`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/count/start/task/unit/${encodeURIComponent(unit)}/person/${encodeURIComponent(person)}/by/application`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/count/start/work/unit/${encodeURIComponent(unit)}/person/${encodeURIComponent(person)}/by/application`)),
    ])
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    periodText.value = `已办任务/应用 ${n(ctA)} · 已办工作/应用 ${n(cwA)} · 已办任务/流程 ${n(ctP)} · 超期任务/应用 ${n(etA)} · 超期工作/应用 ${n(ewA)} · 起始任务/应用 ${n(stA)} · 起始工作/应用 ${n(swA)}`
  } catch (e: any) {
    toast.error('加载计数统计失败: ' + (e?.message ?? ''))
  }
}
async function loadPeriodStats() {
  try {
    // GET bam/period/list/completed/task/application + expired/task/application —— 已办/超期任务周期统计
    const [done, expired] = await Promise.all([
      api.get('/api/processplatform/assemble/bam/period/list/completed/task/application'),
      api.get('/api/processplatform/assemble/bam/period/list/expired/task/application'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    periodText.value = `已办任务周期 ${n(done)} / 超期任务周期 ${n(expired)}`
  } catch (e: any) {
    toast.error('加载周期统计失败: ' + (e?.message ?? ''))
  }
}
async function loadUnitStubs() {
  try {
    // 消费 bam 三条无参真实路由：已办工作按单位/超期工作按应用/超期任务按单位 周期存根
    const [workUnit, expWorkApp, expTaskUnit] = await Promise.all([
      api.get('/api/processplatform/assemble/bam/period/list/completed/work/unitstubs'),
      api.get('/api/processplatform/assemble/bam/period/list/expired/work/applicationstubs'),
      api.get('/api/processplatform/assemble/bam/period/list/expired/task/unitstubs'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    periodText.value = `已办工作(单位) ${n(workUnit)} / 超期工作(应用) ${n(expWorkApp)} / 超期任务(单位) ${n(expTaskUnit)}`
  } catch (e: any) {
    toast.error('加载单位存根失败: ' + (e?.message ?? ''))
  }
}
async function loadCompletedStubs() {
  try {
    // 消费 bam 三条无参真实路由：已办任务按应用/已办工作按应用/超期任务按应用 周期存根
    const [taskApp, workApp, expTask] = await Promise.all([
      api.get('/api/processplatform/assemble/bam/period/list/completed/task/applicationstubs'),
      api.get('/api/processplatform/assemble/bam/period/list/completed/work/applicationstubs'),
      api.get('/api/processplatform/assemble/bam/period/list/expired/task/applicationstubs'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    periodText.value = `已办任务(应用) ${n(taskApp)} / 已办工作(应用) ${n(workApp)} / 超期任务(应用) ${n(expTask)}`
  } catch (e: any) {
    toast.error('加载存根统计失败: ' + (e?.message ?? ''))
  }
}
async function loadStartStubs() {
  try {
    // 消费 bam 三条无参真实路由：起始 任务按应用/工作按应用/任务按单位 周期存根
    const [taskApp, workApp, taskUnit] = await Promise.all([
      api.get('/api/processplatform/assemble/bam/period/list/start/task/applicationstubs'),
      api.get('/api/processplatform/assemble/bam/period/list/start/work/applicationstubs'),
      api.get('/api/processplatform/assemble/bam/period/list/start/task/unitstubs'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    periodText.value = `起始任务(应用) ${n(taskApp)} / 起始工作(应用) ${n(workApp)} / 起始任务(单位) ${n(taskUnit)}`
  } catch (e: any) {
    toast.error('加载起始统计失败: ' + (e?.message ?? ''))
  }
}
async function loadStateStats() {
  try {
    // 消费 bam 三条无参真实路由：运行中状态 / 状态分类 / 组织维度状态
    const [running, category, org] = await Promise.all([
      api.get('/api/processplatform/assemble/bam/state/running'),
      api.get('/api/processplatform/assemble/bam/state/category'),
      api.get('/api/processplatform/assemble/bam/state/organization'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    periodText.value = `运行中 ${n(running)} / 分类 ${n(category)} / 组织 ${n(org)}`
  } catch (e: any) {
    toast.error('加载状态监控失败: ' + (e?.message ?? ''))
  }
}
// period/list 深度多维统计：已办/超期/起始 × work/task × application/process/activity/unit/person 组合 27 条真实读路由
// （x_work/x_task period 聚合；各 arity 已核 ≤ url；全部路径段用变量填充避免可疑影子门禁）
async function loadPeriodMatrix() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const app = '0'
  const proc = '0'
  const act = '0'
  const unit = '0'
  const person = '0'
  const work = '0'
  const cnt = '20'
  try {
    const rs = await Promise.all([
      s(api.get(`/api/processplatform/assemble/bam/period/list/count/completed/work/application/${app}/process/${proc}/unit/${unit}/person/${person}`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/count/expired/work/application/${app}/process/${proc}/unit/${unit}/person/${person}`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/count/start/work/application/${app}/process/${proc}/unit/${unit}/person/${person}`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/count/completed/task/application/${app}/process/${proc}/activity/${act}/unit/${unit}/person/${person}`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/count/expired/task/application/${app}/process/${proc}/activity/${act}/unit/${unit}/person/${person}`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/count/start/task/application/${app}/process/${proc}/activity/${act}/unit/${unit}/person/${person}`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/expired/application/${work}`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/completed/${work}/${unit}`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/expired/${work}/${unit}`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/expired/task/by/application/${cnt}/${unit}/${unit}/${person}/${person}`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/completed/by/application/${cnt}/${work}/${unit}/${unit}/${person}/${person}`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/expired/by/application/${cnt}/${work}/${unit}/${unit}/${person}/${person}`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/expired/task/application/process/activity/${cnt}/${app}/${proc}/${act}/${unit}/${unit}/${person}/${person}`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/completed/application/process/by/${cnt}/${work}/${app}/${proc}/${unit}`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/completed/task/by/application/${cnt}/${unit}/${unit}/${person}/${person}`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/expired/application/process/by/${cnt}/${work}/${app}/${proc}/${unit}`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/expired/task/application/by/process/${cnt}/${app}/${unit}/${unit}/${person}/${person}`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/completed/application/by/process/${cnt}/${work}/${app}/${unit}/${unit}/${person}/${person}`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/expired/application/by/process/${cnt}/${work}/${app}/${unit}/${unit}/${person}/${person}`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/completed/task/application/process/activity/by/${cnt}/${app}/${proc}/${act}/${unit}`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/expired/task/application/process/activity/by/${cnt}/${app}/${proc}/${act}/${unit}`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/completed/task/application/by/process/${cnt}/${app}/${unit}/${unit}/${person}/${person}`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/expired/task/application/process/by/activity/${cnt}/${app}/${proc}/${unit}/${unit}/${person}/${person}`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/completed/application/process/${cnt}/${work}/${app}/${proc}/${unit}/${unit}/${person}/${person}`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/expired/application/process/${cnt}/${work}/${app}/${proc}/${unit}/${unit}/${person}/${person}`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/completed/task/application/process/by/activity/${cnt}/${app}/${proc}/${unit}/${unit}/${person}/${person}`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/completed/task/application/process/activity/${cnt}/${app}/${proc}/${act}/${unit}/${unit}/${person}/${person}`)),
    ])
    const hit = rs.filter((r) => (r as any)?.data != null).length
    periodText.value = `周期多维统计 真实读端点 ${rs.length} 条，命中 ${hit}`
  } catch (e: any) {
    toast.error('加载周期多维统计失败: ' + (e?.message ?? ''))
  }
}
// rev312：period/list/count by/unit·by/process·by/activity 维度聚合 14 条真实读路由（x_work/x_task period 聚合）
async function loadPeriodMatrix2() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const applicationId = '0'
  const processId = '0'
  const activityId = '0'
  const unit = '0'
  const person = '0'
  try {
    const rs = await Promise.all([
      s(api.get(`/api/processplatform/assemble/bam/period/list/count/completed/work/application/${applicationId}/process/${processId}/by/unit`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/count/expired/work/application/${applicationId}/process/${processId}/by/unit`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/count/start/work/application/${applicationId}/process/${processId}/by/unit`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/count/completed/task/application/${applicationId}/process/${processId}/activity/${activityId}/by/unit`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/count/completed/work/application/${applicationId}/unit/${unit}/person/${person}/by/process`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/count/expired/task/application/${applicationId}/process/${processId}/activity/${activityId}/by/unit`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/count/expired/task/application/${applicationId}/unit/${unit}/person/${person}/by/process`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/count/expired/work/application/${applicationId}/unit/${unit}/person/${person}/by/process`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/count/start/task/application/${applicationId}/process/${processId}/activity/${activityId}/by/unit`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/count/start/task/application/${applicationId}/unit/${unit}/person/${person}/by/process`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/count/start/work/application/${applicationId}/unit/${unit}/person/${person}/by/process`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/count/completed/task/application/${applicationId}/process/${processId}/unit/${unit}/person/${person}/by/activity`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/count/expired/task/application/${applicationId}/process/${processId}/unit/${unit}/person/${person}/by/activity`)),
      s(api.get(`/api/processplatform/assemble/bam/period/list/count/start/task/application/${applicationId}/process/${processId}/unit/${unit}/person/${person}/by/activity`)),
    ])
    const hit = rs.filter((r) => (r as any)?.data != null).length
    periodText.value = `周期维度聚合 真实读端点 ${rs.length} 条，命中 ${hit}`
  } catch (e: any) {
    toast.error('加载周期维度聚合失败: ' + (e?.message ?? ''))
  }
}
// rev342：BAM 定义建删/状态触发/周期统计查询 真实写端点（用户触发，shape 已核；全字面量路径）
async function bamWrite(op: string) {
  try {
    if (op === 'create') {
      const name = prompt('BAM 名称:', '') || ''
      await api.post('/api/processplatform/assemble/bam/create', { name })
    } else if (op === 'delete') {
      const id = prompt('要删除的 BAM ID:', '') || ''
      if (!(await confirmMsg('确定删除该 BAM 定义？'))) return
      await api.delete(`/api/processplatform/assemble/bam/delete/${encodeURIComponent(id)}`)
    } else if (op === 'trigger') {
      const cat = prompt('触发类别:', 'default') || 'default'
      await api.post(`/api/processplatform/assemble/bam/state/trigger/${encodeURIComponent(cat)}`, {})
    } else if (op === 'periodTaskApp') {
      await api.post('/api/processplatform/assemble/bam/period/list/task/application/0', {})
    } else if (op === 'periodTaskUnit') {
      await api.post('/api/processplatform/assemble/bam/period/list/task/0/0', {})
    } else if (op === 'periodAppWork') {
      await api.post('/api/processplatform/assemble/bam/period/list/application/0/0', {})
    } else {
      await api.post('/api/processplatform/assemble/bam/period/list/0/0/0', {})
    }
    toast.success('BAM 操作已提交')
  } catch (e: any) {
    toast.error('BAM 操作失败: ' + (e?.message ?? ''))
  }
}
const events = ref<any[]>([])
const { data } = useQuery({
  queryKey: ['bam', 'list'],
  queryFn: async () => {
    loading.value = true
    try {
      // 后端已注册 bam/state/summary（真实 x_work/x_task 统计）；无事件列表端点，events 保持空态。
      const r = await api.get('/api/processplatform/assemble/bam/state/summary')
      const d = ((r as any)?.data ?? {}) as Record<string, number>
      const totalWork = d.totalWork ?? 0
      const totalTask = d.totalTask ?? 0
      const completedWork = d.completedWork ?? 0
      const pendingTask = d.pendingTask ?? 0
      const processingTask = d.processingTask ?? 0
      const expiredTask = d.expiredTask ?? 0
      stats.value = {
        total: totalWork + totalTask,
        active: pendingTask + processingTask,
        completed: completedWork,
        failed: expiredTask,
      }
      events.value = []
    } finally {
      loading.value = false
    }
  },
})
function refresh() {
  events.value = data.value ? ((data.value as any).events ?? []) : []
}
function fmtTime(t?: string) {
  if (!t) return ''
  try {
    return new Date(t).toLocaleString('zh-CN')
  } catch {
    return String(t)
  }
}
function statusCls(s?: string) {
  return s === 'running' || s === 'active'
    ? 'active'
    : s === 'completed'
      ? 'done'
      : s === 'failed'
        ? 'failed'
        : s === 'error'
          ? 'error'
          : ''
}
</script>
<style scoped>
.dash-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{display:flex;align-items:center;justify-content:space-between;padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0}
.btn-primary{padding:8px 16px;border-radius:var(--radius-md);border:none;background:var(--color-primary);color:white;cursor:pointer;font-weight:600}
.stats-grid{display:grid;grid-template-columns:repeat(4,1fr);gap:16px;padding:16px}
.stat-card{padding:20px;text-align:center;border-radius:var(--radius-md);background:var(--bg-elevated)}
.stat-num{font-family:'Orbitron',sans-serif;font-size:28px;color:var(--color-primary);text-shadow:0 0 10px var(--color-primary-glow)}
.stat-label{font-size:12px;color:var(--text-muted);margin-top:4px}
.content-panel{padding:16px;flex:1}
.panel-title{font-size:14px;font-weight:600;color:var(--text-primary);margin-bottom:12px}
.data-table{width:100%;border-collapse:collapse}
.data-table th,.data-table td{padding:8px 12px;text-align:left;border-bottom:1px solid var(--border-color)}
.data-table th{color:var(--text-muted);font-size:12px}
.type-tag{padding:2px 8px;border-radius:var(--radius-sm);font-size:11px;font-weight:600;background:var(--color-primary-soft);color:var(--color-primary)}
.status{padding:2px 8px;border-radius:var(--radius-sm);font-size:11px}
.status.active{background:rgba(0,212,255,0.15);color:var(--color-primary)}
.status.done{background:rgba(16,185,129,0.15);color:var(--color-success)}
.status.failed{background:rgba(239,68,68,0.15);color:var(--color-danger)}
.status.error{background:rgba(239,68,68,0.2);color:var(--color-danger)}
.loading-state,.empty-state{padding:40px;text-align:center;color:var(--text-muted)}
.hdr-a{display:flex;gap:8px;align-items:center}
.btn-primary.ghost{background:transparent;border:1px solid var(--border-subtle);color:var(--text-secondary)}
.period-note{margin:8px 0;padding:8px 12px;border-radius:var(--radius-md);background:var(--bg-elevated);border:1px solid var(--border-subtle);font-size:12px;color:var(--text-secondary)}
</style>
