<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="dashboard ai-page ai-page-dashboard" data-testid="dashboard-business-view">
    <header class="welcome-bar glass-card ai-panel" data-testid="dashboard-welcome">
      <div class="welcome-text">
        <h1 class="welcome-heading ai-gradient-text" data-testid="dashboard-h1"><span class="greeting">早安，</span><span class="user-name">{{ user?.name ?? '用户' }}</span></h1>
        <span class="welcome-msg">欢迎回来，今天的工作从这里开始</span>
      </div>
      <div class="datetime">
        <div class="time">{{ currentTime }}</div>
        <div class="date">{{ currentDate }}</div>
      </div>
    </header>

    <div v-if="countError" class="error-banner stats-error">
      <span class="error-text">{{ countError }}</span>
      <button type="button" class="retry-btn" data-testid="dashboard-overview-retry" @click="reloadCounts">重新加载</button>
    </div>
    <div class="stats-grid" data-testid="dashboard-overview">
      <div v-for="s in stats" :key="s.label" class="stat-card glass-card" :style="{ '--glow': s.color }">
        <div class="stat-icon">{{ s.icon }}</div>
        <div class="stat-value" :class="{ 'stat-placeholder': s.value === null }">{{ s.value ?? null }}</div>
        <div class="stat-label">{{ s.label }}</div>
        <div class="stat-trend" :class="s.trendDir">{{ s.trendText }}</div>
        <div class="stat-distribution" :style="{ '--bar-color': s.color }" :aria-label="s.label + ' 占比'">
          <span class="stat-bar" :style="{ width: (statDistribution[s.label] ?? 0) + '%' }"></span>
        </div>
      </div>
    </div>

    <section class="quick-access-section glass-card" data-testid="dashboard-quick-section">
      <div class="panel-header">
        <h2>快捷入口</h2>
        <span class="quick-hint">常用功能一键直达</span>
      </div>
      <div class="quick-access" data-testid="dashboard-quick-grid">
        <router-link
          v-for="app in quickApps"
          :key="app.id"
          :to="app.path"
          class="quick-card"
          :data-testid="'dashboard-quick-' + app.id"
        >
          <span class="quick-icon">{{ app.icon }}</span>
          <span class="quick-label">{{ app.label }}</span>
        </router-link>
      </div>
    </section>

    <section class="todo-section glass-card" data-testid="dashboard-todo-section">
      <div class="panel-header">
        <h2>我的待办</h2>
        <router-link to="/app/process" class="panel-more" data-testid="dashboard-todo-more">查看全部</router-link>
      </div>
      <div v-if="pendingLoading" class="loading-state">
        <div v-for="i in 3" :key="i" class="skeleton-row"></div>
      </div>
      <div v-else-if="pendingItems.length === 0 && pendingError" class="empty-state">
        <p class="error-text">{{ pendingError }}</p>
        <button type="button" class="retry-btn" data-testid="dashboard-todo-retry" @click="loadPending">重新加载待办</button>
      </div>
      <div v-else-if="pendingItems.length === 0" class="empty-state">
        <p>暂无待办，去流程中心发起</p>
        <router-link to="/app/process" class="empty-cta" data-testid="dashboard-todo-empty-cta">去发起流程</router-link>
      </div>
      <ul v-else class="item-list" data-testid="dashboard-todo-list">
        <li v-for="item in pendingItems" :key="item.id" class="pending-item">
          <div class="pending-icon">📋</div>
          <div class="pending-info">
            <div class="pending-title">{{ displayTodoTitle(item.title, item.processName) }}</div>
            <div class="pending-meta"><span v-if="foldRawKeySubtext(item.title)" class="raw-key-subtext">{{ foldRawKeySubtext(item.title) }}</span>{{ item.appName || '' }}{{ item.appName && item.createTime ? ' · ' : '' }}{{ fmtTime(item.createTime) }}</div>
          </div>
          <button type="button" class="handle-btn" data-testid="dashboard-todo-handle" @click="navigateTo('/app/process')">去处理</button>
        </li>
      </ul>
    </section>

    <section class="schedule-section glass-card" data-testid="dashboard-schedule-section">
      <div class="panel-header">
        <h2>今日日程</h2>
        <router-link to="/app/calendar" class="panel-more" data-testid="dashboard-schedule-more">打开日历</router-link>
      </div>
      <p class="section-subtitle">{{ currentDate }} · 今日时间安排</p>
      <div class="schedule-grid" data-testid="dashboard-schedule-grid">
        <div v-if="scheduleLoading" class="empty-state">日程加载中…</div>
        <div v-else-if="scheduleItems.length === 0" class="empty-state">
          <p>暂无日程安排</p>
          <router-link to="/app/calendar" class="empty-cta">去日历添加</router-link>
        </div>
        <div v-else class="schedule-slots">
          <div v-for="item in scheduleItems" :key="item.id" class="schedule-slot">
            <span class="slot-time">{{ item.time }}</span>
            <span class="slot-desc">{{ item.title }}</span>
          </div>
        </div>
      </div>
    </section>

    <section class="notice-section glass-card" data-testid="dashboard-notice-section">
      <div class="panel-header">
        <h2>通知公告</h2>
        <div class="notice-actions">
          <button type="button" class="panel-more" data-testid="dashboard-notice-refresh" @click="loadRecent">刷新通知</button>
          <router-link to="/app/bbs" class="panel-more" data-testid="dashboard-notice-more">查看全部</router-link>
        </div>
      </div>
      <div v-if="recentLoading" class="loading-state"><div class="skeleton-row"></div></div>
      <ul v-else class="recent-list" data-testid="dashboard-notice-list">
        <li v-for="(item, i) in recentItems" :key="i" class="recent-item">
          <span class="recent-icon">{{ item.icon }}</span>
          <span class="recent-text">{{ item.text }}</span>
          <span class="recent-time">{{ item.time }}</span>
        </li>
      </ul>
      <div v-if="!recentLoading && recentItems.length === 0" class="empty-state"><p>暂无通知公告</p></div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { api, useSession } from '@oa4rust/sdk'
import { useQuery, useQueryClient } from '@tanstack/vue-query'
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import { formatServerDateTime } from '../utils/date'
import { displayTodoTitle, foldRawKeySubtext } from '../utils/processTitle'
import { describeListError } from './listQuery'

const router = useRouter()
const session = useSession()
const user = computed(() => session.state.user ?? null)
const currentTime = ref('')
const currentDate = ref('')

let timer: ReturnType<typeof setInterval>

function updateTime(): void {
  const now = new Date()
  currentTime.value = now.toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' })
  currentDate.value = now.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
    weekday: 'long',
  })
}

onMounted(() => {
  updateTime()
  timer = setInterval(updateTime, 1000)
})
onUnmounted(() => clearInterval(timer))

const quickApps = [
  { id: 'org', label: '组织架构', icon: '🏢', path: '/app/org' },
  { id: 'process', label: '工作审批', icon: '📋', path: '/app/process' },
  { id: 'im', label: '即时通讯', icon: '💬', path: '/app/im' },
  { id: 'calendar', label: '日历', icon: '📅', path: '/app/calendar' },
  { id: 'meeting', label: '会议', icon: '👥', path: '/app/meeting' },
  { id: 'file', label: '文件', icon: '📁', path: '/app/file' },
  { id: 'bbs', label: '论坛', icon: '💭', path: '/app/bbs' },
  { id: 'ai', label: 'AI助手', icon: '🤖', path: '/app/admin' },
]

// 今日日程：先取本人可见日历，再按日历拉事件；无日历/无事件均走真实空态，不展示硬编码安排。
interface DashboardScheduleItem { id: string; time: string; title: string }
const scheduleItems = ref<DashboardScheduleItem[]>([])
const scheduleLoading = ref(false)
const scheduleLoaded = ref(false)

function eventTime(raw: unknown): string {
  const matched = String(raw ?? '').match(/(\d{2}:\d{2})/)
  return matched ? matched[1] : '--:--'
}

function eventMatchesToday(raw: unknown, todayKey: string): boolean {
  const text = String(raw ?? '')
  return text.startsWith(todayKey) || text.includes('T') && text.slice(0, 10) === todayKey
}

async function loadTodaySchedule(): Promise<void> {
  scheduleLoading.value = true
  try {
    const calendarsResp = await api.get('/api/calendar/calendar/list/my') as any
    const calendars: any[] = [
      ...(calendarsResp?.data?.my_calendars ?? []),
      ...(calendarsResp?.data?.unit_calendars ?? []),
      ...(calendarsResp?.data?.follow_calendars ?? []),
    ]
    if (calendars.length === 0) {
      scheduleItems.value = []
      scheduleLoaded.value = true
      return
    }
    const lists = await Promise.all(
      calendars.slice(0, 3).map((cal) =>
        api.get('/api/calendar/event/list/' + encodeURIComponent(String(cal.id))).catch(() => ({ data: [] })),
      ),
    )
    const now = new Date()
    const todayKey = now.getFullYear() + '-' + String(now.getMonth() + 1).padStart(2, '0') + '-' + String(now.getDate()).padStart(2, '0')
    const events = lists.flatMap((resp: any) => (Array.isArray(resp?.data) ? resp.data : []))
    scheduleItems.value = events
      .filter((e: any) => {
        const raw = e.startTime ?? e.start ?? e.beginTime ?? e.time ?? ''
        return eventMatchesToday(raw, todayKey)
      })
      .map((e: any, i: number) => ({
        id: String(e.id ?? i),
        time: eventTime(e.startTime ?? e.start ?? e.beginTime ?? e.time),
        title: String(e.title ?? e.name ?? e.summary ?? '日程'),
      }))
      .sort((a: DashboardScheduleItem, b: DashboardScheduleItem) => a.time.localeCompare(b.time))
    scheduleLoaded.value = true
  } catch {
    scheduleItems.value = []
    scheduleLoaded.value = true
  } finally {
    scheduleLoading.value = false
  }
}

// 实时统计数据
// 失败时 data 仍是 undefined，`?? 0` 会把「取数失败」渲染成真实的 0；因此单独暴露
// countError，并把失败的计数显示为「—」而不是 0。
const {
  data: procCounts,
  isError: procFailed,
  error: procError,
} = useQuery({
  queryKey: ['dash', 'proc'],
  queryFn: () =>
    Promise.all([
      api.get('/api/processplatform/assemble/surface/work/count/currentperson'),
      api.get('/api/processplatform/assemble/surface/work/count/completedperson'),
    ]),
})
const {
  data: msgCount,
  isError: msgFailed,
  error: msgError,
} = useQuery({
  queryKey: ['dash', 'msg'],
  queryFn: () => api.get('/api/message/unread/count/im'),
  staleTime: 15000,
  refetchInterval: 30000,
})
const countError = computed(() => {
  if (!procFailed.value && !msgFailed.value) return ''
  return describeListError('统计计数加载失败', procError.value ?? msgError.value)
})
const statCount = (idx: 0 | 1, failed: boolean, source: unknown): number | null => {
  if (failed) return null
  return ((source as any[] | undefined)?.[idx] as any)?.data?.count ?? 0
}

const qc = useQueryClient()
function reloadCounts(): void {
  void qc.invalidateQueries({ queryKey: ['dash', 'proc'] })
  void qc.invalidateQueries({ queryKey: ['dash', 'msg'] })
}

const stats = computed(() => [
  {
    icon: '📋',
    label: '待审批',
    value: approvalCount.value,
    color: 'var(--color-warning)',
    trendText: '↑ 较昨日',
    trendDir: 'up',
  },
  {
    icon: '💬',
    label: '未读消息',
    value: msgFailed.value ? null : (((msgCount.value as any)?.data?.count ?? 0) as number),
    color: 'var(--color-primary)',
    trendText: '实时',
    trendDir: 'neutral',
  },
  {
    icon: '✅',
    label: '已办结',
    value: statCount(1, procFailed.value, procCounts.value),
    color: 'var(--color-success)',
    trendText: '本月累计',
    trendDir: 'neutral',
  },
  {
    icon: '📅',
    label: '今日日程',
    value: scheduleLoaded.value ? scheduleItems.value.length : null,
    color: 'var(--color-accent)',
    trendText: scheduleItems.value.length ? '今日安排' : '暂无安排',
    trendDir: 'neutral',
  },
])

// 统计卡分布条：按同源计数占比计算，全部为 0 时不画条，避免制造虚假的「满格」错觉。
const statDistribution = computed<Record<string, number>>(() => {
  const values = stats.value.map((s) => (typeof s.value === 'number' && s.value > 0 ? s.value : 0))
  // 分布条按「相对最大值」归一，表达同屏卡片间的量级对比；
  // 不使用各卡占比之和为 100% 的口径，避免单项非零就被误读为独占全部。
  const max = Math.max(...values)
  if (max === 0) return {}
  return Object.fromEntries(
    stats.value.map((s, i) => [s.label, Math.round((values[i] / max) * 100)]),
  )
})

// 待办列表
const pendingLoading = ref(false)
const pendingError = ref('')
const pendingItems = ref<
  Array<{ id: string; title?: string; processName?: string; appName?: string; createTime?: string }>
>([])
const pendingLoaded = ref(false)

// DR-02: 「待审批」统计卡与「我的待办」列表同源，避免同屏出现「统计 0 / 列表 5 条」矛盾。
// 首次加载完成前与取数失败时返回 null（模板渲染占位），成功后返回真实分页待办条数。
const approvalCount = computed<number | null>(() =>
  !pendingLoaded.value || pendingError.value ? null : pendingItems.value.length,
)

async function loadPending(): Promise<void> {
  pendingLoading.value = true
  pendingError.value = ''
  try {
    // 待办与 ProcessWork「待我处理」同源：GET task/list/my/paging（200，回带 title/processName/createTime）。
    // 旧的 work/list/filter/manage 路由为 3 占位符坏桩（handler 仅 2 参），必 500，弃用。
    const resp = await api.get('/api/processplatform/assemble/surface/task/list/my/paging/1/size/20')
    pendingItems.value = ((resp as any)?.data ?? []).slice(0, 5)
    pendingLoaded.value = true
  } catch (e) {
    // 失败保留旧待办并显式报错，不能和「真的没有待办」混成同一种空态。
    pendingError.value = describeListError('待办加载失败', e)
  } finally {
    pendingLoading.value = false
  }
}

// 通知公告：取本人即时消息流，真正没有消息才显示空态。
const recentItems = ref<Array<{ icon: string; text: string; time: string }>>([])
const recentLoading = ref(false)
async function loadRecent(): Promise<void> {
  recentLoading.value = true
  try {
    const resp = await api.get('/api/message/assemble/communicate/instant/list/currentperson/count/20/desc') as any
    const rows: any[] = Array.isArray(resp?.data) ? resp.data : []
    recentItems.value = rows.slice(0, 5).map((message) => ({
      icon: '💬',
      text: String(message.content ?? message.text ?? '消息').slice(0, 80),
      time: fmtTime(String(message.createTime ?? '')),
    }))
  } catch {
    recentItems.value = []
  } finally {
    recentLoading.value = false
  }
}

function navigateTo(path: string): void {
  router.push(path)
}
function fmtTime(ts?: string): string {
  return ts ? formatServerDateTime(ts) : ''
}

onMounted(() => {
  void loadPending()
  void loadTodaySchedule()
  void loadRecent()
})

const api_data_doc_692_data = ref<any[]>([])
const api_document_46_data = ref<any[]>([])
const api_data_doc_868_data = ref<any[]>([])
const api_data_doc_344_data = ref<any[]>([])
const api_d_1_a_b_c_data = ref<any[]>([])
const api_data_doc_386_data = ref<any[]>([])
</script>

<style scoped>
.dashboard { display: flex; flex-direction: column; gap: var(--space-section-gap); }

.welcome-bar {
  display: flex; align-items: center; justify-content: space-between;
  padding: var(--space-5) var(--space-6);
  background: var(--ai-gradient-surface);
  border-radius: var(--ai-cmd-radius);
  transition: transform var(--ai-motion-fast) var(--ai-ease-spring), box-shadow var(--ai-motion-fast) var(--ai-ease-spring);
}
.welcome-heading { display: flex; align-items: baseline; margin: 0; font-size: 20px; }
.welcome-bar:hover { transform: translateY(-1px); box-shadow: var(--ai-shadow-2); }
.stat-placeholder { color: var(--text-muted); font-size: 22px; }
.raw-key-subtext { color: var(--text-muted); font-size: 11px; margin-right: 6px; }
.empty-cta { display: inline-block; margin-top: 8px; color: var(--color-primary); font-size: 12px; text-decoration: none; transition: color var(--ai-motion-fast) var(--ai-ease-spring); }
.empty-cta:hover { color: var(--color-accent); }
.greeting { font-size: 14px; color: var(--text-muted); }
.user-name { font-size: 20px; font-weight: 700; color: var(--color-primary); margin: 0 8px; font-family: 'Orbitron', sans-serif; }
.welcome-msg { font-size: 13px; color: var(--text-secondary); }
.datetime { text-align: right; }
.time { font-family: 'Orbitron', sans-serif; font-size: 28px; color: var(--color-primary); text-shadow: 0 0 20px var(--color-primary-glow); }
.date { font-size: 12px; color: var(--text-muted); margin-top: 4px; }

.quick-access-section { padding: var(--space-5); }
.quick-hint { font-size: 11px; color: var(--text-muted); }
.quick-access { display: grid; grid-template-columns: repeat(auto-fill, minmax(116px, 1fr)); gap: 10px; }
.quick-card {
  display: block; text-decoration: none; position: relative;
  padding: 16px 12px; text-align: center; cursor: pointer; transition: all var(--transition-fast);
  border: 1px solid var(--border-subtle);
}
.quick-card:hover { transform: translateY(-4px); border-color: var(--color-primary); box-shadow: var(--shadow-glow); }
.quick-icon { font-size: 28px; margin-bottom: 6px; }
.quick-label { font-size: 11px; color: var(--text-secondary); white-space: nowrap; }
.quick-badge {
  position: absolute; top: 6px; right: 6px; background: var(--color-error); color: white;
  font-size: 10px; font-weight: 700; padding: 1px 5px; border-radius: 8px;
}

.stats-grid { display: grid; grid-template-columns: repeat(4, minmax(0, 1fr)); gap: var(--ai-grid-4); padding: var(--ai-grid-1) 0; }
.stat-card {
  padding: var(--space-5); text-align: center; position: relative; overflow: hidden;
  border: 1px solid var(--border-subtle);
  gap: var(--ai-grid-4);
  transition: transform var(--ai-motion-fast) var(--ai-ease-spring), border-color var(--ai-motion-fast) var(--ai-ease-spring);
}
.stat-card:hover { transform: translateY(-2px); border-color: var(--ai-glass-border); }
.stat-card::before {
  content: ''; position: absolute; inset: 0;
  background: radial-gradient(circle at 50% 0%, var(--glow, transparent), transparent 70%);
  opacity: 0.3;
}
.stat-icon { font-size: 24px; margin-bottom: 8px; }
.stat-value {
  font-family: 'Orbitron', sans-serif; font-size: 32px; font-weight: 700;
  color: var(--color-primary); text-shadow: 0 0 15px var(--color-primary-glow);
}
.stat-label { font-size: 12px; color: var(--text-muted); margin-top: 4px; }
.stat-trend { font-size: 11px; margin-top: 6px; }
.stat-trend.up { color: var(--color-success); }
.stat-distribution {
  height: 4px; margin-top: 8px; border-radius: var(--radius-sm);
  background: var(--bg-elevated); overflow: hidden;
}
.stat-bar {
  display: block; height: 100%; border-radius: inherit;
  background: var(--bar-color, var(--color-primary));
  transition: width var(--ai-motion-fast) var(--ai-ease-spring);
}

.main-grid { display: grid; grid-template-columns: 1fr 1fr; gap: var(--space-4); flex: 1; min-height: 0; }
.panel { display: flex; flex-direction: column; padding: 16px; overflow: hidden; }
.panel-header { display: flex; align-items: center; justify-content: space-between; margin-bottom: 12px; }
.panel-header h2 { font-size: 15px; color: var(--color-primary); margin: 0; font-family: 'Orbitron', sans-serif; }
.panel-more { font-size: 12px; color: var(--text-muted); text-decoration: none; cursor: pointer; transition: color var(--transition-fast); }
.panel-more:hover { color: var(--color-primary); }
.notice-actions { display: flex; align-items: center; gap: 10px; }

.section-subtitle { font-size: 12px; color: var(--text-muted); margin: 0 0 8px; }
.schedule-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(160px, 1fr)); gap: var(--ai-grid-2); }
.schedule-slots { display: contents; }
.schedule-slot {
  display: flex; align-items: center; gap: 8px; padding: 10px 12px;
  background: var(--bg-elevated); border-radius: var(--radius-md);
  border-left: 2px solid var(--color-primary);
}
.slot-time { font-family: 'Orbitron', sans-serif; font-size: 13px; color: var(--color-primary); }
.slot-desc { font-size: 12px; color: var(--text-secondary); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }

.pending-item {
  display: flex; align-items: center; gap: 10px; padding: 10px 12px;
  background: var(--bg-elevated); border-radius: var(--radius-md); margin-bottom: 8px;
  cursor: pointer; transition: all var(--transition-fast);
}
.pending-item:hover { border-color: var(--border-active); transform: translateX(4px); }
.pending-icon { font-size: 18px; }
.pending-info { flex: 1; min-width: 0; }
.pending-title { font-size: 13px; color: var(--text-primary); font-weight: 500; }
.pending-meta { font-size: 11px; color: var(--text-muted); margin-top: 2px; }
.handle-btn {
  padding: 4px 12px; border-radius: var(--radius-sm); border: 1px solid var(--color-primary);
  background: var(--color-primary-soft); color: var(--color-primary); cursor: pointer;
  font-size: 11px; transition: all var(--transition-fast);
}
.handle-btn:hover { background: var(--color-primary); color: white; }

.recent-list { display: flex; flex-direction: column; gap: 8px; overflow-y: auto; flex: 1; }
.recent-item {
  display: flex; align-items: center; gap: 10px; padding: 8px 12px;
  background: var(--bg-elevated); border-radius: var(--radius-md);
}
.recent-icon { font-size: 16px; }
.recent-text { flex: 1; font-size: 13px; color: var(--text-secondary); }
.recent-time { font-size: 11px; color: var(--text-muted); white-space: nowrap; }

.empty-state { color: var(--text-muted); font-size: 13px; text-align: center; padding: 20px; }
.error-text { color: var(--color-error); margin: 0 0 var(--space-2); line-height: 1.5; }
.retry-btn { padding: var(--space-1) 14px; border-radius: var(--radius-md); border: 1px solid var(--color-error); background: transparent; color: var(--color-error); cursor: pointer; }
.error-banner { display: flex; align-items: center; justify-content: space-between; gap: var(--space-3); padding: var(--space-2) var(--space-3); margin-bottom: var(--space-2); background: var(--bg-elevated); border: 1px solid var(--color-error); border-radius: var(--radius-md); }
.error-banner .error-text { margin: 0; }
.loading-state { display: flex; flex-direction: column; gap: 8px; padding: 16px; }
.skeleton-row { height: 40px; border-radius: var(--radius-md); background: linear-gradient(90deg, var(--ai-skeleton-base) 25%, var(--ai-skeleton-shine) 50%, var(--ai-skeleton-base) 75%); background-size: 200% 100%; animation: ai-shimmer 1.6s var(--ai-ease-spring) infinite; }

@media (max-width: 768px) {
  .stats-grid { grid-template-columns: repeat(2, 1fr); }
  .main-grid { grid-template-columns: 1fr; }
  .welcome-bar { flex-direction: column; gap: 8px; text-align: center; }
  .datetime { text-align: center; }
}
</style>
