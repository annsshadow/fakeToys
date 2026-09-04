<template>
  <div class="work-view">
    <div class="view-header glass-card">
      <h1>{{ title }}</h1>
      <p class="subtitle">{{ subtitle }}</p>
    </div>

    <!-- 统计卡片 -->
    <div class="stats-row">
      <div v-for="s in stats" :key="s.label" class="stat-card glass-card">
        <div class="stat-num" :style="{ color: s.color }">{{ s.value }}</div>
        <div class="stat-label">{{ s.label }}</div>
      </div>
    </div>

    <!-- 主内容区 -->
    <div class="content-panel glass-card">
      <div v-if="loading" class="loading-state">
        <div v-for="i in 5" :key="i" class="skeleton-row"></div>
      </div>
      <div v-else-if="items.length === 0" class="empty-state">
        <div class="empty-icon">{{ emptyIcon }}</div>
        <p>{{ emptyText }}</p>
      </div>
      <div v-else class="item-list">
        <div v-for="item in items" :key="item.id" class="item-card">
          <div class="item-icon">{{ item.icon }}</div>
          <div class="item-body">
            <div class="item-title">{{ item.title }}</div>
            <div class="item-meta">{{ item.meta }}</div>
          </div>
          <div class="item-actions">
            <button class="btn-sm" @click="handleAction(item)">处理</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';

defineProps<{
  title: string;
  subtitle: string;
  emptyIcon?: string;
  emptyText?: string;
}>();

const loading = ref(false);
const items = ref<Array<{ id: string; icon: string; title: string; meta: string }>>([]);

const stats = ref([
  { label: '待处理', value: 0, color: 'var(--color-warning)' },
  { label: '进行中', value: 0, color: 'var(--color-info)' },
  { label: '已完成', value: 0, color: 'var(--color-success)' },
  { label: '已超时', value: 0, color: 'var(--color-error)' },
]);

function handleAction(_item: unknown): void {
  // Override in child component
}
</script>

<style scoped>
.work-view { display: flex; flex-direction: column; gap: 16px; height: 100%; }

.view-header { padding: 20px 24px; }
.view-header h1 {
  font-family: 'Orbitron', sans-serif; font-size: 20px; color: var(--color-primary);
  margin: 0 0 4px; text-shadow: 0 0 15px var(--color-primary-glow);
}
.subtitle { font-size: 13px; color: var(--text-muted); margin: 0; }

.stats-row { display: grid; grid-template-columns: repeat(4, 1fr); gap: 12px; }
.stat-card { padding: 16px; text-align: center; }
.stat-num {
  font-family: 'Orbitron', sans-serif; font-size: 28px; font-weight: 700; margin-bottom: 4px;
}
.stat-label { font-size: 12px; color: var(--text-muted); }

.content-panel { flex: 1; overflow-y: auto; padding: 16px; }

.item-list { display: flex; flex-direction: column; gap: 8px; }
.item-card {
  display: flex; align-items: center; gap: 12px; padding: 12px 16px;
  background: var(--bg-elevated); border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md); transition: all var(--transition-fast);
}
.item-card:hover { border-color: var(--border-active); transform: translateX(4px); }
.item-icon { font-size: 20px; width: 32px; text-align: center; }
.item-body { flex: 1; min-width: 0; }
.item-title { font-size: 14px; color: var(--text-primary); font-weight: 500; }
.item-meta { font-size: 12px; color: var(--text-muted); margin-top: 2px; }
.btn-sm {
  padding: 4px 12px; border-radius: var(--radius-sm); border: 1px solid var(--color-primary);
  background: var(--color-primary-soft); color: var(--color-primary); cursor: pointer;
  font-size: 12px; transition: all var(--transition-fast);
}
.btn-sm:hover { background: var(--color-primary); color: var(--text-inverse); }

.empty-state { display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 60px; color: var(--text-muted); gap: 12px; }
.empty-icon { font-size: 48px; opacity: 0.4; }
.loading-state { display: flex; flex-direction: column; gap: 8px; padding: 16px; }
.skeleton-row { height: 48px; border-radius: var(--radius-md); margin-bottom: 8px; }

@media (max-width: 768px) {
  .stats-row { grid-template-columns: repeat(2, 1fr); }
}
</style>
