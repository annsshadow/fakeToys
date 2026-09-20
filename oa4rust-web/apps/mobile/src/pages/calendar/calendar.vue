<script setup lang="ts">
/**
 * 日历（阶段 G / F3）——我的日历 + 今日/本周日程。
 *
 * 契约注意（实测后端 u2::event_list_filter）：`PUT event/list/filter` 只读
 * body 的 `calendarId` / `title`，**不支持时间范围**，且返回行是 snake_case
 * （`calendar_id` / `start_time` / `end_time` / `all_day` / `createor`）。
 * 因此时间范围在客户端按 start_time 过滤。
 */
import { onShow } from '@dcloudio/uni-app'
import { computed, ref } from 'vue'
import { type CalendarRow, calendarApi } from '@/services'
import { ensureAuthenticated } from '@/utils/auth-guard'

interface EventRow {
  id?: string
  title?: string
  content?: string
  location?: string
  calendar_id?: string
  calendarId?: string
  start_time?: string
  startTime?: string
  end_time?: string
  endTime?: string
  all_day?: boolean
  [key: string]: unknown
}

type RangeKey = 'today' | 'week'
const ranges: Array<{ key: RangeKey; label: string }> = [
  { key: 'today', label: '今日' },
  { key: 'week', label: '本周' },
]

const range = ref<RangeKey>('today')
const calendars = ref<CalendarRow[]>([])
const events = ref<EventRow[]>([])
const loading = ref(false)

function startOfDay(d: Date): Date {
  const x = new Date(d)
  x.setHours(0, 0, 0, 0)
  return x
}
function rangeBounds(): { from: number; to: number } {
  const now = new Date()
  const from = startOfDay(now)
  if (range.value === 'today') {
    const to = new Date(from)
    to.setDate(to.getDate() + 1)
    return { from: from.valueOf(), to: to.valueOf() }
  }
  // 本周：以周一为首日
  const monday = new Date(from)
  const dow = (monday.getDay() + 6) % 7
  monday.setDate(monday.getDate() - dow)
  const nextMonday = new Date(monday)
  nextMonday.setDate(nextMonday.getDate() + 7)
  return { from: monday.valueOf(), to: nextMonday.valueOf() }
}
function startMs(e: EventRow): number {
  const raw = e.start_time ?? e.startTime
  if (!raw) return Number.NaN
  return new Date(String(raw)).valueOf()
}

async function load(): Promise<void> {
  loading.value = true
  try {
    const cals = await calendarApi.myCalendars()
    calendars.value = (cals.data ?? []) as CalendarRow[]
    const all: EventRow[] = []
    // 逐个日历拉事件（后端 filter 只按 calendarId 过滤），再在客户端做时间窗筛选
    for (const c of calendars.value) {
      try {
        const resp = await calendarApi.eventsFilter({ calendarId: c.id })
        all.push(...((resp.data ?? []) as EventRow[]))
      } catch {
        /* 单个日历失败不影响其余 */
      }
    }
    const { from, to } = rangeBounds()
    events.value = all
      .filter((e) => {
        const ms = startMs(e)
        return Number.isFinite(ms) && ms >= from && ms < to
      })
      .sort((a, b) => startMs(a) - startMs(b))
  } catch {
    calendars.value = []
    events.value = []
  } finally {
    loading.value = false
  }
}

onShow(async () => {
  if (!(await ensureAuthenticated())) return
  void load()
})

function switchRange(key: RangeKey): void {
  if (range.value === key) return
  range.value = key
  void load()
}

// 事件详情（event/{id}）：移动端在列表内展开，避免多跳一层页面
const detail = ref<EventRow | null>(null)
const detailLoading = ref(false)

async function openEvent(e: EventRow): Promise<void> {
  const id = String(e.id ?? '')
  if (!id) return
  detailLoading.value = true
  detail.value = null
  try {
    const resp = await calendarApi.eventDetail(id)
    detail.value = ((resp as { data?: EventRow }).data ?? e) as EventRow
  } catch {
    detail.value = e
  } finally {
    detailLoading.value = false
  }
}

function fmtTime(e: EventRow): string {
  const s = e.start_time ?? e.startTime
  const t = e.end_time ?? e.endTime
  const f = (v: unknown): string => {
    if (!v) return ''
    const d = new Date(String(v))
    if (Number.isNaN(d.valueOf())) return String(v)
    return `${String(d.getHours()).padStart(2, '0')}:${String(d.getMinutes()).padStart(2, '0')}`
  }
  const a = f(s)
  const b = f(t)
  if (!a && !b) return '全天'
  return b ? `${a} - ${b}` : a
}

const emptyText = computed(() => (range.value === 'today' ? '今日暂无日程' : '本周暂无日程'))
</script>

<template>
  <view class="page">
    <view class="tabs">
      <view v-for="r in ranges" :key="r.key" class="tab" :class="{ on: range === r.key }" @tap="switchRange(r.key)">
        {{ r.label }}
      </view>
    </view>

    <view class="sub">我的日历 {{ calendars.length }} 个</view>

    <view v-if="loading && events.length === 0" class="tip">加载中…</view>
    <view v-else-if="events.length === 0" class="tip">{{ emptyText }}</view>

    <view v-for="e in events" :key="e.id" class="card" @tap="openEvent(e)">
      <view class="row">
        <text class="time">{{ fmtTime(e) }}</text>
        <text class="title">{{ e.title || '未命名日程' }}</text>
      </view>
      <view v-if="e.location" class="meta">📍 {{ e.location }}</view>
      <view v-if="e.content" class="content">{{ e.content }}</view>
    </view>

    <view v-if="detailLoading" class="tip">详情加载中…</view>
    <view v-else-if="detail" class="card detail">
      <view class="detail-head">
        <text class="title">{{ detail.title || '未命名日程' }}</text>
        <text class="detail-close" @tap="detail = null">关闭</text>
      </view>
      <view class="meta">{{ fmtTime(detail) }}</view>
      <view v-if="detail.location" class="meta">📍 {{ detail.location }}</view>
      <view v-if="detail.content" class="content">{{ detail.content }}</view>
    </view>
  </view>
</template>

<style scoped>
.page {
  min-height: 100vh;
  background: #f5f7fa;
  padding: 24rpx;
  box-sizing: border-box;
}
.tabs {
  display: flex;
  gap: 12rpx;
  margin-bottom: 16rpx;
}
.tab {
  flex: 1;
  text-align: center;
  padding: 16rpx 0;
  font-size: 28rpx;
  color: #5b6572;
  background: #fff;
  border-radius: 12rpx;
}
.tab.on {
  background: #2d8cf0;
  color: #fff;
  font-weight: 600;
}
.sub {
  font-size: 24rpx;
  color: #90979f;
  margin-bottom: 20rpx;
}
.card {
  background: #fff;
  border-radius: 16rpx;
  padding: 24rpx;
  margin-bottom: 20rpx;
}
.row {
  display: flex;
  align-items: baseline;
  gap: 16rpx;
}
.time {
  font-size: 26rpx;
  color: #2d8cf0;
  font-weight: 600;
  min-width: 150rpx;
}
.title {
  font-size: 30rpx;
  font-weight: 700;
  color: #263238;
}
.meta {
  margin-top: 10rpx;
  font-size: 24rpx;
  color: #90979f;
}
.content {
  margin-top: 12rpx;
  font-size: 26rpx;
  color: #5b6572;
  line-height: 1.5;
}
.tip {
  color: #90979f;
  font-size: 26rpx;
  text-align: center;
  padding: 80rpx 0;
}
.detail-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12rpx;
}
.detail-close {
  font-size: 26rpx;
  color: #2d8cf0;
}
</style>
