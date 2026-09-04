<template>
  <div class="mod-view">
    <div class="view-header glass-card">
      <h1>回收站</h1>
      <p class="subtitle">/jaxrs/recycle/*</p>
    </div>
    <div class="content-panel glass-card">
      <div class="stats-row">
        <div class="stat-card glass-card"><div class="stat-num">{{ items.length }}</div><div class="stat-label">回收项目</div></div>
        <div class="stat-card glass-card"><div class="stat-num" style="color:var(--color-error)">{{ items.filter(i=>i.deleted).length }}</div><div class="stat-label">已删除</div></div>
      </div>
      <div class="toolbar">
        <button class="btn-primary" @click="loadItems">刷新</button>
        <button class="btn-danger" :disabled="!hasItems" @click="emptyRecycle">清空回收站</button>
      </div>
      <div class="list-panel">
        <div v-if="loading" class="loading-row"><div class="sk" v-for="i in 5" :key="i"></div></div>
        <div v-else-if="items.length===0" class="empty"><div class="ei">🗑️</div><p>回收站为空</p></div>
        <div v-else class="item-table">
          <div class="table-header">
            <span class="col-name">名称</span>
            <span class="col-id">ID</span>
            <span class="col-date">删除时间</span>
            <span class="col-actions">操作</span>
          </div>
          <div v-for="item in items" :key="item.id" class="table-row glass-card">
            <span class="col-name">{{ item.name || item.title || item.fileName || '未知项目' }}</span>
            <span class="col-id font-mono">{{ item.id?.slice(0,8) }}...</span>
            <span class="col-date">{{ formatDate(item.deletedAt || item.deleteTime) }}</span>
            <span class="col-actions">
              <button class="btn-sm" @click="resume(item)">恢复</button>
              <button class="btn-del" @click="permanentDelete(item)">永久删除</button>
            </span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { api } from '@oa4rust/sdk'

type RecycleItem = { id: string; name?: string; title?: string; fileName?: string; deleted?: boolean; deletedAt?: string; deleteTime?: string }

const loading = ref(false)
const items = ref<RecycleItem[]>([])

const hasItems = computed(() => items.value.length > 0)

async function loadItems() {
  loading.value = true
  try {
    const r = await api.get('/jaxrs/recycle/list')
    items.value = r.data ?? []
  } catch { items.value = [] } finally { loading.value = false }
}

async function resume(item: RecycleItem) {
  try {
    await api.post(`/jaxrs/recycle/resume/${item.id}`, null)
    items.value = items.value.filter(i => i.id !== item.id)
  } catch (e: any) { alert('恢复失败: ' + (e?.message ?? '')) }
}

async function permanentDelete(item: RecycleItem) {
  if (!confirm(`确定永久删除「${item.name || item.id}」？此操作不可恢复。`)) return
  try {
    await api.delete(`/jaxrs/recycle/${item.id}`)
    items.value = items.value.filter(i => i.id !== item.id)
  } catch (e: any) { alert('删除失败: ' + (e?.message ?? '')) }
}

async function emptyRecycle() {
  if (!confirm('确定清空回收站？所有项目将被永久删除。')) return
  try {
    await api.post('/jaxrs/recycle/empty', null)
    items.value = []
  } catch (e: any) { alert('清空失败: ' + (e?.message ?? '')) }
}

function formatDate(d?: string) {
  return d ? new Date(d).toLocaleString('zh-CN') : '-'
}

loadItems()
</script>

<style scoped>
.mod-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0;font-family:'JetBrains Mono',monospace}
.content-panel{flex:1;overflow-y:auto;padding:16px;display:flex;flex-direction:column;gap:16px}
.stats-row{display:grid;grid-template-columns:repeat(2,1fr);gap:12px}
.stat-card{padding:16px;text-align:center}
.stat-num{font-family:'Orbitron',sans-serif;font-size:28px;font-weight:700}
.stat-label{font-size:12px;color:var(--text-muted);margin-top:4px}
.toolbar{display:flex;gap:8px}
.btn-primary{padding:8px 20px;background:var(--color-primary);color:#000;border:none;border-radius:var(--radius-md);font-size:13px;cursor:pointer;font-weight:600}
.btn-danger{padding:8px 20px;background:var(--color-error);color:#fff;border:none;border-radius:var(--radius-md);font-size:13px;cursor:pointer;font-weight:600}
.btn-danger:disabled{opacity:0.4;cursor:not-allowed}
.list-panel{flex:1}
.item-table{display:flex;flex-direction:column;gap:8px}
.table-header{display:grid;grid-template-columns:2fr 1fr 160px 160px;padding:8px 12px;background:var(--bg-elevated);border-radius:var(--radius-sm);font-size:12px;color:var(--text-muted);font-weight:600}
.table-row{display:grid;grid-template-columns:2fr 1fr 160px 160px;padding:12px;align-items:center;transition:all var(--transition-fast);border:1px solid var(--border-subtle);border-radius:var(--radius-md);background:var(--bg-elevated)}
.table-row:hover{border-color:var(--color-primary)}
.col-name{font-size:14px;font-weight:500;color:var(--text-primary)}
.col-id{font-size:11px;color:var(--text-muted);font-family:'JetBrains Mono',monospace}
.col-date{font-size:12px;color:var(--text-muted)}
.btn-sm{padding:4px 10px;background:transparent;border:1px solid var(--color-success);color:var(--color-success);border-radius:var(--radius-sm);font-size:12px;cursor:pointer;margin-right:6px}
.btn-sm:hover{background:var(--color-success);color:#fff}
.btn-del{padding:4px 10px;background:transparent;border:1px solid var(--color-error);color:var(--color-error);border-radius:var(--radius-sm);font-size:12px;cursor:pointer}
.btn-del:hover{background:var(--color-error);color:#fff}
.empty,.loading-row{display:flex;flex-direction:column;align-items:center;justify-content:center;padding:40px;color:var(--text-muted);gap:12px;flex:1}
.ei{font-size:48px;opacity:0.4}
.sk{height:40px;border-radius:var(--radius-md);background:var(--bg-elevated);animation:pulse 1.2s ease-in-out infinite}
@keyframes pulse{0%,100%{opacity:.4}50%{opacity:.8}}
.font-mono{font-family:'JetBrains Mono',monospace}
</style>
