<template>
  <div class="pd">
    <!-- Header -->
    <div class="pd-header glass-card">
      <div class="pd-title"><h1>流程设计器</h1><p class="subtitle">/jaxrs/processplatform/assemble/designer/*</p></div>
      <div class="pd-actions">
        <button class="btn" @click="undo" :disabled="!canUndo">↩ 撤销</button>
        <button class="btn" @click="redo" :disabled="!canRedo">↪ 重做</button>
        <button class="btn" @click="zoomIn">🔍+</button>
        <button class="btn" @click="zoomOut">🔍-</button>
        <button class="btn" @click="fitCanvas">⊞ 适配</button>
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
          <div v-for="p in filteredProc" :key="p.id" class="sb-item" :class="{active:currentProcess?.id===p.id}" @click="loadProcess(p)">
            <div class="si-icon">{{ p.status==='disabled'?'⏸':'▶' }}</div>
            <div class="si-info"><div class="si-name">{{ p.name||p.processName||'未命名' }}</div><div class="si-meta">{{ p.flag||p.id }}</div></div>
          </div>
        </div>
      </aside>

      <!-- Left: Node Palette -->
      <aside class="pd-palette glass-card" v-if="currentProcess">
        <div class="pal-title">节点类型</div>
        <div class="pal-grid">
          <div v-for="nt in nodeTypes" :key="nt.type" class="pal-item" draggable="true"
            @dragstart="onDragNode($event, nt)" @click="addNode(nt.type)">
            <span class="ni">{{ nt.icon }}</span><span class="nl">{{ nt.label }}</span>
          </div>
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
        @click.self="selectedNode=null">
        <div v-if="!currentProcess" class="canvas-empty">
          <div class="ce-icon">⚙️</div>
          <p>选择或新建流程开始设计</p>
          <button class="btn btn-primary" @click="newProcess">+ 新建流程</button>
        </div>
        <div v-else class="canvas-wrap">
          <svg class="canvas-svg" :style="svgStyle">
            <!-- Edges -->
            <g class="edges">
              <path v-for="(edge,i) in processDef?.edges||[]" :key="i"
                :d="edgePath(edge)" :class="['edge', {selected: selectedEdge===i}]"
                @click.stop="selectedEdge=i" />
            </g>
            <!-- Nodes -->
            <g class="nodes" :transform="`translate(${panX},${panY}) scale(${zoom})`">
              <g v-for="(node,i) in processDef?.nodes||[]" :key="node.id"
                :transform="`translate(${node.x},${node.y})`"
                :class="['node', {selected: selectedNode===i}]"
                @click.stop="selectNode(i)" @mousedown="onNodeDragStart($event, i)">
                <rect :class="['node-bg', node.type]" :width="node.w||120" :height="node.h||40" rx="6" />
                <text :x="(node.w||120)/2" :y="(node.h||40)/2+5" text-anchor="middle" class="node-label">{{ node.label||node.type }}</text>
                <!-- Port dots -->
                <circle v-if="node.type!=='end'" cx="0" :cy="(node.h||40)/2" r="4" class="port port-out" />
                <circle v-if="node.type!=='start'" cx="(node.w||120)" :cy="(node.h||40)/2" r="4" class="port port-in" />
              </g>
            </g>
          </svg>
          <div class="canvas-hint">提示：拖拽节点到画布，点击节点编辑属性</div>
        </div>
      </main>

      <!-- Right: Properties -->
      <aside class="pd-props glass-card" v-if="currentProcess && (selectedNode!==null || selectedEdge!==null)">
        <div v-if="selectedNode!==null" class="props-section">
          <div class="props-title">节点属性</div>
          <div class="props-body">
            <div class="pg"><label>类型</label><span class="pv">{{ _getNodeProp('type') }}</span></div>
            <div class="pg"><label>标签</label><input :value="_getNodeProp('label')" @input="_setNodeProp('label', $event.target.value)" class="pi" /></div>
            <div class="pg"><label>X</label><input :value="_getNodeProp('x')" @input="_setNodeProp('x', +$event.target.value)" type="number" class="pi" /></div>
            <div class="pg"><label>Y</label><input :value="_getNodeProp('y')" @input="_setNodeProp('y', +$event.target.value)" type="number" class="pi" /></div>
            <div class="pg"><label>宽</label><input :value="_getNodeProp('w')" @input="_setNodeProp('w', +$event.target.value)" type="number" class="pi" min="80" max="300" /></div>
            <div class="pg"><label>高</label><input :value="_getNodeProp('h')" @input="_setNodeProp('h', +$event.target.value)" type="number" class="pi" min="30" max="100" /></div>
            <div class="pg"><label>负责人</label><input :value="_getNodeProp('assignee')" @input="_setNodeProp('assignee', $event.target.value)" class="pi" placeholder="负责人标识" /></div>
            <div class="pg"><label>条件表达式</label><input :value="_getNodeProp('condition')" @input="_setNodeProp('condition', $event.target.value)" class="pi" placeholder="如: amount > 1000" /></div>
            <button class="btn-del-sm" @click="deleteNode(selectedNode)">🗑 删除节点</button>
          </div>
        </div>
        <div v-if="selectedEdge!==null" class="props-section">
          <div class="props-title">连线属性</div>
          <div class="props-body">
            <div class="pg"><label>标签</label><input :value="_getEdgeProp('label')" @input="_setEdgeProp('label', $event.target.value)" class="pi" /></div>
            <button class="btn-del-sm" @click="deleteEdge(selectedEdge)">🗑 删除连线</button>
          </div>
        </div>
      </aside>
    </div>

    <!-- New Process Modal -->
    <div v-if="showNewModal" class="modal-overlay" @click.self="showNewModal=false">
      <div class="modal glass-card">
        <h3>新建流程</h3>
        <div class="fg"><label>流程名称</label><input v-model="newForm.name" class="fi" placeholder="如: 请假审批" /></div>
        <div class="fg"><label>唯一标识</label><input v-model="newForm.flag" class="fi" placeholder="如: leave_approval" /></div>
        <div class="fg"><label>描述</label><textarea v-model="newForm.desc" class="fta" rows="2"></textarea></div>
        <div class="ma"><button class="bc" @click="showNewModal=false">取消</button><button class="bs" :disabled="!newForm.name" @click="createProcess">创建</button></div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useQuery, useMutation, useQueryClient } from '@tanstack/vue-query'
import { api } from '@oa4rust/sdk'

interface PDNode { id: string; type: string; label: string; x: number; y: number; w?: number; h?: number; assignee?: string; condition?: string }
interface PDEdge { id: string; from: string; to: string; label?: string }
interface ProcDef { id?: string; name: string; flag: string; desc?: string; status?: string; config?: { nodes: PDNode[]; edges: PDEdge[] } }

const nodeTypes = [
  { type: 'start', label: '开始', icon: '🟢' },
  { type: 'task',  label: '任务', icon: '📋' },
  { type: 'gate',  label: '网关', icon: '🔶' },
  { type: 'end',   label: '结束', icon: '🔴' },
]

const plLoading = ref(false)
const sbFilter = ref('')
const currentProcess = ref<ProcDef|null>(null)
const processDef = ref<{nodes: PDNode[]; edges: PDEdge[]}>({ nodes: [], edges: [] })
const selectedNode = ref<number|null>(null)
const selectedEdge = ref<number|null>(null)
const showNewModal = ref(false)
const newForm = ref({ name: '', flag: '', desc: '' })
const canvasRef = ref<HTMLElement|null>(null)
const panX = ref(0), panY = ref(0), zoom = ref(1)
const canUndo = ref(false), canRedo = ref(false)
const history: {nodes: PDNode[]; edges: PDEdge[]}[] = []
const histIdx = ref(-1)
const isDragging = ref(false)
const dragNode = ref<number|null>(null)
const dragOffset = ref({x:0,y:0})

const filteredProc = computed(() =>
  sbFilter.value ? procList.value.filter(p => (p.name||'').toLowerCase().includes(sbFilter.value.toLowerCase()) || (p.flag||'').toLowerCase().includes(sbFilter.value.toLowerCase()))
  : procList.value
)

const { data: procData } = useQuery({ queryKey: ['pd','list'], queryFn: async () => { plLoading.value = true; try { const r: any = await api.get('/jaxrs/processplatform/assemble/designer/process/list'); return r?.data?.list ?? r?.data ?? [] } finally { plLoading.value = false } } })
const procList = ref<ProcDef[]>(procData.value ?? [])

const svgStyle = computed(() => `transform-origin:0 0;`)

function saveHistory() {
  if (!processDef.value) return
  history.splice(histIdx.value + 1)
  history.push(JSON.parse(JSON.stringify(processDef.value)))
  histIdx.value = history.length - 1
  canUndo.value = histIdx.value > 0
  canRedo.value = false
}

function getNodeProp(prop: string) {
  if (selectedNode.value === null) return ''
  return (processDef.value?.nodes[selectedNode.value] as any)?.[prop] ?? ''
}
function setNodeProp(prop: string, val: any) {
  if (selectedNode.value === null || !processDef.value) return
  ;(processDef.value.nodes[selectedNode.value] as any)[prop] = val
  saveHistory()
}
function getNodeProp2(prop: string) { return getNodeProp(prop) }
function setNodeProp2(prop: string, val: any) { setNodeProp(prop, val) }

// Override computed getters for template use
const _getNodeProp = (p: string) => getNodeProp(p)
const _setNodeProp = (p: string, v: any) => setNodeProp(p, v)
const _getEdgeProp = (p: string) => { if (selectedEdge.value === null || !processDef.value?.edges[selectedEdge.value]) return ''; return (processDef.value.edges[selectedEdge.value] as any)[p] ?? '' }
const _setEdgeProp = (p: string, v: any) => { if (selectedEdge.value === null || !processDef.value?.edges[selectedEdge.value]) return; (processDef.value.edges[selectedEdge.value] as any)[p] = v; saveHistory() }

function getEdgeProp(p: string) { return _getEdgeProp(p) }
function setEdgeProp(p: string, v: any) { _setEdgeProp(p, v) }

function edgePath(edge: PDEdge) {
  const from = processDef.value?.nodes.find(n => n.id === edge.from)
  const to = processDef.value?.nodes.find(n => n.id === edge.to)
  if (!from || !to) return ''
  const fw = from.w || 120, fh = from.h || 40, tw = to.w || 120, th = to.h || 40
  const x1 = from.x + fw, y1 = from.y + fh / 2
  const x2 = to.x, y2 = to.y + th / 2
  const cx = (x1 + x2) / 2
  return `M ${x1} ${y1} C ${cx} ${y1}, ${cx} ${y2}, ${x2} ${y2}`
}

function selectNode(i: number) { selectedNode.value = i; selectedEdge.value = null }
function deleteNode(i: number) {
  if (!processDef.value) return
  processDef.value.nodes.splice(i, 1)
  processDef.value.edges = (processDef.value.edges||[]).filter(e => e.from !== processDef.value!.nodes[i]?.id && e.to !== processDef.value!.nodes[i]?.id)
  if (selectedNode.value === i) selectedNode.value = null
  saveHistory()
}
function deleteEdge(i: number) {
  if (!processDef.value) return
  processDef.value.edges.splice(i, 1)
  selectedEdge.value = null
  saveHistory()
}
function addNode(type: string) {
  if (!processDef.value) return
  const existing = processDef.value.nodes
  processDef.value.nodes.push({
    id: 'n_' + Date.now(), type, label: type === 'start' ? '开始' : type === 'end' ? '结束' : type === 'gate' ? '网关' : '任务节点',
    x: 100 + (existing.length % 3) * 160, y: 80 + Math.floor(existing.length / 3) * 70,
    w: type === 'gate' ? 100 : 120, h: 40
  })
  saveHistory()
}
function clearCanvas() { if (confirm('清空画布？')) { processDef.value = { nodes: [], edges: [] }; selectedNode.value = null; selectedEdge.value = null } }
function autoLayout() {
  if (!processDef.value || processDef.value.nodes.length === 0) return
  const cols = 3
  processDef.value.nodes.forEach((n, i) => { n.x = 80 + (i % cols) * 160; n.y = 80 + Math.floor(i / cols) * 80 })
  saveHistory()
}

function undo() { if (histIdx.value > 0) { histIdx.value--; processDef.value = JSON.parse(JSON.stringify(history[histIdx.value])); selectedNode.value = null } }
function redo() { if (histIdx.value < history.length - 1) { histIdx.value++; processDef.value = JSON.parse(JSON.stringify(history[histIdx.value])); selectedNode.value = null } }
function zoomIn() { zoom.value = Math.min(3, zoom.value + 0.1) }
function zoomOut() { zoom.value = Math.max(0.3, zoom.value - 0.1) }
function fitCanvas() { zoom.value = 1; panX.value = 0; panY.value = 0 }

function onDragNode(e: DragEvent, nt: { type: string }) {
  (e.dataTransfer as any)?.setData('nodeType', nt.type)
}
function onDropNode(e: DragEvent) {
  e.preventDefault()
  const type = (e.dataTransfer as any)?.getData('nodeType')
  if (!type || !processDef.value) return
  const rect = (e.currentTarget as HTMLElement).getBoundingClientRect()
  const cx = (e.clientX - rect.left - panX.value) / zoom.value
  const cy = (e.clientY - rect.top - panY.value) / zoom.value
  processDef.value.nodes.push({ id: 'n_' + Date.now(), type, label: type === 'start' ? '开始' : type === 'end' ? '结束' : type === 'gate' ? '网关' : '任务节点', x: cx - 60, y: cy - 20, w: 120, h: 40 })
  saveHistory()
}
function onNodeDragStart(e: MouseEvent, i: number) {
  e.stopPropagation()
  isDragging.value = true; dragNode.value = i
  if (!processDef.value) return
  const n = processDef.value.nodes[i]
  dragOffset.value = { x: e.clientX / zoom.value - n.x, y: e.clientY / zoom.value - n.y }
  const onMove = (ev: MouseEvent) => {
    if (!isDragging.value || !processDef.value || dragNode.value === null) return
    processDef.value.nodes[dragNode.value].x = ev.clientX / zoom.value - dragOffset.value.x
    processDef.value.nodes[dragNode.value].y = ev.clientY / zoom.value - dragOffset.value.y
  }
  const onUp = () => { isDragging.value = false; dragNode.value = null; document.removeEventListener('mousemove', onMove); document.removeEventListener('mouseup', onUp); saveHistory() }
  document.addEventListener('mousemove', onMove)
  document.addEventListener('mouseup', onUp)
}

async function loadProcess(p: ProcDef) {
  try {
    const r: any = await api.get(`/jaxrs/processplatform/assemble/designer/process/${p.id}`)
    const data = r?.data ?? p
    currentProcess.value = data
    processDef.value = data.config ?? { nodes: [], edges: [] }
    if (!processDef.value.nodes.length) {
      processDef.value = {
        nodes: [
          { id: 'n_start', type: 'start', label: '开始', x: 80, y: 100, w: 100, h: 40 },
          { id: 'n_task1', type: 'task', label: '审批任务', x: 300, y: 80, w: 120, h: 40 },
          { id: 'n_end', type: 'end', label: '结束', x: 520, y: 100, w: 100, h: 40 }
        ],
        edges: [{ id: 'e1', from: 'n_start', to: 'n_task1' }, { id: 'e2', from: 'n_task1', to: 'n_end' }]
      }
    }
    selectedNode.value = null; selectedEdge.value = null; history.length = 0; histIdx.value = -1; saveHistory()
  } catch { currentProcess.value = { ...p, name: p.name, flag: p.flag, config: { nodes: [], edges: [] } }; processDef.value = { nodes: [], edges: [] } }
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
  const data = { name: newForm.value.name, flag: newForm.value.flag, description: newForm.value.desc, config: processDef.value }
  savePM.mutate(data)
}
async function saveProcess() {
  if (!currentProcess.value) return
  try {
    await api.put(`/jaxrs/processplatform/assemble/designer/process/${currentProcess.value.id}`, {
      name: currentProcess.value.name, flag: currentProcess.value.flag, config: processDef.value
    })
    alert('保存成功')
  } catch (e: any) { alert('保存失败: ' + (e?.message ?? '')) }
}
function loadProcesses() { loadProc() }
const loadProc = async () => { try { const r: any = await api.get('/jaxrs/processplatform/assemble/designer/process/list'); procList.value = r?.data?.list ?? r?.data ?? [] } catch { procList.value = [] } }
onMounted(loadProcesses)
</script>

<style scoped>
.pd{display:flex;flex-direction:column;gap:0;height:100%}
.pd-header{display:flex;align-items:center;justify-content:space-between;padding:10px 16px;flex-shrink:0}
.pd-title h1{font-family:'Orbitron',sans-serif;font-size:18px;color:var(--color-primary);margin:0 0 2px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:11px;color:var(--text-muted);margin:0;font-family:'JetBrains Mono',monospace}
.pd-actions{display:flex;gap:6px;flex-wrap:wrap}
.btn{padding:5px 12px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);cursor:pointer;font-size:12px}
.btn:hover{border-color:var(--color-primary);color:var(--color-primary)}
.btn:disabled{opacity:0.3;cursor:not-allowed}
.btn-primary{background:var(--color-primary);color:#000;border-color:var(--color-primary);font-weight:600}
.btn-outline{background:transparent}
.pd-body{display:flex;flex:1;gap:0;min-height:0;overflow:hidden}
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
.nl{font-size:10px;color:var(--text-muted);margin-top:4px}
/* Canvas */
.pd-canvas{flex:1;position:relative;overflow:hidden;min-width:0}
.canvas-empty{display:flex;flex-direction:column;align-items:center;justify-content:center;height:100%;gap:16px;color:var(--text-muted)}
.ce-icon{font-size:64px;opacity:0.3}
.canvas-wrap{width:100%;height:100%;position:relative;overflow:hidden}
.canvas-svg{width:100%;height:100%;cursor:grab}
.canvas-svg:active{cursor:grabbing}
.canvas-hint{position:absolute;bottom:8px;left:50%;transform:translateX(-50%);font-size:11px;color:var(--text-muted);pointer-events:none}
.edge{fill:none;stroke:var(--color-primary);stroke-width:2;cursor:pointer;opacity:0.6}
.edge:hover,.edge.selected{stroke:var(--color-warning);stroke-width:3;opacity:1}
.node{cursor:pointer}
.node-bg{stroke:var(--border-color);stroke-width:1.5;transition:all 0.15s}
.node.start .node-bg{fill:rgba(16,185,129,.2);stroke:var(--color-success)}
.node.task .node-bg{fill:rgba(0,212,255,.15);stroke:var(--color-primary)}
.node.gate .node-bg{fill:rgba(245,158,11,.15);stroke:var(--color-warning)}
.node.end .node-bg{fill:rgba(239,68,68,.2);stroke:var(--color-danger)}
.node.selected .node-bg{stroke-width:3;filter:drop-shadow(0 0 8px var(--color-primary))}
.node-label{fill:var(--text-primary);font-size:12px;font-weight:500;pointer-events:none}
.port{fill:var(--color-primary);opacity:0.4;cursor:crosshair}
.port:hover{opacity:1}
/* Props */
.pd-props{width:240px;flex-shrink:0;padding:12px;border-left:1px solid var(--border-color);overflow-y:auto}
.props-section{margin-bottom:16px}
.props-title{font-size:11px;color:var(--text-muted);text-transform:uppercase;letter-spacing:1px;margin-bottom:8px;font-weight:600}
.props-body{display:flex;flex-direction:column;gap:8px}
.pg{display:flex;flex-direction:column;gap:3px}
.pg label{font-size:11px;color:var(--text-muted)}
.pi{padding:5px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:12px;outline:none}
.pi:focus{border-color:var(--color-primary)}
.pv{font-size:12px;color:var(--color-primary);font-family:'JetBrains Mono',monospace}
.btn-del-sm{padding:5px 10px;border-radius:var(--radius-sm);border:1px solid var(--color-danger);background:transparent;color:var(--color-danger);cursor:pointer;font-size:12px}
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
