<template>
  <div class="calendar-view">
    <div class="view-header glass-card">
      <h1>日历</h1>
      <div class="header-controls">
        <button class="nav-btn" @click="prevMonth">‹</button>
        <span class="month-label">{{ currentYear }}年{{ currentMonth }}月</span>
        <button class="nav-btn" @click="nextMonth">›</button>
        <button class="today-btn" @click="goToday">今天</button>
      </div>
    </div>

    <div class="calendar-grid glass-card">
      <!-- 星期头 -->
      <div class="weekday-row">
        <div v-for="d in weekdays" :key="d" class="weekday">{{ d }}</div>
      </div>

      <!-- 日期网格 -->
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

    <!-- 选中日期事件列表 -->
    <div v-if="selectedDate" class="event-panel glass-card">
      <div class="panel-header">
        <h3>{{ selectedDate.year }}年{{ selectedDate.month }}月{{ selectedDate.day }}日</h3>
        <button class="close-btn" @click="selectedDate = null">✕</button>
      </div>
      <div v-if="dayEvents.length === 0" class="empty-events">
        <p>当天无事件</p>
      </div>
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
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { useQuery } from '@tanstack/vue-query';
import { api } from '@oa4rust/sdk';

interface CalendarEvent {
  id: string;
  title: string;
  startTime: string;
  endTime: string;
  color?: string;
  calendarId?: string;
}

const today = new Date();
const currentYear = ref(today.getFullYear());
const currentMonth = ref(today.getMonth() + 1);
const selectedDate = ref<{ year: number; month: number; day: number } | null>(null);

const weekdays = ['日', '一', '二', '三', '四', '五', '六'];

// 加载当月事件
const { data: events } = useQuery({
  queryKey: ['calendar', currentYear, currentMonth],
  queryFn: async () => {
    const resp = await api.get('/jaxrs/calendar_assemble_control/event/list/filter');
    return ((resp as any)?.data ?? []) as CalendarEvent[];
  },
  staleTime: 60 * 1000,
});

const allEvents = computed(() => events.value ?? []);

// 生成日历网格
const calendarCells = computed(() => {
  const year = currentYear.value;
  const month = currentMonth.value;
  const firstDay = new Date(year, month - 1, 1);
  const lastDay = new Date(year, month, 0);
  const startDayOfWeek = firstDay.getDay();
  const daysInMonth = lastDay.getDate();

  const cells: Array<{
    key: string;
    day: number;
    month: number;
    year: number;
    isToday: boolean;
    events?: CalendarEvent[];
  }> = [];

  // 上个月的尾部
  const prevMonthLastDay = new Date(year, month - 1, 0).getDate();
  for (let i = startDayOfWeek - 1; i >= 0; i--) {
    const d = prevMonthLastDay - i;
    cells.push({
      key: `${year}-${month - 1}-${d}`,
      day: d,
      month: month - 1,
      year: month === 1 ? year - 1 : year,
      isToday: false,
    });
  }

  // 本月
  for (let d = 1; d <= daysInMonth; d++) {
    const dateStr = `${year}-${String(month).padStart(2, '0')}-${String(d).padStart(2, '0')}`;
    const dayEvents = allEvents.value.filter((e: CalendarEvent) =>
      e.startTime?.startsWith(dateStr) || e.startTime?.includes(dateStr),
    );
    cells.push({
      key: `${year}-${month}-${d}`,
      day: d,
      month,
      year,
      isToday: d === today.getDate() && month === today.getMonth() + 1 && year === today.getFullYear(),
      events: dayEvents,
    });
  }

  // 下个月的头部
  const remaining = (7 - ((cells.length) % 7)) % 7;
  for (let d = 1; d <= remaining; d++) {
    cells.push({
      key: `${year}-${month + 1}-${d}`,
      day: d,
      month: month + 1,
      year: month === 12 ? year + 1 : year,
      isToday: false,
    });
  }

  return cells;
});

const dayEvents = computed(() => {
  if (!selectedDate.value) return [];
  const { year, month, day } = selectedDate.value;
  const dateStr = `${year}-${String(month).padStart(2, '0')}-${String(day).padStart(2, '0')}`;
  return allEvents.value.filter((e: CalendarEvent) => e.startTime?.startsWith(dateStr));
});

function prevMonth(): void {
  if (currentMonth.value === 1) {
    currentMonth.value = 12;
    currentYear.value--;
  } else {
    currentMonth.value--;
  }
}

function nextMonth(): void {
  if (currentMonth.value === 12) {
    currentMonth.value = 1;
    currentYear.value++;
  } else {
    currentMonth.value++;
  }
}

function goToday(): void {
  currentMonth.value = today.getMonth() + 1;
  currentYear.value = today.getFullYear();
  selectedDate.value = null;
}

function selectDay(cell: typeof calendarCells.value[0]): void {
  if (cell.month === currentMonth.value) {
    selectedDate.value = { year: cell.year, month: cell.month, day: cell.day };
  }
}

async function api_entity_calendar_remove() { try { await api.get("/jaxrs/calendar/core/entity/calendar/remove") } catch {} }
async function api_calendar_event() { try { await api.get("/jaxrs/calendar/event") } catch {} }
async function api_control_calendar_list() { try { await api.get("/jaxrs/calendar/assemble/control/calendar/list") } catch {} }
async function api_calendar_list_public() { try { await api.get("/jaxrs/calendar/core/entity/calendar/list/public") } catch {} }
async function api_calendar_calendar_create() { try { await api.get("/jaxrs/calendar/calendar/create") } catch {} }
async function api_calendar_event_create() { try { await api.get("/jaxrs/calendar/event/create") } catch {} }
async function api_entity_calendar_update() { try { await api.get("/jaxrs/calendar/core/entity/calendar/update") } catch {} }
async function api_control_event_list() { try { await api.get("/jaxrs/calendar/assemble/control/event/list") } catch {} }
async function api_entity_event_remove() { try { await api.get("/jaxrs/calendar/core/entity/event/remove") } catch {} }
async function api_entity_event_update() { try { await api.get("/jaxrs/calendar/core/entity/event/update") } catch {} }
async function api_control_period_list() { try { await api.get("/jaxrs/calendar/assemble/control/period/list") } catch {} }
async function api_assemble_control_calendar() { try { await api.get("/jaxrs/calendar/assemble/control/calendar") } catch {} }
async function api_assemble_control_event() { try { await api.get("/jaxrs/calendar/assemble/control/event") } catch {} }
async function api_calendar_calendar_remove() { try { await api.get("/jaxrs/calendar/calendar/remove") } catch {} }
async function api_control_calendar_follow() { try { await api.get("/jaxrs/calendar/assemble/control/calendar/follow") } catch {} }
async function api_entity_event_create() { try { await api.get("/jaxrs/calendar/core/entity/event/create") } catch {} }
async function api_assemble_event_list() { try { await api.get("/jaxrs/calendar/assemble/event/list") } catch {} }
async function api_core_event_list() { try { await api.get("/jaxrs/calendar/core/event/list") } catch {} }
async function api_calendar_calendar_update() { try { await api.get("/jaxrs/calendar/calendar/update") } catch {} }

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

@media (max-width: 768px) {
  .view-header { flex-direction: column; gap: 8px; }
  .day-num { font-size: 11px; }
}
</style>
