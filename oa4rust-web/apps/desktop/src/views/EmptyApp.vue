<template>
  <div class="mod-view">
    <div class="view-header glass-card">
      <h1>{{ title }}</h1>
      <p class="subtitle">{{ subtitle }}</p>
    </div>
    <div class="content-panel glass-card" v-if="loading">
      <div class="loading-state"><div class="sk" v-for="i in 4" :key="i"></div></div>
    </div>
    <div class="content-panel glass-card" v-else-if="items.length === 0">
      <div class="empty"><div class="ei">📂</div><p>{{ emptyText || '暂无数据' }}</p></div>
    </div>
    <div class="content-panel glass-card" v-else>
      <div class="stats-row">
        <div class="stat-card" v-for="s in stats" :key="s.label">
          <div class="stat-num" :style="{ color: s.color }">{{ s.value }}</div>
          <div class="stat-label">{{ s.label }}</div>
        </div>
      </div>
      <div class="item-list">
        <div v-for="item in items" :key="item.id" class="item-card">
          <span class="item-icon">{{ item.icon }}</span>
          <span class="item-title">{{ item.title }}</span>
          <span class="item-meta">{{ item.meta }}</span>
        </div>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { api } from '@oa4rust/sdk'
import { computed, onMounted, ref } from 'vue'

const props = defineProps<{
  title: string
  subtitle: string
  emptyIcon?: string
  emptyText?: string
  apiPath?: string
  iconPrefix?: string
}>()

const loading = ref(false)
const items = ref<Array<{ id: string; icon: string; title: string; meta: string }>>([])

const stats = computed(() => [
  { label: '总计', value: items.value.length, color: 'var(--color-primary)' },
  { label: '待处理', value: 0, color: 'var(--color-warning)' },
  { label: '已完成', value: 0, color: 'var(--color-success)' },
  { label: '已超时', value: 0, color: 'var(--color-error)' },
])

onMounted(async () => {
  if (!props.apiPath) return
  loading.value = true
  try {
    const r = await api.get(props.apiPath)
    const list = (r.data?.list ?? r.data ?? []) as Array<Record<string, unknown>>
    items.value = list.map((m: any, i: number) => ({
      id: String(m.id ?? m.flag ?? i),
      icon: props.iconPrefix ?? '📄',
      title: String(m.name ?? m.title ?? m.label ?? m.flag ?? '未命名'),
      meta: String(m.desc ?? m.description ?? ''),
    }))
  } catch {
    items.value = []
  } finally {
    loading.value = false
  }
})
</script>
<style scoped>
.mod-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{padding:20px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px}
.subtitle{font-size:13px;color:var(--text-muted);margin:0}
.content-panel{flex:1;padding:16px;overflow-y:auto}
.stats-row{display:grid;grid-template-columns:repeat(4,1fr);gap:12px;margin-bottom:16px}
.stat-card{text-align:center;padding:12px}
.stat-num{font-family:'Orbitron',sans-serif;font-size:24px;font-weight:700;margin-bottom:4px}
.stat-label{font-size:12px;color:var(--text-muted)}
.item-list{display:flex;flex-direction:column;gap:8px}
.item-card{display:flex;align-items:center;gap:12px;padding:12px 16px;background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-md)}
.item-icon{font-size:18px}
.item-title{flex:1;font-size:14px;font-weight:500;color:var(--text-primary)}
.item-meta{font-size:12px;color:var(--text-muted)}
.empty{display:flex;flex-direction:column;align-items:center;justify-content:center;padding:60px;color:var(--text-muted);gap:12px}
.ei{font-size:48px;opacity:.4}
.loading-state{display:flex;flex-direction:column;gap:8px;padding:16px}
.sk{height:48px;border-radius:var(--radius-md);background:var(--bg-elevated);animation:pulse 1.2s ease-in-out infinite}
@keyframes pulse{0%,100%{opacity:.4}50%{opacity:.8}}
@media(max-width:768px){.stats-row{grid-template-columns:repeat(2,1fr)}}
</style>
