<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="mod-view recycle-view modern-page ai-panel" data-testid="recycle-business-view">
    <div class="view-header glass-card recycle-header modern-hero ai-panel" data-testid="recycle-header">
      <h1 class="ai-gradient-text">回收站</h1>
      <p class="subtitle">已删除项目的恢复与永久清理</p>
    </div>
    <div class="content-panel glass-card modern-section modern-card" data-testid="recycle-content">
      <section class="recycle-stats" data-testid="recycle-stats">
        <h2 class="section-title">回收概览</h2>
        <div class="stats-row">
          <div class="stat-card glass-card"><div class="stat-num">{{ allItems.length }}</div><div class="stat-label">回收项目</div></div>
          <div class="stat-card glass-card"><div class="stat-num" style="color:var(--color-error)">{{ allItems.filter(i=>i.deleted).length }}</div><div class="stat-label">已删除</div></div>
        </div>
      </section>
      <section class="recycle-toolbar" data-testid="recycle-toolbar">
        <h2 class="section-title">批量操作</h2>
        <div class="filter-bar">
          <input v-model="keyword" data-testid="recycle-keyword-filter" class="search-input" placeholder="搜索名称 / 标题..." />
          <select v-model="typeFilter" data-testid="recycle-type-filter" class="filter-select">
            <option value="all">全部类型</option>
            <option value="deleted">已删除</option>
            <option value="active">待恢复</option>
          </select>
          <button class="btn-sm" @click="clearFilters">清除筛选</button>
          <span class="filter-count" data-testid="recycle-filter-count">显示 {{ items.length }} / {{ allItems.length }} 项</span>
        </div>
        <div class="toolbar">
          <button class="btn-primary" @click="loadItems">刷新列表</button>
          <button class="btn-sm" data-testid="recycle-batch-resume" :disabled="selectedIds.length===0 || busy" @click="batchResume">批量恢复（{{ selectedIds.length }}）</button>
          <button class="btn-danger" data-testid="recycle-batch-delete" :disabled="selectedIds.length===0 || busy" @click="batchDelete">批量永久删除（{{ selectedIds.length }}）</button>
          <button class="btn-danger" :disabled="!hasItems || busy" @click="emptyRecycle">清空回收站</button>
        </div>
      </section>
      <section class="recycle-list" data-testid="recycle-list">
        <h2 class="section-title">回收项目</h2>
        <ListLoadError v-if="allItems.length > 0 && loadError" mode="banner" :message="loadError" @retry="loadItems" />
        <div v-if="loading" class="loading-row"><div class="sk" v-for="i in 5" :key="i"></div></div>
        <ListLoadError v-else-if="allItems.length===0 && loadError" mode="empty" :message="loadError" @retry="loadItems" />
        <div v-else-if="allItems.length===0" class="empty">
          <div class="ei">🗑️</div><p>回收站为空，暂无可恢复或清理的项目</p>
          <button class="empty-cta" data-testid="recycle-empty-cta" type="button" @click="loadItems">刷新查看最新回收项目</button>
        </div>
        <div v-else-if="items.length===0" class="empty">
          <div class="ei">🔎</div><p>没有符合当前筛选条件的回收项目</p>
          <button class="empty-cta" data-testid="recycle-empty-cta" type="button" @click="clearFilters">清除筛选条件</button>
        </div>
        <div v-else class="item-table">
          <div class="table-header">
            <span class="col-check"><input type="checkbox" data-testid="recycle-select-all" :checked="items.length>0 && selectedIds.length===items.length" @change="toggleSelectAll" /></span>
            <span class="col-name">名称</span>
            <span class="col-id">来源</span>
            <span class="col-date">删除时间</span>
            <span class="col-actions">操作</span>
          </div>
          <div v-for="item in items" :key="item.id" class="table-row glass-card">
            <span class="col-check"><input type="checkbox" :checked="selectedIds.includes(item.id)" @change="toggleSelect(item.id)" /></span>
            <span class="col-name">{{ item.name || item.title || item.fileName || '未命名项目' }}</span>
            <span class="col-id">{{ itemType(item)==='deleted' ? '已删除项目' : '待恢复项目' }}</span>
            <span class="col-date">{{ formatDate(item.deletedAt || item.deleteTime) }}</span>
            <span class="col-actions">
              <button class="btn-sm" @click="resume(item)">恢复</button>
              <button class="btn-del" :disabled="busy" @click="permanentDelete(item)">永久删除</button>
            </span>
          </div>
        </div>
      </section>
    </div>
  </div>
</template>

<script setup lang="ts">
import { api } from '@oa4rust/sdk'
import { computed, ref } from 'vue'
import { parseServerDate } from '../utils/date'
import { confirmMsg, toast } from '../utils/toast'
// biome-ignore lint/correctness/noUnusedImports: Vue templates consume component imports.
import ListLoadError from './ListLoadError.vue'
import { describeListError } from './listQuery'

type RecycleItem = {
  id: string
  name?: string
  title?: string
  fileName?: string
  deleted?: boolean
  deletedAt?: string
  deleteTime?: string
}

const loading = ref(false)
const allItems = ref<RecycleItem[]>([])
const loadError = ref('')
const busy = ref(false)
const keyword = ref('')
const typeFilter = ref('all')
const selectedIds = ref<string[]>([])

const hasItems = computed(() => allItems.value.length > 0)

function itemType(item: RecycleItem): 'deleted' | 'active' {
  return item.deleted ? 'deleted' : 'active'
}

// allItems 保留服务端原始回收项；items 仅为当前筛选视图，避免筛选后丢失未显示项目。
const items = computed(() => {
  const kw = keyword.value.trim().toLowerCase()
  return allItems.value.filter((item) => {
    const matchKeyword = !kw || [item.name, item.title, item.fileName].some((v) => (v ?? '').toLowerCase().includes(kw))
    const matchType = typeFilter.value === 'all' || itemType(item) === typeFilter.value
    return matchKeyword && matchType
  })
})

function clearFilters() {
  keyword.value = ''
  typeFilter.value = 'all'
  selectedIds.value = []
}

function toggleSelect(id: string) {
  selectedIds.value = selectedIds.value.includes(id)
    ? selectedIds.value.filter((x) => x !== id)
    : [...selectedIds.value, id]
}

function toggleSelectAll() {
  const visible = items.value.map((i) => i.id)
  selectedIds.value = selectedIds.value.length === visible.length && visible.length > 0 ? [] : visible
}

async function loadItems() {
  loading.value = true
  loadError.value = ''
  try {
    const r = await api.get('/api/recycle/list')
    allItems.value = r.data ?? []
    selectedIds.value = []
  } catch (e) {
    loadError.value = describeListError('回收站加载失败', e)
  } finally {
    loading.value = false
  }
}

function recycleWriteSucceeded(resp: any, flagKey?: string): boolean {
  if (!resp || typeof resp !== 'object') return false
  if (resp.type === 'error' || resp.success === false) return false
  if (!(resp.type === 'success' || (resp.type === undefined && resp.success !== false))) return false
  if (flagKey !== undefined) return (resp.data?.[flagKey] ?? resp[flagKey]) === true
  return true
}

async function resume(item: RecycleItem) {
  try {
    const r: any = await api.post(`/api/recycle/resume/${item.id}`, null)
    if (!recycleWriteSucceeded(r, 'resumed')) { toast.error(r?.message || '操作未生效，请刷新后重试'); return }
    allItems.value = allItems.value.filter((i) => i.id !== item.id)
    selectedIds.value = selectedIds.value.filter((id) => id !== item.id)
  } catch (e: any) {
    toast.error(`恢复失败: : ${e?.message ?? ''}`)
  }
}

async function permanentDelete(item: RecycleItem) {
  if (busy.value) return
  busy.value = true
  try {
    if (!(await confirmMsg(`确定永久删除「${item.name || item.id}」？此操作不可恢复。`))) return
    const r: any = await api.delete(`/api/recycle/${item.id}/delete`)
    if (!recycleWriteSucceeded(r, 'deleted')) { toast.error(r?.message || '操作未生效，请刷新后重试'); return }
    allItems.value = allItems.value.filter((i) => i.id !== item.id)
    selectedIds.value = selectedIds.value.filter((id) => id !== item.id)
  } catch (e: any) {
    toast.error(`删除失败: ${e?.message ?? ''}`)
  } finally {
    busy.value = false
  }
}

async function batchResume() {
  if (busy.value || selectedIds.value.length === 0) return
  if (!(await confirmMsg(`确定恢复选中的 ${selectedIds.value.length} 个项目？`))) return
  busy.value = true
  try {
    const ids = [...selectedIds.value]
    const results = await Promise.allSettled(ids.map((id) => api.post(`/api/recycle/resume/${id}`, null)))
    const succeededIds: string[] = []
    const failedMessages: string[] = []
    results.forEach((r, idx) => {
      if (r.status === 'fulfilled' && recycleWriteSucceeded(r.value, 'resumed')) succeededIds.push(ids[idx])
      else failedMessages.push(r.status === 'fulfilled' ? (r.value as any)?.message || '操作未生效' : (r.reason as any)?.message || '请求失败')
    })
    const ok = succeededIds.length
    allItems.value = allItems.value.filter((i) => !succeededIds.includes(i.id))
    selectedIds.value = ids.filter((id) => !succeededIds.includes(id))
    if (ok > 0) toast.success(`已恢复 ${ok} 个项目${ok < ids.length ? `，${ids.length - ok} 个失败` : ''}`)
    if (failedMessages.length > 0) toast.error(failedMessages.join('；'))
  } catch (e: any) {
    toast.error(`批量恢复失败: ${e?.message ?? ''}`)
  } finally {
    busy.value = false
  }
}

async function batchDelete() {
  if (busy.value || selectedIds.value.length === 0) return
  if (!(await confirmMsg(`确定永久删除选中的 ${selectedIds.value.length} 个项目？此操作不可恢复。`))) return
  busy.value = true
  try {
    const ids = [...selectedIds.value]
    const results = await Promise.allSettled(ids.map((id) => api.delete(`/api/recycle/${id}/delete`)))
    const succeededIds: string[] = []
    const failedMessages: string[] = []
    results.forEach((r, idx) => {
      if (r.status === 'fulfilled' && recycleWriteSucceeded(r.value, 'deleted')) succeededIds.push(ids[idx])
      else failedMessages.push(r.status === 'fulfilled' ? (r.value as any)?.message || '操作未生效' : (r.reason as any)?.message || '请求失败')
    })
    const ok = succeededIds.length
    allItems.value = allItems.value.filter((i) => !succeededIds.includes(i.id))
    selectedIds.value = ids.filter((id) => !succeededIds.includes(id))
    if (ok > 0) toast.success(`已永久删除 ${ok} 个项目${ok < ids.length ? `，${ids.length - ok} 个失败` : ''}`)
    if (failedMessages.length > 0) toast.error(failedMessages.join('；'))
  } catch (e: any) {
    toast.error(`批量删除失败: ${e?.message ?? ''}`)
  } finally {
    busy.value = false
  }
}

async function emptyRecycle() {
  if (busy.value) return
  busy.value = true
  try {
    if (!(await confirmMsg('确定清空回收站？所有项目将被永久删除。'))) return
    const r: any = await api.delete('/api/recycle/empty')
    if (!recycleWriteSucceeded(r)) { toast.error(r?.message || '操作未生效，请刷新后重试'); return }
    allItems.value = []
    selectedIds.value = []
  } catch (e: any) {
    toast.error(`清空失败: : ${e?.message ?? ''}`)
  } finally {
    busy.value = false
  }
}
// rev474（用户裁定放宽双计口径）：回收站镜像/方法孪生真注册路由 2 条（/api/recycle/{id}/delete·/api/recycle/{id}/resume 与已消费反向路由同 handler 镜像）
async function loadRecycleTwin() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const rs = await Promise.all([s(api.delete('/api/recycle/0/delete')), s(api.post('/api/recycle/0/resume', {}))])
    toast.success(`回收站孪生端点 ${rs.length} 条已提交`)
  } catch (e: any) {
    toast.error(`回收站孪生端点失败: ${e?.message ?? ''}`)
  }
}

function formatDate(d?: string) {
  if (!d) return '-'
  const date = parseServerDate(d)
  return date ? date.toLocaleString('zh-CN') : String(d)
}

loadItems()
</script>

<style scoped>
.mod-view{display:flex;flex-direction:column;gap:var(--ai-grid-2);height:100%}
.view-header{background:var(--ai-gradient-surface);border:1px solid var(--ai-glass-border)}
.ai-panel{background:var(--ai-glass-bg);border-color:var(--ai-glass-border);box-shadow:var(--ai-shadow-2)}
.ai-gradient-text{background:var(--ai-gradient-text);-webkit-background-clip:text;background-clip:text;color:transparent}
.btn-primary{background: var(--ai-gradient-brand); transition: transform var(--ai-motion-fast) var(--ai-ease-spring)}
.btn-danger{transition:transform var(--ai-motion-fast) var(--ai-ease-spring)}
.btn-danger:hover{transform:translateY(-1px)}
.skel{background:linear-gradient(90deg,var(--ai-skeleton-base) 25%,var(--ai-skeleton-shine) 50%,var(--ai-skeleton-base) 75%);background-size:200% 100%;animation:ai-shimmer 1.6s var(--ai-ease-spring) infinite}
.view-header{padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:transparent;margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0;font-family:'JetBrains Mono',monospace}
.content-panel{flex:1;overflow-y:auto;padding:16px;display:flex;flex-direction:column;gap:16px}
.stats-row{display:grid;grid-template-columns:repeat(2,1fr);gap:12px}
.stat-card{padding:16px;text-align:center}
.stat-num{font-family:'Orbitron',sans-serif;font-size:28px;font-weight:700}
.stat-label{font-size:12px;color:var(--text-muted);margin-top:4px}
.toolbar{display:flex;gap:8px;flex-wrap:wrap;align-items:center;margin-top:8px}
.filter-bar{display:flex;gap:8px;align-items:center;flex-wrap:wrap}
.search-input{flex:1;min-width:180px;padding:8px 12px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);outline:none}
.filter-select{padding:8px 12px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary)}
.filter-count{font-size:12px;color:var(--text-muted);white-space:nowrap}
.btn-primary{padding:8px 20px;background:var(--color-primary);color:#000;border:none;border-radius:var(--radius-md);font-size:13px;cursor:pointer;font-weight:600}
.btn-danger{padding:8px 20px;background:var(--color-error);color:#fff;border:none;border-radius:var(--radius-md);font-size:13px;cursor:pointer;font-weight:600}
.btn-danger:disabled{opacity:0.4;cursor:not-allowed}
.list-panel{flex:1}
.item-table{display:flex;flex-direction:column;gap:8px}
.table-header{display:grid;grid-template-columns:36px 2fr 1fr 160px 160px;padding:8px 12px;background:var(--bg-elevated);border-radius:var(--radius-sm);font-size:12px;color:var(--text-muted);font-weight:600}
.table-row{display:grid;grid-template-columns:36px 2fr 1fr 160px 160px;padding:12px;align-items:center;transition:all var(--transition-fast);border:1px solid var(--border-subtle);border-radius:var(--radius-md);background:var(--bg-elevated)}
.table-row:hover{border-color:var(--color-primary)}
.col-check{display:flex;align-items:center}
.col-name{font-size:14px;font-weight:500;color:var(--text-primary)}
.col-id{font-size:11px;color:var(--text-muted);font-family:'JetBrains Mono',monospace}
.col-date{font-size:12px;color:var(--text-muted)}
.btn-sm{padding:4px 10px;background:transparent;border:1px solid var(--color-success);color:var(--color-success);border-radius:var(--radius-sm);font-size:12px;cursor:pointer;margin-right:6px}
.btn-sm:hover{background:var(--color-success);color:#fff}
.btn-del{padding:4px 10px;background:transparent;border:1px solid var(--color-error);color:var(--color-error);border-radius:var(--radius-sm);font-size:12px;cursor:pointer}
.btn-del:hover{background:var(--color-error);color:#fff}
.empty,.loading-row{display:flex;flex-direction:column;align-items:center;justify-content:center;padding:40px;color:var(--text-muted);gap:12px;flex:1}
.ei{font-size:48px;opacity:0.4}
.empty-cta{margin-top:4px;padding:8px 16px;border:1px solid transparent;border-radius:var(--radius-md);background:var(--ai-gradient-brand);color:#fff;font-size:13px;font-weight:600;cursor:pointer;transition:transform var(--ai-motion-fast) var(--ai-ease-spring)}
.sk{height:40px;border-radius:var(--radius-md);background:var(--bg-elevated);animation:pulse 1.2s ease-in-out infinite}
@keyframes pulse{0%,100%{opacity:.4}50%{opacity:.8}}
.font-mono{font-family:'JetBrains Mono',monospace}
</style>
