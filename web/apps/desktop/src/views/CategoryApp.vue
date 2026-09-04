<template>
  <div class="mod-view">
    <div class="view-header glass-card">
      <h1>分类管理</h1>
      <p class="subtitle">接入 /jaxrs/category/*</p>
    </div>
    <div class="content-panel glass-card">
      <div class="stats-row">
        <div v-for="s in stats" :key="s.label" class="stat-card glass-card">
          <div class="stat-num" :style="{color:s.color}">{{s.value}}</div>
          <div class="stat-label">{{s.label}}</div>
        </div>
      </div>
      <div class="list-panel">
        <div v-if="loading" class="loading"><div class="sk" v-for="i in 6" :key="i"></div></div>
        <div v-else-if="items.length===0" class="empty"><div class="ei"></div><p>暂无分类管理数据</p></div>
        <div v-else class="item-grid">
          <div v-for="item in items" :key="item.id" class="item-card">
            <div class="ic"></div>
            <div class="ib">
              <div class="it">{{item.name||item.title||'未命名'}}</div>
              <div class="im">{{item.desc||item.content||''}}</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { ref } from 'vue'
const loading = ref(false)
const items = ref([])
const stats = ref([
  {label:'总计',value:0,color:'var(--color-primary)'},
  {label:'启用',value:0,color:'var(--color-success)'},
  {label:'禁用',value:0,color:'var(--color-error)'},
  {label:'今日新增',value:0,color:'var(--color-accent)'},
])
const icon = ''
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
.item-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(200px,1fr));gap:12px}
.item-card{display:flex;align-items:center;gap:12px;padding:12px 16px;background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-md);transition:all var(--transition-fast)}
.item-card:hover{border-color:var(--border-active);transform:translateX(4px)}
.ic{font-size:28px}
.ib{flex:1;min-width:0}
.it{font-size:14px;font-weight:500;color:var(--text-primary)}
.im{font-size:12px;color:var(--text-muted);margin-top:2px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.empty,.loading{display:flex;flex-direction:column;align-items:center;justify-content:center;padding:40px;color:var(--text-muted);gap:12px}
.ei{font-size:48px;opacity:0.4}
.sk{height:40px;border-radius:var(--radius-md);margin-bottom:6px;background:var(--bg-elevated)}
@media(max-width:768px){.stats-row{grid-template-columns:repeat(2,1fr)}}
</style>
