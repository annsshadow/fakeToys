<template>
  <div class="mod-view">
    <div class="view-header glass-card">
      <div><h1>思维导图</h1><p class="subtitle">真实 Minder JSON · /jaxrs/mind/assemble/control/*</p></div>
      <div class="header-actions">
        <button class="btn" :disabled="!currentFolder" @click="createMind">新建导图</button>
        <button class="btn secondary" :disabled="loadingFolder" @click="loadFolders">刷新目录</button>
      </div>
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
            <button class="btn" :disabled="saving || !dirty" @click="saveMind">{{ saving ? '保存中...' : '保存' }}</button><button class="btn secondary" @click="closeEditor">关闭</button>
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
import { api } from '@oa4rust/sdk'
import { computed, nextTick, ref } from 'vue'

type Folder = { id: string; name?: string; title?: string; parentId?: string; children?: Folder[] }
type MindItem = { id: string; name?: string; title?: string; content?: string; folderId?: string; fileVersion?: number; description?: string; shared?: boolean }
type MindNode = { id: string; text: string; children: MindNode[] }
type EditorMind = { id?: string; name: string; folderId: string; description?: string; shared?: boolean; root: MindNode; template: string; theme: string; version: string }
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
  const normalized = items.map((item) => ({
    id: String(item.id ?? ''),
    name: typeof item.name === 'string' ? item.name : undefined,
    title: typeof item.title === 'string' ? item.title : undefined,
    parentId: typeof (item.parentId ?? item['"parentId"']) === 'string' ? String(item.parentId ?? item['"parentId"']) : undefined,
    children: normalizeFolders(item.children),
  })).filter((item) => item.id)
  if (normalized.some((item) => item.children?.length) || !normalized.some((item) => item.parentId)) return normalized
  const byId = new Map(normalized.map((item) => [item.id, item]))
  const roots: Folder[] = []
  for (const item of normalized) {
    const parent = item.parentId ? byId.get(item.parentId) : undefined
    if (parent) (parent.children ??= []).push(item)
    else roots.push(item)
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
function hasChildren(folder: Folder) { return Boolean(folder.children?.length) }
function toggleFolder(folder: Folder) {
  const next = new Set(expandedFolders.value)
  if (next.has(folder.id)) next.delete(folder.id)
  else next.add(folder.id)
  expandedFolders.value = next
}
async function loadFolders() {
  loadingFolder.value = true
  loadError.value = ''
  try {
    const response = await api.get<unknown>('/jaxrs/mind/folder/tree/my')
    folders.value = normalizeFolders(response.data)
    if (!currentFolder.value && folders.value[0]) await selectFolder(folders.value[0])
  } catch (error) {
    folders.value = []
    loadError.value = error instanceof Error ? error.message : '目录加载失败'
  } finally { loadingFolder.value = false }
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
    const response = await api.put<unknown>(`/jaxrs/mind/assemble/control/mind/filter/list/${encodeURIComponent(folderId)}/next/1`, {})
    minds.value = responseArray(response.data) as MindItem[]
  } catch (error) {
    minds.value = []
    loadError.value = error instanceof Error ? error.message : '导图列表加载失败'
  } finally { loadingMinds.value = false }
}
function reloadMinds() { if (currentFolder.value) return loadMinds(currentFolder.value.id) }
function nodeId() { return globalThis.crypto?.randomUUID?.() ?? `node-${Date.now()}-${Math.random().toString(36).slice(2)}` }
function toNode(value: KityNode | undefined, fallback: string): MindNode {
  return { id: value?.data?.id || nodeId(), text: value?.data?.text || fallback, children: (value?.children ?? []).map((child) => toNode(child, '新节点')) }
}
function parseContent(content: unknown, fallback: string) {
  try {
    const parsed = (typeof content === 'string' ? JSON.parse(content) : content) as KityMind
    return { root: toNode(parsed?.root, fallback), template: parsed?.template || 'structure', theme: parsed?.theme || 'fresh-blue', version: parsed?.version || '1.4.33' }
  } catch { return { root: toNode(undefined, fallback), template: 'structure', theme: 'fresh-blue', version: '1.4.33' } }
}
async function openMind(item: MindItem) {
  loadError.value = ''
  try {
    const response = await api.get<MindItem>(`/jaxrs/mind/assemble/control/mind/${encodeURIComponent(item.id)}`)
    const detail = response.data
    const parsed = parseContent(detail.content, detail.name || item.name || '中心主题')
    editor.value = { id: detail.id || item.id, name: detail.name || item.name || '未命名导图', folderId: detail.folderId || currentFolder.value?.id || '', description: detail.description, shared: detail.shared, ...parsed }
    selectedNode.value = editor.value.root
    dirty.value = false
    saveMessage.value = ''
  } catch (error) { loadError.value = error instanceof Error ? error.message : '导图加载失败' }
}
function createMind() {
  if (!currentFolder.value) return
  const root = { id: nodeId(), text: '中心主题', children: [] }
  editor.value = { name: '未命名导图', folderId: currentFolder.value.id, root, template: 'structure', theme: 'fresh-blue', version: '1.4.33' }
  selectedNode.value = root
  dirty.value = true
  saveMessage.value = ''
}
function markDirty() { dirty.value = true; saveMessage.value = '' }
function selectNode(node: MindNode) { selectedNode.value = node }
function startRename(node: MindNode) { selectedNode.value = node; nextTick(() => { renameInput.value?.focus(); renameInput.value?.select() }) }
function finishRename() { renameInput.value?.blur() }
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
function containsNode(root: MindNode, id: string): boolean { return root.id === id || root.children.some((child) => containsNode(child, id)) }
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
  if (!editor.value || !draggedNodeId.value || draggedNodeId.value === target.id || draggedNodeId.value === editor.value.root.id) return
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
  const walk = (node: MindNode, depth: number) => { rows.push({ node, depth }); node.children.forEach((child) => walk(child, depth + 1)) }
  if (editor.value) walk(editor.value.root, 0)
  return rows
})
type LayoutNode = { node: MindNode; x: number; y: number; width: number; height: number }
function buildLayout(root?: MindNode) {
  if (!root) return { nodes: [] as LayoutNode[], edges: [] as Array<{ id: string; path: string }>, width: 900, height: 540 }
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
  for (const parent of nodes) for (const childNode of parent.node.children) {
    const child = byId.get(childNode.id)
    if (!child) continue
    const x1 = parent.x + parent.width
    const y1 = parent.y + parent.height / 2
    const x2 = child.x
    const y2 = child.y + child.height / 2
    const midpoint = (x1 + x2) / 2
    edges.push({ id: `${parent.node.id}-${child.node.id}`, path: `M ${x1} ${y1} C ${midpoint} ${y1}, ${midpoint} ${y2}, ${x2} ${y2}` })
  }
  return { nodes, edges, width: Math.max(900, ...nodes.map((node) => node.x + node.width + 80)), height: Math.max(540, ...nodes.map((node) => node.y + node.height + 80)) }
}
const layout = computed(() => buildLayout(editor.value?.root))
const canvasViewBox = computed(() => `0 0 ${layout.value.width} ${layout.value.height}`)
const canvasStyle = computed(() => ({ width: `${layout.value.width}px`, height: `${layout.value.height}px`, transform: `scale(${zoom.value / 100})`, transformOrigin: 'top left' }))
function toKityNode(node: MindNode): KityNode { return { data: { id: node.id, text: node.text }, children: node.children.map(toKityNode) } }
async function saveMind() {
  if (!editor.value || saving.value) return
  saving.value = true
  saveMessage.value = ''
  const content = JSON.stringify({ root: toKityNode(editor.value.root), template: editor.value.template, theme: editor.value.theme, version: editor.value.version })
  try {
    const response = await api.post<{ id?: string }>('/jaxrs/mind/assemble/control/mind/save', {
      id: editor.value.id, name: editor.value.name.trim() || editor.value.root.text || '未命名导图', folderId: editor.value.folderId,
      description: editor.value.description || '', shared: Boolean(editor.value.shared), content,
    })
    editor.value.id = response.data?.id || editor.value.id
    dirty.value = false
    saveMessage.value = '已保存到服务器'
    await loadMinds(editor.value.folderId)
  } catch (error) { saveMessage.value = error instanceof Error ? `保存失败：${error.message}` : '保存失败'; dirty.value = true }
  finally { saving.value = false }
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