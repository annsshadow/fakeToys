const fs = require('fs');
let content = fs.readFileSync('D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/ProcessDesigner.vue', 'utf8');

// === 1. Enhance GroupInfo interface and add group drag/resize ===
const groupInterfaceMarker = 'interface GroupInfo { node: PDNode; members: PDNode[]; bounds: {x:number;y:number;width:number;height:number} }';
const groupInterfaceEnhanced = `interface GroupInfo { node: PDNode; members: PDNode[]; bounds: {x:number;y:number;width:number;height:number} }
interface GroupDragState { idx: number; startX: number; startY: number; origX: number; origY: number }
interface GroupResizeState { idx: number; dir: string; startX: number; startY: number; origW: number; origH: number; origX: number; origY: number }`;
content = content.replace(groupInterfaceMarker, groupInterfaceEnhanced);

// === 2. Add group drag/resize state ===
const groupStateMarker = 'const groupNodes = computed(() => getGroupNodes())';
const groupStateEnhanced = `const groupNodes = computed(() => getGroupNodes())
const groupDragState = ref<GroupDragState|null>(null)
const groupResizeState = ref<GroupResizeState|null>(null)
const showRoutingPanel = ref(false)
const selectedRoutingEdge = ref<number|null>(null)
const showScriptEditor = ref(false)
const scriptEditorNodeIdx = ref<number|null>(null)
const showBranchAnnot = ref(false)
const branchAnnotation = ref<{type:'fork'|'join'; idx: number; label: string; color: string}>({ type: 'fork', idx: 0, label: '', color: '#f59e0b' })`;
content = content.replace(groupStateMarker, groupStateEnhanced);

// === 3. Enhance createGroup with drag support ===
const createGroupOld = `function createGroup() {
  if (groupedNodes.value.size < 2 || !processDef.value) return
  const members: string[] = Array.from(groupedNodes.value)
  // Find bounding box of all member nodes
  let minX=Infinity, minY=Infinity, maxX=-Infinity, maxY=-Infinity
  for (const id of members) {
    const n = processDef.value.nodes.find(nd => nd.id === id)
    if (!n) continue
    minX = Math.min(minX, n.x); minY = Math.min(minY, n.y)
    maxX = Math.max(maxX, n.x + (n.w||120)); maxY = Math.max(maxY, n.y + (n.h||50))
  }
  // Create group node
  const groupId = genId()
  const groupNode: PDNode = {
    id: groupId, type: 'subprocess', label: '分组',
    x: minX - 15, y: minY - 15,
    w: maxX - minX + 30, h: maxY - minY + 30,
    groupMembers: members, collapsed: false
  }
  // Store original positions before moving members inside
  for (const id of members) {
    const n = processDef.value!.nodes.find(nd => nd.id === id)
    if (n) {
      ;(n as any).__origGroupId = groupId
      ;(n as any).__origX = n.x
      ;(n as any).__origY = n.y
    }
  }
  processDef.value.nodes.push(groupNode)
  groupedNodes.value.clear()
  selectedNode.value = processDef.value.nodes.length - 1
  selectedEdge.value = null
  pushHistory()
}`;

const createGroupNew = `function createGroup() {
  if (groupedNodes.value.size < 2 || !processDef.value) return
  const members: string[] = Array.from(groupedNodes.value)
  let minX=Infinity, minY=Infinity, maxX=-Infinity, maxY=-Infinity
  for (const id of members) {
    const n = processDef.value.nodes.find(nd => nd.id === id)
    if (!n) continue
    minX = Math.min(minX, n.x); minY = Math.min(minY, n.y)
    maxX = Math.max(maxX, n.x + (n.w||120)); maxY = Math.max(maxY, n.y + (n.h||50))
  }
  const groupId = genId()
  const groupNode: PDNode = {
    id: groupId, type: 'subprocess', label: '分组',
    x: minX - 15, y: minY - 15,
    w: maxX - minX + 30, h: maxY - minY + 30,
    groupMembers: members, collapsed: false
  }
  for (const id of members) {
    const n = processDef.value!.nodes.find(nd => nd.id === id)
    if (n) {
      ;(n as any).__origGroupId = groupId
      ;(n as any).__origX = n.x
      ;(n as any).__origY = n.y
    }
  }
  processDef.value.nodes.push(groupNode)
  groupedNodes.value.clear()
  selectedNode.value = processDef.value.nodes.length - 1
  selectedEdge.value = null
  pushHistory()
}

// ── Group Drag & Resize ──────────────────────────────────────────────
function onGroupMouseDown(e: MouseEvent, idx: number) {
  e.stopPropagation()
  if (!processDef.value) return
  const g = groupNodes.value[idx]
  if (!g) return
  groupDragState.value = {
    idx, startX: e.clientX, startY: e.clientY,
    origX: g.node.x, origY: g.node.y
  }
  const onMove = (ev: MouseEvent) => {
    if (!groupDragState.value) return
    const dx = (ev.clientX - groupDragState.value.startX) / zoom.value
    const dy = (ev.clientY - groupDragState.value.startY) / zoom.value
    const g = groupNodes.value[groupDragState.value.idx]
    if (!g) return
    g.node.x = groupDragState.value.origX + dx
    g.node.y = groupDragState.value.origY + dy
    // Snap to grid
    if (snapToGrid.value) {
      g.node.x = Math.round(g.node.x / customGridSize.value) * customGridSize.value
      g.node.y = Math.round(g.node.y / customGridSize.value) * customGridSize.value
    }
    // Move members with group
    for (const id of g.node.groupMembers!) {
      const n = processDef.value!.nodes.find(nd => nd.id === id)
      if (n && (n as any).__origX !== undefined) {
        const ox = (n as any).__origX
        const oy = (n as any).__origY
        const gx = groupDragState.value.origX
        const gy = groupDragState.value.origY
        const ndx = g.node.x - gx
        const ndy = g.node.y - gy
        n.x = ox + ndx
        n.y = oy + ndy
        if (snapToGrid.value) {
          n.x = Math.round(n.x / customGridSize.value) * customGridSize.value
          n.y = Math.round(n.y / customGridSize.value) * customGridSize.value
        }
      }
    }
  }
  const onUp = () => {
    document.removeEventListener('mousemove', onMove)
    document.removeEventListener('mouseup', onUp)
    groupDragState.value = null
    pushHistory()
  }
  document.addEventListener('mousemove', onMove)
  document.addEventListener('mouseup', onUp)
}
function onGroupResizeMouseDown(e: MouseEvent, idx: number, dir: string) {
  e.stopPropagation()
  if (!processDef.value) return
  const g = groupNodes.value[idx]
  if (!g) return
  groupResizeState.value = {
    idx, dir,
    startX: e.clientX, startY: e.clientY,
    origW: g.node.w || 200, origH: g.node.h || 100,
    origX: g.node.x, origY: g.node.y
  }
  const onMove = (ev: MouseEvent) => {
    if (!groupResizeState.value) return
    const dx = (ev.clientX - groupResizeState.value.startX) / zoom.value
    const dy = (ev.clientY - groupResizeState.value.startY) / zoom.value
    const gs = groupResizeState.value
    const gn = processDef.value.nodes[gs.idx]
    if (!gn) return
    switch(gs.dir) {
      case 'se': gn.w = Math.max(100, gs.origW + dx); gn.h = Math.max(60, gs.origH + dy); break
      case 'e': gn.w = Math.max(100, gs.origW + dx); break
      case 's': gn.h = Math.max(60, gs.origH + dy); break
      case 'nw': gn.x = gs.origX + dx; gn.y = gs.origY + dy; gn.w = Math.max(100, gs.origW - dx); gn.h = Math.max(60, gs.origH - dy); break
      case 'sw': gn.y = gs.origY + dy; gn.w = Math.max(100, gs.origW); gn.h = Math.max(60, gs.origH + dy); break
      case 'ne': gn.x = gs.origX + dx; gn.h = Math.max(60, gs.origH + dy); break
      case 'n': gn.y = gs.origY + dy; gn.h = Math.max(60, gs.origH - dy); break
      case 'w': gn.x = gs.origX + dx; gn.w = Math.max(100, gs.origW - dx); break
    }
    gn.w = Math.max(100, Math.round(gn.w / GRID_SIZE) * GRID_SIZE)
    gn.h = Math.max(60, Math.round(gn.h / GRID_SIZE) * GRID_SIZE)
  }
  const onUp = () => {
    document.removeEventListener('mousemove', onMove)
    document.removeEventListener('mouseup', onUp)
    groupResizeState.value = null
    pushHistory()
  }
  document.addEventListener('mousemove', onMove)
  document.addEventListener('mouseup', onUp)
}
function getGroupResizeX(node: PDNode, dir: string): number {
  if (dir.includes('w')) return node.x
  return node.x + (node.w||200)
}
function getGroupResizeY(node: PDNode, dir: string): number {
  if (dir.includes('n')) return node.y
  return node.y + (node.h||100)
}
const groupResizeDirs = ['nw','n','ne','e','se','s','sw','w'] as const`;
content = content.replace(createGroupOld, createGroupNew);

// === 4. Add routing panel state and functions ===
const routingMarker = 'const showHelpModal = ref(false)';
const routingEnhanced = `const showHelpModal = ref(false)

// ── Edge Routing Panel ───────────────────────────────────────────────
interface RoutingPoint { x: number; y: number; type: 'anchor'|'control' }
interface EdgeRouteConfig {
  edgeId: string
  fromNodeIdx: number
  toNodeIdx: number
  routing: 'auto'|'straight'|'horizontal'|'vertical'
  controlPoints: RoutingPoint[]
  offset: number
  labelPos: 'auto'|'start'|'mid'|'end'
  arrowStyle: 'default'|'none'|'both'
  dashArray: string
  strokeWidth: number
  color: string
}
const routingConfigs = ref<Map<string, EdgeRouteConfig>>(new Map())
const showRoutingPanel = ref(false)
const editingRouteId = ref<string|null>(null)
const newControlPoint = ref<{x:number;y:number}>({x:0,y:0})

function openRoutingPanel(edgeIdx: number) {
  if (!processDef.value || !processDef.value.edges[edgeIdx]) return
  selectedRoutingEdge.value = edgeIdx
  showRoutingPanel.value = true
  const edge = processDef.value.edges[edgeIdx]
  if (!routingConfigs.value.has(edge.id)) {
    routingConfigs.value.set(edge.id, {
      edgeId: edge.id, fromNodeIdx: 0, toNodeIdx: 1,
      routing: edge.routing || 'auto', controlPoints: [],
      offset: 0, labelPos: 'auto', arrowStyle: 'default',
      dashArray: 'none', strokeWidth: edge.strokeWidth || 2, color: ''
    })
  }
}
function getRoutingConfig(edgeId: string): EdgeRouteConfig|null {
  return routingConfigs.value.get(edgeId) || null
}
function updateRoutingConfig(edgeId: string, updates: Partial<EdgeRouteConfig>) {
  const cfg = routingConfigs.value.get(edgeId)
  if (cfg) { Object.assign(cfg, updates); routingConfigs.value.set(edgeId, cfg) }
}
function addControlPoint() {
  if (!selectedRoutingEdge.value || !processDef.value) return
  const edge = processDef.value.edges[selectedRoutingEdge.value]
  if (!edge) return
  const cfg = routingConfigs.value.get(edge.id)
  if (!cfg) return
  cfg.controlPoints.push({ x: newControlPoint.value.x, y: newControlPoint.value.y, type: 'control' })
  routingConfigs.value.set(edge.id, cfg)
}
function removeControlPoint(idx: number) {
  if (!selectedRoutingEdge.value || !processDef.value) return
  const edge = processDef.value.edges[selectedRoutingEdge.value]
  if (!edge) return
  const cfg = routingConfigs.value.get(edge.id)
  if (!cfg) return
  cfg.controlPoints.splice(idx, 1)
  routingConfigs.value.set(edge.id, cfg)
}
function computeCustomEdgePath(edge: PDEdge): string {
  const cfg = routingConfigs.value.get(edge.id)
  if (!cfg || cfg.controlPoints.length === 0) return computeEdgePath(edge)
  if (!processDef.value) return ''
  const from = processDef.value.nodes.find(n => n.id === edge.from)
  const to = processDef.value.nodes.find(n => n.id === edge.to)
  if (!from || !to) return ''
  const fp = getNodePort(from, 'out'), tp = getNodePort(to, 'in')
  let d = \`M \${fp.x} \${fp.y}\`
  for (const cp of cfg.controlPoints) {
    d += \` L \${cp.x} \${cp.y}\`
  }
  d += \` L \${tp.x} \${tp.y}\`
  return d
}
function applyRoutingPreset(preset: 'smooth'|'orthogonal'|'manhattan'|'zigzag') {
  if (selectedRoutingEdge.value === null || !processDef.value) return
  const edge = processDef.value.edges[selectedRoutingEdge.value]
  if (!edge) return
  const from = processDef.value.nodes.find(n => n.id === edge.from)
  const to = processDef.value.nodes.find(n => n.id === edge.to)
  if (!from || !to) return
  const fp = getNodePort(from, 'out'), tp = getNodePort(to, 'in')
  const midX = (fp.x + tp.x) / 2, midY = (fp.y + tp.y) / 2
  let points: RoutingPoint[] = []
  switch(preset) {
    case 'smooth':
      points = [{ x: (fp.x+midX)/2, y: fp.y, type:'control' }, { x: (midX+tp.x)/2, y: tp.y, type:'control' }]
      break
    case 'orthogonal':
      points = [{ x: midX, y: fp.y, type:'anchor' }, { x: midX, y: tp.y, type:'anchor' }]
      break
    case 'manhattan':
      points = [{ x: midX, y: fp.y, type:'anchor' }, { x: midX, y: (fp.y+tp.y)/2, type:'anchor' }, { x: tp.x, y: (fp.y+tp.y)/2, type:'anchor' }]
      break
    case 'zigzag':
      const segs = 3
      for (let i = 1; i < segs; i++) {
        const t = i / segs
        points.push({ x: fp.x + (tp.x - fp.x) * t, y: fp.y + (tp.y - fp.y) * t + (i%2===0?-20:20), type:'control' })
      }
      break
  }
  updateRoutingConfig(edge.id, { controlPoints: points, routing: 'custom' })
}

// ── Script Action Editor ─────────────────────────────────────────────
interface ScriptVar { name: string; type: string; defaultValue: string; description: string }
interface ScriptErrorHandling { onFail: 'abort'|'skip'|'retry'; retryCount?: number; retryDelay?: number; onErrorAction?: string }
interface ScriptOutputMapping { from: string; to: string; transform?: string }
interface ScriptActionConfig {
  language: 'javascript'|'python'|'typescript'
  code: string
  imports: string[]
  variables: ScriptVar[]
  errorHandling: ScriptErrorHandling
  outputMapping: ScriptOutputMapping[]
  timeout: number
  description: string
}
const scriptEditors = ref<Map<string, ScriptActionConfig>>(new Map())
const showScriptEditor = ref(false)
const scriptEditorNodeIdx = ref<number|null>(null)

function openScriptEditor(nodeIdx: number) {
  scriptEditorNodeIdx.value = nodeIdx
  showScriptEditor.value = true
  const node = processDef.value?.nodes[nodeIdx]
  if (!node) return
  const key = node.id
  if (!scriptEditors.value.has(key)) {
    scriptEditors.value.set(key, {
      language: 'javascript',
      code: node.script || '// 脚本代码\\n// 可用变量: inputData, context, output\\noutput.result = inputData.value;',
      imports: [],
      variables: [
        { name: 'inputData', type: 'object', defaultValue: '{}', description: '输入数据' },
        { name: 'context', type: 'object', defaultValue: '{}', description: '流程上下文' },
        { name: 'output', type: 'any', defaultValue: 'null', description: '输出结果' },
      ],
      errorHandling: { onFail: 'skip', retryCount: 3, retryDelay: 1000 },
      outputMapping: [],
      timeout: 30000,
      description: ''
    })
  }
}
function closeScriptEditor() { showScriptEditor.value = false; scriptEditorNodeIdx.value = null }
function saveScriptEditor() {
  if (scriptEditorNodeIdx.value === null || !processDef.value) return
  const node = processDef.value.nodes[scriptEditorNodeIdx.value]
  const key = node.id
  const cfg = scriptEditors.value.get(key)
  if (cfg) {
    node.script = cfg.code
    ;(node as any).scriptConfig = cfg
    pushHistory()
  }
  closeScriptEditor()
}
function getScriptConfig(nodeId: string): ScriptActionConfig|null {
  return scriptEditors.value.get(nodeId) || null
}
function addScriptVar() {
  const key = scriptEditorNodeIdx.value ? processDef.value!.nodes[scriptEditorNodeIdx.value].id : null
  if (!key) return
  const cfg = scriptEditors.value.get(key)
  if (!cfg) return
  cfg.variables.push({ name: 'newVar', type: 'string', defaultValue: '', description: '' })
  scriptEditors.value.set(key, cfg)
}
function removeScriptVar(idx: number) {
  const key = scriptEditorNodeIdx.value ? processDef.value!.nodes[scriptEditorNodeIdx.value].id : null
  if (!key) return
  const cfg = scriptEditors.value.get(key)
  if (!cfg) return
  cfg.variables.splice(idx, 1)
  scriptEditors.value.set(key, cfg)
}
function addOutputMapping() {
  const key = scriptEditorNodeIdx.value ? processDef.value!.nodes[scriptEditorNodeIdx.value].id : null
  if (!key) return
  const cfg = scriptEditors.value.get(key)
  if (!cfg) return
  cfg.outputMapping.push({ from: '', to: '', transform: '' })
  scriptEditors.value.set(key, cfg)
}
function removeOutputMapping(idx: number) {
  const key = scriptEditorNodeIdx.value ? processDef.value!.nodes[scriptEditorNodeIdx.value].id : null
  if (!key) return
  const cfg = scriptEditors.value.get(key)
  if (!cfg) return
  cfg.outputMapping.splice(idx, 1)
  scriptEditors.value.set(key, cfg)
}
const scriptPresets = [
  { name: '数据转换', icon: '🔄', code: '// 数据转换脚本\\nconst input = inputData;\\nconst result = {\\n  processed: true,\\n  timestamp: Date.now(),\\n  data: input\\n};\\noutput.result = result;' },
  { name: '条件判断', icon: '🔀', code: '// 条件判断脚本\\nconst value = inputData.value;\\nif (value > 100) {\\n  output.result = "high";\\n  output.level = "A";\\n} else if (value > 50) {\\n  output.result = "medium";\\n  output.level = "B";\\n} else {\\n  output.result = "low";\\n  output.level = "C";\\n}' },
  { name: '数据聚合', icon: '📊', code: '// 数据聚合脚本\\nconst items = inputData.items || [];\\noutput.total = items.length;\\noutput.sum = items.reduce((s,i) => s + (i.value||0), 0);\\noutput.avg = items.length > 0 ? output.sum / items.length : 0;\\noutput.max = Math.max(...items.map(i => i.value||0));\\noutput.min = Math.min(...items.map(i => i.value||0));' },
  { name: '通知发送', icon: '📧', code: '// 通知发送脚本\\nconst recipient = inputData.recipient;\\nconst message = inputData.message;\\noutput.sent = true;\\noutput.timestamp = new Date().toISOString();\\noutput.recipient = recipient;\\n// 调用通知API\\n// await api.post("/jaxrs/notify/send", { to: recipient, msg: message });' },
  { name: '数据验证', icon: '✅', code: '// 数据验证脚本\\nconst data = inputData;\\nconst errors = [];\\nif (!data.name) errors.push("名称不能为空");\\nif (!data.email || !data.email.includes("@")) errors.push("邮箱格式错误");\\nif (data.age !== undefined && (data.age < 0 || data.age > 150)) errors.push("年龄范围错误");\\noutput.valid = errors.length === 0;\\noutput.errors = errors;\\noutput.cleanData = data;' },
  { name: '日期处理', icon: '📅', code: '// 日期处理脚本\\nconst dateStr = inputData.date;\\nconst date = new Date(dateStr);\\noutput.formatted = date.toLocaleDateString("zh-CN");\\noutput.weekday = date.toLocaleDateString("zh-CN", { weekday: "long" });\\noutput.month = date.getMonth() + 1;\\noutput.year = date.getFullYear();\\noutput.day = date.getDate();\\noutput.isWeekend = date.getDay() === 0 || date.getDay() === 6;' },
  { name: '字符串处理', icon: '📝', code: '// 字符串处理脚本\\nconst text = inputData.text || "";\\noutput.upper = text.toUpperCase();\\noutput.lower = text.toLowerCase();\\noutput.title = text.replace(/\\\\b\\\\w/g, c => c.toUpperCase());\\noutput.words = text.split(/\\\\s+/).filter(Boolean);\\noutput.chars = text.length;\\noutput.reverse = text.split("").reverse().join("");' },
  { name: '数学计算', icon: '🔢', code: '// 数学计算脚本\\nconst a = parseFloat(inputData.a) || 0;\\nconst b = parseFloat(inputData.b) || 0;\\noutput.sum = a + b;\\noutput.diff = a - b;\\noutput.prod = a * b;\\noutput.div = b !== 0 ? a / b : null;\\noutput.pow = Math.pow(a, b);\\noutput.sqrtA = Math.sqrt(a);\\noutput.sqrtB = Math.sqrt(b);\\noutput.absA = Math.abs(a);\\noutput.ceil = Math.ceil(a);\\noutput.floor = Math.floor(a);\\noutput.round = Math.round(a);' },
]

// ── Fork/Join Enhanced Annotations ───────────────────────────────────
interface ForkJoinAnnotation {
  id: string; type: 'fork'|'join'
  branchIndices: number[]
  forkNodeIdx: number; joinNodeIdx?: number
  label: string; color: string
  annotations: Array<{type: 'label'|'flow'|'count'; text: string}>
}
const forkJoinAnnotations = ref<ForkJoinAnnotation[]>([])

function detectParallelBranchesEnhanced(): ForkJoinAnnotation[] {
  if (!processDef.value) return []
  const nodes = processDef.value.nodes
  const edges = processDef.value.edges || []
  const annotations: ForkJoinAnnotation[] = []
  // Find fork nodes (nodes with 2+ outgoing edges to different paths)
  for (let i = 0; i < nodes.length; i++) {
    const outgoing = edges.filter(e => e.from === nodes[i].id)
    if (outgoing.length >= 2) {
      const branchMembers = new Set<string>()
      for (const e of outgoing) branchMembers.add(e.to)
      if (branchMembers.size >= 2) {
        const branchIndices: number[] = []
        for (const toId of branchMembers) {
          const idx = nodes.findIndex(n => n.id === toId)
          if (idx !== -1) branchIndices.push(idx)
        }
        // Find join node (node with incoming edges from all branch members)
        let joinNodeIdx: number|undefined
        const potentialJoins = nodes.filter((n, j) => {
          if (j === i) return false
          return branchIndices.every(bi => edges.some(e => e.from === nodes[bi].id && e.to === n.id))
        })
        if (potentialJoins.length > 0) joinNodeIdx = nodes.findIndex(n => n.id === potentialJoins[0].id)
        annotations.push({
          id: genId(), type: 'fork',
          branchIndices, forkNodeIdx: i, joinNodeIdx,
          label: '分支' + (annotations.length + 1),
          color: '#f59e0b',
          annotations: [
            { type: 'label', text: 'FORK #' + (annotations.length + 1) },
            { type: 'flow', text: outgoing.length + ' 路并行' },
            { type: 'count', text: branchMembers.size + ' 分支' }
          ]
        })
      }
    }
  }
  return annotations
}
const forkJoinAnnotationsEnhanced = computed(() => detectParallelBranchesEnhanced())`
content = content.replace(routingMarker, routingEnhanced);

// Write back
fs.writeFileSync('D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/ProcessDesigner.vue', content);
console.log('Lines:', content.split('\n').length);
