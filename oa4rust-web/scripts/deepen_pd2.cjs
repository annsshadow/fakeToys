const fs = require('fs');
let content = fs.readFileSync('D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/ProcessDesigner.vue', 'utf8');

// === 12. Add advanced node type configurations ===
const nodeTypesEnd = "const allNodeTypes = ['start','task','approval','timer','end','gate_and','gate_or','gate_xor','subprocess','script','parallel']";
const nodeTypesExtra = `const allNodeTypes = ['start','task','approval','timer','end','gate_and','gate_or','gate_xor','subprocess','script','parallel']

// ── Advanced Node Configuration ─────────────────────────────────────
interface NodeConfig {
  type: string; label: string; icon: string
  defaultW: number; defaultH: number
  canHaveChildren: boolean
  maxChildren: number
  supportsParallel: boolean
  supportsCondition: boolean
  supportsScript: boolean
  supportsAssignee: boolean
  supportsTimeout: boolean
  supportsRetry: boolean
  supportsDataMapping: boolean
  color: string
}
const nodeConfigs: Record<string, NodeConfig> = {
  start:    { type: 'start', label: '开始', icon: '🟢', defaultW: 100, defaultH: 50, canHaveChildren: false, maxChildren: 0, supportsParallel: false, supportsCondition: false, supportsScript: false, supportsAssignee: false, supportsTimeout: false, supportsRetry: false, supportsDataMapping: false, color: '#10b981' },
  end:      { type: 'end', label: '结束', icon: '🔴', defaultW: 100, defaultH: 50, canHaveChildren: false, maxChildren: 0, supportsParallel: false, supportsCondition: false, supportsScript: false, supportsAssignee: false, supportsTimeout: false, supportsRetry: false, supportsDataMapping: false, color: '#ef4444' },
  task:     { type: 'task', label: '任务', icon: '📋', defaultW: 120, defaultH: 50, canHaveChildren: true, maxChildren: 10, supportsParallel: true, supportsCondition: true, supportsScript: false, supportsAssignee: true, supportsTimeout: true, supportsRetry: true, supportsDataMapping: true, color: '#00d4ff' },
  approval: { type: 'approval', label: '审批', icon: '✅', defaultW: 130, defaultH: 70, canHaveChildren: true, maxChildren: 5, supportsParallel: true, supportsCondition: true, supportsScript: false, supportsAssignee: true, supportsTimeout: true, supportsRetry: true, supportsDataMapping: true, color: '#6366f1' },
  timer:    { type: 'timer', label: '定时', icon: '⏱️', defaultW: 110, defaultH: 50, canHaveChildren: false, maxChildren: 0, supportsParallel: false, supportsCondition: false, supportsScript: true, supportsAssignee: false, supportsTimeout: true, supportsRetry: false, supportsDataMapping: false, color: '#f59e0b' },
  gate_and: { type: 'gate_and', label: '且网关', icon: '🔷', defaultW: 100, defaultH: 50, canHaveChildren: true, maxChildren: 20, supportsParallel: true, supportsCondition: true, supportsScript: false, supportsAssignee: false, supportsTimeout: false, supportsRetry: false, supportsDataMapping: false, color: '#f59e0b' },
  gate_or:  { type: 'gate_or', label: '或网关', icon: '🔶', defaultW: 100, defaultH: 50, canHaveChildren: true, maxChildren: 20, supportsParallel: true, supportsCondition: true, supportsScript: false, supportsAssignee: false, supportsTimeout: false, supportsRetry: false, supportsDataMapping: false, color: '#f59e0b' },
  gate_xor: { type: 'gate_xor', label: '异或网关', icon: '🔹', defaultW: 100, defaultH: 50, canHaveChildren: true, maxChildren: 20, supportsParallel: true, supportsCondition: true, supportsScript: false, supportsAssignee: false, supportsTimeout: false, supportsRetry: false, supportsDataMapping: false, color: '#f59e0b' },
  subprocess: { type: 'subprocess', label: '子流程', icon: '📦', defaultW: 120, defaultH: 60, canHaveChildren: true, maxChildren: 50, supportsParallel: true, supportsCondition: false, supportsScript: false, supportsAssignee: true, supportsTimeout: true, supportsRetry: true, supportsDataMapping: true, color: '#a855f7' },
  script:   { type: 'script', label: '脚本', icon: '💻', defaultW: 120, defaultH: 50, canHaveChildren: false, maxChildren: 0, supportsParallel: false, supportsCondition: false, supportsScript: true, supportsAssignee: false, supportsTimeout: true, supportsRetry: true, supportsDataMapping: true, color: '#22c55e' },
  parallel: { type: 'parallel', label: '并行', icon: '⚡', defaultW: 120, defaultH: 50, canHaveChildren: true, maxChildren: 10, supportsParallel: true, supportsCondition: false, supportsScript: false, supportsAssignee: false, supportsTimeout: false, supportsRetry: false, supportsDataMapping: false, color: '#ec4899' },
}
function getNodeConfig(type: string): NodeConfig {
  return nodeConfigs[type] || nodeConfigs['task']
}
function isGate(type: string): boolean {
  return type === 'gate_and' || type === 'gate_or' || type === 'gate_xor'
}
function getNodeConditions(node: PDNode): string[] {
  const cfg = getNodeConfig(node.type)
  if (!cfg.supportsCondition) return []
  return ['通过', '拒绝', '超时']
}

// ── Node Template Presets ────────────────────────────────────────────
interface NodeTemplate { name: string; icon: string; nodes: Array<{type: string; label: string}>; edges: Array<{from: number; to: number; label?: string}> }
const nodeTemplatesExpanded: NodeTemplate[] = [
  { name: '请假审批', icon: '📝', nodes: [{type:'start',label:'开始'},{type:'task',label:'提交申请'},{type:'approval',label:'主管审批'},{type:'gate_or',label:'金额判断'},{type:'approval',label:'经理审批'},{type:'end',label:'完成'}], edges: [{from:0,to:1},{from:1,to:2},{from:2,to:3},{from:3,to:4,label:'>5000'},{from:3,to:5,label:'<=5000'},{from:4,to:5}] },
  { name: '采购流程', icon: '🛒', nodes: [{type:'start',label:'开始'},{type:'task',label:'创建采购单'},{type:'approval',label:'部门审批'},{type:'approval',label:'财务审批'},{type:'task',label:'执行采购'},{type:'end',label:'完成'}], edges: [{from:0,to:1},{from:1,to:2},{from:2,to:3},{from:3,to:4},{from:4,to:5}] },
  { name: '发布流程', icon: '🚀', nodes: [{type:'start',label:'开始'},{type:'task',label:'代码提交'},{type:'script',label:'自动化测试'},{type:'gate_or',label:'测试通过?'},{type:'approval',label:'人工审核'},{type:'task',label:'部署'},{type:'end',label:'完成'}], edges: [{from:0,to:1},{from:1,to:2},{from:2,to:3},{from:3,to:4,label:'通过'},{from:3,to:5,label:'紧急'},{from:4,to:5},{from:5,to:6}] },
  { name: '并行任务', icon: '⚡', nodes: [{type:'start',label:'开始'},{type:'gate_and',label:'分发'},{type:'task',label:'任务A'},{type:'task',label:'任务B'},{type:'task',label:'任务C'},{type:'gate_and',label:'汇聚'},{type:'end',label:'完成'}], edges: [{from:0,to:1},{from:1,to:2},{from:1,to:3},{from:1,to:4},{from:2,to:5},{from:3,to:5},{from:4,to:5},{from:5,to:6}] },
  { name: '循环重试', icon: '🔄', nodes: [{type:'start',label:'开始'},{type:'task',label:'执行任务'},{type:'gate_or',label:'成功?'},{type:'script',label:'错误处理'},{type:'end',label:'结束'}], edges: [{from:0,to:1},{from:1,to:2},{from:2,to:3,label:'失败'},{from:2,to:4,label:'成功'},{from:3,to:1}] },
  { name: '多级审批', icon: '📑', nodes: [{type:'start',label:'开始'},{type:'task',label:'提交'},{type:'approval',label:'一级审批'},{type:'approval',label:'二级审批'},{type:'approval',label:'三级审批'},{type:'end',label:'完成'}], edges: [{from:0,to:1},{from:1,to:2},{from:2,to:3},{from:3,to:4},{from:4,to:5}] },
]

// ── Edge Style Presets ───────────────────────────────────────────────
interface EdgeStyle { name: string; color: string; width: number; dash: string }
const edgeStylePresets: EdgeStyle[] = [
  { name: '默认', color: 'var(--color-primary)', width: 2, dash: 'none' },
  { name: '虚线', color: 'var(--color-warning)', width: 1.5, dash: '6,4' },
  { name: '粗线', color: 'var(--color-danger)', width: 3, dash: 'none' },
  { name: '点线', color: 'var(--color-info)', width: 1.5, dash: '2,4' },
  { name: '加粗', color: 'var(--color-success)', width: 4, dash: 'none' },
]

// ── Conditional Flow Editor ──────────────────────────────────────────
const showCondEditor = ref(false)
const condEditorField = ref('')
const condEditorOp = ref('>=' as string)
const condEditorValue = ref('')
const condEditorLogic = ref('and' as 'and'|'or')
function openCondEditor() { showCondEditor.value = !showCondEditor.value }
function applyCondExpression() {
  if (!condEditorField.value) return
  const expr = condEditorField.value + ' ' + condEditorOp.value + ' ' + condEditorValue.value
  if (getNodeProp('condition')) {
    _setNodeProp('condition', getNodeProp('condition') + ' ' + condEditorLogic.value + ' ' + expr)
  } else {
    _setNodeProp('condition', expr)
  }
  showCondEditor.value = false
}
function clearCondition() { _setNodeProp('condition', '') }
const condOperators = ['>', '<', '>=', '<=', '===', '!==', 'in', 'contains']
const condFields = ['amount', 'status', 'userId', 'priority', 'deadline', 'department', 'role', 'type', 'result']

// ── Script Binding Editor ────────────────────────────────────────────
const showScriptEditor = ref(false)
const scriptBindingVars = ref<Array<{name: string; type: string; defaultVal: string}>>([
  { name: 'inputData', type: 'object', defaultVal: '{}' },
  { name: 'context', type: 'object', defaultVal: '{}' },
  { name: 'output', type: 'any', defaultVal: 'null' },
])
function addScriptVar() {
  scriptBindingVars.value.push({ name: '', type: 'any', defaultVal: '' })
}
function removeScriptVar(idx: number) {
  scriptBindingVars.value.splice(idx, 1)
}

// ── Retry Strategy Visualizer ────────────────────────────────────────
const showRetryVisualizer = ref(false)
const retryStrategies: Array<{name: string; desc: string; formula: string; example: number[]}> = [
  { name: '固定间隔', desc: '每次重试等待相同时间', formula: 'delay = baseDelay', example: [1000, 1000, 1000, 1000] },
  { name: '线性递增', desc: '每次增加固定延迟', formula: 'delay = baseDelay + attempt * step', example: [1000, 2000, 3000, 4000] },
  { name: '指数退避', desc: '延迟随重试次数指数增长', formula: 'delay = baseDelay * multiplier^attempt', example: [1000, 2000, 4000, 8000] },
  { name: '抖动退避', desc: '指数退避+随机抖动', formula: 'delay = baseDelay * 2^attempt ± jitter', example: [1000, 1800, 3500, 7200] },
]
function getRetryDelays(strategy: string, count: number, baseDelay: number, multiplier: number): number[] {
  const delays: number[] = []
  for (let i = 0; i < count; i++) {
    if (strategy === 'fixed') delays.push(baseDelay)
    else if (strategy === 'linear') delays.push(baseDelay + (i + 1) * baseDelay)
    else if (strategy === 'exponential') delays.push(Math.round(baseDelay * Math.pow(multiplier, i)))
    else if (strategy === 'jitter') delays.push(Math.round(baseDelay * Math.pow(multiplier, i) * (0.8 + Math.random() * 0.4)))
  }
  return delays
}

// ── Subprocess Node Type Config ──────────────────────────────────────
const subNodeTypesExpanded = [
  { type: 'start', label: '开始', icon: '🟢', w: 100, h: 50 },
  { type: 'task', label: '任务', icon: '📋', w: 120, h: 50 },
  { type: 'approval', label: '审批', icon: '✅', w: 130, h: 70 },
  { type: 'end', label: '结束', icon: '🔴', w: 100, h: 50 },
  { type: 'timer', label: '定时', icon: '⏱️', w: 110, h: 50 },
  { type: 'gate_and', label: '且网关', icon: '🔷', w: 100, h: 50 },
  { type: 'gate_or', label: '或网关', icon: '🔶', w: 100, h: 50 },
  { type: 'gate_xor', label: '异或网关', icon: '🔹', w: 100, h: 50 },
  { type: 'subprocess', label: '子流程', icon: '📦', w: 120, h: 60 },
  { type: 'script', label: '脚本', icon: '💻', w: 120, h: 50 },
  { type: 'parallel', label: '并行', icon: '⚡', w: 120, h: 50 },
]

// ── Canvas Zoom Presets ──────────────────────────────────────────────
const zoomPresets = [
  { label: '25%', value: 0.25 }, { label: '50%', value: 0.5 },
  { label: '75%', value: 0.75 }, { label: '100%', value: 1 },
  { label: '150%', value: 1.5 }, { label: '200%', value: 2 },
  { label: '_fit', value: -1 },
]
`
;
content = content.replace(nodeTypesEnd, nodeTypesExtra);

// === 13. Add conditional flow editor to template ===
const oldFlowLabel = '<div class="pg"><label>流向标签</label><input :value="getEdgeProp(\'flowLabel\')" @input="_setEdgeProp(\'flowLabel\', $event.target.value)" class="pi" placeholder="如: 通过/拒绝" /></div>';
const newFlowLabel = `<div class="pg"><label>流向标签</label><input :value="getEdgeProp('flowLabel')" @input="_setEdgeProp('flowLabel',$event.target.value)" class="pi" placeholder="如: 通过/拒绝" /></div>
            <div class="pg"><label>连线样式</label>
              <select :value="getEdgeProp('edgeStyle')" @change="_setEdgeProp('edgeStyle',$event.target.value)" class="pi">
                <option value="default">默认</option><option value="dashed">虚线</option>
                <option value="thick">粗线</option><option value="dotted">点线</option>
              </select>
            </div>`;
content = content.replace(oldFlowLabel, newFlowLabel);

// === 14. Add advanced node config panel to template ===
const propsEmptyMarker = '<div v-else class="props-empty">\n          <p>选择节点或连线编辑属性</p>\n          <p v-if="currentProcess" class="hint">双击子流程节点进入嵌套编辑</p>\n        </div>';
const propsEmptyEnhanced = `<div v-else class="props-empty">
          <p>选择节点或连线编辑属性</p>
          <p class="hint">双击子流程节点进入嵌套编辑</p>
          <div class="quick-actions">
            <button class="btn-sm" @click="showTemplatesModal=true">📐 模板</button>
            <button class="btn-sm" @click="showRulesModal=true">🔗 规则</button>
            <button class="btn-sm" @click="runValidation()">🔍 验证</button>
          </div>
        </div>`;
content = content.replace(propsEmptyMarker, propsEmptyEnhanced);

// === 15. Add CSS for new components ===
const styleEndMarker2 = '</style>';
const extraStyles = `
/* Quick actions in empty props */
.quick-actions{display:flex;gap:4px;margin-top:12px;flex-wrap:wrap}
.quick-actions .btn-sm{flex:1}
/* Condition editor in props */
.cond-editor-panel{margin-top:8px;padding:8px;background:var(--bg-elevated);border-radius:var(--radius-sm);border:1px solid var(--border-color)}
.cond-builder{display:flex;flex-direction:column;gap:6px}
.cond-row{display:flex;align-items:center;gap:4px}
.cond-field-select,.cond-op-select,.cond-val-input{padding:3px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}
.cond-field-select{flex:1;cursor:pointer}.cond-op-select{width:60px;cursor:pointer}.cond-val-input{flex:2}
.cond-logic-select{padding:3px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:10px}
.cond-actions{display:flex;gap:4px;margin-top:4px}
/* Retry strategy visualizer */
.retry-visualizer{margin-top:8px;padding:8px;background:var(--bg-elevated);border-radius:var(--radius-sm)}
.retry-chart{display:flex;align-items:flex-end;gap:4px;height:60px;margin:8px 0}
.retry-bar{flex:1;background:var(--color-warning);border-radius:2px 2px 0 0;min-width:8px;transition:height .3s}
.retry-bar-label{font-size:9px;color:var(--text-muted);text-align:center}
.retry-formula{font-size:10px;color:var(--color-primary);font-family:'JetBrains Mono',monospace;padding:4px;background:rgba(0,212,255,.1);border-radius:var(--radius-sm)}
/* Node config presets */
.config-presets{display:flex;gap:4px;flex-wrap:wrap;margin-top:8px}
.config-preset-btn{padding:4px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:11px}
.config-preset-btn:hover{border-color:var(--color-primary);color:var(--color-primary)}
.config-preset-btn.active{background:var(--color-primary-soft);color:var(--color-primary)}
/* Edge style preview */
.edge-style-preview{display:flex;gap:8px;margin-top:6px;flex-wrap:wrap}
.edge-style-swatch{width:40px;height:20px;border-radius:var(--radius-sm);border:2px solid var(--border-color);cursor:pointer;display:flex;align-items:center;justify-content:center;font-size:10px}
.edge-style-swatch:hover,.edge-style-swatch.active{border-color:var(--color-primary)}
/* Subprocess depth indicator */
.sub-depth-indicator{position:absolute;top:8px;right:8px;z-index:10;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-md);padding:4px 10px;font-size:11px;color:var(--color-primary);font-weight:600}
/* Zoom preset buttons */
.zoom-presets{display:flex;gap:2px;margin-left:8px}
.zoom-preset-btn{padding:2px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:10px;font-family:'JetBrains Mono',monospace}
.zoom-preset-btn:hover,.zoom-preset-btn.active{border-color:var(--color-primary);color:var(--color-primary)}
/* Animation for predicted edge */
@keyframes predictedPulse{0%,100%{opacity:0.5}50%{opacity:1}}
.edge-predicted{animation:predictedPulse 1s ease-in-out infinite}
/* Node type badge colors */
.node-type-badge{display:inline-flex;align-items:center;gap:3px;padding:1px 6px;border-radius:var(--radius-sm);font-size:9px;font-weight:600}
`;
content = content.replace(styleEndMarker2, extraStyles + '</style>');

// Write back
fs.writeFileSync('D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/ProcessDesigner.vue', content);
console.log('Done. Lines:', content.split('\n').length);
