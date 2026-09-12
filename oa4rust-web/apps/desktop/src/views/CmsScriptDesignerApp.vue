<template>
  <div class="view-shell">
    <ScriptWorkbench title="CMS 脚本" :adapter="adapter" />
  </div>
</template>

<script setup lang="ts">
// W7：真实脚本设计器——CodeMirror 编辑器（ScriptWorkbench 内建 XScript 补全）
// + /jaxrs/script u2 CRUD + 脚本内容（scriptContent）。
import { api } from '@oa4rust/sdk'
// biome-ignore lint/correctness/noUnusedImports: Vue templates consume component imports.
import ScriptWorkbench, { type ScriptListItem, type ScriptWorkbenchAdapter } from '../components/ScriptWorkbench.vue'

const base = '/jaxrs/script'

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
    const response = await api.post(`${base}/list/manager`, {})
    return extractData(response).map((row) => ({
      id: String(row.id ?? ''),
      name: String(row.name ?? row.uniqueName ?? row.id ?? ''),
      category: row.uniqueName ? String(row.uniqueName) : undefined,
    }))
  },
  async load(id: string) {
    const response = await api.get(`${base}/${encodeURIComponent(id)}`)
    const row = ((response as { data?: Record<string, unknown> })?.data ?? {}) as Record<string, unknown>
    return {
      name: String(row.name ?? ''),
      category: row.uniqueName ? String(row.uniqueName) : '',
      code: String(row.scriptContent ?? ''),
    }
  },
  create: (data) => api.post(base, { name: data.name, scriptContent: data.code }),
  save: (id, data) => api.put(`${base}/${encodeURIComponent(id)}`, { name: data.name, scriptContent: data.code }),
  remove: (id) => api.delete(`${base}/${encodeURIComponent(id)}`),
}
</script>

<style scoped>
.view-shell{height:100%;min-height:0}
</style>
