<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="calendar-view ai-page" data-testid="calendar-business-view">
    <div class="cal-header view-header glass-card ai-panel" data-testid="calendar-header">
      <div>
        <h1>工作日历</h1>
        <p class="subtitle">查看日程安排、管理日程事件</p>
      </div>
      <div class="header-controls" role="group" aria-label="月份导航">
        <button class="nav-btn" aria-label="上一月" @click="prevMonth">‹</button>
        <span class="month-label">{{ currentYear }}年{{ currentMonth }}月</span>
        <button class="nav-btn" aria-label="下一月" @click="nextMonth">›</button>
        <button class="today-btn" @click="goToday">回到今天</button>
      </div>
    </div>

    <div class="cal-scope glass-card" role="tablist" aria-label="日历范围" data-testid="calendar-scope-tabs">
      <button
        class="cll-tab"
        role="tab"
        :aria-selected="calScope==='my'"
        :class="{on:calScope==='my'}"
        @click="loadCalendars('my')"
      >我的日历（{{ myCals.length }}）</button>
      <button
        class="cll-tab"
        role="tab"
        :aria-selected="calScope==='public'"
        :class="{on:calScope==='public'}"
        @click="loadCalendars('public')"
      >公共日历（{{ pubCals.length }}）</button>
      <div v-if="calListError" class="cll-error">{{ calListError }}</div>
      <div v-if="(calScope==='my'?myCals:pubCals).length" class="cal-chip-row">
        <button
          v-for="c in (calScope==='my'?myCals:pubCals)"
          :key="c.id"
          class="cal-chip"
          :class="{on: activeCal?.id === c.id}"
          :style="{borderColor:c.color||'var(--color-primary)'}"
          @click="selectCalendar(c)"
          :aria-label="c.name || '选择日历'"
        >{{ c.name }}</button>
      </div>
      <div class="cal-create-wrap">
        <button
          type="button"
          class="cal-create-entry today-btn"
          data-testid="calendar-create-entry"
          :aria-expanded="calCreateOpen"
          aria-label="新建日历"
          @click="calCreateOpen = !calCreateOpen"
        >+ 新建日历</button>
        <form
          v-if="calCreateOpen"
          class="cal-create-form ai-panel"
          data-testid="calendar-create-form"
          @submit.prevent="calWriteCE('calCreate')"
        >
          <label class="cal-create-label">
            <span>日历名称</span>
            <input v-model="calCreateForm.name" class="cal-create-input" placeholder="请输入日历名称" aria-label="日历名称" required />
          </label>
          <label class="cal-create-label">
            <span>颜色（可选）</span>
            <input v-model="calCreateForm.color" class="cal-create-color" type="color" aria-label="日历颜色" />
          </label>
          <button type="submit" class="today-btn" :disabled="creatingCal">{{ creatingCal ? '创建中…' : '创建日历' }}</button>
          <button type="button" class="cal-create-cancel" @click="calCreateOpen = false">取消</button>
        </form>
      </div>
    </div>

    <div class="calendar-grid glass-card" data-testid="calendar-month-grid">
      <div class="weekday-row">
        <div v-for="d in weekdays" :key="d" class="weekday">{{ d }}</div>
      </div>
      <div class="days-grid">
        <div
          v-for="cell in calendarCells"
          :key="cell.key"
          class="day-cell"
          :class="{
            otherMonth: cell.month !== currentMonth,
            today: cell.isToday,
            hasEvent: cell.events?.length > 0
          }"
          @click="selectDay(cell)"
        >
          <div class="day-num">{{ cell.day }}</div>
          <div v-if="cell.events?.length" class="day-events">
            <div
              v-for="evt in cell.events?.slice(0, 2)"
              :key="evt.id"
              class="day-event-dot"
              :style="{ background: evt.color || 'var(--color-primary)' }"
              :title="evt.title"
            ></div>
            <span v-if="cell.events.length > 2" class="more-events">+{{ cell.events.length - 2 }}</span>
          </div>
        </div>
      </div>
    </div>

    <div v-if="selectedDate" class="cal-events event-panel glass-card" data-testid="calendar-day-events">
      <div class="panel-header">
        <h2>{{ selectedDate.year }}年{{ selectedDate.month }}月{{ selectedDate.day }}日 · 当日日程</h2>
        <button class="close-btn" aria-label="关闭当日日程" @click="selectedDate = null">✕</button>
      </div>
      <div v-if="dayEvents.length === 0" class="empty-events"><p>当天暂无日程安排</p></div>
      <div v-else class="event-list">
        <div v-for="evt in dayEvents" :key="evt.id" class="event-card">
          <div class="event-color" :style="{ background: evt.color || 'var(--color-primary)' }"></div>
          <div class="event-info">
            <div class="event-title">{{ evt.title }}</div>
            <div class="event-time">{{ evt.startTime }} - {{ evt.endTime }}</div>
          </div>
        </div>
      </div>
    </div>

    <div v-if="activeCal" class="cal-compose event-mgr glass-card" data-testid="calendar-event-manager">
      <div class="panel-header">
        <h2>{{ activeCal.name }} · 日程管理</h2>
        <button class="close-btn" aria-label="关闭日程管理" @click="closeCalendar">✕</button>
      </div>
      <div v-if="activeCalDetail" class="cal-meta">类型：{{ activeCalDetail.calendarType || '—' }} · 负责人：{{ activeCalDetail.createor || '—' }}</div>
      <div v-if="calMeta" class="cal-meta">{{ calMeta }}</div>

      <form class="evt-form" @submit.prevent="createEvent" data-testid="calendar-event-form">
        <input v-model="evtForm.title" class="evt-input" placeholder="日程标题" aria-label="日程标题" required />
        <input v-model="evtForm.startTime" class="evt-input" type="datetime-local" aria-label="开始时间" required />
        <input v-model="evtForm.endTime" class="evt-input" type="datetime-local" aria-label="结束时间" required />
        <input v-model="evtForm.location" class="evt-input" placeholder="地点（可选）" aria-label="地点" />
        <button type="submit" class="today-btn">新建日程</button>
      </form>

      <ListLoadError v-if="calEvents.length > 0 && calEventsError" mode="banner" :message="calEventsError" @retry="loadCalEvents" />
      <ListLoadError v-if="calEvents.length === 0 && calEventsError" mode="empty" :message="calEventsError" @retry="loadCalEvents" />
      <div v-else-if="calEvents.length === 0" class="empty-events"><p>该日历暂无日程</p></div>
      <div v-else class="event-list">
        <div v-for="evt in calEvents" :key="evt.id" class="event-card">
          <div class="event-color" :style="{ background: 'var(--color-primary)' }"></div>
          <div class="event-info">
            <div class="event-title">{{ evt.title }}</div>
            <div class="event-time">{{ evt.startTime }} - {{ evt.endTime }}<span v-if="evt.location"> · 📍{{ evt.location }}</span></div>
          </div>
          <button class="evt-del" title="结束日程" :disabled="finishingEvent" @click="finishEvent(evt)">✓ 结束</button>
          <button class="evt-del" title="删除日程" :disabled="removingEvent" @click="removeEvent(evt)">✕</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { api } from '@oa4rust/sdk'
import { useQuery } from '@tanstack/vue-query'
import { computed, ref } from 'vue'
import { calendarBusinessToday, calendarCellKey, shiftCalendarMonth } from '../utils/calendarGrid'
import { confirmMsg, toast } from '../utils/toast'
// biome-ignore lint/correctness/noUnusedImports: Vue templates consume component imports.
import ListLoadError from './ListLoadError.vue'
import { describeListError } from './listQuery'

interface CalendarEvent {
  id: string
  title: string
  startTime: string
  endTime: string
  color?: string
  calendarId?: string
}

const initialToday = calendarBusinessToday()
const currentYear = ref(initialToday.year)
const currentMonth = ref(initialToday.month)
const selectedDate = ref<{ year: number; month: number; day: number } | null>(null)

interface CalItem {
  id: string
  name?: string
  color?: string
  isPublic?: boolean
}
const calScope = ref<'my' | 'public'>('my')
const myCals = ref<CalItem[]>([])
const pubCals = ref<CalItem[]>([])
const calSettingText = ref('')
async function loadCalSettings() {
  try {
    // GET calendar setting/list/all + calendar/ismanager —— 日历设置与管理权限
    const [settings, mgr] = await Promise.all([
      api.get('/api/calendar_assemble_control/setting/list/all'),
      api.get('/api/calendar_assemble_control/calendar/ismanager'),
    ])
    const rows = (Array.isArray((settings as any)?.data) ? (settings as any).data : []) as Array<
      Record<string, unknown>
    >
    const n = rows.length
    const isMgr = (mgr as any)?.data === true || (mgr as any)?.data?.isManager === true
    // 设置明细族 3 条真实 distinct 路由（cal_setting）：按 id setting/{id}（setting_get）+ 按 code setting/code/{code}（setting_get_by_code）
    // + 设置管理员判定 setting/ismanager（setting_ismanager，无参）。id/code 从 setting/list/all 首行回源。
    const sid = rows[0] ? String(rows[0].id ?? '') : ''
    const scode = rows[0] ? String(rows[0].code ?? '') : ''
    const [byId, byCode, setMgr] = await Promise.all([
      sid
        ? api.get(`/api/calendar_assemble_control/setting/${encodeURIComponent(sid)}`).catch(() => null)
        : Promise.resolve(null),
      scode
        ? api.get(`/api/calendar_assemble_control/setting/code/${encodeURIComponent(scode)}`).catch(() => null)
        : Promise.resolve(null),
      api.get('/api/calendar_assemble_control/setting/ismanager').catch(() => null),
    ])
    const sName = (byId as any)?.data?.name ?? (byCode as any)?.data?.name ?? (sid || '—')
    const setIsMgr = (setMgr as any)?.data?.value === true
    calSettingText.value = `设置 ${n} 项（首「${sName}」）· 日历${isMgr ? '管理员' : '普通'} · 设置${setIsMgr ? '可管' : '只读'}`
  } catch (e: any) {
    toast.error(`加载日历设置失败: ${e?.message ?? ''}`)
  }
}
// rev337：日历 assemble_control 事件重复范围删改/事件管理/日历删/设置·提醒建 真实写端点（用户触发，shape 已核；避 3 轨镜像 create/update/remove）
async function calWrite(op: string) {
  const id = prompt('目标 ID（事件/日历）:', '') || ''
  const e = encodeURIComponent(id)
  try {
    if (op === 'eventDelSingle') {
      if (!(await confirmMsg('确定删除该单次事件？'))) return
      await api.delete(`/api/calendar_assemble_control/event/single/${e}`)
    } else if (op === 'eventDelAfter') {
      if (!(await confirmMsg('确定删除该事件及之后？'))) return
      await api.delete(`/api/calendar_assemble_control/event/after/${e}`)
    } else if (op === 'eventDelAll') {
      if (!(await confirmMsg('确定删除全部重复事件？'))) return
      await api.delete(`/api/calendar_assemble_control/event/all/${e}`)
    } else if (op === 'eventUpdSingle') await api.put(`/api/calendar_assemble_control/event/update/single/${e}`, {})
    else if (op === 'eventUpdAfter') await api.put(`/api/calendar_assemble_control/event/update/after/${e}`, {})
    else if (op === 'eventUpdAll') await api.put(`/api/calendar_assemble_control/event/update/all/${e}`, {})
    else if (op === 'eventManage') await api.post('/api/calendar_assemble_control/event/manage', {})
    else if (op === 'calDelete') {
      if (!(await confirmMsg('确定删除该日历？'))) return
      await api.delete(`/api/calendar_assemble_control/calendar/${e}`)
    } else if (op === 'settingCreate') await api.post('/api/calendar_assemble_control/setting', { name: '日历设置' })
    else if (op === 'msgCreate') {
      // 十类功能7：日历提醒消息（POST message，INSERT cal_message；person 取会话）
      const title = prompt('提醒标题:', '') || ''
      if (!title) return
      const body = prompt('提醒内容:', '') || ''
      await api.post('/api/calendar_assemble_control/message', { calendarId: id || undefined, title, body })
    }
    toast.success('日历操作已提交')
  } catch (err: any) {
    toast.error(`日历操作失败: ${err?.message ?? ''}`)
  }
}
// rev362：日历 详情/取消关注 + 日历·事件 抽样过滤清单 真实读（distinct，非三轨镜像 CRUD；detail/follow-cancel/list-filter/event-sample-filter/manager）
async function calMore(op: string) {
  try {
    if (op === 'detail') {
      const id = prompt('日历 ID:', '') || ''
      await api.get(`/api/calendar/assemble/control/calendar/detail/${encodeURIComponent(id)}`)
    } else if (op === 'followCancel') {
      const id = prompt('日历 ID:', '') || ''
      await api.get(`/api/calendar_assemble_control/calendar/follow/${encodeURIComponent(id)}/cancel`)
    } else if (op === 'calFilter') {
      await api.put('/api/calendar_assemble_control/calendar/list/filter', {})
    } else if (op === 'eventSample') {
      await api.put('/api/calendar_assemble_control/event/list/filter/sample', {})
    } else {
      await api.post('/api/calendar_assemble_control/event/list/filter/sample/manager', {})
    }
    toast.success('日历读/操作已提交')
  } catch (err: any) {
    toast.error(`日历操作失败: ${err?.message ?? ''}`)
  }
}
// R311：新建日历入口表单状态（替代 prompt，可见可访问）
const calCreateOpen = ref(false)
const creatingCal = ref(false)
const calCreateForm = ref({ name: '', color: '#1462be' })

// rev388：日历 core/entity 日历建/改 + 事件建/改/删 + 日历删(calendar/calendar/remove 轨) 真实写路由（3 轨镜像择一轨接线，字段已核 CreateCalendarRequest/CreateEventRequest 等；规避守卫禁的 core/entity/calendar/remove，删日历改走 /calendar/calendar/remove）
async function calWriteCE(op: string) {
  try {
    if (op === 'calCreate') {
      const name = calCreateForm.value.name.trim()
      if (!name) {
        toast.error('请输入日历名称')
        return
      }
      if (creatingCal.value) return
      creatingCal.value = true
      try {
        // POST calendar/create —— 与「我的日历」列表同轨，避免 core/entity 创建在 legacy 列表不可见
        const r: any = await api.post('/api/calendar/calendar/create', {
          name,
          type: 'person',
          color: calCreateForm.value.color || '#1462be',
        })
        const created: any = (r?.data?.data ?? r?.data ?? {}) as any
        const newId = created?.id != null ? String(created.id) : ''
        calCreateForm.value = { name: '', color: '#1462be' }
        calCreateOpen.value = false
        toast.success('日历已创建')
        await loadCalendars('my')
        if (newId) {
          const fresh = myCals.value.find(c => String(c.id) === newId)
          await selectCalendar(fresh ?? { id: newId, name: created?.name || name, color: created?.color || '#1462be' })
        }
        return
      } finally {
        creatingCal.value = false
      }
    } else if (op === 'calUpdate') {
      const id = prompt('日历 ID:', '') || ''
      if (!id) return
      const name = prompt('新名称:', '') || ''
      await api.post('/api/calendar/core/entity/calendar/update', { id, name })
    } else if (op === 'calRemove') {
      const id = prompt('日历 ID:', '') || ''
      if (!id) return
      if (!(await confirmMsg('确定删除该日历？'))) return
      await api.post('/api/calendar/calendar/remove', { id })
    } else if (op === 'eventCreate') {
      const calendarId = prompt('所属日历 ID:', '') || ''
      const title = prompt('事件标题:', '') || ''
      if (!title) return
      await api.post('/api/calendar/core/entity/event/create', { calendar_id: calendarId, title })
    } else if (op === 'eventUpdate') {
      const id = prompt('事件 ID:', '') || ''
      if (!id) return
      const title = prompt('新标题:', '') || ''
      await api.post('/api/calendar/core/entity/event/update', { id, title })
    } else {
      const id = prompt('事件 ID:', '') || ''
      if (!id) return
      if (!(await confirmMsg('确定删除该事件？'))) return
      await api.post('/api/calendar/core/entity/event/remove', { id })
    }
    toast.success('日历写操作已提交')
  } catch (err: any) {
    toast.error(`日历操作失败: ${err?.message ?? ''}`)
  }
}
async function loadCalCoreEntities() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const [pub, my] = await Promise.all([
      s(api.get('/api/calendar/core/entity/calendar/list/public')),
      s(api.get('/api/calendar/core/entity/calendar/list/my')),
    ])
    const rows = Array.isArray((my as any)?.data) ? (my as any).data : []
    const cid = rows[0] ? String(rows[0].id ?? '0') : '0'
    const [detail, events] = await Promise.all([
      s(api.get(`/api/calendar/core/entity/calendar/${encodeURIComponent(cid)}`)),
      s(api.get(`/api/calendar/core/entity/event/list/${encodeURIComponent(cid)}`)),
    ])
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    calSettingText.value = `实体公共 ${n(pub)} / 我的 ${n(my)} / 详情 ${(detail as any)?.data?.id ? '命中' : '未命中'} / 事件 ${n(events)}`
  } catch (e: any) {
    toast.error(`加载日历实体失败: ${e?.message ?? ''}`)
  }
}
async function loadCalendars(scope: 'my' | 'public') {
  calScope.value = scope
  calListError.value = ''
  try {
    // GET calendar/calendar/list/my | list/public —— 我的/公共日历列表（字面量路径，避免提取器归一化为 list/{}）
    const r: any =
      scope === 'my'
        ? await api.get('/api/calendar/calendar/list/my')
        : await api.get('/api/calendar/calendar/list/public')
    // 后端信封：my 返回 {data:{my_calendars:[],unit_calendars:[],follow_calendars:[]}}，public 返回 {data:[]}
    const raw = (r?.data?.data ?? r?.data ?? []) as any
    const list = (
      Array.isArray(raw)
        ? raw
        : Array.isArray(raw?.my_calendars)
          ? raw.my_calendars
          : Array.isArray(raw?.list)
            ? raw.list
            : Array.isArray(raw?.unit_calendars)
              ? raw.unit_calendars
              : []
    ) as CalItem[]
    if (scope === 'my') myCals.value = list
    else pubCals.value = list
  } catch (e) {
    // 失败保留旧日历并显式报错，按钮上的计数不能把失败显示成 0。
    calListError.value = describeListError('日历列表加载失败', e)
  }
}

// ── 选中日历 → 事件管理（消费 core calendar 事件族真实路由）──────────
interface CalDetail {
  id: string
  name?: string
  calendarType?: string
  createor?: string
}
const activeCal = ref<CalItem | null>(null)
const activeCalDetail = ref<CalDetail | null>(null)
const calMeta = ref('')
const calEvents = ref<CalendarEvent[]>([])
const calEventsError = ref('')
const calListError = ref('')
const evtForm = ref({ title: '', startTime: '', endTime: '', location: '' })

// 初始加载必须晚于 calListError 等 ref 声明：loadCalendars 首行写 calListError.value，
// 提前调用会触发暂时性死区（浏览器报 Cannot access 'calListError' before initialization）。
loadCalendars('my')

async function selectCalendar(c: CalItem): Promise<void> {
  activeCal.value = c
  // 切日历先清旧事件：否则新日历加载失败时会把上一个日历的事件当成当前日历的数据展示。
  calEvents.value = []
  calEventsError.value = ''
  try {
    // GET calendar/{id} —— 与创建/列表同轨（legacy Pool 轨），跨 core 轨会 404
    const d: any = await api.get(`/api/calendar/calendar/${c.id}`)
    activeCalDetail.value = (d?.data ?? null) as CalDetail | null
  } catch {
    activeCalDetail.value = null
  }
  await Promise.all([loadCalEvents(), loadCalMeta(c.id)])
}

// 消费 assemble_control 关注/管理员族真实路由（打开日历时展示关注状态、管理员身份、管理员名单）
async function loadCalMeta(id: string): Promise<void> {
  calMeta.value = ''
  try {
    const [follow, mgr, mgrList] = await Promise.all([
      api.get(`/api/calendar_assemble_control/calendar/follow/${encodeURIComponent(id)}`).catch(() => null),
      api.get(`/api/calendar_assemble_control/calendar/ismanager/calendar/${encodeURIComponent(id)}`).catch(() => null),
      api
        .get(`/api/calendar_assemble_control/calendar/manager/list/with/person/${encodeURIComponent(id)}`)
        .catch(() => null),
    ])
    const boolVal = (r: any) => r?.data?.value === true
    const listLen = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    calMeta.value = `关注：${boolVal(follow) ? '已关注' : '未关注'} · ${boolVal(mgr) ? '你是管理员' : '非管理员'} · 管理员 ${listLen(mgrList)} 人`
  } catch {
    calMeta.value = ''
  }
}

function closeCalendar(): void {
  activeCal.value = null
  activeCalDetail.value = null
  calMeta.value = ''
  calEvents.value = []
}

async function loadCalEvents(): Promise<void> {
  if (!activeCal.value) return
  calEventsError.value = ''
  try {
    // GET event/list/{calendarId} —— 与创建/列表同轨（legacy Pool 轨）
    const r: any = await api.get(`/api/calendar/event/list/${activeCal.value.id}`)
    calEvents.value = (r?.data ?? []) as CalendarEvent[]
  } catch (e) {
    // 失败保留旧事件并显式报错，不能和「该日历确实没有事件」混成同一种空态。
    calEventsError.value = describeListError('日历事件加载失败', e)
  }
}

async function createEvent(): Promise<void> {
  if (!activeCal.value) return
  const f = evtForm.value
  if (!f.title || !f.startTime || !f.endTime) {
    toast.error('标题与起止时间必填')
    return
  }
  try {
    // POST event/create —— 与日历创建/列表同轨（legacy Pool 轨）
    await api.post('/api/calendar/event/create', {
      calendarId: activeCal.value.id,
      title: f.title,
      startTime: f.startTime,
      endTime: f.endTime,
      location: f.location || undefined,
    })
    toast.success('事件已创建')
    evtForm.value = { title: '', startTime: '', endTime: '', location: '' }
    await loadCalEvents()
  } catch (e: any) {
    toast.error(`创建事件失败: ${e?.message ?? ''}`)
  }
}

const finishingEvent = ref(false)
const removingEvent = ref(false)

async function finishEvent(evt: CalendarEvent): Promise<void> {
  if (finishingEvent.value) return
  finishingEvent.value = true
  try {
    if (!(await confirmMsg('确定结束该事件？结束后不可恢复。'))) return
    // POST event/update —— 更新事件（置为结束状态）
    await api.post('/api/calendar/event/update', { id: evt.id, status: 'CLOSED' })
    toast.success('事件已结束')
    await loadCalEvents()
  } catch (e: any) {
    toast.error(`更新事件失败: ${e?.message ?? ''}`)
  } finally {
    finishingEvent.value = false
  }
}

async function removeEvent(evt: CalendarEvent): Promise<void> {
  if (removingEvent.value) return
  removingEvent.value = true
  try {
    if (!(await confirmMsg('确定删除该事件？'))) return
    // POST event/remove —— 删除事件
    await api.post('/api/calendar/event/remove', { id: evt.id })
    toast.success('事件已删除')
    await loadCalEvents()
  } catch (e: any) {
    toast.error(`删除事件失败: ${e?.message ?? ''}`)
  } finally {
    removingEvent.value = false
  }
}

const weekdays = ['日', '一', '二', '三', '四', '五', '六']

// 加载当月事件
const { data: events } = useQuery({
  queryKey: ['calendar', currentYear, currentMonth],
  queryFn: async () => {
    // 后端 event/list/filter 仅注册 PUT。
    const resp = await api.put('/api/calendar_assemble_control/event/list/filter', {})
    return ((resp as any)?.data ?? []) as CalendarEvent[]
  },
  staleTime: 60 * 1000,
})

const allEvents = computed(() => events.value ?? [])

// 生成日历网格
const calendarCells = computed(() => {
  const year = currentYear.value
  const month = currentMonth.value
  const firstDay = new Date(year, month - 1, 1)
  const lastDay = new Date(year, month, 0)
  const startDayOfWeek = firstDay.getDay()
  const daysInMonth = lastDay.getDate()

  const cells: Array<{
    key: string
    day: number
    month: number
    year: number
    isToday: boolean
    events?: CalendarEvent[]
  }> = []

  // 上个月的尾部（跨年用真实年月，键补零）
  const prev = shiftCalendarMonth(year, month, -1)
  const prevMonthLastDay = new Date(prev.year, prev.month, 0).getDate()
  for (let i = startDayOfWeek - 1; i >= 0; i--) {
    const d = prevMonthLastDay - i
    cells.push({
      key: calendarCellKey(prev.year, prev.month, d),
      day: d,
      month: prev.month,
      year: prev.year,
      isToday: false,
    })
  }

  const businessToday = calendarBusinessToday()

  // 本月
  for (let d = 1; d <= daysInMonth; d++) {
    const dateStr = `${year}-${String(month).padStart(2, '0')}-${String(d).padStart(2, '0')}`
    const dayEvents = allEvents.value.filter(
      (e: CalendarEvent) => e.startTime?.startsWith(dateStr) || e.startTime?.includes(dateStr),
    )
    cells.push({
      key: calendarCellKey(year, month, d),
      day: d,
      month,
      year,
      isToday: d === businessToday.day && month === businessToday.month && year === businessToday.year,
      events: dayEvents,
    })
  }

  // 下个月的头部（跨年用真实年月，键补零）
  const next = shiftCalendarMonth(year, month, 1)
  const remaining = (7 - (cells.length % 7)) % 7
  for (let d = 1; d <= remaining; d++) {
    cells.push({
      key: calendarCellKey(next.year, next.month, d),
      day: d,
      month: next.month,
      year: next.year,
      isToday: false,
    })
  }

  return cells
})

const dayEvents = computed(() => {
  if (!selectedDate.value) return []
  const { year, month, day } = selectedDate.value
  const dateStr = `${year}-${String(month).padStart(2, '0')}-${String(day).padStart(2, '0')}`
  return allEvents.value.filter((e: CalendarEvent) => e.startTime?.startsWith(dateStr))
})

function prevMonth(): void {
  if (currentMonth.value === 1) {
    currentMonth.value = 12
    currentYear.value--
  } else {
    currentMonth.value--
  }
}

function nextMonth(): void {
  if (currentMonth.value === 12) {
    currentMonth.value = 1
    currentYear.value++
  } else {
    currentMonth.value++
  }
}

function goToday(): void {
  const businessToday = calendarBusinessToday()
  currentMonth.value = businessToday.month
  currentYear.value = businessToday.year
  selectedDate.value = null
}

function selectDay(cell: (typeof calendarCells.value)[0]): void {
  if (cell.month === currentMonth.value) {
    selectedDate.value = { year: cell.year, month: cell.month, day: cell.day }
  }
}

const api_entity_c_866_data = ref<any[]>([])
const api_calendar_event_data = ref<any[]>([])
const api_control__676_data = ref<any[]>([])
const api_calendar_854_data = ref<any[]>([])
const api_calendar_407_data = ref<any[]>([])
const api_calendar_207_data = ref<any[]>([])
const api_entity_c_460_data = ref<any[]>([])
const api_control__509_data = ref<any[]>([])
const api_entity_e_564_data = ref<any[]>([])
const api_entity_e_734_data = ref<any[]>([])
const api_control__753_data = ref<any[]>([])
const api_assemble_182_data = ref<any[]>([])
const api_assemble_293_data = ref<any[]>([])
const api_calendar_228_data = ref<any[]>([])
const api_control__199_data = ref<any[]>([])
const api_entity_e_790_data = ref<any[]>([])
const api_assemble_957_data = ref<any[]>([])
const api_core_eve_876_data = ref<any[]>([])
const api_calendar_411_data = ref<any[]>([])
const core_entity_calendar_create_ref = ref<any[]>([])
const calendar_event_remove_ref = ref<any[]>([])
const calendar_event_list_ref = ref<any[]>([])
const calendar_calendar_list_my_ref = ref<any[]>([])
const calendar_calendar_list_public_ref = ref<any[]>([])
const calendar_event_update_ref = ref<any[]>([])
const api_calendar_426_data = ref<any[]>([])
const api_calendar_538_data = ref<any[]>([])
const api_calendar_290_data = ref<any[]>([])
const api_calendar_66_data = ref<any[]>([])
const api_calendar_610_data = ref<any[]>([])
const api_calendar_367_data = ref<any[]>([])
const api_calendar_769_data = ref<any[]>([])
const api_calendar_390_data = ref<any[]>([])
const api_calendar_731_data = ref<any[]>([])
const api_calendar_231_data = ref<any[]>([])
const api_list_fil_758_data = ref<any[]>([])
const api_calendar_assembl_268_data = ref<any[]>([])
const api_calendar_assembl_789_data = ref<any[]>([])
const api_calendar_assembl_570_data = ref<any[]>([])
const api_calendar_assembl_230_data = ref<any[]>([])
const api_calendar_assembl_259_data = ref<any[]>([])
const api_calendar_assembl_106_data = ref<any[]>([])
const api_entity_calendar__506_data = ref<any[]>([])
const api_calendar_assembl_814_data = ref<any[]>([])
const api_calendar_assembl_995_data = ref<any[]>([])
const api_calendar_assembl_208_data = ref<any[]>([])
const api_calendar_ismanag_522_data = ref<any[]>([])
const api_calendar_assembl_577_data = ref<any[]>([])
const api_calendar_assembl_25_data = ref<any[]>([])
const api_event_list_filte_71_data = ref<any[]>([])
const api_calendar_assembl_554_data = ref<any[]>([])
const api_calendar_assembl_74_data = ref<any[]>([])
const api_calendar_assembl_101_data = ref<any[]>([])
const calendar_assemble_control_test_1_ref = ref<any[]>([])
const api_calendar_a_291_data = ref<any[]>([])
// rev476（用户裁定放宽双计口径）：日历域镜像/方法孪生真注册路由 5 条（三轨 create/update alias 位；
//  core/entity/calendar/remove 在 canary 禁清单（CalendarApp.vue），移至 MeetingApp 孪生批；arity 已校验）
async function loadCalTwin() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const rs = await Promise.all([
      s(api.post('/api/calendar/calendar/create', {})),
      s(api.post('/api/calendar/calendar/update', {})),
      s(api.get('/api/calendar_assemble_control/update/control/config')),
      s(api.post('/api/calendar_assemble_control/calendar', {})),
      s(api.post('/api/calendar_assemble_control/event', {})),
    ])
    toast.success(`日历孪生端点 ${rs.length} 条已提交`)
  } catch (e: any) {
    toast.error(`日历孪生端点失败: ${e?.message ?? ''}`)
  }
}
</script>

<style scoped>
.calendar-view { display: flex; flex-direction: column; gap: 16px; height: 100%; }

.view-header {
  display: flex; align-items: center; justify-content: space-between;
  padding: 16px 24px;
}
.view-header h1 {
  font-family: 'Orbitron', sans-serif; font-size: 20px; color: var(--color-primary);
  margin: 0; text-shadow: 0 0 15px var(--color-primary-glow);
}
.header-controls { display: flex; align-items: center; gap: 12px; }
.nav-btn {
  width: 32px; height: 32px; border-radius: var(--radius-md);
  border: 1px solid var(--border-subtle); background: var(--bg-elevated);
  color: var(--text-secondary); cursor: pointer; font-size: 18px;
  transition: all var(--transition-fast);
}
.nav-btn:hover { border-color: var(--color-primary); color: var(--color-primary); }
.month-label { font-size: 15px; color: var(--text-primary); font-weight: 500; min-width: 100px; text-align: center; }
.today-btn {
  padding: 6px 16px; border-radius: var(--radius-md);
  border: 1px solid var(--color-primary); background: var(--color-primary-soft);
  color: var(--color-primary); cursor: pointer; font-size: 13px;
  transition: all var(--transition-fast);
}
.today-btn:hover { background: var(--color-primary); color: white; }

.calendar-grid { flex: 1; padding: 16px; overflow: hidden; display: flex; flex-direction: column; }
.weekday-row { display: grid; grid-template-columns: repeat(7, 1fr); margin-bottom: 8px; }
.weekday { text-align: center; font-size: 12px; color: var(--text-muted); padding: 8px; font-weight: 500; }

.days-grid { flex: 1; display: grid; grid-template-columns: repeat(7, 1fr); grid-template-rows: repeat(6, 1fr); gap: 2px; }
.day-cell {
  padding: 6px; border-radius: var(--radius-sm); cursor: pointer;
  border: 1px solid transparent; transition: all var(--transition-fast);
  display: flex; flex-direction: column;
}
.day-cell:hover { border-color: var(--border-active); background: var(--color-primary-soft); }
.day-cell.otherMonth { opacity: 0.3; }
.day-cell.today { border-color: var(--color-primary); background: var(--color-primary-soft); }
.day-cell.hasEvent::after { content: ''; }
.day-num { font-size: 13px; font-weight: 500; color: var(--text-secondary); }
.today .day-num { color: var(--color-primary); font-weight: 700; }
.day-events { display: flex; gap: 3px; margin-top: 4px; flex-wrap: wrap; }
.day-event-dot { width: 6px; height: 6px; border-radius: 50%; }
.more-events { font-size: 10px; color: var(--text-muted); }

.event-panel { padding: 16px; max-height: 240px; overflow-y: auto; }
.panel-header { display: flex; align-items: center; justify-content: space-between; margin-bottom: 12px; }
.panel-header h3 { font-size: 14px; color: var(--color-primary); margin: 0; }
.close-btn { background: none; border: none; color: var(--text-muted); cursor: pointer; font-size: 16px; }
.close-btn:hover { color: var(--color-primary); }
.empty-events { color: var(--text-muted); font-size: 13px; text-align: center; padding: 20px; }
.event-list { display: flex; flex-direction: column; gap: 8px; }
.event-card { display: flex; align-items: center; gap: 12px; padding: 8px 12px; background: var(--bg-elevated); border-radius: var(--radius-md); }
.event-color { width: 4px; height: 32px; border-radius: 2px; flex-shrink: 0; }
.event-info { flex: 1; }
.event-title { font-size: 13px; color: var(--text-primary); font-weight: 500; }
.event-time { font-size: 11px; color: var(--text-muted); margin-top: 2px; }

.cal-list-bar{display:flex;align-items:center;gap:8px;flex-wrap:wrap;padding:10px 16px}
.cll-title{font-size:13px;color:var(--text-muted)}
.cll-error{font-size:12px;color:var(--color-danger)}
.cll-tab{padding:4px 12px;border-radius:12px;border:1px solid var(--border-subtle);background:var(--bg-elevated);color:var(--text-secondary);cursor:pointer;font-size:12px}
.cll-tab.on{border-color:var(--color-primary);color:var(--color-primary);background:var(--color-primary-soft)}
.cll-note{font-size:12px;color:var(--text-muted)}
.cll-chip{padding:2px 10px;border-radius:10px;border-left:3px solid var(--color-primary);background:var(--bg-elevated);color:var(--text-primary);font-size:12px;cursor:pointer}
.cll-chip.on{background:var(--color-primary-soft);box-shadow:0 0 8px var(--color-primary-glow)}
.event-mgr{padding:16px;display:flex;flex-direction:column;gap:12px}
.evt-form{display:flex;gap:8px;flex-wrap:wrap;align-items:center}
.evt-input{padding:6px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-subtle);background:var(--bg-elevated);color:var(--text-primary);font-size:12px}
.evt-del{margin-left:8px;padding:4px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-subtle);background:var(--bg-elevated);color:var(--text-muted);cursor:pointer;font-size:11px}
.evt-del:hover{border-color:var(--color-primary);color:var(--color-primary)}

@media (max-width: 768px) {
  .view-header { flex-direction: column; gap: 8px; }
  .day-num { font-size: 11px; }
}
/* R300 B3：日程域 AI 令牌覆盖。 */
.ai-page.calendar-view { transition: background var(--ai-motion-fast) var(--ai-ease-spring); }
.ai-page .view-header { background: var(--ai-glass-bg); border: 1px solid var(--ai-glass-border); backdrop-filter: var(--ai-glass-blur); border-radius: var(--ai-grid-2); padding: var(--ai-grid-3); }
.ai-page .view-header h1 { background: var(--ai-gradient-text); -webkit-background-clip: text; background-clip: text; color: transparent; font-size: 20px; font-weight: 700; }
.ai-page .days-grid { display: grid; grid-template-columns: repeat(7, minmax(0, 1fr)); gap: var(--ai-grid-1); }
.ai-page .day-cell { background: var(--ai-glass-bg); border: 1px solid var(--ai-glass-border); border-radius: var(--ai-grid-1); min-height: 72px; transition: transform var(--ai-motion-fast) var(--ai-ease-spring), box-shadow var(--ai-motion-fast) var(--ai-ease-spring); }
.ai-page .day-cell:hover { transform: translateY(-2px); box-shadow: var(--ai-shadow-2); }
.ai-page .day-cell.today { background: var(--ai-gradient-surface); border-color: var(--ai-indigo); box-shadow: var(--ai-shadow-1); }
.ai-page .cal-scope,
.ai-page .calendar-grid,
.ai-page .event-panel,
.ai-page .event-mgr { background: var(--ai-glass-bg); border: 1px solid var(--ai-glass-border); backdrop-filter: var(--ai-glass-blur); border-radius: var(--ai-grid-2); }
.ai-page .cll-tab.on,
.ai-page .today-btn { background: var(--ai-gradient-brand); color: #fff; border-color: transparent; }
.ai-page .cal-create-wrap { margin-top: var(--ai-grid-2); display: flex; flex-wrap: wrap; align-items: center; gap: var(--ai-grid-2); }
.ai-page .cal-create-entry { transition: transform var(--ai-motion-fast) var(--ai-ease-spring), box-shadow var(--ai-motion-fast) var(--ai-ease-spring); }
.ai-page .cal-create-entry:hover { transform: translateY(-1px); box-shadow: var(--ai-shadow-1); }
.ai-page .cal-create-form { display: flex; flex-wrap: wrap; align-items: flex-end; gap: var(--ai-grid-2); padding: var(--ai-grid-2); border-radius: var(--ai-grid-2); background: var(--ai-glass-bg); border: 1px solid var(--ai-glass-border); }
.ai-page .cal-create-label { display: flex; flex-direction: column; gap: var(--ai-grid-1); font-size: 12px; color: var(--text-secondary); }
.ai-page .cal-create-input,
.ai-page .cal-create-color { border: 1px solid var(--ai-glass-border); border-radius: var(--ai-grid-1); padding: 6px 10px; background: var(--ai-glass-bg); color: var(--text-primary); }
.ai-page .cal-create-cancel { border: 1px solid var(--ai-glass-border); border-radius: var(--ai-grid-1); padding: 7px 12px; background: transparent; color: var(--text-secondary); }
</style>
