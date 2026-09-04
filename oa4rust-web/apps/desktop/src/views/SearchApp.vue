<template>
  <div class="search-view">
    <div class="view-header glass-card">
      <div><h1>全局搜索</h1><p class="subtitle">全局搜索</p></div>
    </div>
    <div class="search-panel glass-card">
      <div class="search-box">
        <input v-model="query" placeholder="搜索文档、流程、组织..." class="search-input-lg" @keydown.enter="doSearch" />
        <button class="btn-search" @click="doSearch">🔍</button>
      </div>
      <div v-if="loading" class="loading-state">搜索中...</div>
      <div v-else-if="results.length===0&&queried" class="empty-state"><p>未找到结果</p></div>
      <div v-else-if="results.length>0" class="results">
        <div v-for="r in results" :key="r.id" class="result-item">
          <div class="ri-icon">{{ r.type==='doc'?'📄':r.type==='process'?'⚡':r.type==='org'?'🏢':'📋' }}</div>
          <div class="ri-body"><div class="ri-title">{{ r.title }}</div><div class="ri-desc">{{ r.desc }}</div></div>
        </div>
      </div>
      <div v-else class="empty-state"><p>输入关键词开始搜索</p></div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { ref } from 'vue'
import { api } from '@oa4rust/sdk'
const query = ref('')
const results = ref<any[]>([])
const loading = ref(false)
const queried = ref(false)
async function doSearch() {
  if (!query.value.trim()) return
  loading.value = true
  queried.value = true
  try {
    const r: any = await api.get('/jaxrs/queryview/search', { params: { keyword: query.value } })
    results.value = (r?.data ?? []) as any[]
  } finally {
    loading.value = false
  }
}
</script>
<style scoped>
.search-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.search-panel{padding:24px;flex:1}
.search-box{display:flex;gap:8px;margin-bottom:24px}
.search-input-lg{flex:1;padding:12px 16px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:16px;outline:none}
.btn-search{padding:12px 20px;border-radius:var(--radius-md);border:none;background:var(--color-primary);color:white;cursor:pointer;font-size:16px}
.results{display:flex;flex-direction:column;gap:8px}
.result-item{display:flex;align-items:center;gap:12px;padding:12px;border-radius:var(--radius-md);background:var(--bg-elevated);cursor:pointer}
.result-item:hover{background:var(--color-primary-soft)}
.ri-icon{font-size:24px}
.ri-body{flex:1}
.ri-title{font-size:14px;color:var(--text-primary)}
.ri-desc{font-size:12px;color:var(--text-muted);margin-top:2px}
.loading-state,.empty-state{padding:40px;text-align:center;color:var(--text-muted)}
</style>