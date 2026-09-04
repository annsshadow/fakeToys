<template>
  <div class="dashboard">
    <h1 class="page-title">工作台</h1>

    <!-- 快捷入口卡片 -->
    <div class="quick-access">
      <div
        v-for="app in quickApps"
        :key="app.id"
        class="quick-card glass-card"
        @click="navigateTo(app.path)"
      >
        <div class="quick-icon">{{ app.icon }}</div>
        <div class="quick-label">{{ app.label }}</div>
        <div v-if="app.badge" class="quick-badge">{{ app.badge }}</div>
      </div>
    </div>

    <!-- 数据概览 -->
    <div class="stats-grid">
      <div class="stat-card glass-card">
        <div class="stat-value">{{ stats.pending }}</div>
        <div class="stat-label">待办事项</div>
      </div>
      <div class="stat-card glass-card">
        <div class="stat-value">{{ stats.messages }}</div>
        <div class="stat-label">未读消息</div>
      </div>
      <div class="stat-card glass-card">
        <div class="stat-value">{{ stats.meetings }}</div>
        <div class="stat-label">今日会议</div>
      </div>
      <div class="stat-card glass-card">
        <div class="stat-value">{{ stats.tasks }}</div>
        <div class="stat-label">进行中任务</div>
      </div>
    </div>

    <!-- 最近动态 -->
    <div class="recent-section">
      <h2 class="section-title">最近动态</h2>
      <div class="recent-list">
        <div v-for="(item, i) in recentItems" :key="i" class="recent-item">
          <span class="recent-icon">{{ item.icon }}</span>
          <span class="recent-text">{{ item.text }}</span>
          <span class="recent-time">{{ item.time }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useRouter } from 'vue-router';

const router = useRouter();

const quickApps = [
  { id: 'org', label: '组织管理', icon: '🏢', path: '/app/org' },
  { id: 'process', label: '工作审批', icon: '📋', path: '/app/process', badge: 3 },
  { id: 'im', label: '即时通讯', icon: '💬', path: '/app/im', badge: 12 },
  { id: 'calendar', label: '日历', icon: '📅', path: '/app/calendar' },
  { id: 'meeting', label: '会议', icon: '👥', path: '/app/meeting' },
  { id: 'file', label: '文件', icon: '📁', path: '/app/file' },
  { id: 'bbs', label: '论坛', icon: '💭', path: '/app/bbs' },
  { id: 'admin', label: '管理后台', icon: '🔧', path: '/app/admin' },
];

const stats = ref({
  pending: 7,
  messages: 12,
  meetings: 2,
  tasks: 5,
});

const recentItems = ref([
  { icon: '📋', text: '您有一个新的审批待处理', time: '5 分钟前' },
  { icon: '💬', text: '张三 给您发了消息', time: '15 分钟前' },
  { icon: '📅', text: '下午 3 点有部门例会', time: '1 小时前' },
  { icon: '🔔', text: '系统通知：本月考勤已生成', time: '2 小时前' },
]);

function navigateTo(path: string): void {
  router.push(path);
}
</script>

<style scoped>
.dashboard {
  max-width: 1200px;
  margin: 0 auto;
}

.page-title {
  font-family: 'Orbitron', sans-serif;
  font-size: 24px;
  font-weight: 600;
  color: var(--color-primary);
  margin-bottom: 24px;
  text-shadow: 0 0 15px var(--color-primary-glow);
}

.quick-access {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
  gap: 12px;
  margin-bottom: 32px;
}

.quick-card {
  padding: 20px 16px;
  text-align: center;
  cursor: pointer;
  transition: all var(--transition-fast);
  position: relative;
}

.quick-card:hover {
  transform: translateY(-4px);
  border-color: var(--color-primary);
  box-shadow: var(--shadow-glow);
}

.quick-icon {
  font-size: 32px;
  margin-bottom: 8px;
}

.quick-label {
  font-size: 12px;
  color: var(--text-secondary);
}

.quick-badge {
  position: absolute;
  top: 8px;
  right: 8px;
  background: var(--color-error);
  color: white;
  font-size: 10px;
  font-weight: 700;
  padding: 2px 6px;
  border-radius: 10px;
  min-width: 18px;
  text-align: center;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 16px;
  margin-bottom: 32px;
}

.stat-card {
  padding: 24px;
  text-align: center;
}

.stat-value {
  font-family: 'Orbitron', sans-serif;
  font-size: 36px;
  font-weight: 700;
  color: var(--color-primary);
  text-shadow: 0 0 20px var(--color-primary-glow);
}

.stat-label {
  font-size: 13px;
  color: var(--text-muted);
  margin-top: 8px;
}

.recent-section {
  margin-top: 8px;
}

.section-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 16px;
  padding-bottom: 8px;
  border-bottom: 1px solid var(--border-subtle);
}

.recent-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.recent-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  background: var(--bg-elevated);
  border-radius: var(--radius-md);
  border: 1px solid var(--border-subtle);
  transition: all var(--transition-fast);
}

.recent-item:hover {
  border-color: var(--border-active);
  background: var(--bg-surface);
}

.recent-icon {
  font-size: 18px;
}

.recent-text {
  flex: 1;
  font-size: 13px;
  color: var(--text-secondary);
}

.recent-time {
  font-size: 12px;
  color: var(--text-muted);
  white-space: nowrap;
}

@media (max-width: 768px) {
  .quick-access {
    grid-template-columns: repeat(4, 1fr);
    gap: 8px;
  }

  .quick-card {
    padding: 12px 8px;
  }

  .quick-icon {
    font-size: 24px;
  }

  .stats-grid {
    grid-template-columns: repeat(2, 1fr);
  }

  .stat-value {
    font-size: 28px;
  }
}
</style>
