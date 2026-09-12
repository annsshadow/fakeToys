<template>
  <div class="view-shell">
    <ScriptWorkbench title="门户脚本" :adapter="adapter" with-category />
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

const base = '/jaxrs/portal/assemble/designer'

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
    const response = await api.post(`${base}/script/list/manager`, {})
    return extractData(response).map((row) => ({
      id: String(row.id ?? ''),
      name: String(row.name ?? row.flag ?? row.id ?? ''),
      category: row.category ? String(row.category) : undefined,
      updateTime: row.createTime ? String(row.createTime) : undefined,
    }))
  },
  async load(id: string) {
    const response = await api.get(`${base}/script/${encodeURIComponent(id)}`)
    const row = ((response as { data?: Record<string, unknown> })?.data ?? {}) as Record<string, unknown>
    return {
      name: String(row.name ?? ''),
      category: row.category ? String(row.category) : '',
      code: String(row.content ?? ''),
    }
  },
  create: (data) => api.post(`${base}/script`, { name: data.name, category: data.category, content: data.code }),
  save: (id, data) =>
    api.put(`${base}/script/${encodeURIComponent(id)}`, {
      name: data.name,
      category: data.category,
      content: data.code,
    }),
  remove: (id) => api.delete(`${base}/script/${encodeURIComponent(id)}`),
  async versions(id: string): Promise<ScriptVersion[]> {
    const response = await api.get(`${base}/scriptversion/list/script/${encodeURIComponent(id)}`)
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
</style>
