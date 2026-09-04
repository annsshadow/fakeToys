<template>
  <div class="pd">
    <!-- Header -->
    <div class="pd-header glass-card">
      <div class="pd-title">
        <h1>流程设计器</h1>
        <p class="subtitle">/jaxrs/processplatform/assemble/designer/* — 可视化流程编排</p>
      </div>
      <div class="pd-actions">
        <button class="btn" @click="undo" :disabled="!canUndo" title="撤销">↩</button>
        <button class="btn" @click="redo" :disabled="!canRedo" title="重做">↪</button>
        <button class="btn" @click="zoomIn" title="放大">🔍+</button>
        <button class="btn" @click="zoomOut" title="缩小">🔍-</button>
        <button class="btn" @click="fitCanvas" title="适配画布">⊞</button>
        <button class="btn btn-outline" @click="loadProcesses">🔄 刷新</button>
        <button class="btn btn-primary" @click="saveProcess" :disabled="!currentProcess">💾 保存</button>
      </div>
    </div>

    <div class="pd-body">
      <!-- Left: Process List -->
      <aside class="pd-sidebar glass-card">
        <div class="sb-header"><span>📋 流程列表</span><button class="btn-sm" @click="newProcess">+ 新建</button></div>
        <div class="sb-search"><input v-model="sbFilter" placeholder="搜索..." class="sb-input" /></div>
        <div class="sb-list">
          <div v-if="plLoading" class="sb-loading">加载中...</div>
          <div v-else-if="procList.length===0" class="sb-empty">暂无流程</div>
          <div v-for="p in filteredProc" :key="p.id" class="sb-item"
            :class="{active: currentProcess?.id===p.id}"
            @click="loadProcess(p)">
            <div class="si-icon">{{ p.status==='disabled'?'⏸':'▶' }}</div>
            <div class="si-info">
              <div class="si-name">{{ p.name||p.processName||'未命名' }}</div>
              <div class="si-meta">{{ p.flag||p.id }}</div>
            </div>
          </div>
        </div>
      </aside>

      <!-- Left: Node Palette -->
      <aside class="pd-palette glass-card" v-if="currentProcess">
        <div class="pal-title">基础节点</div>
        <div class="pal-grid">
          <div v-for="nt in nodeTypes" :key="nt.type"
            class="pal-item" draggable="true"
            @dragstart="onDragNode($event, nt)"
            @click="addNode(nt.type)">
            <span class="ni">{{ nt.icon }}</span><span class="nl">{{ nt.label }}</span>
          </div>
        </div>
        <div class="pal-sep"></div>
        <div class="pal-title">条件节点</div>
        <div class="pal-grid">
          <div class="pal-item" @click="addNode('gate_and')"><span class="ni">🔷</span><span class="nl">且网关</span></div>
          <div class="pal-item" @click="addNode('gate_or')"><span class="ni">🔶</span><span class="nl">或网关</span></div>
          <div class="pal-item" @click="addNode('gate_xor')"><span class="ni">🔹</span><span class="nl">异或网关</span></div>
          <div class="pal-item" @click="addNode('subprocess')"><span class="ni">📦</span><span class="nl">子流程</span></div>
        </div>
        <div class="pal-sep"></div>
        <div class="pal-title">操作</div>
        <div class="pal-grid">
          <div class="pal-item" @click="clearCanvas"><span class="ni">🗑</span><span class="nl">清空</span></div>
          <div class="pal-item" @click="autoLayout"><span class="ni">⊞</span><span class="nl">自动排列</span></div>
        </div>
      </aside>

      <!-- Center: Canvas -->
      <main class="pd-canvas glass-card" ref="canvasRef"
        @drop="onDropNode" @dragover.prevent
        @click.self="selectedNode=null; selectedEdge=null; tempEdge=null">
        <!-- Grid background -->
        <div class="canvas-bg" :style="{ backgroundSize: gridScale+'px '+gridScale+'px', backgroundPosition: panX+'px '+panY+'px' }"></div>

        <svg class="canvas-svg" :style="svgTransform" @mousedown="onCanvasMouseDown">
          <!-- Defs for arrow markers -->
          <defs>
            <marker id="arrowhead" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
              <polygon points="0 0, 10 3.5, 0 7" fill="var(--color-primary)" />
            </marker>
            <marker id="arrowhead-selected" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
              <polygon points="0 0, 10 3.5, 0 7" fill="var(--color-warning)" />
            </marker>
          </defs>

          <!-- Edges -->
          <g class="edges" :transform="edgeTransform">
            <path v-for="(edge, i) in processDef?.edges||[]" :key="edge.id"
              :d="computeEdgePath(edge)"
              :class="['edge-path', { selected: selectedEdge===i }]"
              :marker-end="selectedEdge===i ? 'url(#arrowhead-selected)' : 'url(#arrowhead)'"
              @click.stop="selectEdge(i)" />
          </g>

          <!-- Temp edge (while creating) -->
          <path v-if="tempEdge" :d="tempEdgePath()" class="edge-temp" marker-end="url(#arrowhead)" />

          <!-- Nodes -->
          <g class="nodes" :transform="nodeTransform">
            <g v-for="(node, i) in processDef?.nodes||[]" :key="node.id"
              :transform="`translate(${node.x},${node.y})`"
              :class="['node-group', { selected: selectedNode===i, dragging: isDraggingNode&&dragNodeIdx===i }]">

              <!-- Snapping guides (shown when near other nodes) -->
              <line v-if="snapX !== null" x1="0" y1="-20" x2="0" y2="(node.h||50)+20" stroke="var(--color-success)" stroke-width="1" stroke-dasharray="4,4" opacity="0.6" />
              <line v-if="snapY !== null" x1="-20" y1="0" x2="(node.w||120)+20" y2="0" stroke="var(--color-success)" stroke-width="1" stroke-dasharray="4,4" opacity="0.6" />

              <!-- Node body -->
              <rect :class="['node-body', node.type]"
                :width="node.w||120" :height="node.h||50" rx="8" />

              <!-- Node icon + label -->
              <text :x="(node.w||120)/2" :y="(node.h||50)/2-4"
                text-anchor="middle" class="node-label">{{ node.label || getNodeLabel(node.type) }}</text>
              <text :x="(node.w||120)/2" :y="(node.h||50)/2+12"
                text-anchor="middle" class="node-sublabel" font-size="10">{{ node.assignee || '' }}</text>

              <!-- In port (left) -->
              <circle v-if="node.type!=='start'" cx="0" :cy="(node.h||50)/2" r="6"
                class="port port-in" @mousedown.stop="onPortMouseDown($event, i, 'in')" />
              <!-- Out port (right) -->
              <circle v-if="node.type!=='end'" cx="(node.w||120)" :cy="(node.h||50)/2" r="6"
                class="port port-out" @mousedown.stop="onPortMouseDown($event, i, 'out')" />

              <!-- Conditional badge -->
              <rect v-if="node.condition" x="4" y="4" width="8" height="8" rx="2" fill="var(--color-warning)" />
              <text v-if="node.condition" x="8" y="11" font-size="6" fill="#000" text-anchor="middle">?</text>
            </g>
          </g>
        </svg>

        <!-- Canvas hint -->
        <div class="canvas-hint">
          <span>拖拽右侧节点到画布 | 从端口拖出创建连线 | Ctrl+滚轮缩放</span>
        </div>
      </main>

      <!-- Right: Properties -->
      <aside class="pd-props glass-card" v-if="currentProcess">
        <!-- Node properties -->
        <div v-if="selectedNode!==null" class="props-section">
          <div class="props-title">
            <span>节点属性</span>
            <span class="props-badge">{{ getNodeProp('type') }}</span>
          </div>
          <div class="props-body">
            <div class="pg"><label>节点标签</label><input :value="getNodeProp('label')" @input="_setNodeProp('label',$event.target.value)" class="pi" /></div>
            <div class="pg"><label>负责人</label><input :value="getNodeProp('assignee')" @input="_setNodeProp('assignee',$event.target.value)" class="pi" placeholder="如: manager_zhang" /></div>
            <div class="pg"><label>条件表达式</label><input :value="getNodeProp('condition')" @input="_setNodeProp('condition',$event.target.value)" class="pi" placeholder="如: amount > 1000" /></div>
            <div class="pg"><label>超时(分钟)</label><input :value="getNodeProp('timeout')" type="number" @input="_setNodeProp('timeout',+$event.target.value)" class="pi" /></div>
            <div class="pg"><label>优先级</label>
              <select :value="getNodeProp('priority')" @change="_setNodeProp('priority',$event.target.value)" class="pi">
                <option value="">默认</option><option value="high">高</option><option value="medium">中</option><option value="low">低</option>
              </select>
            </div>
            <div class="pg"><label>X</label><input :value="getNodeProp('x')" type="number" @input="_setNodeProp('x',+$event.target.value)" class="pi" /></div>
            <div class="pg"><label>Y</label><input :value="getNodeProp('y')" type="number" @input="_setNodeProp('y',+$event.target.value)" class="pi" /></div>
            <div class="pg"><label>宽</label><input :value="getNodeProp('w')" type="number" @input="_setNodeProp('w',+$event.target.value)" class="pi" min="80" max="300" /></div>
            <div class="pg"><label>高</label><input :value="getNodeProp('h')" type="number" @input="_setNodeProp('h',+$event.target.value)" class="pi" min="40" max="120" /></div>
            <button class="btn-del-sm" @click="deleteNode(selectedNode)">🗑 删除节点</button>
          </div>
        </div>

        <!-- Edge properties -->
        <div v-if="selectedEdge!==null" class="props-section">
          <div class="props-title"><span>连线属性</span></div>
          <div class="props-body">
            <div class="pg"><label>标签</label><input :value="getEdgeProp('label')" @input="_setEdgeProp('label',$event.target.value)" class="pi" /></div>
            <div class="pg"><label>流向</label><span class="pv">{{ getEdgeFromLabel() }} → {{ getEdgeToLabel() }}</span></div>
            <button class="btn-del-sm" @click="deleteEdge(selectedEdge)">🗑 删除连线</button>
          </div>
        </div>

        <div v-if="selectedNode===null && selectedEdge===null" class="props-empty">
          <p>选择节点或连线编辑属性</p>
        </div>
      </aside>
    </div>

    <!-- New Process Modal -->
    <div v-if="showNewModal" class="modal-overlay" @click.self="showNewModal=false">
      <div class="modal glass-card">
        <h3>新建流程</h3>
        <div class="fg"><label>流程名称</label><input v-model="newForm.name" class="fi" placeholder="如: 请假审批流程" /></div>
        <div class="fg"><label>唯一标识</label><input v-model="newForm.flag" class="fi" placeholder="如: leave_approval_v2" /></div>
        <div class="fg"><label>描述</label><textarea v-model="newForm.desc" class="fta" rows="2"></textarea></div>
        <div class="ma">
          <button class="bc" @click="showNewModal=false">取消</button>
          <button class="bs" :disabled="!newForm.name" @click="createProcess">创建</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useQuery, useMutation, useQueryClient } from '@tanstack/vue-query'
import { api } from '@oa4rust/sdk'

// ── Types ────────────────────────────────────────────────────────────
interface PDNode {
  id: string; type: string; label?: string; x: number; y: number
  w?: number; h?: number; assignee?: string; condition?: string
  timeout?: number; priority?: string
}
interface PDEdge { id: string; from: string; to: string; label?: string; condition?: string }
interface ProcDef { id?: string; name: string; flag: string; desc?: string; status?: string; config?: { nodes: PDNode[]; edges: PDEdge[] } }

// ── Constants ─────────────────────────────────────────────────────────
const GRID_SIZE = 20
const SNAP_THRESHOLD = 15

const nodeTypes = [
  { type: 'start',    label: '开始', icon: '🟢' },
  { type: 'task',     label: '任务', icon: '📋' },
  { type: 'approval', label: '审批', icon: '✅' },
  { type: 'timer',    label: '定时', icon: '⏱️' },
  { type: 'end',      label: '结束', icon: '🔴' },
]

// ── State ─────────────────────────────────────────────────────────────
const plLoading = ref(false), sbFilter = ref('')
const currentProcess = ref<ProcDef|null>(null)
const processDef = ref<{nodes: PDNode[]; edges: PDEdge[]}>({ nodes: [], edges: [] })
const selectedNode = ref<number|null>(null)
const selectedEdge = ref<number|null>(null)
const showNewModal = ref(false), newForm = ref({ name: '', flag: '', desc: '' })
const canvasRef = ref<HTMLElement|null>(null)
const panX = ref(0), panY = ref(0), zoom = ref(1)

// Undo/Redo
const history = ref<{nodes: PDNode[]; edges: PDEdge[]}[]>([])
const histIdx = ref(-1)
const canUndo = computed(() => histIdx.value > 0)
const canRedo = computed(() => histIdx.value < history.value.length - 1)

// Drag state
const isDraggingNode = ref(false)
const dragNodeIdx = ref<number|null>(null)
const dragOffset = ref({ x: 0, y: 0 })
const snapX = ref<number|null>(null)
const snapY = ref<number|null>(null)

// Temp edge (being created)
const tempEdge = ref<{ from: number; fromPort: 'out'|'in'; startX: number; startY: number; endX: number; endY: number }|null>(null)

// ── Computed ──────────────────────────────────────────────────────────
const filteredProc = computed(() =>
  sbFilter.value
    ? procList.value.filter(p => (p.name||'').toLowerCase().includes(sbFilter.value.toLowerCase()) || (p.flag||'').toLowerCase().includes(sbFilter.value.toLowerCase()))
    : procList.value
)

const svgTransform = computed(() => ({ transform: `translate(${panX.value}px,${panY.value}px) scale(${zoom.value})`, transformOrigin: '0 0' }))
const edgeTransform = computed(() => ({ transform: `translate(${-panX.value}px,${-panY.value}px) scale(${1/zoom.value})` }))
const nodeTransform = computed(() => ({ transform: `translate(${-panX.value}px,${-panY.value}px) scale(${1/zoom.value})` }))
const gridScale = computed(() => GRID_SIZE * zoom.value)

// ── Process List ──────────────────────────────────────────────────────
const { data: procData } = useQuery({ queryKey: ['pd','list'], queryFn: async () => {
  plLoading.value = true
  try { const r: any = await api.get('/jaxrs/processplatform/assemble/designer/process/list'); return r?.data?.list ?? r?.data ?? [] }
  finally { plLoading.value = false }
}})
const procList = ref<ProcDef[]>(procData.value ?? [])

// ── History ───────────────────────────────────────────────────────────
function pushHistory() {
  if (!processDef.value) return
  const snap = JSON.parse(JSON.stringify(processDef.value))
  history.value = history.value.slice(0, histIdx.value + 1)
  history.value.push(snap)
  histIdx.value = history.value.length - 1
}

function undo() {
  if (histIdx.value <= 0) return
  histIdx.value--
  processDef.value = JSON.parse(JSON.stringify(history.value[histIdx.value]))
  selectedNode.value = null
}

function redo() {
  if (histIdx.value >= history.value.length - 1) return
  histIdx.value++
  processDef.value = JSON.parse(JSON.stringify(history.value[histIdx.value]))
  selectedNode.value = null
}

// ── Node CRUD ─────────────────────────────────────────────────────────
function genId() { return 'n_' + Date.now() + '_' + Math.random().toString(36).slice(2,6) }
function genEdgeId() { return 'e_' + Date.now() + '_' + Math.random().toString(36).slice(2,6) }

function getNodeLabel(type: string) {
  const labels: Record<string, string> = { start:'开始', end:'结束', task:'任务', approval:'审批', timer:'定时', gate_and:'且网关', gate_or:'或网关', gate_xor:'异或网关', subprocess:'子流程' }
  return labels[type] || type
}

function addNode(type: string) {
  if (!processDef.value) return
  const w = type.includes('gate') ? 100 : 120
  const h = type === 'approval' ? 70 : 50
  processDef.value.nodes.push({
    id: genId(), type, label: getNodeLabel(type),
    x: 100 + Math.random() * 200, y: 80 + Math.random() * 100,
    w, h
  })
  pushHistory()
}

function deleteNode(i: number) {
  if (!processDef.value) return
  const id = processDef.value.nodes[i].id
  processDef.value.nodes.splice(i, 1)
  processDef.value.edges = (processDef.value.edges||[]).filter(e => e.from !== id && e.to !== id)
  if (selectedNode.value === i) selectedNode.value = null
  else if (selectedNode.value !== null && selectedNode.value > i) selectedNode.value--
  pushHistory()
}

function clearCanvas() {
  if (!processDef.value || !confirm('清空画布，所有节点和连线将删除？')) return
  processDef.value = { nodes: [], edges: [] }
  selectedNode.value = null; selectedEdge.value = null
  pushHistory()
}

function autoLayout() {
  if (!processDef.value || processDef.value.nodes.length === 0) return
  const cols = Math.ceil(Math.sqrt(processDef.value.nodes.length))
  processDef.value.nodes.forEach((n, i) => {
    n.x = 80 + (i % cols) * (n.w || 120) * 1.3
    n.y = 80 + Math.floor(i / cols) * ((n.h || 50) * 1.4)
  })
  // Re-layout edges to follow topological order
  pushHistory()
}

// ── Property access (for template - non-v-model) ─────────────────────
function getNodeProp(prop: string): any {
  if (selectedNode.value === null || !processDef.value?.nodes[selectedNode.value]) return ''
  return processDef.value.nodes[selectedNode.value][prop as keyof PDNode] ?? ''
}
function _setNodeProp(prop: string, val: any) {
  if (selectedNode.value === null || !processDef.value) return
  ;(processDef.value.nodes[selectedNode.value] as any)[prop] = val
}
function getEdgeProp(prop: string): any {
  if (selectedEdge.value === null || !processDef.value?.edges[selectedEdge.value]) return ''
  return processDef.value.edges[selectedEdge.value][prop as keyof PDEdge] ?? ''
}
function _setEdgeProp(prop: string, val: any) {
  if (selectedEdge.value === null || !processDef.value) return
  ;(processDef.value.edges[selectedEdge.value] as any)[prop] = val
}
function getEdgeFromLabel() {
  if (selectedEdge.value === null || !processDef.value) return ''
  const edge = processDef.value.edges[selectedEdge.value]
  const node = processDef.value.nodes.find(n => n.id === edge.from)
  return node?.label || node?.id?.slice(0,8) || '?'
}
function getEdgeToLabel() {
  if (selectedEdge.value === null || !processDef.value) return ''
  const edge = processDef.value.edges[selectedEdge.value]
  const node = processDef.value.nodes.find(n => n.id === edge.to)
  return node?.label || node?.id?.slice(0,8) || '?'
}

// ── Edge CRUD ─────────────────────────────────────────────────────────
function createEdge(fromId: string, toId: string) {
  if (!processDef.value) return
  // Check duplicate
  const exists = processDef.value.edges.some(e => e.from === fromId && e.to === toId)
  if (exists) return
  processDef.value.edges.push({ id: genEdgeId(), from: fromId, to: toId })
  pushHistory()
}

function deleteEdge(i: number) {
  if (!processDef.value) return
  processDef.value.edges.splice(i, 1)
  selectedEdge.value = null
  pushHistory()
}

function selectEdge(i: number) { selectedEdge.value = i; selectedNode.value = null }

// ── Edge path computation ─────────────────────────────────────────────
function getNodeCenter(node: PDNode) {
  return { x: node.x + (node.w || 120) / 2, y: node.y + (node.h || 50) / 2 }
}
function getNodePort(node: PDNode, port: 'in'|'out') {
  const w = node.w || 120, h = node.h || 50
  if (port === 'in') return { x: node.x, y: node.y + h / 2 }
  return { x: node.x + w, y: node.y + h / 2 }
}

function computeEdgePath(edge: PDEdge): string {
  if (!processDef.value) return ''
  const from = processDef.value.nodes.find(n => n.id === edge.from)
  const to = processDef.value.nodes.find(n => n.id === edge.to)
  if (!from || !to) return ''
  const fromPort = getNodePort(from, 'out')
  const toPort = getNodePort(to, 'in')
  const dx = Math.abs(toPort.x - fromPort.x)
  const cx = Math.max(dx * 0.5, 60)
  return `M ${fromPort.x} ${fromPort.y} C ${fromPort.x + cx} ${fromPort.y}, ${toPort.x - cx} ${toPort.y}, ${toPort.x} ${toPort.y}`
}

function tempEdgePath(): string {
  if (!tempEdge.value) return ''
  const { startX, startY, endX, endY } = tempEdge.value
  const from = processDef.value?.nodes[tempEdge.value.from]
  const toNode = processDef.value?.nodes[tempEdge.value.from] // same as from for temp
  if (!from) return ''
  const fromPort = getNodePort(from, tempEdge.value.fromPort === 'out' ? 'out' : 'in')
  const cx = Math.max(Math.abs(endX - fromPort.x) * 0.5, 60)
  const cp1x = fromPort.x + (tempEdge.value.fromPort === 'out' ? cx : -cx)
  const cp2x = endX - (tempEdge.value.fromPort === 'out' ? cx : -cx)
  return `M ${fromPort.x} ${fromPort.y} C ${cp1x} ${fromPort.y}, ${cp2x} ${endY}, ${endX} ${endY}`
}

// ── Drag: Node ────────────────────────────────────────────────────────
function onNodeMouseDown(e: MouseEvent, i: number) {
  if (!processDef.value) return
  isDraggingNode.value = true
  dragNodeIdx.value = i
  const node = processDef.value.nodes[i]
  dragOffset.value = {
    x: (e.clientX - panX.value) / zoom.value - node.x,
    y: (e.clientY - panY.value) / zoom.value - node.y
  }
  selectedNode.value = i
  selectedEdge.value = null
}

function onNodeMouseMove(e: MouseEvent) {
  if (!isDraggingNode.value || dragNodeIdx.value === null || !processDef.value) return
  const idx = dragNodeIdx.value
  const rawX = (e.clientX - panX.value) / zoom.value - dragOffset.value.x
  const rawY = (e.clientY - panY.value) / zoom.value - dragOffset.value.y
  // Snap to grid
  const snappedX = Math.round(rawX / GRID_SIZE) * GRID_SIZE
  const snappedY = Math.round(rawY / GRID_SIZE) * GRID_SIZE
  processDef.value.nodes[idx].x = snappedX
  processDef.value.nodes[idx].y = snappedY

  // Compute snap to other nodes
  let nearSnapX: number|null = null, nearSnapY: number|null = null
  for (let j = 0; j < processDef.value.nodes.length; j++) {
    if (j === idx) continue
    const other = processDef.value.nodes[j]
    const ow = other.w || 120, oh = other.h || 50
    // Horizontal snap
    if (Math.abs(snappedX - other.x) < SNAP_THRESHOLD) nearSnapX = other.x
    if (Math.abs(snappedX - (other.x + ow)) < SNAP_THRESHOLD) nearSnapX = other.x + ow
    if (Math.abs(snappedX - (other.x + ow/2)) < SNAP_THRESHOLD) nearSnapX = other.x + ow/2
    // Vertical snap
    if (Math.abs(snappedY - other.y) < SNAP_THRESHOLD) nearSnapY = other.y
    if (Math.abs(snappedY - (other.y + oh)) < SNAP_THRESHOLD) nearSnapY = other.y + oh
    if (Math.abs(snappedY - (other.y + oh/2)) < SNAP_THRESHOLD) nearSnapY = other.y + oh/2
  }
  snapX.value = nearSnapX
  snapY.value = nearSnapY
}

function onNodeMouseUp() {
  if (isDraggingNode.value && processDef.value && dragNodeIdx.value !== null) {
    // Apply final snap
    const node = processDef.value.nodes[dragNodeIdx.value]
    if (snapX.value !== null) node.x = snapX.value
    if (snapY.value !== null) node.y = snapY.value
    pushHistory()
  }
  isDraggingNode.value = false
  dragNodeIdx.value = null
  snapX.value = null
  snapY.value = null
}

// ── Drag: Edge creation from port ─────────────────────────────────────
function onPortMouseDown(e: MouseEvent, nodeIdx: number, port: 'in'|'out') {
  e.stopPropagation()
  if (!processDef.value) return
  const node = processDef.value.nodes[nodeIdx]
  const portPos = getNodePort(node, port)
  tempEdge.value = {
    from: nodeIdx, fromPort: port,
    startX: portPos.x, startY: portPos.y,
    endX: portPos.x, endY: portPos.y
  }

  const onMove = (ev: MouseEvent) => {
    if (!tempEdge.value) return
    tempEdge.value.endX = (ev.clientX - panX.value) / zoom.value
    tempEdge.value.endY = (ev.clientY - panY.value) / zoom.value
  }
  const onUp = (ev: MouseEvent) => {
    document.removeEventListener('mousemove', onMove)
    document.removeEventListener('mouseup', onUp)
    if (!tempEdge.value || !processDef.value) { tempEdge.value = null; return }

    // Find target node near mouse
    let targetIdx: number|null = null
    const mx = (ev.clientX - panX.value) / zoom.value
    const my = (ev.clientY - panY.value) / zoom.value
    for (let i = 0; i < processDef.value.nodes.length; i++) {
      const n = processDef.value.nodes[i]
      if (i === tempEdge.value!.from) continue
      if (mx >= n.x && mx <= n.x + (n.w||120) && my >= n.y && my <= n.y + (n.h||50)) {
        targetIdx = i; break
      }
    }

    if (targetIdx !== null) {
      const fromPort = tempEdge.value!.fromPort === 'out' ? 'out' : 'in'
      const toPort = fromPort === 'out' ? 'in' : 'out'
      const fromNode = processDef.value.nodes[tempEdge.value!.from]
      const toNode = processDef.value.nodes[targetIdx]
      // Only connect out→in
      if (fromPort === 'out' && toPort === 'in') {
        createEdge(fromNode.id, toNode.id)
      } else if (fromPort === 'in' && toPort === 'out') {
        createEdge(toNode.id, fromNode.id)
      }
    }
    tempEdge.value = null
  }
  document.addEventListener('mousemove', onMove)
  document.addEventListener('mouseup', onUp)
}

// ── Drag: Node (from rect, not port) ─────────────────────────────────
function onNodeRectMouseDown(e: MouseEvent, i: number) {
  e.stopPropagation()
  onNodeMouseDown(e, i)
}

// ── Canvas drag (pan) ─────────────────────────────────────────────────
let isPanning = false, panStart = { x: 0, y: 0 }
function onCanvasMouseDown(e: MouseEvent) {
  if (e.button !== 0) return
  isPanning = true
  panStart = { x: e.clientX - panX.value, y: e.clientY - panY.value }
  const onMove = (ev: MouseEvent) => {
    if (!isPanning) return
    panX.value = ev.clientX - panStart.x
    panY.value = ev.clientY - panStart.y
  }
  const onUp = () => { isPanning = false }
  document.addEventListener('mousemove', onMove)
  document.addEventListener('mouseup', onUp)
}

// ── Zoom ──────────────────────────────────────────────────────────────
function zoomIn() { zoom.value = Math.min(3, zoom.value + 0.1) }
function zoomOut() { zoom.value = Math.max(0.3, zoom.value - 0.1) }
function fitCanvas() { zoom.value = 1; panX.value = 0; panY.value = 0 }

// ── Drag from palette ─────────────────────────────────────────────────
function onDragNode(e: DragEvent, nt: { type: string }) {
  ;(e.dataTransfer as any)?.setData('nodeType', nt.type)
}
function onDropNode(e: DragEvent) {
  e.preventDefault()
  const type: string = (e.dataTransfer as any)?.getData('nodeType')
  if (!type || !processDef.value) return
  const rect = (e.currentTarget as HTMLElement).getBoundingClientRect()
  const x = (e.clientX - rect.left - panX.value) / zoom.value
  const y = (e.clientY - rect.top - panY.value) / zoom.value
  const snappedX = Math.round(x / GRID_SIZE) * GRID_SIZE
  const snappedY = Math.round(y / GRID_SIZE) * GRID_SIZE
  const w = type.includes('gate') ? 100 : 120
  const h = type === 'approval' ? 70 : 50
  processDef.value.nodes.push({ id: genId(), type, label: getNodeLabel(type), x: snappedX - w/2, y: snappedY - h/2, w, h })
  pushHistory()
}

// ── Process CRUD ──────────────────────────────────────────────────────
async function loadProcess(p: ProcDef) {
  try {
    const r: any = await api.get(`/jaxrs/processplatform/assemble/designer/process/${p.id}`)
    const data = r?.data ?? p
    currentProcess.value = data
    processDef.value = data.config ?? { nodes: [], edges: [] }
    // Ensure at least start/end nodes
    if (!processDef.value.nodes.length) {
      processDef.value = {
        nodes: [
          { id: genId(), type: 'start', label: '开始', x: 80, y: 120, w: 100, h: 50 },
          { id: genId(), type: 'task', label: '审批任务', x: 300, y: 100, w: 120, h: 50, assignee: '' },
          { id: genId(), type: 'end', label: '结束', x: 520, y: 120, w: 100, h: 50 },
        ],
        edges: [
          { id: genEdgeId(), from: processDef.value.nodes[0].id, to: processDef.value.nodes[1].id },
          { id: genEdgeId(), from: processDef.value.nodes[1].id, to: processDef.value.nodes[2].id },
        ]
      }
    }
    selectedNode.value = null; selectedEdge.value = null
    history.value = []; histIdx.value = -1; pushHistory()
  } catch {
    currentProcess.value = { ...p, name: p.name, flag: p.flag, config: { nodes: [], edges: [] } }
    processDef.value = { nodes: [], edges: [] }
  }
}
function newProcess() { newForm.value = { name: '', flag: '', desc: '' }; showNewModal.value = true }
const savePM = useMutation({
  mutationFn: async (data: any) => {
    if (currentProcess.value?.id) return api.put(`/jaxrs/processplatform/assemble/designer/process/${currentProcess.value!.id}`, data)
    return api.post('/jaxrs/processplatform/assemble/designer/process', data)
  },
  onSuccess: () => { showNewModal.value = false; loadProcesses() }
})
async function createProcess() {
  if (!newForm.value.name.trim()) return
  savePM.mutate({ name: newForm.value.name, flag: newForm.value.flag, description: newForm.value.desc, config: processDef.value })
}
async function saveProcess() {
  if (!currentProcess.value) return
  try {
    await api.put(`/jaxrs/processplatform/assemble/designer/process/${currentProcess.value.id}`, {
      name: currentProcess.value.name, flag: currentProcess.value.flag,
      description: currentProcess.value.desc, config: processDef.value
    })
    alert('保存成功')
  } catch (e: any) { alert('保存失败: ' + (e?.message ?? '')) }
}
async function loadProcesses() {
  try { const r: any = await api.get('/jaxrs/processplatform/assemble/designer/process/list'); procList.value = r?.data?.list ?? r?.data ?? [] }
  catch { procList.value = [] }
}

// ── Lifecycle ─────────────────────────────────────────────────────────
onMounted(() => {
  document.addEventListener('mousemove', (e) => { onNodeMouseMove(e); if(isPanning) onCanvasMouseDown(e) })
  document.addEventListener('mouseup', () => { onNodeMouseUp(); isPanning = false })
  loadProcesses()
})
onUnmounted(() => {
  document.removeEventListener('mousemove', () => {})
  document.removeEventListener('mouseup', () => {})
})
</script>

<style scoped>
.pd{display:flex;flex-direction:column;height:100%}
.pd-header{display:flex;align-items:center;justify-content:space-between;padding:10px 16px;flex-shrink:0}
.pd-title h1{font-family:'Orbitron',sans-serif;font-size:18px;color:var(--color-primary);margin:0 0 2px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:11px;color:var(--text-muted);margin:0;font-family:'JetBrains Mono',monospace}
.pd-actions{display:flex;gap:6px;flex-wrap:wrap}
.btn{padding:5px 12px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);cursor:pointer;font-size:12px}
.btn:hover{border-color:var(--color-primary);color:var(--color-primary)}
.btn:disabled{opacity:0.3;cursor:not-allowed}
.btn-primary{background:var(--color-primary);color:#000;border-color:var(--color-primary);font-weight:600}
.btn-outline{background:transparent}
.pd-body{display:flex;flex:1;overflow:hidden}
/* Sidebar */
.pd-sidebar{width:200px;flex-shrink:0;display:flex;flex-direction:column;border-right:1px solid var(--border-color)}
.sb-header{display:flex;align-items:center;justify-content:space-between;padding:8px 10px;border-bottom:1px solid var(--border-color);font-size:13px;font-weight:600;color:var(--color-primary)}
.btn-sm{padding:3px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:11px}
.sb-search{padding:6px 8px}
.sb-input{width:100%;padding:5px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:12px;outline:none;box-sizing:border-box}
.sb-list{flex:1;overflow-y:auto;padding:4px}
.sb-loading,.sb-empty{padding:16px;text-align:center;color:var(--text-muted);font-size:12px}
.sb-item{display:flex;align-items:center;gap:6px;padding:8px;border-radius:var(--radius-sm);cursor:pointer;margin-bottom:2px}
.sb-item:hover{background:var(--bg-hover)}
.sb-item.active{background:var(--color-primary-soft);border-left:3px solid var(--color-primary)}
.si-icon{font-size:16px;flex-shrink:0}
.si-info{flex:1;min-width:0}
.si-name{font-size:13px;color:var(--text-primary);overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.si-meta{font-size:10px;color:var(--text-muted);font-family:'JetBrains Mono',monospace}
/* Palette */
.pd-palette{width:140px;flex-shrink:0;padding:12px;border-right:1px solid var(--border-color)}
.pal-title{font-size:11px;color:var(--text-muted);text-transform:uppercase;letter-spacing:1px;margin:8px 0 6px;font-weight:600}
.pal-sep{height:1px;background:var(--border-color);margin:8px 0}
.pal-grid{display:grid;grid-template-columns:1fr 1fr;gap:6px}
.pal-item{display:flex;flex-direction:column;align-items:center;padding:10px 4px;border-radius:var(--radius-md);border:1px solid var(--border-color);cursor:grab;background:var(--bg-elevated);transition:all var(--transition-fast)}
.pal-item:hover{border-color:var(--color-primary);background:var(--color-primary-soft)}
.ni{font-size:20px}
.nl{font-size:10px;color:var(--text-muted);margin-top:4px;text-align:center}
/* Canvas */
.pd-canvas{flex:1;position:relative;overflow:hidden;min-width:0;background:var(--bg-surface)}
.canvas-bg{position:absolute;inset:0;pointer-events:none;
  background-image:radial-gradient(circle,var(--border-color) 1px,transparent 1px);
  background-repeat:repeat}
.canvas-svg{position:absolute;inset:0;width:100%;height:100%;cursor:default}
.canvas-svg.panning{cursor:move}
.canvas-hint{position:absolute;bottom:8px;left:50%;transform:translateX(-50%);font-size:11px;color:var(--text-muted);pointer-events:none;white-space:nowrap}
/* Edges */
.edge-path{fill:none;stroke:var(--color-primary);stroke-width:2;cursor:pointer;opacity:0.7;transition:all 0.15s}
.edge-path:hover{stroke:var(--color-warning);stroke-width:3;opacity:1}
.edge-path.selected{stroke:var(--color-warning);stroke-width:3;opacity:1}
.edge-temp{fill:none;stroke:var(--color-secondary);stroke-width:2;stroke-dasharray:6,3;opacity:0.8}
/* Nodes */
.node-group{cursor:move;transition:filter 0.15s}
.node-group.selected .node-body{filter:drop-shadow(0 0 10px var(--color-primary));stroke-width:3}
.node-group.dragging{opacity:0.8}
.node-body{stroke:var(--border-color);stroke-width:2;transition:all 0.15s}
.node-body.start{fill:rgba(16,185,129,.2);stroke:var(--color-success)}
.node-body.task{fill:rgba(0,212,255,.15);stroke:var(--color-primary)}
.node-body.approval{fill:rgba(99,102,241,.15);stroke:rgb(99,102,241)}
.node-body.timer{fill:rgba(245,158,11,.15);stroke:var(--color-warning)}
.node-body.end{fill:rgba(239,68,68,.2);stroke:var(--color-danger)}
.node-body.gate_and,.node-body.gate_or,.node-body.gate_xor{fill:rgba(245,158,11,.15);stroke:var(--color-warning)}
.node-body.subprocess{fill:rgba(168,85,247,.15);stroke:rgb(168,85,247)}
.node-label{fill:var(--text-primary);font-size:12px;font-weight:600;pointer-events:none}
.node-sublabel{fill:var(--text-muted);font-size:9px;pointer-events:none}
.port{fill:var(--color-primary);stroke:var(--bg-surface);stroke-width:2;cursor:crosshair;transition:all 0.15s}
.port:hover{r:8;fill:var(--color-warning)}
/* Props */
.pd-props{width:240px;flex-shrink:0;padding:12px;border-left:1px solid var(--border-color);overflow-y:auto}
.props-section{margin-bottom:16px}
.props-title{display:flex;align-items:center;gap:6px;padding-bottom:8px;border-bottom:1px solid var(--border-color);margin-bottom:10px}
.props-title span:first-child{font-size:11px;color:var(--text-muted);text-transform:uppercase;letter-spacing:1px;font-weight:600}
.props-badge{font-size:10px;padding:2px 6px;border-radius:var(--radius-sm);background:var(--color-primary-soft);color:var(--color-primary);font-family:'JetBrains Mono',monospace}
.props-body{display:flex;flex-direction:column;gap:8px}
.pg{display:flex;flex-direction:column;gap:3px}
.pg label{font-size:11px;color:var(--text-muted);text-transform:uppercase;letter-spacing:0.5px}
.pi{padding:6px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:12px;outline:none}
.pi:focus{border-color:var(--color-primary)}
.pv{font-size:12px;color:var(--color-primary);font-family:'JetBrains Mono',monospace}
.btn-del-sm{padding:6px 10px;border-radius:var(--radius-sm);border:1px solid var(--color-danger);background:transparent;color:var(--color-danger);cursor:pointer;font-size:12px;width:100%;margin-top:8px}
.props-empty{padding:20px;text-align:center;color:var(--text-muted);font-size:12px}
/* Modal */
.modal-overlay{position:fixed;inset:0;background:rgba(0,0,0,.6);display:flex;align-items:center;justify-content:center;z-index:200}
.modal{padding:24px;width:480px;max-width:90vw;display:flex;flex-direction:column;gap:12px}
.modal h3{font-size:16px;color:var(--color-primary);margin:0}
.fg{display:flex;flex-direction:column;gap:4px}
.fg label{font-size:12px;color:var(--text-muted)}
.fi,.fta{padding:8px 12px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);outline:none;font-size:13px;box-sizing:border-box}
.fta{resize:vertical;font-family:inherit}
.ma{display:flex;justify-content:flex-end;gap:8px;margin-top:8px}
.bc{padding:8px 16px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:transparent;color:var(--text-primary);cursor:pointer}
.bs{padding:8px 16px;border-radius:var(--radius-md);border:none;background:var(--color-primary);color:#000;cursor:pointer;font-weight:600}
.bs:disabled{opacity:0.4;cursor:not-allowed}
</style>
