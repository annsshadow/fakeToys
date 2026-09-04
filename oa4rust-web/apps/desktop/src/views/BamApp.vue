<template>
  <div class="dash-view">
    <div class="view-header glass-card">
      <div><h1>业务活动监控</h1><p class="subtitle">/jaxrs/processplatform/assemble/bam/*</p></div>
      <button class="btn-primary" @click="refresh">🔄 刷新</button>
    </div>
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
            <td><span class="status" :class="e.status">{{ e.status }}</span></td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>
<script setup lang="ts">
import { ref } from 'vue'
import { useQuery } from '@tanstack/vue-query'
import { api } from '@oa4rust/sdk'
const loading = ref(false)
const stats = ref({ total: 0, active: 0, completed: 0, failed: 0 })
const events = ref<any[]>([])
const { data } = useQuery({
  queryKey: ['bam', 'list'],
  queryFn: async () => {
    loading.value = true
    try {
      const r = await api.get('/jaxrs/processplatform/assemble/bam/list')
      const d = (r as any)?.data ?? {}
      stats.value = d.stats ?? { total: 0, active: 0, completed: 0, failed: 0 }
      events.value = d.events ?? []
    } finally { loading.value = false }
  }
})
function refresh() { events.value = data.value ? (data.value as any).events ?? [] : [] }
function fmtTime(t?: string) { if (!t) return ''; try { return new Date(t).toLocaleString('zh-CN') } catch { return String(t) } }
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
.loading-state,.empty-state{padding:40px;text-align:center;color:var(--text-muted)}
</style>
