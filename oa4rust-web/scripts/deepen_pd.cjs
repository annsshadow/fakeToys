const fs = require('fs');
let content = fs.readFileSync('D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/ProcessDesigner.vue', 'utf8');

// === 1. Add path prediction state after tempEdge ===
const tempEdgeMarker = 'const tempEdge = ref<{ from: number; fromPort: \'out\'|\'in\'; startX: number; startY: number; endX: number; endY: number }|null>(null)';
const tempEdgeExtra = `const tempEdge = ref<{ from: number; fromPort: 'out'|'in'; startX: number; startY: number; endX: number; endY: number }|null>(null)
const predictedTarget = ref<number|null>(null)
const predictedPath = ref<string>('')
const showPrediction = ref(false)`;
content = content.replace(tempEdgeMarker, tempEdgeExtra);

// === 2. Add subprocess stack state ===
const subEditMarker = 'const subprocessEditing = ref(false)';
const subEditExtra = `const subprocessEditing = ref(false)
const subprocessStack = ref<Array<{nodes: PDNode[]; edges: PDEdge[]; title: string; parentIdx?: number}>>([])
const subprocessDepth = ref(0)
const activeSubprocessIdx = ref<number|null>(null)`;
content = content.replace(subEditMarker, subEditExtra);

// === 3. Replace onPortMouseDown with enhanced version including prediction ===
const oldPortMouseDown = `function onPortMouseDown(e: MouseEvent, nodeIdx: number, port: 'in'|'out') {
  e.stopPropagation()
  if (!processDef.value) return
  const node = processDef.value.nodes[nodeIdx]
  const pp = getNodePort(node, port)
  tempEdge.value = { from: nodeIdx, fromPort: port, startX: pp.x, startY: pp.y, endX: pp.x, endY: pp.y }
  const onMove = (ev: MouseEvent) => {
    if (!tempEdge.value) return
    tempEdge.value.endX = (ev.clientX - panX.value) / zoom.value
    tempEdge.value.endY = (ev.clientY - panY.value) / zoom.value
  }
  const onUp = (ev: MouseEvent) => {
    document.removeEventListener('mousemove', onMove); document.removeEventListener('mouseup', onUp)
    if (!tempEdge.value || !processDef.value) { tempEdge.value = null; return }
    const mx = (ev.clientX - panX.value) / zoom.value, my = (ev.clientY - panX.value) / zoom.value
    let targetIdx: number|null = null
    for (let i = 0; i < processDef.value.nodes.length; i++) {
      if (i === tempEdge.value!.from) continue
      const n = processDef.value.nodes[i]
      if (mx >= n.x-10 && mx <= n.x+(n.w||120)+10 && my >= n.y-10 && my <= n.y+(n.h||50)+10) { targetIdx = i; break }
    }
    if (targetIdx !== null) {
      const fn = processDef.value.nodes[tempEdge.value!.from]
      const tn = processDef.value.nodes[targetIdx]
      const fp = tempEdge.value!.fromPort
      if (fp === 'out' && tn.type !== 'start') createEdge(fn.id, tn.id)
      else if (fp === 'in' && fn.type !== 'end') createEdge(tn.id, fn.id)
    }
    tempEdge.value = null
  }
  document.addEventListener('mousemove', onMove); document.addEventListener('mouseup', onUp)
}`;

const newPortMouseDown = `function onPortMouseDown(e: MouseEvent, nodeIdx: number, port: 'in'|'out') {
  e.stopPropagation()
  if (!processDef.value) return
  const node = processDef.value.nodes[nodeIdx]
  const pp = getNodePort(node, port)
  tempEdge.value = { from: nodeIdx, fromPort: port, startX: pp.x, startY: pp.y, endX: pp.x, endY: pp.y }
  predictedTarget.value = null
  predictedPath.value = ''
  showPrediction.value = true
  const onMove = (ev: MouseEvent) => {
    if (!tempEdge.value) return
    tempEdge.value.endX = (ev.clientX - panX.value) / zoom.value
    tempEdge.value.endY = (ev.clientY - panY.value) / zoom.value
    // Path prediction: find nearest valid target
    const mx = (ev.clientX - panX.value) / zoom.value
    const my = (ev.clientY - panY.value) / zoom.value
    let bestIdx: number|null = null, bestDist = Infinity
    for (let i = 0; i < processDef.value!.nodes.length; i++) {
      if (i === tempEdge.value!.from) continue
      const n = processDef.value!.nodes[i]
      if (mx >= n.x-20 && mx <= n.x+(n.w||120)+20 && my >= n.y-20 && my <= n.y+(n.h||50)+20) {
        const cx = n.x + (n.w||120)/2, cy = n.y + (n.h||50)/2
        const d = Math.hypot(mx - cx, my - cy)
        if (d < bestDist) { bestDist = d; bestIdx = i }
      }
    }
    predictedTarget.value = bestIdx
    if (bestIdx !== null) {
      const from = processDef.value!.nodes[tempEdge.value!.from]
      const to = processDef.value!.nodes[bestIdx]
      const fp = getNodePort(from, tempEdge.value!.fromPort)
      const tp = getNodePort(to, port === 'out' ? 'in' : 'out')
      const dx = Math.abs(tp.x - fp.x), cx = Math.max(dx * 0.5, 60)
      predictedPath.value = \`M \${fp.x} \${fp.y} C \${fp.x+cx} \${fp.y}, \${tp.x-cx} \${tp.y}, \${tp.x} \${tp.y}\`
    } else {
      predictedPath.value = ''
    }
  }
  const onUp = (ev: MouseEvent) => {
    document.removeEventListener('mousemove', onMove); document.removeEventListener('mouseup', onUp)
    showPrediction.value = false
    predictedTarget.value = null
    predictedPath.value = ''
    if (!tempEdge.value || !processDef.value) { tempEdge.value = null; return }
    const mx = (ev.clientX - panX.value) / zoom.value, my = (ev.clientY - panY.value) / zoom.value
    let targetIdx: number|null = null
    for (let i = 0; i < processDef.value.nodes.length; i++) {
      if (i === tempEdge.value!.from) continue
      const n = processDef.value.nodes[i]
      if (mx >= n.x-10 && mx <= n.x+(n.w||120)+10 && my >= n.y-10 && my <= n.y+(n.h||50)+10) { targetIdx = i; break }
    }
    if (targetIdx !== null) {
      const fn = processDef.value.nodes[tempEdge.value!.from]
      const tn = processDef.value.nodes[targetIdx]
      const fp = tempEdge.value!.fromPort
      if (fp === 'out' && tn.type !== 'start') createEdge(fn.id, tn.id)
      else if (fp === 'in' && fn.type !== 'end') createEdge(tn.id, fn.id)
    }
    tempEdge.value = null
  }
  document.addEventListener('mousemove', onMove); document.addEventListener('mouseup', onUp)
}

// ── Predicted edge rendering ─────────────────────────────────────────
function predictedEdgePath(): string { return predictedPath.value }
function getPredictedTargetNode(): PDNode|undefined {
  if (predictedTarget.value === null || !processDef.value) return undefined
  return processDef.value.nodes[predictedTarget.value]
}`;

content = content.replace(oldPortMouseDown, newPortMouseDown);

// === 4. Add predicted edge to SVG template ===
const tempEdgeSvgMarker = '<path v-if="tempEdge" :d="tempEdgePath()" class="edge-temp" marker-end="url(#arrowhead-temp)" />';
const tempEdgeSvgExtra = `<path v-if="tempEdge" :d="tempEdgePath()" class="edge-temp" marker-end="url(#arrowhead-temp)" />
          <!-- Predicted connection path -->
          <path v-if="showPrediction && predictedPath" :d="predictedPath" class="edge-predicted" stroke-dasharray="6,3" />
          <!-- Target highlight -->
          <rect v-if="predictedTarget !== null && processDef" :x="(processDef.nodes[predictedTarget]!.x)-6" :y="(processDef.nodes[predictedTarget]!.y)-6"
            :width="(processDef.nodes[predictedTarget]!.w||120)+12" :height="(processDef.nodes[predictedTarget]!.h||50)+12"
            rx="10" fill="rgba(0,212,255,0.15)" stroke="var(--color-primary)" stroke-width="2" stroke-dasharray="4,2" pointer-events="none" />`;
content = content.replace(tempEdgeSvgMarker, tempEdgeSvgExtra);

// === 5. Enhance enterSubprocess for recursive depth support ===
const oldEnterSub = `function enterSubprocess(nodeIdx: number) {
  if (!processDef.value) return
  const node = processDef.value.nodes[nodeIdx]
  if (node.type !== 'subprocess') return
  subprocessNodeIdx.value = nodeIdx
  subprocessTitle.value = node.label || '子流程'
  // Load subprocess definition
  const subs = (currentProcess.value?.subprocesses as any) || {}
  const subData = subs[node.id] || { nodes: [], edges: [] }
  subprocessDef.value = JSON.parse(JSON.stringify(subData))
  subprocessEditing.value = true
  subSelectedNode.value = null; subSelectedEdge.value = null
  subHistory.value = []; subHistIdx.value = -1
  subPanX.value = 0; subPanY.value = 0; subZoom.value = 1
}`;

const newEnterSub = `function enterSubprocess(nodeIdx: number) {
  if (!processDef.value) return
  const node = processDef.value.nodes[nodeIdx]
  if (node.type !== 'subprocess') return
  // Push current context to stack
  subprocessStack.value.push({
    nodes: processDef.value!.nodes.map(n => ({...n})),
    edges: (processDef.value!.edges || []).map(e => ({...e})),
    title: '主流程' + (subprocessStack.value.length > 0 ? ' #' + (subprocessStack.value.length) : ''),
    parentIdx: subprocessNodeIdx.value
  })
  subprocessDepth.value = subprocessStack.value.length + 1
  subprocessNodeIdx.value = nodeIdx
  subprocessTitle.value = node.label || '子流程'
  // Load subprocess definition
  const subs = (currentProcess.value?.subprocesses as any) || {}
  const subData = subs[node.id] || { nodes: [], edges: [] }
  subprocessDef.value = JSON.parse(JSON.stringify(subData))
  subprocessEditing.value = true
  subSelectedNode.value = null; subSelectedEdge.value = null
  subHistory.value = []; subHistIdx.value = -1
  subPanX.value = 0; subPanY.value = 0; subZoom.value = 1
}

function exitSubprocess() {
  if (subprocessStack.value.length > 0) {
    // Restore parent context
    const parent = subprocessStack.value.pop()!
    activeSubprocessIdx.value = parent.parentIdx ?? null
    subprocessDepth.value = subprocessStack.value.length
    // Restore parent nodes and edges
    processDef.value!.nodes = parent.nodes
    processDef.value!.edges = parent.edges
    subprocessEditing.value = false
    subSelectedNode.value = null; subSelectedEdge.value = null
    pushHistory()
  } else {
    subprocessEditing.value = false
    subSelectedNode.value = null; subSelectedEdge.value = null
  }
}

function jumpToLevel(level: number) {
  while (subprocessStack.value.length > level) {
    exitSubprocess()
  }
}

function getBreadcrumbs(): Array<{label: string; idx: number|null; depth: number}> {
  const crumbs = [{ label: '主流程', idx: null, depth: 0 }]
  for (let i = 0; i < subprocessStack.value.length; i++) {
    crumbs.push({ label: '子流程 #' + (i+1), idx: subprocessStack.value[i].parentIdx ?? null, depth: i+1 })
  }
  if (subprocessEditing.value) {
    crumbs.push({ label: subprocessTitle.value, idx: subprocessNodeIdx.value, depth: subprocessDepth.value })
  }
  return crumbs
}`;

content = content.replace(oldEnterSub, newEnterSub);

// === 6. Enhance template breadcrumb area ===
const oldBreadcrumb = '<span v-if="subprocessEditing">← 返回主流程 | 拖拽节点 | 点击边缘拖出连线 | Shift+多选</span>';
const newBreadcrumb = '<span v-if="subprocessEditing">';
const breadcrumbHtml = `<span v-if="subprocessEditing">`;
const breadcrumbContent = `
          <button class="tb-btn" @click="jumpToLevel(0)" title="返回主流程">🏠 主页</button>
          <template v-for="(crumb, ci) in getBreadcrumbs()" :key="ci">
            <span class="breadcrumb-sep">›</span>
            <button v-if="ci < getBreadcrumbs().length - 1" class="tb-btn" @click="jumpToLevel(ci)">{{ crumb.label }}</button>
            <span v-else class="breadcrumb-current">{{ crumb.label }}</span>
          </template>
          <span class="breadcrumb-sep">|</span>
          <button class="tb-btn" @click="exitSubprocess">✕ 退出层级</button>
          <span> | 拖拽节点 | 点击边缘拖出连线 | Shift+多选</span>`;
const newBreadcrumbFull = breadcrumbHtml + breadcrumbContent + '</span>';
content = content.replace(oldBreadcrumb, newBreadcrumbFull);

// === 7. Add condition expression editor, script variable binding, retry visualization to props ===
const oldConditionProp = '<div class="pg"><label>流转条件</label><input :value="getNodeProp(\'condition\')" @input="_setNodeProp(\'condition\', $event.target.value)" class="pi" placeholder="如: amount > 1000" /></div>';
const newConditionProp = `<div class="pg"><label>流转条件</label>
              <input :value="getNodeProp('condition')" @input="_setNodeProp('condition',$event.target.value)" class="pi" placeholder="如: amount > 1000" />
              <div class="cond-editor" v-if="getNodeProp('type')==='approval' || getNodeProp('type')==='task'">
                <div class="cond-presets">
                  <button class="cond-preset" @click="_setNodeProp('condition','amount > 1000')">金额>1000</button>
                  <button class="cond-preset" @click="_setNodeProp('condition','status === \'pending\'')">状态=pending</button>
                  <button class="cond-preset" @click="_setNodeProp('condition','userId === currentUser')">当前用户</button>
                  <button class="cond-preset" @click="_setNodeProp('condition','')">清空</button>
                </div>
                <div class="cond-vars">
                  <span class="cond-var-label">可用变量:</span>
                  <span v-for="v in nodeVars" :key="v" class="cond-var" @click="_setNodeProp('condition', getNodeProp('condition') + ' ' + v)">{{ v }}</span>
                </div>
              </div>
            </div>`;
content = content.replace(oldConditionProp, newConditionProp);

// === 8. Add script variable binding editor ===
const oldScriptVars = '<div v-if="scriptTab===\'vars\'" class="script-vars">\n                  <div class="var-row"><span class="var-label">输入变量</span><input class="var-input" placeholder="inputData" /></div>\n                  <div class="var-row"><span class="var-label">输出变量</span><input class="var-input" placeholder="output" /></div>\n                  <div class="var-row"><span class="var-label">上下文</span><input class="var-input" placeholder="context" /></div>\n                </div>';
const newScriptVars = `<div v-if="scriptTab==='vars'" class="script-vars">
                  <div class="var-section-title">变量绑定配置</div>
                  <div class="var-row"><span class="var-label">输入变量</span><input class="var-input" :value="getNodeProp('inputVar')" @input="_setNodeProp('inputVar',$event.target.value)" placeholder="inputData" /></div>
                  <div class="var-row"><span class="var-label">输出变量</span><input class="var-input" :value="getNodeProp('outputVar')" @input="_setNodeProp('outputVar',$event.target.value)" placeholder="output" /></div>
                  <div class="var-row"><span class="var-label">上下文</span><input class="var-input" :value="getNodeProp('contextVar')" @input="_setNodeProp('contextVar',$event.target.value)" placeholder="context" /></div>
                  <div class="var-mappings">
                    <div class="var-mapping-title">数据映射</div>
                    <div v-for="(m, mi) in getNodeMappings()" :key="mi" class="var-mapping-row">
                      <select :value="m.from" @change="getNodeMappings()[mi].from=$event.target.value" class="var-mapping-select">
                        <option value="">选择输入字段</option>
                        <option v-for="f in availableFields" :key="f" :value="f">{{ f }}</option>
                      </select>
                      <span class="var-mapping-arrow">→</span>
                      <input :value="m.to" @input="getNodeMappings()[mi].to=$event.target.value" class="var-mapping-input" placeholder="输出字段" />
                      <button class="var-mapping-del" @click="removeDataMapping(mi)">×</button>
                    </div>
                    <button class="var-mapping-add" @click="addDataMapping">+ 添加映射</button>
                  </div>
                </div>`;
content = content.replace(oldScriptVars, newScriptVars);

// === 9. Add retry strategy visualization ===
const oldRetryProp = '<div class="pg"><label>重试次数</label><input :value="getNodeProp(\'retryCount\')" type="number" @input="_setNodeProp(\'retryCount\',+$event.target.value)" class="pi" min="0" max="10" /></div>';
const newRetryProp = `<div class="pg"><label>重试策略</label>
              <select :value="getNodeProp('retryStrategy')" @change="_setNodeProp('retryStrategy',$event.target.value)" class="pi">
                <option value="none">不重试</option>
                <option value="fixed">固定间隔</option>
                <option value="exponential">指数退避</option>
                <option value="linear">线性递增</option>
              </select>
            </div>
            <div class="pg" v-if="getNodeProp('retryStrategy')!=='none'">
              <label>重试次数</label>
              <div class="retry-visual">
                <input :value="getNodeProp('retryCount')" type="range" min="0" max="10" @input="_setNodeProp('retryCount',+$event.target.value)" class="retry-slider" />
                <div class="retry-dots">
                  <span v-for="i in 10" :key="i" :class="['retry-dot', { active: i <= (getNodeProp('retryCount')||0) }]">{{ i <= (getNodeProp('retryCount')||0) ? '●' : '○' }}</span>
                </div>
                <span class="retry-val">{{ getNodeProp('retryCount')||0 }} 次</span>
              </div>
            </div>
            <div class="pg" v-if="getNodeProp('retryStrategy')==='exponential'">
              <label>初始延迟(ms)</label>
              <input :value="getNodeProp('retryDelay')" type="number" @input="_setNodeProp('retryDelay',+$event.target.value)" class="pi" min="100" step="100" placeholder="1000" />
              <label>倍增系数</label>
              <input :value="getNodeProp('retryMultiplier')" type="number" @input="_setNodeProp('retryMultiplier',+$event.target.value)" class="pi" min="1" step="0.5" placeholder="2" />
            </div>
            <div class="pg" v-if="getNodeProp('retryStrategy')==='fixed'">
              <label>固定间隔(ms)</label>
              <input :value="getNodeProp('retryDelay')" type="number" @input="_setNodeProp('retryDelay',+$event.target.value)" class="pi" min="100" step="100" placeholder="1000" />
            </div>`;
content = content.replace(oldRetryProp, newRetryProp);

// === 10. Add nodeVars and availableFields computed ===
const nodeTypesEnd = 'const allNodeTypes = [\'start\',\'task\',\'approval\',\'timer\',\'end\',\'gate_and\',\'gate_or\',\'gate_xor\',\'subprocess\',\'script\',\'parallel\']';
const nodeTypesExtra = `const allNodeTypes = ['start','task','approval','timer','end','gate_and','gate_or','gate_xor','subprocess','script','parallel']

// ── Condition Editor Helpers ────────────────────────────────────────
const nodeVars = ref<string[]>(['amount', 'userId', 'status', 'priority', 'deadline', 'department', 'role'])
const availableFields = ref<string[]>(['name', 'amount', 'status', 'userId', 'priority', 'date', 'comment', 'result', 'output'])`;
content = content.replace(nodeTypesEnd, nodeTypesExtra);

// === 11. Add predicted edge CSS and new UI styles ===
const styleEndMarker = '</style>';
const newStyles = `
/* Path prediction */
.edge-predicted{fill:none;stroke:var(--color-primary);stroke-width:2;opacity:0.7;animation:dashFlow 0.5s linear infinite}
@keyframes dashFlow{from{stroke-dashoffset:0}to{stroke-dashoffset:-18}}
/* Condition editor */
.cond-editor{margin-top:6px;padding:8px;background:var(--bg-elevated);border-radius:var(--radius-sm)}
.cond-presets{display:flex;gap:4px;flex-wrap:wrap;margin-bottom:6px}
.cond-preset{padding:2px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:10px}
.cond-preset:hover{border-color:var(--color-primary);color:var(--color-primary)}
.cond-vars{display:flex;align-items:center;gap:4px;flex-wrap:wrap}
.cond-var-label{font-size:10px;color:var(--text-muted)}
.cond-var{padding:1px 6px;border-radius:var(--radius-sm);background:rgba(0,212,255,.1);color:var(--color-primary);font-size:9px;cursor:pointer;font-family:'JetBrains Mono',monospace}
.cond-var:hover{background:rgba(0,212,255,.2)}
/* Script variable binding */
.script-vars{display:flex;flex-direction:column;gap:6px}
.var-section-title{font-size:11px;font-weight:600;color:var(--color-primary);padding-bottom:4px;border-bottom:1px solid var(--border-color)}
.var-mappings{display:flex;flex-direction:column;gap:4px}
.var-mapping-title{font-size:10px;color:var(--text-muted);margin-top:4px}
.var-mapping-row{display:flex;align-items:center;gap:4px}
.var-mapping-select,.var-mapping-input{padding:2px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:10px;outline:none}
.var-mapping-select{flex:1;cursor:pointer}.var-mapping-input{flex:2}
.var-mapping-arrow{color:var(--color-primary);font-size:10px}
.var-mapping-del{width:16px;height:16px;border-radius:50%;border:none;background:transparent;color:var(--color-danger);cursor:pointer;font-size:12px}
.var-mapping-add{padding:2px 8px;border-radius:var(--radius-sm);border:1px dashed var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:10px;align-self:flex-start}
.var-mapping-add:hover{border-color:var(--color-primary);color:var(--color-primary)}
/* Retry visualization */
.retry-visual{display:flex;align-items:center;gap:8px;padding:4px 0}
.retry-slider{flex:1;-webkit-appearance:none;height:4px;background:var(--border-color);border-radius:2px;outline:none}
.retry-slider::-webkit-slider-thumb{-webkit-appearance:none;width:12px;height:12px;border-radius:50%;background:var(--color-primary);cursor:pointer}
.retry-dots{display:flex;gap:2px}
.retry-dot{font-size:8px;color:var(--border-color);cursor:pointer;transition:color .15s}
.retry-dot.active{color:var(--color-warning)}
.retry-val{font-size:10px;color:var(--text-muted);min-width:30px;text-align:right;font-family:'JetBrains Mono',monospace}
/* Subprocess breadcrumbs */
.breadcrumb-sep{color:var(--text-muted);font-size:12px;margin:0 4px}
.breadcrumb-current{font-size:11px;color:var(--color-primary);font-weight:600;padding:2px 8px;background:var(--color-primary-soft);border-radius:var(--radius-sm)}
.sp-breadcrumbs{display:flex;align-items:center;gap:2px;padding:4px 8px;border-bottom:1px solid var(--border-color);background:rgba(0,212,255,.05);flex-shrink:0}
.sp-depth-badge{padding:1px 6px;border-radius:var(--radius-sm);background:var(--color-primary);color:#000;font-size:9px;font-weight:700;margin-left:8px}
/* Prediction target highlight is handled in SVG */
`;
content = content.replace(styleEndMarker, newStyles + '</style>');

// Write back
fs.writeFileSync('D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/ProcessDesigner.vue', content);
console.log('Done. Lines:', content.split('\n').length);
