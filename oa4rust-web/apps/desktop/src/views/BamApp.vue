<template>
  <div class="dash-view">
    <div class="view-header glass-card">
      <div><h1>业务活动监控</h1><p class="subtitle">/api/processplatform/assemble/bam/*</p></div>
      <span class="hdr-a">
        <button class="btn-primary ghost" @click="loadPeriodStats">周期统计</button>
        <button class="btn-primary ghost" @click="loadStateStats">状态监控</button>
        <button class="btn-primary ghost" @click="loadStartStubs">起始统计</button>
        <button class="btn-primary ghost" @click="loadCompletedStubs">已办/超期存根</button>
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
import { toast } from '../utils/toast'

const loading = ref(false)
const stats = ref({ total: 0, active: 0, completed: 0, failed: 0 })
const periodText = ref('')
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
