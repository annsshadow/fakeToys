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
import { toast } from '../utils/toast';
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

const api_entity_c_866_data = ref<any[]>([]);
const { data: api_entity_c_866_q } = useQuery({queryKey: ['api_entity_c_866', '/jaxrs/calendar/core/entity/calendar/remove'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar/core/entity/calendar/remove"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_entity_c_866_q, (v) => { api_entity_c_866_data.value = v ?? []; });
const api_calendar_event_data = ref<any[]>([]);
const { data: api_calendar_event_q } = useQuery({queryKey: ['api_calendar_event', '/jaxrs/calendar/event'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar/event"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_calendar_event_q, (v) => { api_calendar_event_data.value = v ?? []; });
const api_control__676_data = ref<any[]>([]);
const { data: api_control__676_q } = useQuery({queryKey: ['api_control__676', '/jaxrs/calendar/assemble/control/calendar/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar/assemble/control/calendar/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_control__676_q, (v) => { api_control__676_data.value = v ?? []; });
const api_calendar_854_data = ref<any[]>([]);
const { data: api_calendar_854_q } = useQuery({queryKey: ['api_calendar_854', '/jaxrs/calendar/core/entity/calendar/list/public'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar/core/entity/calendar/list/public"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_calendar_854_q, (v) => { api_calendar_854_data.value = v ?? []; });
const api_calendar_407_data = ref<any[]>([]);
const { data: api_calendar_407_q } = useQuery({queryKey: ['api_calendar_407', '/jaxrs/calendar/calendar/create'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar/calendar/create"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_calendar_407_q, (v) => { api_calendar_407_data.value = v ?? []; });
const api_calendar_207_data = ref<any[]>([]);
const { data: api_calendar_207_q } = useQuery({queryKey: ['api_calendar_207', '/jaxrs/calendar/event/create'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar/event/create"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_calendar_207_q, (v) => { api_calendar_207_data.value = v ?? []; });
const api_entity_c_460_data = ref<any[]>([]);
const { data: api_entity_c_460_q } = useQuery({queryKey: ['api_entity_c_460', '/jaxrs/calendar/core/entity/calendar/update'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar/core/entity/calendar/update"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_entity_c_460_q, (v) => { api_entity_c_460_data.value = v ?? []; });
const api_control__509_data = ref<any[]>([]);
const { data: api_control__509_q } = useQuery({queryKey: ['api_control__509', '/jaxrs/calendar/assemble/control/event/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar/assemble/control/event/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_control__509_q, (v) => { api_control__509_data.value = v ?? []; });
const api_entity_e_564_data = ref<any[]>([]);
const { data: api_entity_e_564_q } = useQuery({queryKey: ['api_entity_e_564', '/jaxrs/calendar/core/entity/event/remove'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar/core/entity/event/remove"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_entity_e_564_q, (v) => { api_entity_e_564_data.value = v ?? []; });
const api_entity_e_734_data = ref<any[]>([]);
const { data: api_entity_e_734_q } = useQuery({queryKey: ['api_entity_e_734', '/jaxrs/calendar/core/entity/event/update'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar/core/entity/event/update"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_entity_e_734_q, (v) => { api_entity_e_734_data.value = v ?? []; });
const api_control__753_data = ref<any[]>([]);
const { data: api_control__753_q } = useQuery({queryKey: ['api_control__753', '/jaxrs/calendar/assemble/control/period/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar/assemble/control/period/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_control__753_q, (v) => { api_control__753_data.value = v ?? []; });
const api_assemble_182_data = ref<any[]>([]);
const { data: api_assemble_182_q } = useQuery({queryKey: ['api_assemble_182', '/jaxrs/calendar/assemble/control/calendar'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar/assemble/control/calendar"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_assemble_182_q, (v) => { api_assemble_182_data.value = v ?? []; });
const api_assemble_293_data = ref<any[]>([]);
const { data: api_assemble_293_q } = useQuery({queryKey: ['api_assemble_293', '/jaxrs/calendar/assemble/control/event'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar/assemble/control/event"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_assemble_293_q, (v) => { api_assemble_293_data.value = v ?? []; });
const api_calendar_228_data = ref<any[]>([]);
const { data: api_calendar_228_q } = useQuery({queryKey: ['api_calendar_228', '/jaxrs/calendar/calendar/remove'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar/calendar/remove"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_calendar_228_q, (v) => { api_calendar_228_data.value = v ?? []; });
const api_control__199_data = ref<any[]>([]);
const { data: api_control__199_q } = useQuery({queryKey: ['api_control__199', '/jaxrs/calendar/assemble/control/calendar/follow'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar/assemble/control/calendar/follow"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_control__199_q, (v) => { api_control__199_data.value = v ?? []; });
const api_entity_e_790_data = ref<any[]>([]);
const { data: api_entity_e_790_q } = useQuery({queryKey: ['api_entity_e_790', '/jaxrs/calendar/core/entity/event/create'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar/core/entity/event/create"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_entity_e_790_q, (v) => { api_entity_e_790_data.value = v ?? []; });
const api_assemble_957_data = ref<any[]>([]);
const { data: api_assemble_957_q } = useQuery({queryKey: ['api_assemble_957', '/jaxrs/calendar/assemble/event/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar/assemble/event/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_assemble_957_q, (v) => { api_assemble_957_data.value = v ?? []; });
const api_core_eve_876_data = ref<any[]>([]);
const { data: api_core_eve_876_q } = useQuery({queryKey: ['api_core_eve_876', '/jaxrs/calendar/core/event/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar/core/event/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_core_eve_876_q, (v) => { api_core_eve_876_data.value = v ?? []; });
const api_calendar_411_data = ref<any[]>([]);
const { data: api_calendar_411_q } = useQuery({queryKey: ['api_calendar_411', '/jaxrs/calendar/calendar/update'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar/calendar/update"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_calendar_411_q, (v) => { api_calendar_411_data.value = v ?? []; });


const core_entity_calendar_create_ref = ref<any[]>([]);
const core_entity_calendar_create_q = useQuery({
  queryKey: ['core_entity_calendar_create'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/calendar/core/entity/calendar/create"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const calendar_event_remove_ref = ref<any[]>([]);
const calendar_event_remove_q = useQuery({
  queryKey: ['calendar_event_remove'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/calendar/event/remove"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const calendar_event_list_ref = ref<any[]>([]);
const calendar_event_list_q = useQuery({
  queryKey: ['calendar_event_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/calendar/event/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const calendar_calendar_list_my_ref = ref<any[]>([]);
const calendar_calendar_list_my_q = useQuery({
  queryKey: ['calendar_calendar_list_my'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/calendar/calendar/list/my"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const calendar_calendar_list_public_ref = ref<any[]>([]);
const calendar_calendar_list_public_q = useQuery({
  queryKey: ['calendar_calendar_list_public'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/calendar/calendar/list/public"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const calendar_event_update_ref = ref<any[]>([]);
const calendar_event_update_q = useQuery({
  queryKey: ['calendar_event_update'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/calendar/event/update"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});


const api_calendar_426_data = ref<any[]>([]);
const { data: api_calendar_426_q } = useQuery({queryKey: ['api_calendar_426', '/jaxrs/calendar_assemble_control/update/control/config'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar_assemble_control/update/control/config"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_calendar_426_q, (v) => { api_calendar_426_data.value = v ?? []; });
const api_calendar_538_data = ref<any[]>([]);
const { data: api_calendar_538_q } = useQuery({queryKey: ['api_calendar_538', '/jaxrs/calendar_assemble_control/setting/x'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar_assemble_control/setting/x"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_calendar_538_q, (v) => { api_calendar_538_data.value = v ?? []; });
const api_calendar_290_data = ref<any[]>([]);
const { data: api_calendar_290_q } = useQuery({queryKey: ['api_calendar_290', '/jaxrs/calendar_assemble_control/calendar/follow/x/cancel'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar_assemble_control/calendar/follow/x/cancel"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_calendar_290_q, (v) => { api_calendar_290_data.value = v ?? []; });
const api_calendar_66_data = ref<any[]>([]);
const { data: api_calendar_66_q } = useQuery({queryKey: ['api_calendar_66', '/jaxrs/calendar_assemble_control/setting/code/x'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar_assemble_control/setting/code/x"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_calendar_66_q, (v) => { api_calendar_66_data.value = v ?? []; });
const api_calendar_610_data = ref<any[]>([]);
const { data: api_calendar_610_q } = useQuery({queryKey: ['api_calendar_610', '/jaxrs/calendar_assemble_control/event/all/x'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar_assemble_control/event/all/x"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_calendar_610_q, (v) => { api_calendar_610_data.value = v ?? []; });
const api_calendar_367_data = ref<any[]>([]);
const { data: api_calendar_367_q } = useQuery({queryKey: ['api_calendar_367', '/jaxrs/calendar_assemble_control/calendar/list/my'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar_assemble_control/calendar/list/my"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_calendar_367_q, (v) => { api_calendar_367_data.value = v ?? []; });
const api_calendar_769_data = ref<any[]>([]);
const { data: api_calendar_769_q } = useQuery({queryKey: ['api_calendar_769', '/jaxrs/calendar_assemble_control/calendar/ismanager'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar_assemble_control/calendar/ismanager"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_calendar_769_q, (v) => { api_calendar_769_data.value = v ?? []; });
const api_calendar_390_data = ref<any[]>([]);
const { data: api_calendar_390_q } = useQuery({queryKey: ['api_calendar_390', '/jaxrs/calendar_assemble_control/setting/ismanager'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar_assemble_control/setting/ismanager"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_calendar_390_q, (v) => { api_calendar_390_data.value = v ?? []; });
const api_calendar_731_data = ref<any[]>([]);
const { data: api_calendar_731_q } = useQuery({queryKey: ['api_calendar_731', '/jaxrs/calendar_assemble_control/calendar/list/filter'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar_assemble_control/calendar/list/filter"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_calendar_731_q, (v) => { api_calendar_731_data.value = v ?? []; });
const api_calendar_231_data = ref<any[]>([]);
const { data: api_calendar_231_q } = useQuery({queryKey: ['api_calendar_231', '/jaxrs/calendar_assemble_control/calendar/x'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar_assemble_control/calendar/x"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_calendar_231_q, (v) => { api_calendar_231_data.value = v ?? []; });
const api_list_fil_758_data = ref<any[]>([]);
const { data: api_list_fil_758_q } = useQuery({queryKey: ['api_list_fil_758', '/jaxrs/calendar_assemble_control/event/list/filter/sample/manager'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar_assemble_control/event/list/filter/sample/manager"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_list_fil_758_q, (v) => { api_list_fil_758_data.value = v ?? []; });
const api_calendar_assembl_268_data = ref<any[]>([]);
const { data: api_calendar_assembl_268_q } = useQuery({queryKey: ['api_calendar_assembl_268', '/jaxrs/calendar_assemble_control/calendar'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar_assemble_control/calendar"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_calendar_assembl_268_q, (v) => { api_calendar_assembl_268_data.value = v ?? []; });
const api_calendar_assembl_789_data = ref<any[]>([]);
const { data: api_calendar_assembl_789_q } = useQuery({queryKey: ['api_calendar_assembl_789', '/jaxrs/calendar_assemble_control/list/control/calendars'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar_assemble_control/list/control/calendars"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_calendar_assembl_789_q, (v) => { api_calendar_assembl_789_data.value = v ?? []; });
const api_calendar_assembl_570_data = ref<any[]>([]);
const { data: api_calendar_assembl_570_q } = useQuery({queryKey: ['api_calendar_assembl_570', '/jaxrs/calendar_assemble_control/event/rfc/x'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar_assemble_control/event/rfc/x"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_calendar_assembl_570_q, (v) => { api_calendar_assembl_570_data.value = v ?? []; });
const api_calendar_assembl_230_data = ref<any[]>([]);
const { data: api_calendar_assembl_230_q } = useQuery({queryKey: ['api_calendar_assembl_230', '/jaxrs/calendar_assemble_control/event'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar_assemble_control/event"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_calendar_assembl_230_q, (v) => { api_calendar_assembl_230_data.value = v ?? []; });


const api_calendar_assembl_259_data = ref<any[]>([]);
const { data: api_calendar_assembl_259_q } = useQuery({queryKey: ['api_calendar_assembl_259', '/jaxrs/calendar_assemble_control/message'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar_assemble_control/message"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_calendar_assembl_259_q, (v) => { api_calendar_assembl_259_data.value = v ?? []; });
const api_calendar_assembl_106_data = ref<any[]>([]);
const { data: api_calendar_assembl_106_q } = useQuery({queryKey: ['api_calendar_assembl_106', '/jaxrs/calendar_assemble_control/event/some-id'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar_assemble_control/event/some-id"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_calendar_assembl_106_q, (v) => { api_calendar_assembl_106_data.value = v ?? []; });
const api_entity_calendar__506_data = ref<any[]>([]);
const { data: api_entity_calendar__506_q } = useQuery({queryKey: ['api_entity_calendar__506', '/jaxrs/calendar/core/entity/calendar/list/my'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar/core/entity/calendar/list/my"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_entity_calendar__506_q, (v) => { api_entity_calendar__506_data.value = v ?? []; });
const api_calendar_assembl_814_data = ref<any[]>([]);
const { data: api_calendar_assembl_814_q } = useQuery({queryKey: ['api_calendar_assembl_814', '/jaxrs/calendar_assemble_control/calendar/some-id'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar_assemble_control/calendar/some-id"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_calendar_assembl_814_q, (v) => { api_calendar_assembl_814_data.value = v ?? []; });
const api_calendar_assembl_995_data = ref<any[]>([]);
const { data: api_calendar_assembl_995_q } = useQuery({queryKey: ['api_calendar_assembl_995', '/jaxrs/calendar_assemble_control/event/after/x'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar_assemble_control/event/after/x"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_calendar_assembl_995_q, (v) => { api_calendar_assembl_995_data.value = v ?? []; });
const api_calendar_assembl_208_data = ref<any[]>([]);
const { data: api_calendar_assembl_208_q } = useQuery({queryKey: ['api_calendar_assembl_208', '/jaxrs/calendar_assemble_control/event/manage'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar_assemble_control/event/manage"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_calendar_assembl_208_q, (v) => { api_calendar_assembl_208_data.value = v ?? []; });
const api_calendar_ismanag_522_data = ref<any[]>([]);
const { data: api_calendar_ismanag_522_q } = useQuery({queryKey: ['api_calendar_ismanag_522', '/jaxrs/calendar_assemble_control/calendar/ismanager/calendar/x'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar_assemble_control/calendar/ismanager/calendar/x"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_calendar_ismanag_522_q, (v) => { api_calendar_ismanag_522_data.value = v ?? []; });
const api_calendar_assembl_577_data = ref<any[]>([]);
const { data: api_calendar_assembl_577_q } = useQuery({queryKey: ['api_calendar_assembl_577', '/jaxrs/calendar_assemble_control/calendar/list/public'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar_assemble_control/calendar/list/public"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_calendar_assembl_577_q, (v) => { api_calendar_assembl_577_data.value = v ?? []; });
const api_calendar_assembl_25_data = ref<any[]>([]);
const { data: api_calendar_assembl_25_q } = useQuery({queryKey: ['api_calendar_assembl_25', '/jaxrs/calendar_assemble_control/calendar/follow/x'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar_assemble_control/calendar/follow/x"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_calendar_assembl_25_q, (v) => { api_calendar_assembl_25_data.value = v ?? []; });
const api_event_list_filte_71_data = ref<any[]>([]);
const { data: api_event_list_filte_71_q } = useQuery({queryKey: ['api_event_list_filte_71', '/jaxrs/calendar_assemble_control/event/list/filter/sample'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar_assemble_control/event/list/filter/sample"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_event_list_filte_71_q, (v) => { api_event_list_filte_71_data.value = v ?? []; });
const api_calendar_assembl_554_data = ref<any[]>([]);
const { data: api_calendar_assembl_554_q } = useQuery({queryKey: ['api_calendar_assembl_554', '/jaxrs/calendar_assemble_control/setting'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar_assemble_control/setting"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_calendar_assembl_554_q, (v) => { api_calendar_assembl_554_data.value = v ?? []; });
const api_calendar_assembl_74_data = ref<any[]>([]);
const { data: api_calendar_assembl_74_q } = useQuery({queryKey: ['api_calendar_assembl_74', '/jaxrs/calendar_assemble_control/setting/list/all'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar_assemble_control/setting/list/all"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_calendar_assembl_74_q, (v) => { api_calendar_assembl_74_data.value = v ?? []; });
const api_calendar_assembl_101_data = ref<any[]>([]);
const { data: api_calendar_assembl_101_q } = useQuery({queryKey: ['api_calendar_assembl_101', '/jaxrs/calendar_assemble_control/get/control/config'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar_assemble_control/get/control/config"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_calendar_assembl_101_q, (v) => { api_calendar_assembl_101_data.value = v ?? []; });
const calendar_assemble_control_test_1_ref = ref<any[]>([]);
const calendar_assemble_control_test_1_q = useQuery({
  queryKey: ['calendar_assemble_control_test_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/calendar_assemble_control/test/1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});

const api_jaxrs_calendar_a_291_data = ref<any[]>([]);
const { data: api_jaxrs_calendar_a_291_q } = useQuery({queryKey: ['api_jaxrs_calendar_a_291', '/jaxrs/calendar_assemble_control/calendar/manager/list/with/person/x'], queryFn: async () => { try { const r = await api.get("/jaxrs/calendar_assemble_control/calendar/manager/list/with/person/x"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_calendar_a_291_q, (v) => { api_jaxrs_calendar_a_291_data.value = v ?? []; });
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
