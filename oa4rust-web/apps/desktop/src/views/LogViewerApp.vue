<template>
  <div class="crud-view">
    <div class="view-header glass-card">
      <div>
        <h1>日志查看器</h1>
        <p class="subtitle">/jaxrs/log/list（x_cms_log，只读）</p>
      </div>
      <button class="btn-refresh" @click="loadData">🔄 刷新</button>
    </div>
    <div class="content-panel glass-card">
      <div class="toolbar">
        <input v-model="search" placeholder="搜索应用 / 操作 / 成员..." class="search-input" />
      </div>
      <div v-if="loading" class="loading-state"><div class="skel" v-for="i in 8" :key="i"></div></div>
      <div v-else-if="items.length===0" class="empty-state"><div class="empty-icon">📜</div><p>暂无日志</p></div>
      <table v-else class="data-table">
        <thead><tr><th>应用</th><th>操作</th><th>级别</th><th>成员</th><th>IP</th><th>详情</th></tr></thead>
        <tbody>
          <tr v-for="item in filtered" :key="item.id">
            <td class="mono">{{ item.app_id||'—' }}</td>
            <td>{{ item.operation_type||'—' }}</td>
            <td class="mono">{{ item.operation_level||'—' }}</td>
            <td class="mono">{{ item.person_id||'—' }}</td>
            <td class="mono">{{ item.ip_address||'—' }}</td>
            <td class="val-cell">{{ item.operation_detail||'—' }}</td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>
<script setup lang="ts">
import { api } from '@oa4rust/sdk'
import { useQuery, useQueryClient } from '@tanstack/vue-query'
import { computed, ref } from 'vue'

interface Item {
  id: string
  app_id?: string
  category_id?: string
  doc_id?: string
  person_id?: string
  operation_level?: string
  operation_type?: string
  operation_detail?: string
  ip_address?: string
}

const listEp = '/jaxrs/log/list'
const qk = ['LogViewer', 'list']

const search = ref('')
const items = ref<Item[]>([])
const loading = ref(false)
const qc = useQueryClient()

const { data } = useQuery({
  queryKey: qk,
  queryFn: async () => {
    loading.value = true
    try {
      const r = (await api.get(listEp)) as unknown as { data?: unknown }
      return Array.isArray(r?.data) ? (r.data as Item[]) : []
    } finally {
      loading.value = false
    }
  },
})
items.value = Array.isArray(data.value) ? (data.value as Item[]) : []

const filtered = computed(() =>
  search.value
    ? items.value.filter(
        (i) =>
          (i.app_id || '').toLowerCase().includes(search.value.toLowerCase()) ||
          (i.operation_type || '').toLowerCase().includes(search.value.toLowerCase()) ||
          (i.person_id || '').toLowerCase().includes(search.value.toLowerCase()),
      )
    : items.value,
)

function loadData() {
  qc.invalidateQueries({ queryKey: qk })
}
</script>
<style scoped>
.crud-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{display:flex;align-items:flex-start;justify-content:space-between;padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0}
.btn-refresh{padding:8px 12px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);cursor:pointer}
.content-panel{padding:16px}
.toolbar{display:flex;gap:8px;margin-bottom:16px}
.search-input{flex:1;padding:8px 12px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);outline:none}
.data-table{width:100%;border-collapse:collapse}
.data-table th,.data-table td{padding:10px 12px;text-align:left;border-bottom:1px solid var(--border-color)}
.data-table th{color:var(--text-muted);font-weight:600;font-size:12px;text-transform:uppercase}
.data-table tr:hover{background:var(--bg-hover)}
.val-cell{max-width:320px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.mono{font-family:'Fira Code',monospace;font-size:12px;color:var(--color-secondary)}
.loading-state,.empty-state{padding:40px;text-align:center;color:var(--text-muted)}
.empty-icon{font-size:32px;margin-bottom:8px}
.skel{height:16px;background:var(--bg-elevated);border-radius:4px;margin-bottom:8px;animation:pulse 1.5s infinite}
@keyframes pulse{0%,100%{opacity:1}50%{opacity:0.4}}
</style>
