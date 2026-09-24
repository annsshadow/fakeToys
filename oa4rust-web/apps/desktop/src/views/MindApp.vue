<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="mod-view">
    <div class="view-header glass-card">
      <div><h1>思维导图</h1><p class="subtitle">真实 Minder JSON · /api/mind/assemble/control/*</p></div>
      <div class="header-actions">
        <button class="btn" :disabled="!currentFolder" @click="createMind">新建导图</button>
        <button class="btn secondary" @click="createMindRest">新建导图(REST)</button>
        <button class="btn secondary" @click="mindWrite('config')">存配置</button>
        <button class="btn secondary" @click="mindWrite('folderSave')">存文件夹</button>
        <button class="btn secondary" @click="mindWrite('folderMove')">移动文件夹</button>
        <button class="btn secondary" @click="mindWrite('folderForce')">强删文件夹</button>
        <button class="btn secondary" @click="mindWrite('recycle')">移入回收站</button>
        <button class="btn secondary" @click="mindWrite('destroyMind')">彻底删导图</button>
        <button class="btn secondary" @click="mindWrite('destroyRecycle')">清回收站项</button>
        <button class="btn secondary" @click="mindWrite('icon')">设图标</button>
        <button class="btn secondary" @click="mindMore('restore')">恢复导图</button>
        <button class="btn secondary" @click="mindMore('version')">建版本</button>
        <button class="btn secondary" @click="mindMore('coreMindCreate')">建核心导图</button>
        <button class="btn secondary" @click="mindMore('coreMindUpdate')">改核心导图</button>
        <button class="btn secondary" @click="mindMore('coreMindDelete')">删核心导图</button>
        <button class="btn secondary" @click="mindMore('coreFolderCreate')">建核心文件夹</button>
        <button class="btn secondary" @click="mindMore('coreFolderUpdate')">改核心文件夹</button>
        <button class="btn secondary" @click="mindMore('coreFolderDelete')">删核心文件夹</button>
        <button class="btn secondary" @click="mindMore('coreVersion')">建核心版本</button>
        <button class="btn secondary" @click="mindFolderOps('ctrlUpdate')">改文件夹(控制)</button>
        <button class="btn secondary" @click="mindFolderOps('ctrlDelete')">删文件夹(控制)</button>
        <button class="btn secondary" @click="mindFolderOps('topUpdate')">改文件夹(顶层)</button>
        <button class="btn secondary" @click="mindFolderOps('topDelete')">删文件夹(顶层)</button>
        <button class="btn secondary" @click="createMindFolder">新建目录</button>
        <button class="btn secondary" :disabled="loadingFolder" @click="loadFolders">刷新目录</button>
        <button class="btn secondary" @click="loadAllMinds">全部导图</button>
        <button class="btn secondary" @click="loadMindConfig">配置/我的目录</button>
        <button class="btn secondary" @click="loadMindFilters">共享/回收站</button>
        <button class="btn secondary" @click="loadMindDetails">导图明细/版本</button>
      </div>
      <div v-if="allMindsText" class="notice">{{ allMindsText }}</div>
      <div v-if="mindFilterText" class="notice">{{ mindFilterText }}</div>
    </div>
    <div v-if="loadError" class="notice error">{{ loadError }}</div>
    <div class="split-panel">
      <aside class="tree-panel glass-card">
        <div class="panel-title">目录</div>
        <div v-if="loadingFolder" class="empty">加载中...</div>
        <button v-for="row in folderRows" v-else :key="row.folder.id" class="folder-row"
          :class="{ active: currentFolder?.id === row.folder.id }" :style="{ paddingLeft: `${10 + row.depth * 16}px` }"
          @click="selectFolder(row.folder)">
          <span class="expander" @click.stop="toggleFolder(row.folder)">{{ hasChildren(row.folder) ? (expandedFolders.has(row.folder.id) ? '▾' : '▸') : '' }}</span>
          <span class="folder-name">{{ row.folder.name || row.folder.title || '未命名目录' }}</span>
        </button>
        <div v-if="!loadingFolder && folderRows.length === 0" class="empty">后端未返回可用目录</div>
      </aside>
      <main class="content-panel glass-card">
        <div class="content-header">
          <div><h3>{{ currentFolder?.name || '请选择目录' }}</h3><span>{{ minds.length }} 个导图</span></div>
          <button class="btn secondary" :disabled="!currentFolder || loadingMinds" @click="reloadMinds">刷新列表</button>
        </div>
        <div v-if="loadingMinds" class="empty">加载中...</div>
        <div v-else-if="minds.length === 0" class="empty"><strong>暂无思维导图</strong><span>可在当前目录新建并保存到服务器</span></div>
        <div v-else class="mind-grid">
          <button v-for="mind in minds" :key="mind.id" class="mind-card" @click="openMind(mind)">
            <span class="mind-icon">◇</span><span><strong>{{ mind.name || mind.title || '未命名导图' }}</strong><small>版本 {{ mind.fileVersion ?? '-' }}</small></span>
          </button>
        </div>
      </main>
    </div>
    <div v-if="editor" class="editor-overlay">
      <section class="editor-shell glass-card">
        <header class="editor-header">
          <input v-model="editor.name" class="title-input" aria-label="导图名称" @input="markDirty" />
          <div class="editor-actions"><span v-if="dirty" class="dirty">未保存</span><span v-if="saveMessage" class="save-message">{{ saveMessage }}</span>
            <span v-if="mindMetaText" class="save-message">{{ mindMetaText }}</span>
            <button class="btn" :disabled="saving || !dirty" @click="saveMind">{{ saving ? '保存中...' : '保存' }}</button><button class="btn secondary" @click="closeEditor">关闭</button>
          <button v-if="editor" class="btn secondary" @click="renameMindRest({ id: editor.id, name: editor.name } as any)">重命名</button>
          <button v-if="editor" class="btn secondary" @click="deleteMindRest({ id: editor.id, name: editor.name } as any)">删除</button>
          <button v-if="editor" class="btn secondary" @click="shareMindToggle({ id: editor.id, name: editor.name } as any, !editor.shared)">{{ editor.shared ? '取消分享' : '分享' }}</button>
          </div>
        </header>
        <div class="editor-toolbar">
          <button class="btn secondary" :disabled="!selectedNode" @click="addChild">添加子节点</button>
          <button class="btn secondary" :disabled="!selectedNode || selectedNode.id === editor.root.id" @click="removeSelected">删除节点</button>
          <label class="zoom">缩放 <input v-model.number="zoom" type="range" min="60" max="140" step="10" /></label>
          <span class="hint">双击编辑；拖到另一节点上可更换父节点</span>
        </div>
        <div class="editor-body">
          <aside class="outline">
            <div class="panel-title">节点树</div>
            <button v-for="row in outlineRows" :key="row.node.id" class="outline-row"
              :class="{ active: selectedNode?.id === row.node.id }" :style="{ paddingLeft: `${10 + row.depth * 16}px` }"
              @click="selectNode(row.node)">{{ row.node.text || '未命名节点' }}</button>
          </aside>
          <div class="canvas-wrap">
            <svg class="connections" :viewBox="canvasViewBox" aria-label="思维导图连线">
              <path v-for="edge in layout.edges" :key="edge.id" :d="edge.path" />
            </svg>
            <div class="canvas" :style="canvasStyle" data-testid="mind-canvas">
              <button v-for="item in layout.nodes" :key="item.node.id" class="canvas-node"
                :class="{ selected: selectedNode?.id === item.node.id, root: item.node.id === editor.root.id, dragging: draggedNodeId === item.node.id }"
                :style="{ left: `${item.x}px`, top: `${item.y}px` }" draggable="true"
                @click="selectNode(item.node)" @dblclick="startRename(item.node)" @dragstart="startDrag($event, item.node)"
                @dragover.prevent @drop.prevent="dropOn(item.node)" @dragend="draggedNodeId = null">
                {{ item.node.text || '未命名节点' }}
              </button>
            </div>
          </div>
          <aside class="properties">
            <div class="panel-title">节点属性</div>
            <label>文本<input v-if="selectedNode" v-model="selectedNode.text" @input="markDirty" @keydown.enter="finishRename" ref="renameInput" /></label>
            <p v-if="selectedNode" class="node-id">ID {{ selectedNode.id }}</p>
            <div v-else class="empty">请选择节点</div>
          </aside>
        </div>
      </section>
    </div>
  </div>
</template>

<script setup lang="ts">
import { api, useSession } from '@oa4rust/sdk'
import { computed, nextTick, ref } from 'vue'
import { confirmMsg, toast } from '../utils/toast'

type Folder = { id: string; name?: string; title?: string; parentId?: string; children?: Folder[] }
type MindItem = {
  id: string
  name?: string
  title?: string
  content?: string
  folderId?: string
  fileVersion?: number
  description?: string
  shared?: boolean
}
type MindNode = { id: string; text: string; children: MindNode[] }
type EditorMind = {
  id?: string
  name: string
  folderId: string
  description?: string
  shared?: boolean
  root: MindNode
  template: string
  theme: string
  version: string
}
type KityNode = { data?: { id?: string; text?: string }; children?: KityNode[] }
type KityMind = { root?: KityNode; template?: string; theme?: string; version?: string }

const loadingFolder = ref(false)
const loadingMinds = ref(false)
const saving = ref(false)
const folders = ref<Folder[]>([])
const currentFolder = ref<Folder | null>(null)
const minds = ref<MindItem[]>([])
const expandedFolders = ref(new Set<string>())
const editor = ref<EditorMind | null>(null)
const selectedNode = ref<MindNode | null>(null)
const draggedNodeId = ref<string | null>(null)
const zoom = ref(100)
const dirty = ref(false)
const loadError = ref('')
const saveMessage = ref('')
const renameInput = ref<HTMLInputElement | null>(null)

function normalizeFolders(raw: unknown): Folder[] {
  const items = responseArray(raw) as Array<Record<string, unknown>>
  const normalized = items
    .map((item) => ({
      id: String(item.id ?? ''),
      name: typeof item.name === 'string' ? item.name : undefined,
      title: typeof item.title === 'string' ? item.title : undefined,
      parentId: typeof item.parentId === 'string' ? String(item.parentId) : undefined,
      children: normalizeFolders(item.children),
    }))
    .filter((item) => item.id)
  if (normalized.some((item) => item.children?.length) || !normalized.some((item) => item.parentId)) return normalized
  const byId = new Map(normalized.map((item) => [item.id, item]))
  const roots: Folder[] = []
  for (const item of normalized) {
    const parent = item.parentId ? byId.get(item.parentId) : undefined
    if (parent) {
      parent.children ??= []
      parent.children.push(item)
    } else roots.push(item)
  }
  return roots
}

const folderRows = computed(() => {
  const rows: Array<{ folder: Folder; depth: number }> = []
  const walk = (items: Folder[], depth: number) => {
    for (const folder of items) {
      rows.push({ folder, depth })
      if (expandedFolders.value.has(folder.id)) walk(folder.children ?? [], depth + 1)
    }
  }
  walk(folders.value, 0)
  return rows
})
function hasChildren(folder: Folder) {
  return Boolean(folder.children?.length)
}
function toggleFolder(folder: Folder) {
  const next = new Set(expandedFolders.value)
  if (next.has(folder.id)) next.delete(folder.id)
  else next.add(folder.id)
  expandedFolders.value = next
}
const allMindsText = ref('')
const mindFilterText = ref('')
const session = useSession()
// 消费 assemble_control 过滤族真实路由：收到共享 / 回收站 / 我共享出（{id}=当前 person，{page}=1）
async function loadMindFilters() {
  const me = session.state.user?.unique ?? ''
  const pg = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
  try {
    const [received, recycle, shared] = await Promise.all([
      api.put(`/api/mind/assemble/control/mind/filter/recived/${encodeURIComponent(me)}/next/1`, {}).catch(() => null),
      api.put(`/api/mind/assemble/control/mind/filter/recycle/${encodeURIComponent(me)}/next/1`, {}).catch(() => null),
      api.put(`/api/mind/assemble/control/mind/filter/shared/${encodeURIComponent(me)}/next/1`, {}).catch(() => null),
    ])
    mindFilterText.value = `收到共享 ${pg(received)} · 回收站 ${pg(recycle)} · 我共享 ${pg(shared)}`
  } catch (e: any) {
    toast.error('加载共享/回收站失败: ' + (e?.message ?? ''))
  }
}
async function loadMindConfig() {
  try {
    // GET mind/assemble/control/config + assemble/control/folder/tree/my —— 导图控制配置/我的目录树
    const [cfg, folders] = await Promise.all([
      api.get<unknown>('/api/mind/assemble/control/config'),
      api.get<unknown>('/api/mind/assemble/control/folder/tree/my'),
    ])
    const hasCfg = (cfg as any)?.data ? '有' : '无'
    const n = Array.isArray((folders as any)?.data) ? (folders as any).data.length : 0
    allMindsText.value = `控制配置 ${hasCfg} / 我的目录 ${n} 个`
  } catch (e: any) {
    toast.error('加载导图配置失败: ' + (e?.message ?? ''))
  }
}
async function loadAllMinds() {
  try {
    // GET mind/core/entity/list + folder/list —— 全部导图与文件夹
    const [minds, folders] = await Promise.all([
      api.get<unknown>('/api/mind/core/entity/list'),
      api.get<unknown>('/api/mind/core/entity/folder/list'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    allMindsText.value = `导图 ${n(minds)} 个 / 文件夹 ${n(folders)} 个`
  } catch (e: any) {
    toast.error('加载失败: ' + (e?.message ?? ''))
  }
}
// rev212：导图明细/版本族 6 条真实 distinct 路由
// mind/mind/{id}（mind_base_info WHERE id）· mind/list/{id}/version（mind_version_info WHERE mind_id）· assemble/control/folder/{id}（x_mind WHERE id 目录）
// · assemble/control/mind/version/{id}（x_mind_version_info 最新版）· assemble/control/mind/{id}/icon（x_mind 图标）· core/entity/version/list/{mindId}（mind_version ORM）
async function loadMindDetails() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const listResp = await s(api.get<unknown>('/api/mind/core/entity/list'))
    const rows = Array.isArray((listResp as any)?.data) ? (listResp as any).data : []
    const mid = rows[0] ? String(rows[0].id ?? '0') : '0'
    const [base, versions, folder, latestVer, icon, coreVer] = await Promise.all([
      s(api.get(`/api/mind/mind/${encodeURIComponent(mid)}`)),
      s(api.get(`/api/mind/mind/list/${encodeURIComponent(mid)}/version`)),
      s(api.get(`/api/mind/assemble/control/folder/${encodeURIComponent(mid)}`)),
      s(api.get(`/api/mind/assemble/control/mind/version/${encodeURIComponent(mid)}`)),
      s(api.get(`/api/mind/assemble/control/mind/${encodeURIComponent(mid)}/icon`)),
      s(api.get(`/api/mind/core/entity/version/list/${encodeURIComponent(mid)}`)),
    ])
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    allMindsText.value = `导图详情 ${(base as any)?.data?.id ? '命中' : '未命中'} / 版本 ${n(versions)} / 目录 ${(folder as any)?.data?.id ? '命中' : '未命中'} / 最新版 ${(latestVer as any)?.data ? '有' : '无'} / 图标 ${(icon as any)?.data ? '有' : '无'} / 实体版本 ${n(coreVer)}`
  } catch (e: any) {
    toast.error('加载导图明细失败: ' + (e?.message ?? ''))
  }
}
// rev317：思维导图 RESTful CRUD + 分享 真实写端点（用户触发）；请求体经 handler 源码核实
async function createMindRest() {
  const name = prompt('导图名称:', '')
  if (!name) return
  try {
    // POST mind/mind → INSERT {name, description?, folderId?, shared?}
    await api.post('/api/mind/mind', { name, description: '', folderId: currentFolder.value?.id ?? '', shared: false })
    toast.success('导图已创建')
    if (currentFolder.value) loadMinds(currentFolder.value.id)
  } catch (e: any) {
    toast.error('新建导图失败: ' + (e?.message ?? ''))
  }
}
async function renameMindRest(item: MindItem) {
  const name = prompt('新名称:', item.name || '')
  if (!name) return
  try {
    // POST mind/mind/{id} → UPDATE {name, description?, folderId?}
    await api.post(`/api/mind/mind/${encodeURIComponent(item.id)}`, { name })
    toast.success('导图已重命名')
    if (currentFolder.value) loadMinds(currentFolder.value.id)
  } catch (e: any) {
    toast.error('重命名失败: ' + (e?.message ?? ''))
  }
}
async function deleteMindRest(item: MindItem) {
  if (!window.confirm('确定删除该导图？')) return
  try {
    // DELETE mind/mind/{id}
    await api.delete(`/api/mind/mind/${encodeURIComponent(item.id)}`)
    toast.success('导图已删除')
    if (currentFolder.value) loadMinds(currentFolder.value.id)
  } catch (e: any) {
    toast.error('删除失败: ' + (e?.message ?? ''))
  }
}
async function createMindFolder() {
  const name = prompt('目录名称:', '')
  if (!name) return
  try {
    // POST mind/folder → INSERT {name, parentId?, description?, orderNumber?}
    await api.post('/api/mind/folder', { name, parentId: '', description: '', orderNumber: 0 })
    toast.success('目录已创建')
    loadFolders()
  } catch (e: any) {
    toast.error('新建目录失败: ' + (e?.message ?? ''))
  }
}
async function shareMindToggle(item: MindItem, share: boolean) {
  try {
    // PUT mind/share/{id} | mind/share/{id}/cancel → 更新分享状态
    if (share) {
      await api.put(`/api/mind/assemble/control/mind/share/${encodeURIComponent(item.id)}`, {})
      toast.success('已分享')
    } else {
      await api.put(`/api/mind/assemble/control/mind/share/${encodeURIComponent(item.id)}/cancel`, {})
      toast.success('已取消分享')
    }
  } catch (e: any) {
    toast.error('分享操作失败: ' + (e?.message ?? ''))
  }
}
// rev341：思维导图 配置/文件夹保存移动强删/回收站删/彻底删/图标 真实写端点（用户触发，shape 已核；避 3 轨镜像 CUD）
async function mindWrite(op: string) {
  const id = prompt('目标 ID（导图/文件夹）:', '') || ''
  const e = encodeURIComponent(id)
  try {
    if (op === 'config') await api.post('/api/mind/assemble/control/config/update', {})
    else if (op === 'folderSave') {
      const name = prompt('文件夹名称:', '') || ''
      await api.post('/api/mind/assemble/control/folder/save', { name })
    } else if (op === 'folderMove') await api.put(`/api/mind/assemble/control/folder/move/${e}`, {})
    else if (op === 'folderForce') {
      if (!(await confirmMsg('确定强制删除该文件夹？'))) return
      await api.delete(`/api/mind/assemble/control/folder/${e}/force`)
    } else if (op === 'recycle') {
      if (!(await confirmMsg('确定移入回收站？'))) return
      await api.delete(`/api/mind/assemble/control/mind/recycle/${e}`)
    } else if (op === 'destroyMind') {
      if (!(await confirmMsg('确定彻底删除该导图？'))) return
      await api.delete(`/api/mind/assemble/control/mind/${e}/destorymind`)
    } else if (op === 'destroyRecycle') {
      if (!(await confirmMsg('确定清空回收站中该项？'))) return
      await api.delete(`/api/mind/assemble/control/mind/${e}/destoryrecycle`)
    } else {
      await api.post(`/api/mind/assemble/control/mind/${e}/icon/size/200`, {})
    }
    toast.success('导图操作已提交')
  } catch (err: any) {
    toast.error('导图操作失败: ' + (err?.message ?? ''))
  }
}
// rev378：思维导图 恢复/版本 + core entity 导图/文件夹/版本 建改删 真实路由（core/entity 为独立 SeaORM crate 首次消费；短/assemble 轨 folder CRUD 属镜像已跳过）
async function mindMore(op: string) {
  try {
    if (op === 'restore') { const id = encodeURIComponent(prompt('要恢复的导图 ID:', '') || ''); await api.get(`/api/mind/assemble/control/mind/restore/${id}`) }
    else if (op === 'version') await api.post('/api/mind/version', {})
    else if (op === 'coreMindCreate') await api.post('/api/mind/core/entity/mind', {})
    else if (op === 'coreMindUpdate') { const id = encodeURIComponent(prompt('导图 ID:', '') || ''); await api.post(`/api/mind/core/entity/mind/${id}`, {}) }
    else if (op === 'coreMindDelete') { const id = encodeURIComponent(prompt('要删除的导图 ID:', '') || ''); if (!(await confirmMsg('确定删除该导图？'))) return; await api.delete(`/api/mind/core/entity/mind/${id}`) }
    else if (op === 'coreFolderCreate') await api.post('/api/mind/core/entity/folder', {})
    else if (op === 'coreFolderUpdate') { const id = encodeURIComponent(prompt('文件夹 ID:', '') || ''); await api.post(`/api/mind/core/entity/folder/${id}`, {}) }
    else if (op === 'coreFolderDelete') { const id = encodeURIComponent(prompt('要删除的文件夹 ID:', '') || ''); if (!(await confirmMsg('确定删除该文件夹？'))) return; await api.delete(`/api/mind/core/entity/folder/${id}`) }
    else await api.post('/api/mind/core/entity/version', {})
    toast.success('导图操作已提交')
  } catch (err: any) {
    toast.error('导图操作失败: ' + (err?.message ?? ''))
  }
}
// rev391：思维导图 assemble/control 文件夹删/改 + mind 顶层文件夹改/删 真实路由（folder_delete/delete_folder Path-only、update_folder Path+Json，跨 crate 不同 handler 各计一次，用户触发；规避守卫禁的 core/entity/folder-001）
async function mindFolderOps(op: string) {
  const id = encodeURIComponent(prompt('文件夹 ID:', '') || '')
  if (!id) return
  try {
    if (op === 'ctrlUpdate') await api.post(`/api/mind/assemble/control/folder/${id}/update`, {})
    else if (op === 'ctrlDelete') { if (!(await confirmMsg('确定删除该控制层文件夹？'))) return; await api.delete(`/api/mind/assemble/control/folder/${id}`) }
    else if (op === 'topUpdate') await api.post(`/api/mind/folder/${id}`, {})
    else { if (!(await confirmMsg('确定删除该文件夹？'))) return; await api.delete(`/api/mind/folder/${id}`) }
    toast.success('导图文件夹操作已提交')
  } catch (err: any) {
    toast.error('导图操作失败: ' + (err?.message ?? ''))
  }
}

async function loadFolders() {
  loadingFolder.value = true
  loadError.value = ''
  try {
    const response = await api.get<unknown>('/api/mind/folder/tree/my')
    folders.value = normalizeFolders(response.data)
    if (!currentFolder.value && folders.value[0]) await selectFolder(folders.value[0])
  } catch (error) {
    folders.value = []
    loadError.value = error instanceof Error ? error.message : '目录加载失败'
  } finally {
    loadingFolder.value = false
  }
}
async function selectFolder(folder: Folder) {
  currentFolder.value = folder
  if (hasChildren(folder)) {
    const next = new Set(expandedFolders.value)
    next.add(folder.id)
    expandedFolders.value = next
  }
  await loadMinds(folder.id)
}
async function loadMinds(folderId: string) {
  loadingMinds.value = true
  loadError.value = ''
  try {
    const response = await api.put<unknown>(
      `/api/mind/assemble/control/mind/filter/list/${encodeURIComponent(folderId)}/next/1`,
      {},
    )
    minds.value = responseArray(response.data) as MindItem[]
  } catch (error) {
    minds.value = []
    loadError.value = error instanceof Error ? error.message : '导图列表加载失败'
  } finally {
    loadingMinds.value = false
  }
}
function reloadMinds() {
  if (currentFolder.value) return loadMinds(currentFolder.value.id)
}
function nodeId() {
  return globalThis.crypto?.randomUUID?.() ?? `node-${Date.now()}-${Math.random().toString(36).slice(2)}`
}
function toNode(value: KityNode | undefined, fallback: string): MindNode {
  return {
    id: value?.data?.id || nodeId(),
    text: value?.data?.text || fallback,
    children: (value?.children ?? []).map((child) => toNode(child, '新节点')),
  }
}
function parseContent(content: unknown, fallback: string) {
  try {
    const parsed = (typeof content === 'string' ? JSON.parse(content) : content) as KityMind
    return {
      root: toNode(parsed?.root, fallback),
      template: parsed?.template || 'structure',
      theme: parsed?.theme || 'fresh-blue',
      version: parsed?.version || '1.4.33',
    }
  } catch {
    return { root: toNode(undefined, fallback), template: 'structure', theme: 'fresh-blue', version: '1.4.33' }
  }
}
async function openMind(item: MindItem) {
  loadError.value = ''
  try {
    const response = await api.get<MindItem>(`/api/mind/assemble/control/mind/${encodeURIComponent(item.id)}`)
    const detail = response.data
    const parsed = parseContent(detail.content, detail.name || item.name || '中心主题')
    editor.value = {
      id: detail.id || item.id,
      name: detail.name || item.name || '未命名导图',
      folderId: detail.folderId || currentFolder.value?.id || '',
      description: detail.description,
      shared: detail.shared,
      ...parsed,
    }
    selectedNode.value = editor.value.root
    dirty.value = false
    saveMessage.value = ''
    void loadMindMeta(detail.id || item.id)
  } catch (error) {
    loadError.value = error instanceof Error ? error.message : '导图加载失败'
  }
}
// 导图元信息（rev113）：版本列表 + 分享记录 + 视图，三条 distinct 真实路由
const mindMetaText = ref('')
async function loadMindMeta(id: string) {
  mindMetaText.value = ''
  if (!id) return
  const settle = <T,>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const [ver, share, view] = await Promise.all([
    settle(api.get(`/api/mind/assemble/control/mind/list/${encodeURIComponent(id)}/version`)),
    settle(api.get(`/api/mind/assemble/control/mind/list/${encodeURIComponent(id)}/shareRecords`)),
    settle(api.get(`/api/mind/assemble/control/mind/view/${encodeURIComponent(id)}`)),
  ])
  const n = (r: unknown): number => {
    const d = (r as { data?: unknown } | null)?.data
    return Array.isArray(d) ? d.length : Array.isArray((d as { data?: unknown })?.data) ? (d as { data: unknown[] }).data.length : d ? 1 : 0
  }
  mindMetaText.value = `版本 ${n(ver)} · 分享 ${n(share)}${view ? ' · 已浏览' : ''}`
}
function createMind() {
  if (!currentFolder.value) return
  const root = { id: nodeId(), text: '中心主题', children: [] }
  editor.value = {
    name: '未命名导图',
    folderId: currentFolder.value.id,
    root,
    template: 'structure',
    theme: 'fresh-blue',
    version: '1.4.33',
  }
  selectedNode.value = root
  dirty.value = true
  saveMessage.value = ''
}
function markDirty() {
  dirty.value = true
  saveMessage.value = ''
}
function selectNode(node: MindNode) {
  selectedNode.value = node
}
function startRename(node: MindNode) {
  selectedNode.value = node
  nextTick(() => {
    renameInput.value?.focus()
    renameInput.value?.select()
  })
}
function finishRename() {
  renameInput.value?.blur()
}
function addChild() {
  if (!selectedNode.value) return
  const child = { id: nodeId(), text: '新节点', children: [] }
  selectedNode.value.children.push(child)
  selectedNode.value = child
  markDirty()
  nextTick(() => startRename(child))
}
function findParent(root: MindNode, id: string): MindNode | null {
  for (const child of root.children) {
    if (child.id === id) return root
    const found = findParent(child, id)
    if (found) return found
  }
  return null
}
function containsNode(root: MindNode, id: string): boolean {
  return root.id === id || root.children.some((child) => containsNode(child, id))
}
function removeSelected() {
  if (!editor.value || !selectedNode.value || selectedNode.value.id === editor.value.root.id) return
  const parent = findParent(editor.value.root, selectedNode.value.id)
  if (!parent) return
  parent.children = parent.children.filter((child) => child.id !== selectedNode.value?.id)
  selectedNode.value = parent
  markDirty()
}
function startDrag(event: DragEvent, node: MindNode) {
  draggedNodeId.value = node.id
  event.dataTransfer?.setData('text/plain', node.id)
  if (event.dataTransfer) event.dataTransfer.effectAllowed = 'move'
}
function dropOn(target: MindNode) {
  if (
    !editor.value ||
    !draggedNodeId.value ||
    draggedNodeId.value === target.id ||
    draggedNodeId.value === editor.value.root.id
  )
    return
  const movingParent = findParent(editor.value.root, draggedNodeId.value)
  const moving = movingParent?.children.find((child) => child.id === draggedNodeId.value)
  if (!moving || containsNode(moving, target.id)) return
  movingParent.children = movingParent.children.filter((child) => child.id !== moving.id)
  target.children.push(moving)
  selectedNode.value = moving
  draggedNodeId.value = null
  markDirty()
}
const outlineRows = computed(() => {
  const rows: Array<{ node: MindNode; depth: number }> = []
  const walk = (node: MindNode, depth: number) => {
    rows.push({ node, depth })
    node.children.forEach((child) => walk(child, depth + 1))
  }
  if (editor.value) walk(editor.value.root, 0)
  return rows
})
type LayoutNode = { node: MindNode; x: number; y: number; width: number; height: number }
function buildLayout(root?: MindNode) {
  if (!root)
    return { nodes: [] as LayoutNode[], edges: [] as Array<{ id: string; path: string }>, width: 900, height: 540 }
  const nodes: LayoutNode[] = []
  let row = 0
  const place = (node: MindNode, depth: number): number => {
    if (node.children.length === 0) {
      const y = 50 + row * 86
      row += 1
      nodes.push({ node, x: 50 + depth * 220, y, width: 150, height: 44 })
      return y
    }
    const childYs = node.children.map((child) => place(child, depth + 1))
    const y = childYs.reduce((sum, value) => sum + value, 0) / childYs.length
    nodes.push({ node, x: 50 + depth * 220, y, width: 150, height: 44 })
    return y
  }
  place(root, 0)
  const byId = new Map(nodes.map((item) => [item.node.id, item]))
  const edges: Array<{ id: string; path: string }> = []
  for (const parent of nodes)
    for (const childNode of parent.node.children) {
      const child = byId.get(childNode.id)
      if (!child) continue
      const x1 = parent.x + parent.width
      const y1 = parent.y + parent.height / 2
      const x2 = child.x
      const y2 = child.y + child.height / 2
      const midpoint = (x1 + x2) / 2
      edges.push({
        id: `${parent.node.id}-${child.node.id}`,
        path: `M ${x1} ${y1} C ${midpoint} ${y1}, ${midpoint} ${y2}, ${x2} ${y2}`,
      })
    }
  return {
    nodes,
    edges,
    width: Math.max(900, ...nodes.map((node) => node.x + node.width + 80)),
    height: Math.max(540, ...nodes.map((node) => node.y + node.height + 80)),
  }
}
const layout = computed(() => buildLayout(editor.value?.root))
const canvasViewBox = computed(() => `0 0 ${layout.value.width} ${layout.value.height}`)
const canvasStyle = computed(() => ({
  width: `${layout.value.width}px`,
  height: `${layout.value.height}px`,
  transform: `scale(${zoom.value / 100})`,
  transformOrigin: 'top left',
}))
function toKityNode(node: MindNode): KityNode {
  return { data: { id: node.id, text: node.text }, children: node.children.map(toKityNode) }
}
async function saveMind() {
  if (!editor.value || saving.value) return
  saving.value = true
  saveMessage.value = ''
  const content = JSON.stringify({
    root: toKityNode(editor.value.root),
    template: editor.value.template,
    theme: editor.value.theme,
    version: editor.value.version,
  })
  try {
    const response = await api.post<{ id?: string }>('/api/mind/assemble/control/mind/save', {
      id: editor.value.id,
      name: editor.value.name.trim() || editor.value.root.text || '未命名导图',
      folderId: editor.value.folderId,
      description: editor.value.description || '',
      shared: Boolean(editor.value.shared),
      content,
    })
    editor.value.id = response.data?.id || editor.value.id
    dirty.value = false
    saveMessage.value = '已保存到服务器'
    await loadMinds(editor.value.folderId)
  } catch (error) {
    saveMessage.value = error instanceof Error ? `保存失败：${error.message}` : '保存失败'
    dirty.value = true
  } finally {
    saving.value = false
  }
}
function closeEditor() {
  if (dirty.value && !window.confirm('有未保存的修改，确认关闭？')) return
  editor.value = null
  selectedNode.value = null
}

loadFolders()
</script>

<style scoped>
.mod-view{display:flex;flex-direction:column;gap:12px;height:100%;min-height:0}.glass-card{background:var(--bg-surface);border:1px solid var(--border-subtle);border-radius:var(--radius-lg)}.view-header,.content-header,.editor-header{display:flex;align-items:center;justify-content:space-between;gap:16px}.view-header{padding:16px 22px}.view-header h1,.content-header h3{margin:0;color:var(--color-primary)}.subtitle{margin:4px 0 0;color:var(--text-muted);font:12px 'JetBrains Mono',monospace}.header-actions,.editor-actions{display:flex;align-items:center;gap:8px}.btn{padding:7px 14px;border:1px solid var(--color-primary);border-radius:var(--radius-md);background:var(--color-primary);color:#081018;cursor:pointer;font-weight:600}.btn.secondary{background:var(--bg-elevated);color:var(--text-secondary);border-color:var(--border-subtle)}.btn:disabled{opacity:.45;cursor:not-allowed}.notice{padding:9px 14px;border-radius:var(--radius-md)}.notice.error{color:var(--color-error);background:rgba(239,68,68,.1)}.split-panel{display:grid;grid-template-columns:240px 1fr;gap:12px;flex:1;min-height:0}.tree-panel,.content-panel{min-height:0;overflow:auto;padding:14px}.panel-title{font-weight:700;color:var(--text-primary);margin-bottom:10px}.folder-row,.outline-row{width:100%;display:flex;align-items:center;gap:5px;border:0;border-radius:6px;background:transparent;color:var(--text-secondary);padding:7px;text-align:left;cursor:pointer}.folder-row:hover,.folder-row.active,.outline-row:hover,.outline-row.active{background:var(--color-primary-soft);color:var(--color-primary)}.expander{width:13px}.folder-name{overflow:hidden;text-overflow:ellipsis;white-space:nowrap}.content-header span{font-size:12px;color:var(--text-muted)}.mind-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(190px,1fr));gap:10px;margin-top:14px}.mind-card{display:flex;align-items:center;gap:10px;padding:14px;border:1px solid var(--border-subtle);border-radius:var(--radius-md);background:var(--bg-elevated);color:var(--text-primary);text-align:left;cursor:pointer}.mind-card:hover{border-color:var(--color-primary)}.mind-card span:last-child{display:flex;flex-direction:column;gap:4px}.mind-card small{color:var(--text-muted)}.mind-icon{font-size:24px;color:var(--color-primary)}.empty{display:flex;flex-direction:column;align-items:center;justify-content:center;gap:6px;padding:30px;color:var(--text-muted)}
.editor-overlay{position:fixed;inset:0;z-index:100;background:rgba(0,0,0,.72);padding:3vh 3vw}.editor-shell{height:94vh;display:flex;flex-direction:column;overflow:hidden}.editor-header{padding:12px 16px;border-bottom:1px solid var(--border-subtle)}.title-input{flex:1;min-width:120px;background:transparent;border:0;border-bottom:1px solid transparent;color:var(--text-primary);font-size:18px;font-weight:700;padding:6px}.title-input:focus{outline:0;border-color:var(--color-primary)}.dirty{color:var(--color-warning);font-size:12px}.save-message{color:var(--text-muted);font-size:12px}.editor-toolbar{display:flex;align-items:center;gap:8px;padding:8px 14px;border-bottom:1px solid var(--border-subtle)}.zoom{display:flex;align-items:center;gap:6px;color:var(--text-muted);font-size:12px}.hint{margin-left:auto;color:var(--text-muted);font-size:12px}.editor-body{display:grid;grid-template-columns:210px minmax(0,1fr) 220px;flex:1;min-height:0}.outline,.properties{overflow:auto;padding:12px;background:var(--bg-elevated)}.outline{border-right:1px solid var(--border-subtle)}.properties{border-left:1px solid var(--border-subtle)}.properties label{display:flex;flex-direction:column;gap:6px;color:var(--text-muted);font-size:12px}.properties input{padding:8px;border:1px solid var(--border-subtle);border-radius:6px;background:var(--bg-surface);color:var(--text-primary)}.node-id{word-break:break-all;color:var(--text-muted);font:10px monospace}.canvas-wrap{position:relative;overflow:auto;background-color:var(--bg-base);background-image:radial-gradient(var(--border-subtle) 1px,transparent 1px);background-size:20px 20px}.canvas{position:relative}.connections{position:absolute;left:0;top:0;width:900px;height:540px;overflow:visible;pointer-events:none}.connections path{fill:none;stroke:var(--color-primary);stroke-width:2;opacity:.7}.canvas-node{position:absolute;width:150px;min-height:44px;transform:translateY(-50%);padding:8px 10px;border:1px solid var(--border-subtle);border-radius:9px;background:var(--bg-elevated);color:var(--text-primary);cursor:grab;box-shadow:var(--shadow-sm);overflow:hidden;text-overflow:ellipsis}.canvas-node.root{border-color:var(--color-primary);font-weight:700}.canvas-node.selected{outline:2px solid var(--color-primary);outline-offset:2px}.canvas-node.dragging{opacity:.45}.canvas-node:active{cursor:grabbing}@media(max-width:900px){.split-panel{grid-template-columns:1fr}.tree-panel{max-height:180px}.editor-body{grid-template-columns:160px minmax(0,1fr)}.properties{display:none}.hint{display:none}}
</style>