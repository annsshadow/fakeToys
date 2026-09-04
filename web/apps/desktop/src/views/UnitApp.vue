<template>
  <div class="mod-view">
    <div class="view-header glass-card">
      <h1>单元管理</h1>
      <p class="subtitle">/jaxrs/unit/* — 组织单元与身份映射</p>
    </div>
    <div class="content-panel glass-card">
      <div class="toolbar">
        <input v-model="keyword" placeholder="搜索单元..." class="search-input" @keyup.enter="doSearch" />
        <button class="btn-primary" @click="doSearch">搜索</button>
        <button class="btn-primary" @click="loadUnits">刷新全部</button>
      </div>
      <div class="list-panel">
        <div v-if="loading" class="loading-row"><div class="sk" v-for="i in 6" :key="i"></div></div>
        <div v-else-if="units.length===0" class="empty"><div class="ei">🏗️</div><p>暂无单元数据</p></div>
        <div v-else class="item-grid">
          <div v-for="u in units" :key="u.id" class="item-card glass-card">
            <div class="ic">🏗️</div>
            <div class="ib">
              <div class="it">{{ u.name || u.title || '未命名单元' }}</div>
              <div class="im">flag: {{ u.flag || u.unitFlag || u.id }}</div>
              <div class="meta">{{ u.desc || u.description || '' }}</div>
            </div>
            <button class="btn-sm" @click="checkUnit(u)">验证</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { api } from '@oa4rust/sdk'

type UnitItem = { id: string; name?: string; title?: string; flag?: string; unitFlag?: string; desc?: string; description?: string }

const keyword = ref('')
const loading = ref(false)
const units = ref<UnitItem[]>([])

async function doSearch() {
  loading.value = true
  try {
    // Try search by keyword
    const r = await api.get('/jaxrs/unit/list')
    units.value = r.data ?? []
  } catch { units.value = [] } finally { loading.value = false }
}

async function checkUnit(u: UnitItem) {
  try {
    await api.get(`/jaxrs/unit/check/${u.flag || u.id}`)
  } catch (e: any) { alert('验证失败: ' + (e?.message ?? '')) }
}

loadUnits()
</script>

<style scoped>
.mod-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0;font-family:'JetBrains Mono',monospace}
.content-panel{flex:1;overflow-y:auto;padding:16px;display:flex;flex-direction:column;gap:16px}
.toolbar{display:flex;gap:8px}
.search-input{flex:1;background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-md);color:var(--text-primary);padding:8px 12px;font-size:14px}
.search-input:focus{outline:none;border-color:var(--color-primary)}
.btn-primary{padding:8px 20px;background:var(--color-primary);color:#000;border:none;border-radius:var(--radius-md);font-size:13px;cursor:pointer;font-weight:600}
.list-panel{flex:1}
.item-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(220px,1fr));gap:12px}
.item-card{display:flex;align-items:center;gap:12px;padding:14px;transition:all var(--transition-fast);border:1px solid var(--border-subtle);border-radius:var(--radius-md);background:var(--bg-elevated)}
.item-card:hover{border-color:var(--color-primary);transform:translateX(4px);box-shadow:var(--shadow-glow)}
.ic{font-size:28px}
.ib{flex:1;min-width:0}
.it{font-size:14px;font-weight:600;color:var(--text-primary)}
.im{font-size:12px;color:var(--text-muted);margin-top:2px}
.meta{font-size:11px;color:var(--text-muted);margin-top:2px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.btn-sm{padding:4px 10px;background:transparent;border:1px solid var(--border-subtle);color:var(--text-secondary);border-radius:var(--radius-sm);font-size:12px;cursor:pointer}
.btn-sm:hover{border-color:var(--color-primary);color:var(--color-primary)}
.empty,.loading-row{display:flex;flex-direction:column;align-items:center;justify-content:center;padding:40px;color:var(--text-muted);gap:12px;flex:1}
.ei{font-size:48px;opacity:0.4}
.sk{height:40px;border-radius:var(--radius-md);background:var(--bg-elevated);animation:pulse 1.2s ease-in-out infinite}
@keyframes pulse{0%,100%{opacity:.4}50%{opacity:.8}}
</style>
