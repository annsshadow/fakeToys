<template>
  <div class="view-shell">
    <ScriptWorkbench title="流程脚本" :adapter="adapter" with-category />
  </div>
</template>

<script setup lang="ts">
// W7：真实脚本设计器——CodeMirror 编辑器（ScriptWorkbench 内建 XScript 补全）
// + PP_E_SCRIPT 写路径（u2_script）+ PP_E_SCRIPTVERSION 版本历史。
import { api } from '@oa4rust/sdk'
// biome-ignore lint/correctness/noUnusedImports: Vue templates consume component imports.
import ScriptWorkbench, {
  type ScriptListItem,
  type ScriptVersion,
  type ScriptWorkbenchAdapter,
} from '../components/ScriptWorkbench.vue'

const base = '/jaxrs/processplatform/assemble/designer'

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
    const response = await api.get(`${base}/script/list/paging/1/50/50`)
    return extractData(response).map((row) => ({
      id: String(row.id ?? ''),
      name: String(row.name ?? row.id ?? ''),
      category: row.application ? String(row.application) : undefined,
      updateTime: row.updateTime ? String(row.updateTime) : undefined,
    }))
  },
  async load(id: string) {
    const response = await api.get(`${base}/script/${encodeURIComponent(id)}`)
    const row = ((response as { data?: Record<string, unknown> })?.data ?? {}) as Record<string, unknown>
    return {
      name: String(row.name ?? ''),
      category: row.application ? String(row.application) : '',
      code: String(row.code ?? ''),
    }
  },
  create: (data) => api.post(`${base}/script`, { name: data.name, application: data.category, code: data.code }),
  save: (id, data) => api.put(`${base}/script/${encodeURIComponent(id)}`, { name: data.name, code: data.code }),
  remove: (id) => api.delete(`${base}/script/${encodeURIComponent(id)}`),
  async versions(id: string): Promise<ScriptVersion[]> {
    const response = await api.get(`${base}/scriptversion/list/script/${encodeURIComponent(id)}`)
    return extractData(response).map((row) => ({
      version: row.version === undefined ? undefined : String(row.version),
      content: row.code === undefined ? undefined : String(row.code),
      creator: undefined,
      createTime: row.createTime ? String(row.createTime) : undefined,
    }))
  },
}
</script>

<style scoped>
.view-shell{height:100%;min-height:0}
</style>
