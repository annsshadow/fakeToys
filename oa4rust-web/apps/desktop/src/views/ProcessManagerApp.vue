<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="crud-view">
    <div class="view-header glass-card">
      <div>
        <h1>流程实例管理</h1>
        <p class="subtitle">/api/processplatform/assemble/surface/process_manager/list（x_process_definition，只读）</p>
      </div>
      <button class="btn-refresh" @click="loadData">🔄 刷新</button>
    </div>
    <div class="content-panel glass-card">
      <div class="toolbar">
        <input v-model="search" placeholder="搜索流程 / 分类 / 创建人..." class="search-input" />
        <button class="btn-refresh" @click="loadRunningProcesses">⚙️ 运行中流程</button>
        <button class="btn-refresh" @click="loadManagedApps">👤 我管理的应用</button>
        <button class="btn-refresh" @click="loadAppProcesses">🗂️ 应用与流程</button>
        <button class="btn-refresh" @click="loadOrphans">🧹 孤儿元素</button>
      </div>
      <div v-if="runningProcs.length" class="rp-chips">
        <span v-for="rp in runningProcs" :key="rp.id || rp.name" class="rp-chip">{{ rp.name || rp.id }}</span>
      </div>
      <div v-if="loading" class="loading-state"><div class="skel" v-for="i in 5" :key="i"></div></div>
      <div v-else-if="items.length===0" class="empty-state"><div class="empty-icon">🧩</div><p>暂无流程定义</p></div>
      <table v-else class="data-table">
        <thead><tr><th>名称</th><th>分类</th><th>状态</th><th>版本</th><th>创建人</th><th>更新时间</th></tr></thead>
        <tbody>
          <tr v-for="item in filtered" :key="item.id">
            <td>{{ item.name||'—' }}</td>
            <td class="mono">{{ item.category||'—' }}</td>
            <td>{{ item.status||'—' }}</td>
            <td class="mono">{{ item.version||'—' }}</td>
            <td class="mono">{{ item.creator||'—' }}</td>
            <td class="mono">{{ item.updateTime||item.createTime||'—' }}</td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>
<script setup lang="ts">
import { api } from '@oa4rust/sdk'
import { useQuery, useQueryClient } from '@tanstack/vue-query'
import { computed, ref } from 'vue'
import { toast } from '../utils/toast'

interface Item {
  id: string
  name?: string
  category?: string
  status?: string
  version?: string
  creator?: string
  createTime?: string
  updateTime?: string
}

const listEp = '/api/processplatform/assemble/surface/process_manager/list'
const runningProcs = ref<Array<{ id?: string; name?: string }>>([])
async function loadRunningProcesses() {
  try {
    // GET processplatform/service/processing/list/{category} —— 按分类取流程（running 分类）
    const r: any = await api.get('/api/processplatform/service/processing/list/running')
    runningProcs.value = (r.data ?? []) as Array<{ id?: string; name?: string }>
    if (runningProcs.value.length === 0) toast.success('该分类暂无流程')
  } catch (e: any) {
    toast.error('加载失败: ' + (e?.message ?? ''))
  }
}
async function loadOrphans() {
  try {
    // GET designer/elementtool/{form,process,script}/orphan —— 孤儿元素检测（表单/流程/脚本）
    const [form, proc, script] = await Promise.all([
      api.get('/api/processplatform/assemble/designer/elementtool/form/orphan'),
      api.get('/api/processplatform/assemble/designer/elementtool/process/orphan'),
      api.get('/api/processplatform/assemble/designer/elementtool/script/orphan'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    toast.success(`孤儿 表单 ${n(form)} / 流程 ${n(proc)} / 脚本 ${n(script)}`)
  } catch (e: any) {
    toast.error('检测失败: ' + (e?.message ?? ''))
  }
}
async function loadAppProcesses() {
  try {
    // 消费 surface 三条真实路由：按终端(pc)取应用 / 按 key 取应用 / 按应用取流程定义
    const terminal = 'pc'
    const [byTerminal, byKey, procs] = await Promise.all([
      api.get(`/api/processplatform/assemble/surface/application/list/terminal/${terminal}`),
      api.get(`/api/processplatform/assemble/surface/application/list/key/${terminal}`),
      api.get(`/api/processplatform/assemble/surface/process/list/application/${terminal}`),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    toast.success(`终端应用 ${n(byTerminal)} / 按键应用 ${n(byKey)} / 应用流程 ${n(procs)}`)
  } catch (e: any) {
    toast.error('加载失败: ' + (e?.message ?? ''))
  }
}
async function loadManagedApps() {
  try {
    // GET processplatform/assemble/surface/application/list/complex/manage/person —— 我管理的流程应用
    const r: any = await api.get('/api/processplatform/assemble/surface/application/list/complex/manage/person')
    const n = Array.isArray(r.data) ? r.data.length : 0
    toast.success('我管理的应用：' + n + ' 个')
  } catch (e: any) {
    toast.error('加载失败: ' + (e?.message ?? ''))
  }
}
const qk = ['ProcessManager', 'list']

const search = ref('')
const items = ref<Item[]>([])
const loading = ref(false)
const qc = useQueryClient()

const { data } = useQuery({
  queryKey: qk,
  queryFn: async () => {
    loading.value = true
    try {
      const r = (await api.get(listEp)) as unknown as { data?: unknown }
      return Array.isArray(r?.data) ? (r.data as Item[]) : []
    } finally {
      loading.value = false
    }
  },
})
items.value = Array.isArray(data.value) ? (data.value as Item[]) : []

const filtered = computed(() =>
  search.value
    ? items.value.filter(
        (i) =>
          (i.name || '').toLowerCase().includes(search.value.toLowerCase()) ||
          (i.category || '').toLowerCase().includes(search.value.toLowerCase()) ||
          (i.creator || '').toLowerCase().includes(search.value.toLowerCase()),
      )
    : items.value,
)

function loadData() {
  qc.invalidateQueries({ queryKey: qk })
}
</script>
<style scoped>
.crud-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{display:flex;align-items:flex-start;justify-content:space-between;padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0}
.btn-refresh{padding:8px 12px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);cursor:pointer}
.content-panel{padding:16px}
.toolbar{display:flex;gap:8px;margin-bottom:16px}
.search-input{flex:1;padding:8px 12px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);outline:none}
.data-table{width:100%;border-collapse:collapse}
.data-table th,.data-table td{padding:10px 12px;text-align:left;border-bottom:1px solid var(--border-color)}
.data-table th{color:var(--text-muted);font-weight:600;font-size:12px;text-transform:uppercase}
.data-table tr:hover{background:var(--bg-hover)}
.mono{font-family:'Fira Code',monospace;font-size:12px;color:var(--color-secondary)}
.loading-state,.empty-state{padding:40px;text-align:center;color:var(--text-muted)}
.empty-icon{font-size:32px;margin-bottom:8px}
.skel{height:16px;background:var(--bg-elevated);border-radius:4px;margin-bottom:8px;animation:pulse 1.5s infinite}
@keyframes pulse{0%,100%{opacity:1}50%{opacity:0.4}}
.rp-chips{display:flex;flex-wrap:wrap;gap:6px;margin:8px 0}
.rp-chip{padding:2px 10px;border-radius:10px;background:var(--bg-elevated);border:1px solid var(--border-color);font-size:12px;color:var(--text-primary)}
</style>
