<template>
  <div class="dashboard">
    <!-- 欢迎区 -->
    <div class="welcome-bar glass-card">
      <div class="welcome-text">
        <span class="greeting">早安，</span>
        <span class="user-name">{{ user?.name ?? '用户' }}</span>
        <span class="welcome-msg">欢迎来到 OA4Rust 工作台</span>
      </div>
      <div class="datetime">
        <div class="time">{{ currentTime }}</div>
        <div class="date">{{ currentDate }}</div>
      </div>
    </div>

    <!-- 快捷入口 -->
    <div class="quick-access">
      <div v-for="app in quickApps" :key="app.id" class="quick-card glass-card" @click="navigateTo(app.path)">
        <div class="quick-icon">{{ app.icon }}</div>
        <div class="quick-label">{{ app.label }}</div>
        <div v-if="app.badge" class="quick-badge">{{ app.badge }}</div>
      </div>
    </div>

    <!-- 实时统计 -->
    <div class="stats-grid">
      <div class="stat-card glass-card" :style="{ '--glow': s.color }" v-for="s in stats" :key="s.label">
        <div class="stat-icon">{{ s.icon }}</div>
        <div class="stat-value">{{ s.value }}</div>
        <div class="stat-label">{{ s.label }}</div>
        <div class="stat-trend" :class="s.trendDir">{{ s.trendText }}</div>
      </div>
    </div>

    <!-- 两栏布局：待办 + 动态 -->
    <div class="main-grid">
      <!-- 我的待办 -->
      <div class="panel glass-card pending-panel">
        <div class="panel-header">
          <h2>我的待办</h2>
          <router-link to="/app/process" class="panel-more">查看全部</router-link>
        </div>
        <div v-if="pendingLoading" class="loading-state"><div class="skeleton-row" v-for="i in 3" :key="i"></div></div>
        <div v-else-if="pendingItems.length === 0" class="empty-state">
          <p>🎉 暂无待办事项</p>
        </div>
        <div v-else class="item-list">
          <div v-for="item in pendingItems" :key="item.id" class="pending-item" @click="navigateTo('/app/process')">
            <div class="pending-icon">📋</div>
            <div class="pending-info">
              <div class="pending-title">{{ item.title || item.processName || '新审批' }}</div>
              <div class="pending-meta">{{ item.appName || '未知应用' }} · {{ fmtTime(item.createTime) }}</div>
            </div>
            <button class="handle-btn">处理</button>
          </div>
        </div>
      </div>

      <!-- 最近动态 -->
      <div class="panel glass-card">
        <div class="panel-header">
          <h2>最近动态</h2>
          <button class="panel-more" @click="loadRecent">刷新</button>
        </div>
        <div class="recent-list">
          <div v-for="(item, i) in recentItems" :key="i" class="recent-item">
            <span class="recent-icon">{{ item.icon }}</span>
            <span class="recent-text">{{ item.text }}</span>
            <span class="recent-time">{{ item.time }}</span>
          </div>
          <div v-if="recentItems.length === 0" class="empty-state"><p>暂无动态</p></div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { useRouter } from 'vue-router';
import { useQuery } from '@tanstack/vue-query';
import { api, useSession } from '@oa4rust/sdk';

const router = useRouter();
const session = useSession();
const user = computed(() => session.state.value?.user ?? null);
const currentTime = ref('');
const currentDate = ref('');

let timer: ReturnType<typeof setInterval>;

function updateTime(): void {
  const now = new Date();
  currentTime.value = now.toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' });
  currentDate.value = now.toLocaleDateString('zh-CN', { year: 'numeric', month: 'long', day: 'numeric', weekday: 'long' });
}

onMounted(() => { updateTime(); timer = setInterval(updateTime, 1000); });
onUnmounted(() => clearInterval(timer));

const quickApps = [
  { id: 'org', label: '组织架构', icon: '🏢', path: '/app/org' },
  { id: 'process', label: '工作审批', icon: '📋', path: '/app/process' },
  { id: 'im', label: '即时通讯', icon: '💬', path: '/app/im' },
  { id: 'calendar', label: '日历', icon: '📅', path: '/app/calendar' },
  { id: 'meeting', label: '会议', icon: '👥', path: '/app/meeting' },
  { id: 'file', label: '文件', icon: '📁', path: '/app/file' },
  { id: 'bbs', label: '论坛', icon: '💭', path: '/app/bbs' },
  { id: 'ai', label: 'AI助手', icon: '🤖', path: '/app/admin' },
];

// 实时统计数据
const { data: procCounts } = useQuery({ queryKey: ['dash', 'proc'], queryFn: () => Promise.all([
  api.get('/jaxrs/processplatform/assemble/surface/work/count/currentperson'),
  api.get('/jaxrs/processplatform/assemble/surface/work/count/completedperson'),
]) });
const { data: msgCount } = useQuery({ queryKey: ['dash', 'msg'], queryFn: () =>
  api.get('/jaxrs/message/unread/count/im') }, { staleTime: 15000, refetchInterval: 30000 });

const stats = computed(() => [
  { icon: '📋', label: '待审批', value: ((procCounts.value?.[0] as any)?.data?.count ?? 0) as number, color: 'var(--color-warning)', trendText: '↑ 较昨日', trendDir: 'up' },
  { icon: '💬', label: '未读消息', value: ((msgCount.value as any)?.data?.count ?? 0) as number, color: 'var(--color-primary)', trendText: '实时', trendDir: 'neutral' },
  { icon: '✅', label: '已办结', value: ((procCounts.value?.[1] as any)?.data?.count ?? 0) as number, color: 'var(--color-success)', trendText: '本月累计', trendDir: 'neutral' },
  { icon: '📅', label: '今日会议', value: 0, color: 'var(--color-accent)', trendText: '暂无安排', trendDir: 'neutral' },
]);

// 待办列表
const pendingLoading = ref(false);
const pendingItems = ref<Array<{ id: string; title?: string; processName?: string; appName?: string; createTime?: string }>>([]);

async function loadPending(): Promise<void> {
  pendingLoading.value = true;
  try {
    const resp = await api.post('/jaxrs/processplatform/assemble/surface/work/list/filter/manage/1/5/manage', {});
    pendingItems.value = ((resp as any)?.data ?? []).slice(0, 5);
  } catch { pendingItems.value = []; }
  finally { pendingLoading.value = false; }
}

// 最近动态
const recentItems = ref<Array<{ icon: string; text: string; time: string }>>([]);
async function loadRecent(): Promise<void> {
  await loadPending();
  // Simulate dynamic events (in production would come from /jaxrs/message/unread/count or similar)
  recentItems.value = [
    { icon: '📋', text: '您的报销申请已通过审批', time: '10 分钟前' },
    { icon: '💬', text: '张三 给您发了一条消息', time: '30 分钟前' },
    { icon: '📅', text: '明天 14:00 有部门例会', time: '1 小时前' },
    { icon: '🔔', text: '系统通知：本月考勤已生成', time: '2 小时前' },
  ];
}

function navigateTo(path: string): void { router.push(path); }
function fmtTime(ts?: string): string {
  if (!ts) return '';
  try { return new Date(ts).toLocaleString('zh-CN', { month: '2-digit', day: '2-digit', hour: '2-digit', minute: '2-digit' }); }
  catch { return String(ts); }
}

onMounted(loadPending);
</script>

<style scoped>
.dashboard { display: flex; flex-direction: column; gap: 16px; }

.welcome-bar {
  display: flex; align-items: center; justify-content: space-between;
  padding: 20px 24px;
}
.greeting { font-size: 14px; color: var(--text-muted); }
.user-name { font-size: 20px; font-weight: 700; color: var(--color-primary); margin: 0 8px; font-family: 'Orbitron', sans-serif; }
.welcome-msg { font-size: 13px; color: var(--text-secondary); }
.datetime { text-align: right; }
.time { font-family: 'Orbitron', sans-serif; font-size: 28px; color: var(--color-primary); text-shadow: 0 0 20px var(--color-primary-glow); }
.date { font-size: 12px; color: var(--text-muted); margin-top: 4px; }

.quick-access { display: grid; grid-template-columns: repeat(auto-fill, minmax(100px, 1fr)); gap: 10px; }
.quick-card {
  padding: 16px 12px; text-align: center; cursor: pointer; transition: all var(--transition-fast);
  border: 1px solid var(--border-subtle);
}
.quick-card:hover { transform: translateY(-4px); border-color: var(--color-primary); box-shadow: var(--shadow-glow); }
.quick-icon { font-size: 28px; margin-bottom: 6px; }
.quick-label { font-size: 11px; color: var(--text-secondary); }
.quick-badge {
  position: absolute; top: 6px; right: 6px; background: var(--color-error); color: white;
  font-size: 10px; font-weight: 700; padding: 1px 5px; border-radius: 8px;
}

.stats-grid { display: grid; grid-template-columns: repeat(4, 1fr); gap: 12px; }
.stat-card {
  padding: 20px; text-align: center; position: relative; overflow: hidden;
  border: 1px solid var(--border-subtle);
}
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

.main-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 16px; flex: 1; min-height: 0; }
.panel { display: flex; flex-direction: column; padding: 16px; overflow: hidden; }
.panel-header { display: flex; align-items: center; justify-content: space-between; margin-bottom: 12px; }
.panel-header h2 { font-size: 15px; color: var(--color-primary); margin: 0; font-family: 'Orbitron', sans-serif; }
.panel-more { font-size: 12px; color: var(--text-muted); text-decoration: none; cursor: pointer; transition: color var(--transition-fast); }
.panel-more:hover { color: var(--color-primary); }

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
.loading-state { display: flex; flex-direction: column; gap: 8px; padding: 16px; }
.skeleton-row { height: 40px; border-radius: var(--radius-md); background: var(--bg-elevated); }

@media (max-width: 768px) {
  .stats-grid { grid-template-columns: repeat(2, 1fr); }
  .main-grid { grid-template-columns: 1fr; }
  .welcome-bar { flex-direction: column; gap: 8px; text-align: center; }
  .datetime { text-align: center; }
}
</style>
