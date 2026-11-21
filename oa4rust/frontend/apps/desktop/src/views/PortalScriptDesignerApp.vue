<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="view-shell modern-page ai-page">
    <div class="script-hero ai-page ai-panel">
      <span class="hero-kicker">PORTAL SCRIPT</span>
      <p class="empty-hint">管理门户脚本，创建后可在脚本版本历史中还原；暂无脚本时从「新建」开始。</p>
    </div>
    <div class="script-workbench ai-panel">
      <ScriptWorkbench title="门户脚本" :adapter="adapter" with-category />
    </div>
  </div>
</template>

<script setup lang="ts">
// W7：真实脚本设计器——CodeMirror 编辑器（ScriptWorkbench 内建 XScript 补全）
// + x_portal_script u2 写路径（u2_script，每次保存落版本快照）
// + scriptversion 版本历史（可还原内容）。
import { api } from '@oa4rust/sdk'
// biome-ignore lint/correctness/noUnusedImports: Vue templates consume component imports.
import ScriptWorkbench, {
  type ScriptListItem,
  type ScriptVersion,
  type ScriptWorkbenchAdapter,
} from '../components/ScriptWorkbench.vue'

function extractData(response: unknown): Record<string, unknown>[] {
  const data = (response as { data?: unknown })?.data
  if (Array.isArray(data)) return data
  if (data && typeof data === 'object' && Array.isArray((data as { data?: unknown }).data)) {
    return (data as { data: Record<string, unknown>[] }).data
  }
  return []
}

const adapter: ScriptWorkbenchAdapter = {
  async list(): Promise<ScriptListItem[]> {
    const response = await api.post('/api/portal/assemble/designer/script/list/manager', {})
    return extractData(response).map((row) => ({
      id: String(row.id ?? ''),
      name: String(row.name ?? row.flag ?? row.id ?? ''),
      category: row.category ? String(row.category) : undefined,
      updateTime: row.createTime ? String(row.createTime) : undefined,
    }))
  },
  async load(id: string) {
    const response = await api.get(`/api/portal/assemble/designer/script/${encodeURIComponent(id)}`)
    const row = ((response as { data?: Record<string, unknown> })?.data ?? {}) as Record<string, unknown>
    return {
      name: String(row.name ?? ''),
      category: row.category ? String(row.category) : '',
      code: String(row.content ?? ''),
    }
  },
  create: (data) =>
    api.post('/api/portal/assemble/designer/script', { name: data.name, category: data.category, content: data.code }),
  save: (id, data) =>
    api.put(`/api/portal/assemble/designer/script/${encodeURIComponent(id)}`, {
      name: data.name,
      category: data.category,
      content: data.code,
    }),
  remove: (id) => api.delete(`/api/portal/assemble/designer/script/${encodeURIComponent(id)}`),
  async versions(id: string): Promise<ScriptVersion[]> {
    const response = await api.get(`/api/portal/assemble/designer/scriptversion/list/script/${encodeURIComponent(id)}`)
    return extractData(response).map((row) => ({
      version: row.version === undefined ? undefined : String(row.version),
      content: row.content === undefined ? undefined : String(row.content),
      creator: row.creator ? String(row.creator) : undefined,
      createTime: row.createTime ? String(row.createTime) : undefined,
    }))
  },
}
</script>

<style scoped>
.view-shell{height:100%;min-height:0}

/* R306 C 组：AI 令牌深度消费（玻璃/渐变/栅格/骨架/动效），不触碰业务与接口。 */
.ai-page{display:flex;flex-direction:column;gap:var(--ai-grid-2)}
.ai-page .btn-primary{border:none;background:var(--ai-gradient-brand);color:#fff;transition:background var(--ai-motion-fast) var(--ai-ease-spring)}
@keyframes ai-shimmer{0%{background-position:200% 0}100%{background-position:-200% 0}}
.ai-page .script-hero{padding:var(--ai-grid-2) var(--ai-grid-3);border:1px solid var(--ai-glass-border);border-radius:var(--ai-cmd-radius);background:var(--ai-gradient-surface)}
.hero-kicker{font-size:11px;letter-spacing:.12em;background:var(--ai-gradient-text);-webkit-background-clip:text;background-clip:text;color:transparent}
.ai-page .script-workbench{flex:1;min-height:0;padding:var(--ai-grid-2);border:1px solid var(--ai-glass-border);border-radius:var(--ai-cmd-radius);background:var(--ai-glass-bg)}
.ai-skeleton-line{height:8px;border-radius:var(--radius-sm);background:var(--ai-skeleton-base);background-size:200% 100%;animation:ai-shimmer 1.6s var(--ai-ease-spring) infinite}
.empty-hint{margin-top:var(--ai-grid-1);color:var(--text-secondary);font-size:12px}
</style>
