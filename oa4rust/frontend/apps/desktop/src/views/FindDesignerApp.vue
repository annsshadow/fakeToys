<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="editor-view modern-page ai-page">
    <div class="view-header glass-card ai-page ai-panel">
      <div class="header-actions">
        <button class="btn-secondary" @click="preview">预览</button>
        <button class="btn-primary" @click="save">💾 保存</button>
      </div>
    </div>
    <div class="editor-layout glass-card">
      <div class="editor-sidebar">
        <div class="sb-title">配置列表</div>
        <ListLoadError v-if="items.length > 0 && loadError" mode="banner" :message="loadError" @retry="reload" />
        <div v-if="loading" class="loading-sm ai-skeleton-line" aria-hidden="true"></div>
        <ListLoadError v-else-if="items.length === 0 && loadError" mode="empty" :message="loadError" @retry="reload" />
        <ul v-else class="sb-list">
          <li v-for="item in items" :key="item.id" class="sb-item" :class="{active:selected?.id===item.id}" @click="selectItem(item)">{{ item.name||item.flag||item.id }}</li>
          <li v-if="items.length===0" class="empty">暂无配置，试试「新建」一份查询配置</li>
        </ul>
        <button class="btn-sm sb-add" @click="createNew">+ 新建</button>
      </div>
      <div class="editor-main">
        <div v-if="!selected" class="empty-main"><div class="emi">⚙</div><p>选择或创建配置</p><p class="empty-hint">试试「新建」一份配置，右侧编辑器随即就绪</p></div>
        <div v-else class="editor-content">
          <div class="ec-header"><span class="ec-title">{{ selected.name||selected.flag||'未命名' }}</span></div>
          <textarea v-model="config" class="code-editor" placeholder="在此输入JSON配置..." rows="20"></textarea>
        </div>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { api } from '@oa4rust/sdk'
import { useQuery } from '@tanstack/vue-query'
import { ref, watch } from 'vue'
import { toast } from '../utils/toast'
// biome-ignore lint/correctness/noUnusedImports: Vue templates consume component imports.
import ListLoadError from './ListLoadError.vue'
import { useListError } from './listQuery'

interface Item {
  id: string
  name?: string
  flag?: string
  config?: string
}
const loading = ref(false),
  items = ref<Item[]>([]),
  selected = ref<Item | null>(null),
  config = ref('')
// 后端无 find 族；查询设计器真实列表端点为 designer/list/all。
const ep = '/api/query/assemble/designer/list/all'
const { data, isError, error, refetch } = useQuery({
  queryKey: ['FindDesigner', 'list'],
  queryFn: async () => {
    loading.value = true
    try {
      const r = await api.get(ep)
      return (r as any)?.data ?? []
    } finally {
      loading.value = false
    }
  },
})
watch(
  data,
  (rows) => {
    if (rows) items.value = rows as Item[]
  },
  { immediate: true },
)
const { loadError, reload } = useListError({
  isError,
  error,
  queryKey: ['FindDesigner', 'list'],
  label: '查找设计配置加载失败',
})
function selectItem(item: Item) {
  selected.value = item
  config.value = item.config ? `\n${item.config}` : '{}'
}
function createNew() {
  const n: Item = { id: Date.now().toString(), name: '未命名', flag: '', config: '{}' }
  items.value = [n, ...items.value]
  selectItem(n)
}
function preview() {
  toast.info(`配置预览: ${config.value}`)
}
function findDesignerWriteSucceeded(resp: any): boolean {
  if (!resp || typeof resp !== 'object') return false
  if (resp.type === 'error' || resp.success === false) return false
  if (resp.type === 'success') return true
  if (resp.type) return false
  return true
}
function save() {
  const item = selected.value
  if (!item || !config.value) return
  if (!item.id) {
    // 该视图为只读占位：后端无「find 设计」模型，新建请走查询设计器。
    toast.info('该视图为只读占位，新建请使用查询设计器')
    return
  }
  api
    // 后端 save_designer 读取 query（查询定义）；find 的 config 即其定义。
    .put(`/api/query/assemble/designer/save/${encodeURIComponent(item.id)}`, {
      ...item,
      query: config.value,
    })
    .then((r: any) => {
      if (!findDesignerWriteSucceeded(r)) {
        toast.error(r?.message || '保存未生效，请刷新后重试')
        return
      }
      toast.info('保存成功')
      refetch()
    })
    .catch((e: any) => toast.error(`保存失败: ${e?.message ?? ''}`))
}
</script>
<style scoped>
.editor-view{display:flex;flex-direction:column;gap:var(--ai-grid-2);height:100%}
.view-header{display:flex;align-items:center;justify-content:space-between;padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0}
.header-actions{display:flex;gap:8px}
.btn-secondary{padding:8px 16px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);cursor:pointer}
.btn-primary{padding:var(--ai-grid-1) var(--ai-grid-2);border-radius:var(--radius-md);border:none;background:var(--ai-gradient-brand);color:#fff;cursor:pointer;font-weight:600;transition:background var(--ai-motion-fast) var(--ai-ease-spring)}
.editor-layout{display:flex;flex:1;min-height:0;gap:16px}
.editor-sidebar{width:240px;flex-shrink:0;display:flex;flex-direction:column;padding:12px}
.sb-title{font-size:12px;color:var(--text-muted);text-transform:uppercase;margin-bottom:8px}
.sb-list{list-style:none;padding:0;margin:0;flex:1;overflow-y:auto}
.sb-item{padding:8px 12px;border-radius:var(--radius-md);cursor:pointer;font-size:13px;color:var(--text-primary);margin-bottom:2px}
.sb-item:hover,.sb-item.active{background:var(--color-primary-soft)}
.sb-add{margin-top:8px;padding:6px 12px;border-radius:var(--radius-sm);border:1px dashed var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:12px;width:100%}
.empty{padding:12px;color:var(--text-muted);text-align:center;font-size:12px}
.editor-main{flex:1;display:flex;flex-direction:column;min-height:0}
.empty-main{flex:1;display:flex;flex-direction:column;align-items:center;justify-content:center;color:var(--text-muted)}
.emi{font-size:32px;margin-bottom:8px}
.editor-content{flex:1;display:flex;flex-direction:column;gap:8px}
.ec-header{display:flex;align-items:center;justify-content:space-between}
.ec-title{font-size:14px;font-weight:600;color:var(--text-primary)}
.code-editor{flex:1;min-height:300px;padding:12px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-terminal);color:#7fdbca;font-family:'Fira Code',monospace;font-size:13px;outline:none;resize:none;tab-size:2}
.loading-sm{padding:var(--ai-grid-2);color:var(--text-muted);font-size:12px}
.ai-skeleton-line{height:16px;border-radius:var(--radius-sm);background:var(--ai-skeleton-base);background-size:200% 100%;animation:ai-shimmer 1.6s var(--ai-ease-spring) infinite}
.empty-hint{margin-top:var(--ai-grid-1);color:var(--text-secondary);font-size:12px}

/* R306 C 组：AI 令牌深度消费（玻璃/渐变/栅格/骨架/动效），不触碰业务与接口。 */
.ai-page{display:flex;flex-direction:column;gap:var(--ai-grid-2)}
.ai-page .btn-primary{border:none;background:var(--ai-gradient-brand);color:#fff;transition:background var(--ai-motion-fast) var(--ai-ease-spring)}
@keyframes ai-shimmer{0%{background-position:200% 0}100%{background-position:-200% 0}}
.ai-page .view-header{padding:var(--ai-grid-2) var(--ai-grid-3);border:1px solid var(--ai-glass-border);border-radius:var(--ai-cmd-radius);background:var(--ai-gradient-surface)}
.ai-page .editor-layout{border:1px solid var(--ai-glass-border);border-radius:var(--ai-cmd-radius);background:var(--ai-glass-bg);gap:var(--ai-grid-2)}
</style>