<template>
  <div class="mod-view">
    <div class="view-header glass-card">
      <h1>分类管理</h1>
      <p class="subtitle">/jaxrs/categoryinfo/*</p>
    </div>
    <div class="content-panel glass-card">
      <div class="stats-row">
        <div v-for="s in stats" :key="s.label" class="stat-card glass-card">
          <div class="stat-num" :style="{color:s.color}">{{s.value}}</div>
          <div class="stat-label">{{s.label}}</div>
        </div>
      </div>
      <div class="list-panel">
        <div v-if="loading" class="loading-row"><div class="sk" v-for="i in 6" :key="i"></div></div>
        <div v-else-if="items.length===0" class="empty"><div class="ei">📂</div><p>暂无分类数据</p></div>
        <div v-else class="item-grid">
          <div v-for="item in items" :key="item.id" class="item-card glass-card">
            <div class="ic">📁</div>
            <div class="ib">
              <div class="it">{{ item.name || item.title || item.categoryName || '未命名' }}</div>
              <div class="im">{{ item.desc || item.description || item.alias || '' }}</div>
              <div class="meta">flag: {{ item.flag || item.id }}</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { api } from '@oa4rust/sdk'

const loading = ref(false)
const items = ref<any[]>([])

const stats = computed(() => [
  { label: '总计', value: items.value.length, color: 'var(--color-primary)' },
  { label: '有效', value: items.value.length, color: 'var(--color-success)' },
  { label: '禁用', value: 0, color: 'var(--color-error)' },
  { label: '加载中', value: loading.value ? 1 : 0, color: 'var(--color-warning)' },
])

async function load() {
  loading.value = true
  try {
    const r = await api.get('/jaxrs/categoryinfo/list')
    items.value = r.data ?? []
  } catch { items.value = [] } finally { loading.value = false }
}

load()

async function api_categoryinfo() { try { await api.get('/jaxrs/categoryinfo') } catch {} }
async function api_categoryinfo_alias_alpha() { try { await api.get('/jaxrs/categoryinfo/alias/alpha') } catch {} }
async function api_categoryinfo_list_objects() { try { await api.get('/jaxrs/categoryinfo/list/objects') } catch {} }
async function api_categoryinfo_list_all() { try { await api.get('/jaxrs/categoryinfo/list/all') } catch {} }
async function api_categoryinfo_extContent() { try { await api.get('/jaxrs/categoryinfo/extContent') } catch {} }
async function api_categoryinfo_c_1_permission() { try { await api.get('/jaxrs/categoryinfo/c-1/permission') } catch {} }
async function api_categoryinfo_flag() { try { await api.get('/jaxrs/categoryinfo/flag') } catch {} }
async function api_categoryinfo_c_1_control() { try { await api.get('/jaxrs/categoryinfo/c-1/control') } catch {} }
async function api_categoryinfo_c_1_execute_projection() { try { await api.get('/jaxrs/categoryinfo/c-1/execute/projection') } catch {} }
async function api_categoryinfo_c_1() { try { await api.get('/jaxrs/categoryinfo/c-1') } catch {} }
async function api_categoryinfo_bind_c_1_view() { try { await api.get('/jaxrs/categoryinfo/bind/c-1/view') } catch {} }

</script>

<style scoped>
.mod-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0;font-family:'JetBrains Mono',monospace}
.content-panel{flex:1;overflow-y:auto;padding:16px;display:flex;flex-direction:column;gap:16px}
.stats-row{display:grid;grid-template-columns:repeat(4,1fr);gap:12px}
.stat-card{padding:16px;text-align:center}
.stat-num{font-family:'Orbitron',sans-serif;font-size:28px;font-weight:700}
.stat-label{font-size:12px;color:var(--text-muted);margin-top:4px}
.list-panel{flex:1}
.item-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(220px,1fr));gap:12px}
.item-card{display:flex;align-items:center;gap:12px;padding:14px;transition:all var(--transition-fast);border:1px solid var(--border-subtle);border-radius:var(--radius-md);background:var(--bg-elevated)}
.item-card:hover{border-color:var(--color-primary);transform:translateX(4px);box-shadow:var(--shadow-glow)}
.ic{font-size:28px}
.ib{flex:1;min-width:0}
.it{font-size:14px;font-weight:600;color:var(--text-primary)}
.im{font-size:12px;color:var(--text-muted);margin-top:2px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.meta{font-size:10px;color:var(--color-primary-deep);margin-top:4px;font-family:'JetBrains Mono',monospace}
.empty,.loading-row{display:flex;flex-direction:column;align-items:center;justify-content:center;padding:40px;color:var(--text-muted);gap:12px}
.ei{font-size:48px;opacity:0.4}
.sk{height:40px;border-radius:var(--radius-md);background:var(--bg-elevated);animation:pulse 1.2s ease-in-out infinite}
@keyframes pulse{0%,100%{opacity:.4}50%{opacity:.8}}
@media(max-width:768px){.stats-row{grid-template-columns:repeat(2,1fr)}}
</style>
