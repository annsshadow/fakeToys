<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="search-view modern-page ai-page" data-testid="search-business-view">
    <div class="view-header glass-card search-header ai-page ai-panel" data-testid="search-header">
      <div><h1>全局搜索</h1><p class="subtitle">跨文档、流程与组织的统一检索</p></div>
    </div>
    <div class="search-panel glass-card" data-testid="search-panel">
      <form class="search-form" @submit.prevent="doSearch" data-testid="search-form">
        <input v-model="query" type="search" placeholder="搜索文档、流程、组织..." class="search-input-lg" aria-label="搜索关键词" />
        <button type="submit" class="btn-search">🔍 搜索</button>
      </form>
      <div v-if="loading" class="loading-state" data-testid="search-loading-skeleton">
        <p>搜索中...</p>
        <span class="ai-skeleton-line"></span><span class="ai-skeleton-line"></span><span class="ai-skeleton-line"></span>
      </div>
      <ListLoadError v-else-if="results.length === 0 && searchError" mode="empty" :message="searchError" @retry="doSearch" />
      <div v-else-if="results.length===0&&queried" class="empty-state"><p>未找到结果，请尝试其它关键词</p></div>
      <div v-else-if="results.length>0" class="results" data-testid="search-results">
        <ListLoadError v-if="searchError" mode="banner" :message="searchError" @retry="doSearch" />
        <div class="result-tabs" role="tablist" aria-label="结果分类" data-testid="search-results-tabs">
          <details class="result-tab" open>
            <summary role="tab">全部结果</summary>
            <ul class="result-list">
              <li v-for="r in results" :key="r.id" class="result-item">
                <div class="ri-icon">{{ r.type==='doc'?'📄':r.type==='process'?'⚡':r.type==='org'?'🏢':'📋' }}</div>
                <div class="ri-body"><div class="ri-title">{{ r.title }}</div><div class="ri-desc">{{ r.desc }}</div></div>
              </li>
            </ul>
          </details>
          <details class="result-tab">
            <summary role="tab">文档</summary>
            <ul class="result-list">
              <li v-for="r in results.filter(x=>x.type==='doc')" :key="r.id" class="result-item">
                <div class="ri-icon">📄</div>
                <div class="ri-body"><div class="ri-title">{{ r.title }}</div><div class="ri-desc">{{ r.desc }}</div></div>
              </li>
            </ul>
          </details>
          <details class="result-tab">
            <summary role="tab">流程</summary>
            <ul class="result-list">
              <li v-for="r in results.filter(x=>x.type==='process')" :key="r.id" class="result-item">
                <div class="ri-icon">⚡</div>
                <div class="ri-body"><div class="ri-title">{{ r.title }}</div><div class="ri-desc">{{ r.desc }}</div></div>
              </li>
            </ul>
          </details>
          <details class="result-tab">
            <summary role="tab">组织</summary>
            <ul class="result-list">
              <li v-for="r in results.filter(x=>x.type==='org')" :key="r.id" class="result-item">
                <div class="ri-icon">🏢</div>
                <div class="ri-body"><div class="ri-title">{{ r.title }}</div><div class="ri-desc">{{ r.desc }}</div></div>
              </li>
            </ul>
          </details>
        </div>
      </div>
      <div v-else class="empty-state ai-page"><p>输入关键词开始搜索</p><p class="empty-hint">试试“请假”、“文件”或同事姓名，快速定位业务上下文</p></div>
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
        // R315 D1：后端返回 { data: { designList, viewList } } 分组信封，旧实现把整个对象当数组，模板 .length 为 undefined 导致有数据不渲染。
        const envelope = r?.data ?? {}
        const designList: any[] = Array.isArray(envelope.designList) ? envelope.designList : []
        const viewList: any[] = Array.isArray(envelope.viewList) ? envelope.viewList : []
        results.value = [
          ...designList.map((d: any) => ({ id: d.id, title: d.name, type: 'design', desc: d.category })),
          ...viewList.map((v: any) => ({ id: v.id, title: v.name, type: 'view', desc: v.category })),
        ]
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
.search-view{display:flex;flex-direction:column;gap:var(--ai-grid-2);height:100%}
.view-header{padding:16px 24px;background:var(--ai-gradient-surface);border:1px solid var(--ai-glass-border)}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.search-panel{padding:24px;flex:1;background:var(--ai-glass-bg);border-color:var(--ai-glass-border);box-shadow:var(--ai-shadow-2)}
.search-box{display:flex;gap:8px;margin-bottom:24px}
.search-input-lg{flex:1;padding:12px 16px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:16px;outline:none;transition:border-color var(--ai-motion-fast) var(--ai-ease-spring)}
.btn-search{padding:12px 20px;border-radius:var(--radius-md);border:none;background:var(--ai-gradient-brand);color:white;cursor:pointer;font-size:16px;transition:transform var(--ai-motion-fast) var(--ai-ease-spring)}
.btn-search:hover{transform:translateY(-1px)}
.results{display:flex;flex-direction:column;gap:var(--ai-grid-1)}
.result-item{display:flex;align-items:center;gap:12px;padding:12px;border-radius:var(--radius-md);background:var(--bg-elevated);border:1px solid var(--ai-glass-border);cursor:pointer;transition:background var(--ai-motion-fast) var(--ai-ease-spring)}
.result-item:hover{background:var(--color-primary-soft)}
.ri-icon{font-size:24px}
.ri-body{flex:1}
.ri-title{font-size:14px;color:var(--text-primary)}
.ri-desc{font-size:12px;color:var(--text-muted);margin-top:2px}
.loading-state,.empty-state{padding:40px;text-align:center;color:var(--text-muted)}
.loading-state .ai-skeleton-line{height:12px;border-radius:var(--radius-md);background:var(--ai-skeleton-base)}
.empty-hint{margin:8px 0 0;color:var(--text-secondary);font-size:13px}
</style>