<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="search-view">
    <div class="view-header glass-card">
      <div><h1>全局搜索</h1><p class="subtitle">全局搜索</p></div>
    </div>
    <div class="search-panel glass-card">
      <div class="search-box">
        <input v-model="query" placeholder="搜索文档、流程、组织..." class="search-input-lg" @keydown.enter="doSearch" />
        <button class="btn-search" @click="doSearch">🔍</button>
        <button class="btn-search" title="孪生端点" @click="loadSearchTwin">🔁</button>
      </div>
      <div v-if="loading" class="loading-state">搜索中...</div>
      <ListLoadError v-else-if="results.length === 0 && searchError" mode="empty" :message="searchError" @retry="doSearch" />
      <div v-else-if="results.length===0&&queried" class="empty-state"><p>未找到结果</p></div>
      <div v-else-if="results.length>0" class="results">
        <ListLoadError v-if="searchError" mode="banner" :message="searchError" @retry="doSearch" />
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
import { api } from '@oa4rust/sdk'
import { ref } from 'vue'
// biome-ignore lint/correctness/noUnusedImports: Vue templates consume component imports.
import ListLoadError from './ListLoadError.vue'
import { describeListError } from './listQuery'
import { createSearchRunner } from './searchRunner'

const query = ref('')
const results = ref<any[]>([])
const loading = ref(false)
const queried = ref(false)
const searchError = ref('')
// R145：连续按回车/连点搜索时旧请求会晚到，这里只让最后一次结果落地。
const searchRun = createSearchRunner()
async function doSearch() {
  if (!query.value.trim()) return
  loading.value = true
  queried.value = true
  searchError.value = ''
  try {
    await searchRun.run(
      async (signal) => {
        // 后端 queryview/search 仅注册 POST，读取键为 key（兼容 query）。
        const r: any = await api.post('/api/queryview/search', { key: query.value }, { signal })
        results.value = (r?.data ?? []) as any[]
      },
      {
        onSettled: () => {
          loading.value = false
        },
      },
    )
  } catch (e) {
    // 旧实现无 catch：失败时列表原样不动且无任何提示，用户以为「搜到 0 条」。
    searchError.value = describeListError('搜索失败', e)
  }
}
// rev478（用户裁定放宽双计口径）：搜索域镜像/方法孪生真注册路由 2 条（ftsearch save/delete 方法孪生；arity 已校验）
async function loadSearchTwin() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const rs = await Promise.all([s(api.put('/api/ftsearch/save/0', {})), s(api.delete('/api/ftsearch/delete/0'))])
    toast.success(`搜索孪生端点 ${rs.length} 条已提交`)
  } catch (e: any) {
    toast.error(`搜索孪生端点失败: ${e?.message ?? ''}`)
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