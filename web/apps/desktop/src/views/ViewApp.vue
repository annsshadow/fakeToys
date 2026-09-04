<template>
  <div class="mod-view">
    <div class="view-header glass-card">
      <h1>视图管理</h1>
      <p class="subtitle">/jaxrs/view/*</p>
    </div>
    <div class="content-panel glass-card">
      <div class="toolbar">
        <button class="btn-primary" @click="loadViews">刷新</button>
      </div>
      <div class="list-panel">
        <div v-if="loading" class="loading-row"><div class="sk" v-for="i in 5" :key="i"></div></div>
        <div v-else-if="views.length===0" class="empty"><div class="ei">📊</div><p>暂无视图数据</p></div>
        <div v-else class="item-table">
          <div class="table-header">
            <span class="col-name">视图名称</span>
            <span class="col-id">ID</span>
            <span class="col-actions">操作</span>
          </div>
          <div v-for="v in views" :key="v.id" class="table-row glass-card">
            <span class="col-name">{{ v.name || v.viewName || v.title || '未命名' }}</span>
            <span class="col-id font-mono">{{ v.id?.slice(0,8) }}...</span>
            <span class="col-actions">
              <button class="btn-sm" @click="viewData(v)">查看数据</button>
            </span>
          </div>
        </div>
      </div>
    </div>
    <!-- Data modal -->
    <div v-if="activeView" class="modal-overlay" @click.self="activeView=null">
      <div class="modal glass-card">
        <div class="modal-header">
          <h3>{{ activeView.name || activeView.id }}</h3>
          <button class="btn-close" @click="activeView=null">✕</button>
        </div>
        <div v-if="dataLoading" class="loading-row"><div class="sk" v-for="i in 3" :key="i"></div></div>
        <div v-else class="result-table-wrap">
          <table v-if="dataResult.length > 0" class="result-table">
            <thead><tr><th v-for="k in cols" :key="k">{{ k }}</th></tr></thead>
            <tbody>
              <tr v-for="(row, ri) in dataResult" :key="ri">
                <td v-for="k in cols" :key="k">{{ row[k] ?? '-' }}</td>
              </tr>
            </tbody>
          </table>
          <div v-else class="empty-result">无数据</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { api } from '@oa4rust/sdk'

type ViewItem = { id: string; name?: string; viewName?: string; title?: string }

const loading = ref(false)
const views = ref<ViewItem[]>([])
const activeView = ref<ViewItem | null>(null)
const dataLoading = ref(false)
const dataResult = ref<Record<string, unknown>[]>([])
const cols = ref<string[]>([])

async function loadViews() {
  loading.value = true
  try {
    const r = await api.get('/jaxrs/view/list')
    views.value = r.data ?? []
  } catch { views.value = [] } finally { loading.value = false }
}

async function viewData(v: ViewItem) {
  activeView.value = v
  dataLoading.value = true
  dataResult.value = []
  cols.value = []
  try {
    const r = await api.post(`/jaxrs/view/viewdata/${v.id}`, {})
    const list = r.data?.list ?? r.data ?? []
    dataResult.value = Array.isArray(list) ? list : []
    if (dataResult.value.length > 0) cols.value = Object.keys(dataResult.value[0])
  } catch { dataResult.value = [] } finally { dataLoading.value = false }
}

loadViews()
</script>

<style scoped>
.mod-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0;font-family:'JetBrains Mono',monospace}
.content-panel{flex:1;overflow-y:auto;padding:16px;display:flex;flex-direction:column;gap:16px}
.toolbar{display:flex;gap:8px}
.btn-primary{padding:8px 20px;background:var(--color-primary);color:#000;border:none;border-radius:var(--radius-md);font-size:13px;cursor:pointer;font-weight:600}
.list-panel{flex:1}
.item-table{display:flex;flex-direction:column;gap:8px}
.table-header{display:grid;grid-template-columns:2fr 1fr 120px;padding:8px 12px;background:var(--bg-elevated);border-radius:var(--radius-sm);font-size:12px;color:var(--text-muted);font-weight:600}
.table-row{display:grid;grid-template-columns:2fr 1fr 120px;padding:12px;align-items:center;transition:all var(--transition-fast);border:1px solid var(--border-subtle);border-radius:var(--radius-md);background:var(--bg-elevated)}
.table-row:hover{border-color:var(--color-primary)}
.col-name{font-size:14px;font-weight:500;color:var(--text-primary)}
.col-id{font-size:11px;color:var(--text-muted);font-family:'JetBrains Mono',monospace}
.btn-sm{padding:4px 10px;background:transparent;border:1px solid var(--border-subtle);color:var(--text-secondary);border-radius:var(--radius-sm);font-size:12px;cursor:pointer}
.btn-sm:hover{border-color:var(--color-primary);color:var(--color-primary)}
.empty,.loading-row{display:flex;flex-direction:column;align-items:center;justify-content:center;padding:40px;color:var(--text-muted);gap:12px;flex:1}
.ei{font-size:48px;opacity:0.4}
.sk{height:40px;border-radius:var(--radius-md);background:var(--bg-elevated);animation:pulse 1.2s ease-in-out infinite}
@keyframes pulse{0%,100%{opacity:.4}50%{opacity:.8}}
.modal-overlay{position:fixed;inset:0;background:rgba(0,0,0,.7);display:flex;align-items:center;justify-content:center;z-index:100}
.modal{background:var(--bg-surface);border:1px solid var(--border-subtle);border-radius:var(--radius-lg);padding:24px;width:800px;max-width:95vw;max-height:85vh;display:flex;flex-direction:column;overflow:hidden}
.modal-header{display:flex;align-items:center;justify-content:space-between;margin-bottom:16px;flex-shrink:0}
.modal-header h3{font-family:'Orbitron',sans-serif;color:var(--color-primary);margin:0;font-size:15px}
.btn-close{background:none;border:none;color:var(--text-muted);font-size:18px;cursor:pointer}
.btn-close:hover{color:var(--color-error)}
.result-table-wrap{flex:1;overflow:auto}
.result-table{width:100%;border-collapse:collapse;font-size:13px}
.result-table th{background:var(--bg-elevated);color:var(--color-primary);padding:8px 12px;text-align:left;font-weight:600;border:1px solid var(--border-subtle);position:sticky;top:0}
.result-table td{padding:6px 12px;border:1px solid var(--border-subtle);color:var(--text-secondary)}
.result-table tr:hover td{background:var(--color-primary-soft);color:var(--text-primary)}
.empty-result{color:var(--text-muted);text-align:center;padding:20px}
.font-mono{font-family:'JetBrains Mono',monospace}
</style>
