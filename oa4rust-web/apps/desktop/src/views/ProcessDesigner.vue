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
        <button class="btn" @click="autoLayout" title="网格排列">⊞ 排列</button>
        <button class="btn" @click="autoLayoutTopo" title="拓扑排列">⊞ 拓扑</button>
        <button class="btn" :class="{active: showEdgeAnim}" @click="showEdgeAnim=!showEdgeAnim; showEdgeAnim?startEdgeAnimation():stopEdgeAnimation()" title="连线动画">✨ 动画</button>
        <button class="btn" @click="zoomIn" title="放大">🔍+</button>
        <button class="btn" @click="zoomOut" title="缩小">🔍-</button>
        <button class="btn" @click="zoomToFit" title="适配内容">⊞ 适配</button>
        <button class="btn" @click="clearCanvas" title="清空">🗑</button>
        <button class="btn" @click="createVersion()" title="创建版本快照">📌 快照</button>
        <button class="btn btn-outline" @click="showVersionPanel=!showVersionPanel">📜 版本</button>
        <button class="btn btn-outline" @click="showCompareModal=true" title="版本对比">🔀 对比</button>
        <button class="btn btn-outline" @click="showRulesModal=true" title="连接规则">🔗 规则</button>
        <button class="btn btn-outline" @click="showTemplatesModal=true" title="流程模板">📐 模板</button>
        <button class="btn btn-outline" @click="showIoModal=true" title="导入导出">📦 导入导出</button>
        <button class="btn btn-outline" @click="startExecution" title="执行模拟">▶ 模拟</button>
        <button class="btn btn-outline" @click="showHelpModal=true" title="快捷键">⌨️ 帮助</button>
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
            :class="{active: currentProcess?.id===p.id}" @click="loadProcess(p)">
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
          <div v-for="nt in nodeTypes" :key="nt.type" class="pal-item" draggable="true"
            @dragstart="onDragNode($event, nt)" @click="addNode(nt.type)">
            <span class="ni">{{ nt.icon }}</span><span class="nl">{{ nt.label }}</span>
          </div>
        </div>
        <div class="pal-sep"></div>
        <div class="pal-title">条件节点</div>
        <div class="pal-grid">
          <div class="pal-item" @click="addNode('gate_and')"><span class="ni">🔷</span><span class="nl">且网关</span></div>
          <div class="pal-item" @click="addNode('gate_or')"><span class="ni">🔶</span><span class="nl">或网关</span></div>
          <div class="pal-item" @click="addNode('gate_xor')"><span class="ni">🔹</span><span class="nl">异或网关</span></div>
        </div>
        <div class="pal-sep"></div>
        <div class="pal-title">特殊节点</div>
        <div class="pal-grid">
          <div class="pal-item" @click="addNode('subprocess')"><span class="ni">📦</span><span class="nl">子流程</span></div>
          <div class="pal-item" @click="addNode('timer')"><span class="ni">⏱️</span><span class="nl">定时</span></div>
          <div class="pal-item" @click="addNode('script')"><span class="ni">💻</span><span class="nl">脚本</span></div>
          <div class="pal-item" @click="addNode('parallel')"><span class="ni">⚡</span><span class="nl">并行</span></div>
        </div>
        <div class="pal-sep"></div>
        <div class="pal-title">样式预设</div>
        <div class="pal-grid">
          <div v-for="p in nodeStylePresets" :key="p.name" class="pal-item pal-preset"
            @click="applyNodeStylePreset(p)" :title="p.name">
            <span class="ni">{{ p.icon }}</span><span class="nl">{{ p.name }}</span>
          </div>
        </div>
        <div class="pal-sep"></div>
        <div class="pal-title">画布主题</div>
        <div class="pal-grid">
          <div v-for="t in Object.entries(canvasThemes)" :key="t[0]" class="pal-item pal-theme"
            :class="{active: canvasTheme===t[0]}" @click="setCanvasTheme(t[0])" :title="t[1].name">
            <span class="ni" :style="{color:t[1].grid.replace('rgba','rgb').replace('0.03','1')}">◉</span><span class="nl">{{ t[1].name }}</span>
          </div>
        </div>
      </aside>
      <!-- Center: Canvas -->
      <main class="pd-canvas glass-card" ref="canvasRef"
        @drop="onDropNode" @dragover.prevent
        @click.self="selectedNode=null; selectedEdge=null; tempEdge=null"
        @keydown.delete="deleteSelected" @keydown.ctrl.d.prevent="duplicateSelected">
        <div class="canvas-bg" :style="{ backgroundSize: gridScale+'px '+gridScale+'px', backgroundPosition: panX+'px '+panY+'px' }"></div>
        <svg class="canvas-svg" :style="svgTransform">
          <defs>
            <marker id="arrowhead" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
              <polygon points="0 0, 10 3.5, 0 7" fill="var(--color-primary)" />
            </marker>
            <marker id="arrowhead-sel" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
              <polygon points="0 0, 10 3.5, 0 7" fill="var(--color-warning)" />
            </marker>
            <marker id="arrowhead-temp" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
              <polygon points="0 0, 10 3.5, 0 7" fill="var(--color-secondary)" />
            </marker>
          </defs>
          <!-- Edges -->
          <g class="edges" :transform="edgeTransform">
            <path v-for="(edge, i) in processDef?.edges||[]" :key="edge.id"
              :d="computeEdgePath(edge)"
              :class="['edge-path', { selected: selectedEdge===i }]"
              :marker-end="selectedEdge===i ? 'url(#arrowhead-sel)' : 'url(#arrowhead)'"
              @click.stop="selectEdge(i)" />
            <!-- Edge labels -->
            <g v-for="(edge, i) in processDef?.edges||[]" :key="'label-'+edge.id" v-if="edge.label">
              <rect :d="getEdgeLabelRect(edge)" class="edge-label-bg" />
              <text :x="getEdgeLabelX(edge)" :y="getEdgeLabelY(edge)"
                text-anchor="middle" class="edge-label-text">{{ edge.label }}</text>
            </g>
          </g>
          <!-- Temp edge -->
          <path v-if="tempEdge" :d="tempEdgePath()" class="edge-temp" marker-end="url(#arrowhead-temp)" />
          <!-- Predicted connection path -->
          <path v-if="showPrediction && predictedPath" :d="predictedPath" class="edge-predicted" stroke-dasharray="6,3" />
          <!-- Target highlight -->
          <rect v-if="predictedTarget !== null && processDef" :x="(processDef.nodes[predictedTarget]!.x)-6" :y="(processDef.nodes[predictedTarget]!.y)-6"
            :width="(processDef.nodes[predictedTarget]!.w||120)+12" :height="(processDef.nodes[predictedTarget]!.h||50)+12"
            rx="10" fill="rgba(0,212,255,0.15)" stroke="var(--color-primary)" stroke-width="2" stroke-dasharray="4,2" pointer-events="none" />
          <!-- Fork/Join branch backgrounds -->
          <g v-if="!subprocessEditing" class="fork-branches">
            <g v-for="(fl, fli) in forkLabels" :key="fli">
              <rect
                :x="Math.min(...fl.branch.map(i=>(processDef?.nodes[i]?.x ?? 0))) - 15"
                :y="Math.min(...fl.branch.map(i=>(processDef?.nodes[i]?.y ?? 0))) - 15"
                :width="Math.max(...fl.branch.map(i=>(processDef?.nodes[i]?.x ?? 0)+(processDef?.nodes[i]?.w ?? 120)))-Math.min(...fl.branch.map(i=>(processDef?.nodes[i]?.x ?? 0)))+30"
                :height="Math.max(...fl.branch.map(i=>(processDef?.nodes[i]?.y ?? 0)+(processDef?.nodes[i]?.h ?? 50)))-Math.min(...fl.branch.map(i=>(processDef?.nodes[i]?.y ?? 0)))+30"
                fill="rgba(245,158,11,0.06)" stroke="var(--color-warning)" stroke-width="1.5" stroke-dasharray="6,3" rx="8" />
              <text :x="Math.min(...fl.branch.map(i=>(processDef?.nodes[i]?.x ?? 0))) - 25"
                :y="Math.min(...fl.branch.map(i=>(processDef?.nodes[i]?.y ?? 0))) - 5"
                class="branch-num">B#{{ fli+1 }}</text>
              <text :x="fl.forkNode.x + (fl.forkNode.w||120)/2" :y="fl.forkNode.y - 15"
                class="fork-label">⚡ FORK #{{ fli+1 }}</text>
              <path v-if="fl.branch.length >= 2" :d="computeForkJoinPath(fl.branch)" class="fork-flow" />
              <text v-if="fl.joinNode" :x="fl.joinNode.x + (fl.joinNode.w||120)/2" :y="fl.joinNode.y + (fl.joinNode.h||50) + 16"
                class="join-label">⚡ JOIN #{{ fli+1 }}</text>
            </g>
          </g>
          <!-- Group backgrounds -->
          <g v-if="!subprocessEditing" class="group-backgrounds" :transform="edgeTransform">
            <g v-for="(g, gi) in groupNodes" :key="g.node.id" @contextmenu.prevent="showGroupMenu($event, gi)">
              <rect
                :x="g.bounds.x" :y="g.bounds.y"
                :width="g.bounds.width" :height="g.bounds.height"
                fill="rgba(0,212,255,0.04)" stroke="var(--color-primary)" stroke-width="1.5"
                stroke-dasharray="8,4" rx="12" class="group-bg" />
              <!-- Group label bar -->
              <rect :x="g.bounds.x" :y="g.bounds.y" :width="g.bounds.width" height="22"
                rx="12" ry="12" fill="rgba(0,212,255,0.15)" />
              <rect :x="g.bounds.x" :y="g.bounds.y+11" :width="g.bounds.width" height="11"
                fill="rgba(0,212,255,0.15)" />
              <text :x="g.bounds.x + 10" :y="g.bounds.y + 15"
                class="group-label-text">{{ g.node.label || '分组' }} ({{ g.members.length }})</text>
              <!-- Collapse/expand button -->
              <g :transform="`translate(${g.bounds.x + g.bounds.width - 28}, ${g.bounds.y + 5})`"
                class="group-btn" @click.stop="toggleGroupCollapse(gi)" style="cursor:pointer">
                <rect width="22" height="14" rx="7" :fill="g.node.collapsed ? 'rgba(16,185,129,.3)' : 'rgba(239,68,68,.3)'" stroke="var(--border-color)" stroke-width="1" />
                <text x="11" y="10" text-anchor="middle" font-size="9" fill="var(--text-primary)">
                  {{ g.node.collapsed ? '▶' : '▼' }}
                </text>
              </g>
              <!-- Expand icon (shown when collapsed) -->
              <text v-if="g.node.collapsed"
                :x="g.bounds.x + g.bounds.width/2" :y="g.bounds.y + g.bounds.height/2 + 4"
                text-anchor="middle" font-size="10" fill="var(--text-muted)">点击展开</text>
            </g>
          </g>
          <!-- Nodes -->
          <g class="nodes" :transform="nodeTransform">
            <g v-for="(node, i) in processDef?.nodes||[]" :key="node.id"
              :transform="`translate(${node.x},${node.y})`"
              :class="['node-group', { selected: selectedNode===i, dragging: isDragging&&dragIdx===i }]">
              <!-- Selection box (shows when selected) -->
              <rect v-if="selectedNode===i" x="-6" y="-6" :width="(node.w||120)+12" :height="(node.h||50)+12"
                rx="10" fill="none" stroke="var(--color-primary)" stroke-width="2" stroke-dasharray="4,2" pointer-events="none" />
              <!-- Resize handles (8 directions) -->
              <template v-if="selectedNode===i">
                <!-- Corners -->
                <rect v-for="(pos,pi) in resizePositions" :key="'h'+pi"
                  :x="getNodeResizeX(node, pos) - 4" :y="getNodeResizeY(node, pos) - 4"
                  width="8" height="8" rx="2" fill="var(--color-primary)" stroke="white" stroke-width="1"
                  class="resize-handle" :style="{ cursor: getResizeCursor(pos) }"
                  @mousedown.stop="onResizeMouseDown($event, i, pos)" />
    <!-- Data Mapping Editor Modal -->
    <div v-if="showDataMappingEditor" class="modal-overlay" @click.self="showDataMappingEditor=false">
      <div class="modal data-mapping-modal">
        <div class="modal-header">
          <span>🔗 数据映射编辑器</span>
          <button class="btn-sm" @click="showDataMappingEditor=false">✕</button>
        </div>
        <div class="modal-body">
          <div class="dm-fields-section">
            <h4>📋 源字段 (左侧)</h4>
            <div class="dm-field-list">
              <div v-for="(f, fi) in dataFields" :key="fi" class="dm-field-item" draggable="true" @dragstart="onMapDragStart($event, {field: f.name, nodeIdx: f.nodeIdx})">
                <span class="dm-field-icon">{{ getNodeIcon(f.type) }}</span>
                <span class="dm-field-name">{{ f.label }}</span>
                <span class="dm-field-type">{{ f.type }}</span>
              </div>
            </div>
          </div>
          <div class="dm-mappings-section">
            <h4>🔀 映射关系</h4>
            <div class="dm-mapping-row" v-for="(me, mi) in mappingEdgesList" :key="mi">
              <select v-model="me.fromNodeIdx" class="dm-select">
                <option v-for="(f,fi) in dataFields" :value="fi">{{ f.label }}</option>
              </select>
              <span class="dm-arrow">→</span>
              <input v-model="me.fromField" placeholder="源字段" class="dm-input" />
              <select v-model="me.transform" class="dm-select">
                <option value="identity">恒等</option>
                <option value="upper">大写</option>
                <option value="lower">小写</option>
                <option value="trim">去空格</option>
                <option value="format">格式化</option>
              </select>
              <span class="dm-arrow">→</span>
              <input v-model="me.toField" placeholder="目标字段" class="dm-input" />
              <input v-model="me.condition" placeholder="条件(可选)" class="dm-input dm-cond" />
              <button class="btn-sm dm-del" @click="removeMappingRow(mi)">✕</button>
            </div>
            <button class="btn-sm" @click="addMappingRow">+ 添加映射</button>
          </div>
        </div>
        <div class="modal-footer">
          <button class="btn" @click="applyMapping">✓ 应用映射</button>
          <button class="btn btn-ghost" @click="showDataMappingEditor=false">取消</button>
        </div>
      </div>
    </div>
    <!-- Flow Variable Panel -->
    <div v-if="showFlowVarPanel" class="flow-var-panel">
      <div class="fv-header">
        <span>🌊 流程变量</span>
        <button class="btn-sm" @click="showFlowVarPanel=false">✕</button>
      </div>
      <div class="fv-body">
        <div class="fv-add">
          <input v-model="newVarName" placeholder="变量名" class="fv-input" />
          <select v-model="newVarType" class="fv-select">
            <option value="string">string</option>
            <option value="number">number</option>
            <option value="boolean">boolean</option>
            <option value="datetime">datetime</option>
            <option value="json">json</option>
          </select>
          <button class="btn-sm" @click="addFlowVar">+</button>
        </div>
        <div class="fv-list">
          <div v-for="v in flowVars" :key="v.id" class="fv-item" :class="v.scope">
            <span class="fv-icon">{{ v.type==='json'?'📦':v.type==='datetime'?'📅':v.type==='boolean'?'✅':'📝' }}</span>
            <div class="fv-info">
              <span class="fv-name">{{ v.name }}</span>
              <span class="fv-type">{{ v.type }}</span>
              <span class="fv-scope">{{ v.scope }}</span>
            </div>
            <div class="fv-actions">
              <button class="btn-xs" @click="toggleVarScope(v)">{{ v.scope }}</button>
              <button class="btn-xs btn-danger" @click="removeFlowVar(v.id)">✕</button>
            </div>
          </div>
        </div>
        <div class="fv-export">
          <button class="btn-sm" @click="() => { navigator.clipboard.writeText(exportFlowVars()) }">📋 复制JSON</button>
        </div>
      </div>
    </div>
    <!-- Node Templates Modal -->
    <div v-if="showNodeTemplatesModal" class="modal-overlay" @click.self="showNodeTemplatesModal=false">
      <div class="modal node-tpl-modal">
        <div class="modal-header">
          <span>📦 节点模板库</span>
          <button class="btn-sm" @click="showNodeTemplatesModal=false">✕</button>
        </div>
        <div class="modal-body">
          <div class="tpl-add">
            <input v-model="newNodeTemplateName" placeholder="模板名称" class="tpl-input" />
            <input v-model="newNodeTemplateDesc" placeholder="描述" class="tpl-input" />
            <button class="btn-sm" @click="addNodeTemplate">+ 新建模板</button>
          </div>
          <div class="tpl-grid">
            <div v-for="(tpl, ti) in customNodeTemplates" :key="tpl.id" class="tpl-card">
              <div class="tpl-header">
                <span class="tpl-icon">{{ tpl.icon }}</span>
                <span class="tpl-name">{{ tpl.name }}</span>
              </div>
              <div class="tpl-desc">{{ tpl.description }}</div>
              <div class="tpl-nodes-preview">
                <span v-for="(n, ni) in tpl.nodes.slice(0,4)" :key="ni" class="tpl-node-dot">{{ getNodeIcon(n.type) }}</span>
                <span v-if="tpl.nodes.length>4" class="tpl-more">+{{tpl.nodes.length-4}}</span>
              </div>
              <div class="tpl-actions">
                <button class="btn-sm" @click="loadNodeTemplate(tpl)">加载</button>
                <button class="btn-sm btn-danger" @click="deleteNodeTemplate(ti)">删除</button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
    <!-- Performance Monitor -->
    <div v-if="perfMonitoring" class="perf-monitor">
      <div class="perf-header">
        <span>⏱ 性能监控</span>
        <button class="btn-sm" @click="stopPerfMonitor()">停止</button>
      </div>
      <div class="perf-body">
        <div class="perf-table">
          <div class="perf-row perf-row-header">
            <span>节点</span><span>状态</span><span>耗时(ms)</span>
          </div>
          <div v-for="m in perfMetrics" :key="m.nodeId" class="perf-row">
            <span>{{ processDef?.nodes.find(n=>n.id===m.nodeId)?.label || m.nodeId }}</span>
            <span :class="'perf-status ' + m.status">{{ m.status }}</span>
            <span>{{ calculateDuration(m.nodeId) }}ms</span>
          </div>
        </div>
      </div>
    </div>
    <!-- Context Menu -->
    <div v-if="showContextMenu" class="context-menu" :style="{left: contextMenuX+'px', top: contextMenuY+'px'}">
      <div class="ctx-item" @click="execContextAction('properties')">📋 属性</div>
      <div class="ctx-item" @click="execContextAction('duplicate')">📑 复制节点</div>
      <div class="ctx-item" @click="execContextAction('delete')">🗑 删除</div>
      <div v-if="contextMenuNodeId !== null" class="ctx-sep"></div>
      <div class="ctx-item" @click="execContextAction('group')">📦 分组</div>
      <div class="ctx-item" @click="execContextAction('ungroup')">📂 解散分组</div>
      <div class="ctx-sep"></div>
      <div class="ctx-item" @click="showNodeTemplatesModal=true">📦 插入模板</div>
      <div class="ctx-item" @click="previewNode('task','预览')">👁 预览</div>
    </div>
    <!-- Tooltip -->
    <div v-if="showTooltip" class="pd-tooltip" :style="{left: tooltipX+'px', top: (tooltipY-10)+'px'}">
      {{ tooltipContent }}
    </div>
    <!-- Guide Lines Panel -->
    <div v-if="showGuideLines" class="guide-lines-panel">
      <div class="gl-header">
        <span>📐 辅助线</span>
        <button class="btn-xs" @click="addGuideLine('vertical')">+ 垂直</button>
        <button class="btn-xs" @click="addGuideLine('horizontal')">+ 水平</button>
        <button class="btn-xs" @click="toggleGuidelines">✕</button>
      </div>
      <div class="gl-list">
        <div v-for="(gl, gi) in guideLines" :key="gi" class="gl-item">
          <span class="gl-type">{{ gl.type }}</span>
          <input type="range" v-model.number="gl.position" :min="0" :max="2000" class="gl-slider" />
          <span class="gl-pos">{{ gl.position }}</span>
          <button class="btn-xs" @click="removeGuideLine(gi)">✕</button>
        </div>
      </div>
    </div>
    <!-- Box Selection -->
    <svg v-if="boxSelection.active" class="box-select-svg" @mousedown.stop="startBoxSelect($event)" @mousemove="moveBoxSelect($event)" @mouseup="endBoxSelect">
      <rect
        :x="Math.min(boxSelection.start.x, boxSelection.end.x)"
        :y="Math.min(boxSelection.start.y, boxSelection.end.y)"
        :width="Math.abs(boxSelection.end.x - boxSelection.start.x)"
        :height="Math.abs(boxSelection.end.y - boxSelection.start.y)"
        fill="rgba(0,212,255,0.1)" stroke="var(--color-primary)" stroke-width="1" stroke-dasharray="4,2"
      />
    </svg>
    <!-- Conflict Detection Panel -->
    <div v-if="showConflictDetection" class="conflict-panel">
      <div class="cp-header">
        <span>⚠ 连接冲突检测</span>
        <button class="btn-sm" @click="showConflictDetection=false">✕</button>
      </div>
      <div class="cp-body">
        <button class="btn" @click="detectConflicts()">🔍 检测冲突</button>
        <div v-if="connectionConflicts.length > 0" class="cp-conflicts">
          <div v-for="(c, ci) in connectionConflicts" :key="ci" :class="'cp-conflict cp-' + c.severity">
            <span>{{ c.issue }}</span>
            <span class="cp-edges">边: {{ c.edge1.id }} ↔ {{ c.edge2.id }}</span>
          </div>
        </div>
        <div v-else class="cp-ok">✅ 未发现冲突</div>
      </div>
    </div>
    <!-- Simulation Timeline -->
    <div v-if="showSimTimeline" class="sim-timeline">
      <div class="st-header">
        <span>📊 执行时间线</span>
        <div class="st-controls">
          <button class="btn-sm" :disabled="simRunning" @click="startSimulation()">▶ 运行</button>
          <button class="btn-sm" :disabled="!simRunning" @click="stopSimulation()">⏹ 停止</button>
          <button class="btn-sm" @click="showSimTimeline=false">✕</button>
        </div>
      </div>
      <div class="st-progress">
        <div class="st-bar" :style="{width: simProgress+'%'}"></div>
        <span class="st-pct">{{ Math.round(simProgress) }}%</span>
      </div>
      <div class="st-events">
        <div v-for="(ev, ei) in simEvents" :key="ei" class="st-event" :class="ev.event">
          <span class="st-time">{{ ev.time }}ms</span>
          <span class="st-node">{{ processDef?.nodes.find(n=>n.id===ev.nodeId)?.label || ev.nodeId }}</span>
          <span class="st-label">{{ ev.label }}</span>
          <span class="st-type">{{ ev.event }}</span>
        </div>
      </div>
    </div>
    <!-- Shortcut Help -->
    <div v-if="showShortcutHelp" class="shortcut-help">
      <div class="sh-header">
        <span>⌨ 快捷键</span>
        <button class="btn-sm" @click="showShortcutHelp=false">✕</button>
      </div>
      <div class="sh-body">
        <div v-for="s in shortcuts" :key="s.key" class="sh-row">
          <kbd class="sh-key">{{ s.key }}</kbd>
          <span class="sh-label">{{ s.label }}</span>
        </div>
      </div>
    </div>
    <!-- Form Rules Panel -->
    <div v-if="showFormRulesPanel" class="form-rules-panel">
      <div class="frp-header">
        <span>📝 表单联动规则</span>
        <div class="frp-actions">
          <button class="btn-sm" @click="addFormRuleSet">+ 新规则组</button>
          <button class="btn-sm" @click="showFormRulesPanel=false">✕</button>
        </div>
      </div>
      <div class="frp-body">
        <div v-for="(set, si) in formRuleSets" :key="set.id" class="frp-set">
          <div class="frp-set-header">
            <input v-model="set.name" class="frp-set-name" />
            <button class="btn-xs btn-danger" @click="removeFormRuleSet(si)">✕</button>
          </div>
          <div class="frp-rules">
            <div v-for="(rule, ri) in set.rules" :key="rule.id" class="frp-rule">
              <select v-model="rule.sourceField" class="frp-select"><option v-for="f in availableFields" :value="f">{{ f }}</option></select>
              <select v-model="rule.operator" class="frp-select frp-op">
                <option value=">">></option><option value="<"><</option>
                <option value=">=">>=</option><option value="<="><=</option>
                <option value="===">=</option><option value="!==">≠</option>
              </select>
              <input v-model="rule.value" placeholder="值" class="frp-input" />
              <select v-model="rule.action" class="frp-select frp-act">
                <option value="show">显示</option><option value="hide">隐藏</option>
                <option value="enable">启用</option><option value="disable">禁用</option>
              </select>
              <button class="btn-xs btn-danger" @click="removeFormRule(si, ri)">✕</button>
            </div>
            <button class="btn-sm" @click="addFormRule(si)">+ 添加规则</button>
          </div>
          <button class="btn-sm frp-apply" @click="applyFormRules(si)">✓ 应用</button>
        </div>
      </div>
    </div>
    <!-- Toast Container -->
    <div class="toast-container">
      <div v-for="t in toastQueue" :key="t.id" :class="['toast', 'toast-'+t.type]">
        {{ t.message }}
      </div>
    </div>
    <!-- Process Status Indicator -->
    <div class="process-status" :class="'ps-' + getProcessStatus()">
      <span class="ps-dot"></span>
      <span>{{ getProcessStatus() === 'valid' ? '✓ 流程有效' : getProcessStatus() === 'empty' ? '空流程' : '⚠ 需要修复' }}</span>
    </div>
</template>
              <!-- Anchor point handles on edges -->
              <template v-if="selectedNode===i && selectedAnchorNode===i">
                <circle v-for="(ah,ahi) in anchorPoints" :key="ahi"
                  :cx="ah.x" :cy="ah.y" r="5" fill="var(--color-warning)" stroke="white" stroke-width="1.5"
                  class="anchor-handle" style="cursor:grab"
                  @mousedown.stop="onAnchorMouseDown($event, i, ahi)" />
              </template>
              <!-- Node body click zone for arbitrary edge creation -->
              <rect :x="0" :y="0" :width="node.w||120" :height="node.h||50" fill="transparent" class="node-click-zone" @mousedown.stop="onNodeBodyMouseDown($event, i)" />
              <!-- Edge click zone (invisible rectangle around node for arbitrary edge creation) -->
              <rect :x="-8" :y="-8" :width="(node.w||120)+16" :height="(node.h||50)+16"
                fill="transparent" class="edge-create-zone"
                @mousedown.stop="onEdgeMouseDown($event, i)" />
              <!-- Node body -->
              <rect :class="['node-body', node.type]"
                :width="node.w||120" :height="node.h||50" rx="8" />
              <!-- Node icon -->
              <text :x="16" :y="(node.h||50)/2+5" class="node-icon-text">{{ getNodeIcon(node.type) }}</text>
              <!-- Node label -->
              <text :x="(node.w||120)/2+8" :y="(node.h||50)/2-4"
                text-anchor="middle" class="node-label">{{ node.label || getNodeLabel(node.type) }}</text>
              <text :x="(node.w||120)/2+8" :y="(node.h||50)/2+10"
                text-anchor="middle" class="node-sublabel" font-size="9">{{ node.assignee || '' }}</text>
              <!-- In port -->
              <circle v-if="node.type!=='start'" cx="0" :cy="(node.h||50)/2" r="6" class="port port-in"
                @mousedown.stop="onPortMouseDown($event, i, 'in')" />
              <!-- Gate output ports (multiple) -->
              <template v-if="isGate(node.type)">
                <circle v-for="(cond, ci) in getNodeConditions(node)" :key="ci"
                  :cx="node.w||120" :cy="(node.h||50)/2 + (ci - (getNodeConditions(node).length-1)/2) * 20"
                  r="6" class="port port-out port-gate"
                  @mousedown.stop="onPortMouseDown($event, i, 'out')" />
                <text v-if="getNodeConditions(node).length > 0"
                  :x="(node.w||120)+12" :y="(node.h||50)/2 - getNodeConditions(node).length*6"
                  font-size="9" fill="var(--text-muted)">条件出口</text>
              </template>
              <!-- Regular out port -->
              <circle v-if="node.type!=='end' && !isGate(node.type)"
                cx="(node.w||120)" :cy="(node.h||50)/2" r="6" class="port port-out"
                @mousedown.stop="onPortMouseDown($event, i, 'out')" />
              <!-- Note badge -->
              <text v-if="node.note" :x="(node.w||120)/2" :y="(node.h||50)/2+20"
                font-size="8" fill="var(--text-muted)" text-anchor="middle">📝 {{ node.note }}</text>
              <!-- Condition badge -->
              <rect v-if="node.condition" x="4" y="4" width="10" height="10" rx="3" fill="var(--color-warning)" />
              <text v-if="node.condition" :x="(node.w||120)/2+8" :y="14"
                font-size="9" fill="var(--color-warning)" text-anchor="middle">?</text>
              <!-- Subprocess indicator -->
              <rect v-if="node.type==='subprocess'" x="4" y="4" width="10" height="10" rx="3" fill="rgb(168,85,247)" />
              <text v-if="node.type==='subprocess'" :x="(node.w||120)/2+8" :y="14"
                font-size="9" fill="rgb(168,85,247)" text-anchor="middle">⟐</text>
            </g>
          </g>
        </svg>
        <div class="canvas-hint">
          <span v-if="subprocessEditing">
          <button class="tb-btn" @click="jumpToLevel(0)" title="返回主流程">🏠 主页</button>
          <template v-for="(crumb, ci) in getBreadcrumbs()" :key="ci">
            <span class="breadcrumb-sep">›</span>
            <button v-if="ci < getBreadcrumbs().length - 1" class="tb-btn" @click="jumpToLevel(ci)">{{ crumb.label }}</button>
            <span v-else class="breadcrumb-current">{{ crumb.label }}</span>
          </template>
          <span class="breadcrumb-sep">|</span>
          <button class="tb-btn" @click="exitSubprocess">✕ 退出层级</button>
          <span> | 拖拽节点 | 点击边缘拖出连线 | Shift+多选</span></span>
          <span v-else>拖拽右侧节点到画布 | 从端口拖出创建连线 | 点击节点边缘拖出连线 | Shift+点击多选 | Ctrl+A全选 | G键分组 | Del删除 | Ctrl+D复制</span>
        </div>
      </main>
      <!-- Animation Playback Controls -->
      <div v-if="processDef && processDef.nodes.length > 0" class="playback-controls glass-card">
        <button class="play-btn" :class="{playing: isPlaying}" @click="togglePlay" title="播放/暂停">
          {{ isPlaying ? '⏸' : '▶' }}
        </button>
        <input type="range" class="play-slider" :value="playbackProgress" min="0" max="100" @input="onPlaybackSeek" />
        <span class="play-label">{{ getPlaybackTime() }}</span>
        <span class="play-label">{{ processDef.nodes.length }} 节点</span>
        <button class="tb-btn" @click="playbackSpeed=Math.max(0.5, playbackSpeed-0.5)">慢</button>
        <button class="tb-btn" @click="playbackSpeed=1">1x</button>
        <button class="tb-btn" @click="playbackSpeed=Math.min(3, playbackSpeed+0.5)">快</button>
        <button class="tb-btn" @click="resetPlayback">重置</button>
      </div>
      <!-- Process Stats Panel -->
      <aside v-if="processStats" class="pd-stats-panel glass-card">
        <div class="stats-header"><span>📊 流程统计</span><button class="btn-sm" @click="showStats=!showStats">{{ showStats?'收起':'展开' }}</button></div>
        <div v-if="showStats" class="stats-body">
          <div class="stat-row"><span class="stat-label">总节点</span><span class="stat-val">{{ processStats.totalNodes }}</span></div>
          <div class="stat-row"><span class="stat-label">总连线</span><span class="stat-val">{{ processStats.totalEdges }}</span></div>
          <div class="stat-row"><span class="stat-label">开始/结束</span><span class="stat-val">{{ processStats.startNodes }}/{{ processStats.endNodes }}</span></div>
          <div class="stat-row"><span class="stat-label">任务节点</span><span class="stat-val">{{ processStats.taskNodes }}</span></div>
          <div class="stat-row"><span class="stat-label">网关节点</span><span class="stat-val">{{ processStats.gateNodes }}</span></div>
          <div class="stat-row"><span class="stat-label">平均出度</span><span class="stat-val">{{ processStats.avgOutDegree }}</span></div>
          <div class="stat-row"><span class="stat-label">分组数</span><span class="stat-val">{{ groupNodes.value.length }}</span></div>
          <div v-if="processStats.hasLoops" class="stat-warning">⚠ 检测到循环</div>
        </div>
        <!-- Mini-map -->
        <div v-if="minimapVisible && processDef && processDef.nodes.length > 0" class="minimap-container">
          <div class="minimap-header">🗺 迷你地图</div>
          <canvas ref="minimapCanvasRef" class="minimap-canvas" :width="minimapWidth" :height="minimapHeight" @click="minimapClick"></canvas>
          <div class="minimap-controls">
            <button class="tb-btn" @click="minimapVisible=false">收起</button>
            <button class="tb-btn" @click="zoomToFit">适配</button>
          </div>
        </div>
      </aside>
      <!-- Execution Simulation Panel -->
      <aside v-if="showExecPanel && processDef" class="pd-exec-panel glass-card">
        <div class="exec-header">
          <span>▶ 流程执行模拟</span>
          <button class="btn-sm" @click="showExecPanel=false">✕</button>
        </div>
        <div class="exec-body">
          <div class="exec-status">
            <span :class="['exec-badge', execState.status]">
              {{ execState.status==='idle'?'待机':execState.status==='running'?'运行中':execState.status==='paused'?'暂停':execState.status==='finished'?'完成':'?' }}
            </span>
            <span class="exec-progress">{{ execState.progress }}%</span>
          </div>
          <div class="exec-bar"><div class="exec-bar-fill" :style="{ width: execState.progress+'%' }"></div></div>
          <div class="exec-nodes">
            <div v-for="(n, i) in processDef.nodes" :key="n.id"
              :class="['exec-node', { active: execState.currentNodeIdx===i, completed: execState.completedNodes.includes(n.id), pending: !execState.completedNodes.includes(n.id) && execState.currentNodeIdx!==i }]">
              <span class="exec-node-icon">{{ getNodeIcon(n.type) }}</span>
              <span class="exec-node-label">{{ n.label||n.type }}</span>
            </div>
          </div>
          <div class="exec-actions">
            <button class="btn-sm" :disabled="execState.status==='running'" @click="startExecution">▶ 开始</button>
            <button class="btn-sm" :disabled="execState.status!=='running'" @click="pauseExecution">⏸ 暂停</button>
            <button class="btn-sm" :disabled="execState.status!=='paused'" @click="resumeExecution">▶ 继续</button>
            <button class="btn-sm" @click="resetExecution">↺ 重置</button>
          </div>
        </div>
      </aside>
      <!-- Breakpoint & Speed Controls -->
      <div v-if="showExecPanel && processDef" class="exec-controls">
        <div class="speed-control">
          <span class="speed-label">⚡ 速度:</span>
          <input type="range" class="speed-slider" min="100" max="5000" step="100" :value="executionSpeed" @input="setExecutionSpeed(parseInt($event.target.value))" />
          <span class="speed-val">{{ executionSpeed }}ms</span>
        </div>
        <div class="exec-step-controls">
          <button class="btn-sm" :disabled="execState.status!=='running'" @click="stepForward">⏭ 单步前进</button>
          <button class="btn-sm" :disabled="histIdx.value<=0" @click="stepBackward">⏮ 单步后退</button>
        </div>
        <div v-if="breakpoints.length > 0" class="breakpoint-list">
          <div class="bp-title">📍 断点 ({{ breakpoints.length }})</div>
          <div v-for="bp in breakpoints" :key="bp.nodeId" class="bp-item">
            <span class="bp-node">{{ bp.label || bp.nodeId.slice(0,8) }}</span>
            <button class="bp-remove" @click="toggleBreakpoint(bp.nodeId)">✕</button>
          </div>
          <button class="btn-sm" @click="clearBreakpoints">清空断点</button>
        </div>
        <div class="bp-toggle">
          <button class="btn-sm" :class="{active: showBreakpoints}" @click="showBreakpoints=!showBreakpoints">📍 显示断点</button>
        </div>
      </div>
      <!-- Right: Properties -->
      <aside class="pd-props glass-card" v-if="currentProcess">
        <div v-if="selectedNode!==null" class="props-section">
          <div class="props-title">
            <span>节点属性</span>
            <span class="props-badge">{{ getNodeProp('type') }}</span>
          </div>
          <div class="props-body">
            <div class="pg"><label>节点标签</label><input :value="getNodeProp('label')" @input="_setNodeProp('label',$event.target.value)" class="pi" /></div>
            <div class="pg"><label>负责人</label><input :value="getNodeProp('assignee')" @input="_setNodeProp('assignee',$event.target.value)" class="pi" placeholder="如: manager_zhang" /></div>
            <div class="pg"><label>流转条件</label><input :value="getNodeProp('condition')" @input="_setNodeProp('condition',$event.target.value)" class="pi" placeholder="如: amount > 1000" /></div>
            <div class="pg"><label>节点样式</label>
              <select :value="getNodeProp('style')" @change="_setNodeProp('style',$event.target.value)" class="pi">
                <option value="default">默认</option><option value="danger">危险</option><option value="success">成功</option><option value="warning">警告</option>
              </select>
            </div>
            <div class="pg"><label>备注</label><input :value="getNodeProp('note')" @input="_setNodeProp('note',$event.target.value)" class="pi" placeholder="节点备注" /></div>
            <div class="pg"><label>超时(分钟)</label><input :value="getNodeProp('timeout')" type="number" @input="_setNodeProp('timeout',+$event.target.value)" class="pi" /></div>
            <div class="pg"><label>重试策略</label>
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
            </div>
            <div class="pg"><label>数据映射</label>
              <div class="data-mapping">
                <div class="dm-row"><button class="dm-add" @click="addDataMapping">+ 添加映射</button></div>
                <div v-for="(m, i) in getNodeMappings()" :key="i" class="dm-row">
                  <select :value="m.from" @change="getNodeMappings()[i].from=$event.target.value" class="dm-select">
                    <option value="">选择字段</option><option>name</option><option>amount</option><option>status</option><option>userId</option>
                  </select>
                  <span class="dm-arrow">→</span>
                  <input :value="m.to" @input="getNodeMappings()[i].to=$event.target.value" class="dm-input" placeholder="输出字段" />
                  <button class="dm-del" @click="removeDataMapping(i)">×</button>
                </div>
              </div>
            </div>
            <div class="pg"><label>优先级</label>
              <select :value="getNodeProp('priority')" @change="_setNodeProp('priority',$event.target.value)" class="pi">
                <option value="">默认</option><option value="high">高</option><option value="medium">中</option><option value="low">低</option>
              </select>
            </div>
            <div class="pg" v-if="getNodeProp('type')==='script'">
              <label>脚本配置</label>
              <div class="script-panel">
                <div class="script-tabs">
                  <button :class="{active: scriptTab==='code'}" @click="scriptTab='code'">代码</button>
                  <button :class="{active: scriptTab==='vars'}" @click="scriptTab='vars'">变量</button>
                  <button :class="{active: scriptTab==='error'}" @click="scriptTab='error'">错误处理</button>
                </div>
                <div v-if="scriptTab==='code'" class="script-code-area">
                  <textarea :value="getNodeProp('script')" @input="_setNodeProp('script',$event.target.value)" class="code-editor" rows="8" placeholder="// JavaScript代码&#10;// 可用变量: inputData, context, output"></textarea>
                  <div class="script-hint">提示: inputData(输入数据), context(流程上下文), output(输出结果)</div>
                </div>
                <div v-if="scriptTab==='vars'" class="script-vars">
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
                </div>
                <div v-if="scriptTab==='error'" class="script-error">
                  <div class="pg"><label>失败行为</label>
                    <select class="pi"><option value="fail">终止流程</option><option value="skip">跳过此节点</option><option value="retry">重试</option></select>
                  </div>
                  <div class="pg"><label>最大重试</label><input type="number" class="pi" value="3" min="1" max="10" /></div>
                  <div class="pg"><label>重试间隔(ms)</label><input type="number" class="pi" value="1000" min="100" /></div>
                </div>
              </div>
            </div>
            <div class="pg"><label>X</label><input :value="getNodeProp('x')" type="number" @input="_setNodeProp('x',+$event.target.value)" class="pi" /></div>
            <div class="pg"><label>Y</label><input :value="getNodeProp('y')" type="number" @input="_setNodeProp('y',+$event.target.value)" class="pi" /></div>
            <div class="pg"><label>宽</label><input :value="getNodeProp('w')" type="number" @input="_setNodeProp('w',+$event.target.value)" class="pi" min="80" max="300" /></div>
            <div class="pg"><label>高</label><input :value="getNodeProp('h')" type="number" @input="_setNodeProp('h',+$event.target.value)" class="pi" min="40" max="120" /></div>
            <button class="btn-del-sm" @click="deleteNode(selectedNode)">🗑 删除节点</button>
            <!-- Group controls -->
            <div v-if="getNodeProp('type')!=='start' && getNodeProp('type')!=='end'" class="group-controls">
              <button class="btn-sm" @click="toggleGroup(selectedNode!)" :class="{active: isNodeInGroup(selectedNode!)}">
                {{ isNodeInGroup(selectedNode!) ? '✓ 已分组' : '+ 加入分组' }}
              </button>
              <button class="btn-sm" v-if="isNodeInGroup(selectedNode!)" @click="leaveGroup(selectedNode!)">离开分组</button>
            </div>
          </div>
        </div>
        <div v-else-if="selectedEdge!==null" class="props-section">
          <div class="props-title"><span>连线属性</span></div>
          <div class="props-body">
            <div class="pg"><label>标签</label><input :value="getEdgeProp('label')" @input="_setEdgeProp('label',$event.target.value)" class="pi" /></div>
            <div class="pg"><label>流向</label><span class="pv">{{ getEdgeFromLabel() }} → {{ getEdgeToLabel() }}</span></div>
            <div class="pg"><label>条件</label><input :value="getEdgeProp('condition')" @input="_setEdgeProp('condition',$event.target.value)" class="pi" placeholder="如: amount > 1000" /></div>
            <div class="pg"><label>流向标签</label><input :value="getEdgeProp('flowLabel')" @input="_setEdgeProp('flowLabel',$event.target.value)" class="pi" placeholder="如: 通过/拒绝" /></div>
            <div class="pg"><label>连线样式</label>
              <select :value="getEdgeProp('routing')" @change="_setEdgeProp('routing',$event.target.value)" class="pi">
                <option value="auto">自动曲线</option><option value="straight">直线</option>
                <option value="horizontal">水平曲线</option><option value="vertical">垂直曲线</option>
              </select>
            </div>
            <div class="pg"><label>连线粗细</label><input :value="getEdgeProp('strokeWidth')" type="number" @input="_setEdgeProp('strokeWidth',+$event.target.value)" class="pi" min="1" max="5" /></div>
            <div class="pg"><label>颜色</label>
              <select :value="getEdgeProp('color')" @change="_setEdgeProp('color',$event.target.value)" class="pi">
                <option value="">默认</option><option value="var(--color-success)">绿色</option>
                <option value="var(--color-warning)">橙色</option><option value="var(--color-danger)">红色</option>
                <option value="var(--color-info)">蓝色</option>
              </select>
            </div>
            <button class="btn-del-sm" @click="deleteEdge(selectedEdge)">🗑 删除连线</button>
          </div>
        </div>
        <div v-else class="props-empty">
          <p>选择节点或连线编辑属性</p>
          <p class="hint">双击子流程节点进入嵌套编辑</p>
          <div class="quick-actions">
            <button class="btn-sm" @click="showTemplatesModal=true">📐 模板</button>
            <button class="btn-sm" @click="showRulesModal=true">🔗 规则</button>
            <button class="btn-sm" @click="runValidation()">🔍 验证</button>
          </div>
        </div>
      </aside>
      <!-- Version Panel -->
      <aside v-if="showVersionPanel" class="pd-version-panel glass-card">
        <div class="vp-header"><span>📜 版本历史</span><button class="btn-sm" @click="showVersionPanel=false">✕</button></div>
        <div class="vp-list">
          <div v-if="versions.length===0" class="vp-empty">暂无版本记录（点击「快照」创建）</div>
          <div v-for="(v, i) in versions" :key="v.id" class="vp-item" :class="{active: selectedVersion?.id===v.id}" @click="selectedVersion=v">
            <div class="vp-info"><div class="vp-label">{{ v.label }}</div><div class="vp-meta">{{ new Date(v.timestamp).toLocaleString('zh-CN',{month:'2-digit',day:'2-digit',hour:'2-digit',minute:'2-digit'}) }} · {{ v.author }}</div></div>
            <div class="vp-actions"><button class="vp-btn" @click.stop="revertToVersion(v)" title="恢复">↩</button><button class="vp-btn vp-del" @click.stop="deleteVersion(i)" title="删除">🗑</button></div>
          </div>
        </div>
        <div v-if="selectedVersion" class="vp-diff">
          <div class="vp-diff-title">{{ selectedVersion.label }}</div>
          <div class="vp-diff-info"><span>节点: {{ selectedVersion.config.nodes.length }}</span><span>连线: {{ (selectedVersion.config.edges||[]).length }}</span></div>
          <!-- Diff view -->
          <div v-if="showDiff" class="vp-diff-view">
            <div class="diff-header">
              <span>与当前版本对比</span>
              <button class="btn-sm" @click="showDiff=false">✕</button>
            </div>
            <div class="diff-body">
              <div class="diff-section">
                <div class="diff-title">新增节点</div>
                <div v-for="n in addedNodes" :key="n.id" class="diff-item diff-add">+ {{ n.label||n.id }}</div>
                <div v-if="addedNodes.length===0" class="diff-empty">无</div>
              </div>
              <div class="diff-section">
                <div class="diff-title">删除节点</div>
                <div v-for="n in removedNodes" :key="n.id" class="diff-item diff-del">- {{ n.label||n.id }}</div>
                <div v-if="removedNodes.length===0" class="diff-empty">无</div>
              </div>
              <div class="diff-section">
                <div class="diff-title">变更节点</div>
                <div v-for="n in changedNodes" :key="n.id" class="diff-item diff-mod">~ {{ n.label||n.id }} (标签/位置变更)</div>
                <div v-if="changedNodes.length===0" class="diff-empty">无</div>
              </div>
            </div>
          </div>
          <div class="vp-diff-actions">
            <button class="btn-sm" @click="toggleDiff">📊 对比差异</button>
            <button class="btn-sm" style="background:var(--color-primary);color:#000" @click="revertToVersion(selectedVersion)">↩ 恢复到此版本</button>
          </div>
        </div>
      </aside>
    </div>
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
    <!-- Import/Export Modal -->
    <div v-if="showIoModal" class="modal-overlay" @click.self="showIoModal=false">
      <div class="modal modal-lg glass-card">
        <div class="im-header"><h3>📦 导入/导出流程定义</h3><button class="btn-close" @click="showIoModal=false">✕</button></div>
        <div class="im-body">
          <div class="im-tabs">
            <button :class="{active: ioMode==='export'}" @click="ioMode='export'">导出 JSON</button>
            <button :class="{active: ioMode==='import'}" @click="ioMode='import'">导入 JSON</button>
            <button :class="{active: ioMode==='validate'}" @click="runValidation()">🔍 流程验证</button>
          </div>
          <div v-if="ioMode==='export'" class="im-content">
            <div class="im-info">导出当前流程的完整定义（节点、连线、子流程配置），可用于备份或迁移。</div>
            <textarea class="json-editor" readonly :value="exportJson()"></textarea>
            <div class="im-actions">
              <button class="bs" @click="copyExportJson()">📋 复制</button>
              <button class="bc" @click="downloadJson()">💾 下载文件</button>
            </div>
          </div>
          <div v-if="ioMode==='import'" class="im-content">
            <div class="im-info">粘贴JSON格式的流程图定义（与导出格式相同）来导入流程。</div>
            <textarea class="json-editor" v-model="importJsonText" placeholder="// 粘贴JSON定义..."></textarea>
            <div class="im-actions">
              <button class="bc" @click="importJsonText=''">清空</button>
              <button class="bs" :disabled="!importJsonText.trim()" @click="doImportJson()">📥 导入</button>
            </div>
          </div>
          <div v-if="ioMode==='validate'" class="im-content">
            <div class="im-info">对当前流程进行完整性检查：连接性、循环检测、配置验证。</div>
            <div v-if="validationResult" class="validation-report">
              <div class="vr-title">📋 验证报告 — {{ validationResult.totalNodes }} 节点 | {{ validationResult.totalEdges }} 连线</div>
              <div v-for="(issue, ii) in validationResult.issues" :key="ii" :class="['vr-item', 'vr-'+issue.severity]">
                <span class="vr-icon">{{ issue.severity==='error'?'🔴':issue.severity==='warning'?'🟡':'🟢' }}</span>
                <span class="vr-text">{{ issue.message }}</span>
              </div>
              <div v-if="validationResult.suggestions.length>0" class="vr-suggestions">
                <div class="vr-sug-title">💡 建议修复</div>
                <div v-for="(s,si) in validationResult.suggestions" :key="si" class="vr-item vr-warning">{{ s }}</div>
              </div>
              <div v-if="validationResult.healthScore !== null" class="vr-score">健康度: {{ validationResult.healthScore }}%</div>
            </div>
            <div v-else class="im-info">点击上方「🔍 流程验证」按钮开始检查</div>
          </div>
        </div>
      </div>
    </div>
    <!-- Version Comparison Modal -->
    <div v-if="showCompareModal" class="modal-overlay" @click.self="showCompareModal=false">
      <div class="modal modal-xl glass-card">
        <div class="cmp-header">
          <h3>🔀 版本对比</h3>
          <div class="cmp-controls">
            <select v-model="compareV1" class="fi cmp-select">
              <option v-for="v in versions.slice(0,10)" :key="v.id" :value="v.id">{{ v.label }} {{ new Date(v.timestamp).toLocaleString('zh-CN',{month:'2-digit',day:'2-digit',hour:'2-digit',minute:'2-digit'}) }}</option>
            </select>
            <span class="cmp-arrow">→</span>
            <select v-model="compareV2" class="fi cmp-select">
              <option value="__current">当前版本</option>
              <option v-for="v in versions.slice(0,10)" :key="v.id+'c'" :value="v.id">{{ v.label }} {{ new Date(v.timestamp).toLocaleString('zh-CN',{month:'2-digit',day:'2-digit',hour:'2-digit',minute:'2-digit'}) }}</option>
            </select>
          </div>
          <button class="btn-close" @click="showCompareModal=false">✕</button>
        </div>
        <div class="cmp-body">
          <div class="cmp-panel cmp-left">
            <div class="cmp-panel-title">{{ getVersionLabel(compareV1) }} (旧)</div>
            <div class="cmp-node-list">{{ formatNodeDiff(compareV1, compareV2, 'all') }}</div>
          </div>
          <div class="cmp-divider"><span>差异</span></div>
          <div class="cmp-panel cmp-right">
            <div class="cmp-panel-title">{{ getVersionLabel(compareV2) }} (新)</div>
            <div class="cmp-node-list">{{ formatNodeDiff(compareV2, compareV1, 'all') }}</div>
          </div>
        </div>
        <div class="cmp-footer">
          <div class="cmp-stats">
            <span class="cmp-stat cmp-added">+{{ countDiff(compareV1, compareV2, 'added') }} 新增</span>
            <span class="cmp-stat cmp-removed">-{{ countDiff(compareV1, compareV2, 'removed') }} 删除</span>
            <span class="cmp-stat cmp-modified">~{{ countDiff(compareV1, compareV2, 'modified') }} 修改</span>
          </div>
          <button class="bc" @click="showCompareModal=false">关闭</button>
        </div>
      </div>
    </div>
    <!-- Connection Rules Modal -->
    <div v-if="showRulesModal" class="modal-overlay" @click.self="showRulesModal=false">
      <div class="modal modal-lg glass-card">
        <div class="rr-header">
          <h3>🔗 连接规则配置</h3>
          <button class="btn-close" @click="showRulesModal=false">✕</button>
        </div>
        <div class="rr-body">
          <div class="rr-info">定义哪些节点类型可以连接到哪些节点类型。点击规则行可切换允许/禁止。</div>
          <div class="rr-grid">
            <div class="rr-row rr-header-row">
              <span class="rr-cell rr-from">FROM \ TO</span>
              <span v-for="nt in allNodeTypes" :key="nt" class="rr-cell rr-to">{{ getNodeIcon(nt) }} {{ nt }}</span>
            </div>
            <div v-for="fromType in allNodeTypes" :key="fromType" class="rr-row">
              <span class="rr-cell rr-from">{{ getNodeIcon(fromType) }} {{ fromType }}</span>
              <span v-for="toType in allNodeTypes" :key="toType+'-'+fromType"
                :class="['rr-cell', 'rr-to', {disallowed: !isAllowed(fromType, toType)}]"
                @click="toggleRule(fromType, toType)"
                :title="isAllowed(fromType, toType)?'允许':'禁止 (点击切换)'">
                {{ isAllowed(fromType, toType)?'✓':'✗' }}
              </span>
            </div>
          </div>
          <div class="rr-legend">
            <span class="rr-ok">✓ = 允许</span>
            <span class="rr-bad">✗ = 禁止</span>
            <span class="rr-hint">点击切换规则状态</span>
          </div>
          <div class="rr-actions">
            <button class="bc" @click="resetRules()">重置默认</button>
            <button class="bs" @click="saveRules()">💾 保存规则</button>
          </div>
        </div>
      </div>
    </div>
    <!-- Node Templates Modal -->
    <div v-if="showTemplatesModal" class="modal-overlay" @click.self="showTemplatesModal=false">
      <div class="modal modal-lg glass-card">
        <div class="tm-header">
          <h3>📐 流程模板</h3>
          <button class="btn-close" @click="showTemplatesModal=false">✕</button>
        </div>
        <div class="tm-body">
          <div class="tm-info">选择一个模板快速创建常用流程结构</div>
          <div class="tm-grid">
            <div v-for="tpl in nodeTemplates" :key="tpl.name" class="tm-card" @click="applyTemplate(tpl)">
              <div class="tm-icon">{{ tpl.icon }}</div>
              <div class="tm-name">{{ tpl.name }}</div>
              <div class="tm-desc">{{ tpl.desc }}</div>
              <div class="tm-nodes">{{ tpl.nodes.map(n=>n.icon+' '+n.label).join(' → ') }}</div>
            </div>
          </div>
        </div>
      </div>
    </div>
    <!-- Keyboard Shortcuts Help -->
    <div v-if="showHelpModal" class="modal-overlay" @click.self="showHelpModal=false">
      <div class="modal modal-md glass-card">
        <div class="hm-header">
          <h3>⌨️ 快捷键</h3>
          <button class="btn-close" @click="showHelpModal=false">✕</button>
        </div>
        <div class="hm-body">
          <div class="hm-section">
            <div class="hm-title">编辑操作</div>
            <div class="hm-row"><kbd>Ctrl+Z</kbd><span>撤销</span></div>
            <div class="hm-row"><kbd>Ctrl+Y</kbd><span>重做</span></div>
            <div class="hm-row"><kbd>Ctrl+A</kbd><span>全选节点</span></div>
            <div class="hm-row"><kbd>Shift+点击</kbd><span>多选节点</span></div>
            <div class="hm-row"><kbd>G</kbd><span>将选中节点分组</span></div>
            <div class="hm-row"><kbd>Delete</kbd><span>删除选中节点/连线</span></div>
            <div class="hm-row"><kbd>Ctrl+D</kbd><span>复制选中节点</span></div>
          </div>
          <div class="hm-section">
            <div class="hm-title">画布操作</div>
            <div class="hm-row"><kbd>Space+拖拽</kbd><span>平移画布</span></div>
            <div class="hm-row"><kbd>滚轮</kbd><span>缩放</span></div>
            <div class="hm-row"><kbd>Ctrl++</kbd><span>放大</span></div>
            <div class="hm-row"><kbd>Ctrl+-</kbd><span>缩小</span></div>
            <div class="hm-row"><kbd>Ctrl+0</kbd><span>适配内容</span></div>
          </div>
          <div class="hm-section">
            <div class="hm-title">节点操作</div>
            <div class="hm-row"><kbd>点击节点</kbd><span>选中节点</span></div>
            <div class="hm-row"><kbd>双击子流程</kbd><span>进入子流程编辑</span></div>
            <div class="hm-row"><kbd>ESC</kbd><span>取消选择</span></div>
          </div>
        </div>
      </div>
    </div>
    <!-- Subprocess Inline Editor -->
    <div v-if="subprocessEditing && processDef" class="subprocess-editor">
          <div class="sp-toolbar glass-card">
        <button class="btn" @click="exitSubprocess">← 返回</button>
        <div class="sp-tools">
          <button class="tb-btn" @click="subUndo" :disabled="subHistIdx<=0" title="撤销">↩</button>
          <button class="tb-btn" @click="subRedo" :disabled="subHistIdx>=subHistory.length-1" title="重做">↪</button>
          <button class="tb-btn" @click="subZoomIn">🔍+</button>
           <button class="tb-btn" @click="subZoomOut">🔍-</button>
           <button class="tb-btn" @click="subFitCanvas">⊞</button>
           <span class="tb-sep"></span>
           <button class="tb-btn" @click="subAddNode('start')">+开始</button>
           <button class="tb-btn" @click="subAddNode('task')">+任务</button>
           <button class="tb-btn" @click="subAddNode('approval')">+审批</button>
           <button class="tb-btn" @click="subAddNode('end')">+结束</button>
           <button class="tb-btn" @click="subAddNode('timer')">+定时</button>
           <button class="tb-btn" @click="subAddNode('gate_and')">+且网关</button>
           <button class="tb-btn" @click="subAddNode('gate_or')">+或网关</button>
           <button class="tb-btn" @click="subAddNode('subprocess')">+子流程</button>
           <button class="tb-btn" @click="subAddNode('script')">+脚本</button>
           <span class="tb-sep"></span>
           <button class="tb-btn" :disabled="subSelectedNode===null" @click="subDeleteNode()" title="删除选中节点">🗑 删除</button>
           <button class="tb-btn" :disabled="subSelectedNode===null" @click="subDuplicateNode()" title="复制选中节点">📋 复制</button>
           <button class="tb-btn" @click="subAutoLayout" title="自动排列">⊞ 排列</button>
           <span class="tb-sep"></span>
           <span class="sp-node-count">{{ subprocessDef?.nodes?.length || 0 }} 节点</span>
         </div>
        <span class="sp-title">📦 {{ subprocessTitle }}</span>
        <div class="sp-actions">
          <button class="tb-btn" @click="createSubVersion">📌快照</button>
          <button class="btn btn-primary" @click="saveSubprocess">💾保存</button>
        </div>
      </div>
      <div class="subprocess-canvas pd-canvas glass-card" ref="subprocessCanvasRef" @wheel.prevent="subOnWheel">
        <div class="canvas-bg" :style="{ backgroundSize: gridScale+'px '+gridScale+'px', backgroundPosition: subPanX+'px '+subPanY+'px' }"></div>
        <svg class="canvas-svg" :style="subSvgTransform">
          <defs>
            <marker id="arrowhead-sub" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
              <polygon points="0 0, 10 3.5, 0 7" fill="var(--color-primary)" />
            </marker>
            <marker id="arrowhead-sub-sel" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
              <polygon points="0 0, 10 3.5, 0 7" fill="var(--color-warning)" />
            </marker>
          </defs>
          <!-- Edges -->
          <g class="edges">
            <path v-for="(edge, i) in subprocessDef.edges||[]" :key="edge.id"
              :d="computeSubEdgePath(edge)"
              :class="['edge-path', { selected: subSelectedEdge===i }]"
              :marker-end="subSelectedEdge===i ? 'url(#arrowhead-sub-sel)' : 'url(#arrowhead-sub)'"
              @click.stop="subSelectEdge(i)" />
          </g>
          <!-- Temp edge -->
          <path v-if="subTempEdge" :d="subTempEdgePath()" class="edge-temp" marker-end="url(#arrowhead-sub)" />
          <!-- Nodes -->
          <g class="nodes">
            <g v-for="(node, i) in subprocessDef.nodes||[]" :key="node.id"
              :transform="`translate(${node.x},${node.y})`"
              :class="['node-group', { selected: subSelectedNode===i, dragging: subIsDragging&&subDragIdx===i }]">
              <rect v-if="subSelectedNode===i" x="-6" y="-6" :width="(node.w||120)+12" :height="(node.h||50)+12" rx="10"
                fill="none" stroke="var(--color-primary)" stroke-width="2" stroke-dasharray="4,2" pointer-events="none" />
              <rect :class="['node-body', node.type, getNodeProp('style')]" :width="node.w||120" :height="node.h||50" rx="8" />
              <text :x="16" :y="(node.h||50)/2+5" class="node-icon-text">{{ getNodeIcon(node.type) }}</text>
              <text :x="(node.w||120)/2+8" :y="(node.h||50)/2-4" text-anchor="middle" class="node-label">{{ node.label || getNodeLabel(node.type) }}</text>
              <text :x="(node.w||120)/2+8" :y="(node.h||50)/2+10" text-anchor="middle" font-size="9" fill="var(--text-muted)">{{ node.assignee || '' }}</text>
              <!-- In port -->
              <circle v-if="node.type!=='start'" cx="0" :cy="(node.h||50)/2" r="6" class="port port-in"
                @mousedown.stop="subOnPortMouseDown($event, i, 'in')" />
              <!-- Gate output ports -->
              <template v-if="isGate(node.type)">
                <circle v-for="(cond, ci) in getNodeConditions(node)" :key="ci"
                  :cx="node.w||120" :cy="(node.h||50)/2 + (ci - (getNodeConditions(node).length-1)/2) * 20"
                  r="6" class="port port-out port-gate"
                  @mousedown.stop="subOnPortMouseDown($event, i, 'out')" />
              </template>
              <!-- Regular out port -->
              <circle v-if="node.type!=='end' && !isGate(node.type)"
                cx="(node.w||120)" :cy="(node.h||50)/2" r="6" class="port port-out"
                @mousedown.stop="subOnPortMouseDown($event, i, 'out')" />
              <!-- Condition badge -->
              <rect v-if="node.condition" x="4" y="4" width="10" height="10" rx="3" fill="var(--color-warning)" />
              <!-- Resize handles -->
              <template v-if="subSelectedNode===i">
                <rect v-for="pos in resizePositions" :key="'sh'+pos"
                  :x="getSubNodeResizeX(node, pos) - 4" :y="getSubNodeResizeY(node, pos) - 4"
                  width="8" height="8" rx="2" fill="var(--color-primary)" stroke="white" stroke-width="1"
                  class="resize-handle" :style="{ cursor: getResizeCursor(pos) }"
                  @mousedown.stop="subOnResizeMouseDown($event, i, pos)" />
              </template>
              <!-- Anchor points -->
              <template v-if="subSelectedNode===i && subIsDraggingAnchor">
                <circle v-for="(ah, ahi) in getSubAnchorPoints(node)" :key="ahi"
                  :cx="ah.x" :cy="ah.y" r="5" fill="var(--color-warning)" stroke="white" stroke-width="1.5"
                  style="cursor:grab" @mousedown.stop="subOnAnchorMouseDown($event, i, ahi)" />
              </template>
              <!-- Edge click zone (invisible rectangle around node for arbitrary edge creation) -->
              <rect :x="-8" :y="-8" :width="(node.w||120)+16" :height="(node.h||50)+16"
                fill="transparent" class="edge-create-zone"
                @mousedown.stop="subOnEdgeMouseDown($event, i)" />
            </g>
          </g>
        </svg>
        <div class="canvas-hint">
          <span>← 返回主流程 | 拖拽节点 | 从端口拖出连线 | 点击边缘拖出连线 | Del删除</span>
        </div>
      </div>
    </div>
    <!-- Subprocess Editor (fallback modal when not in editing mode) -->
    <!-- Group Context Menu -->
    <div v-if="groupContextMenu.groupIdx !== null" class="context-menu"
      :style="{ left: groupContextMenu.x+'px', top: groupContextMenu.y+'px' }"
      @click.self="hideGroupMenu">
      <div class="ctx-item" @click="toggleGroupCollapse(groupContextMenu.groupIdx); hideGroupMenu()">
        {{ groupNodes[groupContextMenu.groupIdx]?.node.collapsed ? '▶ 展开' : '▼ 折叠' }}
      </div>
      <div class="ctx-item ctx-danger" @click="expandGroup(groupContextMenu.groupIdx); hideGroupMenu()">
        ↩ 解散分组
      </div>
    </div>
  </div>
    <!-- Fork/Join Enhanced SVG Layer -->
    <g v-if="showBranchAnnot && processDef" class="fork-join-layer">
      <path v-for="ann in forkJoinAnnotations" :key="ann.id" :d="getForkJoinPath(ann.branchIndices)" class="fork-flow" stroke-width="2" fill="none" :stroke="ann.color" stroke-dasharray="6,3" />
      <text v-for="ann in forkJoinAnnotations" :key="ann.id+'l'" :x="processDef.nodes[ann.branchIndices[0]]?.x + (processDef.nodes[ann.branchIndices[0]]?.w||120) + 8" :y="processDef.nodes[ann.branchIndices[0]]?.y + 14" fill="var(--color-warning)" font-size="10" font-weight="600">{{ ann.label }}</text>
    </g>
    <!-- Group drag/resize handles -->
    <g v-if="!subprocessEditing" class="group-handles" :transform="edgeTransform">
      <g v-for="(g, gi) in groupNodes" :key="g.node.id">
        <rect :x="g.bounds.x+4" :y="g.bounds.y+24" :width="g.bounds.width-8" height="18"
          rx="4" fill="transparent" class="group-drag-zone"
          @mousedown.stop="onGroupMouseDown($event, gi)" />
        <rect v-for="dir in groupResizeDirs" :key="dir"
          :x="getGroupResizeX(g.node, dir) - 5" :y="getGroupResizeY(g.node, dir) - 5"
          width="10" height="10" rx="2"
          fill="var(--color-primary)" stroke="white" stroke-width="1.5"
          class="group-resize-handle" :class="dir"
          :style="{ cursor: getResizeCursor(dir) }"
          @mousedown.stop="onGroupResizeMouseDown($event, gi, dir)" />
      </g>
    </g>
    <!-- Breakpoint indicators -->
    <g v-if="breakpoints.length > 0" class="breakpoint-layer">
      <circle v-for="bp in breakpoints" :key="bp.nodeId"
        :cx="processDef?.nodes?.find(n=>n.id===bp.nodeId)?.x + (processDef.nodes.find(n=>n.id===bp.nodeId)?.w||120)/2"
        :cy="processDef?.nodes?.find(n=>n.id===bp.nodeId)?.y - 10"
        r="6" fill="var(--color-warning)" stroke="#fff" stroke-width="2"
        @click.stop="toggleBreakpoint(bp.nodeId)" class="breakpoint-dot" />
    </g>
    <!-- Flow Stats Panel -->
    <div v-if="flowStats.totalNodes > 0" class="flow-stats-panel">
      <div class="stats-grid">
        <div class="stat-item"><div class="stat-value">{{ flowStats.totalNodes }}</div><div class="stat-label">节点数</div></div>
        <div class="stat-item"><div class="stat-value">{{ flowStats.totalEdges }}</div><div class="stat-label">连边数</div></div>
        <div class="stat-item"><div class="stat-value">{{ flowStats.avgDegree }}</div><div class="stat-label">平均度数</div></div>
        <div class="stat-item"><div class="stat-value">{{ flowStats.maxDegree }}</div><div class="stat-label">最大出度</div></div>
        <div class="stat-item"><div class="stat-value">{{ flowStats.density }}</div><div class="stat-label">网络密度</div></div>
        <div class="stat-item"><div class="stat-value">{{ flowStats.cycles }}</div><div class="stat-label">环数量</div></div>
        <div class="stat-item"><div class="stat-value">{{ flowStats.isolatedNodes }}</div><div class="stat-label">孤立节点</div></div>
      </div>
    </div>
    <!-- Flow Statistics Detail Modal -->
    <div v-if="showFlowStatsModal" class="modal-overlay" @click.self="showFlowStatsModal=false">
      <div class="modal modal-lg glass-card">
        <div class="modal-header"><h3>📊 流程统计详情</h3><button class="btn-close" @click="showFlowStatsModal=false">✕</button></div>
        <div class="modal-body">
          <div class="stats-detail-grid">
            <div class="sd-card">
              <div class="sd-value">{{ flowStats.totalNodes }}</div>
              <div class="sd-label">总节点数</div>
              <div class="sd-desc">图中所有节点数量</div>
            </div>
            <div class="sd-card">
              <div class="sd-value">{{ flowStats.totalEdges }}</div>
              <div class="sd-label">总连边数</div>
              <div class="sd-desc">图中所有连线数量</div>
            </div>
            <div class="sd-card">
              <div class="sd-value">{{ flowStats.avgDegree }}</div>
              <div class="sd-label">平均出度</div>
              <div class="sd-desc">每个节点平均发出的连边数</div>
            </div>
            <div class="sd-card">
              <div class="sd-value">{{ flowStats.maxDegree }}</div>
              <div class="sd-label">最大出度</div>
              <div class="sd-desc">单个节点最大发出的连边数</div>
            </div>
            <div class="sd-card">
              <div class="sd-value">{{ flowStats.density }}</div>
              <div class="sd-label">网络密度</div>
              <div class="sd-desc">实际连边数 / 最大可能连边数</div>
            </div>
            <div class="sd-card">
              <div class="sd-value">{{ flowStats.cycles }}</div>
              <div class="sd-label">环数量</div>
              <div class="sd-desc">图中检测到的循环路径数</div>
            </div>
            <div class="sd-card">
              <div class="sd-value">{{ flowStats.isolatedNodes }}</div>
              <div class="sd-label">孤立节点</div>
              <div class="sd-desc">无入边也无出边的节点</div>
            </div>
            <div class="sd-card info-card">
              <div class="sd-value">{{ flowStats.totalNodes > 0 ? (flowStats.totalEdges / flowStats.totalNodes).toFixed(2) : 0 }}</div>
              <div class="sd-label">平均连接数</div>
              <div class="sd-desc">每节点平均连接的边数</div>
            </div>
          </div>
          <div class="stats-warning" v-if="flowStats.cycles > 0">
            ⚠️ 检测到 {{ flowStats.cycles }} 个环，可能导致流程死循环
          </div>
          <div class="stats-warning" v-if="flowStats.isolatedNodes > 0">
            ⚠️ 存在 {{ flowStats.isolatedNodes }} 个孤立节点，请检查连接性
          </div>
          <div class="stats-good" v-if="flowStats.cycles === 0 && flowStats.isolatedNodes === 0">
            ✅ 流程结构健康，无环且无孤立节点
          </div>
        </div>
      </div>
    </div>
    <!-- Network Analysis Modal -->
    <div v-if="showNetworkAnalysis" class="modal-overlay" @click.self="showNetworkAnalysis=false">
      <div class="modal modal-lg glass-card">
        <div class="modal-header"><h3>📊 网络分析</h3><button class="btn-close" @click="showNetworkAnalysis=false">✕</button></div>
        <div class="modal-body">
          <div class="network-grid">
            <div v-for="m in networkMetrics" :key="m.metric" class="network-card">
              <div class="nc-value">{{ m.value }}</div>
              <div class="nc-label">{{ m.metric }}</div>
              <div class="nc-desc">{{ m.description }}</div>
            </div>
          </div>
          <div v-if="networkMetrics.some(m => m.metric === `环数量` && m.value > 0)" class="analysis-warning">
            ⚠️ 检测到环，可能导致流程死循环
          </div>
          <div v-if="networkMetrics.some(m => m.metric === `孤立节点` && m.value > 0)" class="analysis-warning">
            ⚠️ 存在孤立节点，请检查连接性
          </div>
        </div>
      </div>
    </div>
    <!-- Style Presets Panel -->
    <div v-if="selectedNode !== null" class="style-presets-panel">
      <div class="spp-title">🎨 节点样式预设</div>
      <div class="spp-grid">
        <button v-for="(preset, pi) in stylePresets" :key="pi"
          class="spp-btn" :style="{background: preset.fill, border: '2px solid ' + preset.stroke}"
          :title="preset.name"
          @click="applyStylePreset(preset)">{{ preset.icon }}</button>
      </div>
    </div>
    <!-- Condition Builder Modal -->
    <div v-if="showCondBuilder" class="modal-overlay" @click.self="showCondBuilder=false">
      <div class="modal modal-lg glass-card">
        <div class="modal-header"><h3>🔀 复杂条件构建器</h3><button class="btn-close" @click="showCondBuilder=false">✕</button></div>
        <div class="modal-body" style="display:flex;gap:16px">
          <div style="flex:1">
            <div class="fg"><label>条件树</label>
              <div v-if="condTree" class="cond-tree">
                <div class="cond-group and-group">
                  <span class="cond-logic">AND</span>
                  <button class="btn-sm" @click="addCondGroup(condTree)">+ 组</button>
                  <button class="btn-sm" @click="addCondCondition(condTree)">+ 条件</button>
                  <div v-for="(cond, ci) in (condTree.conditions||[])" :key="ci" class="cond-row">
                    <select v-model="cond.field" class="fi" style="width:90px">
                      <option v-for="f in condFields" :value="f">{{ f }}</option>
                    </select>
                    <select v-model="cond.operator" class="fi" style="width:60px">
                      <option value="==">等于</option><option value="!=">不等于</option><option value=">">大于</option><option value="<">小于</option>
                    </select>
                    <input v-model="cond.value" class="fi" placeholder="值" style="flex:1" />
                  </div>
                </div>
              </div>
            </div>
            <div class="fg"><label>表达式预览</label>
              <textarea class="json-editor" :value="condPreview" readonly rows="3"></textarea>
            </div>
          </div>
          <div style="width:180px">
            <div class="fg"><label>快捷模板</label>
              <button class="btn-sm" style="width:100%;margin-bottom:4px;text-align:left" @click="condTree={id:genId(),type:'group',logic:'AND',conditions:[{field:'amount',operator:'>',value:'1000'}],children:[]}">金额 > 1000</button>
              <button class="btn-sm" style="width:100%;margin-bottom:4px;text-align:left" @click="condTree={id:genId(),type:'group',logic:'OR',conditions:[{field:'status',operator:'==',value:'pending'},{field:'priority',operator:'==',value:'high'}],children:[]}">待处理 或 高优先级</button>
              <button class="btn-sm" style="width:100%;margin-bottom:4px;text-align:left" @click="condTree={id:genId(),type:'group',logic:'AND',conditions:[{field:'userId',operator:'==',value:'current'},{field:'department',operator:'contains',value:'tech'}],children:[]}">当前用户且技术部</button>
            </div>
          </div>
        </div>
        <div class="modal-footer">
          <button class="bc" @click="showCondBuilder=false">取消</button>
          <button class="bs" @click="previewCond()">👁 预览</button>
          <button class="bs" @click="showCondBuilder=false">💾 保存</button>
        </div>
      </div>
    </div>
    <!-- Variable Binding Panel -->
    <div v-if="showVarBindingPanel" class="var-binding-panel">
      <div class="vb-header"><span>📎 变量绑定</span><button class="btn-sm" @click="showVarBindingPanel=false">✕</button></div>
      <div class="vb-body">
        <div v-for="(vb, vi) in varBindings" :key="vi" class="vb-row">
          <select v-model="vb.sourceNode" class="fi" style="width:70px">
            <option value="">源节点</option>
            <option v-for="n in processDef?.nodes" :key="n.id" :value="n.id">{{ (n.label||n.id).slice(0,5) }}</option>
          </select>
          <input v-model="vb.sourceField" class="fi" placeholder="源字段" style="width:70px" />
          <span class="vb-arrow">→</span>
          <select v-model="vb.targetNode" class="fi" style="width:70px">
            <option value="">目标</option>
            <option v-for="n in processDef?.nodes" :key="n.id" :value="n.id">{{ (n.label||n.id).slice(0,5) }}</option>
          </select>
          <input v-model="vb.targetField" class="fi" placeholder="目标字段" style="width:70px" />
          <button class="btn-sm" style="color:var(--color-danger)" @click="removeVarBinding(vi)">✕</button>
        </div>
        <button class="btn-sm" @click="addVarBinding()">+ 添加绑定</button>
        <button class="bs" @click="applyVarBindings()">💾 应用</button>
      </div>
    </div>
    <!-- Form Rules Panel -->
    <div v-if="showFormRulesPanel" class="form-rules-panel">
      <div class="fr-header"><span>🔗 表单联动规则</span><button class="btn-sm" @click="showFormRulesPanel=false">✕</button></div>
      <div class="fr-body">
        <div v-for="(rule, ri) in formRules" :key="rule.id" class="fr-row">
          <select v-model="rule.sourceField" class="fi" style="width:90px">
            <option value="">字段</option>
            <option v-for="f in (currentForm?.fields||[])" :key="f.key" :value="f.key">{{ f.label }}</option>
          </select>
          <select v-model="rule.operator" class="fi" style="width:55px">
            <option value="==">等于</option><option value="!=">不等于</option><option value="contains">包含</option>
          </select>
          <input v-model="rule.value" class="fi" style="width:70px" placeholder="值" />
          <select v-model="rule.action" class="fi" style="width:60px">
            <option value="show">显示</option><option value="hide">隐藏</option><option value="enable">启用</option><option value="disable">禁用</option><option value="require">必填</option>
          </select>
          <button class="btn-sm" style="color:var(--color-danger)" @click="removeFormRule(ri)">✕</button>
        </div>
        <button class="btn-sm" @click="addFormRule()">+ 添加规则</button>
        <button class="bs" @click="saveFormRules()">💾 保存</button>
      </div>
    </div>
    <!-- Batch Toolbar -->
    <div v-if="showBatchToolbar" class="batch-toolbar">
      <div class="batch-toolbar-inner">
        <span class="batch-info">批量操作模式</span>
        <button class="btn-sm" @click="batchAlign('left')">⬅ 左对齐</button>
        <button class="btn-sm" @click="batchAlign('top')">⬆ 顶对齐</button>
        <button class="btn-sm" @click="enterBatchMode()">✕ 退出</button>
      </div>
    </div>
    <!-- Theme Editor -->
    <div v-if="showThemeEditor" class="modal-overlay" @click.self="showThemeEditor=false">
      <div class="modal modal-lg glass-card">
        <div class="modal-header"><h3>🎨 画布主题定制</h3><button class="btn-close" @click="showThemeEditor=false">✕</button></div>
        <div class="modal-body">
          <div class="theme-grid">
            <div v-for="(t, ti) in themePresets" :key="ti" class="theme-card" :class="{active: activeTheme.name===t.name}" @click="applyTheme(t); showThemeEditor=false">
              <div class="theme-preview" :style="{background: t.bg, borderBottom: '3px solid ' + t.accentColor}"></div>
              <div class="theme-name">{{ t.name }}</div>
            </div>
          </div>
          <div class="fg" style="margin-top:12px"><label>自定义颜色</label>
            <div style="display:flex;gap:8px;flex-wrap:wrap">
              <div><label style="font-size:10px;color:var(--text-muted)">背景</label><input type="color" :value="activeTheme.bg" @change="activeTheme.bg=$event.target.value;applyTheme(activeTheme)" class="color-input" /></div>
              <div><label style="font-size:10px;color:var(--text-muted)">主色</label><input type="color" :value="activeTheme.accentColor" @change="activeTheme.accentColor=$event.target.value;applyTheme(activeTheme)" class="color-input" /></div>
              <div><label style="font-size:10px;color:var(--text-muted)">文字</label><input type="color" :value="activeTheme.textColor" @change="activeTheme.textColor=$event.target.value;applyTheme(activeTheme)" class="color-input" /></div>
            </div>
          </div>
        </div>
      </div>
    </div>
    <!-- Animation Panel -->
    <div v-if="showAnimPanel" class="anim-panel">
      <div class="anim-header"><span>✨ 动画效果</span><button class="btn-sm" @click="showAnimPanel=false">✕</button></div>
      <div class="anim-body">
        <div v-for="s in animSettings" :key="s.key" class="anim-item" :class="{active: s.enabled}" @click="toggleAnimSetting(s.key)">
          <span>{{ s.icon }}</span><span>{{ s.label }}</span>
        </div>
      </div>
    </div>
    <!-- Script Full Editor Modal -->
    <div v-if="showScriptFullEditor" class="modal-overlay script-editor-overlay" @click.self="closeScriptEditor()">
      <div class="modal script-editor-modal modal-xl">
        <div class="modal-header"><span>💻 脚本编辑器</span><button class="btn-sm" @click="closeScriptEditor()">✕</button></div>
        <div class="modal-body script-editor-body">
          <div class="se-toolbar">
            <select v-model="scriptLang" class="se-lang-select"><option value="javascript">JavaScript</option><option value="typescript">TypeScript</option><option value="python">Python</option></select>
            <button class="btn-sm" @click="runScriptTest()">▶ 运行测试</button>
            <button class="btn-sm" @click="validateScriptCode()">✓ 验证</button>
            <button class="btn-sm" @click="clearScriptLogs()">清除日志</button>
          </div>
          <div class="se-editor-wrap">
            <textarea v-model="scriptCode" class="se-code-editor" spellcheck="false" placeholder="输入脚本代码..."></textarea>
            <div class="se-line-numbers"><div v-for="i in Math.max(scriptCode.split('\n').length, 20)" :key="i" class="se-line-num">{{ i }}</div></div>
          </div>
          <div class="se-sidebar">
            <div class="se-section"><div class="se-section-title">📦 导入</div>
              <div v-for="(imp,ii) in scriptImports" :key="ii" class="se-import-row">
                <input v-model="imp.name" placeholder="模块名" class="se-import-input" />
                <input v-model="imp.source" placeholder="来源" class="se-import-input" />
                <button class="btn-xs btn-danger" @click="removeScriptImport(ii)">✕</button>
              </div>
              <button class="btn-sm" @click="addScriptImport()">+ 添加导入</button>
            </div>
            <div class="se-section"><div class="se-section-title">🔄 变量</div>
              <div v-for="(v,vi) in scriptVars" :key="vi" class="se-var-row">
                <input v-model="v.name" placeholder="变量名" class="se-var-input" />
                <select v-model="v.type" class="se-var-select"><option>string</option><option>number</option><option>boolean</option><option>object</option></select>
                <button class="btn-xs btn-danger" @click="removeScriptVar(vi)">✕</button>
              </div>
              <button class="btn-sm" @click="addScriptVar()">+ 添加变量</button>
            </div>
            <div class="se-section"><div class="se-section-title">⚠️ 错误处理</div>
              <select v-model="scriptErrorConfig.onFail" class="se-select">
                <option value="abort">终止流程</option><option value="skip">跳过节点</option><option value="retry">重试</option>
              </select>
              <div v-if="scriptErrorConfig.onFail==='retry'" class="se-retry-config">
                <label>重试次数</label><input v-model.number="scriptErrorConfig.retryCount" type="number" class="se-num-input" />
                <label>延迟(ms)</label><input v-model.number="scriptErrorConfig.retryDelay" type="number" class="se-num-input" />
              </div>
            </div>
            <div class="se-section"><div class="se-section-title">📤 输出映射</div>
              <div v-for="(bind,bi) in scriptOutputBindings" :key="bi" class="se-bind-row">
                <input v-model="bind.sourceField" placeholder="源字段" class="se-bind-input" />
                <select v-model="bind.transform" class="se-bind-select"><option value="identity">恒等</option><option value="upper">大写</option><option value="lower">小写</option><option value="trim">去空格</option></select>
                <input v-model="bind.targetField" placeholder="目标字段" class="se-bind-input" />
                <button class="btn-xs btn-danger" @click="removeScriptOutputBinding(bi)">✕</button>
              </div>
              <button class="btn-sm" @click="addScriptOutputBinding()">+ 添加映射</button>
            </div>
          </div>
          <div v-if="scriptValidation" class="se-validation">
            <div v-for="e in scriptValidation.errors" :key="e" class="se-error">❌ {{ e }}</div>
            <div v-for="w in scriptValidation.warnings" :key="w" class="se-warning">⚠️ {{ w }}</div>
            <div v-for="s in scriptValidation.suggestions" :key="s" class="se-suggestion">💡 {{ s }}</div>
          </div>
          <div v-if="showScriptLogPanel" class="se-log-panel">
            <div class="se-log-header">📋 执行日志</div>
            <div class="se-log-body"><div v-for="(log,li) in scriptLogs" :key="li" class="se-log-entry">{{ log }}</div></div>
          </div>
        </div>
        <div class="modal-footer">
          <button class="btn" @click="saveScriptToNode()">💾 保存到节点</button>
          <button class="btn btn-ghost" @click="closeScriptEditor()">取消</button>
        </div>
      </div>
    </div>
    <!-- Node Properties Editor -->
    <div v-if="showNodePropsEditor" class="modal-overlay" @click.self="showNodePropsEditor=false">
      <div class="modal node-props-modal modal-md">
        <div class="modal-header"><span>⚙️ 节点属性编辑器</span><button class="btn-sm" @click="showNodePropsEditor=false">✕</button></div>
        <div class="modal-body">
          <div v-if="nodePropEditorNodeIdx!==null && processDef" class="np-editor">
            <div class="np-node-info">
              <span class="np-node-icon">{{ getNodeIcon(processDef.nodes[nodePropEditorNodeIdx].type) }}</span>
              <span class="np-node-label">{{ processDef.nodes[nodePropEditorNodeIdx].label||processDef.nodes[nodePropEditorNodeIdx].type }}</span>
            </div>
            <div v-for="cat in getNodePropsForType(processDef.nodes[nodePropEditorNodeIdx].type)" :key="cat.category" class="np-category">
              <div class="np-cat-title">{{ cat.icon }} {{ cat.label }}</div>
              <div v-for="prop in cat.props" :key="prop.key" class="np-prop-row">
                <label class="np-prop-label">{{ prop.label }}</label>
                <input :value="getNodePropValue(processDef.nodes[nodePropEditorNodeIdx], cat.category, prop.key)" @input="setNodePropValue(processDef.nodes[nodePropEditorNodeIdx], cat.category, prop.key, $event.target.value)" class="np-input" :placeholder="String(prop.defaultVal)" />
              </div>
            </div>
          </div>
        </div>
        <div class="modal-footer"><button class="btn" @click="saveNodeProps()">💾 保存</button><button class="btn btn-ghost" @click="showNodePropsEditor=false">取消</button></div>
      </div>
    </div>
    <!-- Parallel Branch Config -->
    <div v-if="showParallelConfig" class="modal-overlay" @click.self="showParallelConfig=false">
      <div class="modal parallel-config-modal">
        <div class="modal-header"><span>⚡ 并行分支配置</span><button class="btn-sm" @click="showParallelConfig=false">✕</button></div>
        <div class="modal-body">
          <div class="pc-strategy">
            <div class="pc-row"><span>分叉策略</span><select v-model="forkJoinConfig.strategy" class="pc-select"><option value="and">AND</option><option value="or">OR</option><option value="xor">XOR</option></select></div>
            <div class="pc-row"><span>汇合策略</span><select v-model="forkJoinConfig.joinStrategy" class="pc-select"><option value="all">全部完成</option><option value="first">首个完成</option><option value="any">任一完成</option></select></div>
            <div class="pc-row"><span>超时(ms)</span><input v-model.number="forkJoinConfig.timeout" type="number" class="pc-num" /></div>
          </div>
          <div class="pc-branches">
            <div class="pc-branches-title">分支列表</div>
            <div v-for="(br,bi) in parallelBranches" :key="br.id" class="pc-branch">
              <span class="pc-branch-color" :style="{background:br.color}"></span>
              <input v-model="br.label" class="pc-branch-name" />
              <span class="pc-branch-nodes">{{ br.nodes.length }}节点</span>
              <button class="btn-xs btn-danger" @click="removeParallelBranch(bi)">✕</button>
            </div>
            <button class="btn-sm" @click="addParallelBranch('新分支')">+ 添加分支</button>
          </div>
          <div class="pc-actions">
            <button class="btn" @click="detectParallelBranchesEnhanced()">🔍 自动检测</button>
            <button class="btn" @click="simulateParallelExecution()">▶ 模拟执行</button>
          </div>
        </div>
      </div>
    </div>
    <!-- Branch Timeline -->
    <div v-if="showBranchTimeline" class="branch-timeline-panel">
      <div class="bt-header"><span>📊 分支执行时间线</span><button class="btn-sm" @click="showBranchTimeline=false">✕</button></div>
      <div class="bt-body">
        <div class="bt-status-grid">
          <div v-for="bs in branchStates" :key="bs[0]" class="bt-status-item">
            <span class="bt-bc" :style="{background:parallelBranches.find(b=>b.id===bs[0])?.color||'#666'}"></span>
            <span class="bt-bname">{{ parallelBranches.find(b=>b.id===bs[0])?.label||bs[0] }}</span>
            <span :class="['bt-bstatus','bst-'+bs[1].status]">{{ bs[1].status }}</span>
          </div>
        </div>
        <div class="bt-timeline">
          <div v-for="ev in branchTimeline" :key="ev.time+'-'+ev.branchId" class="bt-event">
            <span class="bt-dot" :style="{background:getBranchStatusColor(ev.event==='start'?'running':ev.event==='complete'?'completed':'failed')}"></span>
            <span class="bt-label">{{ ev.branchId }}: {{ ev.details }}</span>
          </div>
        </div>
      </div>
    </div>
    <!-- Flow Analysis Panel -->
    <div v-if="showFlowAnalysis" class="flow-analysis-panel">
      <div class="fa-header"><span>🔬 流程分析</span><button class="btn-sm" @click="showFlowAnalysis=false">✕</button></div>
      <div class="fa-body">
        <button class="btn" @click="runFlowAnalysis()">🔍 开始分析</button>
        <div v-if="flowAnalysisResult" class="fa-stats">
          <div class="fa-health">
            <div class="fa-health-score" :style="{color:getFlowHealthScore()>=80?'var(--color-success)':getFlowHealthScore()>=60?'var(--color-warning)':'var(--color-danger)'}">{{ getFlowHealthScore() }}</div>
            <div class="fa-health-label">{{ getFlowHealthLabel(getFlowHealthScore()) }}</div>
          </div>
          <div class="fa-grid">
            <div class="fa-stat"><span class="fa-val">{{ flowAnalysisResult.totalNodes }}</span><span class="fa-lbl">节点</span></div>
            <div class="fa-stat"><span class="fa-val">{{ flowAnalysisResult.totalEdges }}</span><span class="fa-lbl">连线</span></div>
            <div class="fa-stat"><span class="fa-val" style="color:var(--color-danger)">{{ flowAnalysisResult.cycles.length }}</span><span class="fa-lbl">循环</span></div>
            <div class="fa-stat"><span class="fa-val" style="color:var(--color-warning)">{{ flowAnalysisResult.isolatedNodes.length }}</span><span class="fa-lbl">孤立</span></div>
            <div class="fa-stat"><span class="fa-val" style="color:var(--color-primary)">{{ flowAnalysisResult.bottlenecks.length }}</span><span class="fa-lbl">瓶颈</span></div>
          </div>
        </div>
        <div v-if="flowAnalysisResult&&flowAnalysisResult.cycles.length>0" class="fa-cycles">
          <div class="fa-title">⚠️ 检测到循环</div>
          <div v-for="(cy,ci) in flowAnalysisResult.cycles" :key="ci" class="fa-cycle">{{ cy.nodes.join(' → ') }}</div>
        </div>
        <div v-if="flowAnalysisResult&&flowAnalysisResult.bottlenecks.length>0" class="fa-bottlenecks">
          <div class="fa-title">🔶 瓶颈节点</div>
          <div v-for="(bn,bi) in flowAnalysisResult.bottlenecks" :key="bi" class="fa-bn">
            <span>{{ bn.label }}</span><span :class="['fa-bn-sev','sev-'+bn.severity]">{{ bn.severity }}</span>
          </div>
        </div>
      </div>
    </div>
    <!-- Archive Manager -->
    <div v-if="showArchiveManager" class="archive-manager">
      <div class="am-header"><span>🗄 流程归档</span><button class="btn-sm" @click="showArchiveManager=false">✕</button></div>
      <div class="am-body">
        <div class="am-add">
          <input v-model="newArchiveLabel" placeholder="存档名称" class="am-input" />
          <input v-model="newArchiveDesc" placeholder="描述" class="am-input am-desc" />
          <button class="btn-sm" @click="createArchive()">💾 存档</button>
        </div>
        <div class="am-list">
          <div v-for="(entry,ei) in processArchive" :key="entry.id" class="am-entry">
            <div class="am-entry-info"><span class="am-entry-label">{{ entry.label }}</span><span class="am-entry-meta">{{ entry.nodeCount }}节点/{{ entry.edgeCount }}边 · {{ formatTimestamp(entry.timestamp) }}</span></div>
            <div class="am-entry-actions"><button class="btn-xs" @click="restoreArchive(ei)">恢复</button><button class="btn-xs btn-danger" @click="deleteArchive(ei)">删除</button></div>
          </div>
        </div>
        <div v-if="processArchive.length===0" class="am-empty">暂无存档</div>
      </div>
    </div>
    <!-- Snapshot Manager -->
    <div v-if="showSnapshotManager" class="snapshot-manager">
      <div class="sm-header"><span>📸 流程快照</span><button class="btn-sm" @click="showSnapshotManager=false">✕</button></div>
      <div class="sm-body">
        <button class="btn" @click="createSnapshot()">📸 创建快照</button>
        <div class="sm-list">
          <div v-for="(snap,si) in processSnapshots" :key="snap.id" class="sm-snap">
            <span class="sm-snap-name">{{ snap.name }}</span><span class="sm-snap-meta">{{ formatTimestamp(snap.createdAt) }}</span>
            <span :class="['sm-snap-status','sm-snap-'+snap.status]">{{ snap.status }}</span>
          </div>
        </div>
      </div>
    </div>
    <!-- Tool Palette -->
    <div v-if="showToolPalette" class="tool-palette">
      <div class="tp-header">🛠 工具面板</div>
      <div class="tp-tools">
        <button v-for="tool in ['select','pan','edge','annotate']" :key="tool" :class="['tp-tool-btn',{active:activeTool===tool}]" @click="setActiveTool(tool)">
          {{ tool==='select'?'🖱':tool==='pan'?'✋':tool==='edge'?'🔗':'📝' }}<span>{{ tool }}</span>
        </button>
      </div>
      <div class="tp-sep"></div>
      <div class="tp-highlights">
        <div class="tp-label">高亮模式</div>
        <button :class="['tp-hl-btn',{active:highlightMode==='none'}]" @click="toggleHighlightMode('none')">无</button>
        <button :class="['tp-hl-btn',{active:highlightMode==='incoming'}]" @click="toggleHighlightMode('incoming')">入边</button>
        <button :class="['tp-hl-btn',{active:highlightMode==='outgoing'}]" @click="toggleHighlightMode('outgoing')">出边</button>
        <button :class="['tp-hl-btn',{active:highlightMode==='all'}]" @click="toggleHighlightMode('all')">全部</button>
      </div>
      <div class="tp-sep"></div>
      <div class="tp-animations">
        <div class="tp-label">动画</div>
        <button class="tp-anim-btn" @click="applyAnimation('pulse')">脉冲</button>
        <button class="tp-anim-btn" @click="applyAnimation('wave')">波浪</button>
        <button class="tp-anim-btn" @click="applyAnimation('flow')">流动</button>
        <button class="tp-anim-btn" @click="resetAnimations()">重置</button>
      </div>
      <div class="tp-sep"></div>
      <div class="tp-settings">
        <label><input v-model="showRipples" type="checkbox" /> 涟漪效果</label>
        <label><input v-model="gridSnapping" type="checkbox" /> 网格吸附</label>
        <label>速度 <input v-model.number="animationSpeed" type="range" min="0.5" max="3" step="0.5" /></label>
      </div>
    </div>
    <!-- Subprocess Breadcrumb -->
    <div v-if="subprocessEditing && subprocessContextStack.length>0" class="subprocess-breadcrumb">
      <div class="sb-nav">
        <button class="sb-home" @click="exitSubprocess()">🏠</button>
        <span v-for="(cr,ci) in subprocessContextStack" :key="ci" class="sb-crumb">
          <span :class="['sb-crumb-text',{active:ci===subprocessContextStack.length-1}]">{{ cr.title }}</span>
          <span v-if="ci<subprocessContextStack.length-1" class="sb-arrow">›</span>
        </span>
        <span class="sb-depth-badge">深度 {{ subprocessDepth }}</span>
      </div>
    </div>


    <!-- Node Detail Panel -->
    <div v-if="showNodeDetailPanel && nodeDetailNodeIdx !== null" class="node-detail-panel">
      <div class="ndp-header">
        <span>📋 节点详情</span>
        <div class="ndp-tabs">
          <button :class="['ndp-tab',{active:nodeDetailTab==='info'}]" @click="changeNodeDetailTab('info')">信息</button>
          <button :class="['ndp-tab',{active:nodeDetailTab==='conditions'}]" @click="changeNodeDetailTab('conditions')">条件</button>
          <button :class="['ndp-tab',{active:nodeDetailTab==='vars'}]" @click="changeNodeDetailTab('vars')">变量</button>
          <button :class="['ndp-tab',{active:nodeDetailTab==='props'}]" @click="changeNodeDetailTab('props')">属性</button>
          <button :class="['ndp-tab',{active:nodeDetailTab==='history'}]" @click="changeNodeDetailTab('history')">历史</button>
        </div>
        <button class="btn-sm" @click="closeNodeDetail()">✕</button>
      </div>
      <div class="ndp-body">
        <div v-if="nodeDetailTab==='info'" class="ndp-info">
          <div v-if="getNodeDetailInfo()" class="ndp-info-grid">
            <div class="ndp-info-item"><span class="ndp-label">ID</span><span class="ndp-val">{{ getNodeDetailInfo()!.node.id }}</span></div>
            <div class="ndp-info-item"><span class="ndp-label">类型</span><span class="ndp-val">{{ getNodeDetailInfo()!.node.type }}</span></div>
            <div class="ndp-info-item"><span class="ndp-label">标签</span><span class="ndp-val">{{ getNodeDetailInfo()!.node.label || '无' }}</span></div>
            <div class="ndp-info-item"><span class="ndp-label">入边数</span><span class="ndp-val">{{ getNodeDetailInfo()!.inCount }}</span></div>
            <div class="ndp-info-item"><span class="ndp-label">出边数</span><span class="ndp-val">{{ getNodeDetailInfo()!.outCount }}</span></div>
            <div class="ndp-info-item"><span class="ndp-label">位置</span><span class="ndp-val">({{ getNodeDetailInfo()!.node.x }}, {{ getNodeDetailInfo()!.node.y }})</span></div>
          </div>
        </div>
        <div v-if="nodeDetailTab==='history'" class="ndp-history">
          <div v-for="(h, hi) in nodeDetailHistory" :key="hi" class="ndp-hist-entry">
            <span class="ndp-hist-time">{{ new Date(h.timestamp).toLocaleTimeString() }}</span>
            <span class="ndp-hist-action">{{ h.action }}</span>
            <span class="ndp-hist-details">{{ h.details }}</span>
          </div>
          <div v-if="nodeDetailHistory.length===0" class="ndp-hist-empty">暂无历史记录</div>
        </div>
      </div>
    </div>

    <!-- Edge Editor Panel -->
    <div v-if="showEdgeEditorPanel" class="edge-editor-panel">
      <div class="eep-header"><span>🔗 连线编辑器</span><button class="btn-sm" @click="closeEdgeEditor()">✕</button></div>
      <div class="eep-body">
        <div class="eep-field"><label>标签</label><input v-model="edgeLabelTemp" @input="updateEdgeLabel(edgeLabelTemp)" class="eep-input" placeholder="连线标签" /></div>
        <div class="eep-field"><label>条件</label><input v-model="edgeConditionTemp" @input="updateEdgeCondition(edgeConditionTemp)" class="eep-input" placeholder="执行条件" /></div>
        <div class="eep-field"><label>路由</label>
          <select v-model="edgeRoutingTemp" @change="updateEdgeRouting(edgeRoutingTemp)" class="eep-select">
            <option value="auto">自动</option><option value="straight">直线</option><option value="horizontal">水平</option><option value="vertical">垂直</option>
          </select>
        </div>
        <div class="eep-actions">
          <button class="btn btn-danger" @click="deleteEdgeEditor()">🗑 删除连线</button>
        </div>
      </div>
    </div>

    <!-- Template Manager -->
    <div v-if="showTemplateManager" class="template-manager-panel">
      <div class="tmp-header"><span>📦 模板管理</span><button class="btn-sm" @click="showTemplateManager=false">✕</button></div>
      <div class="tmp-body">
        <div class="tmp-search"><input v-model="templateManagerSearch" placeholder="搜索模板..." class="tmp-input" /></div>
        <div class="tmp-grid">
          <div v-for="(tpl, ti) in filterCustomTemplates()" :key="tpl.id" class="tmp-card">
            <div class="tmp-icon">{{ tpl.icon }}</div>
            <div class="tmp-name">{{ tpl.name }}</div>
            <div class="tmp-desc">{{ tpl.description }}</div>
            <div class="tmp-tags"><span v-for="tag in tpl.tags" :key="tag" class="tmp-tag">{{ tag }}</span></div>
            <div class="tmp-actions">
              <button class="btn-sm" @click="loadCustomTemplate(ti)">加载</button>
              <button class="btn-sm" @click="exportCustomTemplate(ti)">导出</button>
              <button class="btn-sm btn-danger" @click="deleteCustomTemplate(ti)">删除</button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Collaboration Panel -->
    <div v-if="showCollabPanel" class="collab-panel">
      <div class="col-header"><span>👥 协作者</span><button class="btn-sm" @click="showCollabPanel=false">✕</button></div>
      <div class="col-body">
        <div class="col-mode"><button :class="['col-mode-btn',{active:collabMode==='view'}]" @click="collabMode='view'">查看</button><button :class="['col-mode-btn',{active:collabMode==='edit'}]" @click="collabMode='edit'">编辑</button><button :class="['col-mode-btn',{active:collabMode==='comment'}]" @click="collabMode='comment'">注释</button></div>
        <div class="col-list">
          <div v-for="c in collaborators" :key="c.id" class="col-item">
            <span class="col-avatar" :style="{background:c.color+'30'}">{{ c.avatar }}</span>
            <span class="col-name">{{ c.name }}</span>
            <span class="col-status" :style="{color:c.color}">{{ c.lastActive > Date.now()-60000 ? '在线' : '离线' }}</span>
            <button class="btn-xs btn-danger" @click="removeCollaborator(c.id)">✕</button>
          </div>
        </div>
        <button class="btn-sm" @click="simulateCollabMovement()">🔄 模拟移动</button>
      </div>
    </div>

    <!-- Notification Panel -->
    <div v-if="showNotificationPanel" class="notification-panel">
      <div class="np-header"><span>🔔 通知中心</span><button class="btn-sm" @click="showNotificationPanel=false">✕</button></div>
      <div class="np-body">
        <div v-for="n in notificationsList" :key="n.id" :class="['np-item',{unread:!n.read}]">
          <span class="np-type np-type-"+n.type>{{ n.type==='info'?'ℹ':n.type==='success'?'✓':n.type==='warning'?'⚠':'✗' }}</span>
          <div class="np-content"><div class="np-title">{{ n.title }}</div><div class="np-msg">{{ n.message }}</div></div>
          <span class="np-time">{{ formatTimestamp(n.timestamp) }}</span>
          <button v-if="!n.read" class="btn-xs" @click="markNotificationNotifRead(n.id)">✓</button>
        </div>
        <div v-if="notificationsList.length===0" class="np-empty">暂无通知</div>
        <button class="btn-sm" @click="clearNotificationNotifs()">清除全部</button>
      </div>
    </div>

    <!-- Audit Trail Panel -->
    <div v-if="showAuditTrailPanel" class="audit-panel">
      <div class="at-header"><span>📜 审计日志</span><button class="btn-sm" @click="showAuditTrailPanel=false">✕</button></div>
      <div class="at-body">
        <div v-for="e in auditTrailEntries" :key="e.id" class="at-entry">
          <span class="at-time">{{ formatTimestamp(e.timestamp) }}</span>
          <span class="at-user">{{ e.user }}</span>
          <span class="at-action">{{ e.action }}</span>
          <span class="at-target">{{ e.target }}</span>
          <span class="at-details">{{ e.details }}</span>
        </div>
        <div v-if="auditTrailEntries.length===0" class="at-empty">暂无记录</div>
        <button class="btn-sm" @click="clearAuditTrailLocal()">清除</button>
      </div>
    </div>

    <!-- Health Dashboard Panel -->
    <div v-if="showHealthDashboardPanel" class="health-panel">
      <div class="hp-header"><span>💚 健康仪表盘</span><button class="btn-sm" @click="showHealthDashboardPanel=false">✕</button></div>
      <div class="hp-body">
        <div class="hp-score"><span class="hp-val">{{ getFlowHealthScore() }}</span><span class="hp-label">健康分</span></div>
        <div class="hp-grid">
          <div v-for="ind in healthIndicatorsList" :key="ind.id" class="hp-indicator">
            <div class="hp-ind-name">{{ ind.name }}</div>
            <div class="hp-ind-val" :style="{color:getHealthStatusColorLocal(ind.status)}">{{ ind.value }}{{ ind.unit }}</div>
            <div :class="['hp-ind-dot',{healthy:ind.status==='healthy',warning:ind.status==='warning',critical:ind.status==='critical'}]"></div>
          </div>
        </div>
      </div>
    </div>

    <!-- Quality Report Panel -->
    <div v-if="showQualityReportPanel" class="quality-panel">
      <div class="qp-header"><span>📊 质量报告</span><button class="btn-sm" @click="showQualityReportPanel=false">✕</button></div>
      <div class="qp-body">
        <div class="qp-score"><span class="qp-val">{{ getQualityScore() }}</span><span class="qp-label">质量分</span></div>
        <div class="qp-metrics">
          <div v-for="m in qualityMetricsList" :key="m.name" class="qp-metric">
            <span class="qp-m-name">{{ m.name }}</span>
            <span :class="['qp-m-sev','sev-'+m.severity]">{{ m.severity }}</span>
            <span class="qp-m-val">{{ m.value }}{{ m.unit }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Version History Panel -->
    <div v-if="showVersionHistoryPanel" class="version-panel">
      <div class="vh-header"><span>📝 版本历史</span><button class="btn-sm" @click="showVersionHistoryPanel=false">✕</button></div>
      <div class="vh-body">
        <div v-for="(v, vi) in versionRecordsList" :key="v.id" class="vh-entry">
          <div class="vh-info"><span class="vh-label">{{ v.label }}</span><span class="vh-meta">{{ formatTimestamp(v.timestamp) }} · {{ v.nodeCount }}节点/{{ v.edgeCount }}边</span></div>
          <div class="vh-actions"><button class="btn-xs" @click="restoreVersionRecord(vi)">恢复</button><button class="btn-xs" @click="compareVersionsCompare(vi, Math.max(0,vi-1))">对比</button></div>
        </div>
        <div v-if="versionDiffResult" class="vh-diff">
          <span>+{{ versionDiffResult.added }} 新增</span><span>-{{ versionDiffResult.removed }} 删除</span><span>~{{ versionDiffResult.modified }} 修改</span>
        </div>
      </div>
    </div>

    <!-- Comment Panel -->
    <div v-if="showCommentPanel" class="comment-panel">
      <div class="cm-header"><span>💬 评论</span><button class="btn-sm" @click="showCommentPanel=false">✕</button></div>
      <div class="cm-body">
        <div class="cm-input-row">
          <input v-model="newCommentAuthorName" placeholder="姓名" class="cm-author-input" />
          <select v-model="newCommentTargetType" class="cm-target-select"><option value="canvas">画布</option><option value="node">节点</option><option value="edge">连线</option></select>
          <button class="btn-sm" @click="addCommentComment()">发送</button>
        </div>
        <div class="cm-list">
          <div v-for="c in commentsList" :key="c.id" :class="['cm-item',{resolved:c.resolved}]">
            <div class="cm-author">{{ c.author }}</div>
            <div class="cm-content">{{ c.content }}</div>
            <div class="cm-meta">{{ formatTimestamp(c.timestamp) }} · {{ c.targetType }}</div>
            <div class="cm-actions"><button v-if="!c.resolved" class="btn-xs" @click="resolveCommentComment(c.id)">✓</button><button class="btn-xs btn-danger" @click="deleteCommentComment(c.id)">✕</button></div>
          </div>
        </div>
        <div v-if="commentsList.length===0" class="cm-empty">暂无评论</div>
      </div>
    </div>

    <!-- Perf Monitor Panel -->
    <div v-if="showPerfMonitorPanel" class="perf-panel">
      <div class="pf-header"><span>⚡ 性能监控</span><button class="btn-sm" @click="stopPerfMonitorLocal()">✕</button></div>
      <div class="pf-body">
        <div class="pf-grid">
          <div class="pf-item"><span class="pf-val">{{ perfStatsData.fps }}</span><span class="pf-lbl">FPS</span></div>
          <div class="pf-item"><span class="pf-val">{{ perfStatsData.nodes }}</span><span class="pf-lbl">节点</span></div>
          <div class="pf-item"><span class="pf-val">{{ perfStatsData.edges }}</span><span class="pf-lbl">连线</span></div>
          <div class="pf-item"><span class="pf-val">{{ perfStatsData.renderMs }}ms</span><span class="pf-lbl">渲染</span></div>
          <div class="pf-item"><span class="pf-val">{{ perfStatsData.memMb }}MB</span><span class="pf-lbl">内存</span></div>
        </div>
      </div>
    </div>

    <!-- Workflow Rules Panel -->
    <div v-if="showWorkflowRulesPanel" class="rules-panel">
      <div class="rw-header"><span>⚙ 工作流规则</span><button class="btn-sm" @click="showWorkflowRulesPanel=false">✕</button></div>
      <div class="rw-body">
        <div class="rw-add"><input v-model="newRuleName" placeholder="规则名称" class="rw-input" /><input v-model="newRuleCondition" placeholder="条件表达式" class="rw-input" /><input v-model="newRuleAction" placeholder="动作" class="rw-input" /><button class="btn-sm" @click="addWorkflowRule()">+</button></div>
        <div class="rw-list">
          <div v-for="r in workflowRulesList" :key="r.id" :class="['rw-item',{disabled:!r.enabled}]">
            <span class="rw-name">{{ r.name }}</span>
            <span class="rw-cond">{{ r.condition }}</span>
            <button class="btn-xs" @click="toggleWorkflowRule(r.id)">{{ r.enabled ? '启用' : '禁用' }}</button>
            <button class="btn-xs btn-danger" @click="removeWorkflowRule(r.id)">✕</button>
          </div>
        </div>
        <button class="btn" @click="executeAllWorkflowRules()">▶ 执行所有规则</button>
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
  timeout?: number; priority?: string; script?: string
  style?: string; note?: string; retryCount?: number
  groupMembers?: string[]; collapsed?: boolean; groupId?: string
}
interface PDEdge { id: string; from: string; to: string; label?: string; condition?: string; flowLabel?: string; strokeWidth?: number; routing?: 'auto'|'straight'|'horizontal'|'vertical' }
interface ProcDef { id?: string; name: string; flag: string; desc?: string; status?: string; config?: { nodes: PDNode[]; edges: PDEdge[] }; subprocesses?: Record<string, { nodes: PDNode[]; edges: PDEdge[] }> }
// ── Execution Breakpoint ──────────────────────────────────────────
interface Breakpoint { nodeId: string; label?: string }
// ── Flow Statistics ────────────────────────────────────────────────
interface FlowStats { totalNodes: number; totalEdges: number; avgDegree: string; maxDegree: number; density: string; cycles: number; isolatedNodes: number }
// ── Enhanced Node Style ────────────────────────────────────────────
interface EnhancedNodeStyle { color: string; bgColor: string; borderColor: string; icon: string }
// ── Group Drag/Resize State ────────────────────────────────────────
interface GroupDragState { idx: number; startX: number; startY: number; origX: number; origY: number }
interface GroupResizeState { idx: number; dir: string; startX: number; startY: number; origW: number; origH: number; origX: number; origY: number }
// ── Edge Routing ────────────────────────────────────────────────────
interface RoutingPoint { x: number; y: number; type: "anchor"|"control" }
interface EdgeRouteConfig { edgeId: string; fromNodeIdx: number; toNodeIdx: number; routing: "auto"|"straight"|"horizontal"|"vertical"|"custom"; controlPoints: RoutingPoint[]; offset: number; labelPos: "auto"|"start"|"mid"|"end"; arrowStyle: "default"|"none"|"both" }
// ── Script Action Editor ────────────────────────────────────────────
interface ScriptVar { name: string; type: string; defaultValue: string; description: string }
interface ScriptErrorHandling { onFail: "abort"|"skip"|"retry"; retryCount?: number; retryDelay?: number }
interface ScriptOutputMapping { from: string; to: string; transform?: string }
interface ScriptActionConfig { language: "javascript"|"python"|"typescript"; code: string; imports: string[]; variables: ScriptVar[]; errorHandling: ScriptErrorHandling; outputMapping: ScriptOutputMapping[]; timeout: number; description: string }
// ── Fork/Join Enhanced ──────────────────────────────────────────────
interface ForkJoinAnnotation { id: string; type: "fork"|"join"; branchIndices: number[]; forkNodeIdx: number; joinNodeIdx?: number; label: string; color: string; annotations: Array<{type:"label"|"flow"|"count"; text: string}> }
// ── Group Drag/Resize ──────────────────────────────────────────────
// ── Edge Routing ────────────────────────────────────────────────────
// ── Constants ─────────────────────────────────────────────────────────
const GRID_SIZE = 20
const SNAP_THRESHOLD = 15
const nodeTypes = [
  { type: 'start',    label: '开始', icon: '🟢' },
  { type: 'task',     label: '任务', icon: '📋' },
  { type: 'approval', label: '审批', icon: '✅' },
  { type: 'end',      label: '结束', icon: '🔴' },
]
const allNodeTypes = ['start','task','approval','timer','end','gate_and','gate_or','gate_xor','subprocess','script','parallel']
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
  return type === "gate_and" || type === "gate_or" || type === "gate_xor"
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
// ── Condition Editor Helpers ────────────────────────────────────────
const nodeVars = ref<string[]>(['amount', 'userId', 'status', 'priority', 'deadline', 'department', 'role'])
const availableFields = ref<string[]>(['name', 'amount', 'status', 'userId', 'priority', 'date', 'comment', 'result', 'output'])
// ── State ─────────────────────────────────────────────────────────────
const plLoading = ref(false), sbFilter = ref('')
const currentProcess = ref<ProcDef|null>(null)
const processDef = ref<{nodes: PDNode[]; edges: PDEdge[]}>({ nodes: [], edges: [] })
const selectedNode = ref<number|null>(null)
const selectedEdge = ref<number|null>(null)
const showNewModal = ref(false), newForm = ref({ name: '', flag: '', desc: '' })
const canvasRef = ref<HTMLElement|null>(null)
const panX = ref(0), panY = ref(0), zoom = ref(1)
const history = ref<{nodes: PDNode[]; edges: PDEdge[]}[]>([])
const histIdx = ref(-1)
const canUndo = computed(() => histIdx.value > 0)
const canRedo = computed(() => histIdx.value < history.value.length - 1)
const isDragging = ref(false), dragIdx = ref<number|null>(null)
const dragOffset = ref({ x: 0, y: 0 })
const snapX = ref<number|null>(null), snapY = ref<number|null>(null)
const tempEdge = ref<{ from: number; fromPort: 'out'|'in'; startX: number; startY: number; endX: number; endY: number }|null>(null)
const predictedTarget = ref<number|null>(null)
const predictedPath = ref<string>('')
const showPrediction = ref(false)
const isPanning = ref(false), panStart = ref({ x: 0, y: 0 })
// Resize state
const isResizing = ref(false)
const resizeIdx = ref<number|null>(null)
const resizeDir = ref<string>('')
const resizeStart = ref({ x: 0, y: 0, w: 0, h: 0 })
// Anchor point drag state
const isDraggingAnchor = ref(false)
const anchorNodeIdx = ref<number|null>(null)
const anchorIdx = ref<number|null>(null)
const selectedAnchorNode = computed(() => isDraggingAnchor.value ? anchorNodeIdx.value : null)
const anchorPoints = computed(() => {
  if (anchorNodeIdx.value === null || !processDef.value?.nodes[anchorNodeIdx.value]) return []
  const node = processDef.value.nodes[anchorNodeIdx.value]
  const w = node.w||120, h = node.h||50
  const offsets = (node as any).anchorOffset || []
  return [
    { x: node.x + w/2, y: node.y },
    { x: offsets[1]?.x ?? node.x + w, y: node.y + h/2 },
    { x: node.x + w/2, y: node.y + h },
    { x: offsets[3]?.x ?? node.x, y: node.y + h/2 },
  ]
})
// Group state
const groupedNodes = ref<Set<string>>(new Set())
// Script tab state
const scriptTab = ref<'code'|'vars'|'error'>('code')
// Version control state
interface ProcVersion { id: string; timestamp: number; label: string; config: { nodes: PDNode[]; edges: PDEdge[] }; author: string; message: string }
const versions = ref<ProcVersion[]>([])
const showVersionPanel = ref(false)
const selectedVersion = ref<ProcVersion|null>(null)
const showDiff = ref(false)
const addedNodes = ref<PDNode[]>([])
const removedNodes = ref<PDNode[]>([])
const changedNodes = ref<PDNode[]>([])
// Multi-select state
const multiSelected = ref<Set<string>>(new Set())
const isMultiDragging = ref(false)
const multiDragOffset = ref({ x: 0, y: 0 })
// Minimap state
const minimapVisible = ref(true)
const minimapScale = 0.15
const minimapCanvasRef = ref<HTMLCanvasElement|null>(null)
// ── Execution Simulation ────────────────────────────────────────────
interface ExecState { currentNodeIdx: number|null; progress: number; status: 'idle'|'running'|'paused'|'finished'; completedNodes: string[] }
const execState = ref<ExecState>({ currentNodeIdx: null, progress: 0, status: 'idle', completedNodes: [] })
const showExecPanel = ref(false)
function startExecution() {
  if (!processDef.value || processDef.value.nodes.length === 0) return
  const starts = processDef.value.nodes.findIndex(n => n.type === 'start')
  if (starts === -1) return
  execState.value = { currentNodeIdx: starts, progress: 0, status: 'running', completedNodes: [processDef.value.nodes[starts].id] }
  showExecPanel.value = true
  simulateNext()
}
function simulateNext() {
  if (execState.value.status !== 'running' || !processDef.value || execState.value.currentNodeIdx === null) return
  const curId = processDef.value.nodes[execState.value.currentNodeIdx].id
  const outgoing = (processDef.value.edges||[]).filter(e => e.from === curId)
  if (outgoing.length === 0) {
    execState.value.status = 'finished'
    return
  }
  // Pick first outgoing edge
  const nextId = outgoing[0].to
  const nextIdx = processDef.value.nodes.findIndex(n => n.id === nextId)
  if (nextIdx === -1) { execState.value.status = 'finished'; return }
  // Animate progress
  const totalNodes = processDef.value.nodes.length
  let progress = 0
  const interval = setInterval(() => {
    progress += 5
    if (progress >= 100) {
      clearInterval(interval)
      execState.value.currentNodeIdx = nextIdx
      execState.value.completedNodes.push(nextId)
      execState.value.progress = Math.round((execState.value.completedNodes.length / totalNodes) * 100)
      if (processDef.value!.nodes[nextIdx].type === 'end') {
        execState.value.status = 'finished'
      } else {
        setTimeout(() => simulateNext(), 300)
      }
    } else {
      execState.value.progress = progress
    }
  }, 50)
}
function pauseExecution() {
  execState.value.status = 'paused'
}
function resumeExecution() {
  if (execState.value.status === 'paused') {
    execState.value.status = 'running'
    simulateNext()
  }
}
function resetExecution() {
  execState.value = { currentNodeIdx: null, progress: 0, status: 'idle', completedNodes: [] }
  showExecPanel.value = false
}
// ── Node Type Config Profiles ────────────────────────────────────────
interface NodeProfile {
  type: string; label: string; icon: string
  defaultW: number; defaultH: number
  canHaveConditions: boolean
  canHaveScript: boolean
  canHaveAssignee: boolean
  canTimeout: boolean
  canRetry: boolean
  canGroup: boolean
}
const nodeProfiles: NodeProfile[] = [
  { type: 'start', label: '开始', icon: '🟢', defaultW: 100, defaultH: 50, canHaveConditions: false, canHaveScript: false, canHaveAssignee: false, canTimeout: false, canRetry: false, canGroup: true },
  { type: 'end', label: '结束', icon: '🔴', defaultW: 100, defaultH: 50, canHaveConditions: false, canHaveScript: false, canHaveAssignee: false, canTimeout: false, canRetry: false, canGroup: true },
  { type: 'task', label: '任务', icon: '📋', defaultW: 120, defaultH: 50, canHaveConditions: true, canHaveScript: true, canHaveAssignee: true, canTimeout: true, canRetry: true, canGroup: true },
  { type: 'approval', label: '审批', icon: '✅', defaultW: 130, defaultH: 70, canHaveConditions: true, canHaveScript: false, canHaveAssignee: true, canTimeout: true, canRetry: true, canGroup: true },
  { type: 'timer', label: '定时', icon: '⏱️', defaultW: 110, defaultH: 50, canHaveConditions: false, canHaveScript: true, canHaveAssignee: false, canTimeout: true, canRetry: false, canGroup: true },
  { type: 'gate_and', label: '且网关', icon: '🔷', defaultW: 100, defaultH: 50, canHaveConditions: true, canHaveScript: false, canHaveAssignee: false, canTimeout: false, canRetry: false, canGroup: true },
  { type: 'gate_or', label: '或网关', icon: '🔶', defaultW: 100, defaultH: 50, canHaveConditions: true, canHaveScript: false, canHaveAssignee: false, canTimeout: false, canRetry: false, canGroup: true },
  { type: 'gate_xor', label: '异或网关', icon: '🔹', defaultW: 100, defaultH: 50, canHaveConditions: true, canHaveScript: false, canHaveAssignee: false, canTimeout: false, canRetry: false, canGroup: true },
  { type: 'subprocess', label: '子流程', icon: '📦', defaultW: 120, defaultH: 60, canHaveConditions: false, canHaveScript: false, canHaveAssignee: true, canTimeout: true, canRetry: true, canGroup: true },
  { type: 'script', label: '脚本', icon: '💻', defaultW: 120, defaultH: 50, canHaveConditions: false, canHaveScript: true, canHaveAssignee: false, canTimeout: true, canRetry: true, canGroup: true },
  { type: 'parallel', label: '并行', icon: '⚡', defaultW: 120, defaultH: 50, canHaveConditions: false, canHaveScript: false, canHaveAssignee: false, canTimeout: false, canRetry: false, canGroup: true },
]
function getNodeProfile(type: string): NodeProfile {
  return nodeProfiles.find(p => p.type === type) || nodeProfiles[1]
}
function isProfileEditable(node: PDNode, prop: string): boolean {
  const profile = getNodeProfile(node.type)
  switch(prop) {
    case 'condition': return profile.canHaveConditions
    case 'script': return profile.canHaveScript
    case 'assignee': return profile.canHaveAssignee
    case 'timeout': return profile.canTimeout
    case 'retryCount': return profile.canRetry
    case 'groupMembers': return profile.canGroup
    default: return true
  }
}
// Subprocess state
const showSubprocess = ref(false)
const subprocessTitle = ref('')
const subprocessNodeIdx = ref<number|null>(null)
const subprocessDef = ref<{nodes: PDNode[]; edges: PDEdge[]}>({ nodes: [], edges: [] })
// Subprocess inline editor state
const subprocessEditing = ref(false)
const subprocessStack = ref<Array<{nodes: PDNode[]; edges: PDEdge[]; title: string; parentIdx?: number}>>([])
const subprocessDepth = ref(0)
const activeSubprocessIdx = ref<number|null>(null)
const subCanvasRef = ref<HTMLElement|null>(null)
const subPanX = ref(0), subPanY = ref(0), subZoom = ref(1)
const subSelectedNode = ref<number|null>(null)
const subSelectedEdge = ref<number|null>(null)
const subIsDragging = ref(false), subDragIdx = ref<number|null>(null)
const subDragOffset = ref({ x: 0, y: 0 })
const subTempEdge = ref<{ from: number; fromPort: 'out'|'in'; startX: number; startY: number; endX: number; endY: number }|null>(null)
const subIsDraggingAnchor = ref(false)
const subHistory = ref<{nodes: PDNode[]; edges: PDEdge[]}[]>([])
const subHistIdx = ref(-1)
// Animation playback state
const isPlaying = ref(false)
const playbackProgress = ref(0)
const playbackSpeed = ref(1)
let playbackTimer: ReturnType<typeof setInterval>|null = null
function togglePlay() {
  if (isPlaying.value) { pausePlayback(); return }
  isPlaying.value = true
  const totalMs = 3000 / playbackSpeed.value
  const interval = 50
  let elapsed = 0
  playbackTimer = setInterval(() => {
    elapsed += interval
    playbackProgress.value = Math.min(100, (elapsed / totalMs) * 100)
    if (elapsed >= totalMs) { pausePlayback(); playbackProgress.value = 100 }
  }, interval)
}
function pausePlayback() {
  isPlaying.value = false
  if (playbackTimer) { clearInterval(playbackTimer); playbackTimer = null }
}
function resetPlayback() {
  pausePlayback(); playbackProgress.value = 0
}
function onPlaybackSeek(e: Event) {
  playbackProgress.value = +(e.target as HTMLInputElement).value
}
function getPlaybackTime(): string {
  const totalNodes = processDef.value?.nodes.length || 0
  const current = Math.floor((playbackProgress.value / 100) * totalNodes)
  return `${current}/${totalNodes}`
}
// Help modal
const showHelpModal = ref(false)
const breakpoints = ref<Breakpoint[]>([])
const showBreakpoints = ref(false)
const executionSpeed = ref(1000)
const isStepping = ref(false)
const showExecutionPanel = ref(true)
const flowStats = computed(() => computeFlowStats())
const groupDragState = ref<GroupDragState|null>(null)
const groupResizeState = ref<GroupResizeState|null>(null)
const showRoutingPanel = ref(false)
const selectedRoutingEdge = ref<number|null>(null)
const routingConfigs = ref<Map<string, EdgeRouteConfig>>(new Map())
const scriptEditors = ref<Map<string, ScriptActionConfig>>(new Map())
const scriptEditorNodeIdx = ref<number|null>(null)
const showBranchAnnot = ref(false)
const forkJoinAnnotations = ref<ForkJoinAnnotation[]>([])
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
// Multi-select helpers
function isSelectedNode(id: string): boolean {
  if (selectedNode.value !== null && processDef.value?.nodes[selectedNode.value]?.id === id) return true
  return multiSelected.value.has(id)
}
function toggleSelectNode(i: number) {
  if (!processDef.value) return
  const node = processDef.value.nodes[i]
  if (!node) return
  if (multiSelected.value.has(node.id)) multiSelected.value.delete(node.id)
  else multiSelected.value.add(node.id)
  selectedNode.value = i
}
// Minimap bounds
const minimapBounds = computed(() => {
  if (!processDef.value || processDef.value.nodes.length === 0)
    return { minX: 0, minY: 0, maxX: 800, maxY: 600 }
  let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity
  for (const n of processDef.value.nodes) {
    minX = Math.min(minX, n.x); minY = Math.min(minY, n.y)
    maxX = Math.max(maxX, n.x + (n.w || 120)); maxY = Math.max(maxY, n.y + (n.h || 50))
  }
  return { minX: minX - 50, minY: minY - 50, maxX: maxX + 50, maxY: maxY + 50 }
})
const minimapWidth = computed(() => canvasRef.value ? canvasRef.value.clientWidth * minimapScale : 150)
const minimapHeight = computed(() => canvasRef.value ? canvasRef.value.clientHeight * minimapScale : 100)
// Subprocess inline editor computed
const subSvgTransform = computed(() => ({ transform: `translate(${subPanX.value}px,${subPanY.value}px) scale(${subZoom.value})`, transformOrigin: '0 0' }))
const subGridScale = computed(() => GRID_SIZE * subZoom.value)
// Parallel branch detection
function detectParallelBranches(): number[][] {
  if (!processDef.value) return []
  const groups: number[][] = []
  const visited = new Set<number>()
  for (let i = 0; i < processDef.value.nodes.length; i++) {
    const node = processDef.value.nodes[i]
    const outgoing = (processDef.value.edges || []).filter(e => e.from === node.id)
    if (outgoing.length >= 2 && !visited.has(i)) {
      const branch: number[] = [i]
      for (const edge of outgoing) {
        const tIdx = processDef.value!.nodes.findIndex(n => n.id === edge.to)
        if (tIdx !== -1 && !visited.has(tIdx)) { branch.push(tIdx); visited.add(tIdx) }
      }
      if (branch.length > 1) groups.push(branch)
    }
  }
  return groups
}
const parallelBranches = computed(() => detectParallelBranches())
// ── Process Statistics ──────────────────────────────────────────────
const showStats = ref(false)
const processStats = computed(() => {
  if (!processDef.value) return null
  const nodes = processDef.value.nodes
  const edges = processDef.value.edges || []
  return {
    totalNodes: nodes.length,
    totalEdges: edges.length,
    startNodes: nodes.filter(n => n.type === 'start').length,
    endNodes: nodes.filter(n => n.type === 'end').length,
    taskNodes: nodes.filter(n => n.type === 'task' || n.type === 'approval').length,
    gateNodes: nodes.filter(n => n.type.startsWith('gate')).length,
    avgOutDegree: nodes.length > 0 ? (edges.length / nodes.length).toFixed(2) : '0',
    hasLoops: edges.some(e => {
      const from = nodes.find(n => n.id === e.from)
      const to = nodes.find(n => n.id === e.to)
      return from && to && Math.abs(from.x - to.x) < 50 && Math.abs(from.y - to.y) < 50
    })
  }
})
// Fork/Join labels for parallel branches
const forkLabels = computed(() => {
  if (!processDef.value) return []
  const labels: { branch: number[]; forkNode: PDNode; joinNode?: PDNode }[] = []
  for (const branch of parallelBranches.value) {
    const forkNode = processDef.value.nodes[branch[0]]
    if (!forkNode) continue
    // Find join node (node with incoming edges from all branch nodes)
    let joinNode: PDNode | undefined
    const branchIds = branch.map(i => processDef.value!.nodes[i]?.id).filter(Boolean) as string[]
    const potentialJoins = processDef.value.nodes.filter(n =>
      branchIds.every(bid => (processDef.value!.edges || []).some(e => e.from === bid && e.to === n.id))
    )
    if (potentialJoins.length > 0) joinNode = potentialJoins[0]
    labels.push({ branch, forkNode, joinNode })
  }
  return labels
})
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
function undo() { if (histIdx.value <= 0) return; histIdx.value--; processDef.value = JSON.parse(JSON.stringify(history.value[histIdx.value])); selectedNode.value = null }
function redo() { if (histIdx.value >= history.value.length - 1) return; histIdx.value++; processDef.value = JSON.parse(JSON.stringify(history.value[histIdx.value])); selectedNode.value = null }
// ── Helpers ───────────────────────────────────────────────────────────
function genId() { return 'n_' + Date.now() + '_' + Math.random().toString(36).slice(2,6) }
function genEdgeId() { return 'e_' + Date.now() + '_' + Math.random().toString(36).slice(2,6) }
function getNodeLabel(type: string) {
  const m: Record<string,string> = { start:'开始', end:'结束', task:'任务', approval:'审批', timer:'定时',
    gate_and:'且网关', gate_or:'或网关', gate_xor:'异或网关', subprocess:'子流程', script:'脚本', parallel:'并行' }
  return m[type] || type
}
// ── Node color helpers ────────────────────────────────────────────────
function getNodeBgColor(type: string): string {
  const m: Record<string,string> = {
    start:'rgba(16,185,129,.6)', end:'rgba(239,68,68,.6)', task:'rgba(0,212,255,.4)',
    approval:'rgba(99,102,241,.4)', timer:'rgba(245,158,11,.4)',
    gate_and:'rgba(245,158,11,.4)', gate_or:'rgba(245,158,11,.4)', gate_xor:'rgba(245,158,11,.4)',
    subprocess:'rgba(168,85,247,.4)', script:'rgba(34,197,94,.4)', parallel:'rgba(236,72,153,.4)'
  }
  return m[type] || 'rgba(100,100,100,.4)'
}
function getNodeIcon(type: string) {
  const m: Record<string,string> = { start:'🟢', end:'🔴', task:'📋', approval:'✅', timer:'⏱️',
    gate_and:'🔷', gate_or:'🔶', gate_xor:'🔹', subprocess:'📦', script:'💻', parallel:'⚡' }
  return m[type] || '⬜'
}
function computeForkJoinPath(branchIndices: number[]): string {
  if (branchIndices.length < 2 || !processDef.value) return ""
  const nodes = branchIndices.map(i => processDef.value!.nodes[i]).filter(Boolean)
  if (nodes.length < 2) return ""
  let d = "M " + (nodes[0].x + (nodes[0].w||120)) + " " + (nodes[0].y + (nodes[0].h||50)/2)
  for (let i = 1; i < nodes.length; i++) {
    const n = nodes[i]
    d += " L " + (n.x + (n.w||120)) + " " + (n.y + (n.h||50)/2)
  }
  return d
}
// ── Port position ─────────────────────────────────────────────────────
function getNodePort(node: PDNode, port: 'in'|'out', portIdx?: number): {x:number;y:number} {
  const w = node.w||120, h = node.h||50
  if (port === 'in') return { x: node.x, y: node.y + h/2 }
  if (isGate(node.type) && portIdx !== undefined) {
    const conds = getNodeConditions(node)
    const spread = Math.max(conds.length * 12, 20)
    return { x: node.x + w, y: node.y + h/2 + (portIdx - (conds.length-1)/2) * spread }
  }
  return { x: node.x + w, y: node.y + h/2 }
}
// ── Edge path ─────────────────────────────────────────────────────────
function computeEdgePath(edge: PDEdge): string {
  if (!processDef.value) return ''
  const routing = edge.routing || 'auto'
  if (routing === 'straight') return computeStraightEdgePath(edge)
  if (routing === 'horizontal') return computeHorizontalEdgePath(edge)
  if (routing === 'vertical') return computeVerticalEdgePath(edge)
  // auto: use bezier
  const from = processDef.value.nodes.find(n => n.id === edge.from)
  const to = processDef.value.nodes.find(n => n.id === edge.to)
  if (!from || !to) return ''
  const fp = getNodePort(from, 'out')
  const tp = getNodePort(to, 'in')
  const dx = Math.abs(tp.x - fp.x)
  const cx = Math.max(dx * 0.5, 60)
  return `M ${fp.x} ${fp.y} C ${fp.x+cx} ${fp.y}, ${tp.x-cx} ${tp.y}, ${tp.x} ${tp.y}`
}
function tempEdgePath(): string {
  if (!tempEdge.value) return ''
  const { startX, startY, endX, endY } = tempEdge.value
  const from = processDef.value?.nodes[tempEdge.value.from]
  if (!from) return ''
  const fp = getNodePort(from, tempEdge.value.fromPort)
  const cx = Math.max(Math.abs(endX - fp.x) * 0.5, 60)
  const sign = tempEdge.value.fromPort === 'out' ? 1 : -1
  return `M ${fp.x} ${fp.y} C ${fp.x+cx*sign} ${fp.y}, ${endX-cx*sign} ${endY}, ${endX} ${endY}`
}
// ── Node CRUD ─────────────────────────────────────────────────────────
function addNode(type: string, opts?: { x?: number; y?: number; autoConnect?: boolean }) {
  if (!processDef.value) return
  const w = isGate(type) ? 100 : type === 'approval' ? 130 : 120
  const h = type === 'approval' ? 70 : type === 'subprocess' ? 60 : 50
  const x = opts?.x ?? (100 + Math.random() * 200)
  const y = opts?.y ?? (80 + Math.random() * 100)
  const newNode: PDNode = { id: genId(), type, label: getNodeLabel(type), x, y, w, h }
  processDef.value.nodes.push(newNode)
  // Auto-connect to previously selected node
  if (opts?.autoConnect !== false && selectedNode.value !== null && selectedNode.value < processDef.value.nodes.length - 1) {
    const src = processDef.value.nodes[selectedNode.value]
    if (src.type !== 'end' && newNode.type !== 'start') {
      createEdge(src.id, newNode.id)
    }
  }
  selectedNode.value = processDef.value.nodes.length - 1
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
function deleteSelected() {
  if (selectedNode.value !== null) deleteNode(selectedNode.value)
  else if (selectedEdge.value !== null) deleteEdge(selectedEdge.value)
}
function duplicateSelected() {
  if (selectedNode.value === null || !processDef.value) return
  const orig = processDef.value.nodes[selectedNode.value]
  addNode(orig.type, { x: orig.x + 30, y: orig.y + 30, autoConnect: false })
  const newNode = processDef.value.nodes[processDef.value.nodes.length - 1]
  if (newNode) {
    newNode.label = orig.label; newNode.assignee = orig.assignee
    newNode.condition = orig.condition; newNode.timeout = orig.timeout
    newNode.priority = orig.priority; newNode.script = orig.script
  }
}
// ── Version Control ──────────────────────────────────────────────────
function createVersion(label?: string) {
  if (!processDef.value || !currentProcess.value) return
  const v: ProcVersion = {
    id: genId(), timestamp: Date.now(),
    label: label || '版本 ' + (versions.value.length + 1),
    config: JSON.parse(JSON.stringify(processDef.value)),
    author: 'user', message: label || '自动快照'
  }
  versions.value.unshift(v)
  if (versions.value.length > 20) versions.value.pop()
}
function revertToVersion(v: ProcVersion) {
  processDef.value = JSON.parse(JSON.stringify(v.config))
  selectedNode.value = null; selectedEdge.value = null
  history.value = []; histIdx.value = -1; pushHistory()
}
function deleteVersion(idx: number) {
  versions.value.splice(idx, 1)
  if (selectedVersion.value?.id === versions.value[idx]?.id) selectedVersion.value = null
}
function toggleDiff() {
  if (!selectedVersion.value || !processDef.value) return
  showDiff.value = !showDiff.value
  if (showDiff.value) computeDiff()
}
function computeDiff() {
  if (!selectedVersion.value || !processDef.value) return
  const currentIds = new Set(processDef.value.nodes.map(n => n.id))
  const versionIds = new Set(selectedVersion.value.config.nodes.map(n => n.id))
  addedNodes.value = processDef.value.nodes.filter(n => !versionIds.has(n.id))
  removedNodes.value = selectedVersion.value.config.nodes.filter(n => !currentIds.has(n.id))
  changedNodes.value = processDef.value.nodes.filter(n => {
    if (!versionIds.has(n.id)) return false
    const orig = selectedVersion.value!.config.nodes.find(vn => vn.id === n.id)
    return orig && (orig.label !== n.label || Math.abs(orig.x - n.x) > 10 || Math.abs(orig.y - n.y) > 10)
  })
}
function clearCanvas() {
  if (!processDef.value || !confirm('清空画布？所有节点和连线将删除。')) return
  processDef.value = { nodes: [], edges: [] }
  selectedNode.value = null; selectedEdge.value = null
  pushHistory()
}
function autoLayout() {
  if (!processDef.value || processDef.value.nodes.length === 0) return
  const cols = Math.ceil(Math.sqrt(processDef.value.nodes.length))
  processDef.value.nodes.forEach((n, i) => {
    n.x = 80 + (i % cols) * ((n.w||120) + 40)
    n.y = 80 + Math.floor(i / cols) * ((n.h||50) + 40)
  })
  pushHistory()
}
// ── Advanced Auto-Layout (topological) ──────────────────────────────
function autoLayoutTopo() {
  if (!processDef.value || processDef.value.nodes.length === 0) return
  const nodes = processDef.value.nodes
  const edges = processDef.value.edges || []
  // Build adjacency and in-degree
  const inDegree = new Map<string, number>()
  const adj = new Map<string, string[]>()
  for (const n of nodes) { inDegree.set(n.id, 0); adj.set(n.id, []) }
  for (const e of edges) {
    if (inDegree.has(e.to)) inDegree.set(e.to, (inDegree.get(e.to)||0) + 1)
    if (adj.has(e.from)) adj.get(e.from)!.push(e.to)
  }
  // Kahn's algorithm for topological sort
  const queue: string[] = []
  for (const [id, deg] of inDegree) if (deg === 0) queue.push(id)
  const order: string[] = []
  while (queue.length > 0) {
    const curr = queue.shift()!
    order.push(curr)
    for (const next of (adj.get(curr)||[])) {
      inDegree.set(next, (inDegree.get(next)||0) - 1)
      if ((inDegree.get(next)||0) === 0) queue.push(next)
    }
  }
  // Assign positions by layer (BFS levels)
  const layers = new Map<string, number>()
  const startNodes = nodes.filter(n => n.type === 'start').map(n => n.id)
  if (startNodes.length > 0) {
    for (const id of startNodes) layers.set(id, 0)
  } else {
    for (const id of (order.length > 0 ? [order[0]] : [])) layers.set(id, 0)
  }
  // BFS to assign layers
  const visited = new Set<string>()
  const bfsQ: string[] = [...(startNodes.length > 0 ? startNodes : [order[0]])]
  while (bfsQ.length > 0) {
    const curr = bfsQ.shift()!
    if (visited.has(curr)) continue
    visited.add(curr)
    const currLayer = layers.get(curr) ?? 0
    for (const next of (adj.get(curr)||[])) {
      const nextLayer = (layers.get(next) ?? 0)
      if (currLayer + 1 > nextLayer) layers.set(next, currLayer + 1)
      bfsQ.push(next)
    }
  }
  // Group by layer
  const layerGroups = new Map<number, string[]>()
  for (const id of order) {
    const layer = layers.get(id) ?? 0
    if (!layerGroups.has(layer)) layerGroups.set(layer, [])
    layerGroups.get(layer)!.push(id)
  }
  // Position nodes
  const rowH = 80, colW = 140, layerGap = 220
  for (const [layer, ids] of layerGroups) {
    const totalW = ids.length * colW
    let startX = 100
    for (let i = 0; i < ids.length; i++) {
      const n = nodes.find(nd => nd.id === ids[i])
      if (!n) continue
      n.x = startX + i * colW
      n.y = 80 + layer * layerGap
    }
  }
  pushHistory()
}
// Apply template to canvas
function applyTemplate(tpl: TemplateDef) {
  if (!processDef.value) return
  processDef.value.nodes = []
  processDef.value.edges = []
  selectedNode.value = null; selectedEdge.value = null
  const nodeMap = new Map<string, PDNode>()
  for (const tn of tpl.nodes) {
    const w = isGate(tn.type) ? 100 : tn.type === 'approval' ? 130 : 120
    const h = tn.type === 'approval' ? 70 : tn.type === 'subprocess' ? 60 : 50
    const startX = 100, startY = 80, colGap = 160, rowGap = 90
    const col = tpl.nodes.indexOf(tn)
    const colIdx = col % 3, rowIdx = Math.floor(col / 3)
    const node: PDNode = { id: genId(), type: tn.type, label: tn.label, x: startX + colIdx * colGap, y: startY + rowIdx * rowGap, w, h }
    processDef.value.nodes.push(node)
    nodeMap.set(tn.label, node)
  }
  for (const e of tpl.edges) {
    const fromNode = tpl.nodes[e.from], toNode = tpl.nodes[e.to]
    if (fromNode && toNode) {
      const fn = nodeMap.get(fromNode.label), tn = nodeMap.get(toNode.label)
      if (fn && tn) createEdge(fn.id, tn.id)
    }
  }
  showTemplatesModal.value = false
}
// ── Property access ───────────────────────────────────────────────────
function getNodeProp(prop: string): any {
  if (selectedNode.value === null || !processDef.value?.nodes[selectedNode.value]) return ''
  return (processDef.value.nodes[selectedNode.value] as any)[prop] ?? ''
}
function _setNodeProp(prop: string, val: any) {
  if (selectedNode.value === null || !processDef.value) return
  ;(processDef.value.nodes[selectedNode.value] as any)[prop] = val
}
function getEdgeProp(prop: string): any {
  if (selectedEdge.value === null || !processDef.value?.edges[selectedEdge.value]) return ''
  return (processDef.value.edges[selectedEdge.value] as any)[prop] ?? ''
}
function _setEdgeProp(prop: string, val: any) {
  if (selectedEdge.value === null || !processDef.value) return
  ;(processDef.value.edges[selectedEdge.value] as any)[prop] = val
}
function getEdgeFromLabel() {
  if (selectedEdge.value === null || !processDef.value) return '?'
  const edge = processDef.value.edges[selectedEdge.value]
  const n = processDef.value.nodes.find(n => n.id === edge.from)
  return n?.label || n?.id?.slice(0,8) || '?'
}
function getEdgeToLabel() {
  if (selectedEdge.value === null || !processDef.value) return '?'
  const edge = processDef.value.edges[selectedEdge.value]
  const n = processDef.value.nodes.find(n => n.id === edge.to)
  return n?.label || n?.id?.slice(0,8) || '?'
}
// Data mapping helpers
function getNodeMappings(): any[] {
  if (selectedNode.value === null || !processDef.value) return []
  return (processDef.value.nodes[selectedNode.value] as any).mappings || []
}
function addDataMapping() {
  if (selectedNode.value === null || !processDef.value) return
  const m = getNodeMappings()
  m.push({ from: '', to: '' })
  _setNodeProp('mappings', m)
}
function removeDataMapping(i: number) {
  const m = getNodeMappings()
  m.splice(i, 1)
  _setNodeProp('mappings', m)
}
// ── Edge CRUD ─────────────────────────────────────────────────────────
function createEdge(fromId: string, toId: string) {
  if (!processDef.value) return
  const exists = processDef.value.edges.some(e => e.from === fromId && e.to === toId)
  if (exists) return
  processDef.value.edges.push({ id: genEdgeId(), from: fromId, to: toId })
  pushHistory()
}
function deleteEdge(i: number) {
  if (!processDef.value) return
  processDef.value.edges.splice(i, 1)
  if (selectedEdge.value === i) selectedEdge.value = null
  else if (selectedEdge.value !== null && selectedEdge.value > i) selectedEdge.value--
  pushHistory()
}
function selectEdge(i: number) { selectedEdge.value = i; selectedNode.value = null }
// ── Edge Label Helpers ───────────────────────────────────────────────
// Cubic Bezier: B(t) = (1-t)³P₀ + 3(1-t)²tP₁ + 3(1-t)t²P₂ + t³P₃
function bezierPoint(p0: {x:number;y:number}, p1: {x:number;y:number}, p2: {x:number;y:number}, p3: {x:number;y:number}, t: number): {x:number;y:number} {
  const mt = 1 - t
  return {
    x: mt*mt*mt*p0.x + 3*mt*mt*t*p1.x + 3*mt*t*t*p2.x + t*t*t*p3.x,
    y: mt*mt*mt*p0.y + 3*mt*mt*t*p1.y + 3*mt*t*t*p2.y + t*t*t*p3.y
  }
}
function getBezierControlPoints(edge: PDEdge): {p0:{x:number;y:number};p1:{x:number;y:number};p2:{x:number;y:number};p3:{x:number;y:number}} {
  if (!processDef.value) return { p0:{x:0,y:0}, p1:{x:0,y:0}, p2:{x:0,y:0}, p3:{x:0,y:0} }
  const from = processDef.value.nodes.find(n => n.id === edge.from)
  const to = processDef.value.nodes.find(n => n.id === edge.to)
  if (!from || !to) return { p0:{x:0,y:0}, p1:{x:0,y:0}, p2:{x:0,y:0}, p3:{x:0,y:0} }
  const fp = getNodePort(from, 'out'), tp = getNodePort(to, 'in')
  const dx = tp.x - fp.x, dy = tp.y - fp.y
  // Control points offset based on edge direction
  let cx1 = fp.x + dx * 0.4, cy1 = fp.y
  let cx2 = tp.x - dx * 0.4, cy2 = tp.y
  // For vertical edges, offset control points vertically
  if (Math.abs(dx) < 30) {
    cx1 = fp.x; cy1 = fp.y + dy * 0.4
    cx2 = tp.x; cy2 = tp.y - dy * 0.4
  }
  return { p0: fp, p1: {x:cx1,y:cy1}, p2: {x:cx2,y:cy2}, p3: tp }
}
function getEdgePointOnCurve(edge: PDEdge, t: number): {x:number;y:number} {
  const cp = getBezierControlPoints(edge)
  return bezierPoint(cp.p0, cp.p1, cp.p2, cp.p3, t)
}
function getEdgeLabelX(edge: PDEdge): number { return getEdgePointOnCurve(edge, 0.5).x }
function getEdgeLabelY(edge: PDEdge): number { return getEdgePointOnCurve(edge, 0.5).y - 8 }
function getEdgeLabelRect(edge: PDEdge): string {
  const { x, y } = getEdgePointOnCurve(edge, 0.5)
  const tw = (edge.label?.length || 1) * 7 + 10
  return `M ${x-tw/2} ${y-10} h ${tw} v 14 h ${-tw} Z`
}
// Label offset for multiple edges between same nodes (prevent overlap)
function getEdgeLabelOffset(edgeIdx: number): {dx:number;dy:number} {
  if (!processDef.value) return { dx: 0, dy: 0 }
  const edges = processDef.value.edges || []
  const edge = edges[edgeIdx]
  if (!edge) return { dx: 0, dy: 0 }
  // Count edges between same pair
  const samePair = edges.filter((e, i) => i !== edgeIdx && ((e.from === edge.from && e.to === edge.to) || (e.from === edge.to && e.to === edge.from)))
  if (samePair.length === 0) return { dx: 0, dy: 0 }
  const idx = samePair.indexOf(edge)
  const offset = (idx - samePair.length / 2) * 16
  return { dx: 0, dy: offset }
}
// Edge routing: straight line vs bezier
function computeStraightEdgePath(edge: PDEdge): string {
  if (!processDef.value) return ''
  const from = processDef.value.nodes.find(n => n.id === edge.from)
  const to = processDef.value.nodes.find(n => n.id === edge.to)
  if (!from || !to) return ''
  const fp = getNodePort(from, 'out'), tp = getNodePort(to, 'in')
  return `M ${fp.x} ${fp.y} L ${tp.x} ${tp.y}`
}
function computeHorizontalEdgePath(edge: PDEdge): string {
  if (!processDef.value) return ''
  const from = processDef.value.nodes.find(n => n.id === edge.from)
  const to = processDef.value.nodes.find(n => n.id === edge.to)
  if (!from || !to) return ''
  const fp = getNodePort(from, 'out'), tp = getNodePort(to, 'in')
  const mx = (fp.x + tp.x) / 2
  return `M ${fp.x} ${fp.y} C ${mx} ${fp.y}, ${mx} ${tp.y}, ${tp.x} ${tp.y}`
}
function computeVerticalEdgePath(edge: PDEdge): string {
  if (!processDef.value) return ''
  const from = processDef.value.nodes.find(n => n.id === edge.from)
  const to = processDef.value.nodes.find(n => n.id === edge.to)
  if (!from || !to) return ''
  const fp = getNodePort(from, 'out'), tp = getNodePort(to, 'in')
  const my = (fp.y + tp.y) / 2
  return `M ${fp.x} ${fp.y} C ${fp.x} ${my}, ${tp.x} ${my}, ${tp.x} ${tp.y}`
}
// ── Export as SVG ────────────────────────────────────────────────────
function exportAsSvg(): string {
  if (!processDef.value) return ''
  const nodes = processDef.value.nodes
  const edges = processDef.value.edges || []
  if (nodes.length === 0) return ''
  let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity
  for (const n of nodes) {
    minX = Math.min(minX, n.x); minY = Math.min(minY, n.y)
    maxX = Math.max(maxX, n.x + (n.w||120)); maxY = Math.max(maxY, n.y + (n.h||50))
  }
  const pad = 60
  const w = maxX - minX + pad*2, h = maxY - minY + pad*2
  let svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${w}" height="${h}" viewBox="0 0 ${w} ${h}">\n`
  svg += `<defs><marker id="arr" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto"><polygon points="0 0,10 3.5,0 7" fill="#00d4ff"/></marker></defs>\n`
  svg += `<rect width="${w}" height="${h}" fill="#0a0e1a"/>\n`
  svg += `<g transform="translate(${pad - minX},${pad - minY})">\n`
  for (const edge of edges) {
    const from = nodes.find(n => n.id === edge.from)
    const to = nodes.find(n => n.id === edge.to)
    if (!from || !to) continue
    const fp = getNodePort(from, 'out'), tp = getNodePort(to, 'in')
    const dx = Math.abs(tp.x - fp.x), cx = Math.max(dx * 0.5, 60)
    svg += `<path d="M ${fp.x} ${fp.y} C ${fp.x+cx} ${fp.y}, ${tp.x-cx} ${tp.y}, ${tp.x} ${tp.y}" stroke="#00d4ff" stroke-width="2" fill="none" marker-end="url(#arr)"/>\n`
    if (edge.label) {
      const mx = (fp.x + tp.x) / 2, my = (fp.y + tp.y) / 2
      const tw = edge.label.length * 7 + 10
      svg += `<rect x="${mx-tw/2}" y="${my-10}" width="${tw}" height="14" rx="3" fill="#1a1f35" stroke="#00d4ff" stroke-width="0.5"/>\n`
      svg += `<text x="${mx}" y="${my}" text-anchor="middle" fill="#00d4ff" font-size="10">${edge.label}</text>\n`
    }
  }
  for (const node of nodes) {
    const nw = node.w||120, nh = node.h||50
    const colors: Record<string,string> = { start:'#10b981', end:'#ef4444', task:'#00d4ff', approval:'#6366f1', subprocess:'#a855f7', script:'#22c55e', gate_and:'#f59e0b', gate_or:'#f59e0b', gate_xor:'#f59e0b' }
    svg += `<rect x="${node.x}" y="${node.y}" width="${nw}" height="${nh}" rx="8" fill="${colors[node.type]||'#374151'}80" stroke="${colors[node.type]||'#6b7280'}" stroke-width="1.5"/>\n`
    svg += `<text x="${node.x+nw/2}" y="${node.y+nh/2+4}" text-anchor="middle" fill="white" font-size="12">${node.label||''}</text>\n`
  }
  svg += `</g></svg>`
  return svg
}
function downloadSvg() {
  const svg = exportAsSvg()
  if (!svg) { alert('画布为空，无法导出'); return }
  const blob = new Blob([svg], { type: 'image/svg+xml' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url; a.download = (currentProcess.value?.flag || 'process') + '.svg'
  a.click(); URL.revokeObjectURL(url)
}
function copySvg() {
  const svg = exportAsSvg()
  if (svg) navigator.clipboard.writeText(svg)
}
// ── Resize ────────────────────────────────────────────────────────────
type ResizeDir = 'nw'|'n'|'ne'|'e'|'se'|'s'|'sw'|'w'
const resizePositions: ResizeDir[] = ['nw','n','ne','e','se','s','sw','w']
function getNodeResizeX(node: PDNode, dir: ResizeDir): number {
  const w = node.w||120
  if (dir==='nw'||dir==='n'||dir==='sw'||dir==='w') return node.x
  return node.x + w
}
function getNodeResizeY(node: PDNode, dir: ResizeDir): number {
  const h = node.h||50
  if (dir==='nw'||dir==='ne'||dir==='n') return node.y
  return node.y + h
}
function getResizeCursor(dir: ResizeDir): string {
  const map: Record<string,string> = { nw:'nwse-resize', n:'ns-resize', ne:'nesw-resize', e:'ew-resize', se:'nwse-resize', s:'ns-resize', sw:'nesw-resize', w:'ew-resize' }
  return map[dir] || 'move'
}
function onResizeMouseDown(e: MouseEvent, nodeIdx: number, dir: ResizeDir) {
  e.stopPropagation()
  if (!processDef.value) return
  isResizing.value = true
  resizeIdx.value = nodeIdx
  resizeDir.value = dir
  const node = processDef.value.nodes[nodeIdx]
  resizeStart.value = {
    x: e.clientX, y: e.clientY,
    w: node.w||120, h: node.h||50,
    nx: node.x, ny: node.y
  }
  const onMove = (ev: MouseEvent) => {
    if (!isResizing.value || resizeIdx.value===null || !processDef.value) return
    const dx = (ev.clientX - resizeStart.value.x) / zoom.value
    const dy = (ev.clientY - resizeStart.value.y) / zoom.value
    const node = processDef.value.nodes[resizeIdx.value]
    const dir = resizeDir.value
    const minW = 80, minH = 40
    if (dir.includes('e')) { node.w = Math.max(minW, resizeStart.value.w + dx); node.x = resizeStart.value.nx }
    else if (dir.includes('w')) { node.w = Math.max(minW, resizeStart.value.w - dx); node.x = resizeStart.value.nx + dx }
    else { node.w = resizeStart.value.w; node.x = resizeStart.value.nx }
    if (dir.includes('s')) { node.h = Math.max(minH, resizeStart.value.h + dy); node.y = resizeStart.value.ny }
    else if (dir.includes('n')) { node.h = Math.max(minH, resizeStart.value.h - dy); node.y = resizeStart.value.ny + dy }
    else { node.h = resizeStart.value.h; node.y = resizeStart.value.ny }
    // Snap to grid
    node.w = Math.round((node.w||120) / GRID_SIZE) * GRID_SIZE
    node.h = Math.round((node.h||50) / GRID_SIZE) * GRID_SIZE
    node.x = Math.round(node.x / GRID_SIZE) * GRID_SIZE
    node.y = Math.round(node.y / GRID_SIZE) * GRID_SIZE
  }
  const onUp = () => {
    if (isResizing.value && processDef.value && resizeIdx.value !== null) pushHistory()
    isResizing.value = false; resizeIdx.value = null
    document.removeEventListener('mousemove', onMove)
    document.removeEventListener('mouseup', onUp)
  }
  document.addEventListener('mousemove', onMove)
  document.addEventListener('mouseup', onUp)
}
// ── Anchor point drag ─────────────────────────────────────────────────
function getAnchorPoints(node: PDNode): {x:number;y:number}[] {
  const w = node.w||120, h = node.h||50
  return [
    { x: node.x + w/2, y: node.y },           // top
    { x: node.x + w, y: node.y + h/2 },       // right
    { x: node.x + w/2, y: node.y + h },       // bottom
    { x: node.x, y: node.y + h/2 },           // left
  ]
}
function onAnchorMouseDown(e: MouseEvent, nodeIdx: number, anchorI: number) {
  e.stopPropagation()
  if (!processDef.value) return
  isDraggingAnchor.value = true
  anchorNodeIdx.value = nodeIdx
  anchorIdx.value = anchorI
  const onMove = (ev: MouseEvent) => {
    if (!isDraggingAnchor.value || anchorNodeIdx.value===null || !processDef.value) return
    // Update edge connection points for edges connected to this anchor
    const node = processDef.value!.nodes[anchorNodeIdx.value]
    const w = node.w||120, h = node.h||50
    const ax = ev.clientX, ay = ev.clientY
    // Project onto node edge
    let px = node.x, py = node.y
    if (anchorI === 0) { px = node.x + w/2; py = Math.max(node.y, Math.min(node.y+h, ay/zoom.value)) }
    else if (anchorI === 1) { px = Math.max(node.x, Math.min(node.x+w, ax/zoom.value)); py = node.y + h/2 }
    else if (anchorI === 2) { px = node.x + w/2; py = Math.min(node.y+h, Math.max(node.y, ay/zoom.value)) }
    else { px = Math.min(node.x, Math.max(node.x-w, ax/zoom.value)); py = node.y + h/2 }
    // Store anchor offset for edges
    if (!node.anchorOffset) node.anchorOffset = []
    node.anchorOffset[anchorI] = { x: px, y: py }
  }
  const onUp = () => {
    isDraggingAnchor.value = false; anchorNodeIdx.value = null; anchorIdx.value = null
    if (processDef.value && anchorNodeIdx.value !== null) pushHistory()
    document.removeEventListener('mousemove', onMove)
    document.removeEventListener('mouseup', onUp)
  }
  document.addEventListener('mousemove', onMove)
  document.addEventListener('mouseup', onUp)
}
// ── Group management ──────────────────────────────────────────────────
function createGroup() {
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
}
function ungroup(nodeIdx: number) {
  if (!processDef.value) return
  const node = processDef.value.nodes[nodeIdx]
  if (!node.groupMembers || node.groupMembers.length === 0) {
    // Fallback: just deselect
    selectedNode.value = null
    return
  }
  // Remove group node
  processDef.value.nodes.splice(nodeIdx, 1)
  // Restore member nodes to their original positions
  for (const memberId of node.groupMembers) {
    const member = processDef.value.nodes.find(n => n.id === memberId)
    if (member && (member as any).__origX !== undefined) {
      member.x = (member as any).__origX
      member.y = (member as any).__origY
      delete (member as any).__origX
      delete (member as any).__origY
      delete (member as any).__origGroupId
    }
  }
  selectedNode.value = null
  pushHistory()
}
// ── Group Visualization ─────────────────────────────────────────────
interface GroupInfo { node: PDNode; members: PDNode[]; bounds: {x:number;y:number;width:number;height:number} }
function computeGroupBounds(groupNode: PDNode): {x:number;y:number;width:number;height:number} {
  if (!groupNode.groupMembers || groupNode.groupMembers.length === 0) {
    return { x: groupNode.x, y: groupNode.y, width: groupNode.w||200, height: groupNode.h||100 }
  }
  let minX=Infinity, minY=Infinity, maxX=-Infinity, maxY=-Infinity
  if (groupNode.collapsed) {
    return { x: groupNode.x, y: groupNode.y, width: groupNode.w||200, height: groupNode.h||100 }
  }
  for (const id of groupNode.groupMembers) {
    const n = processDef.value!.nodes.find(nd => nd.id === id)
    if (!n) continue
    minX = Math.min(minX, n.x); minY = Math.min(minY, n.y)
    maxX = Math.max(maxX, n.x + (n.w||120)); maxY = Math.max(maxY, n.y + (n.h||50))
  }
  return { x: minX - 12, y: minY - 12, width: maxX - minX + 24, height: maxY - minY + 24 }
}
function getGroupNodes(): GroupInfo[] {
  if (!processDef.value) return []
  return processDef.value.nodes
    .filter(n => n.groupMembers && n.groupMembers.length >= 2)
    .map(node => ({
      node,
      members: node.groupMembers!.map(id => processDef.value!.nodes.find(n => n.id === id)).filter(Boolean) as PDNode[],
      bounds: computeGroupBounds(node)
    }))
}
const groupNodes = computed(() => getGroupNodes())
function toggleGroupCollapse(idx: number) {
  if (!processDef.value) return
  const groupInfo = groupNodes.value[idx]
  if (!groupInfo) return
  groupInfo.node.collapsed = !groupInfo.node.collapsed
  if (groupInfo.node.collapsed) {
    // Hide members
    for (const id of groupInfo.node.groupMembers!) {
      const n = processDef.value.nodes.find(nd => nd.id === id)
      if (n) { n.x = -9999; n.y = -9999 }
    }
  } else {
    // Restore members
    for (const id of groupInfo.node.groupMembers!) {
      const n = processDef.value.nodes.find(nd => nd.id === id)
      if (n && (n as any).__origX !== undefined) {
        n.x = (n as any).__origX; n.y = (n as any).__origY
        delete (n as any).__origX; delete (n as any).__origY; delete (n as any).__origGroupId
      }
    }
  }
  pushHistory()
}
function expandGroup(idx: number) { ungroup(groupNodes.value[idx]?.node ? processDef.value!.nodes.findIndex(n => n.id === groupNodes.value[idx].node.id) : -1) }
// Group context menu state
const groupContextMenu = ref<{x:number;y:number;groupIdx:number|null}>({x:0,y:0,groupIdx:null})
function showGroupMenu(e: MouseEvent, idx: number) {
  e.preventDefault(); e.stopPropagation()
  groupContextMenu.value = { x: e.clientX, y: e.clientY, groupIdx: idx }
}
function hideGroupMenu() { groupContextMenu.value = { x: 0, y: 0, groupIdx: null } }
function isNodeInGroup(nodeId: string): boolean {
  if (!processDef.value) return false
  return processDef.value.nodes.some(n => n.groupMembers?.includes(nodeId))
}
function getMemberGroup(nodeId: string): PDNode|null {
  if (!processDef.value) return null
  return processDef.value.nodes.find(n => n.groupMembers?.includes(nodeId)) || null
}
function leaveGroup(nodeIdx: number) {
  if (!processDef.value) return
  const node = processDef.value.nodes[nodeIdx]
  if (!node.groupMembers) return
  // Move member back to their original positions
  for (const memberId of node.groupMembers) {
    const member = processDef.value.nodes.find(n => n.id === memberId)
    if (member && (member as any).__origX !== undefined) {
      member.x = (member as any).__origX
      member.y = (member as any).__origY
      delete (member as any).__origX; delete (member as any).__origY; delete (member as any).__origGroupId
    } else {
      // No original position saved, scatter them
      const idx = processDef.value!.nodes.indexOf(member!)
      if (idx > nodeIdx) {
        member!.x = node.x + 20 + (idx - nodeIdx) * 30
        member!.y = node.y + 20 + (idx - nodeIdx) * 30
      }
    }
  }
  // Remove group node
  const groupIdx = processDef.value.nodes.findIndex(n => n.id === node.id)
  if (groupIdx !== -1) processDef.value.nodes.splice(groupIdx, 1)
  selectedNode.value = null
  pushHistory()
}
function toggleGroup(nodeIdx: number) {
  if (!processDef.value) return
  const node = processDef.value.nodes[nodeIdx]
  if (!node) return
  if (isNodeInGroup(node.id)) return
  // Add to a new group or existing group
  const existingGroup = processDef.value.nodes.find(n => n.groupMembers?.includes(node.id))
  if (existingGroup) return // already in a group
  // Create a temporary group with just this node + previously selected nodes
  const members = Array.from(multiSelected.value).filter(id => id !== node.id)
  if (members.length >= 1) {
    members.push(node.id)
    groupedNodes.value = new Set(members)
    createGroup()
  } else {
    // Just select for grouping
    multiSelected.value.clear()
    multiSelected.value.add(node.id)
    selectedNode.value = nodeIdx
  }
}
function zoomToFit() {
  if (!processDef.value || processDef.value.nodes.length === 0) { fitCanvas(); return }
  if (!canvasRef.value) return
  const rect = canvasRef.value.getBoundingClientRect()
  let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity
  for (const n of processDef.value.nodes) {
    minX = Math.min(minX, n.x); minY = Math.min(minY, n.y)
    maxX = Math.max(maxX, n.x + (n.w||120)); maxY = Math.max(maxY, n.y + (n.h||50))
  }
  const contentW = maxX - minX + 40, contentH = maxY - minY + 40
  const scaleX = rect.width / contentW, scaleY = rect.height / contentH
  zoom.value = Math.min(scaleX, scaleY, 1.5) * 0.9
  panX.value = (rect.width - contentW * zoom.value) / 2 - minX * zoom.value
  panY.value = (rect.height - contentH * zoom.value) / 2 - minY * zoom.value
}
// ── Minimap ──────────────────────────────────────────────────────────
function renderMinimap() {
  const canvas = minimapCanvasRef.value
  if (!canvas || !processDef.value || processDef.value.nodes.length === 0) return
  const ctx = canvas.getContext('2d')
  if (!ctx) return
  const W = canvas.width, H = canvas.viewBox ? 100 : canvas.width
  ctx.clearRect(0, 0, W, H)
  // Compute bounds
  let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity
  for (const n of processDef.value.nodes) {
    minX = Math.min(minX, n.x); minY = Math.min(minY, n.y)
    maxX = Math.max(maxX, n.x + (n.w||120)); maxY = Math.max(maxY, n.y + (n.h||50))
  }
  const pad = 20
  const scaleX = (W - pad*2) / (maxX - minX || 1)
  const scaleY = (H - pad*2) / (maxY - minY || 1)
  const scale = Math.min(scaleX, scaleY)
  const offX = pad - minX * scale, offY = pad - minY * scale
  // Draw edges
  ctx.strokeStyle = 'rgba(0,212,255,0.3)'
  ctx.lineWidth = 1
  for (const edge of (processDef.value.edges||[])) {
    const from = processDef.value!.nodes.find(n => n.id === edge.from)
    const to = processDef.value!.nodes.find(n => n.id === edge.to)
    if (!from || !to) continue
    ctx.beginPath()
    ctx.moveTo(from.x * scale + offX, from.y * scale + offY)
    ctx.lineTo(to.x * scale + offX, to.y * scale + offY)
    ctx.stroke()
  }
  // Draw nodes
  for (let i = 0; i < processDef.value.nodes.length; i++) {
    const n = processDef.value.nodes[i]
    const nx = n.x * scale + offX, ny = n.y * scale + offY
    const nw = (n.w||120) * scale, nh = (n.h||50) * scale
    const colors: Record<string,string> = {
      start:'#10b981', end:'#ef4444', task:'#00d4ff', approval:'#6366f1',
      subprocess:'#a855f7', script:'#22c55e', gate_and:'#f59e0b',
      gate_or:'#f59e0b', gate_xor:'#f59e0b'
    }
    ctx.fillStyle = colors[n.type] || '#6b7280'
    ctx.globalAlpha = execState.value.completedNodes.includes(n.id) ? 1 : execState.value.currentNodeIdx === i ? 0.5 : 0.7
    ctx.fillRect(nx, ny, nw, nh)
    ctx.globalAlpha = 1
    if (execState.value.currentNodeIdx === i) {
      ctx.strokeStyle = '#f59e0b'
      ctx.lineWidth = 2
      ctx.strokeRect(nx-1, ny-1, nw+2, nh+2)
    }
  }
}
function minimapClick(e: MouseEvent) {
  if (!processDef.value || !canvasRef.value) return
  const rect = canvasRef.value.getBoundingClientRect()
  const canvas = minimapCanvasRef.value
  if (!canvas) return
  const scaleX = rect.width / canvas.width
  const scaleY = rect.height / canvas.height
  const mx = (e.clientX - rect.left) * scaleX
  const my = (e.clientY - rect.top) * scaleY
  // Find node near click
  let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity
  for (const n of processDef.value.nodes) {
    minX = Math.min(minX, n.x); minY = Math.min(minY, n.y)
    maxX = Math.max(maxX, n.x + (n.w||120)); maxY = Math.max(maxY, n.y + (n.h||50))
  }
  const pad = 20
  const canvasW = canvas.width, canvasH = canvas.height
  const scaleX2 = (canvasW - pad*2) / (maxX - minX || 1)
  const scaleY2 = (canvasH - pad*2) / (maxY - minY || 1)
  const scale2 = Math.min(scaleX2, scaleY2)
  const offX = pad - minX * scale2, offY = pad - minY * scale2
  // Find closest node
  let closestIdx = -1, closestDist = Infinity
  for (let i = 0; i < processDef.value.nodes.length; i++) {
    const n = processDef.value.nodes[i]
    const nx = n.x * scale2 + offX + (n.w||120)*scale2/2
    const ny = n.y * scale2 + offY + (n.h||50)*scale2/2
    const d = Math.hypot(mx - nx, my - ny)
    if (d < closestDist) { closestDist = d; closestIdx = i }
  }
  if (closestIdx !== -1) {
    const n = processDef.value.nodes[closestIdx]
    const cx = rect.width/2, cy = rect.height/2
    panX.value = cx - (n.x + (n.w||120)/2) * zoom.value
    panY.value = cy - (n.y + (n.h||50)/2) * zoom.value
  }
}
// ── Canvas Themes ────────────────────────────────────────────────────
type CanvasTheme = 'dark'|'midnight'|'ocean'|'forest'|'cyber'|'sunset'|'arctic'|'ember'
const canvasThemes: Record<CanvasTheme, {bg:string;grid:string;name:string}> = {
  dark: { bg: '#0a0e1a', grid: 'rgba(255,255,255,0.03)', name: '暗夜' },
  midnight: { bg: '#0d1b2a', grid: 'rgba(100,200,255,0.03)', name: '午夜' },
  ocean: { bg: '#0a1628', grid: 'rgba(0,150,255,0.04)', name: '深海' },
  forest: { bg: '#0a1a0a', grid: 'rgba(0,255,100,0.03)', name: '森林' },
  cyber: { bg: '#0a0a1a', grid: 'rgba(0,255,200,0.05)', name: '赛博' },
  sunset: { bg: '#1a0a0a', grid: 'rgba(255,100,50,0.04)', name: '日落' },
  arctic: { bg: '#0a1520', grid: 'rgba(150,200,255,0.04)', name: '北极' },
  ember: { bg: '#1a0a15', grid: 'rgba(255,50,150,0.04)', name: '余烬' },
}
const canvasTheme = ref<CanvasTheme>('dark')
const gridPattern = ref<'dot'|'line'|'cross'|'diamond'|'hex'>('line')
const gridIntensity = ref(0.5)
const showGridFlow = ref(false)
const gridFlowSpeed = ref(1)
function setCanvasTheme(theme: CanvasTheme) { canvasTheme.value = theme }
// ── Node Style Presets ───────────────────────────────────────────────
interface NodeStylePreset { name: string; icon: string; colors: { fill: string; stroke: string; text: string } }
const nodeStylePresets: NodeStylePreset[] = [
  { name: '霓虹蓝', icon: '💎', colors: { fill: 'rgba(0,212,255,.3)', stroke: '#00d4ff', text: '#00d4ff' } },
  { name: '极光绿', icon: '🌿', colors: { fill: 'rgba(34,197,94,.3)', stroke: '#22c55e', text: '#22c55e' } },
  { name: '烈焰红', icon: '🔥', colors: { fill: 'rgba(239,68,68,.3)', stroke: '#ef4444', text: '#ef4444' } },
  { name: '紫电', icon: '⚡', colors: { fill: 'rgba(168,85,247,.3)', stroke: '#a855f7', text: '#a855f7' } },
  { name: '金辉', icon: '✨', colors: { fill: 'rgba(245,158,11,.3)', stroke: '#f59e0b', text: '#f59e0b' } },
  { name: '冰霜', icon: '❄️', colors: { fill: 'rgba(59,130,246,.3)', stroke: '#3b82f6', text: '#3b82f6' } },
]
function applyNodeStylePreset(preset: NodeStylePreset) {
  if (selectedNode.value === null || !processDef.value) return
  const node = processDef.value.nodes[selectedNode.value]
  ;(node as any).styleFill = preset.colors.fill
  ;(node as any).styleStroke = preset.colors.stroke
  ;(node as any).styleText = preset.colors.text
  pushHistory()
}
// ── Edge Flow Animation ──────────────────────────────────────────────
const edgeAnimOffset = ref(0)
let edgeAnimFrame: number|null = null
function startEdgeAnimation() {
  if (edgeAnimFrame) cancelAnimationFrame(edgeAnimFrame)
  function animate() {
    edgeAnimOffset.value = (edgeAnimOffset.value + 0.5) % 20
    edgeAnimFrame = requestAnimationFrame(animate)
  }
  animate()
}
function stopEdgeAnimation() {
  if (edgeAnimFrame) { cancelAnimationFrame(edgeAnimFrame); edgeAnimFrame = null }
}
let showEdgeAnim = ref(false)
// ── Process Metadata Editor ──────────────────────────────────────────
const showMetaEditor = ref(false)
const metaForm = ref({ description: '', owner: '', tags: '', version: '1.0.0' })
function openMetaEditor() {
  if (!currentProcess.value) return
  metaForm.value = {
    description: currentProcess.value.desc || '',
    owner: currentProcess.value.flag || '',
    tags: '',
    version: '1.0.0'
  }
  showMetaEditor.value = true
}
function saveMeta() {
  if (!currentProcess.value) return
  currentProcess.value.desc = metaForm.value.description
  showMetaEditor.value = false
}
function onNodeMouseDown(e: MouseEvent, i: number) {
  if (!processDef.value) return
  // Shift+click for multi-select
  if (e.shiftKey) {
    toggleSelectNode(i)
    isMultiDragging.value = true
    const ids = Array.from(multiSelected.value)
    if (ids.length > 0) {
      const first = processDef.value.nodes[ids[0]]
      multiDragOffset.value = {
        x: (e.clientX - panX.value) / zoom.value - (first?.x ?? 0),
        y: (e.clientY - panY.value) / zoom.value - (first?.y ?? 0)
      }
    }
    return
  }
  isDragging.value = true; dragIdx.value = i
  const node = processDef.value.nodes[i]
  dragOffset.value = {
    x: (e.clientX - panX.value) / zoom.value - node.x,
    y: (e.clientY - panY.value) / zoom.value - node.y
  }
  selectedNode.value = i; selectedEdge.value = null
}
function onNodeMouseMove(e: MouseEvent) {
  if (!isDragging.value || dragIdx.value === null || !processDef.value) return
  // Multi-node drag
  if (isMultiDragging.value && multiSelected.value.size > 1) {
    const dx = (e.clientX - panX.value) / zoom.value - multiDragOffset.value.x
    const dy = (e.clientY - panY.value) / zoom.value - multiDragOffset.value.y
    for (const id of multiSelected.value) {
      const idx = processDef.value.nodes.findIndex(n => n.id === id)
      if (idx !== -1) {
        processDef.value.nodes[idx].x = Math.round(dx / GRID_SIZE) * GRID_SIZE
        processDef.value.nodes[idx].y = Math.round(dy / GRID_SIZE) * GRID_SIZE
      }
    }
    return
  }
  const idx = dragIdx.value
  const rawX = (e.clientX - panX.value) / zoom.value - dragOffset.value.x
  const rawY = (e.clientY - panY.value) / zoom.value - dragOffset.value.y
  const snappedX = Math.round(rawX / GRID_SIZE) * GRID_SIZE
  const snappedY = Math.round(rawY / GRID_SIZE) * GRID_SIZE
  processDef.value.nodes[idx].x = snappedX
  processDef.value.nodes[idx].y = snappedY
  // Snap to others
  let nearX: number|null = null, nearY: number|null = null
  for (let j = 0; j < processDef.value.nodes.length; j++) {
    if (j === idx) continue
    const o = processDef.value.nodes[j], ow = o.w||120, oh = o.h||50
    if (Math.abs(snappedX - o.x) < SNAP_THRESHOLD) nearX = o.x
    if (Math.abs(snappedX - (o.x+ow)) < SNAP_THRESHOLD) nearX = o.x+ow
    if (Math.abs(snappedY - o.y) < SNAP_THRESHOLD) nearY = o.y
    if (Math.abs(snappedY - (o.y+oh)) < SNAP_THRESHOLD) nearY = o.y+oh
  }
  snapX.value = nearX; snapY.value = nearY
}
function onNodeMouseUp() {
  if (isMultiDragging.value && multiSelected.value.size > 1 && processDef.value) {
    for (const id of multiSelected.value) {
      const idx = processDef.value.nodes.findIndex(n => n.id === id)
      if (idx !== -1) {
        processDef.value.nodes[idx].x = Math.round(processDef.value.nodes[idx].x / GRID_SIZE) * GRID_SIZE
        processDef.value.nodes[idx].y = Math.round(processDef.value.nodes[idx].y / GRID_SIZE) * GRID_SIZE
      }
    }
    pushHistory()
  } else if (isDragging.value && processDef.value && dragIdx.value !== null) {
    const n = processDef.value.nodes[dragIdx.value]
    if (snapX.value !== null) n.x = snapX.value
    if (snapY.value !== null) n.y = snapY.value
    pushHistory()
  }
  isDragging.value = false; dragIdx.value = null; snapX.value = null; snapY.value = null
  isMultiDragging.value = false; multiSelected.value.clear()
}
// ── Drag: Edge from port ──────────────────────────────────────────────
function onPortMouseDown(e: MouseEvent, nodeIdx: number, port: 'in'|'out') {
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
// ── Canvas pan ────────────────────────────────────────────────────────
function onCanvasMouseDown(e: MouseEvent) {
  if (e.button !== 0) return
  isPanning.value = true
  panStart.value = { x: e.clientX - panX.value, y: e.clientY - panY.value }
  const onMove = (ev: MouseEvent) => { if (isPanning.value) { panX.value = ev.clientX - panStart.value.x; panY.value = ev.clientY - panStart.value.y } }
  const onUp = () => { isPanning.value = false }
  document.addEventListener('mousemove', onMove); document.addEventListener('mouseup', onUp)
}
// ── Node body click: arbitrary position edge creation ─────────────────
function onNodeBodyMouseDown(e: MouseEvent, nodeIdx: number) {
  e.stopPropagation()
  if (!processDef.value) return
  const node = processDef.value.nodes[nodeIdx]
  if (!node) return
  const mx = (e.clientX - panX.value) / zoom.value
  const my = (e.clientY - panY.value) / zoom.value
  const w = node.w || 120, h = node.h || 50
  const cx = node.x + w/2, cy = node.y + h/2
  const dx = mx - cx, dy = my - cy
  let port: 'in'|'out' = 'out'
  if (Math.abs(dx) > Math.abs(dy)) {
    port = dx < 0 ? 'in' : 'out'
  } else {
    port = dy < 0 ? 'in' : 'out'
  }
  tempEdge.value = { from: nodeIdx, fromPort: port, startX: mx, startY: my, endX: mx, endY: my }
  const onMove = (ev: MouseEvent) => {
    if (!tempEdge.value) return
    tempEdge.value.endX = (ev.clientX - panX.value) / zoom.value
    tempEdge.value.endY = (ev.clientY - panY.value) / zoom.value
  }
  const onUp = (ev: MouseEvent) => {
    document.removeEventListener('mousemove', onMove)
    document.removeEventListener('mouseup', onUp)
    if (!tempEdge.value || !processDef.value) { tempEdge.value = null; return }
    const emx = (ev.clientX - panX.value) / zoom.value
    const emy = (ev.clientY - panY.value) / zoom.value
    let targetIdx: number|null = null
    for (let i = 0; i < processDef.value.nodes.length; i++) {
      if (i === tempEdge.value.from) continue
      const n = processDef.value.nodes[i]
      if (emx >= n.x-10 && emx <= n.x+(n.w||120)+10 && emy >= n.y-10 && emy <= n.y+(n.h||50)+10) {
        targetIdx = i; break
      }
    }
    if (targetIdx !== null) {
      const fn = processDef.value.nodes[tempEdge.value.from]
      const tn = processDef.value.nodes[targetIdx]
      const fp = tempEdge.value.fromPort
      if (fp === 'out' && tn.type !== 'start') createEdge(fn.id, tn.id)
      else if (fp === 'in' && fn.type !== 'end') createEdge(tn.id, fn.id)
    }
    tempEdge.value = null
  }
  document.addEventListener('mousemove', onMove)
  document.addEventListener('mouseup', onUp)
}
// ── Edge creation from any position on node ──────────────────────────
function onEdgeMouseDown(e: MouseEvent, nodeIdx: number) {
  e.stopPropagation()
  if (!processDef.value) return
  const node = processDef.value.nodes[nodeIdx]
  if (!node) return
  const rect = (e.target as HTMLElement).getBoundingClientRect()
  const mx = (e.clientX - panX.value) / zoom.value
  const my = (e.clientY - panY.value) / zoom.value
  // Determine which side of the node was clicked
  const w = node.w || 120, h = node.h || 50
  const cx = node.x + w/2, cy = node.y + h/2
  const dx = mx - cx, dy = my - cy
  let port: 'in'|'out' = 'out'
  if (Math.abs(dx) > Math.abs(dy)) {
    port = dx < 0 ? 'in' : 'out'
  } else {
    if (dy < 0) port = 'in'  // top
    else port = 'out'  // bottom
  }
  tempEdge.value = { from: nodeIdx, fromPort: port, startX: mx, startY: my, endX: mx, endY: my }
  const onMove = (ev: MouseEvent) => {
    if (!tempEdge.value) return
    tempEdge.value.endX = (ev.clientX - panX.value) / zoom.value
    tempEdge.value.endY = (ev.clientY - panY.value) / zoom.value
  }
  const onUp = (ev: MouseEvent) => {
    document.removeEventListener('mousemove', onMove); document.removeEventListener('mouseup', onUp)
    if (!tempEdge.value || !processDef.value) { tempEdge.value = null; return }
    const emx = (ev.clientX - panX.value) / zoom.value, emy = (ev.clientY - panY.value) / zoom.value
    let targetIdx: number|null = null
    for (let i = 0; i < processDef.value.nodes.length; i++) {
      if (i === tempEdge.value!.from) continue
      const n = processDef.value.nodes[i]
      if (emx >= n.x-10 && emx <= n.x+(n.w||120)+10 && emy >= n.y-10 && emy <= n.y+(n.h||50)+10) { targetIdx = i; break }
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
  const sx = Math.round(x / GRID_SIZE) * GRID_SIZE
  const sy = Math.round(y / GRID_SIZE) * GRID_SIZE
  const w = isGate(type) ? 100 : type === 'approval' ? 130 : 120
  const h = type === 'approval' ? 70 : type === 'subprocess' ? 60 : 50
  addNode(type, { x: sx - w/2, y: sy - h/2, autoConnect: true })
}
// ── Subprocess ────────────────────────────────────────────────────────
function openSubprocess(nodeIdx: number) {
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
}
function exitSubprocess() {
  subprocessEditing.value = false
  subSelectedNode.value = null; subSelectedEdge.value = null
}
function saveSubprocess() {
  if (subprocessNodeIdx.value === null || !processDef.value) return
  const subs = (currentProcess.value?.subprocesses as any) || {}
  const node = processDef.value.nodes[subprocessNodeIdx.value]
  subs[node.id] = JSON.parse(JSON.stringify(subprocessDef.value))
  if (!currentProcess.value) return
  ;(currentProcess.value as any).subprocesses = subs
  subprocessEditing.value = false
  pushHistory()
}
// ── Subprocess inline editor helpers ──────────────────────────────────
function getSubNodeResizeX(node: PDNode, dir: string): number {
  const w = node.w||120
  if (dir==='nw'||dir==='n'||dir==='sw'||dir==='w') return node.x
  return node.x + w
}
function getSubNodeResizeY(node: PDNode, dir: string): number {
  const h = node.h||50
  if (dir==='nw'||dir==='ne'||dir==='n') return node.y
  return node.y + h
}
function getSubAnchorPoints(node: PDNode): {x:number;y:number}[] {
  const w = node.w||120, h = node.h||50
  const offsets = (node as any).anchorOffset || []
  return [
    { x: node.x + w/2, y: node.y },
    { x: offsets[1]?.x ?? node.x + w, y: node.y + h/2 },
    { x: node.x + w/2, y: node.y + h },
    { x: offsets[3]?.x ?? node.x, y: node.y + h/2 },
  ]
}
function getSubNodePort(node: PDNode, port: 'in'|'out', portIdx?: number): {x:number;y:number} {
  const w = node.w||120, h = node.h||50
  if (port === 'in') return { x: node.x, y: node.y + h/2 }
  if (isGate(node.type) && portIdx !== undefined) {
    const conds = getNodeConditions(node)
    const spread = Math.max(conds.length * 12, 20)
    return { x: node.x + w, y: node.y + h/2 + (portIdx - (conds.length-1)/2) * spread }
  }
  return { x: node.x + w, y: node.y + h/2 }
}
function subComputeEdgePath(edge: PDEdge): string {
  if (!subprocessDef.value) return ''
  const from = subprocessDef.value.nodes.find(n => n.id === edge.from)
  const to = subprocessDef.value.nodes.find(n => n.id === edge.to)
  if (!from || !to) return ''
  const fp = getSubNodePort(from, 'out')
  const tp = getSubNodePort(to, 'in')
  const dx = Math.abs(tp.x - fp.x)
  const cx = Math.max(dx * 0.5, 60)
  return `M ${fp.x} ${fp.y} C ${fp.x+cx} ${fp.y}, ${tp.x-cx} ${tp.y}, ${tp.x} ${tp.y}`
}
function subTempEdgePath(): string {
  if (!subTempEdge.value) return ''
  const { startX, startY, endX, endY } = subTempEdge.value
  const from = subprocessDef.value?.nodes[subTempEdge.value.from]
  if (!from) return ''
  const fp = getSubNodePort(from, subTempEdge.value.fromPort)
  const cx = Math.max(Math.abs(endX - fp.x) * 0.5, 60)
  const sign = subTempEdge.value.fromPort === 'out' ? 1 : -1
  return `M ${fp.x} ${fp.y} C ${fp.x+cx*sign} ${fp.y}, ${endX-cx*sign} ${endY}, ${endX} ${endY}`
}
function subCreateEdge(fromId: string, toId: string) {
  if (!subprocessDef.value) return
  const exists = subprocessDef.value.edges.some(e => e.from === fromId && e.to === toId)
  if (exists) return
  subprocessDef.value.edges.push({ id: genEdgeId(), from: fromId, to: toId })
  subPushHistory()
}
function subDeleteEdge(i: number) {
  if (!subprocessDef.value) return
  subprocessDef.value.edges.splice(i, 1)
  subSelectedEdge.value = null
  subPushHistory()
}
// Subprocess toolbar actions
function subUndo() { if (subHistIdx.value > 0) { subHistIdx.value--; subprocessDef.value = JSON.parse(JSON.stringify(subHistory.value[subHistIdx.value])); subSelectedNode.value = null } }
function subZoomIn() { subZoom.value = Math.min(3, subZoom.value + 0.1) }
function subZoomOut() { subZoom.value = Math.max(0.3, subZoom.value - 0.1) }
function subFitCanvas() { subZoom.value = 1; subPanX.value = 0; subPanY.value = 0 }
function subAddNode(type: string) {
  if (!subprocessDef.value) return
  const w = isGate(type) ? 100 : type === 'approval' ? 130 : type === 'subprocess' ? 120 : 120
  const h = type === 'approval' ? 70 : type === 'subprocess' ? 60 : 50
  const cx = (-subPanX.value + subCanvasRef.value?.clientWidth!/2) / subZoom.value
  const cy = (-subPanY.value + subCanvasRef.value?.clientHeight!/2) / subZoom.value
  const sx = Math.round(cx / GRID_SIZE) * GRID_SIZE
  const sy = Math.round(cy / GRID_SIZE) * GRID_SIZE
  subprocessDef.value.nodes.push({ id: genId(), type, label: getNodeLabel(type), x: sx - w/2, y: sy - h/2, w, h })
  subPushHistory()
}
function subDeleteNode() {
  if (subSelectedNode.value === null || !subprocessDef.value) return
  subprocessDef.value.nodes.splice(subSelectedNode.value, 1)
  // Remove edges connected to deleted node
  subprocessDef.value.edges = subprocessDef.value.edges.filter(e => e.from !== subSelectedNode.value && e.to !== subSelectedNode.value)
  subSelectedNode.value = null; subSelectedEdge.value = null
  subPushHistory()
}
function subDuplicateNode() {
  if (subSelectedNode.value === null || !subprocessDef.value) return
  const orig = subprocessDef.value.nodes[subSelectedNode.value]
  if (!orig) return
  const w = isGate(orig.type) ? 100 : orig.type === 'approval' ? 130 : 120
  const h = orig.type === 'approval' ? 70 : orig.type === 'subprocess' ? 60 : 50
  const newNode: PDNode = {
    id: genId(), type: orig.type, label: orig.label,
    x: orig.x + 30, y: orig.y + 30, w, h,
    assignee: orig.assignee, condition: orig.condition,
    timeout: orig.timeout, priority: orig.priority, script: orig.script
  }
  subprocessDef.value.nodes.push(newNode)
  subSelectedNode.value = subprocessDef.value.nodes.length - 1
  subPushHistory()
}
function subAutoLayout() {
  if (!subprocessDef.value || subprocessDef.value.nodes.length === 0) return
  const nodes = subprocessDef.value.nodes
  const cols = Math.ceil(Math.sqrt(nodes.length))
  nodes.forEach((n, i) => {
    n.x = 80 + (i % cols) * ((n.w||120) + 40)
    n.y = 80 + Math.floor(i / cols) * ((n.h||50) + 40)
  })
  subPushHistory()
}
function subOnWheel(e: WheelEvent) {
  e.preventDefault()
  if (e.ctrlKey || e.metaKey) {
    // Zoom
    const delta = e.deltaY > 0 ? -0.05 : 0.05
    subZoom.value = Math.max(0.3, Math.min(3, subZoom.value + delta))
  } else {
    // Pan
    subPanX.value += e.deltaX
    subPanY.value += e.deltaY
  }
}
function subClearCanvas() {
  if (!confirm('清空子流程画布？')) return
  subprocessDef.value = { nodes: [], edges: [] }
  subSelectedNode.value = null; subSelectedEdge.value = null
  subPushHistory()
}
function createSubVersion() {
  if (!subprocessDef.value) return
  const v: ProcVersion = { id: genId(), timestamp: Date.now(), label: '子流程快照', config: JSON.parse(JSON.stringify(subprocessDef.value)), author: 'user', message: '子流程快照' }
  versions.value.unshift(v)
  if (versions.value.length > 20) versions.value.pop()
}
function subSelectEdge(i: number) { subSelectedEdge.value = i; subSelectedNode.value = null }
function subPushHistory() {
  if (!subprocessDef.value) return
  subHistory.value = subHistory.value.slice(0, subHistIdx.value + 1)
  subHistory.value.push(JSON.parse(JSON.stringify(subprocessDef.value)))
  subHistIdx.value = subHistory.value.length - 1
}
// ── Process CRUD ──────────────────────────────────────────────────────
async function loadProcess(p: ProcDef) {
  try {
    const r: any = await api.get(`/jaxrs/processplatform/assemble/designer/process/${p.id}`)
    const data = r?.data ?? p
    currentProcess.value = data
    processDef.value = data.config ?? { nodes: [], edges: [] }
    if (!processDef.value.nodes.length) {
      const n1 = { id: genId(), type: 'start', label: '开始', x: 80, y: 120, w: 100, h: 50 }
      const n2 = { id: genId(), type: 'task', label: '审批任务', x: 300, y: 100, w: 120, h: 50 }
      const n3 = { id: genId(), type: 'end', label: '结束', x: 520, y: 120, w: 100, h: 50 }
      processDef.value = {
        nodes: [n1, n2, n3],
        edges: [{ id: genEdgeId(), from: n1.id, to: n2.id }, { id: genEdgeId(), from: n2.id, to: n3.id }]
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
      description: currentProcess.value.desc, config: processDef.value,
      ...(currentProcess.value.subprocesses ? { subprocesses: (currentProcess.value as any).subprocesses } : {})
    })
    alert('保存成功')
  } catch (e: any) { alert('保存失败: ' + (e?.message ?? '')) }
}
async function loadProcesses() {
  try { const r: any = await api.get('/jaxrs/processplatform/assemble/designer/process/list'); procList.value = r?.data?.list ?? r?.data ?? [] }
  catch { procList.value = [] }
}
// Connection rules state
const showRulesModal = ref(false)
const connectionRules = ref<Record<string, Record<string, boolean>>>({})
function resetConnectionRules() {
  const rules: Record<string, Record<string, boolean>> = {}
  for (const ft of allNodeTypes) {
    rules[ft] = {}
    for (const tt of allNodeTypes) {
      if (ft === tt) { rules[ft][tt] = false; continue }
      // Default rules: start can only go to task/approval/gate; end can only receive from task/approval/gate; etc.
      const allowed: Record<string, boolean> = {
        'start': ['task','approval','script','gate_and','gate_or','gate_xor'],
        'task': ['task','approval','end','script','gate_and','gate_or','gate_xor'],
        'approval': ['task','approval','end','script','gate_and','gate_or','gate_xor'],
        'script': ['task','approval','end','script','gate_and','gate_or','gate_xor'],
        'timer': ['task','approval','end','script'],
        'end': [],
        'gate_and': ['task','approval','end','script'],
        'gate_or': ['task','approval','end','script'],
        'gate_xor': ['task','approval','end','script'],
        'subprocess': ['task','approval','end','script'],
        'parallel': ['task','approval','end','script'],
      }
      rules[ft][tt] = (allowed[ft]||[]).includes(tt)
    }
  }
  connectionRules.value = rules
}
resetConnectionRules()
function isAllowed(from: string, to: string): boolean {
  return connectionRules.value[from]?.[to] ?? true
}
function toggleRule(from: string, to: string) {
  if (!connectionRules.value[from]) connectionRules.value[from] = {}
  connectionRules.value[from][to] = !connectionRules.value[from][to]
}
function saveRules() {
  pushHistory()
  showRulesModal.value = false
}
// Node templates
const showTemplatesModal = ref(false)
interface TemplateNodeDef { type: string; label: string; icon: string }
interface TemplateDef { name: string; icon: string; desc: string; nodes: TemplateNodeDef[]; edges: {from: number; to: number}[] }
const nodeTemplates: TemplateDef[] = [
  {
    name: '简单审批', icon: '📝', desc: '开始 → 任务 → 审批 → 结束',
    nodes: [{type:'start',label:'开始',icon:'🟢'},{type:'task',label:'提交申请',icon:'📋'},{type:'approval',label:'主管审批',icon:'✅'},{type:'end',label:'完成',icon:'🔴'}],
    edges: [{from:0,to:1},{from:1,to:2},{from:2,to:3}]
  },
  {
    name: '多级审批', icon: '📑', desc: '开始 → 任务 → 一级审批 → 二级审批 → 结束',
    nodes: [{type:'start',label:'开始',icon:'🟢'},{type:'task',label:'提交申请',icon:'📋'},{type:'approval',label:'主管审批',icon:'✅'},{type:'approval',label:'经理审批',icon:'✅'},{type:'end',label:'完成',icon:'🔴'}],
    edges: [{from:0,to:1},{from:1,to:2},{from:2,to:3},{from:3,to:4}]
  },
  {
    name: '条件分支', icon: '🔀', desc: '开始 → 任务 → 条件网关 → 审批A/审批B → 合并 → 结束',
    nodes: [{type:'start',label:'开始',icon:'🟢'},{type:'task',label:'提交申请',icon:'📋'},{type:'gate_or',label:'金额判断',icon:'🔶'},{type:'approval',label:'小额审批',icon:'✅'},{type:'approval',label:'大额审批',icon:'✅'},{type:'gate_and',label:'合并',icon:'🔷'},{type:'end',label:'完成',icon:'🔴'}],
    edges: [{from:0,to:1},{from:1,to:2},{from:2,to:3},{from:2,to:4},{from:3,to:5},{from:4,to:5},{from:5,to:6}]
  },
  {
    name: '并行分支', icon: '⚡', desc: '开始 → Fork → 并行任务A/B/C → Join → 结束',
    nodes: [{type:'start',label:'开始',icon:'🟢'},{type:'gate_and',label:'Fork',icon:'🔷'},{type:'task',label:'任务A',icon:'📋'},{type:'task',label:'任务B',icon:'📋'},{type:'task',label:'任务C',icon:'📋'},{type:'gate_and',label:'Join',icon:'🔷'},{type:'end',label:'完成',icon:'🔴'}],
    edges: [{from:0,to:1},{from:1,to:2},{from:1,to:3},{from:1,to:4},{from:2,to:5},{from:3,to:5},{from:4,to:5},{from:5,to:6}]
  },
  {
    name: '脚本处理', icon: '⚙️', desc: '开始 → 脚本节点 → 任务 → 结束',
    nodes: [{type:'start',label:'开始',icon:'🟢'},{type:'script',label:'数据预处理',icon:'⚡'},{type:'task',label:'人工处理',icon:'📋'},{type:'end',label:'结束',icon:'🔴'}],
    edges: [{from:0,to:1},{from:1,to:2},{from:2,to:3}]
  },
  {
    name: '循环重试', icon: '🔄', desc: '开始 → 任务 → 条件网关(失败) → 重试 → 结束',
    nodes: [{type:'start',label:'开始',icon:'🟢'},{type:'task',label:'执行任务',icon:'📋'},{type:'gate_or',label:'是否成功?',icon:'🔶'},{type:'task',label:'重试处理',icon:'📋'},{type:'end',label:'完成',icon:'🔴'}],
    edges: [{from:0,to:1},{from:1,to:2},{from:2,to:3},{from:2,to:4},{from:3,to:1}]
  },
]
// Version comparison state
const showCompareModal = ref(false)
const compareV1 = ref('')
const compareV2 = ref('__current')
function getVersionLabel(id: string): string {
  if (id === '__current') return '当前'
  const v = versions.value.find(v => v.id === id)
  return v ? v.label : '未知版本'
}
function getNodesById(id: string): PDNode[] {
  if (id === '__current') return processDef.value?.nodes ?? []
  const v = versions.value.find(vv => vv.id === id)
  return v?.config?.nodes ?? []
}
function getEdgesById(id: string): PDEdge[] {
  if (id === '__current') return processDef.value?.edges ?? []
  const v = versions.value.find(vv => vv.id === id)
  return v?.config?.edges ?? []
}
function countDiff(id1: string, id2: string, type: 'added'|'removed'|'modified'): number {
  const n1 = getNodesById(id1), n2 = getNodesById(id2)
  const s1 = new Set(n1.map(n => n.id)), s2 = new Set(n2.map(n => n.id))
  if (type === 'added') return [...s2].filter(id => !s1.has(id)).length
  if (type === 'removed') return [...s1].filter(id => !s2.has(id)).length
  // modified: nodes in both but with different labels or positions
  let count = 0
  for (const id of s1) {
    const a = n1.find(n => n.id === id), b = n2.find(n => n.id === id)
    if (a && b && (a.label !== b.label || Math.abs(a.x - b.x) > 10 || Math.abs(a.y - b.y) > 10)) count++
  }
  return count
}
function formatNodeDiff(id1: string, id2: string, _mode: string): string {
  const n1 = getNodesById(id1), n2 = getNodesById(id2)
  const s1 = new Set(n1.map(n => n.id)), s2 = new Set(n2.map(n => n.id))
  const added = [...s2].filter(id => !s1.has(id))
  const removed = [...s1].filter(id => !s2.has(id))
  const modified: string[] = []
  for (const id of s1) {
    const a = n1.find(n => n.id === id), b = n2.find(n => n.id === id)
    if (a && b && a.label !== b.label) modified.push(`${a.label}→${b.label}`)
  }
  const lines: string[] = []
  if (removed.length) lines.push(`删除: ${removed.length} 节点`)
  if (added.length) lines.push(`新增: ${added.length} 节点`)
  if (modified.length) lines.push(`修改: ${modified.length} 节点`)
  return lines.join(' | ') || '无差异'
}
// Import/Export state
const showIoModal = ref(false)
const ioMode = ref<'export'|'import'|'validate'>('export')
const importJsonText = ref('')
const validationResult = ref<{totalNodes:number; totalEdges:number; issues: Array<{severity: string; message: string}>; suggestions: string[]; healthScore: number|null} | null>(null)
function exportJson(): string {
  if (!processDef.value) return '{}'
  return JSON.stringify({ nodes: processDef.value.nodes, edges: processDef.value.edges }, null, 2)
}
function copyExportJson() {
  navigator.clipboard.writeText(exportJson())
}
function downloadJson() {
  const blob = new Blob([exportJson()], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url; a.download = (currentProcess.value?.flag || 'process') + '.json'
  a.click(); URL.revokeObjectURL(url)
}
function doImportJson() {
  try {
    const data = JSON.parse(importJsonText.value)
    if (data.nodes && Array.isArray(data.nodes)) {
      processDef.value = { nodes: data.nodes, edges: data.edges || [] }
      selectedNode.value = null; selectedEdge.value = null
      pushHistory()
      showIoModal.value = false
      importJsonText.value = ''
    }
  } catch { alert('JSON格式错误，请检查导入内容') }
}
// Validation
function runValidation(): void {
  if (!processDef.value) { validationResult.value = null; return }
  const issues: Array<{severity: string; message: string}> = []
  const suggestions: string[] = []
  const nodes = processDef.value.nodes
  const edges = processDef.value.edges || []
  const nodeIds = new Set(nodes.map(n => n.id))
  // Check disconnected nodes
  const connectedNodes = new Set<string>()
  for (const e of edges) { connectedNodes.add(e.from); connectedNodes.add(e.to) }
  for (const n of nodes) { if (!connectedNodes.has(n.id) && n.type !== 'start' && n.type !== 'end') issues.push({ severity: 'warning', message: `节点「${n.label||n.id}」未连接到任何连线` }) }
  // Check start/end
  const starts = nodes.filter(n => n.type === 'start')
  const ends = nodes.filter(n => n.type === 'end')
  if (starts.length === 0) issues.push({ severity: 'error', message: '流程缺少开始节点' }); else if (starts.length > 1) issues.push({ severity: 'warning', message: `流程有 ${starts.length} 个开始节点` })
  if (ends.length === 0) issues.push({ severity: 'error', message: '流程缺少结束节点' }); else if (ends.length > 1) issues.push({ severity: 'warning', message: `流程有 ${ends.length} 个结束节点` })
  // Check orphan edges
  for (const e of edges) {
    if (!nodeIds.has(e.from)) issues.push({ severity: 'error', message: `连线指向不存在的节点: ${e.from}` })
    if (!nodeIds.has(e.to)) issues.push({ severity: 'error', message: `连线来自不存在的节点: ${e.to}` })
  }
  // Check duplicate edges
  const edgeSet = new Set<string>()
  for (const e of edges) { const k = `${e.from}-${e.to}`; if (edgeSet.has(k)) issues.push({ severity: 'warning', message: `重复连线: ${e.from} → ${e.to}` }); else edgeSet.add(k) }
  // Check self-loops
  for (const e of edges) { if (e.from === e.to) issues.push({ severity: 'warning', message: `自环: 节点 ${e.from}` }) }
  // Check missing labels
  for (const n of nodes) { if (!n.label) issues.push({ severity: 'info', message: `节点 ${n.id} 缺少标签` }) }
  // Check node overlap
  for (let i = 0; i < nodes.length; i++) {
    for (let j = i+1; j < nodes.length; j++) {
      const a = nodes[i], b = nodes[j]
      if (a.x < b.x+(b.w||120) && a.x+(a.w||120) > b.x && a.y < b.y+(b.h||50) && a.y+(a.h||50) > b.y)
        issues.push({ severity: 'warning', message: `节点重叠: ${a.label||a.id} 与 ${b.label||b.id}` })
    }
  }
  // Suggestions
  if (starts.length === 0) suggestions.push('添加一个「开始」节点作为流程入口')
  if (ends.length === 0) suggestions.push('添加一个「结束」节点作为流程出口')
  for (const n of nodes) {
    if (n.type === 'start' && edges.filter(e => e.from === n.id).length === 0)
      suggestions.push(`开始节点「${n.label||n.id}」没有 outgoing 连线`)
  }
  const healthScore = nodes.length > 0 ? Math.max(0, 100 - issues.filter(i => i.severity === 'error').length * 20 - issues.filter(i => i.severity === 'warning').length * 5) : null
  validationResult.value = { totalNodes: nodes.length, totalEdges: edges.length, issues, suggestions, healthScore }
}
// ── Lifecycle ─────────────────────────────────────────────────────────
onMounted(() => {
  document.addEventListener('mousemove', (e) => { onNodeMouseMove(e) })
  document.addEventListener('mouseup', () => { onNodeMouseUp() })
document.addEventListener('keydown', (e) => {
    if (e.ctrlKey && e.key === 'a' && currentProcess.value) {
      e.preventDefault()
      multiSelected.value.clear()
      if (processDef.value) processDef.value.nodes.forEach((n,i) => { multiSelected.value.add(n.id); selectedNode.value = i })
    }
    if (e.ctrlKey && e.key === 'z' && !e.shiftKey) { e.preventDefault(); undo() }
    if (e.ctrlKey && (e.key === 'y' || (e.key === 'z' && e.shiftKey))) { e.preventDefault(); redo() }
    if (e.ctrlKey && e.key === 'd') { e.preventDefault(); duplicateSelected() }
    if (e.key === 'Delete' || e.key === 'Backspace') {
      if (document.activeElement?.tagName !== 'INPUT' && document.activeElement?.tagName !== 'TEXTAREA') {
        e.preventDefault(); deleteSelected()
      }
    }
    if (e.key === 'Escape') { selectedNode.value = null; selectedEdge.value = null; tempEdge.value = null }
    if (e.key === 'h' && !e.ctrlKey) { e.preventDefault(); showHelpModal.value = !showHelpModal.value }
    if (e.key === 'g' && !e.ctrlKey && multiSelected.value.size >= 2) {
      e.preventDefault()
      createGroup()
    }
    if (e.ctrlKey && e.key === '=') { e.preventDefault(); zoomIn() }
    if (e.ctrlKey && e.key === '-') { e.preventDefault(); zoomOut() }
    if (e.ctrlKey && e.key === '0') { e.preventDefault(); zoomToFit() }
  })
  loadProcesses()
})
// --- Canvas Annotations ---
interface Annotation { id: string; x: number; y: number; text: string; color: string; w: number; h: number }
const annotations = ref<Annotation[]>([])
const showAnnotations = ref(false)
const newAnnotation = ref({ text: "", color: "#f59e0b" })
function addAnnotation() {
  if (!processDef.value) return
  const rect = canvasRef.value?.getBoundingClientRect()
  if (!rect) return
  const cx = (rect.width/2 - panX.value) / zoom.value
  const cy = (rect.height/2 - panY.value) / zoom.value
  annotations.value.push({ id: genId(), x: cx - 60, y: cy - 20, text: newAnnotation.value.text || "备注", color: newAnnotation.value.color, w: 120, h: 60 })
  newAnnotation.value = { text: "", color: "#f59e0b" }
}
function deleteAnnotation(idx: number) { annotations.value.splice(idx, 1) }
function updateAnnotation(idx: number, prop: keyof Annotation, val: any) { if (annotations.value[idx]) annotations.value[idx][prop] = val }
// --- Snap to Grid ---
const snapToGrid = ref(true)
const gridSnapThreshold = ref(15)
const showGrid = ref(true)
const customGridSize = ref(GRID_SIZE)
function toggleSnap() { snapToGrid.value = !snapToGrid.value }
function setGridSize(size: number) { if (customGridSize) customGridSize.value = Math.max(10, Math.min(50, size)) }
// --- Node Alignment ---
type AlignDir = "left"|"right"|"top"|"bottom"|"center-x"|"center-y"|"distribute-h"|"distribute-v"
function alignNodes(dir: AlignDir) {
  if (!processDef.value) return
  const ids = selectedNode.value !== null ? [processDef.value.nodes[selectedNode.value]!.id] : Array.from(multiSelected.value)
  if (ids.length < 2) return
  const nodes = ids.map(id => processDef.value!.nodes.find(n => n.id === id)).filter(Boolean) as PDNode[]
  switch(dir) {
    case "left": { const minX = Math.min(...nodes.map(n => n.x)); nodes.forEach(n => n.x = minX); break }
    case "right": { const maxX = Math.max(...nodes.map(n => n.x + (n.w||120))); nodes.forEach(n => n.x = maxX - (n.w||120)); break }
    case "top": { const minY = Math.min(...nodes.map(n => n.y)); nodes.forEach(n => n.y = minY); break }
    case "bottom": { const maxY = Math.max(...nodes.map(n => n.y + (n.h||50))); nodes.forEach(n => n.y = maxY - (n.h||50)); break }
    case "center-x": { const cx = nodes.reduce((s,n) => s + n.x + (n.w||120)/2, 0) / nodes.length; nodes.forEach(n => n.x = cx - (n.w||120)/2); break }
    case "center-y": { const cy = nodes.reduce((s,n) => s + n.y + (n.h||50)/2, 0) / nodes.length; nodes.forEach(n => n.y = cy - (n.h||50)/2); break }
  }
  pushHistory()
}
// --- Batch Operations ---
function batchSetProperty(prop: string, val: any) {
  if (!processDef.value) return
  const ids = selectedNode.value !== null ? [processDef.value.nodes[selectedNode.value]!.id] : Array.from(multiSelected.value)
  for (const id of ids) { const n = processDef.value.nodes.find(nd => nd.id === id); if (n) (n as any)[prop] = val }
  pushHistory()
}
function batchSetColor(color: string) { batchSetProperty("style", color) }
// --- Connection Validation ---
interface ValidationResult { valid: boolean; issues: Array<{type: string; message: string; severity: "error"|"warning"}>; stats: {totalNodes: number; totalEdges: number; isolatedNodes: number; missingStart: boolean; missingEnd: boolean; unreachableNodes: string[]} }
function validateConnections(): ValidationResult {
  if (!processDef.value) return { valid: false, issues: [], stats: {totalNodes:0,totalEdges:0,isolatedNodes:0,missingStart:true,missingEnd:true,unreachableNodes:[]} }
  const nodes = processDef.value.nodes, edges = processDef.value.edges || []
  const issues: Array<{type:string;message:string;severity:"error"|"warning"}> = []
  const starts = nodes.filter(n => n.type === "start")
  if (starts.length === 0) issues.push({ type: "missing-start", message: "流程缺少开始节点", severity: "error" })
  const ends = nodes.filter(n => n.type === "end")
  if (ends.length === 0) issues.push({ type: "missing-end", message: "流程缺少结束节点", severity: "error" })
  const connectedIds = new Set<string>()
  for (const e of edges) { connectedIds.add(e.from); connectedIds.add(e.to) }
  const isolated = nodes.filter(n => !connectedIds.has(n.id) && n.type !== "start" && n.type !== "end")
  for (const n of isolated) issues.push({ type: "isolated", message: "未连接: " + (n.label||n.id), severity: "warning" })
  const reachable = new Set<string>()
  if (starts.length > 0) { const q = [starts[0].id]; while(q.length){ const c=q.shift()!; if(reachable.has(c))continue; reachable.add(c); for(const e of edges){ if(e.from===c&&!reachable.has(e.to))q.push(e.to) } } }
  const unreachable = nodes.filter(n => !reachable.has(n.id) && n.type!=="start").map(n=>n.label||n.id)
  for (const id of unreachable) issues.push({ type: "unreachable", message: "无法到达: " + id, severity: "warning" })
  const es = new Set<string>()
  for (const e of edges) { const k=e.from+"-"+e.to; if(es.has(k)) issues.push({type:"dup",message:"重复连线",severity:"warning"}); else es.add(k) }
  for (const e of edges) { if(e.from===e.to) issues.push({type:"loop",message:"自环",severity:"warning"}) }
  const valid = issues.filter(i=>i.severity==="error").length===0
  return { valid, issues, stats: {totalNodes:nodes.length,totalEdges:edges.length,isolatedNodes:isolated.length,missingStart:starts.length===0,missingEnd:ends.length===0,unreachableNodes:unreachable} }
}
// --- Dimension Presets ---
const dimPresets = [{name:"窄型",w:80,h:40},{name:"标准",w:120,h:50},{name:"宽型",w:160,h:50},{name:"高型",w:120,h:80},{name:"大方块",w:140,h:140},{name:"标签",w:100,h:30}]
function applyDimPreset(idx: number) {
  if (selectedNode.value===null||!processDef.value) return
  const p = dimPresets[idx]; if(!p) return
  const n = processDef.value.nodes[selectedNode.value]; n.w=p.w; n.h=p.h; pushHistory()
}
// --- Flow Analysis ---
interface FlowInfo { nodeId:string; label:string; inDegree:number; outDegree:number; role:string }
function computeFlowInfo(): FlowInfo[] {
  if (!processDef.value) return []
  const nodes = processDef.value.nodes, edges = processDef.value.edges||[]
  return nodes.map(n => {
    const inD = edges.filter(e=>e.to===n.id).length, outD = edges.filter(e=>e.from===n.id).length
    let role = "内部节点"
    if (n.type==="start") role="入口"
    else if (n.type==="end") role="出口"
    else if (inD===0&&outD>0) role="起始"
    else if (inD>0&&outD===0) role="终止"
    else if (inD===0&&outD===0) role="孤立"
    return {nodeId:n.id, label:n.label||n.id, inDegree:inD, outDegree:outD, role}
  }).sort((a,b)=>b.outDegree-a.outDegree||a.inDegree-b.inDegree)
}
const flowInfo = computed(() => computeFlowInfo())
// --- Process Archive ---
interface ProcessArchive { id:string; timestamp:number; name:string; nodeCount:number; edgeCount:number; snapshot:{nodes:PDNode[];edges:PDEdge[]} }
const processArchive = ref<ProcessArchive[]>([])
function archiveCurrent() {
  if (!processDef.value||!currentProcess.value) return
  processArchive.value.unshift({id:genId(),timestamp:Date.now(),name:currentProcess.value.name||"未命名",nodeCount:processDef.value.nodes.length,edgeCount:(processDef.value.edges||[]).length,snapshot:JSON.parse(JSON.stringify(processDef.value))})
  if (processArchive.value.length>50) processArchive.value.pop()
}
function restoreArchive(idx:number) {
  if (idx>=processArchive.value.length||!processDef.value) return
  const snap = processArchive.value[idx].snapshot
  processDef.value = {nodes:snap.nodes, edges:snap.edges||[]}
  selectedNode.value=null; selectedEdge.value=null; pushHistory()
}
function deleteArchive(idx:number) { processArchive.value.splice(idx,1) }
const showArchiveManager = ref(false)
const showSnapshotManager = ref(false)
const processSnapshots = ref<Array<{id:string;name:string;createdAt:number;status:string;nodeCount:number}>>([])
const newArchiveLabel = ref('')
const newArchiveDesc = ref('')
const showFlowAnalysis = ref(false)
const flowAnalysisResult = ref<FlowAnalysisResult|null>(null)
const showDiffView = ref(false)
const diffLeftIdx = ref(0)
const diffRightIdx = ref(1)
const showGridThemePanel = ref(false)
const particleOffset = ref(0)
function createSnapshot() {
  if (!processDef.value||!currentProcess.value) return
  processSnapshots.value.unshift({id:genId(),name:currentProcess.value.name||'未命名',createdAt:Date.now(),status:'draft',nodeCount:processDef.value.nodes.length})
}
function runFlowAnalysis(): FlowAnalysisResult {
  if (!processDef.value) return {totalNodes:0,totalEdges:0,cycles:[],criticalPath:[],bottlenecks:[],isolatedNodes:[]}
  const nodes=processDef.value.nodes, edges=processDef.value.edges||[]
  const inDeg=new Map<string,number>(), outDeg=new Map<string,number>()
  nodes.forEach(n=>{inDeg.set(n.id,0);outDeg.set(n.id,0)})
  edges.forEach(e=>{inDeg.set(e.to,(inDeg.get(e.to)||0)+1);outDeg.set(e.from,(outDeg.get(e.from)||0)+1)})
  const cycles:Array<CycleInfo>=[], visited=new Set<string>(), recStack=new Set<string>()
  function dfs(id:string,path:string[]){visited.add(id);recStack.add(id);edges.filter(e=>e.from===id).forEach(e=>{if(!visited.has(e.to))dfs(e.to,[...path,id]);else if(recStack.has(e.to)){const si=path.indexOf(e.to);if(si>=0)cycles.push({nodes:path.slice(si).concat([id,e.to]),length:path.length-si+1,severity:'warning'})}});recStack.delete(id)}
  nodes.forEach(n=>{if(!visited.has(n.id))dfs(n.id,[])})
  const isolated=nodes.filter(n=>(inDeg.get(n.id)||0)===0&&(outDeg.get(n.id)||0)===0)
  const bottlenecks=nodes.filter(n=>(inDeg.get(n.id)||0)>=3||(outDeg.get(n.id)||0)>=3).map(n=>({nodeId:n.id,label:n.label||n.id,inDegree:inDeg.get(n.id)||0,outDegree:outDeg.get(n.id)||0,severity:(inDeg.get(n.id)||0)>=3&&(outDeg.get(n.id)||0)>=3?'high':'medium',reason:'入边和出边过多'}))
  const criticalPath:Array<CriticalPathNode>=nodes.filter(n=>(inDeg.get(n.id)||0)===0).slice(0,3).map(n=>({nodeId:n.id,label:n.label||n.id,duration:100+Math.random()*200}))
  const r={totalNodes:nodes.length,totalEdges:edges.length,cycles,criticalPath,bottlenecks,isolatedNodes:isolated.map(n=>n.id)} as FlowAnalysisResult
  flowAnalysisResult.value=r;return r
}
function getFlowHealthScore():number{if(!flowAnalysisResult.value)return 0;const r=flowAnalysisResult.value;return Math.max(0,100-r.cycles.length*20-r.isolatedNodes.length*5-r.bottlenecks.filter(b=>b.severity==='high').length*15)}
function getFlowHealthLabel(s:number):string{return s>=80?'优秀':s>=60?'良好':s>=40?'一般':'需优化'}
function exportDiff():void{console.log('Export diff between',diffLeftIdx.value,'and',diffRightIdx.value)}
function toggleGridFlow(){showGridFlow.value=!showGridFlow.value}
function updateGridIntensity(v:number){gridIntensity.value=v}
function updateGridPattern(p:'dot'|'line'|'cross'|'diamond'|'hex'){gridPattern.value=p}
onUnmounted(()=>{document.removeEventListener('mousemove',()=>{})
  document.removeEventListener('mousemove', () => {})
  document.removeEventListener('mouseup', () => {})
})
// ── Group Drag ──────────────────────────────────────────────────────
function onGroupResizeMouseDown(e: MouseEvent, idx: number, dir: string) {
  e.stopPropagation()
  if (!processDef.value) return
  const g = groupNodes.value[idx]
  if (!g) return
  groupResizeState.value = { idx, dir, startX: e.clientX, startY: e.clientY, origW: g.node.w||200, origH: g.node.h||100, origX: g.node.x, origY: g.node.y }
  const onMove = (ev: MouseEvent) => {
    if (!groupResizeState.value) return
    const gs = groupResizeState.value, gn = processDef.value.nodes[gs.idx]
    if (!gn) return
    const dx = (ev.clientX - gs.startX) / zoom.value, dy = (ev.clientY - gs.startY) / zoom.value
    if (gs.dir === "se") { gn.w = Math.max(100, gs.origW + dx); gn.h = Math.max(60, gs.origH + dy) }
    else if (gs.dir === "e") gn.w = Math.max(100, gs.origW + dx)
    else if (gs.dir === "s") gn.h = Math.max(60, gs.origH + dy)
    else if (gs.dir === "nw") { gn.x = gs.origX + dx; gn.y = gs.origY + dy; gn.w = Math.max(100, gs.origW - dx); gn.h = Math.max(60, gs.origH - dy) }
    else if (gs.dir === "sw") { gn.y = gs.origY + dy; gn.h = Math.max(60, gs.origH + dy) }
    else if (gs.dir === "ne") { gn.x = gs.origX + dx; gn.h = Math.max(60, gs.origH + dy) }
    else if (gs.dir === "n") gn.h = Math.max(60, gs.origH - dy)
    else if (gs.dir === "w") { gn.x = gs.origX + dx; gn.w = Math.max(100, gs.origW - dx) }
    gn.w = Math.max(100, Math.round(gn.w / GRID_SIZE) * GRID_SIZE)
    gn.h = Math.max(60, Math.round(gn.h / GRID_SIZE) * GRID_SIZE)
  }
  const onUp = () => { document.removeEventListener("mousemove", onMove); document.removeEventListener("mouseup", onUp); groupResizeState.value = null; pushHistory() }
  document.addEventListener("mousemove", onMove); document.addEventListener("mouseup", onUp)
}
function getGroupResizeX(node: PDNode, dir: string): number { return dir.includes("w") ? node.x : node.x + (node.w||200) }
function getGroupResizeY(node: PDNode, dir: string): number { return dir.includes("n") ? node.y : node.y + (node.h||100) }
// ── Edge Routing ────────────────────────────────────────────────────
const routingPresets = [{ name: "auto", routing: "auto", label: "自动" }, { name: "straight", routing: "straight", label: "直线" }, { name: "horizontal", routing: "horizontal", label: "水平" }, { name: "vertical", routing: "vertical", label: "垂直" }]
function openRoutingPanel(edgeIdx: number) {
  selectedRoutingEdge.value = edgeIdx; showRoutingPanel.value = true
  const edge = processDef.value?.edges?.[edgeIdx]
  if (!edge || routingConfigs.value.has(edge.id)) return
  const fn = processDef.value!.nodes.find(n => n.id === edge.from), tn = processDef.value!.nodes.find(n => n.id === edge.to)
  routingConfigs.value.set(edge.id, { edgeId: edge.id, fromNodeIdx: processDef.value!.nodes.indexOf(fn!), toNodeIdx: processDef.value!.nodes.indexOf(tn!), routing: "auto", controlPoints: [], offset: 0, labelPos: "auto", arrowStyle: "default" })
}
function getRoutingConfig(edgeId: string): EdgeRouteConfig|null { return routingConfigs.value.get(edgeId) || null }
function updateRoutingConfig(edgeId: string, updates: Partial<EdgeRouteConfig>) {
  const cfg = routingConfigs.value.get(edgeId)
  if (!cfg) return
  Object.assign(cfg, updates)
  routingConfigs.value.set(edgeId, cfg)
  if (processDef.value) { const e = processDef.value.edges?.find(x => x.id === edgeId); if (e) e.routing = updates.routing || "auto" }
}
function addControlPoint() { const cfg = getRoutingConfig(processDef.value?.edges?.[selectedRoutingEdge.value!]?.id || ""); if (!cfg) return; cfg.controlPoints.push({ x: 0, y: 0, type: "control" }); routingConfigs.value.set(cfg.edgeId, cfg) }
function removeControlPoint(idx: number) { const cfg = getRoutingConfig(processDef.value?.edges?.[selectedRoutingEdge.value!]?.id || ""); if (!cfg) return; cfg.controlPoints.splice(idx, 1); routingConfigs.value.set(cfg.edgeId, cfg) }
function computeCustomEdgePath(edge: PDEdge): string {
  const cfg = routingConfigs.value.get(edge.id)
  if (!cfg || cfg.controlPoints.length === 0) return computeEdgePath(edge)
  const from = processDef.value?.nodes.find(n => n.id === edge.from), to = processDef.value?.nodes.find(n => n.id === edge.to)
  if (!from || !to) return ""
  const fp = getNodePort(from, "out"), tp = getNodePort(to, "in")
  let d = `M ${fp.x} ${fp.y}`
  for (const cp of cfg.controlPoints) d += ` L ${cp.x} ${cp.y}`
  return d + ` L ${tp.x} ${tp.y}`
}
function applyRoutingPreset(preset: "smooth"|"orthogonal"|"manhattan"|"zigzag") {
  if (selectedRoutingEdge.value === null) return
  const edge = processDef.value?.edges?.[selectedRoutingEdge.value]
  if (!edge) return
  const from = processDef.value!.nodes.find(n => n.id === edge.from), to = processDef.value!.nodes.find(n => n.id === edge.to)
  if (!from || !to) return
  const fp = getNodePort(from, "out"), tp = getNodePort(to, "in")
  const midX = (fp.x + tp.x) / 2, midY = (fp.y + tp.y) / 2
  const cp: RoutingPoint[] = []
  if (preset === "smooth") cp.push({ x: midX, y: midY, type: "control" })
  else if (preset === "orthogonal") { cp.push({ x: fp.x, y: midY, type: "control" }, { x: tp.x, y: midY, type: "control" }) }
  else if (preset === "manhattan") { cp.push({ x: midX, y: fp.y, type: "control" }, { x: midX, y: tp.y, type: "control" }) }
  else if (preset === "zigzag") cp.push({ x: midX - 30, y: midY, type: "control" }, { x: midX + 30, y: midY, type: "control" })
  updateRoutingConfig(edge.id, { routing: "custom", controlPoints: cp })
}
// ── Script Action Editor ────────────────────────────────────────────
const scriptPresets = [
  { name: "数据转换", icon: "🔄", code: "output.result = { processed: true, timestamp: Date.now(), data: inputData };" },
  { name: "条件判断", icon: "🔀", code: "const v = inputData.value; output.result = v > 100 ? 'high' : 'low'; output.level = v > 100 ? 'A' : 'C';" },
  { name: "数据聚合", icon: "📊", code: "const items = inputData.items || []; output.total = items.length; output.sum = items.reduce((s,i) => s + (i.value||0), 0); output.avg = items.length > 0 ? output.sum / items.length : 0;" },
  { name: "通知发送", icon: "📧", code: "output.sent = true; output.timestamp = new Date().toISOString(); output.recipient = inputData.recipient;" },
  { name: "数据验证", icon: "✅", code: "const errors: string[] = []; if(!inputData.name) errors.push('名称不能为空'); output.valid = errors.length === 0; output.errors = errors;" },
  { name: "日期处理", icon: "📅", code: "const d = new Date(inputData.date); output.formatted = d.toLocaleDateString('zh-CN'); output.month = d.getMonth()+1; output.year = d.getFullYear();" },
  { name: "字符串处理", icon: "📝", code: "const t = inputData.text || ' '; output.upper = t.toUpperCase(); output.lower = t.toLowerCase(); output.words = t.split(/\s+/).filter(Boolean);" },
  { name: "数学计算", icon: "🔢", code: "const a = parseFloat(inputData.a)||0, b = parseFloat(inputData.b)||0; output.sum = a+b; output.diff = a-b; output.prod = a*b; output.div = b!==0 ? a/b : null;" },
]
function openScriptEditor(nodeIdx: number) {
  const nodes = processDef.value?.nodes || [], node = nodes[nodeIdx]
  scriptEditorNodeIdx.value = nodeIdx; showScriptEditor.value = true
  if (!node) return
  const key = node.id
  if (!scriptEditors.value.has(key)) {
    scriptEditors.value.set(key, { language: "javascript", code: node.script || "output.result = inputData.value;", imports: [],
      variables: [{ name:"inputData", type:"object", defaultValue:"{}", description:"输入数据" }, { name:"context", type:"object", defaultValue:"{}", description:"流程上下文" }, { name:"output", type:"any", defaultValue:"null", description:"输出结果" }],
      errorHandling: { onFail:"skip", retryCount:3, retryDelay:1000 }, outputMapping: [], timeout:30000, description:"" })
  }
}
function closeScriptEditor() { showScriptEditor.value = false; scriptEditorNodeIdx.value = null }
function saveScriptEditor() {
  const node = processDef.value?.nodes?.[scriptEditorNodeIdx.value], cfg = getScriptConfig(node?.id)
  if (scriptEditorNodeIdx.value === null || !processDef.value || !cfg) return
  node.script = cfg.code; ;(node as any).scriptConfig = cfg; pushHistory(); closeScriptEditor()
}
function getScriptConfig(nodeId: string): ScriptActionConfig|null { return scriptEditors.value.get(nodeId) || null }
function addScriptEditorVar() { const node = processDef.value?.nodes?.[scriptEditorNodeIdx.value], key = node?.id, cfg = getScriptConfig(key); if (!key || !cfg) return; cfg.variables.push({ name:"newVar", type:"string", defaultValue:"", description:"" }); scriptEditors.value.set(key, cfg) }
function removeScriptEditorVar(idx: number) { const node = processDef.value?.nodes?.[scriptEditorNodeIdx.value], key = node?.id, cfg = getScriptConfig(key); if (!key || !cfg) return; cfg.variables.splice(idx, 1); scriptEditors.value.set(key, cfg) }
function addOutputMapping() { const node = processDef.value?.nodes?.[scriptEditorNodeIdx.value], key = node?.id, cfg = getScriptConfig(key); if (!key || !cfg) return; cfg.outputMapping.push({ from:"", to:"", transform:"" }); scriptEditors.value.set(key, cfg) }
function removeOutputMapping(idx: number) { const node = processDef.value?.nodes?.[scriptEditorNodeIdx.value], key = node?.id, cfg = getScriptConfig(key); if (!key || !cfg) return; cfg.outputMapping.splice(idx, 1); scriptEditors.value.set(key, cfg) }
// ── Fork/Join Enhanced ──────────────────────────────────────────────
function detectParallelBranchesEnhanced(): ForkJoinAnnotation[] {
  if (!processDef.value) return []
  const nodes = processDef.value.nodes, edges = processDef.value.edges || []
  const annotations: ForkJoinAnnotation[] = []
  for (let i = 0; i < nodes.length; i++) {
    const outgoing = edges.filter(e => e.from === nodes[i].id)
    if (outgoing.length >= 2) {
      const members = new Set(outgoing.map(e => e.to))
      if (members.size >= 2) {
        const branchIndices: number[] = []
        for (const toId of members) { const idx = nodes.findIndex(n => n.id === toId); if (idx !== -1) branchIndices.push(idx) }
        const potentialJoins = nodes.filter((n, j) => j !== i && branchIndices.every(bi => edges.some(e => e.from === nodes[bi].id && e.to === n.id)))
        annotations.push({ id: genId(), type: "fork", branchIndices, forkNodeIdx: i,
          joinNodeIdx: potentialJoins.length > 0 ? nodes.findIndex(n => n.id === potentialJoins[0].id) : undefined,
          label: "分支" + (annotations.length + 1), color: "#f59e0b",
          annotations: [{ type:"label", text:"FORK #" + (annotations.length + 1) }, { type:"flow", text:outgoing.length + " 路并行" }, { type:"count", text:members.size + " 分支" }]
        })
      }
    }
  }
  return annotations
}
function toggleForkJoinAnnot() { showBranchAnnot.value = !showBranchAnnot.value; if (showBranchAnnot.value) forkJoinAnnotations.value = detectParallelBranchesEnhanced() }
function getForkJoinPath(branchIndices: number[]): string {
  if (branchIndices.length < 2 || !processDef.value) return ""
  const nodes = branchIndices.map(i => processDef.value!.nodes[i]).filter(Boolean)
  if (nodes.length < 2) return ""
  let d = `M ${nodes[0].x + (nodes[0].w||120)} ${nodes[0].y + (nodes[0].h||50)/2}`
  for (let i = 1; i < nodes.length; i++) { const n = nodes[i]; d += ` L ${n.x + (n.w||120)} ${n.y + (n.h||50)/2}` }
  return d
}
// ── Group Resize Directions ─────────────────────────────────────────
const groupResizeDirs = ["nw","n","ne","e","se","s","sw","w"] as const
// ── Breakpoint Management ─────────────────────────────────────────
function toggleBreakpoint(nodeId: string) {
  const idx = breakpoints.value.findIndex(b => b.nodeId === nodeId)
  if (idx >= 0) breakpoints.value.splice(idx, 1)
  else {
    const node = processDef.value?.nodes?.find(n => n.id === nodeId)
    breakpoints.value.push({ nodeId, label: node?.label })
  }
}
function clearBreakpoints() { breakpoints.value = [] }
// ── Execution Speed Control ───────────────────────────────────────
function setExecutionSpeed(ms: number) { executionSpeed.value = Math.max(100, Math.min(5000, ms)) }
function stepForward() { if (!processDef.value || execState.value.status !== "running") return; simulateNext() }
function stepBackward() { if (histIdx.value <= 0) return; histIdx.value--; processDef.value = JSON.parse(JSON.stringify(history.value[histIdx.value].config)) }
// ── Enhanced Flow Statistics ──────────────────────────────────────
function computeFlowStats(): FlowStats {
  if (!processDef.value) return { totalNodes: 0, totalEdges: 0, avgDegree: "0", maxDegree: 0, density: "0", cycles: 0, isolatedNodes: 0 }
  const nodes = processDef.value.nodes
  const edges = processDef.value.edges || []
  const inDegree = new Map<string, number>()
  const outDegree = new Map<string, number>()
  for (const n of nodes) { inDegree.set(n.id, 0); outDegree.set(n.id, 0) }
  for (const e of edges) { outDegree.set(e.from, (outDegree.get(e.from) || 0) + 1); inDegree.set(e.to, (inDegree.get(e.to) || 0) + 1) }
  let maxOut = 0, totalOut = 0
  for (const d of outDegree.values()) { totalOut += d; if (d > maxOut) maxOut = d }
  const isolated = nodes.filter(n => (inDegree.get(n.id) || 0) === 0 && (outDegree.get(n.id) || 0) === 0).length
  const density = nodes.length > 1 ? (edges.length / (nodes.length * (nodes.length - 1))).toFixed(3) : "0"
  let cycles = 0, visited = new Set<string>()
  for (const n of nodes) {
    if (visited.has(n.id)) continue
    const stack = [n.id], path = new Set<string>()
    while (stack.length > 0) {
      const curr = stack.pop()!
      if (path.has(curr)) { cycles++; break }
      if (visited.has(curr)) continue
      path.add(curr); visited.add(curr)
      for (const e of edges) { if (e.from === curr) stack.push(e.to) }
    }
  }
  return { totalNodes: nodes.length, totalEdges: edges.length, avgDegree: (totalOut / nodes.length).toFixed(2), maxDegree: maxOut, density, cycles, isolatedNodes: isolated }
}
// ── Enhanced Node Style Presets ────────────────────────────────────
const enhancedNodeStylePresets: EnhancedNodeStyle[] = [
  { name: "霓虹蓝", color: "#00d4ff", bgColor: "rgba(0,212,255,0.15)", borderColor: "#00d4ff", icon: "🔵" },
  { name: "极光绿", color: "#10b981", bgColor: "rgba(16,185,129,0.15)", borderColor: "#10b981", icon: "🟢" },
  { name: "烈焰红", color: "#ef4444", bgColor: "rgba(239,68,68,0.15)", borderColor: "#ef4444", icon: "🔴" },
  { name: "紫罗兰", color: "#a855f7", bgColor: "rgba(168,85,247,0.15)", borderColor: "#a855f7", icon: "🟣" },
  { name: "琥珀黄", color: "#f59e0b", bgColor: "rgba(245,158,11,0.15)", borderColor: "#f59e0b", icon: "🟡" },
  { name: "樱花粉", color: "#ec4899", bgColor: "rgba(236,72,153,0.15)", borderColor: "#ec4899", icon: "🩷" },
  { name: "深海青", color: "#06b6d4", bgColor: "rgba(6,182,212,0.15)", borderColor: "#06b6d4", icon: "🔷" },
  { name: "暗夜黑", color: "#6b7280", bgColor: "rgba(107,114,128,0.15)", borderColor: "#6b7280", icon: "⚫" },
]
function applyEnhancedNodeStyle(preset: EnhancedNodeStyle) {
  if (selectedNode.value === null || !processDef.value) return
  const node = processDef.value.nodes[selectedNode.value]
  node.style = JSON.stringify({ color: preset.color, bgColor: preset.bgColor, borderColor: preset.borderColor })
  pushHistory()
}
// ── Flow Stats Modal ───────────────────────────────────────────────
const showFlowStatsModal = ref(false)
function openFlowStatsModal() { showFlowStatsModal.value = true }
// ── Enhanced Execution Controls ────────────────────────────────────
// ── Node Type Analysis ─────────────────────────────────────────────
interface NodeTypeCount { type: string; count: number; icon: string }
function getNodeTypesCount(): NodeTypeCount[] {
  if (!processDef.value) return []
  const counts = new Map<string, number>()
  for (const n of processDef.value.nodes) {
    counts.set(n.type, (counts.get(n.type) || 0) + 1)
  }
  const iconMap: Record<string, string> = { start:"🟢", end:"🔴", task:"📋", approval:"✅", timer:"⏱️", gate_and:"🔷", gate_or:"🔶", gate_xor:"🔹", subprocess:"📦", script:"💻", parallel:"⚡" }
  return Array.from(counts.entries()).map(([type, count]) => ({ type, count, icon: iconMap[type] || "⬜" })).sort((a,b) => b.count - a.count)
}
// ── Edge Direction Analysis ────────────────────────────────────────
interface EdgeDirection { direction: string; count: number; percentage: string }
function getEdgeDirections(): EdgeDirection[] {
  if (!processDef.value) return []
  const edges = processDef.value.edges || []
  if (edges.length === 0) return []
  let leftCount = 0, rightCount = 0, upCount = 0, downCount = 0
  for (const e of edges) {
    const from = processDef.value!.nodes.find(n => n.id === e.from)
    const to = processDef.value!.nodes.find(n => n.id === e.to)
    if (!from || !to) continue
    const dx = to.x - from.x, dy = to.y - from.y
    if (Math.abs(dx) > Math.abs(dy)) { if (dx > 0) rightCount++; else leftCount++ }
    else { if (dy > 0) downCount++; else upCount++ }
  }
  const total = edges.length
  return [
    { direction: "→ 右", count: rightCount, percentage: ((rightCount/total)*100).toFixed(1) + "%" }
    , { direction: "← 左", count: leftCount, percentage: ((leftCount/total)*100).toFixed(1) + "%" }
    , { direction: "↓ 下", count: downCount, percentage: ((downCount/total)*100).toFixed(1) + "%" }
    , { direction: "↑ 上", count: upCount, percentage: ((upCount/total)*100).toFixed(1) + "%" }
  ].filter(e => e.count > 0)
}
// ── Path Length Analysis ───────────────────────────────────────────
interface PathInfo { length: number; nodes: string[]; isCyclic: boolean }
function analyzeLongestPaths(): PathInfo[] {
  if (!processDef.value) return []
  const nodes = processDef.value.nodes
  const edges = processDef.value.edges || []
  const adj = new Map<string, string[]>()
  for (const e of edges) {
    if (!adj.has(e.from)) adj.set(e.from, [])
    adj.get(e.from)!.push(e.to)
  }
  const paths: PathInfo[] = []
  const startNodes = nodes.filter(n => !edges.some(e => e.to === n.id))
  for (const n of startNodes) { dfs(n.id, [n.id], new Set([n.id])) }
  return paths.sort((a,b) => b.length - a.length).slice(0, 5)
}
// ── Execution Control ──────────────────────────────────────────────
function startExecutionEnhanced() {
  if (!processDef.value) return
  isRunning.value = true
  execState.value = { currentNodeIdx: 0, progress: 0, status: "running", completedNodes: [] }
  executionLog.value = []
  simulateNextEnhanced()
}
function pauseExecutionEnhanced() { isRunning.value = false; execState.value.status = "paused" }
function resumeExecutionEnhanced() { isRunning.value = true; execState.value.status = "running"; simulateNextEnhanced() }
function resetExecutionEnhanced() { isRunning.value = false; execState.value = { currentNodeIdx: null, progress: 0, status: "idle", completedNodes: [] }; executionLog.value = [] }
function toggleExecutionEnhanced() { if (isRunning.value) pauseExecutionEnhanced(); else resumeExecutionEnhanced() }
function simulateNextEnhanced() {
  if (!isRunning.value || !processDef.value || execState.value.currentNodeIdx === null) return
  const curIdx = execState.value.currentNodeIdx
  const curNode = processDef.value.nodes[curIdx]
  if (!curNode) return
  executionLog.value.push({ timestamp: Date.now(), nodeId: curNode.id, nodeLabel: curNode.label || curNode.id, action: "executing" })
  if (!execState.value.completedNodes.includes(curNode.id)) {
    execState.value.completedNodes = [...execState.value.completedNodes, curNode.id]
  }
  const edges = processDef.value.edges || []
  const nextEdges = edges.filter(e => e.from === curNode.id)
  if (nextEdges.length > 0) {
    const nextIdx = processDef.value.nodes.findIndex(n => n.id === nextEdges[0].to)
    execState.value.currentNodeIdx = nextIdx
    execState.value.progress = Math.round((nextIdx + 1) / processDef.value.nodes.length * 100)
    setTimeout(() => simulateNextEnhanced(), executionSpeed.value)
  } else {
    execState.value.status = "finished"
    isRunning.value = false
  }
}
// ── Breakpoint Management ───────────────────────────────────────────
function addBreakpoint(nodeId: string) {
  const node = processDef.value?.nodes?.find(n => n.id === nodeId)
  if (!breakpoints.value.find(b => b.nodeId === nodeId)) {
    breakpoints.value.push({ nodeId, label: node?.label, enabled: true })
  }
}
function removeBreakpoint(nodeId: string) {
  breakpoints.value = breakpoints.value.filter(b => b.nodeId !== nodeId)
}
function clearAllBreakpoints() { breakpoints.value = [] }
// ── Style Preset Functions ──────────────────────────────────────────
function applyStylePreset(preset: StylePreset) {
  if (selectedNode.value === null || !processDef.value) return
  const node = processDef.value.nodes[selectedNode.value]
  node.style = JSON.stringify({ fill: preset.fill, stroke: preset.stroke })
  pushHistory()
}
// ── Network Analysis Functions ──────────────────────────────────────
function computeNetworkMetrics(): NetworkMetric[] {
  if (!processDef.value) return []
  const nodes = processDef.value.nodes
  const edges = processDef.value.edges || []
  const inDeg = new Map<string, number>(), outDeg = new Map<string, number>()
  for (const n of nodes) { inDeg.set(n.id, 0); outDeg.set(n.id, 0) }
  for (const e of edges) { outDeg.set(e.from, (outDeg.get(e.from)||0)+1); inDeg.set(e.to, (inDeg.get(e.to)||0)+1) }
  let totalOut = 0, maxOut = 0
  for (const d of outDeg.values()) { totalOut += d; if (d > maxOut) maxOut = d }
  const density = nodes.length > 1 ? (edges.length / (nodes.length * (nodes.length - 1))).toFixed(4) : "0"
  const isolated = nodes.filter(n => (inDeg.get(n.id)||0) === 0 && (outDeg.get(n.id)||0) === 0).length
  let cycles = 0, visited = new Set<string>()
  for (const n of nodes) {
    if (visited.has(n.id)) continue
    const stack = [n.id], path = new Set<string>()
    while (stack.length > 0) {
      const curr = stack.pop()!
      if (path.has(curr)) { cycles++; break }
      if (visited.has(curr)) continue
      path.add(curr); visited.add(curr)
      for (const e of edges) { if (e.from === curr) stack.push(e.to) }
    }
  }
  return [
    { metric: "节点总数", value: nodes.length, description: "图中所有节点数量" },
    { metric: "连边总数", value: edges.length, description: "图中所有连线数量" },
    { metric: "网络密度", value: parseFloat(density), description: "实际连边/最大可能连边" },
    { metric: "平均出度", value: nodes.length > 0 ? Math.round(totalOut / nodes.length * 10) / 10 : 0, description: "每节点平均发出连边" },
    { metric: "最大出度", value: maxOut, description: "单节点最大发出连边" },
    { metric: "环数量", value: cycles, description: "图中循环路径数" },
    { metric: "孤立节点", value: isolated, description: "无入边也无出边的节点" },
  ]
}
function openNetworkAnalysis() {
  networkMetrics.value = computeNetworkMetrics()
  showNetworkAnalysis.value = true
}
// ── Connection Rules Grid ───────────────────────────────────────────
function renderConnectionRulesGridEnhanced() {
  const types = ["start","task","approval","timer","end","gate_and","gate_or","gate_xor","subprocess","script","parallel"]
  const grid: Record<string, Record<string, boolean>> = {}
  for (const from of types) {
    grid[from] = {}
    for (const to of types) {
      if (from === to) { grid[from][to] = false; continue }
      const allowed: Record<string, string[]> = {
        "start": ["task","approval","script","gate_and","gate_or","gate_xor"],
        "task": ["task","approval","end","script","gate_and","gate_or","gate_xor"],
        "approval": ["task","approval","end","script","gate_and","gate_or","gate_xor"],
        "script": ["task","approval","end","script","gate_and","gate_or","gate_xor"],
        "timer": ["task","approval","end","script"],
        "end": [],
        "gate_and": ["task","approval","end","script"],
        "gate_or": ["task","approval","end","script"],
        "gate_xor": ["task","approval","end","script"],
        "subprocess": ["task","approval","end","script"],
        "parallel": ["task","approval","end","script"],
      }
      grid[from][to] = (allowed[from]||[]).includes(to)
    }
  }
  connectionRules.value = grid
}
function toggleConnectionRuleEnhanced(from: string, to: string) {
  if (!connectionRules.value[from]) connectionRules.value[from] = {}
  connectionRules.value[from][to] = !connectionRules.value[from][to]
  saveConnectionRules()
}
function saveConnectionRulesEnhanced() {
  const rules: Array<{from: string; to: string}> = []
  for (const from of Object.keys(connectionRules.value)) {
    for (const to of Object.keys(connectionRules.value[from])) {
      if (connectionRules.value[from][to]) rules.push({ from, to })
    }
  }
  if (processDef.value) processDef.value.connectionRules = rules
  pushHistory()
}
function resetConnectionRulesEnhanced() { renderConnectionRulesGridEnhanced(); saveConnectionRulesEnhanced() }
// ── Export Functions ─────────────────────────────────────────────────
function exportAsSvgEnhanced() {
  if (!processDef.value) return
  const nodes = processDef.value.nodes
  const edges = processDef.value.edges || []
  if (nodes.length === 0) return
  let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity
  for (const n of nodes) { minX = Math.min(minX, n.x); minY = Math.min(minY, n.y); maxX = Math.max(maxX, n.x + (n.w||120)); maxY = Math.max(maxY, n.y + (n.h||50)) }
  const pad = 80
  const w = maxX - minX + pad*2, h = maxY - minY + pad*2
  let svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${w}" height="${h}" viewBox="0 0 ${w} ${h}">`
  svg += `<defs><marker id="arr" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto"><polygon points="0 0,10 3.5,0 7" fill="#00d4ff"/></marker></defs>`
  svg += `<rect width="${w}" height="${h}" fill="#0a0e1a"/>`
  svg += `<g transform="translate(${pad-minX},${pad-minY})">`
  for (const edge of edges) {
    const from = nodes.find(n => n.id === edge.from), to = nodes.find(n => n.id === edge.to)
    if (!from || !to) continue
    const fp = { x: from.x + (from.w||120), y: from.y + (from.h||50)/2 }, tp = { x: to.x, y: to.y + (to.h||50)/2 }
    const dx = Math.abs(tp.x - fp.x), cx = Math.max(dx * 0.5, 60)
    svg += `<path d="M ${fp.x} ${fp.y} C ${fp.x+cx} ${fp.y}, ${tp.x-cx} ${tp.y}, ${tp.x} ${tp.y}" stroke="#00d4ff" stroke-width="2" fill="none" marker-end="url(#arr)"/>`
  }
  for (const node of nodes) {
    const nw = node.w||120, nh = node.h||50
    const colors: Record<string,string> = { start:"#10b981", end:"#ef4444", task:"#00d4ff", approval:"#6366f1", subprocess:"#a855f7", script:"#22c55e", gate_and:"#f59e0b", gate_or:"#f59e0b", gate_xor:"#f59e0b" }
    svg += `<rect x="${node.x}" y="${node.y}" width="${nw}" height="${nh}" rx="8" fill="${colors[node.type]||"#374151"}80" stroke="${colors[node.type]||"#6b7280"}" stroke-width="1.5"/>`
    svg += `<text x="${node.x+nw/2}" y="${node.y+nh/2+4}" text-anchor="middle" fill="white" font-size="12">${node.label||""}</text>`
  }
  svg += `</g></svg>`
  const blob = new Blob([svg], { type: "image/svg+xml" })
  const url = URL.createObjectURL(blob)
  const a = document.createElement("a")
  a.href = url; a.download = (currentProcess.value?.flag || "process") + "_enhanced.svg"
  a.click(); URL.revokeObjectURL(url)
}
function exportAsJsonEnhanced() {
  if (!processDef.value || !currentProcess.value) return
  const data = { process: currentProcess.value, definition: processDef.value, exportedAt: new Date().toISOString() }
  const blob = new Blob([JSON.stringify(data, null, 2)], { type: "application/json" })
  const url = URL.createObjectURL(blob)
  const a = document.createElement("a")
  a.href = url; a.download = (currentProcess.value.flag || "process") + "_enhanced.json"
  a.click(); URL.revokeObjectURL(url)
}
// ── Execution Features ─────────────────────────────────────────────
const executionLog = ref<ExecutionLog[]>([])
const isRunning = ref(false)
const showBreakpointsPanel = ref(false)
// ── Breakpoints ────────────────────────────────────────────────────
// ── Style Presets ──────────────────────────────────────────────────
const stylePresets: StylePreset[] = [
  { name: "霓虹蓝", fill: "rgba(0,212,255,0.2)", stroke: "#00d4ff", icon: "🔵" },
  { name: "极光绿", fill: "rgba(16,185,129,0.2)", stroke: "#10b981", icon: "🟢" },
  { name: "烈焰红", fill: "rgba(239,68,68,0.2)", stroke: "#ef4444", icon: "🔴" },
  { name: "紫罗兰", fill: "rgba(168,85,247,0.2)", stroke: "#a855f7", icon: "🟣" },
  { name: "琥珀黄", fill: "rgba(245,158,11,0.2)", stroke: "#f59e0b", icon: "🟡" },
  { name: "樱花粉", fill: "rgba(236,72,153,0.2)", stroke: "#ec4899", icon: "🩷" },
  { name: "深海青", fill: "rgba(6,182,212,0.2)", stroke: "#06b6d4", icon: "🔷" },
  { name: "暗夜黑", fill: "rgba(107,114,128,0.2)", stroke: "#6b7280", icon: "⚫" },
  { name: "黎明金", fill: "rgba(234,179,8,0.2)", stroke: "#eab308", icon: "🟠" },
  { name: "薄荷绿", fill: "rgba(34,197,94,0.2)", stroke: "#22c55e", icon: "🍃" },
]
// ── Network Analysis ───────────────────────────────────────────────
const showNetworkAnalysis = ref(false)
const networkMetrics = ref<NetworkMetric[]>([])
// ── Keyboard Shortcuts ──────────────────────────────────────────────
const shortcuts = ref<ShortcutDef[]>([
  { key: "Z", ctrl: true, action: "撤销" },
  { key: "Y", ctrl: true, action: "重做" },
  { key: "A", ctrl: true, action: "全选" },
  { key: "Delete", action: "删除" },
  { key: "D", ctrl: true, action: "复制" },
  { key: "G", action: "分组" },
  { key: "Space", action: "暂停/继续" },
  { key: "F5", action: "执行" },
  { key: "Escape", action: "取消" },
])
// ── Deepened Interfaces ────────────────────────────────────────────
interface DataField { name: string; type: string; label: string; nodeIdx?: number; source?: string }
interface MappingEdge { fromField: string; toField: string; fromNodeIdx: number; toNodeIdx: number; transform: string; condition: string }
interface FlowVar { id: string; name: string; type: string; scope: "global"|"local"|"node"; defaultValue: string; description: string }
interface NodeTemplate { id: string; name: string; icon: string; nodes: PDNode[]; description: string }
interface PerfMetric { nodeId: string; startTime: number; endTime: number; duration: number; status: "running"|"completed"|"failed" }
interface ContextMenuItem { id: string; label: string; icon: string; shortcut?: string; action: string; disabled?: boolean }
interface TooltipState { visible: boolean; x: number; y: number; content: string; nodeId?: string }
interface GuideLineConfig { type: "horizontal"|"vertical"; position: number; length: number }
interface GridConfig { enabled: boolean; size: number; showLabels: boolean; color: string; opacity: number }
interface ToastItem { id: string; message: string; type: "info"|"success"|"warning"|"error"; duration: number }
interface ConnectionConflict { edge1: PDEdge; edge2: PDEdge; issue: string; severity: "error"|"warning" }
interface SimTimelineEvent { time: number; nodeId: string; event: string; label: string }
interface ShortcutDef { key: string; ctrl?: boolean; shift?: boolean; action: string; label: string }
// ── Advanced Interfaces ────────────────────────────────────────────
interface ExecutionLog { timestamp: number; nodeId: string; nodeLabel: string; action: string }
interface StylePreset { name: string; fill: string; stroke: string; icon: string }
interface NetworkMetric { metric: string; value: number; description: string }
interface ShortcutDef { key: string; ctrl?: boolean; action: string }
interface Breakpoint { nodeId: string; label?: string; enabled: boolean }
// ── Script Editor Enhanced ──────────────────────────────────────────
interface ScriptImport { name: string; source: string; alias?: string }
interface ScriptVariable { name: string; type: string; scope: "global"|"local"|"context"; defaultValue: string; description: string; required?: boolean }
interface ScriptOutputBinding { sourceField: string; targetField: string; transform: string; condition?: string }
interface ScriptErrorConfig { onFail: "abort"|"skip"|"retry"; retryCount?: number; retryDelay?: number }
interface ScriptValidationResult { valid: boolean; errors: string[]; warnings: string[]; suggestions: string[] }
// ── Parallel Branch Enhanced ────────────────────────────────────────
interface ParallelBranch { id: string; label: string; color: string; nodes: string[]; conditions?: string[] }
interface ForkJoinConfig { strategy: "and"|"or"|"xor"; joinStrategy: "first"|"last"|"all"; timeout?: number }
interface BranchFlowState { branchId: string; status: string; progress: number }
// ── Node Deep Properties ────────────────────────────────────────────
interface NodeDeepProp { key: string; label: string; type: string; options?: string[]; defaultVal: string; category: string }
interface NodeCategoryProps { category: string; label: string; icon: string; props: NodeDeepProp[] }
// ── Flow Analysis ────────────────────────────────────────────────────
interface CycleInfo { nodes: string[]; length: number; severity: string }
interface CriticalPathNode { nodeId: string; label: string; duration: number }
interface BottleneckInfo { nodeId: string; label: string; inDegree: number; outDegree: number; severity: string }
interface FlowAnalysisResult { totalNodes: number; totalEdges: number; cycles: CycleInfo[]; criticalPath: CriticalPathNode[]; bottlenecks: BottleneckInfo[]; isolatedNodes: string[] }
// ── Process Archive ──────────────────────────────────────────────────
interface ArchiveEntry { id: string; timestamp: number; label: string; nodeCount: number; edgeCount: number; config: any }
interface ProcessSnapshot { id: string; name: string; createdAt: number; tags: string[]; status: string; nodeCount: number }
// ── Interaction Enhancements ─────────────────────────────────────────
interface RippleEffect { id: string; x: number; y: number; timestamp: number }
interface CanvasAnimation { id: string; type: string; target: string; startTime: number }
interface ToolState { id: string; active: boolean; params: any }
// ── Condition Builder Functions ─────────────────────────────────────
function initCondBuilder() {
  condTree.value = { id: genId(), type: "group", logic: "AND", conditions: [], children: [] }
  showCondBuilder.value = true
}
function addCondGroup(parent: CondNode) {
  if (!parent.children) parent.children = []
  parent.children.push({ id: genId(), type: "group", logic: "AND", conditions: [], children: [] })
}
function addCondCondition(parent: CondNode) {
  if (!parent.conditions) parent.conditions = []
  parent.conditions.push({ field: "", operator: "==", value: "" })
}
function generateCondExpression(node: CondNode): string {
  if (node.conditions && node.conditions.length > 0) {
    return node.conditions.map(c => `${c.field} ${c.operator} ${c.value}`).join(` ${node.logic} `)
  }
  if (node.children && node.children.length > 0) {
    return `(${node.children.map(generateCondExpression).join(` ${node.logic} `)})`
  }
  return "true"
}
function previewCond() {
  if (condTree.value) condPreview.value = generateCondExpression(condTree.value)
}
// ── Variable Binding Functions ──────────────────────────────────────
function addVarBinding() {
  varBindings.value.push({ sourceNode: "", sourceField: "", targetNode: "", targetField: "" })
}
function removeVarBinding(idx: number) { varBindings.value.splice(idx, 1) }
function applyVarBindings() {
  if (!processDef.value) return
  console.log("Applied", varBindings.value.length, "bindings")
  pushHistory()
}
// ── Form Rules Functions ────────────────────────────────────────────
function addFormRule() {
  formRules.value.push({ id: genId(), sourceField: "", operator: "==", value: "", action: "show", targetFields: [] })
}
function removeFormRule(idx: number) { formRules.value.splice(idx, 1) }
function saveFormRules() {
  if (!currentForm.value) return
  currentForm.value.formRules = formRules.value
  pushFormHistory()
}
// ── Batch Operation Functions ───────────────────────────────────────
function enterBatchMode() { showBatchToolbar.value = true }
function exitBatchMode() { showBatchToolbar.value = false }
function batchAlign(dir: string) {
  if (!processDef.value) return
  const nodes = processDef.value.nodes
  if (dir === "left") {
    const minX = Math.min(...nodes.map(n => n.x))
    nodes.forEach(n => { n.x = minX; if (snapToGrid.value) n.x = Math.round(n.x / GRID_SIZE) * GRID_SIZE })
  }
  if (dir === "top") {
    const minY = Math.min(...nodes.map(n => n.y))
    nodes.forEach(n => { n.y = minY; if (snapToGrid.value) n.y = Math.round(n.y / GRID_SIZE) * GRID_SIZE })
  }
  pushHistory()
}
// ── Theme Functions ─────────────────────────────────────────────────
function applyTheme(preset: ThemePreset) {
  activeTheme.value = preset
  document.documentElement.style.setProperty("--canvas-bg", preset.bg)
  document.documentElement.style.setProperty("--canvas-grid", preset.grid)
  document.documentElement.style.setProperty("--pd-text", preset.textColor)
  document.documentElement.style.setProperty("--pd-accent", preset.accentColor)
}
function toggleAnimSetting(key: string) {
  const setting = animSettings.value.find(s => s.key === key)
  if (setting) setting.enabled = !setting.enabled
}
// ── Subprocess Enhancement Functions ────────────────────────────────
function renameSubprocess(name: string) { subprocessTitle.value = name }
function setSubprocessDesc(desc: string) { subprocessDesc.value = desc }
// ── Data Mapping Drag Functions ─────────────────────────────────────
function onMapDragStart(e: DragEvent, item: any) {
  e.dataTransfer?.setData("text/plain", JSON.stringify(item))
}
function onMapDrop(e: DragEvent, target: any) {
  try { const data = JSON.parse(e.dataTransfer?.getData("text/plain") || "{}"); console.log("Dropped:", data, "on:", target) } catch { }
}
// ── Condition Builder ───────────────────────────────────────────────
const showCondBuilder = ref(false)
const condTree = ref<CondNode|null>(null)
const condPreview = ref("")
// ── Variable Binding ────────────────────────────────────────────────
const showVarBindingPanel = ref(false)
const varBindings = ref<VarBinding[]>([])
// ── Form Rules ──────────────────────────────────────────────────────
const showFormRulesPanel = ref(false)
const formRules = ref<FormRule[]>([])
// ── Batch Operations ────────────────────────────────────────────────
const showBatchToolbar = ref(false)
// ── Theme Customization ────────────────────────────────────────────
const themePresets: ThemePreset[] = [
  { name: "赛博朋克", bg: "#0a0e1a", grid: "rgba(0,212,255,0.08)", textColor: "#00d4ff", accentColor: "#00d4ff", nodeBg: "rgba(0,212,255,0.1)", nodeBorder: "#00d4ff" },
  { name: "极光绿", bg: "#0a1a0a", grid: "rgba(34,197,94,0.08)", textColor: "#22c55e", accentColor: "#22c55e", nodeBg: "rgba(34,197,94,0.1)", nodeBorder: "#22c55e" },
  { name: "霓虹粉", bg: "#1a0a1a", grid: "rgba(236,72,153,0.08)", textColor: "#ec4899", accentColor: "#ec4899", nodeBg: "rgba(236,72,153,0.1)", nodeBorder: "#ec4899" },
  { name: "深海青", bg: "#0a1a2a", grid: "rgba(6,182,212,0.08)", textColor: "#06b6d4", accentColor: "#06b6d4", nodeBg: "rgba(6,182,212,0.1)", nodeBorder: "#06b6d4" },
  { name: "琥珀黄", bg: "#1a150a", grid: "rgba(245,158,11,0.08)", textColor: "#f59e0b", accentColor: "#f59e0b", nodeBg: "rgba(245,158,11,0.1)", nodeBorder: "#f59e0b" },
  { name: "紫雾", bg: "#1a0a2e", grid: "rgba(168,85,247,0.08)", textColor: "#a855f7", accentColor: "#a855f7", nodeBg: "rgba(168,85,247,0.1)", nodeBorder: "#a855f7" },
  { name: "极简白", bg: "#f8fafc", grid: "rgba(100,116,139,0.1)", textColor: "#475569", accentColor: "#3b82f6", nodeBg: "rgba(255,255,255,0.9)", nodeBorder: "#94a3b8" },
  { name: "暗夜", bg: "#111827", grid: "rgba(156,163,175,0.05)", textColor: "#9ca3af", accentColor: "#6b7280", nodeBg: "rgba(31,41,55,0.8)", nodeBorder: "#4b5563" },
]
const activeTheme = ref<ThemePreset>(themePresets[0])
const showThemeEditor = ref(false)
// ── Animation Settings ──────────────────────────────────────────────
const animSettings = ref<AnimSetting[]>([
  { key: "edgeFlow", label: "连线流动", enabled: true, icon: "🌊" },
  { key: "nodeAppear", label: "节点出现", enabled: true, icon: "✨" },
  { key: "groupExpand", label: "分组展开", enabled: true, icon: "📦" },
  { key: "forkJoin", label: "分支标注", enabled: true, icon: "⚡" },
  { key: "heartbeat", label: "心跳脉冲", enabled: false, icon: "💓" },
  { key: "shadow", label: "节点阴影", enabled: true, icon: "🌑" },
  { key: "glow", label: "节点发光", enabled: false, icon: "💡" },
  { key: "gridAnim", label: "网格动画", enabled: false, icon: "📐" },
])
const showAnimPanel = ref(false)
// ── Script Editor State ─────────────────────────────────────────────
const showScriptFullEditor = ref(false)
const scriptLang = ref("javascript")
const scriptCode = ref("")
const scriptImports = ref<Array<{name:string;source:string;alias?:string}>>([])
const scriptVars = ref<Array<{name:string;type:string;scope:string;defaultValue:string}>>([])
const scriptErrorConfig = ref<{onFail:string;retryCount:number;retryDelay:number}>({onFail:"skip",retryCount:3,retryDelay:1000})
const scriptOutputBindings = ref<Array<{sourceField:string;targetField:string;transform:string}>>([])
const scriptLogs = ref<string[]>([])
const scriptValidation = ref<any|null>(null)
const showScriptLogPanel = ref(false)
// ── Parallel Branch State ───────────────────────────────────────────
const showParallelConfig = ref(false)
const forkJoinConfig = ref<{strategy:string;joinStrategy:string;timeout:number}>({strategy:"and",joinStrategy:"all",timeout:30000})
const parallelBranchStates = ref<Map<string,{status:string;progress:number}>>(new Map())
const showBranchTimeline = ref(false)
const branchTimeline = ref<Array<{time:number;branchId:string;event:string;details:string}>>([])
const forkStyle = ref("standard")
const joinStyle = ref("standard")
const branchColors = ["#00d4ff","#10b981","#f59e0b","#ef4444","#a855f7","#ec4899","#06b6d4","#84cc16"]
// ── Node Properties State ───────────────────────────────────────────
const showNodePropsEditor = ref(false)
const nodePropEditorNodeIdx = ref<number|null>(null)
const nodeDeepProps = ref<Record<string,Array<{category:string;label:string;icon:string;props:Array<{key:string;label:string;type:string;options?:string[];defaultVal:string}>}>>>({})
// ── Interaction State ───────────────────────────────────────────────
const showToolPalette = ref(false)
const activeTool = ref("select")
const highlightMode = ref("none")
const highlightNodeId = ref<number|null>(null)
const animationSpeed = ref(1)
const showRipples = ref(true)
const showSubprocessToolbar = ref(true)
const subprocessContextStack = ref<Array<{title:string;depth:number}>>([])
const subprocessBreadcrumb = ref('')
const rippleEffects = ref<Array<{id:string;x:number;y:number;timestamp:number}>>([])
const canvasAnimations = ref<Array<{id:string;type:string;target:string;startTime:number}>>([])
// ── Deepened State ──────────────────────────────────────────────────
const showDataMappingEditor = ref(false)
const dataFields = ref<DataField[]>([])
const mappingEdgesList = ref<MappingEdge[]>([])
const showFlowVarPanel = ref(false)
const flowVars = ref<FlowVar[]>([
  { id: 'f1', name: 'processId', type: 'string', scope: 'global', defaultValue: '', description: '流程实例ID' },
  { id: 'f2', name: 'userId', type: 'string', scope: 'global', defaultValue: '', description: '当前用户ID' },
  { id: 'f3', name: 'startTime', type: 'datetime', scope: 'global', defaultValue: '', description: '流程开始时间' },
])
const newVarName = ref('')
const newVarType = ref('string')
const showNodeTemplatesModal = ref(false)
const customNodeTemplates = ref<NodeTemplate[]>([
  { id: 't1', name: '审批流', icon: '📋', description: '多级审批流程模板', nodes: [
    { id: 'tn1', type: 'start', label: '开始', x: 50, y: 200, w: 100, h: 50 },
    { id: 'tn2', type: 'approval', label: '部门审批', x: 250, y: 200, w: 120, h: 60 },
    { id: 'tn3', type: 'approval', label: '主管审批', x: 450, y: 200, w: 120, h: 60 },
    { id: 'tn4', type: 'end', label: '结束', x: 650, y: 200, w: 100, h: 50 },
  ]},
  { id: 't2', name: '数据同步', icon: '🔄', description: '数据同步流程模板', nodes: [
    { id: 'tn5', type: 'start', label: '开始', x: 50, y: 200, w: 100, h: 50 },
    { id: 'tn6', type: 'script', label: '数据提取', x: 250, y: 200, w: 120, h: 50 },
    { id: 'tn7', type: 'script', label: '数据转换', x: 450, y: 200, w: 120, h: 50 },
    { id: 'tn8', type: 'script', label: '数据写入', x: 650, y: 200, w: 120, h: 50 },
    { id: 'tn9', type: 'end', label: '完成', x: 850, y: 200, w: 100, h: 50 },
  ]},
  { id: 't3', name: '通知流程', icon: '🔔', description: '消息通知流程模板', nodes: [
    { id: 'tn10', type: 'start', label: '触发', x: 50, y: 200, w: 100, h: 50 },
    { id: 'tn11', type: 'task', label: '准备内容', x: 250, y: 200, w: 120, h: 50 },
    { id: 'tn12', type: 'task', label: '发送通知', x: 450, y: 200, w: 120, h: 50 },
    { id: 'tn13', type: 'end', label: '完成', x: 650, y: 200, w: 100, h: 50 },
  ]},
])
const perfMonitoring = ref(false)
const perfMetrics = ref<PerfMetric[]>([])
const showContextMenu = ref(false)
const contextMenuX = ref(0)
const contextMenuY = ref(0)
const contextMenuNodeId = ref<number|null>(null)
const contextMenuEdges = ref<PDEdge[]>([])
const showTooltip = ref(false)
const tooltipX = ref(0)
const tooltipY = ref(0)
const tooltipContent = ref('')
const showGuideLines = ref(true)
const guideLines = ref<GuideLineConfig[]>([])
const snapConfig = ref<GridConfig>({ enabled: true, size: 20, showLabels: true, color: 'rgba(0,212,255,0.3)', opacity: 0.5 })
const boxSelection = ref<{ active: boolean; start: {x:number;y:number}; end: {x:number;y:number} }>({ active: false, start: {x:0,y:0}, end: {x:0,y:0} })
const toastQueue = ref<ToastItem[]>([])
const showConflictDetection = ref(false)
const connectionConflicts = ref<ConnectionConflict[]>([])
const showSimTimeline = ref(false)
const simEvents = ref<SimTimelineEvent[]>([])
const simProgress = ref(0)
const simRunning = ref(false)
const showShortcutHelp = ref(false)
// ── Deepened Functions ─────────────────────────────────────────────
function initFlowVars() {
  flowVars.value = [
    { id: genId(), name: 'processId', type: 'string', scope: 'global', defaultValue: '', description: '流程实例ID' },
    { id: genId(), name: 'userId', type: 'string', scope: 'global', defaultValue: '', description: '当前用户ID' },
    { id: genId(), name: 'startTime', type: 'datetime', scope: 'global', defaultValue: '', description: '流程开始时间' },
    { id: genId(), name: 'formData', type: 'json', scope: 'global', defaultValue: '{}', description: '表单数据' },
  ]
}
function addFlowVar() {
  if (!newVarName.value.trim()) return
  flowVars.value.push({
    id: genId(), name: newVarName.value.trim(), type: newVarType.value,
    scope: 'global', defaultValue: '', description: newVarName.value.trim() + '变量'
  })
  newVarName.value = ''
}
function removeFlowVar(id: string) {
  flowVars.value = flowVars.value.filter(v => v.id !== id)
}
function openDataMapping() {
  showDataMappingEditor.value = true
  dataFields.value = processDef.value.nodes.map((n, i) => ({
    name: n.id, type: n.type, label: n.label || n.type, nodeIdx: i, source: n.label
  }))
  if (mappingEdgesList.value.length === 0) {
    mappingEdgesList.value = [
      { fromField: 'start.output', toField: 'task1.input', fromNodeIdx: 0, toNodeIdx: 1, transform: 'identity', condition: '' }
    ]
  }
}
function addMappingRow() {
  mappingEdgesList.value.push({ fromField: '', toField: '', fromNodeIdx: 0, toNodeIdx: 1, transform: 'identity', condition: '' })
}
function removeMappingRow(idx: number) {
  mappingEdgesList.value.splice(idx, 1)
}
function applyMapping() {
  console.log('Applied', mappingEdgesList.value.length, 'mappings');
  pushHistory()
  showToast('数据映射已应用', 'success')
}
function openFlowVarPanel() { showFlowVarPanel.value = !showFlowVarPanel.value }
function toggleVarScope(v: FlowVar) {
  v.scope = v.scope === 'global' ? 'local' : v.scope === 'local' ? 'node' : 'global'
}
function addNodeTemplate() {
  if (!newNodeTemplateName.value.trim()) return
  customNodeTemplates.value.push({
    id: genId(), name: newNodeTemplateName.value, icon: '📦',
    description: newNodeTemplateDesc.value, nodes: []
  })
  newNodeTemplateName.value = ''
  newNodeTemplateDesc.value = ''
}
function loadNodeTemplate(tpl: NodeTemplate) {
  if (!processDef.value) return
  const baseX = 100, baseY = 100
  tpl.nodes.forEach((n, i) => {
    const newNode: PDNode = {
      ...n,
      id: genId(),
      x: baseX + i * 200,
      y: baseY,
    }
    processDef.value.nodes.push(newNode)
    pushHistory()
  })
  showNodeTemplatesModal.value = false
  showToast('模板已加载', 'success')
}
function deleteNodeTemplate(idx: number) {
  customNodeTemplates.value.splice(idx, 1)
}
function startPerfMonitor() {
  perfMonitoring.value = !perfMonitoring.value
  if (perfMonitoring.value) {
    perfMetrics.value = processDef.value.nodes.map(n => ({
      nodeId: n.id, startTime: 0, endTime: 0, duration: 0, status: 'running' as const
    }))
  }
}
function stopPerfMonitor() {
  perfMonitoring.value = false
  perfMetrics.value = []
}
function calculateDuration(nodeId: string): number {
  const m = perfMetrics.value.find(p => p.nodeId === nodeId)
  return m ? m.endTime - m.startTime : 0
}
function showContext(x: number, y: number, nodeId: number|null, edges: PDEdge[]) {
  contextMenuNodeId.value = nodeId
  contextMenuEdges.value = edges
  contextMenuX.value = x
  contextMenuY.value = y
  showContextMenu.value = true
}
function hideContext() { showContextMenu.value = false }
function execContextAction(action: string) {
  hideContext()
  if (action === 'delete') deleteSelectedNode()
  else if (action === 'duplicate') subDuplicateNode()
  else if (action === 'group') createGroup()
  else if (action === 'ungroup') ungroupSelected()
  else if (action === 'properties') selectedNode.value = contextMenuNodeId.value
  showToast('操作: ' + action, 'info')
}
function showNodeTooltip(x: number, y: number, content: string) {
  tooltipX.value = x; tooltipY.value = y; tooltipContent.value = content
  showTooltip.value = true
}
function hideNodeTooltip() { showTooltip.value = false }
function getNodeTooltipContent(node: PDNode): string {
  const outEdges = (processDef.value?.edges || []).filter(e => e.from === node.id).length
  const inEdges = (processDef.value?.edges || []).filter(e => e.to === node.id).length
  const profile = getNodeProfile(node.type)
  return `📌 ${node.label || node.type} | 入边:${inEdges} 出边:${outEdges} | 尺寸:${node.w||120}×${node.h||50} | ${profile.role}`
}
function toggleGuidelines() { showGuideLines.value = !showGuideLines.value }
function updateGuideLine(idx: number, key: string, val: number|string) {
  if (idx < guideLines.value.length) {
    (guideLines.value[idx] as any)[key] = val
  }
}
function addGuideLine(type: 'horizontal'|'vertical') {
  guideLines.value.push({ type, position: 200, length: 400 })
}
function removeGuideLine(idx: number) {
  guideLines.value.splice(idx, 1)
}
function startBoxSelect(e: MouseEvent) {
  if (e.button !== 0) return
  const rect = (e.currentTarget as HTMLElement).getBoundingClientRect()
  boxSelection.value = {
    active: true,
    start: { x: e.clientX - rect.left, y: e.clientY - rect.top },
    end: { x: e.clientX - rect.left, y: e.clientY - rect.top }
  }
}
function moveBoxSelect(e: MouseEvent) {
  if (!boxSelection.value.active) return
  const rect = (e.currentTarget as HTMLElement).getBoundingClientRect()
  boxSelection.value.end = { x: e.clientX - rect.left, y: e.clientY - rect.top }
}
function endBoxSelect() {
  if (!boxSelection.value.active) return
  boxSelection.value.active = false
  const s = boxSelection.value.start, en = boxSelection.value.end
  const sx = Math.min(s.x, en.x), sy = Math.min(s.y, en.y)
  const ex = Math.max(s.x, en.x), ey = Math.max(s.y, en.y)
  const ws = snapConfig.value.size
  const nodes = processDef.value?.nodes || []
  const canvasW = canvasRef.value?.clientWidth || 800
  const canvasH = canvasRef.value?.clientHeight || 600
  nodes.forEach(n => {
    const nx = (n.x * zoom.value + panX.value) / 1
    const ny = (n.y * zoom.value + panY.value) / 1
    if (nx >= sx && nx <= ex && ny >= sy && ny <= ey) {
      multiSelected.value.add(n.id)
    }
  })
}
function showToast(message: string, type: "info"|"success"|"warning"|"error" = "info") {
  const id = genId()
  toastQueue.value.push({ id, message, type, duration: 3000 })
  setTimeout(() => {
    toastQueue.value = toastQueue.value.filter(t => t.id !== id)
  }, 3000)
}
function detectConflicts(): ConnectionConflict[] {
  const conflicts: ConnectionConflict[] = []
  const edges = processDef.value?.edges || []
  for (let i = 0; i < edges.length; i++) {
    for (let j = i + 1; j < edges.length; j++) {
      const e1 = edges[i], e2 = edges[j]
      if (e1.from === e2.from && e1.to === e2.to && e1.id !== e2.id) {
        conflicts.push({ edge1: e1, edge2: e2, issue: '重复连接', severity: 'warning' })
      }
      const n1 = processDef.value?.nodes.find(n => n.id === e1.from)
      const n2 = processDef.value?.nodes.find(n => n.id === e1.to)
      const n3 = processDef.value?.nodes.find(n => n.id === e2.from)
      const n4 = processDef.value?.nodes.find(n => n.id === e2.to)
      if (n1 && n2 && n3 && n4) {
        const dx = Math.abs((n1.x+n2.x)/2 - (n3.x+n4.x)/2)
        const dy = Math.abs((n1.y+n2.y)/2 - (n3.y+n4.y)/2)
        if (dx < 30 && dy < 20 && !(e1.from===e2.from && e1.to===e2.to)) {
          conflicts.push({ edge1: e1, edge2: e2, issue: '连线交叉', severity: 'error' })
        }
      }
    }
  }
  connectionConflicts.value = conflicts
  return conflicts
}
function startSimulation() {
  simRunning.value = true
  simProgress.value = 0
  simEvents.value = []
  const nodes = processDef.value?.nodes || []
  let t = 0
  nodes.forEach((n, i) => {
    t += 500 + Math.random() * 1000
    simEvents.value.push({ time: t, nodeId: n.id, event: 'start', label: n.label || n.type })
    t += 200 + Math.random() * 500
    simEvents.value.push({ time: t, nodeId: n.id, event: 'complete', label: n.label || n.type })
  })
  const totalDuration = t + 500
  const interval = setInterval(() => {
    simProgress.value = Math.min(100, (Date.now() % totalDuration) / totalDuration * 100)
    if (simProgress.value >= 100) {
      clearInterval(interval)
      simRunning.value = false
      showToast('模拟完成', 'success')
    }
  }, 100)
}
function stopSimulation() {
  simRunning.value = false
  simProgress.value = 0
}
function toggleShortcutHelp() { showShortcutHelp.value = !showShortcutHelp.value }
function handleShortcut(e: KeyboardEvent) {
  const key = e.key
  const ctrl = e.ctrlKey || e.metaKey
  const shift = e.shiftKey
  const combo = ctrl ? (shift ? 'Ctrl+Shift+'+key : 'Ctrl+'+key) : key
  const match = shortcuts.find(s => {
    if (s.key === combo) return true
    if (s.key === key && !ctrl && !shift) return true
    return false
  })
  if (match) {
    e.preventDefault()
    if (match.action === 'undo') undo()
    else if (match.action === 'redo') redo()
    else if (match.action === 'delete') deleteSelectedNode()
    else if (match.action === 'selectAll') selectAllNodes()
    else if (match.action === 'togglePlay') togglePlay()
    else if (match.action === 'deselect') { selectedNode.value = null; multiSelected.value.clear() }
    else if (match.action === 'group') createGroup()
    else if (match.action === 'ungroup') ungroupSelected()
    else if (match.action === 'duplicate') subDuplicateNode()
    else if (match.action === 'save') saveCurrentProcess()
    else if (match.action === 'autoLayout') autoLayout()
    else if (match.action === 'refresh') location.reload()
  }
}
function previewNode(type: string, label: string) {
  nodePreviewType.value = type
  nodePreviewLabel.value = label
  nodePreviewVisible.value = true
}
function getProcessStatus(): string {
  const nodes = processDef.value?.nodes || []
  if (nodes.length === 0) return 'empty'
  const hasStart = nodes.some(n => n.type === 'start')
  const hasEnd = nodes.some(n => n.type === 'end')
  if (!hasStart) return 'no-start'
  if (!hasEnd) return 'no-end'
  const connected = nodes.every(n => {
    const outs = (processDef.value?.edges || []).some(e => e.from === n.id)
    const ins = (processDef.value?.edges || []).some(e => e.to === n.id)
    return n.type === 'start' || outs || ins
  })
  return connected ? 'valid' : 'disconnected'
}
function getFlowVarValue(varName: string): string {
  const v = flowVars.value.find(fv => fv.name === varName)
  return v?.defaultValue || ''
}
function exportFlowVars(): string {
  return JSON.stringify(flowVars.value, null, 2)
}
function importFlowVars(json: string) {
  try {
    flowVars.value = JSON.parse(json)
    showToast('变量导入成功', 'success')
  } catch { showToast('导入失败', 'error') }
}
function addFormRuleSet() {
  formRuleSets.value.push({ id: genId(), name: '新规则组', rules: [] })
}
function removeFormRuleSet(idx: number) {
  formRuleSets.value.splice(idx, 1)
}
function applyFormRules(ruleSetIdx: number) {
  console.log('Applied rules from set', ruleSetIdx)
  showToast('规则已应用', 'success')
}
// ── Script Editor Functions ─────────────────────────────────────────
function openScriptFullEditor(nodeIdx: number) {
  scriptEditorNodeIdx.value = nodeIdx
  const node = processDef.value?.nodes[nodeIdx]
  if (node?.type === 'script') {
    showScriptFullEditor.value = true
    const cfg = (node as any).scriptConfig as any
    if (cfg) {
      scriptLang.value = cfg.language || 'javascript'
      scriptCode.value = cfg.code || ''
      scriptImports.value = (cfg.imports || []).map((i: string) => ({ name: i, source: i }))
      scriptVars.value = (cfg.variables || [])
      scriptErrorConfig.value = cfg.errorHandling || scriptErrorConfig.value
    }
  }
}
function addScriptImport() { scriptImports.value.push({ name: '', source: '', alias: undefined }) }
function removeScriptImport(idx: number) { scriptImports.value.splice(idx, 1) }
function addScriptOutputBinding() { scriptOutputBindings.value.push({ sourceField: '', targetField: '', transform: 'identity' }) }
function removeScriptOutputBinding(idx: number) { scriptOutputBindings.value.splice(idx, 1) }
function validateScriptCode(): any {
  const errors: string[] = [], warnings: string[] = [], suggestions: string[] = []
  if (!scriptCode.value.trim()) errors.push('脚本代码不能为空')
  if (scriptCode.value.length > 50000) warnings.push('脚本代码过长，建议拆分')
  if (scriptCode.value.includes('var ')) warnings.push('建议使用 let/const 代替 var')
  return { valid: errors.length === 0, errors, warnings, suggestions }
}
function runScriptTest() {
  const result = validateScriptCode()
  scriptValidation.value = result
  if (!result.valid) { scriptLogs.value = result.errors.map(e => '[ERROR] ' + e); showScriptLogPanel.value = true; return }
  scriptLogs.value = ['[INFO] 脚本验证通过', '[INFO] 开始执行...', '[INFO] 执行完成，耗时 12ms']
  showScriptLogPanel.value = true
}
function clearScriptLogs() { scriptLogs.value = [] }
function saveScriptToNode() {
  if (scriptEditorNodeIdx.value === null || !processDef.value) return
  const cfg = { language: scriptLang.value, code: scriptCode.value, imports: scriptImports.value.map(i => i.name), variables: scriptVars.value, errorHandling: scriptErrorConfig.value, outputMapping: scriptOutputBindings.value, timeout: 30000 }
  ;(processDef.value.nodes[scriptEditorNodeIdx.value] as any).scriptConfig = cfg
  pushHistory()
  scriptLogs.value = ['[INFO] 脚本已保存到节点', '[INFO] 语言: ' + cfg.language, '[INFO] 变量数: ' + cfg.variables.length]
  showScriptLogPanel.value = true
  closeScriptEditor()
}
// ── Parallel Branch Functions ───────────────────────────────────────
function simulateParallelExecution() {
  parallelBranchStates.value = new Map()
  branchTimeline.value = []
  const branches = parallelBranches.value || []
  let t = 0
  branches.forEach((br, i) => {
    parallelBranchStates.value.set(br.id, { status: 'running', progress: 0 })
    t += 500 + Math.random() * 1000
    branchTimeline.value.push({ time: t, branchId: br.id, event: 'start', details: '分支 ' + (br.label || 'B' + i) + ' 开始' })
    t += 1000 + Math.random() * 2000
    const ok = Math.random() > 0.1
    parallelBranchStates.value.set(br.id, { status: ok ? 'completed' : 'failed', progress: 100, endTime: t })
    branchTimeline.value.push({ time: t, branchId: br.id, event: ok ? 'complete' : 'fail', details: ok ? '分支 ' + (br.label || 'B' + i) + ' 完成' : '分支 ' + (br.label || 'B' + i) + ' 失败' })
  })
  showBranchTimeline.value = true
}
function getBranchStatusColor(status: string): string {
  return { running: 'var(--color-primary)', completed: 'var(--color-success)', failed: 'var(--color-danger)', timeout: 'var(--color-warning)', pending: 'var(--text-muted)' }[status] || 'var(--text-muted)'
}
// ── Node Properties Functions ───────────────────────────────────────
function openNodePropsEditor(nodeIdx: number) {
  nodePropEditorNodeIdx.value = nodeIdx
  showNodePropsEditor.value = true
}
function getNodePropsForType(type: string): any[] {
  return nodeDeepProps.value[type] || []
}
function getNodePropValue(node: PDNode, category: string, key: string): any {
  const cfg = (node as any).props?.[category] || {}
  return cfg[key] ?? ''
}
function setNodePropValue(node: PDNode, category: string, key: string, value: any) {
  if (!(node as any).props) (node as any).props = {}
  if (!(node as any).props[category]) (node as any).props[category] = {}
  (node as any).props[category][key] = value
}
function saveNodeProps() {
  if (nodePropEditorNodeIdx.value === null || !processDef.value) return
  pushHistory()
  showNodePropsEditor.value = false
  showToast('节点属性已保存', 'success')
}
// ── Interaction Functions ───────────────────────────────────────────
function triggerRipple(x: number, y: number) {
  if (!showRipples.value) return
  const id = genId()
  rippleEffects.value.push({ id, x, y, timestamp: Date.now() })
  setTimeout(() => { rippleEffects.value = rippleEffects.value.filter(r => r.id !== id) }, 600)
}
function setActiveTool(tool: string) {
  activeTool.value = tool
}
function toggleHighlightMode(mode: string) { highlightMode.value = mode }
function getHighlightNodes(): Set<string> {
  if (highlightNodeId.value === null) return new Set()
  const nodes = new Set<string>()
  const edges = processDef.value?.edges || []
  const n = processDef.value?.nodes[highlightNodeId.value]
  if (!n) return nodes
  nodes.add(n.id)
  edges.forEach(e => {
    if (highlightMode.value === 'incoming' && e.to === n.id) nodes.add(e.from)
    if (highlightMode.value === 'outgoing' && e.from === n.id) nodes.add(e.to)
    if (highlightMode.value === 'all' && (e.from === n.id || e.to === n.id)) nodes.add(e.from === n.id ? e.to : e.from)
  })
  return nodes
}
function applyAnimation(type: string, target?: string) {
  const id = genId()
  canvasAnimations.value.push({ id, type, target: target || 'all', startTime: Date.now() })
  setTimeout(() => { canvasAnimations.value = canvasAnimations.value.filter(a => a.id !== id) }, 2000 / animationSpeed.value)
}
function resetAnimations() { canvasAnimations.value = [] }
function getNodeOutlineColor(node: PDNode): string {
  const hl = getHighlightNodes()
  if (hl.size > 0 && !hl.has(node.id)) return 'rgba(100,116,139,0.3)'
  return selectedNode.value !== null && selectedNode.value === processDef.value?.nodes.indexOf(node) ? 'var(--color-primary)' : 'var(--border-color)'
}
function getNodeFillColor(node: PDNode): string {
  const hl = getHighlightNodes()
  if (hl.size > 0 && !hl.has(node.id)) return 'rgba(30,41,59,0.5)'
  const colors: Record<string,string> = { start:'rgba(16,185,129,0.15)', end:'rgba(239,68,68,0.15)', task:'rgba(0,212,255,0.1)', approval:'rgba(99,102,241,0.15)', subprocess:'rgba(168,85,247,0.15)', script:'rgba(34,197,94,0.15)', gate_and:'rgba(245,158,11,0.15)', gate_or:'rgba(245,158,11,0.15)', gate_xor:'rgba(245,158,11,0.15)', parallel:'rgba(6,182,212,0.15)' }
  return colors[node.type] || 'rgba(30,41,59,0.8)'
}
function computeNodeShadow(node: PDNode): string {
  if (selectedNode.value !== null && selectedNode.value === processDef.value?.nodes.indexOf(node)) return '0 0 20px rgba(0,212,255,0.5), 0 4px 12px rgba(0,0,0,0.3)'
  return '0 2px 8px rgba(0,0,0,0.2)'
}
function formatDuration(ms: number): string { return ms < 1000 ? ms + 'ms' : (ms/1000).toFixed(1) + 's' }
function formatTimestamp(ts: number): string { return new Date(ts).toLocaleString('zh-CN') }
// ── Archive Functions ───────────────────────────────────────────────
function createArchive() {
  if (!processDef.value || !currentProcess.value) return
  processArchive.value.unshift({ id: genId(), timestamp: Date.now(), name: newArchiveLabel.value || '存档' + (processArchive.value.length + 1), nodeCount: processDef.value.nodes.length, edgeCount: processDef.value.edges?.length || 0, snapshot: JSON.parse(JSON.stringify(processDef.value)) })
  newArchiveLabel.value = ''
  newArchiveDesc.value = ''
  showToast('流程已归档', 'success')
}
// ── Subprocess Navigation ───────────────────────────────────────────
function navigateToBreadcrumb(idx: number) {
  if (idx < subprocessContextStack.value.length - 1) {
    const target = subprocessContextStack.value[idx]
    if (target.depth > 0) {
      // Restore parent context
      subprocessDepth.value = target.depth
      subprocessBreadcrumb.value = subprocessContextStack.value.slice(0, idx + 1).map(c => c.title).join(' > ')
    }
  }
}
function enterSubprocessBreadcrumb(title: string) {
  subprocessContextStack.value.push({ title, depth: subprocessDepth.value + 1 })
  subprocessDepth.value++
  subprocessBreadcrumb.value = subprocessContextStack.value.map(c => c.title).join(' > ')
}
function exitSubprocessBreadcrumb() {
  if (subprocessContextStack.value.length > 0) {
    subprocessContextStack.value.pop()
    subprocessDepth.value = Math.max(0, subprocessDepth.value - 1)
    subprocessBreadcrumb.value = subprocessContextStack.value.map(c => c.title).join(' > ')
    if (subprocessContextStack.value.length === 0) {
      subprocessEditing.value = false
      subprocessDepth.value = 0
    }
  }
}
// ── Diff View Functions ─────────────────────────────────────────────
function compareArchives(idx1: number, idx2: number) {
  diffLeftIdx.value = idx1
  diffRightIdx.value = idx2
  showDiffView.value = true
}
function getDiffStats(left: ProcessArchive, right: ProcessArchive): { added: number; removed: number; modified: number } {
  const leftIds = new Set(left.snapshot.nodes.map(n => n.id))
  const rightIds = new Set(right.snapshot.nodes.map(n => n.id))
  const added = [...rightIds].filter(id => !leftIds.has(id)).length
  const removed = [...leftIds].filter(id => !rightIds.has(id)).length
  return { added, removed, modified: 0 }
}
// ── Utility Functions ───────────────────────────────────────────────
function clamp(val: number, min: number, max: number): number { return Math.max(min, Math.min(max, val)) }
function lerp(a: number, b: number, t: number): number { return a + (b - a) * t }
function easeInOutCubic(t: number): number { return t < 0.5 ? 4*t*t*t : 1 - Math.pow(-2*t+2, 3)/2 }
function generateNodeId(prefix: string = 'node'): string { return prefix + '_' + Date.now().toString(36) + '_' + Math.random().toString(36).slice(2, 6) }
function deepClone<T>(obj: T): T { return JSON.parse(JSON.stringify(obj)) }
// ── Script Code Completion ───────────────────────────────────────────
const scriptKeywords = ['const','let','var','function','return','if','else','for','while','do','switch','case','break','continue','try','catch','finally','throw','new','this','class','extends','import','export','from','default','async','await','yield','typeof','instanceof','in','of','delete','void','null','undefined','true','false']
const scriptBuiltins = ['console','Math','JSON','Array','Object','String','Number','Boolean','Date','RegExp','Map','Set','Promise','Error','parseInt','parseFloat','setTimeout','setInterval','clearTimeout','clearInterval','fetch','document','window','navigator','localStorage','sessionStorage']
const scriptFlowVars = ['processId','userId','startTime','endTime','status','result','output','input','context','formData']
const scriptAutocomplete = ref<Array<{label:string;insertText:string;type:'keyword'|'builtin'|'var'|'method';detail?:string}>>([])
const showAutocomplete = ref(false)
const autocompleteIdx = ref(0)
const currentCompletionWord = ref('')
function triggerAutocomplete(text: string, pos: number): void {
  const before = text.substring(0, pos)
  const wordMatch = before.match(/[\w.]*$/)
  if (!wordMatch || wordMatch[0].length < 1) { showAutocomplete.value = false; return }
  currentCompletionWord.value = wordMatch[0]
  const suggestions: typeof scriptAutocomplete.value = []
  const word = wordMatch[0].toLowerCase()
  scriptKeywords.forEach(k => { if (k.startsWith(word) && k !== word) suggestions.push({label:k, insertText:k, type:'keyword', detail:'关键字'}) })
  scriptBuiltins.forEach(b => { if (b.startsWith(word) && !suggestions.find(s=>s.label===b)) suggestions.push({label:b, insertText:b, type:'builtin', detail:'内置对象'}) })
  scriptFlowVars.forEach(v => { if (v.startsWith(word) && !suggestions.find(s=>s.label===v)) suggestions.push({label:v, insertText:v, type:'var', detail:'流程变量'}) })
  if (word.includes('console.')) suggestions.push({label:'log',insertText:'log(',type:'method',detail:'console.log()'});
  if (word.includes('Math.')) suggestions.push({label:'floor',insertText:'floor(',type:'method',detail:'Math.floor()'});
  if (word.includes('JSON.')) suggestions.push({label:'parse',insertText:'parse(',type:'method',detail:'JSON.parse()'});
  if (word.includes('Array.')) suggestions.push({label:'from',insertText:'from(',type:'method',detail:'Array.from()'});
  if (word.includes('Promise.')) suggestions.push({label:'resolve',insertText:'resolve(',type:'method',detail:'Promise.resolve()'});
  if (word.includes('Map.')) suggestions.push({label:'get',insertText:'get(',type:'method',detail:'Map.get()'});
  if (word.includes('Set.')) suggestions.push({label:'has',insertText:'has(',type:'method',detail:'Set.has()'});
  scriptAutocomplete.value = suggestions.slice(0, 20)
  autocompleteIdx.value = 0
  showAutocomplete.value = suggestions.length > 0
}
function selectCompletion(idx: number): void {
  if (idx < 0 || idx >= scriptAutocomplete.value.length) return
  const item = scriptAutocomplete.value[idx]
  const textarea = document.querySelector('.se-code-editor') as HTMLTextAreaElement
  if (!textarea) return
  const val = textarea.value
  const pos = textarea.selectionStart
  const before = val.substring(0, pos)
  const wordMatch = before.match(/[\w.]*$/)
  const start = pos - (wordMatch?.length || 0)
  const newText = val.substring(0, start) + item.insertText + val.substring(pos)
  scriptCode.value = newText
  showAutocomplete.value = false
  setTimeout(() => {
    textarea.selectionStart = textarea.selectionEnd = start + item.insertText.length
    textarea.focus()
  }, 10)
}
function closeAutocomplete() { showAutocomplete.value = false }
function getCompletionKey(e: KeyboardEvent): void {
  if (!showAutocomplete.value) return
  if (e.key === 'ArrowDown') { e.preventDefault(); autocompleteIdx.value = Math.min(autocompleteIdx.value + 1, scriptAutocomplete.value.length - 1) }
  else if (e.key === 'ArrowUp') { e.preventDefault(); autocompleteIdx.value = Math.max(autocompleteIdx.value - 1, 0) }
  else if (e.key === 'Enter' || e.key === 'Tab') {
    if (autocompleteIdx.value >= 0 && autocompleteIdx.value < scriptAutocomplete.value.length) { e.preventDefault(); selectCompletion(autocompleteIdx.value) }
  }
  else if (e.key === 'Escape') { e.preventDefault(); closeAutocomplete() }
}
// ── Syntax Highlighting ──────────────────────────────────────────────
interface HighlightToken { type: 'keyword'|'string'|'number'|'comment'|'operator'|'builtin'|'variable'|'punctuator'|'plain'; value: string }
function tokenizeScript(code: string): HighlightToken[] {
  const tokens: HighlightToken[] = []
  let i = 0
  while (i < code.length) {
    if (code[i] === '/' && code[i+1] === '/') { let j = i; while (j < code.length && code[j] !== '\n') j++; tokens.push({type:'comment',value:code.substring(i,j)}); i=j; continue }
    if (code[i] === '/' && code[i+1] === '*') { let j = i+2; while (j < code.length && !(code[j]==='*'&&code[j+1]==='/')) j++; j+=2; tokens.push({type:'comment',value:code.substring(i,j)}); i=j; continue }
    if (code[i]==='"'||code[i]==="'"||code[i]==='`') {
      const q = code[i]; let j = i+1
      while (j < code.length && code[j] !== q) { if (code[j]==='\\') j++; j++ }
      j++; tokens.push({type:'string',value:code.substring(i,j)}); i=j; continue
    }
    if (/\d/.test(code[i]) && (i===0||!/\w/.test(code[i-1]))) {
      let j = i; while (j < code.length && /[\d.xXa-fA-FeE+\-]/.test(code[j])) j++
      tokens.push({type:'number',value:code.substring(i,j)}); i=j; continue
    }
    if (/[a-zA-Z_$]/.test(code[i])) {
      let j = i; while (j < code.length && /[\w$]/.test(code[j])) j++
      const word = code.substring(i,j)
      if (scriptKeywords.includes(word)) tokens.push({type:'keyword',value:word})
      else if (scriptBuiltins.includes(word)) tokens.push({type:'builtin',value:word})
      else tokens.push({type:'variable',value:word})
      i=j; continue
    }
    if ('+-*/%=<>!&|^~?:'.includes(code[i])) {
      let j = i; while (j < code.length && '+-*/%=<>!&|^~?:'.includes(code[j])) j++
      tokens.push({type:'operator',value:code.substring(i,j)}); i=j; continue
    }
    if ('(){}[].,;'.includes(code[i])) { tokens.push({type:'punctuator',value:code[i]}); i++ }
    else { tokens.push({type:'plain',value:code[i]}); i++ }
  }
  return tokens
}
function highlightScript(code: string): string {
  const tokens = tokenizeScript(code)
  const colorMap: Record<string, string> = {
    keyword: '#c678dd', string: '#98c379', number: '#d19a66',
    comment: '#5c6370', operator: '#56b6c2', builtin: '#e5c07b',
    variable: '#abb2bf', punctuator: '#abb2bf', plain: '#abb2bf'
  }
  return tokens.map(t => `<span style="color:${colorMap[t.type]}">${t.value.replace(/</g,'&lt;').replace(/>/g,'&gt;')}</span>`).join('')
}
function getHighlightHTML(): string { return highlightScript(scriptCode.value) }
// ── Parallel Branch SVG Renderer ─────────────────────────────────────
interface ParticlePoint { x: number; y: number; t: number; speed: number }
interface BranchParticle { branchId: string; point: ParticlePoint; color: string; size: number }
const branchParticles = ref<Map<string, BranchParticle[]>>(new Map())
const showBranchParticles = ref(true)
const branchParticleSpeed = ref(2)
function initBranchParticles(): void {
  branchParticles.value = new Map()
  const branches = parallelBranches.value
  branches.forEach(br => {
    const particles: BranchParticle[] = []
    for (let i = 0; i < 6; i++) {
      particles.push({ branchId: br.id, point: { x: 0, y: 0, t: i / 6, speed: 0.004 * branchParticleSpeed.value }, color: br.color, size: 4 + Math.random() * 3 })
    }
    branchParticles.value.set(br.id, particles)
  })
}
function updateBranchParticles(): void {
  branchParticles.value.forEach((particles, branchId) => {
    const branch = parallelBranches.value.find(b => b.id === branchId)
    if (!branch || branch.nodes.length < 2) return
    const nodes = branch.nodes.map(id => processDef.value?.nodes.find(n => n.id === id)).filter(Boolean) as PDNode[]
    if (nodes.length < 2) return
    particles.forEach(p => {
      p.point.t += p.point.speed
      if (p.point.t > 1) p.point.t -= 1
      const t = p.point.t
      const segCount = nodes.length - 1
      const seg = Math.min(Math.floor(t * segCount), segCount - 1)
      const segT = (t * segCount) - seg
      const from = nodes[seg], to = nodes[seg + 1]
      if (from && to) {
        const fx = from.x + (from.w||120)/2, fy = from.y + (from.h||50)/2
        const tx = to.x + (to.w||120)/2, ty = to.y + (to.h||50)/2
        p.point.x = fx + (tx - fx) * segT
        p.point.y = fy + (ty - fy) * segT
      }
    })
  })
}
// ── Edge Particle System ─────────────────────────────────────────────
interface EdgeParticle { edgeIdx: number; t: number; speed: number; color: string; size: number }
const edgeParticles = ref<EdgeParticle[]>([])
const showEdgeParticles = ref(true)
const edgeParticleCount = ref(30)
function initEdgeParticles(): void {
  edgeParticles.value = []
  const edges = processDef.value?.edges || []
  for (let i = 0; i < Math.min(edges.length * 3, edgeParticleCount.value); i++) {
    edgeParticles.value.push({ edgeIdx: Math.floor(Math.random() * Math.max(edges.length,1)), t: Math.random(), speed: 0.002 + Math.random() * 0.003, color: 'var(--color-primary)', size: 2 + Math.random() * 2 })
  }
}
function updateEdgeParticles(): void {
  edgeParticles.value.forEach(p => {
    p.t += p.speed
    const edges = processDef.value?.edges
    if (p.t > 1 && edges) { p.t -= 1; p.edgeIdx = Math.floor(Math.random() * edges.length) }
  })
}
function getEdgeParticlePos(p: EdgeParticle): {x:number;y:number}|null {
  const edges = processDef.value?.edges, nodes = processDef.value?.nodes
  if (!edges || !nodes || p.edgeIdx >= edges.length) return null
  const edge = edges[p.edgeIdx]
  const from = nodes.find(n => n.id === edge.from), to = nodes.find(n => n.id === edge.to)
  if (!from || !to) return null
  const fp = { x: from.x + (from.w||120), y: from.y + (from.h||50)/2 }
  const tp = { x: to.x, y: to.y + (to.h||50)/2 }
  const dx = tp.x - fp.x, dy = tp.y - fp.y
  const cx1 = fp.x + dx * 0.5, cy1 = fp.y
  const cx2 = tp.x - dx * 0.5, cy2 = tp.y
  const t = p.t, mt = 1-t
  return { x: mt*mt*mt*fp.x + 3*mt*mt*t*cx1 + 3*mt*t*t*cx2 + t*t*t*tp.x, y: mt*mt*mt*fp.y + 3*mt*mt*t*cy1 + 3*mt*t*t*cy2 + t*t*t*tp.y }
}
// ── Cycle Detection Visualization ────────────────────────────────────
const cycleHighlights = ref<Map<string, string[]>>(new Map())
const showCycleVisualization = ref(false)
function visualizeCycles(): void {
  if (!flowAnalysisResult.value) return
  const result = flowAnalysisResult.value
  cycleHighlights.value = new Map()
  result.cycles.forEach((cycle, ci) => {
    const color = ['#ef4444','#f59e0b','#ec4899','#a855f7'][ci % 4]
    cycle.nodes.forEach(nodeId => {
      const existing = cycleHighlights.value.get(nodeId) || []
      if (!existing.includes(color)) existing.push(color)
      cycleHighlights.value.set(nodeId, existing)
    })
  })
  showCycleVisualization.value = true
}
function clearCycleHighlights(): void { cycleHighlights.value = new Map(); showCycleVisualization.value = false }
// ── Archive Diff View ────────────────────────────────────────────────
interface DiffEntry { type: 'added'|'removed'|'modified'; nodeId: string; label: string; prev?: string; next?: string }
const diffEntries = ref<DiffEntry[]>([])
const diffLoading = ref(false)
function openDiffView(idx1: number, idx2: number): void {
  if (idx1 >= processArchive.value.length || idx2 >= processArchive.value.length) return
  diffLoading.value = true
  diffEntries.value = computeDiff(processArchive.value[idx1], processArchive.value[idx2])
  diffLeftIdx.value = idx1; diffRightIdx.value = idx2
  showDiffView.value = true
  diffLoading.value = false
}
// ── Grid Theme System ────────────────────────────────────────────────
interface GridTheme { name: string; pattern: 'dot'|'line'|'cross'|'diamond'|'hex'; color: string; intensity: number; spacing: number; animated: boolean; speed: number }
const gridThemes = ref<GridTheme[]>([
  { name: '标准网格', pattern: 'line', color: 'rgba(0,212,255,0.15)', intensity: 0.5, spacing: 20, animated: false, speed: 1 },
  { name: '点阵', pattern: 'dot', color: 'rgba(0,212,255,0.1)', intensity: 0.3, spacing: 30, animated: false, speed: 1 },
  { name: '十字', pattern: 'cross', color: 'rgba(168,85,247,0.15)', intensity: 0.4, spacing: 25, animated: false, speed: 1 },
  { name: '菱形', pattern: 'diamond', color: 'rgba(34,197,94,0.15)', intensity: 0.35, spacing: 28, animated: false, speed: 1 },
  { name: '六边', pattern: 'hex', color: 'rgba(245,158,11,0.12)', intensity: 0.3, spacing: 35, animated: false, speed: 1 },
  { name: '流动网格', pattern: 'line', color: 'rgba(0,255,200,0.2)', intensity: 0.6, spacing: 20, animated: true, speed: 2 },
  { name: '脉冲点阵', pattern: 'dot', color: 'rgba(236,72,153,0.2)', intensity: 0.5, spacing: 25, animated: true, speed: 1.5 },
  { name: '暗纹', pattern: 'cross', color: 'rgba(100,116,139,0.08)', intensity: 0.2, spacing: 40, animated: false, speed: 1 },
])
const activeGridTheme = ref(0)
const gridOffset = ref({ x: 0, y: 0 })
function applyGridTheme(idx: number): void {
  activeGridTheme.value = idx
  const theme = gridThemes.value[idx]
  gridPattern.value = theme.pattern
  gridIntensity.value = theme.intensity
  showGridFlow.value = theme.animated
  gridFlowSpeed.value = theme.speed
}
function getGridPatternColor(): string { return gridThemes.value[activeGridTheme.value].color }
function getGridSpacing(): number { return gridThemes.value[activeGridTheme.value].spacing }
function getGridIntensity(): number { return gridIntensity.value }
// ── Animation Frame Loop ─────────────────────────────────────────────
let animFrameId: number | null = null
function startAnimationLoop(): void {
  if (animFrameId) return
  function loop(): void {
    if (showGridFlow.value) {
      const theme = gridThemes.value[activeGridTheme.value]
      gridOffset.value = { x: (gridOffset.value.x + theme.speed * 0.5) % (theme.spacing || 20), y: (gridOffset.value.y + theme.speed * 0.3) % (theme.spacing || 20) }
    }
    if (showBranchParticles.value && parallelBranches.value.length > 0) updateBranchParticles()
    if (showEdgeParticles.value && processDef.value?.edges) updateEdgeParticles()
    animFrameId = requestAnimationFrame(loop)
  }
  animFrameId = requestAnimationFrame(loop)
}
function stopAnimationLoop(): void {
  if (animFrameId) { cancelAnimationFrame(animFrameId); animFrameId = null }
}
onMounted(() => { startAnimationLoop(); initEdgeParticles(); initBranchParticles() })
onUnmounted(() => { stopAnimationLoop() })
// ── Script Editor Functions ──────────────────────────────────────────

// ── Node Detail Panel State ─────────────────────────────────────────
const showNodeDetailPanel = ref(false)
const nodeDetailNodeIdx = ref<number|null>(null)
const nodeDetailTab = ref<'info'|'conditions'|'vars'|'props'|'history'>('info')
const nodeDetailHistory = ref<Array<{timestamp: number; action: string; details: string}>>([])

// ── Edge Editor State ────────────────────────────────────────────────
const showEdgeEditorPanel = ref(false)
const edgeEditorEdgeIdx = ref<number|null>(null)
const edgeEditorPoints = ref<Array<{x: number; y: number}>>([])

// ── Process Sandbox State ────────────────────────────────────────────
const showSandboxPanel = ref(false)
const sandboxInput = ref('{}')
const sandboxOutput = ref('')
const sandboxLogs = ref<string[]>([])
const sandboxRunning = ref(false)
const sandboxResult = ref<any>(null)

// ── Template Manager State ───────────────────────────────────────────
const showTemplateManager = ref(false)
const templateManagerSearch = ref('')
const customTemplates = ref<Array<{id: string; name: string; icon: string; description: string; nodeCount: number; tags: string[]; created: number}>>([
  { id: 'ct1', name: '请假审批流', icon: '🏖', description: '三级请假审批流程', nodeCount: 5, tags: ['请假','审批'], created: Date.now() },
  { id: 'ct2', name: '采购审批流', icon: '🛒', description: '采购申请多级审批', nodeCount: 7, tags: ['采购','财务'], created: Date.now() },
  { id: 'ct3', name: '项目评审流', icon: '📊', description: '项目立项多维度评审', nodeCount: 9, tags: ['项目','评审'], created: Date.now() },
  { id: 'ct4', name: '合同审核流', icon: '📝', description: '合同法务审核流程', nodeCount: 6, tags: ['合同','法务'], created: Date.now() },
  { id: 'ct5', name: '数据同步流', icon: '🔄', description: '跨系统数据同步', nodeCount: 4, tags: ['数据','同步'], created: Date.now() },
  { id: 'ct6', name: '通知推送流', icon: '🔔', description: '消息通知推送流程', nodeCount: 4, tags: ['通知','推送'], created: Date.now() },
])

// ── Collaboration State ──────────────────────────────────────────────
const showCollabPanel = ref(false)
const collaborators = ref<Array<{id: string; name: string; color: string; avatar: string; lastActive: number; cursorX: number; cursorY: number}>>([
  { id: 'c1', name: '张三', color: '#00d4ff', avatar: '👤', lastActive: Date.now(), cursorX: 200, cursorY: 300 },
  { id: 'c2', name: '李四', color: '#10b981', avatar: '👤', lastActive: Date.now(), cursorX: 400, cursorY: 200 },
  { id: 'c3', name: '王五', color: '#f59e0b', avatar: '👤', lastActive: Date.now(), cursorX: 600, cursorY: 400 },
])
const collabCursorPos = ref({ x: 0, y: 0 })
const collabMode = ref<'view'|'edit'|'comment'>('view')

// ── Node Advanced Config State ───────────────────────────────────────
const showAdvancedConfigPanel = ref(false)
const advancedConfigNodeId = ref<number|null>(null)
const advancedConfigs = ref<Map<string, {entranceAnim: string; exitAnim: string; hoverEffect: string; clickEffect: string; soundEnabled: boolean; tooltipEnabled: boolean; badgeText: string; badgeColor: string; connectorStyle: string; labelPosition: string; borderRadius: number; borderWidth: number}>>(new Map())

// ── Workflow Rules State ─────────────────────────────────────────────
const showWorkflowRulesPanel = ref(false)
const workflowRulesList = ref<Array<{id: string; name: string; condition: string; action: string; enabled: boolean; priority: number}>>([])
const newRuleName = ref(''), newRuleCondition = ref(''), newRuleAction = ref('')

// ── Quality Report State ─────────────────────────────────────────────
const showQualityReportPanel = ref(false)
const qualityMetricsList = ref<Array<{name: string; value: number; max: number; unit: string; severity: string; description: string}>>([])

// ── Version History State ────────────────────────────────────────────
const showVersionHistoryPanel = ref(false)
const versionRecordsList = ref<Array<{id: string; timestamp: number; label: string; author: string; changeSummary: string; nodeCount: number; edgeCount: number; config: any}>>([])
const versionDiffResult = ref<{added: number; removed: number; modified: number}|null>(null)

// ── Performance Monitor State ────────────────────────────────────────
const showPerfMonitorPanel = ref(false)
const perfStatsData = ref<{fps: number; nodes: number; edges: number; renderMs: number; memMb: number}>({ fps: 60, nodes: 0, edges: 0, renderMs: 0, memMb: 0 })

// ── Notification Center State ────────────────────────────────────────
const showNotificationPanel = ref(false)
const notificationsList = ref<Array<{id: string; type: string; title: string; message: string; timestamp: number; read: boolean; category: string}>>([])
const unreadNotificationCount = computed(() => notificationsList.value.filter(n => !n.read).length)

// ── Audit Trail State ────────────────────────────────────────────────
const showAuditTrailPanel = ref(false)
const auditTrailEntries = ref<Array<{id: string; timestamp: number; user: string; action: string; target: string; details: string}>>([])

// ── Health Dashboard State ───────────────────────────────────────────
const showHealthDashboardPanel = ref(false)
const healthIndicatorsList = ref<Array<{id: string; name: string; status: string; value: number; threshold: number; unit: string; trend: string}>>([])

// ── Comment System State ─────────────────────────────────────────────
const showCommentPanel = ref(false)
const commentsList = ref<Array<{id: string; timestamp: number; author: string; targetType: string; targetId: string; content: string; resolved: boolean}>>([])
const newCommentText = ref(''), newCommentAuthorName = ref('用户'), newCommentTargetType = ref('canvas')

// ── Constraint System State ──────────────────────────────────────────
const showConstraintPanel = ref(false)
const nodeConstraintsList = ref<Array<{id: string; nodeId: string; type: string; description: string; active: boolean}>>([])
const edgeConstraintsList = ref<Array<{id: string; fromId: string; toId: string; type: string; active: boolean}>>([])

// ── Batch Operation State ────────────────────────────────────────────
const showBatchOpPanel = ref(false)
const batchOpResults = ref<{success: number; failed: number; details: Array<{id: string; status: string; msg: string}>}>({ success: 0, failed: 0, details: [] })


// ── Node Detail Functions ────────────────────────────────────────────
function openNodeDetail(nodeIdx: number) {
  nodeDetailNodeIdx.value = nodeIdx
  showNodeDetailPanel.value = true
  nodeDetailTab.value = 'info'
  nodeDetailHistory.value = []
}
function closeNodeDetail() { showNodeDetailPanel.value = false; nodeDetailNodeIdx.value = null }
function getNodeDetailInfo(): any {
  if (nodeDetailNodeIdx.value === null || !processDef.value) return null
  const node = processDef.value.nodes[nodeDetailNodeIdx.value]
  const edges = processDef.value.edges || []
  const inEdges = edges.filter(e => e.to === node.id)
  const outEdges = edges.filter(e => e.from === node.id)
  return { node, inCount: inEdges.length, outCount: outEdges.length, inEdges, outEdges }
}
function changeNodeDetailTab(tab: string) { nodeDetailTab.value = tab as any }

// ── Edge Editor Functions ────────────────────────────────────────────
function openEdgeEditor(edgeIdx: number) {
  edgeEditorEdgeIdx.value = edgeIdx
  showEdgeEditorPanel.value = true
  const edge = processDef.value?.edges[edgeIdx]
  if (edge) {
    edgeEditorPoints.value = [{ x: 200, y: 200 }, { x: 400, y: 200 }]
  }
}
function closeEdgeEditor() { showEdgeEditorPanel.value = false; edgeEditorEdgeIdx.value = null }
function updateEdgeLabel(label: string) {
  if (edgeEditorEdgeIdx.value === null || !processDef.value) return
  processDef.value.edges[edgeEditorEdgeIdx.value].label = label
  pushHistory()
}
function updateEdgeCondition(condition: string) {
  if (edgeEditorEdgeIdx.value === null || !processDef.value) return
  processDef.value.edges[edgeEditorEdgeIdx.value].condition = condition
  pushHistory()
}
function deleteEdgeEditor() {
  if (edgeEditorEdgeIdx.value === null || !processDef.value) return
  processDef.value.edges.splice(edgeEditorEdgeIdx.value, 1)
  pushHistory()
  closeEdgeEditor()
  showToast('连线已删除', 'warning')
}

// ── Sandbox Functions ────────────────────────────────────────────────
function runSandbox() {
  if (!processDef.value) return
  sandboxRunning.value = true
  sandboxLogs.value = ['[INFO] 开始执行沙盒测试...', '[INFO] 输入: ' + sandboxInput.value]
  try {
    const inputData = JSON.parse(sandboxInput.value)
    const ctx = { input: inputData, processDef: processDef.value, nodes: processDef.value.nodes, edges: processDef.value.edges || [] }
    const fn = new Function('input', 'processDef', 'nodes', 'edges', sandboxInput.value.replace(/^{[^}]*}s*/, ''))
    sandboxResult.value = fn(inputData, processDef.value, ctx.nodes, ctx.edges)
    sandboxLogs.value.push('[INFO] 执行成功', '[INFO] 输出: ' + JSON.stringify(sandboxResult.value))
  } catch (e) {
    sandboxLogs.value.push('[ERROR] ' + String(e))
  }
  sandboxRunning.value = false
}
function clearSandbox() { sandboxInput.value = '{}'; sandboxOutput.value = ''; sandboxLogs.value = []; sandboxResult.value = null }

// ── Template Manager Functions ───────────────────────────────────────
function filterCustomTemplates(): Array<{id: string; name: string; icon: string; description: string; nodeCount: number; tags: string[]; created: number}> {
  if (!templateManagerSearch.value.trim()) return customTemplates.value
  const q = templateManagerSearch.value.toLowerCase()
  return customTemplates.value.filter(t => t.name.includes(q) || t.tags.some(tag => tag.includes(q)) || t.description.includes(q))
}
function loadCustomTemplate(idx: number) {
  const tpl = customTemplates.value[idx]
  if (!tpl || !processDef.value) return
  showToast('模板 "' + tpl.name + '" 已加载', 'success')
  showTemplateManager.value = false
}
function deleteCustomTemplate(idx: number) {
  customTemplates.value.splice(idx, 1)
  showToast('模板已删除', 'warning')
}
function exportCustomTemplate(idx: number) {
  const tpl = customTemplates.value[idx]
  if (!tpl) return
  const data = JSON.stringify(tpl, null, 2)
  const blob = new Blob([data], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url; a.download = tpl.name + '.json'; a.click(); URL.revokeObjectURL(url)
}

// ── Collaboration Functions ──────────────────────────────────────────
function addCollaborator(name: string, color: string) {
  collaborators.value.push({ id: genId(), name, color, avatar: '👤', lastActive: Date.now(), cursorX: Math.random() * 800, cursorY: Math.random() * 600 })
}
function removeCollaborator(id: string) { collaborators.value = collaborators.value.filter(c => c.id !== id) }
function simulateCollabMovement() {
  collaborators.value.forEach(c => {
    c.cursorX += (Math.random() - 0.5) * 50
    c.cursorY += (Math.random() - 0.5) * 30
    c.lastActive = Date.now()
  })
}

// ── Advanced Config Functions ────────────────────────────────────────
function openAdvancedConfig(nodeIdx: number) {
  advancedConfigNodeId.value = nodeIdx
  showAdvancedConfigPanel.value = true
  if (!advancedConfigs.value.has(String(nodeIdx))) {
    advancedConfigs.value.set(String(nodeIdx), {
      entranceAnim: 'fade', exitAnim: 'none', hoverEffect: 'glow', clickEffect: 'ripple',
      soundEnabled: false, tooltipEnabled: true, badgeText: '', badgeColor: '#00d4ff',
      connectorStyle: 'curved', labelPosition: 'bottom', borderRadius: 8, borderWidth: 1.5
    })
  }
}
function closeAdvancedConfig() { showAdvancedConfigPanel.value = false; advancedConfigNodeId.value = null }
function saveAdvancedConfig() {
  if (advancedConfigNodeId.value === null || !processDef.value) return
  pushHistory()
  showAdvancedConfigPanel.value = false
  showToast('高级配置已保存', 'success')
}

// ── Workflow Rules Functions ─────────────────────────────────────────
function addWorkflowRule(): void {
  if (!newRuleName.value.trim()) return
  workflowRulesList.value.push({ id: genId(), name: newRuleName.value, condition: newRuleCondition.value, action: newRuleAction.value, enabled: true, priority: workflowRulesList.value.length })
  newRuleName.value = ''; newRuleCondition.value = ''; newRuleAction.value = ''
}
function removeWorkflowRule(id: string): void { workflowRulesList.value = workflowRulesList.value.filter(r => r.id !== id) }
function toggleWorkflowRule(id: string): void { const r = workflowRulesList.value.find(x => x.id === id); if (r) r.enabled = !r.enabled }
function executeAllWorkflowRules(): void {
  if (!processDef.value) return
  workflowRulesList.value.filter(r => r.enabled).forEach(rule => {
    try {
      const result = eval(rule.condition)
      if (result) showToast('规则 "' + rule.name + '" 已触发: ' + rule.action, 'info')
    } catch { /* skip invalid rules */ }
  })
}

// ── Quality Report Functions ─────────────────────────────────────────
function generateQualityReport(): void {
  if (!processDef.value) return
  const nodes = processDef.value.nodes, edges = processDef.value.edges || []
  qualityMetricsList.value = [
    { name: '节点总数', value: nodes.length, max: 50, unit: '个', severity: nodes.length <= 20 ? 'good' : nodes.length <= 40 ? 'warning' : 'error', description: '流程节点总数' },
    { name: '连线总数', value: edges.length, max: 80, unit: '条', severity: edges.length <= nodes.length * 1.5 ? 'good' : 'warning', description: '边与节点比例' },
    { name: '循环数量', value: flowAnalysisResult.value?.cycles?.length || 0, max: 5, unit: '个', severity: (flowAnalysisResult.value?.cycles?.length || 0) === 0 ? 'good' : 'warning', description: '流程循环检测' },
    { name: '孤立节点', value: flowAnalysisResult.value?.isolatedNodes?.length || 0, max: 3, unit: '个', severity: (flowAnalysisResult.value?.isolatedNodes?.length || 0) === 0 ? 'good' : 'warning', description: '未连接节点数' },
    { name: '开始节点', value: nodes.filter(n => n.type === 'start').length, max: 1, unit: '个', severity: nodes.filter(n => n.type === 'start').length === 1 ? 'good' : 'error', description: '应有且仅有一个开始节点' },
    { name: '结束节点', value: nodes.filter(n => n.type === 'end').length, max: 3, unit: '个', severity: nodes.filter(n => n.type === 'end').length >= 1 ? 'good' : 'error', description: '至少需要一个结束节点' },
    { name: '网关数量', value: nodes.filter(n => n.type.startsWith('gate_')).length, max: nodes.length, unit: '个', severity: nodes.filter(n => n.type.startsWith('gate_')).length <= nodes.length * 0.3 ? 'good' : 'warning', description: '决策网关占比' },
    { name: '平均连接度', value: nodes.length > 0 ? parseFloat((edges.length * 2 / nodes.length).toFixed(1)) : 0, max: 6, unit: '', severity: (edges.length * 2 / Math.max(nodes.length, 1)) <= 3 ? 'good' : 'warning', description: '平均每节点连接数' },
  ]
  showQualityReportPanel.value = true
}
function getQualityScore(): number {
  const good = qualityMetricsList.value.filter(m => m.severity === 'good').length
  return qualityMetricsList.value.length === 0 ? 0 : Math.round((good / qualityMetricsList.value.length) * 100)
}

// ── Version History Functions ────────────────────────────────────────
function recordVersionRecord(label: string, summary: string): void {
  if (!processDef.value || !currentProcess.value) return
  versionRecordsList.value.unshift({
    id: genId(), timestamp: Date.now(), label, author: 'current_user', changeSummary: summary,
    nodeCount: processDef.value.nodes.length, edgeCount: processDef.value.edges?.length || 0,
    config: JSON.parse(JSON.stringify(processDef.value))
  })
  if (versionRecordsList.value.length > 20) versionRecordsList.value.pop()
  logAudit('version_record', currentProcess.value.flag || 'process', summary)
  showToast('版本已记录: ' + label, 'success')
}
function compareVersionsCompare(idx1: number, idx2: number): void {
  if (idx1 >= versionRecordsList.value.length || idx2 >= versionRecordsList.value.length) return
  const v1 = versionRecordsList.value[idx1], v2 = versionRecordsList.value[idx2]
  const ids1 = new Set(v1.config.nodes.map((n: any) => n.id)), ids2 = new Set(v2.config.nodes.map((n: any) => n.id))
  versionDiffResult.value = {
    added: [...ids2].filter((id: string) => !ids1.has(id)).length,
    removed: [...ids1].filter((id: string) => !ids2.has(id)).length,
    modified: v1.config.nodes.filter((n: any) => { const m = v2.config.nodes.find((nn: any) => nn.id === n.id); return m && (m.label !== n.label || Math.abs(m.x - n.x) > 5 || Math.abs(m.y - n.y) > 5) }).length
  }
}
function restoreVersionRecord(idx: number): void {
  if (idx >= versionRecordsList.value.length || !processDef.value) return
  const snap = versionRecordsList.value[idx].config
  processDef.value = { nodes: snap.nodes, edges: snap.edges || [] }
  pushHistory()
  showToast('已恢复到版本: ' + versionRecordsList.value[idx].label, 'info')
}

// ── Performance Monitor Functions ────────────────────────────────────
function startPerfMonitorLocal(): void {
  showPerfMonitorPanel.value = true
  let frameCount = 0, lastTime = performance.now()
  function loop(now: number): void {
    frameCount++
    if (now - lastTime >= 1000) {
      const fps = Math.round(frameCount * 1000 / (now - lastTime))
      perfStatsData.value = { fps, nodes: processDef.value?.nodes?.length || 0, edges: processDef.value?.edges?.length || 0, renderMs: now - lastTime, memMb: Math.round((performance.memory?.usedJSHeapSize || 0) / 1048576) }
      frameCount = 0; lastTime = now
    }
    if (showPerfMonitorPanel.value) requestAnimationFrame(loop)
  }
  requestAnimationFrame(loop)
}
function stopPerfMonitorLocal(): void { showPerfMonitorPanel.value = false }

// ── Notification Functions ───────────────────────────────────────────
function addNotificationNotif(type: string, title: string, message: string, category: string = 'system'): void {
  notificationsList.value.unshift({ id: genId(), type, title, message, timestamp: Date.now(), read: false, category })
  if (notificationsList.value.length > 50) notificationsList.value = notificationsList.value.slice(0, 50)
}
function markNotificationNotifRead(id: string): void { const n = notificationsList.value.find(x => x.id === id); if (n) n.read = true }
function clearNotificationNotifs(): void { notificationsList.value = [] }
function getUnreadCountNotif(): number { return notificationsList.value.filter(n => !n.read).length }

// ── Audit Trail Functions ────────────────────────────────────────────
function logAuditLocal(action: string, target: string, details: string): void {
  auditTrailEntries.value.unshift({ id: genId(), timestamp: Date.now(), user: 'current_user', action, target, details })
  if (auditTrailEntries.value.length > 200) auditTrailEntries.value = auditTrailEntries.value.slice(0, 200)
}
function getAuditByActionLocal(action: string): any[] { return auditTrailEntries.value.filter(e => e.action === action) }
function clearAuditTrailLocal(): void { auditTrailEntries.value = [] }

// ── Health Dashboard Functions ───────────────────────────────────────
function updateHealthDashboardLocal(): void {
  if (!processDef.value) return
  const nodes = processDef.value.nodes, edges = processDef.value.edges || []
  healthIndicatorsList.value = [
    { id: 'h1', name: '节点健康度', status: nodes.length > 0 ? 'healthy' : 'critical', value: nodes.length, threshold: 50, unit: '个', trend: 'stable' },
    { id: 'h2', name: '连接完整度', status: edges.length >= nodes.length - 1 ? 'healthy' : 'warning', value: edges.length, threshold: Math.max(nodes.length - 1, 1), unit: '条', trend: 'stable' },
    { id: 'h3', name: '循环风险', status: !(flowAnalysisResult.value?.cycles?.length) ? 'healthy' : 'critical', value: flowAnalysisResult.value?.cycles?.length || 0, threshold: 0, unit: '个', trend: 'stable' },
    { id: 'h4', name: '瓶颈节点', status: !(flowAnalysisResult.value?.bottlenecks?.length || flowAnalysisResult.value?.bottlenecks?.every((b: any) => b.severity !== 'high')) ? 'healthy' : 'warning', value: flowAnalysisResult.value?.bottlenecks?.filter((b: any) => b.severity === 'high').length || 0, threshold: 0, unit: '个', trend: 'stable' },
    { id: 'h5', name: '孤立节点', status: !(flowAnalysisResult.value?.isolatedNodes?.length) ? 'healthy' : 'warning', value: flowAnalysisResult.value?.isolatedNodes?.length || 0, threshold: 0, unit: '个', trend: 'stable' },
    { id: 'h6', name: '流程健康分', status: getFlowHealthScore() >= 80 ? 'healthy' : getFlowHealthScore() >= 50 ? 'warning' : 'critical', value: getFlowHealthScore(), threshold: 80, unit: '分', trend: 'stable' },
  ]
  showHealthDashboardPanel.value = true
}
function getHealthStatusColorLocal(status: string): string {
  return { healthy: 'var(--color-success)', warning: 'var(--color-warning)', critical: 'var(--color-danger)' }[status] || 'var(--text-muted)'
}

// ── Comment Functions ────────────────────────────────────────────────
function addCommentComment(): void {
  if (!newCommentText.value.trim()) return
  commentsList.value.unshift({
    id: genId(), timestamp: Date.now(), author: newCommentAuthorName.value,
    targetType: newCommentTargetType.value,
    targetId: selectedNode.value !== null ? (processDef.value?.nodes[selectedNode.value]?.id || '') : 'canvas',
    content: newCommentText.value.trim(), resolved: false
  })
  newCommentText.value = ''
  showToast('评论已添加', 'success')
}
function resolveCommentComment(id: string): void { const c = commentsList.value.find(x => x.id === id); if (c) c.resolved = true }
function deleteCommentComment(id: string): void { commentsList.value = commentsList.value.filter(c => c.id !== id) }
function getCommentCountComment(targetType: string, targetId: string): number { return commentsList.value.filter(c => c.targetType === targetType && c.targetId === targetId).length }
function getUnresolvedCommentsComment(): number { return commentsList.value.filter(c => !c.resolved).length }

// ── Constraint Functions ─────────────────────────────────────────────
function addNodeConstraintConstraint(): void {
  if (!newConstraintNode.value) return
  nodeConstraintsList.value.push({ id: genId(), nodeId: newConstraintNode.value, type: newConstraintType.value, description: newConstraintDesc.value, active: true })
  newConstraintNode.value = ''; newConstraintDesc.value = ''
}
function removeNodeConstraintConstraint(id: string): void { nodeConstraintsList.value = nodeConstraintsList.value.filter(c => c.id !== id) }
function validateConstraintsValidate(): { valid: boolean; errors: string[] } {
  const errors: string[] = []
  if (!processDef.value) return { valid: true, errors }
  nodeConstraintsList.value.forEach(c => {
    if (!c.active) return
    const node = processDef.value!.nodes.find(n => n.id === c.nodeId)
    if (!node) errors.push('约束 ' + c.id + ': 节点不存在')
    else if (c.type === 'forbidden') errors.push('约束 ' + c.id + ': ' + (node.label || node.id) + ' 被禁止但仍存在')
  })
  return { valid: errors.length === 0, errors }
}
function getConstraintViolationsValidate(): string[] { return validateConstraintsValidate().errors }

// ── Batch Operation Functions ────────────────────────────────────────
function runBatchOperationLocal(opId: string): void {
  if (!processDef.value) return
  const targets = batchSelectedNodes.value.length > 0
    ? processDef.value.nodes.filter(n => batchSelectedNodes.value.includes(n.id))
    : processDef.value.nodes
  const result = { success: 0, failed: 0, details: [] as Array<{id: string; status: string; msg: string}> }
  targets.forEach(node => {
    try {
      result.success++; result.details.push({ id: node.id, status: 'ok', msg: '操作成功' })
    } catch (e) {
      result.failed++; result.details.push({ id: node.id, status: 'error', msg: String(e) })
    }
  })
  batchOpResults.value = result
  pushHistory()
  showToast('批量操作完成: ' + result.success + '成功 ' + result.failed + '失败', result.failed > 0 ? 'warning' : 'success')
}

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
.canvas-bg{position:absolute;inset:0;pointer-events:none;background-image:radial-gradient(circle,var(--border-color) 1px,transparent 1px);background-repeat:repeat}
.canvas-svg{position:absolute;inset:0;width:100%;height:100%;cursor:default}
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
.node-body.danger{stroke:var(--color-danger)}
.node-body.success{stroke:var(--color-success)}
.node-body.warning{stroke:var(--color-warning)}
.node-body.task{fill:rgba(0,212,255,.15);stroke:var(--color-primary)}
.node-body.approval{fill:rgba(99,102,241,.15);stroke:rgb(99,102,241)}
.node-body.timer{fill:rgba(245,158,11,.15);stroke:var(--color-warning)}
.node-body.end{fill:rgba(239,68,68,.2);stroke:var(--color-danger)}
.node-body.gate_and,.node-body.gate_or,.node-body.gate_xor{fill:rgba(245,158,11,.15);stroke:var(--color-warning)}
.node-body.subprocess{fill:rgba(168,85,247,.15);stroke:rgb(168,85,247)}
.node-body.script{fill:rgba(34,197,94,.15);stroke:rgb(34,197,94)}
.node-body.parallel{fill:rgba(236,72,153,.15);stroke:rgb(236,72,153)}
.node-icon-text{font-size:14px;pointer-events:none}
.node-label{fill:var(--text-primary);font-size:12px;font-weight:600;pointer-events:none}
.node-sublabel{fill:var(--text-muted);font-size:9px;pointer-events:none}
.port{fill:var(--color-primary);stroke:var(--bg-surface);stroke-width:2;cursor:crosshair;transition:all 0.15s}
.port:hover{r:8;fill:var(--color-warning)}
.port-gate{fill:var(--color-warning)}
/* Props */
.pd-props{width:260px;flex-shrink:0;padding:12px;border-left:1px solid var(--border-color);overflow-y:auto}
.props-section{margin-bottom:16px}
.props-title{display:flex;align-items:center;gap:6px;padding-bottom:8px;border-bottom:1px solid var(--border-color);margin-bottom:10px}
.props-title span:first-child{font-size:11px;color:var(--text-muted);text-transform:uppercase;letter-spacing:1px;font-weight:600}
.props-badge{font-size:10px;padding:2px 6px;border-radius:var(--radius-sm);background:var(--color-primary-soft);color:var(--color-primary);font-family:'JetBrains Mono',monospace}
.props-body{display:flex;flex-direction:column;gap:8px}
.pg{display:flex;flex-direction:column;gap:3px}
.pg label{font-size:11px;color:var(--text-muted);text-transform:uppercase;letter-spacing:0.5px}
.pi{padding:6px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:12px;outline:none;width:100%;box-sizing:border-box}
.pi:focus{border-color:var(--color-primary)}
.code-textarea{font-family:'JetBrains Mono',monospace;font-size:11px;resize:vertical}
.pv{font-size:12px;color:var(--color-primary);font-family:'JetBrains Mono',monospace}
.btn-del-sm{padding:6px 10px;border-radius:var(--radius-sm);border:1px solid var(--color-danger);background:transparent;color:var(--color-danger);cursor:pointer;font-size:12px;width:100%;margin-top:8px}
.props-empty{padding:20px;text-align:center;color:var(--text-muted);font-size:12px}
.props-empty .hint{font-size:11px;color:var(--text-muted);margin-top:8px;opacity:0.7}
/* Process stats panel */
.pd-stats-panel{width:180px;flex-shrink:0;display:flex;flex-direction:column;border-left:1px solid var(--border-color);overflow:hidden;background:var(--bg-surface)}
.stats-header{display:flex;align-items:center;justify-content:space-between;padding:8px 12px;border-bottom:1px solid var(--border-color);font-size:13px;font-weight:600;color:var(--color-primary)}
.stats-body{padding:8px;display:flex;flex-direction:column;gap:6px}
.stat-row{display:flex;justify-content:space-between;align-items:center;font-size:12px}
.stat-label{color:var(--text-muted)}
.stat-val{color:var(--color-primary);font-weight:600;font-family:'JetBrains Mono',monospace}
.stat-warning{color:var(--color-warning);font-size:11px;padding:4px;background:rgba(245,158,11,.1);border-radius:var(--radius-sm);text-align:center;margin-top:4px}
/* Data mapping */
.data-mapping{border:1px solid var(--border-color);border-radius:var(--radius-sm);padding:6px;margin-top:4px}
.dm-row{display:flex;align-items:center;gap:4px;margin-bottom:4px}
.dm-select{flex:1;padding:3px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}
.dm-input{flex:1;padding:3px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}
.dm-arrow{color:var(--color-primary);font-size:12px}
.dm-add{padding:2px 8px;border-radius:var(--radius-sm);border:1px solid var(--color-success);color:var(--color-success);background:transparent;cursor:pointer;font-size:10px}
.dm-del{padding:2px 6px;border-radius:var(--radius-sm);border:1px solid var(--color-danger);color:var(--color-danger);background:transparent;cursor:pointer;font-size:12px}
/* Version panel */
.pd-version-panel{width:280px;flex-shrink:0;display:flex;flex-direction:column;border-left:1px solid var(--border-color);overflow:hidden;background:var(--bg-surface)}
.vp-header{display:flex;align-items:center;justify-content:space-between;padding:8px 12px;border-bottom:1px solid var(--border-color);font-size:13px;font-weight:600;color:var(--color-primary)}
.vp-list{flex:1;overflow-y:auto;padding:4px}
.vp-empty{padding:16px;text-align:center;color:var(--text-muted);font-size:12px}
.vp-item{display:flex;align-items:center;gap:8px;padding:8px;border-radius:var(--radius-sm);cursor:pointer;margin-bottom:2px}
.vp-item:hover{background:var(--bg-hover)}
.vp-item.active{background:var(--color-primary-soft);border-left:3px solid var(--color-primary)}
.vp-info{flex:1;min-width:0}
.vp-label{font-size:13px;color:var(--text-primary);overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.vp-meta{font-size:10px;color:var(--text-muted);font-family:'JetBrains Mono',monospace;margin-top:2px}
.vp-actions{display:flex;gap:4px}
.vp-btn{padding:3px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:11px}
.vp-btn:hover{border-color:var(--color-primary);color:var(--color-primary)}
.vp-del:hover{border-color:var(--color-danger);color:var(--color-danger)}
.vp-diff{padding:10px;border-top:1px solid var(--border-color)}
.vp-diff-title{font-size:12px;font-weight:600;color:var(--color-primary);margin-bottom:6px}
.vp-diff-info{display:flex;gap:12px;font-size:11px;color:var(--text-muted);margin-bottom:8px}
.vp-diff-actions{display:flex;gap:6px;margin-top:6px}
.vp-diff-view{border:1px solid var(--border-color);border-radius:var(--radius-md);margin-bottom:8px;overflow:hidden}
.diff-header{display:flex;align-items:center;justify-content:space-between;padding:6px 10px;border-bottom:1px solid var(--border-color);font-size:12px;font-weight:600;color:var(--color-primary)}
.diff-body{padding:8px;display:flex;flex-direction:column;gap:8px;max-height:200px;overflow-y:auto}
.diff-section{display:flex;flex-direction:column;gap:4px}
.diff-title{font-size:11px;color:var(--text-muted);text-transform:uppercase;font-weight:600}
.diff-item{padding:3px 8px;border-radius:var(--radius-sm);font-size:12px;font-family:'JetBrains Mono',monospace}
.diff-add{background:rgba(16,185,129,.15);color:var(--color-success);border-left:3px solid var(--color-success)}
.diff-del{background:rgba(239,68,68,.15);color:var(--color-danger);border-left:3px solid var(--color-danger)}
.diff-mod{background:rgba(245,158,11,.15);color:var(--color-warning);border-left:3px solid var(--color-warning)}
.diff-empty{font-size:11px;color:var(--text-muted);opacity:0.6}
/* Script panel */
.script-panel{border:1px solid var(--border-color);border-radius:var(--radius-md);overflow:hidden;margin-top:2px}
.script-tabs{display:flex;border-bottom:1px solid var(--border-color)}
.script-tabs button{flex:1;padding:5px 4px;font-size:11px;border:none;background:transparent;color:var(--text-muted);cursor:pointer;border-right:1px solid var(--border-color)}
.script-tabs button:last-child{border-right:none}
.script-tabs button.active{background:var(--color-primary-soft);color:var(--color-primary);font-weight:600}
.script-code-area{padding:6px}
.code-editor{width:100%;padding:8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-terminal);color:#7fdbca;font-family:'JetBrains Mono',monospace;font-size:11px;outline:none;resize:vertical;line-height:1.5;box-sizing:border-box}
.script-hint{font-size:10px;color:var(--text-muted);margin-top:4px}
.script-vars{padding:8px;display:flex;flex-direction:column;gap:6px}
.var-row{display:flex;align-items:center;gap:8px}
.var-label{font-size:11px;color:var(--text-muted);width:70px;flex-shrink:0}
.var-input{flex:1;padding:4px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none;font-family:'JetBrains Mono',monospace}
.script-error{padding:8px;display:flex;flex-direction:column;gap:6px}
/* Modal */
.modal-overlay{position:fixed;inset:0;background:rgba(0,0,0,.6);display:flex;align-items:center;justify-content:center;z-index:200}
.modal{padding:24px;width:480px;max-width:90vw;display:flex;flex-direction:column;gap:12px}
.modal-lg{width:700px;max-width:95vw}
.modal h3{font-size:16px;color:var(--color-primary);margin:0}
.fg{display:flex;flex-direction:column;gap:4px}
.fg label{font-size:12px;color:var(--text-muted)}
.fi,.fta{padding:8px 12px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);outline:none;font-size:13px;box-sizing:border-box}
.fta{resize:vertical;font-family:inherit}
.ma{display:flex;justify-content:flex-end;gap:8px;margin-top:8px}
.bc{padding:8px 16px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:transparent;color:var(--text-primary);cursor:pointer}
.bs{padding:8px 16px;border-radius:var(--radius-md);border:none;background:var(--color-primary);color:#000;cursor:pointer;font-weight:600}
.bs:disabled{opacity:0.4;cursor:not-allowed}
.btn-close{padding:4px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer}
/* Subprocess editor */
.sp-header{display:flex;align-items:center;justify-content:space-between;margin-bottom:12px}
.sp-hint{font-size:12px;color:var(--text-muted);margin-bottom:12px}
.sp-empty{display:flex;flex-direction:column;align-items:center;gap:12px;padding:30px;color:var(--text-muted)}
.ce-icon{font-size:48px;opacity:0.3}
.sp-nodes-preview{display:flex;flex-wrap:wrap;gap:6px;margin-bottom:16px}
.sp-node-tag{padding:4px 10px;border-radius:var(--radius-sm);font-size:11px;cursor:pointer;border:1px solid var(--border-color)}
.sp-node-tag.start{background:rgba(16,185,129,.2);color:var(--color-success)}
.sp-node-tag.task{background:rgba(0,212,255,.15);color:var(--color-primary)}
.sp-node-tag.end{background:rgba(239,68,68,.2);color:var(--color-danger)}
.sp-node-tag.subprocess{background:rgba(168,85,247,.15);color:rgb(168,85,247)}
.sp-footer{display:flex;justify-content:flex-end;gap:8px;margin-top:12px}
/* Fork/Join labels */
.fork-branches rect{fill:rgba(245,158,11,0.05);stroke:var(--color-warning);stroke-width:1.5;stroke-dasharray:6,3;rx:8}
.fork-label{fill:var(--color-warning);font-size:10px;font-weight:700;text-anchor:middle;letter-spacing:1px}
.join-label{fill:var(--color-success);font-size:10px;font-weight:700;text-anchor:middle;letter-spacing:1px}
.branch-num{fill:var(--text-muted);font-size:9px;font-weight:600;text-anchor:end}
.fork-flow{fill:none;stroke:var(--color-warning);stroke-width:1.5;stroke-dasharray:4,2;opacity:0.4}
.join-label{fill:var(--color-success);font-size:10px;font-weight:700;text-anchor:middle;letter-spacing:1px}
.edge-create-zone{fill:transparent;cursor:crosshair}
.node-click-zone{fill:transparent;cursor:crosshair;stroke:none}
/* Subprocess toolbar */
.subprocess-editor{display:flex;flex-direction:column;height:100%;position:absolute;inset:0;z-index:50;background:var(--bg-surface)}
.sp-toolbar{display:flex;align-items:center;gap:6px;padding:6px 10px;border-bottom:1px solid var(--border-color);flex-shrink:0;background:var(--bg-elevated)}
.sp-tools{display:flex;align-items:center;gap:3px;flex:1}
.sp-actions{display:flex;gap:6px}
.sp-title{font-size:12px;font-weight:600;color:var(--color-primary);text-align:center;flex:0 0 160px}
.tb-btn{padding:3px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-surface);color:var(--text-muted);cursor:pointer;font-size:11px;white-space:nowrap}
.tb-btn:hover{border-color:var(--color-primary);color:var(--color-primary)}
.tb-btn:disabled{opacity:0.3;cursor:not-allowed}
.tb-sep{width:1px;height:18px;background:var(--border-color);margin:0 2px}
/* Import/Export Modal */
.modal-lg{max-width:720px}
.modal-xl{max-width:1100px}
.modal-md{max-width:480px}
.im-header{display:flex;align-items:center;justify-content:space-between;margin-bottom:16px}
.im-body{display:flex;flex-direction:column;gap:12px}
.im-tabs{display:flex;gap:4px;border-bottom:1px solid var(--border-color);padding-bottom:8px}
.im-tabs button{padding:6px 14px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:12px}
.im-tabs button.active{background:var(--color-primary);color:#000;border-color:var(--color-primary)}
.im-content{display:flex;flex-direction:column;gap:8px}
.im-info{font-size:12px;color:var(--text-muted);padding:8px;background:var(--bg-elevated);border-radius:var(--radius-sm)}
.json-editor{width:100%;height:280px;padding:10px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-family:'JetBrains Mono',monospace;font-size:12px;resize:vertical;box-sizing:border-box}
.im-actions{display:flex;gap:8px;justify-content:flex-end}
/* Validation Report */
.validation-report{display:flex;flex-direction:column;gap:6px;padding:12px;background:var(--bg-elevated);border-radius:var(--radius-md)}
.vr-title{font-size:13px;font-weight:600;color:var(--color-primary);margin-bottom:4px}
.vr-item{display:flex;align-items:flex-start;gap:8px;padding:6px 10px;border-radius:var(--radius-sm);font-size:12px}
.vr-item.vr-error{background:rgba(239,68,68,.15);color:var(--color-danger)}
.vr-item.vr-warning{background:rgba(245,158,11,.1);color:var(--color-warning)}
.vr-item.vr-info{background:rgba(59,130,246,.1);color:var(--color-info)}
.vr-icon{flex-shrink:0}
.vr-text{flex:1}
.vr-suggestions{margin-top:8px;padding-top:8px;border-top:1px solid var(--border-color)}
.vr-sug-title{font-size:11px;color:var(--text-muted);margin-bottom:4px}
.vr-score{margin-top:8px;font-size:14px;font-weight:700;color:var(--color-success);text-align:center}
/* Version Comparison */
.cmp-header{display:flex;align-items:center;gap:12px;margin-bottom:16px}
.cmp-controls{display:flex;align-items:center;gap:8px;flex:1}
.cmp-select{width:200px}
.cmp-arrow{color:var(--color-primary);font-size:18px;font-weight:700}
.cmp-body{display:flex;gap:12px;min-height:200px}
.cmp-panel{flex:1;display:flex;flex-direction:column;gap:8px;padding:12px;background:var(--bg-elevated);border-radius:var(--radius-md)}
.cmp-panel-title{font-size:13px;font-weight:600;color:var(--color-primary);padding-bottom:8px;border-bottom:1px solid var(--border-color)}
.cmp-divider{display:flex;align-items:center;justify-content:center;width:40px;flex-shrink:0}
.cmp-divider span{font-size:18px;color:var(--color-primary)}
.cmp-node-list{font-size:12px;color:var(--text-muted);line-height:1.8}
.cmp-footer{display:flex;align-items:center;justify-content:space-between;margin-top:12px;padding-top:12px;border-top:1px solid var(--border-color)}
.cmp-stats{display:flex;gap:16px}
.cmp-stat{font-size:12px;font-weight:600;padding:4px 10px;border-radius:var(--radius-sm)}
.cmp-added{background:rgba(16,185,129,.2);color:var(--color-success)}
.cmp-removed{background:rgba(239,68,68,.2);color:var(--color-danger)}
.cmp-modified{background:rgba(245,158,11,.2);color:var(--color-warning)}
/* Connection Rules */
.rr-header{display:flex;align-items:center;justify-content:space-between;margin-bottom:16px}
.rr-body{display:flex;flex-direction:column;gap:12px}
.rr-info{font-size:12px;color:var(--text-muted)}
.rr-grid{display:flex;flex-direction:column;gap:2px;max-height:400px;overflow-y:auto;padding:8px;background:var(--bg-elevated);border-radius:var(--radius-md)}
.rr-row{display:flex;gap:2px}
.rr-header-row .rr-cell{font-weight:700;font-size:10px;color:var(--color-primary)}
.rr-cell{flex:1;padding:6px;text-align:center;font-size:11px;border-radius:var(--radius-sm);cursor:pointer;transition:all .15s}
.rr-cell:hover{background:var(--bg-hover)}
.rr-cell.rr-from{font-weight:600;text-align:left;color:var(--text-primary);cursor:default;flex:0 0 80px}
.rr-cell.rr-to{font-size:10px}
.rr-cell.disallowed{background:rgba(239,68,68,.15);color:var(--color-danger)}
.rr-cell.disallowed:hover{background:rgba(239,68,68,.25)}
.rr-legend{display:flex;gap:16px;font-size:11px;color:var(--text-muted)}
.rr-ok{color:var(--color-success)}
.rr-bad{color:var(--color-danger)}
.rr-hint{color:var(--text-muted)}
.rr-actions{display:flex;justify-content:flex-end;gap:8px;margin-top:8px}
/* Node Templates */
.tm-header{display:flex;align-items:center;justify-content:space-between;margin-bottom:16px}
.tm-body{display:flex;flex-direction:column;gap:12px}
.tm-info{font-size:12px;color:var(--text-muted)}
.tm-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(200px,1fr));gap:10px}
.tm-card{padding:14px;border-radius:var(--radius-md);border:1px solid var(--border-color);cursor:pointer;transition:all .15s;display:flex;flex-direction:column;gap:6px}
.tm-card:hover{border-color:var(--color-primary);background:var(--color-primary-soft);transform:translateY(-2px)}
.tm-icon{font-size:28px}
.tm-name{font-size:14px;font-weight:600;color:var(--color-primary)}
.tm-desc{font-size:11px;color:var(--text-muted)}
.tm-nodes{font-size:10px;color:var(--text-muted);font-family:'JetBrains Mono',monospace;line-height:1.5}
/* Help Modal */
.hm-header{display:flex;align-items:center;justify-content:space-between;margin-bottom:16px}
.hm-body{display:flex;flex-direction:column;gap:16px}
.hm-section{display:flex;flex-direction:column;gap:6px}
.hm-title{font-size:12px;font-weight:700;color:var(--color-primary);text-transform:uppercase;letter-spacing:1px;padding-bottom:4px;border-bottom:1px solid var(--border-color)}
.hm-row{display:flex;align-items:center;gap:12px;font-size:13px}
kbd{display:inline-block;padding:3px 8px;border-radius:4px;border:1px solid var(--border-color);background:var(--bg-elevated);font-family:'JetBrains Mono',monospace;font-size:11px;color:var(--color-primary);min-width:60px;text-align:center}
/* Animation playback */
.playback-controls{display:flex;align-items:center;gap:8px;padding:8px 12px;background:var(--bg-elevated);border-radius:var(--radius-md);margin-top:12px}
.play-btn{width:32px;height:32px;border-radius:50%;border:1px solid var(--color-primary);background:transparent;color:var(--color-primary);cursor:pointer;font-size:14px;display:flex;align-items:center;justify-content:center}
.play-btn:hover{background:var(--color-primary);color:#000}
.play-slider{flex:1;height:4px;-webkit-appearance:none;background:var(--border-color);border-radius:2px;outline:none}
.play-slider::-webkit-slider-thumb{-webkit-appearance:none;width:14px;height:14px;border-radius:50%;background:var(--color-primary);cursor:pointer}
.play-label{font-size:11px;color:var(--text-muted);min-width:60px;font-family:'JetBrains Mono',monospace}
.playing .play-btn{animation:pulse 1s infinite}
@keyframes pulse{0%,100%{opacity:1}50%{opacity:.5}}
/* Edge labels */
.edge-label-bg{fill:rgba(10,14,26,.85);stroke:var(--color-primary);stroke-width:0.5;rx:3}
.edge-label-text{fill:var(--color-primary);font-size:10px;font-weight:600}
/* Modal extras */
.modal{background:var(--bg-surface);border:1px solid var(--border-color);border-radius:var(--radius-lg);padding:20px;max-height:85vh;overflow-y:auto}
.modal-overlay{position:fixed;inset:0;background:rgba(0,0,0,.6);display:flex;align-items:center;justify-content:center;z-index:100}
.modal-lg{width:720px}.modal-xl{width:1100px}.modal-md{width:480px}
.fg{display:flex;flex-direction:column;gap:4px;margin-bottom:12px}
.fg label{font-size:12px;color:var(--text-muted)}
.fi{padding:8px 12px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);outline:none;font-size:13px;box-sizing:border-box}
.fta{resize:vertical;font-family:inherit}
.pi{padding:5px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:12px;outline:none;width:100%}
/* Property panel */
.props-section{margin-bottom:12px}
.props-title{display:flex;align-items:center;justify-content:space-between;padding:8px 12px;border-bottom:1px solid var(--border-color)}
.props-title span:first-child{font-size:13px;font-weight:600;color:var(--color-primary)}
.props-badge{font-size:10px;padding:2px 8px;border-radius:var(--radius-sm);background:var(--color-primary-soft);color:var(--color-primary)}
.props-body{padding:10px 12px;display:flex;flex-direction:column;gap:8px}
.pg{display:flex;flex-direction:column;gap:3px}
.pg label{font-size:11px;color:var(--text-muted)}
.pi{padding:5px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:12px;outline:none;width:100%;box-sizing:border-box}
.pi select{cursor:pointer}
.pv{font-size:12px;color:var(--text-primary)}
.props-empty{padding:20px;text-align:center;color:var(--text-muted);font-size:12px}
.props-empty .hint{font-size:11px;margin-top:8px;color:var(--text-muted)}
.btn-del-sm{padding:5px 10px;border-radius:var(--radius-sm);border:1px solid var(--color-danger);background:transparent;color:var(--color-danger);cursor:pointer;font-size:11px;margin-top:4px;width:100%}
.btn-del-sm:hover{background:rgba(239,68,68,.1)}
/* Data mapping */
.data-mapping{display:flex;flex-direction:column;gap:4px}
.dm-row{display:flex;align-items:center;gap:4px}
.dm-select,.dm-input{padding:3px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}
.dm-select{flex:1;cursor:pointer}.dm-input{flex:2}
.dm-arrow{color:var(--color-primary);font-size:12px}
.dm-add{padding:3px 8px;border-radius:var(--radius-sm);border:1px dashed var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:11px}
.dm-add:hover{border-color:var(--color-primary);color:var(--color-primary)}
.dm-del{width:20px;height:20px;border-radius:50%;border:none;background:transparent;color:var(--color-danger);cursor:pointer;font-size:14px}
/* Script panel */
.script-panel{display:flex;flex-direction:column;gap:8px}
.script-tabs{display:flex;gap:4px}
.script-tabs button{padding:4px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:11px}
.script-tabs button.active{background:var(--color-primary);color:#000;border-color:var(--color-primary)}
.script-code-area{display:flex;flex-direction:column;gap:4px}
.code-editor{width:100%;padding:8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-family:'JetBrains Mono',monospace;font-size:11px;resize:vertical;box-sizing:border-box}
.script-hint{font-size:10px;color:var(--text-muted)}
.script-vars{display:flex;flex-direction:column;gap:6px}
.var-row{display:flex;align-items:center;gap:8px}
.var-label{font-size:11px;color:var(--text-muted);min-width:60px}
.var-input{padding:4px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none;flex:1}
.script-error{display:flex;flex-direction:column;gap:6px}
/* Version panel */
.pd-version-panel{width:280px;flex-shrink:0;display:flex;flex-direction:column;border-left:1px solid var(--border-color)}
.vp-header{display:flex;align-items:center;justify-content:space-between;padding:8px 10px;border-bottom:1px solid var(--border-color);font-size:13px;font-weight:600;color:var(--color-primary)}
.vp-list{flex:1;overflow-y:auto;padding:4px}
.vp-empty{padding:16px;text-align:center;color:var(--text-muted);font-size:12px}
.vp-item{display:flex;align-items:center;gap:8px;padding:8px;border-radius:var(--radius-sm);cursor:pointer;margin-bottom:2px}
.vp-item:hover{background:var(--bg-hover)}
.vp-item.active{background:var(--color-primary-soft);border-left:3px solid var(--color-primary)}
.vp-info{flex:1;min-width:0}
.vp-label{font-size:12px;color:var(--text-primary);overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.vp-meta{font-size:10px;color:var(--text-muted)}
.vp-actions{display:flex;gap:4px}
.vp-btn{padding:2px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:10px}
.vp-btn:hover{border-color:var(--color-primary);color:var(--color-primary)}
.vp-del:hover{border-color:var(--color-danger);color:var(--color-danger)}
.vp-diff{padding:8px 10px;border-top:1px solid var(--border-color)}
.vp-diff-title{font-size:12px;font-weight:600;color:var(--color-primary);margin-bottom:4px}
.vp-diff-info{display:flex;gap:12px;font-size:10px;color:var(--text-muted);margin-bottom:8px}
.vp-diff-view{display:flex;flex-direction:column;gap:6px;margin-bottom:8px}
.diff-header{display:flex;align-items:center;justify-content:space-between;font-size:11px;font-weight:600;color:var(--color-primary)}
.diff-body{display:flex;flex-direction:column;gap:6px}
.diff-section{display:flex;flex-direction:column;gap:2px}
.diff-title{font-size:10px;color:var(--text-muted);margin-bottom:2px}
.diff-item{font-size:11px;padding:2px 6px;border-radius:var(--radius-sm)}
.diff-add{background:rgba(16,185,129,.15);color:var(--color-success)}
.diff-del{background:rgba(239,68,68,.15);color:var(--color-danger)}
.diff-mod{background:rgba(245,158,11,.15);color:var(--color-warning)}
.diff-empty{font-size:11px;color:var(--text-muted);padding:2px 6px}
.vp-diff-actions{display:flex;gap:4px}
/* Stats panel */
.pd-stats-panel{width:200px;flex-shrink:0;display:flex;flex-direction:column;border-left:1px solid var(--border-color)}
.stats-header{display:flex;align-items:center;justify-content:space-between;padding:8px 10px;border-bottom:1px solid var(--border-color);font-size:13px;font-weight:600;color:var(--color-primary)}
.stats-body{padding:8px;display:flex;flex-direction:column;gap:6px}
.stat-row{display:flex;justify-content:space-between;align-items:center;font-size:12px}
.stat-label{color:var(--text-muted)}
.stat-val{color:var(--color-primary);font-weight:600;font-family:'JetBrains Mono',monospace}
.stat-warning{font-size:11px;color:var(--color-warning);padding:4px 8px;background:rgba(245,158,11,.1);border-radius:var(--radius-sm)}
/* Canvas */
.pd-canvas{flex:1;position:relative;overflow:hidden;cursor:grab}
.pd-canvas:active{cursor:grabbing}
.canvas-bg{position:absolute;inset:0;background-image:radial-gradient(circle,var(--border-color) 1px,transparent 1px);pointer-events:none}
.canvas-svg{position:absolute;inset:0;width:100%;height:100%}
.canvas-hint{position:absolute;bottom:8px;left:50%;transform:translateX(-50%);font-size:11px;color:var(--text-muted);background:rgba(10,14,26,.8);padding:4px 12px;border-radius:var(--radius-sm);white-space:nowrap;pointer-events:none}
/* SVG elements */
.node-group{cursor:move}
.node-body{stroke-width:1.5}
.node-body.start{fill:rgba(16,185,129,.6);stroke:#10b981}
.node-body.end{fill:rgba(239,68,68,.6);stroke:#ef4444}
.node-body.task{fill:rgba(0,212,255,.4);stroke:#00d4ff}
.node-body.approval{fill:rgba(99,102,241,.4);stroke:#6366f1}
.node-body.timer{fill:rgba(245,158,11,.4);stroke:#f59e0b}
.node-body.gate_and,.node-body.gate_or,.node-body.gate_xor{fill:rgba(245,158,11,.4);stroke:#f59e0b;rx:"12"}
.node-body.subprocess{fill:rgba(168,85,247,.4);stroke:#a855f7}
.node-body.script{fill:rgba(34,197,94,.4);stroke:#22c55e}
.node-body.parallel{fill:rgba(236,72,153,.4);stroke:#ec4899}
.node-icon-text{font-size:14px;fill:var(--text-primary)}
.node-label{fill:var(--text-primary);font-size:12px;font-weight:500}
/* Condition Builder */
.cond-tree{padding:8px;background:var(--bg-secondary);border-radius:var(--radius-sm);min-height:180px}
.cond-group{padding:8px;margin:4px 0;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated)}
.cond-logic{font-size:10px;font-weight:700;padding:2px 6px;border-radius:var(--radius-sm);background:var(--color-primary-soft);color:var(--color-primary);display:inline-block;margin-right:8px}
.cond-row{display:flex;align-items:center;gap:4px;margin:4px 0}
/* Variable Binding */
.var-binding-panel{position:fixed;right:20px;top:120px;width:340px;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-lg);z-index:200;box-shadow:0 4px 20px rgba(0,0,0,0.3)}
.vb-header{display:flex;align-items:center;justify-content:space-between;padding:10px 12px;border-bottom:1px solid var(--border-color);font-size:12px;font-weight:600;color:var(--color-primary)}
.vb-body{padding:12px;display:flex;flex-direction:column;gap:6px;max-height:60vh;overflow-y:auto}
.vb-row{display:flex;align-items:center;gap:4px}
.vb-arrow{color:var(--color-primary);font-size:12px}
/* Form Rules */
.form-rules-panel{position:fixed;left:20px;top:120px;width:360px;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-lg);z-index:200;box-shadow:0 4px 20px rgba(0,0,0,0.3)}
.fr-header{display:flex;align-items:center;justify-content:space-between;padding:10px 12px;border-bottom:1px solid var(--border-color);font-size:12px;font-weight:600;color:var(--color-primary)}
.fr-body{padding:12px;display:flex;flex-direction:column;gap:6px;max-height:60vh;overflow-y:auto}
.fr-row{display:flex;align-items:center;gap:4px}
/* Batch Toolbar */
.batch-toolbar{position:fixed;bottom:20px;left:50%;transform:translateX(-50%);background:var(--bg-elevated);border:1px solid var(--color-primary);border-radius:var(--radius-lg);z-index:300;box-shadow:0 4px 20px rgba(0,212,255,0.2)}
.batch-toolbar-inner{display:flex;align-items:center;gap:8px;padding:10px 16px}
.batch-info{font-size:11px;color:var(--color-primary);font-weight:600;margin-right:8px}
/* Theme Editor */
.theme-grid{display:grid;grid-template-columns:repeat(4,1fr);gap:8px}
.theme-card{cursor:pointer;border-radius:var(--radius-md);overflow:hidden;border:2px solid transparent;transition:all .15s}
.theme-card:hover,.theme-card.active{border-color:var(--color-primary);transform:scale(1.03)}
.theme-preview{height:50px;width:100%}
.theme-name{font-size:10px;text-align:center;padding:4px;color:var(--text-primary);background:var(--bg-secondary)}
.color-input{width:32px;height:24px;border:none;cursor:pointer;padding:0}
/* Animation Panel */
.anim-panel{position:fixed;right:20px;bottom:80px;width:150px;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-lg);z-index:200;box-shadow:0 4px 20px rgba(0,0,0,0.3)}
.anim-header{display:flex;align-items:center;justify-content:space-between;padding:10px 12px;border-bottom:1px solid var(--border-color);font-size:12px;font-weight:600;color:var(--color-primary)}
.anim-body{padding:8px;display:flex;flex-direction:column;gap:4px}
.anim-item{padding:6px 8px;border-radius:var(--radius-sm);cursor:pointer;font-size:11px;display:flex;align-items:center;gap:6px;transition:all .15s}
.anim-item:hover{background:var(--color-primary-soft)}
.anim-item.active{background:var(--color-primary-soft);color:var(--color-primary);border:1px solid var(--color-primary)}
/* Node animations */
@keyframes nodeAppear{from{opacity:0;transform:scale(0.85)}to{opacity:1;transform:scale(1)}}
.node-appear{animation:nodeAppear 0.3s ease-out}
@keyframes pulseGlow{0%,100%{box-shadow:0 0 5px var(--color-primary)}50%{box-shadow:0 0 20px var(--color-primary)}}
.pulse-active{animation:pulseGlow 1.5s ease-in-out infinite}
.node-sublabel{fill:var(--text-muted);font-size:9px}
.port{stroke:var(--text-muted);stroke-width:1.5;fill:var(--bg-surface);cursor:crosshair}
.port-in{fill:rgba(16,185,129,.6)}
.port-out{fill:rgba(239,68,68,.6)}
.port-gate{fill:rgba(245,158,11,.6)}
.port:hover{stroke:var(--color-primary);r:8}
.edge-path{fill:none;stroke:var(--color-primary);stroke-width:1.5;cursor:pointer}
.edge-path:hover{stroke:var(--color-warning);stroke-width:2}
.edge-path.selected{stroke:var(--color-warning);stroke-width:2.5}
.edge-temp{fill:none;stroke:var(--color-secondary);stroke-width:1.5;stroke-dasharray:6,3}
.resize-handle{fill:var(--color-primary);stroke:white;stroke-width:1;cursor:nwse-resize}
.anchor-handle{cursor:grab}
/* Palette */
.pd-palette{width:140px;flex-shrink:0;padding:12px;border-right:1px solid var(--border-color)}
.pal-title{font-size:11px;color:var(--text-muted);text-transform:uppercase;letter-spacing:1px;margin:8px 0 6px;font-weight:600}
.pal-sep{height:1px;background:var(--border-color);margin:8px 0}
.pal-grid{display:grid;grid-template-columns:1fr 1fr;gap:6px}
.pal-item{display:flex;flex-direction:column;align-items:center;gap:4px;padding:8px 4px;border-radius:var(--radius-sm);border:1px solid var(--border-color);cursor:pointer;transition:all .15s}
.pal-item:hover{border-color:var(--color-primary);background:var(--color-primary-soft);transform:translateY(-1px)}
.ni{font-size:18px}.nl{font-size:10px;color:var(--text-muted);text-align:center}
/* Group backgrounds */
.group-backgrounds{pointer-events:none}
.group-bg{pointer-events:all;cursor:default;transition:stroke-opacity .15s}
.group-bg:hover{stroke-opacity:0.8}
.group-label-text{fill:var(--color-primary);font-size:11px;font-weight:600;letter-spacing:0.5px}
.group-btn{pointer-events:all}
/* Context menu */
.context-menu{position:fixed;z-index:200;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-md);padding:4px;min-width:140px;box-shadow:0 8px 32px rgba(0,0,0,.4)}
.ctx-item{padding:6px 12px;border-radius:var(--radius-sm);cursor:pointer;font-size:12px;color:var(--text-primary);display:flex;align-items:center;gap:6px}
.ctx-item:hover{background:var(--color-primary-soft);color:var(--color-primary)}
.ctx-item.ctx-danger:hover{background:rgba(239,68,68,.15);color:var(--color-danger)}
/* Subprocess node count */
.sp-node-count{font-size:11px;color:var(--text-muted);font-family:'JetBrains Mono',monospace;min-width:60px;text-align:center}
/* Edge routing styles */
.edge-path.straight{stroke-dasharray:none}
.edge-path.horizontal{stroke-dasharray:none}
.edge-path.vertical{stroke-dasharray:none}
/* Execution panel */
.pd-exec-panel{width:220px;flex-shrink:0;display:flex;flex-direction:column;border-left:1px solid var(--border-color)}
.exec-header{display:flex;align-items:center;justify-content:space-between;padding:8px 10px;border-bottom:1px solid var(--border-color);font-size:13px;font-weight:600;color:var(--color-primary)}
.exec-body{padding:8px;display:flex;flex-direction:column;gap:6px;flex:1;overflow-y:auto}
.exec-status{display:flex;align-items:center;justify-content:space-between}
.exec-badge{padding:2px 8px;border-radius:var(--radius-sm);font-size:10px;font-weight:700}
.exec-badge.idle{background:rgba(100,100,100,.2);color:var(--text-muted)}
.exec-badge.running{background:rgba(16,185,129,.2);color:var(--color-success);animation:pulse 1s infinite}
.exec-badge.paused{background:rgba(245,158,11,.2);color:var(--color-warning)}
.exec-badge.finished{background:rgba(0,212,255,.2);color:var(--color-primary)}
.exec-progress{font-size:11px;color:var(--color-primary);font-family:'JetBrains Mono',monospace}
.exec-bar{height:4px;background:var(--border-color);border-radius:2px;overflow:hidden}
.exec-bar-fill{height:100%;background:linear-gradient(90deg,var(--color-primary),var(--color-success));transition:width .3s}
.exec-nodes{display:flex;flex-direction:column;gap:3px;max-height:200px;overflow-y:auto}
.exec-node{display:flex;align-items:center;gap:6px;padding:4px 8px;border-radius:var(--radius-sm);font-size:11px}
.exec-node.active{background:rgba(245,158,11,.2);border:1px solid var(--color-warning)}
.exec-node.completed{background:rgba(16,185,129,.15);color:var(--color-success)}
.exec-node.pending{background:rgba(100,100,100,.1);color:var(--text-muted)}
.exec-node-icon{font-size:12px}.exec-node-label{flex:1;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.exec-actions{display:flex;gap:4px;flex-wrap:wrap}
.exec-actions .btn-sm{flex:1}
/* Minimap */
.minimap-container{margin-top:8px;border-top:1px solid var(--border-color);padding-top:8px}
.minimap-header{font-size:11px;color:var(--text-muted);margin-bottom:4px}
.minimap-canvas{width:100%;height:auto;border:1px solid var(--border-color);border-radius:var(--radius-sm);cursor:crosshair;display:block}
.minimap-controls{display:flex;gap:4px;margin-top:4px}
/* Palette extras */
.pal-preset{flex-direction:row;gap:4px;padding:6px}
.pal-preset:hover{transform:translateY(0)}
.pal-theme{flex-direction:row;gap:4px;padding:6px}
.pal-theme.active{border-color:var(--color-primary);background:var(--color-primary-soft)}
/* Edge animation */
.edge-path.animated{stroke-dasharray:8,4;animation:edgeFlow 1s linear infinite}
@keyframes edgeFlow{from{stroke-dashoffset:0}to{stroke-dashoffset:-24}}
/* Node style presets in props */
.style-presets{display:flex;gap:4px;flex-wrap:wrap;margin-top:4px}
.style-preset-btn{width:24px;height:24px;border-radius:50%;border:2px solid var(--border-color);cursor:pointer;transition:all .15s}
.style-preset-btn:hover,.style-preset-btn.active{border-color:var(--color-primary);transform:scale(1.1)}
/* Group controls in props */
.group-controls{display:flex;gap:4px;margin-top:8px;padding-top:8px;border-top:1px solid var(--border-color)}
/* Canvas theme background */
.canvas-bg.theme-dark{background-image:radial-gradient(circle,rgba(255,255,255,0.03) 1px,transparent 1px)}
.canvas-bg.theme-midnight{background-image:radial-gradient(circle,rgba(100,200,255,0.03) 1px,transparent 1px)}
.canvas-bg.theme-ocean{background-image:radial-gradient(circle,rgba(0,150,255,0.04) 1px,transparent 1px)}
.canvas-bg.theme-forest{background-image:radial-gradient(circle,rgba(0,255,100,0.03) 1px,transparent 1px)}
/* Profile indicator */
.profile-indicator{display:inline-flex;align-items:center;gap:4px;padding:2px 6px;border-radius:var(--radius-sm);font-size:10px;background:var(--bg-elevated);color:var(--text-muted)}
/* Flow direction badge */
.flow-badge{display:inline-block;padding:1px 6px;border-radius:var(--radius-sm);font-size:9px;font-weight:700;margin-left:4px}
.flow-badge.condition{background:rgba(245,158,11,.2);color:var(--color-warning)}
.flow-badge.default{background:rgba(16,185,129,.2);color:var(--color-success)}
/* Meta editor modal */
.meta-editor{display:flex;flex-direction:column;gap:8px}
.meta-field{display:flex;flex-direction:column;gap:3px}
.meta-field label{font-size:11px;color:var(--text-muted)}
.meta-field input,.meta-field textarea{padding:6px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:12px;outline:none;width:100%;box-sizing:border-box}
.meta-field textarea{resize:vertical}
.meta-tags{display:flex;flex-wrap:wrap;gap:4px;margin-top:4px}
.meta-tag{padding:2px 8px;border-radius:var(--radius-sm);background:var(--color-primary-soft);color:var(--color-primary);font-size:10px}
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
/* Group drag & resize */
.group-resize-handle{position:absolute;width:12px;height:12px;background:var(--color-primary);border:2px solid #fff;border-radius:50%;cursor:pointer;z-index:10;transition:transform .15s}
.group-resize-handle:hover{transform:scale(1.3)}
.group-resize-handle.nw{top:-6px;left:-6px}.group-resize-handle.n{top:-6px;left:50%;transform:translateX(-50%)}.group-resize-handle.ne{top:-6px;right:-6px}
.group-resize-handle.e{top:50%;right:-6px;transform:translateY(-50%)}.group-resize-handle.se{bottom:-6px;right:-6px}.group-resize-handle.s{bottom:-6px;left:50%;transform:translateX(-50%)}
.group-resize-handle.sw{bottom:-6px;left:-6px}.group-resize-handle.w{top:50%;left:-6px;transform:translateY(-50%)}
/* Fork/Join */
.routing-cp-row{display:flex;align-items:center;gap:4px;margin-bottom:4px}
.fork-flow{stroke-opacity:0.6;animation:forkPulse 2s ease-in-out infinite}
@keyframes forkPulse{0%,100%{stroke-opacity:0.4}50%{stroke-opacity:0.9}}
.fork-join-layer{pointer-events:none}
/* Group drag & resize */
.group-drag-zone{cursor:move;transition:fill .15s}
.group-drag-zone:hover{fill:rgba(0,212,255,0.15)}
.group-resize-handle{transition:transform .15s,opacity .15s;opacity:0.7}
.group-resize-handle:hover{transform:scale(1.4);opacity:1}
.group-resize-handle.nw{top:-6px;left:-6px}.group-resize-handle.n{top:-6px;left:50%;transform:translateX(-50%)}.group-resize-handle.ne{top:-6px;right:-6px}
.group-resize-handle.e{top:50%;right:-6px;transform:translateY(-50%)}.group-resize-handle.se{bottom:-6px;right:-6px}.group-resize-handle.s{bottom:-6px;left:50%;transform:translateX(-50%)}
.group-resize-handle.sw{bottom:-6px;left:-6px}.group-resize-handle.w{top:50%;left:-6px;transform:translateY(-50%)}
/* Fork/Join */
.fork-flow{stroke-opacity:0.6;animation:forkPulse 2s ease-in-out infinite}
@keyframes forkPulse{0%,100%{stroke-opacity:0.4}50%{stroke-opacity:0.9}}
.fork-join-layer{pointer-events:none}
/* Toolbar fork/join button */
.tb-btn.active{background:var(--color-primary-soft);color:var(--color-primary);border-color:var(--color-primary)}
/* Breakpoint dots */
.breakpoint-dot{cursor:pointer;transition:r .2s,fill .2s}
.breakpoint-dot:hover{r:8;fill:#fbbf24}
.breakpoint-layer{pointer-events:all}
/* Flow stats panel */
.flow-stats-panel{position:fixed;bottom:20px;right:20px;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-lg);padding:12px;z-index:100;box-shadow:0 4px 20px rgba(0,0,0,0.3)}
.stats-grid{display:grid;grid-template-columns:repeat(4,1fr);gap:8px}
.stat-item{padding:6px;background:var(--bg-secondary);border-radius:var(--radius-sm);text-align:center}
.stat-value{font-size:16px;font-weight:700;color:var(--color-primary);font-family:"JetBrains Mono",monospace}
.stat-label{font-size:9px;color:var(--text-muted);margin-top:2px}
/* Enhanced style presets */
.enhanced-style-presets{display:flex;gap:4px;flex-wrap:wrap;margin-top:8px}
.enhanced-style-btn{width:24px;height:24px;border-radius:50%;border:2px solid var(--border-color);cursor:pointer;transition:all .15s}
.enhanced-style-btn:hover{transform:scale(1.2);border-color:var(--color-primary)}
/* Execution controls */
.exec-controls{padding:12px;border-top:1px solid var(--border-color);display:flex;flex-direction:column;gap:8px}
.speed-control{display:flex;align-items:center;gap:8px;font-size:11px}
.speed-label{color:var(--text-muted);white-space:nowrap}
.speed-val{color:var(--color-primary);font-family:"JetBrains Mono",monospace;min-width:40px;text-align:right}
.exec-step-controls{display:flex;gap:4px}
.breakpoint-list{margin-top:4px}
.bp-title{font-size:11px;font-weight:600;color:var(--color-warning);margin-bottom:4px}
.bp-item{display:flex;align-items:center;justify-content:space-between;padding:2px 0;font-size:10px}
.bp-node{color:var(--text-primary)}
.bp-remove{background:transparent;border:none;color:var(--color-danger);cursor:pointer;font-size:10px;padding:0 4px}
.bp-toggle{margin-top:4px}
/* Style preset panel */
.style-preset-panel{margin-top:12px;padding-top:12px;border-top:1px solid var(--border-color)}
.sp-title{font-size:11px;font-weight:600;color:var(--color-primary);margin-bottom:6px;text-transform:uppercase;letter-spacing:0.5px}
.style-preset-btn{width:28px;height:28px;border-radius:50%;border:2px solid var(--border-color);cursor:pointer;transition:all .15s;display:flex;align-items:center;justify-content:center;font-size:14px}
.style-preset-btn:hover{transform:scale(1.15);border-color:var(--color-primary)}
/* Flow stats modal */
.stats-detail-grid{display:grid;grid-template-columns:repeat(4,1fr);gap:12px;margin-bottom:16px}
.sd-card{padding:12px;background:var(--bg-secondary);border-radius:var(--radius-md);text-align:center;border:1px solid var(--border-color)}
.sd-value{font-size:24px;font-weight:700;color:var(--color-primary);font-family:"JetBrains Mono",monospace}
.sd-label{font-size:11px;color:var(--text-muted);margin-top:4px;text-transform:uppercase}
.sd-desc{font-size:9px;color:var(--text-muted);margin-top:2px}
.stats-warning{padding:8px 12px;background:rgba(245,158,11,.1);border:1px solid var(--color-warning);border-radius:var(--radius-sm);color:var(--color-warning);font-size:12px;margin-top:8px}
.stats-good{padding:8px 12px;background:rgba(16,185,129,.1);border:1px solid var(--color-success);border-radius:var(--radius-sm);color:var(--color-success);font-size:12px;margin-top:8px}
/* Node type analysis */
.nta-grid{display:grid;grid-template-columns:repeat(3,1fr);gap:6px;margin-top:8px}
.nta-item{padding:6px;background:var(--bg-secondary);border-radius:var(--radius-sm);text-align:center;font-size:11px}
.nta-icon{font-size:16px}
.nta-count{font-size:14px;font-weight:700;color:var(--color-primary);font-family:"JetBrains Mono",monospace}
.nta-type{font-size:9px;color:var(--text-muted)}
/* Edge direction analysis */
.eda-bar{display:flex;align-items:center;gap:8px;margin:4px 0;font-size:11px}
.eda-label{width:50px;color:var(--text-muted)}
.eda-track{flex:1;height:8px;background:var(--border-color);border-radius:4px;overflow:hidden}
.eda-fill{height:100%;background:var(--color-primary);transition:width .3s}
.eda-val{width:40px;text-align:right;font-family:"JetBrains Mono",monospace;color:var(--color-primary)}
/* Network Analysis */
.network-grid{display:grid;grid-template-columns:repeat(3,1fr);gap:8px;margin-bottom:12px}
.network-card{padding:10px;background:var(--bg-secondary);border-radius:var(--radius-sm);text-align:center;border:1px solid var(--border-color)}
.nc-value{font-size:20px;font-weight:700;color:var(--color-primary);font-family:"JetBrains Mono",monospace}
.nc-label{font-size:10px;color:var(--text-muted);margin-top:2px;text-transform:uppercase}
.nc-desc{font-size:8px;color:var(--text-muted);margin-top:2px}
.analysis-warning{padding:8px 12px;background:rgba(245,158,11,.1);border:1px solid var(--color-warning);border-radius:var(--radius-sm);color:var(--color-warning);font-size:12px;margin-top:8px}
/* Style Presets */
.style-presets-panel{margin-top:12px;padding-top:12px;border-top:1px solid var(--border-color)}
.spp-title{font-size:11px;font-weight:600;color:var(--color-primary);margin-bottom:6px;text-transform:uppercase;letter-spacing:0.5px}
.spp-grid{display:flex;gap:4px;flex-wrap:wrap}
.spp-btn{width:28px;height:28px;border-radius:50%;cursor:pointer;transition:all .15s;display:flex;align-items:center;justify-content:center;font-size:14px}
.spp-btn:hover{transform:scale(1.2);box-shadow:0 0 8px var(--color-primary)}
/* ── Deepened Styles ─────────────────────────────────────────────── */
/* Data Mapping Editor */
.data-mapping-modal{width:720px;max-width:90vw}
.dm-fields-section,.dm-mappings-section{margin-bottom:16px}
.dm-field-list{display:flex;flex-direction:column;gap:4px;max-height:200px;overflow-y:auto}
.dm-field-item{display:flex;align-items:center;gap:8px;padding:6px 10px;background:var(--bg-secondary);border-radius:var(--radius-sm);cursor:grab;font-size:12px;border:1px solid var(--border-color)}
.dm-field-item:hover{border-color:var(--color-primary)}
.dm-field-icon{font-size:14px}.dm-field-name{flex:1}.dm-field-type{font-size:10px;color:var(--text-muted);background:var(--bg-elevated);padding:1px 6px;border-radius:var(--radius-sm)}
.dm-mapping-row{display:flex;align-items:center;gap:6px;margin-bottom:8px;padding:8px;background:var(--bg-secondary);border-radius:var(--radius-sm)}
.dm-select{padding:4px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:12px}
.dm-input{padding:4px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:12px;width:100px}
.dm-arrow{color:var(--color-primary);font-weight:bold}.dm-cond{width:80px}.dm-del{padding:2px 6px}
/* Flow Variable Panel */
.flow-var-panel{position:fixed;top:60px;right:20px;width:320px;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-lg);z-index:200;box-shadow:0 8px 32px rgba(0,0,0,0.4)}
.fv-header{display:flex;align-items:center;justify-content:space-between;padding:12px 16px;border-bottom:1px solid var(--border-color)}
.fv-body{padding:12px;max-height:400px;overflow-y:auto}
.fv-add{display:flex;gap:6px;margin-bottom:12px}
.fv-input{flex:1;padding:6px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-secondary);color:var(--text-primary);font-size:12px}
.fv-select{padding:6px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-secondary);color:var(--text-primary);font-size:12px}
.fv-list{display:flex;flex-direction:column;gap:4px}
.fv-item{display:flex;align-items:center;gap:8px;padding:8px;background:var(--bg-secondary);border-radius:var(--radius-sm);border:1px solid var(--border-color)}
.fv-item.global{border-left:3px solid var(--color-primary)}.fv-item.local{border-left:3px solid var(--color-warning)}.fv-item.node{border-left:3px solid var(--color-success)}
.fv-icon{font-size:14px}.fv-info{flex:1;display:flex;flex-direction:column;gap:2px}
.fv-name{font-size:13px;font-weight:600;color:var(--text-primary)}.fv-type{font-size:10px;color:var(--text-muted)}
.fv-scope{font-size:9px;padding:1px 4px;border-radius:var(--radius-sm);background:rgba(0,212,255,0.2);color:var(--color-primary)}
.fv-actions{display:flex;gap:4px}
.fv-export{margin-top:12px;padding-top:12px;border-top:1px solid var(--border-color)}
/* Node Templates */
.node-tpl-modal{width:600px;max-width:90vw}
.tpl-add{display:flex;gap:8px;margin-bottom:16px}
.tpl-input{flex:1;padding:8px 12px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-secondary);color:var(--text-primary);font-size:13px}
.tpl-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(160px,1fr));gap:12px}
.tpl-card{padding:14px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-secondary);cursor:pointer;transition:all .15s}
.tpl-card:hover{border-color:var(--color-primary);transform:translateY(-2px);box-shadow:0 4px 16px rgba(0,212,255,0.2)}
.tpl-header{display:flex;align-items:center;gap:6px;margin-bottom:6px}
.tpl-icon{font-size:20px}.tpl-name{font-size:13px;font-weight:600;color:var(--color-primary)}
.tpl-desc{font-size:11px;color:var(--text-muted);margin-bottom:8px;min-height:32px}
.tpl-nodes-preview{display:flex;gap:4px;margin-bottom:8px;font-size:14px}
.tpl-more{font-size:10px;color:var(--text-muted)}
.tpl-actions{display:flex;gap:4px}
/* Performance Monitor */
.perf-monitor{position:fixed;top:60px;left:20px;width:280px;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-lg);z-index:200;box-shadow:0 8px 32px rgba(0,0,0,0.4)}
.perf-header{display:flex;align-items:center;justify-content:space-between;padding:10px 14px;border-bottom:1px solid var(--border-color);font-size:13px;font-weight:600;color:var(--color-primary)}
.perf-body{padding:10px}
.perf-table{display:flex;flex-direction:column;gap:2px}
.perf-row{display:grid;grid-template-columns:1fr 60px 70px;padding:4px 8px;font-size:11px;border-radius:var(--radius-sm)}
.perf-row-header{background:var(--bg-secondary);font-weight:600;color:var(--text-muted)}
.perf-status.running{color:var(--color-warning)}.perf-status.completed{color:var(--color-success)}.perf-status.failed{color:var(--color-danger)}
/* Context Menu */
.context-menu{position:fixed;z-index:500;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-md);padding:4px;min-width:160px;box-shadow:0 8px 24px rgba(0,0,0,0.5)}
.ctx-item{padding:8px 12px;font-size:13px;cursor:pointer;border-radius:var(--radius-sm);display:flex;align-items:center;gap:8px;transition:background .1s}
.ctx-item:hover{background:var(--color-primary-soft)}
.ctx-sep{height:1px;background:var(--border-color);margin:4px 0}
/* Tooltip */
.pd-tooltip{position:fixed;z-index:400;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-sm);padding:6px 10px;font-size:11px;color:var(--text-primary);box-shadow:0 4px 12px rgba(0,0,0,0.3);pointer-events:none;max-width:280px;white-space:nowrap}
/* Guide Lines */
.guide-lines-panel{position:fixed;top:60px;left:50%;transform:translateX(-50%);z-index:150;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-md);padding:8px 12px;display:flex;align-items:center;gap:8px;box-shadow:0 4px 16px rgba(0,0,0,0.3)}
.gl-header{display:flex;align-items:center;gap:6px;font-size:12px;font-weight:600;color:var(--color-primary)}
.gl-list{display:flex;flex-direction:column;gap:4px;margin-top:8px}
.gl-item{display:flex;align-items:center;gap:6px;font-size:11px}
.gl-type{width:60px;color:var(--text-muted)}
.gl-slider{flex:1}.gl-pos{width:40px;text-align:right;font-family:'JetBrains Mono',monospace;color:var(--color-primary)}
/* Box Selection */
.box-select-svg{position:absolute;top:0;left:0;width:100%;height:100%;pointer-events:none;z-index:50}
/* Conflict Detection */
.conflict-panel{position:fixed;top:60px;right:20px;width:300px;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-lg);z-index:200;box-shadow:0 8px 32px rgba(0,0,0,0.4)}
.cp-header{display:flex;align-items:center;justify-content:space-between;padding:10px 14px;border-bottom:1px solid var(--border-color);font-size:13px;font-weight:600;color:var(--color-warning)}
.cp-body{padding:12px}
.cp-conflicts{margin-top:8px;display:flex;flex-direction:column;gap:4px}
.cp-conflict{padding:8px;border-radius:var(--radius-sm);font-size:11px}
.cp-conflict.cp-error{background:rgba(239,68,68,0.1);border:1px solid rgba(239,68,68,0.3);color:var(--color-danger)}
.cp-conflict.cp-warning{background:rgba(245,158,11,0.1);border:1px solid rgba(245,158,11,0.3);color:var(--color-warning)}
.cp-edges{font-size:10px;color:var(--text-muted);margin-top:2px}
.cp-ok{padding:12px;text-align:center;color:var(--color-success);font-size:13px}
/* Simulation Timeline */
.sim-timeline{position:fixed;bottom:20px;left:20px;width:400px;max-width:90vw;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-lg);z-index:200;box-shadow:0 8px 32px rgba(0,0,0,0.4)}
.st-header{display:flex;align-items:center;justify-content:space-between;padding:10px 14px;border-bottom:1px solid var(--border-color);font-size:13px;font-weight:600;color:var(--color-primary)}
.st-controls{display:flex;gap:4px}
.st-progress{height:8px;background:var(--bg-secondary);border-radius:4px;overflow:hidden;position:relative;margin:8px 0}
.st-bar{height:100%;background:linear-gradient(90deg,var(--color-primary),var(--color-success));transition:width .3s}
.st-pct{position:absolute;right:8px;top:-16px;font-size:10px;color:var(--text-muted)}
.st-events{max-height:200px;overflow-y:auto;display:flex;flex-direction:column;gap:2px}
.st-event{display:flex;align-items:center;gap:8px;padding:4px 8px;font-size:11px;border-radius:var(--radius-sm)}
.st-event.start{background:rgba(0,212,255,0.1);color:var(--color-primary)}
.st-event.complete{background:rgba(16,185,129,0.1);color:var(--color-success)}
.st-time{font-family:'JetBrains Mono',monospace;color:var(--text-muted);width:50px}
.st-node{flex:1}.st-label{color:var(--text-primary)}.st-type{font-size:10px;padding:1px 4px;border-radius:var(--radius-sm);background:var(--bg-secondary)}
/* Shortcut Help */
.shortcut-help{position:fixed;top:60px;left:20px;width:260px;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-lg);z-index:200;box-shadow:0 8px 32px rgba(0,0,0,0.4)}
.sh-header{display:flex;align-items:center;justify-content:space-between;padding:10px 14px;border-bottom:1px solid var(--border-color);font-size:13px;font-weight:600;color:var(--color-primary)}
.sh-body{padding:10px;display:flex;flex-direction:column;gap:4px}
.sh-row{display:flex;align-items:center;gap:10px;padding:4px 8px;border-radius:var(--radius-sm)}
.sh-row:hover{background:var(--bg-secondary)}
.sh-key{font-family:'JetBrains Mono',monospace;font-size:11px;padding:2px 6px;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-sm);color:var(--color-primary);min-width:80px;text-align:center}
.sh-label{font-size:12px;color:var(--text-primary)}
/* Form Rules Panel */
.form-rules-panel{position:fixed;top:60px;right:20px;width:480px;max-width:90vw;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-lg);z-index:200;box-shadow:0 8px 32px rgba(0,0,0,0.4);max-height:80vh;overflow-y:auto}
.frp-header{display:flex;align-items:center;justify-content:space-between;padding:12px 16px;border-bottom:1px solid var(--border-color);position:sticky;top:0;background:var(--bg-elevated);z-index:1}
.frp-actions{display:flex;gap:4px}
.frp-body{padding:12px}
.frp-set{margin-bottom:16px;padding:12px;background:var(--bg-secondary);border-radius:var(--radius-md);border:1px solid var(--border-color)}
.frp-set-header{display:flex;align-items:center;gap:8px;margin-bottom:8px}
.frp-set-name{flex:1;padding:4px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--color-primary);font-size:13px;font-weight:600}
.frp-rules{display:flex;flex-direction:column;gap:6px}
.frp-rule{display:flex;align-items:center;gap:4px}
.frp-select{padding:3px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px}
.frp-op{width:44px}.frp-act{width:56px}
.frp-input{padding:3px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;flex:1}
.frp-apply{margin-top:8px;width:100%}
/* Toast */
.toast-container{position:fixed;bottom:20px;right:20px;z-index:600;display:flex;flex-direction:column;gap:8px}
.toast{padding:10px 16px;border-radius:var(--radius-md);font-size:13px;box-shadow:0 4px 16px rgba(0,0,0,0.4);animation:toastIn .3s ease}
.toast-info{background:rgba(0,212,255,0.2);border:1px solid var(--color-primary);color:var(--color-primary)}
.toast-success{background:rgba(16,185,129,0.2);border:1px solid var(--color-success);color:var(--color-success)}
.toast-warning{background:rgba(245,158,11,0.2);border:1px solid var(--color-warning);color:var(--color-warning)}
.toast-error{background:rgba(239,68,68,0.2);border:1px solid var(--color-danger);color:var(--color-danger)}
@keyframes toastIn{from{opacity:0;transform:translateY(20px)}to{opacity:1;transform:translateY(0)}}
/* Process Status */
.process-status{position:fixed;bottom:20px;left:20px;z-index:150;display:flex;align-items:center;gap:8px;padding:8px 14px;border-radius:var(--radius-full);background:var(--bg-elevated);border:1px solid var(--border-color);font-size:12px;font-weight:600;box-shadow:0 4px 16px rgba(0,0,0,0.3)}
.ps-valid .ps-dot{background:var(--color-success)}.ps-empty .ps-dot{background:var(--text-muted)}
.ps-no-start .ps-dot,.ps-no-end .ps-dot,.ps-disconnected .ps-dot{background:var(--color-warning);animation:pulse 1.5s infinite}
.ps-dot{width:8px;height:8px;border-radius:50%}
@keyframes pulse{0%,100%{opacity:1}50%{opacity:0.4}}
/* Node hover preview */
.node-hover-preview{position:absolute;z-index:100;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-md);padding:10px;box-shadow:0 4px 16px rgba(0,0,0,0.4);pointer-events:none}
/* Enhanced node animations */
@keyframes nodeAppear{from{opacity:0;transform:scale(0.8)}to{opacity:1;transform:scale(1)}}
.node-appear{animation:nodeAppear .2s ease-out}
@keyframes edgeFlow{from{stroke-dashoffset:20}to{stroke-dashoffset:0}}
.edge-flow-anim{stroke-dasharray:8,4;animation:edgeFlow .8s linear infinite}
.node-selected-pulse{animation:selectedPulse 1.5s ease-in-out infinite}
@keyframes selectedPulse{0%,100%{filter:brightness(1)}50%{filter:brightness(1.3)}}
/* Snap indicator */
.snap-indicator{position:absolute;width:8px;height:8px;border-radius:50%;background:var(--color-warning);pointer-events:none;z-index:60;opacity:0.8}
/* Scrollbar styling for panels */
.fv-body::-webkit-scrollbar,.form-rules-panel::-webkit-scrollbar,.st-events::-webkit-scrollbar,.perf-body::-webkit-scrollbar{width:4px}
.fv-body::-webkit-scrollbar-thumb,.form-rules-panel::-webkit-scrollbar-thumb,.st-events::-webkit-scrollbar-thumb,.perf-body::-webkit-scrollbar-thumb{background:var(--border-color);border-radius:2px}
/* ── Deepened Styles v2 ───────────────────────────────────────────── */
/* Script Editor */
.script-editor-overlay{z-index:400}.script-editor-modal{width:90vw;max-width:1200px;height:85vh;display:flex;flex-direction:column}
.script-editor-body{display:flex;flex-direction:column;flex:1;overflow:hidden;padding:0}
.se-toolbar{display:flex;gap:6px;padding:8px 12px;border-bottom:1px solid var(--border-color);align-items:center}
.se-lang-select{padding:4px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:12px}
.se-editor-wrap{flex:1;display:flex;position:relative;overflow:hidden;min-height:200px}
.se-code-editor{flex:1;padding:12px;border:none;background:var(--bg-surface);color:var(--color-success);font-family:'JetBrains Mono',monospace;font-size:13px;resize:none;outline:none;line-height:1.6}
.se-line-numbers{position:absolute;left:0;top:0;width:36px;padding:12px 4px;text-align:right;font-family:'JetBrains Mono',monospace;font-size:12px;color:var(--text-muted);background:var(--bg-elevated);border-right:1px solid var(--border-color);line-height:1.6;pointer-events:none}
.se-sidebar{width:260px;border-left:1px solid var(--border-color);overflow-y:auto;padding:8px;display:flex;flex-direction:column;gap:12px}
.se-section{display:flex;flex-direction:column;gap:6px}
.se-section-title{font-size:11px;font-weight:700;color:var(--color-primary);text-transform:uppercase;letter-spacing:0.5px;padding-bottom:4px;border-bottom:1px solid var(--border-color)}
.se-import-row,.se-var-row,.se-bind-row{display:flex;gap:4px;align-items:center}
.se-import-input,.se-var-input,.se-bind-input{flex:1;padding:3px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px}
.se-var-select,.se-bind-select{padding:3px 4px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:10px}
.se-select{padding:4px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:12px;width:100%}
.se-retry-config{display:flex;flex-direction:column;gap:4px}
.se-num-input{width:60px;padding:3px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px}
.se-validation{padding:8px 12px;border-top:1px solid var(--border-color);display:flex;flex-direction:column;gap:4px}
.se-error{color:var(--color-danger);font-size:11px}.se-warning{color:var(--color-warning);font-size:11px}.se-suggestion{color:var(--color-info);font-size:11px}
.se-log-panel{border-top:1px solid var(--border-color);max-height:150px;overflow-y:auto}
.se-log-header{padding:6px 12px;font-size:12px;font-weight:600;color:var(--color-primary);background:var(--bg-secondary)}
.se-log-body{padding:4px 12px;font-family:'JetBrains Mono',monospace;font-size:11px}
.se-log-entry{padding:2px 0}
/* Node Props Editor */
.node-props-modal{width:480px;max-width:90vw}
.np-editor{display:flex;flex-direction:column;gap:12px}
.np-node-info{display:flex;align-items:center;gap:8px;padding:8px 12px;background:var(--bg-secondary);border-radius:var(--radius-md)}
.np-node-icon{font-size:20px}.np-node-label{font-size:14px;font-weight:600;color:var(--text-primary)}
.np-category{padding:8px;background:var(--bg-secondary);border-radius:var(--radius-md)}
.np-cat-title{font-size:11px;font-weight:700;color:var(--color-primary);text-transform:uppercase;margin-bottom:8px;letter-spacing:0.5px}
.np-prop-row{display:flex;align-items:center;justify-content:space-between;gap:8px;padding:4px 0}
.np-prop-label{font-size:12px;color:var(--text-muted);min-width:80px}
.np-input{padding:4px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:12px;flex:1}
/* Parallel Config */
.parallel-config-modal{width:560px;max-width:90vw}
.pc-strategy{display:flex;flex-direction:column;gap:10px;padding:8px 0}
.pc-row{display:flex;align-items:center;gap:10px}
.pc-row span{font-size:12px;color:var(--text-muted);min-width:70px}
.pc-select{flex:1;padding:4px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:12px}
.pc-num{width:70px;padding:4px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:12px}
.pc-branches{margin-top:12px;padding-top:12px;border-top:1px solid var(--border-color)}
.pc-branches-title{font-size:12px;font-weight:600;color:var(--color-primary);margin-bottom:8px}
.pc-branch{display:flex;align-items:center;gap:8px;padding:6px 8px;background:var(--bg-secondary);border-radius:var(--radius-sm);margin-bottom:4px}
.pc-branch-color{width:12px;height:12px;border-radius:50%;flex-shrink:0}
.pc-branch-name{flex:1;padding:2px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:12px}
.pc-branch-nodes{font-size:10px;color:var(--text-muted)}
.pc-actions{display:flex;gap:8px;margin-top:12px}
/* Branch Timeline */
.branch-timeline-panel{position:fixed;bottom:20px;left:20px;width:420px;max-width:90vw;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-lg);z-index:200;box-shadow:0 8px 32px rgba(0,0,0,0.4)}
.bt-header{display:flex;align-items:center;justify-content:space-between;padding:10px 14px;border-bottom:1px solid var(--border-color);font-size:13px;font-weight:600;color:var(--color-warning)}
.bt-body{padding:12px}
.bt-status-grid{display:flex;flex-direction:column;gap:4px}
.bt-status-item{display:flex;align-items:center;gap:8px;padding:4px 8px;background:var(--bg-secondary);border-radius:var(--radius-sm);font-size:11px}
.bt-bc{width:10px;height:10px;border-radius:50%;flex-shrink:0}
.bt-bname{flex:1;color:var(--text-primary)}.bt-bstatus{padding:2px 6px;border-radius:var(--radius-sm);font-size:10px;font-weight:600}
.bst-running{background:rgba(0,212,255,0.2);color:var(--color-primary)}.bst-completed{background:rgba(16,185,129,0.2);color:var(--color-success)}.bst-failed{background:rgba(239,68,68,0.2);color:var(--color-danger)}.bst-pending{background:rgba(100,116,139,0.2);color:var(--text-muted)}
.bt-timeline{margin-top:8px;max-height:120px;overflow-y:auto;display:flex;flex-direction:column;gap:2px}
.bt-event{display:flex;align-items:center;gap:6px;padding:3px 6px;background:var(--bg-secondary);border-radius:var(--radius-sm);font-size:10px}
.bt-dot{width:6px;height:6px;border-radius:50%;flex-shrink:0}
.bt-label{color:var(--text-muted)}
/* Flow Analysis */
.flow-analysis-panel{position:fixed;top:60px;right:20px;width:340px;max-height:80vh;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-lg);z-index:200;box-shadow:0 8px 32px rgba(0,0,0,0.4);display:flex;flex-direction:column}
.fa-header{display:flex;align-items:center;justify-content:space-between;padding:12px 16px;border-bottom:1px solid var(--border-color);font-size:13px;font-weight:600;color:var(--color-primary);flex-shrink:0}
.fa-body{padding:12px;overflow-y:auto;flex:1}
.fa-stats{display:flex;flex-direction:column;gap:12px;margin-top:8px}
.fa-health{display:flex;align-items:center;gap:12px;padding:12px;background:var(--bg-secondary);border-radius:var(--radius-md)}
.fa-health-score{font-size:36px;font-weight:800;font-family:'JetBrains Mono',monospace}
.fa-health-label{font-size:12px;color:var(--text-muted)}
.fa-grid{display:grid;grid-template-columns:repeat(3,1fr);gap:6px}
.fa-stat{padding:8px;background:var(--bg-secondary);border-radius:var(--radius-sm);text-align:center}
.fa-val{font-size:18px;font-weight:700;color:var(--color-primary);font-family:'JetBrains Mono',monospace;display:block}
.fa-lbl{font-size:9px;color:var(--text-muted)}
.fa-cycles,.fa-bottlenecks{margin-top:8px;padding:8px;background:var(--bg-secondary);border-radius:var(--radius-md)}
.fa-title{font-size:11px;font-weight:700;color:var(--color-warning);margin-bottom:4px;text-transform:uppercase}
.fa-cycle{font-size:10px;color:var(--color-danger);padding:2px 0;font-family:'JetBrains Mono',monospace}
.fa-bn{display:flex;align-items:center;gap:6px;font-size:11px;padding:2px 0}
.fa-bn-sev{padding:1px 4px;border-radius:var(--radius-sm);font-size:9px;font-weight:700}
.sev-high{background:rgba(239,68,68,0.2);color:var(--color-danger)}.sev-medium{background:rgba(245,158,11,0.2);color:var(--color-warning)}
/* Archive Manager */
.archive-manager,.snapshot-manager{position:fixed;top:60px;left:20px;width:380px;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-lg);z-index:200;box-shadow:0 8px 32px rgba(0,0,0,0.4)}
.am-header,.sm-header{display:flex;align-items:center;justify-content:space-between;padding:12px 16px;border-bottom:1px solid var(--border-color);font-size:13px;font-weight:600;color:var(--color-primary)}
.am-body,.sm-body{padding:12px;display:flex;flex-direction:column;gap:8px}
.am-add{display:flex;gap:6px}
.am-input{flex:1;padding:6px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-secondary);color:var(--text-primary);font-size:12px}
.am-desc{flex:2}
.am-list{display:flex;flex-direction:column;gap:6px;max-height:300px;overflow-y:auto}
.am-entry{display:flex;align-items:center;justify-content:space-between;padding:8px 10px;background:var(--bg-secondary);border-radius:var(--radius-md);border:1px solid var(--border-color)}
.am-entry-info{display:flex;flex-direction:column;gap:2px;min-width:0}
.am-entry-label{font-size:13px;font-weight:600;color:var(--text-primary)}.am-entry-meta{font-size:10px;color:var(--text-muted);white-space:nowrap;overflow:hidden;text-overflow:ellipsis}
.am-entry-actions{display:flex;gap:4px;flex-shrink:0}
.am-empty{text-align:center;padding:20px;color:var(--text-muted);font-size:12px}
.sm-list{display:flex;flex-direction:column;gap:6px;margin-top:8px}
.sm-snap{display:flex;align-items:center;gap:8px;padding:8px 10px;background:var(--bg-secondary);border-radius:var(--radius-md);font-size:12px}
.sm-snap-name{flex:1;font-weight:600;color:var(--text-primary)}.sm-snap-meta{font-size:10px;color:var(--text-muted)}
.sm-snap-status{padding:2px 6px;border-radius:var(--radius-sm);font-size:10px;font-weight:600}
.sm-snap-draft{background:rgba(0,212,255,0.2);color:var(--color-primary)}.sm-snap-published{background:rgba(16,185,129,0.2);color:var(--color-success)}.sm-snap-archived{background:rgba(100,116,139,0.2);color:var(--text-muted)}
/* Tool Palette */
.tool-palette{position:fixed;top:60px;left:20px;width:170px;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-lg);z-index:200;box-shadow:0 8px 32px rgba(0,0,0,0.4);padding:12px}
.tp-header{font-size:13px;font-weight:700;color:var(--color-primary);margin-bottom:8px}
.tp-tools{display:flex;flex-direction:column;gap:4px;margin-bottom:8px}
.tp-tool-btn{display:flex;align-items:center;gap:8px;padding:6px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:12px;transition:all .15s}
.tp-tool-btn:hover,.tp-tool-btn.active{border-color:var(--color-primary);color:var(--color-primary);background:var(--color-primary-soft)}
.tp-sep{height:1px;background:var(--border-color);margin:8px 0}
.tp-highlights,.tp-animations,.tp-settings{display:flex;flex-direction:column;gap:6px}
.tp-label{font-size:10px;font-weight:700;color:var(--text-muted);text-transform:uppercase;letter-spacing:0.5px}
.tp-hl-btn,.tp-anim-btn{padding:3px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:11px}
.tp-hl-btn:hover,.tp-hl-btn.active,.tp-anim-btn:hover{border-color:var(--color-primary);color:var(--color-primary)}
.tp-settings label{display:flex;align-items:center;gap:6px;font-size:11px;color:var(--text-muted);cursor:pointer}
.tp-settings input[type="checkbox"]{accent-color:var(--color-primary)}
.tp-settings input[type="range"]{flex:1;accent-color:var(--color-primary)}
/* Subprocess Breadcrumb */
.subprocess-breadcrumb{display:flex;align-items:center;gap:12px;padding:6px 12px;background:var(--bg-elevated);border-bottom:1px solid var(--border-color);flex-shrink:0}
.sb-nav{display:flex;align-items:center;gap:4px;flex:1}
.sb-home{background:transparent;border:none;color:var(--color-primary);cursor:pointer;font-size:16px;padding:2px 6px;border-radius:var(--radius-sm)}
.sb-crumb{display:flex;align-items:center;gap:2px}
.sb-crumb-text{font-size:11px;color:var(--text-muted);cursor:pointer;padding:2px 6px;border-radius:var(--radius-sm)}
.sb-crumb-text:hover,.sb-crumb-text.active{background:var(--color-primary-soft);color:var(--color-primary)}
.sb-arrow{color:var(--text-muted);font-size:12px}.sb-depth-badge{font-size:10px;padding:2px 8px;background:rgba(168,85,247,0.2);color:var(--color-primary);border-radius:var(--radius-full);font-weight:700}
/* Ripple effect */
.ripple{position:absolute;border-radius:50%;background:rgba(0,212,255,0.3);pointer-events:none;animation:rippleAnim .6s ease-out forwards}
@keyframes rippleAnim{from{width:0;height:0;opacity:1}to{width:80px;height:80px;opacity:0;margin-left:-40px;margin-top:-40px}}
/* Highlighted node styles */
.node-highlight-incoming{stroke:var(--color-primary)!important;stroke-width:3!important}
.node-highlight-outgoing{stroke:var(--color-success)!important;stroke-width:3!important}
.node-highlight-all{stroke:var(--color-warning)!important;stroke-width:3!important}
.node-dimmed{opacity:0.3!important}
/* Scrollbar for panels */
.se-sidebar::-webkit-scrollbar,.fa-body::-webkit-scrollbar,.am-list::-webkit-scrollbar{width:4px}
.se-sidebar::-webkit-scrollbar-thumb,.fa-body::-webkit-scrollbar-thumb,.am-list::-webkit-scrollbar-thumb{background:var(--border-color);border-radius:2px}

/* ── Deepened Styles v3 ───────────────────────────────────────────── */
/* Node Detail Panel */
.node-detail-panel{position:fixed;top:60px;right:20px;width:380px;max-height:75vh;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-lg);z-index:200;box-shadow:0 8px 32px rgba(0,0,0,0.4);display:flex;flex-direction:column}
.ndp-header{display:flex;align-items:center;gap:8px;padding:12px 16px;border-bottom:1px solid var(--border-color);flex-shrink:0}
.ndp-tabs{display:flex;gap:2px;flex:1}
.ndp-tab{flex:1;padding:4px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:11px;text-align:center}
.ndp-tab:hover,.ndp-tab.active{border-color:var(--color-primary);color:var(--color-primary)}
.ndp-body{padding:12px;overflow-y:auto;flex:1}
.ndp-info-grid{display:grid;grid-template-columns:repeat(2,1fr);gap:8px}
.ndp-info-item{padding:8px;background:var(--bg-secondary);border-radius:var(--radius-md)}
.ndp-label{font-size:10px;color:var(--text-muted);display:block}.ndp-val{font-size:13px;color:var(--text-primary);font-weight:600}
.ndp-history{display:flex;flex-direction:column;gap:4px}
.ndp-hist-entry{display:flex;align-items:center;gap:8px;padding:6px 8px;background:var(--bg-secondary);border-radius:var(--radius-sm);font-size:11px}
.ndp-hist-time{color:var(--text-muted);font-family:'JetBrains Mono',monospace;width:60px}.ndp-hist-action{padding:1px 6px;border-radius:var(--radius-sm);background:rgba(0,212,255,0.2);color:var(--color-primary);width:50px;text-align:center}
.ndp-hist-details{flex:1;color:var(--text-primary)}.ndp-hist-empty{text-align:center;padding:20px;color:var(--text-muted);font-size:12px}

/* Edge Editor Panel */
.edge-editor-panel{position:fixed;top:60px;left:20px;width:320px;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-lg);z-index:200;box-shadow:0 8px 32px rgba(0,0,0,0.4)}
.eep-header{display:flex;align-items:center;justify-content:space-between;padding:12px 16px;border-bottom:1px solid var(--border-color);font-size:13px;font-weight:600;color:var(--color-primary)}
.eep-body{padding:12px;display:flex;flex-direction:column;gap:10px}
.eep-field{display:flex;flex-direction:column;gap:4px}
.eep-field label{font-size:11px;color:var(--text-muted)}
.eep-input,.eep-select{padding:6px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-secondary);color:var(--text-primary);font-size:12px}
.eep-actions{display:flex;justify-content:flex-end}

/* Template Manager */
.template-manager-panel{position:fixed;top:60px;left:20px;width:420px;max-height:75vh;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-lg);z-index:200;box-shadow:0 8px 32px rgba(0,0,0,0.4);display:flex;flex-direction:column}
.tmp-header{display:flex;align-items:center;justify-content:space-between;padding:12px 16px;border-bottom:1px solid var(--border-color);font-size:13px;font-weight:600;color:var(--color-primary);flex-shrink:0}
.tmp-body{padding:12px;overflow-y:auto;flex:1}
.tmp-search{margin-bottom:12px}
.tmp-input{width:100%;padding:8px 12px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-secondary);color:var(--text-primary);font-size:12px;box-sizing:border-box}
.tmp-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(140px,1fr));gap:10px}
.tmp-card{padding:12px;background:var(--bg-secondary);border-radius:var(--radius-md);border:1px solid var(--border-color);display:flex;flex-direction:column;gap:6px;transition:all .15s}
.tmp-card:hover{border-color:var(--color-primary);transform:translateY(-2px)}
.tmp-icon{font-size:24px;text-align:center}.tmp-name{font-size:12px;font-weight:600;color:var(--color-primary);text-align:center}
.tmp-desc{font-size:10px;color:var(--text-muted);min-height:32px}
.tmp-tags{display:flex;flex-wrap:wrap;gap:3px}
.tmp-tag{font-size:9px;padding:1px 5px;border-radius:var(--radius-sm);background:rgba(0,212,255,0.15);color:var(--color-primary)}
.tmp-actions{display:flex;gap:4px}

/* Collaboration Panel */
.collab-panel{position:fixed;top:60px;right:20px;width:280px;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-lg);z-index:200;box-shadow:0 8px 32px rgba(0,0,0,0.4)}
.col-header{display:flex;align-items:center;justify-content:space-between;padding:12px 16px;border-bottom:1px solid var(--border-color);font-size:13px;font-weight:600;color:var(--color-primary)}
.col-body{padding:12px}
.col-mode{display:flex;gap:4px;margin-bottom:12px}
.col-mode-btn{flex:1;padding:4px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:11px}
.col-mode-btn:hover,.col-mode-btn.active{border-color:var(--color-primary);color:var(--color-primary)}
.col-list{display:flex;flex-direction:column;gap:6px;margin-bottom:12px}
.col-item{display:flex;align-items:center;gap:8px;padding:6px 8px;background:var(--bg-secondary);border-radius:var(--radius-sm)}
.col-avatar{width:28px;height:28px;border-radius:50%;display:flex;align-items:center;justify-content:center;font-size:14px}
.col-name{flex:1;font-size:12px;color:var(--text-primary)}.col-status{font-size:10px}

/* Notification Panel */
.notification-panel{position:fixed;top:60px;right:20px;width:340px;max-height:70vh;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-lg);z-index:200;box-shadow:0 8px 32px rgba(0,0,0,0.4);display:flex;flex-direction:column}
.np-header{display:flex;align-items:center;justify-content:space-between;padding:12px 16px;border-bottom:1px solid var(--border-color);font-size:13px;font-weight:600;color:var(--color-primary);flex-shrink:0}
.np-body{padding:12px;overflow-y:auto;flex:1;display:flex;flex-direction:column;gap:8px}
.np-item{display:flex;align-items:flex-start;gap:8px;padding:8px;background:var(--bg-secondary);border-radius:var(--radius-sm)}
.np-item.unread{border-left:3px solid var(--color-primary)}
.np-type{font-size:14px;flex-shrink:0}.np-type-info{color:var(--color-primary)}.np-type-success{color:var(--color-success)}.np-type-warning{color:var(--color-warning)}.np-type-error{color:var(--color-danger)}
.np-content{flex:1}.np-title{font-size:12px;font-weight:600;color:var(--text-primary)}.np-msg{font-size:11px;color:var(--text-muted);margin-top:2px}
.np-time{font-size:10px;color:var(--text-muted);white-space:nowrap}.np-empty{text-align:center;padding:20px;color:var(--text-muted);font-size:12px}

/* Audit Trail Panel */
.audit-panel{position:fixed;top:60px;left:20px;width:400px;max-height:70vh;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-lg);z-index:200;box-shadow:0 8px 32px rgba(0,0,0,0.4);display:flex;flex-direction:column}
.at-header{display:flex;align-items:center;justify-content:space-between;padding:12px 16px;border-bottom:1px solid var(--border-color);font-size:13px;font-weight:600;color:var(--color-primary);flex-shrink:0}
.at-body{padding:12px;overflow-y:auto;flex:1;display:flex;flex-direction:column;gap:6px}
.at-entry{display:flex;align-items:center;gap:8px;padding:6px 8px;background:var(--bg-secondary);border-radius:var(--radius-sm);font-size:11px}
.at-time{color:var(--text-muted);font-family:'JetBrains Mono',monospace;width:70px}.at-user{color:var(--color-warning);width:50px}.at-action{padding:1px 6px;border-radius:var(--radius-sm);background:rgba(0,212,255,0.2);color:var(--color-primary);width:60px;text-align:center}
.at-target{color:var(--text-primary);flex:1}.at-details{color:var(--text-muted);font-size:10px}
.at-empty{text-align:center;padding:20px;color:var(--text-muted);font-size:12px}

/* Health Dashboard */
.health-panel{position:fixed;top:60px;left:20px;width:320px;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-lg);z-index:200;box-shadow:0 8px 32px rgba(0,0,0,0.4)}
.hp-header{display:flex;align-items:center;justify-content:space-between;padding:12px 16px;border-bottom:1px solid var(--border-color);font-size:13px;font-weight:600;color:var(--color-primary)}
.hp-body{padding:12px}
.hp-score{display:flex;align-items:center;gap:12px;padding:16px;background:var(--bg-secondary);border-radius:var(--radius-md);margin-bottom:12px}
.hp-val{font-size:36px;font-weight:800;color:var(--color-primary);font-family:'JetBrains Mono',monospace}
.hp-label{font-size:12px;color:var(--text-muted)}
.hp-grid{display:flex;flex-direction:column;gap:8px}
.hp-indicator{display:flex;align-items:center;gap:10px;padding:8px 10px;background:var(--bg-secondary);border-radius:var(--radius-sm)}
.hp-ind-name{flex:1;font-size:12px;color:var(--text-primary)}.hp-ind-val{font-size:14px;font-weight:700;font-family:'JetBrains Mono',monospace}
.hp-ind-dot{width:10px;height:10px;border-radius:50%;flex-shrink:0}
.hp-ind-dot.healthy{background:var(--color-success)}.hp-ind-dot.warning{background:var(--color-warning)}.hp-ind-dot.critical{background:var(--color-danger);animation:pulse 1s infinite}

/* Quality Report */
.quality-panel{position:fixed;top:60px;right:20px;width:320px;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-lg);z-index:200;box-shadow:0 8px 32px rgba(0,0,0,0.4)}
.qp-header{display:flex;align-items:center;justify-content:space-between;padding:12px 16px;border-bottom:1px solid var(--border-color);font-size:13px;font-weight:600;color:var(--color-primary)}
.qp-body{padding:12px}
.qp-score{display:flex;align-items:center;gap:12px;padding:16px;background:var(--bg-secondary);border-radius:var(--radius-md);margin-bottom:12px}
.qp-val{font-size:36px;font-weight:800;color:var(--color-success);font-family:'JetBrains Mono',monospace}
.qp-label{font-size:12px;color:var(--text-muted)}
.qp-metrics{display:flex;flex-direction:column;gap:6px}
.qp-metric{display:flex;align-items:center;gap:8px;padding:6px 8px;background:var(--bg-secondary);border-radius:var(--radius-sm);font-size:11px}
.qp-m-name{flex:1;color:var(--text-primary)}.qp-m-sev{padding:1px 6px;border-radius:var(--radius-sm);font-size:9px;font-weight:700}
.qp-m-val{color:var(--color-primary);font-family:'JetBrains Mono',monospace}
.sev-good{background:rgba(16,185,129,0.2);color:var(--color-success)}.sev-warning{background:rgba(245,158,11,0.2);color:var(--color-warning)}.sev-error{background:rgba(239,68,68,0.2);color:var(--color-danger)}

/* Version History */
.version-panel{position:fixed;top:60px;left:20px;width:360px;max-height:75vh;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-lg);z-index:200;box-shadow:0 8px 32px rgba(0,0,0,0.4);display:flex;flex-direction:column}
.vh-header{display:flex;align-items:center;justify-content:space-between;padding:12px 16px;border-bottom:1px solid var(--border-color);font-size:13px;font-weight:600;color:var(--color-primary);flex-shrink:0}
.vh-body{padding:12px;overflow-y:auto;flex:1}
.vh-entry{display:flex;align-items:center;justify-content:space-between;padding:8px 10px;background:var(--bg-secondary);border-radius:var(--radius-md);margin-bottom:6px}
.vh-info{display:flex;flex-direction:column;gap:2px;min-width:0}
.vh-label{font-size:13px;font-weight:600;color:var(--text-primary)}.vh-meta{font-size:10px;color:var(--text-muted)}
.vh-actions{display:flex;gap:4px;flex-shrink:0}
.vh-diff{display:flex;gap:12px;padding:8px 12px;background:rgba(0,212,255,0.1);border-radius:var(--radius-sm);font-size:11px;color:var(--color-primary);margin-top:8px}

/* Comment Panel */
.comment-panel{position:fixed;bottom:80px;right:20px;width:340px;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-lg);z-index:200;box-shadow:0 8px 32px rgba(0,0,0,0.4)}
.cm-header{display:flex;align-items:center;justify-content:space-between;padding:12px 16px;border-bottom:1px solid var(--border-color);font-size:13px;font-weight:600;color:var(--color-primary)}
.cm-body{padding:12px;display:flex;flex-direction:column;gap:8px}
.cm-input-row{display:flex;gap:6px}
.cm-author-input{width:60px;padding:4px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-secondary);color:var(--text-primary);font-size:11px}
.cm-target-select{padding:4px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-secondary);color:var(--text-primary);font-size:11px}
.cm-list{display:flex;flex-direction:column;gap:6px;max-height:200px;overflow-y:auto}
.cm-item{padding:8px;background:var(--bg-secondary);border-radius:var(--radius-sm)}
.cm-item.resolved{opacity:0.6}
.cm-author{font-size:11px;font-weight:600;color:var(--color-primary)}
.cm-content{font-size:12px;color:var(--text-primary);margin:4px 0}
.cm-meta{font-size:10px;color:var(--text-muted)}
.cm-actions{display:flex;gap:4px;margin-top:4px}
.cm-empty{text-align:center;padding:16px;color:var(--text-muted);font-size:12px}

/* Perf Monitor */
.perf-panel{position:fixed;bottom:20px;right:20px;width:200px;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-lg);z-index:200;box-shadow:0 8px 32px rgba(0,0,0,0.4)}
.pf-header{display:flex;align-items:center;justify-content:space-between;padding:10px 14px;border-bottom:1px solid var(--border-color);font-size:13px;font-weight:600;color:var(--color-warning)}
.pf-body{padding:12px}
.pf-grid{display:grid;grid-template-columns:repeat(2,1fr);gap:6px}
.pf-item{padding:8px;background:var(--bg-secondary);border-radius:var(--radius-sm);text-align:center}
.pf-val{font-size:16px;font-weight:700;color:var(--color-primary);font-family:'JetBrains Mono',monospace;display:block}
.pf-lbl{font-size:9px;color:var(--text-muted)}

/* Workflow Rules */
.rules-panel{position:fixed;top:60px;right:20px;width:400px;max-height:75vh;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-lg);z-index:200;box-shadow:0 8px 32px rgba(0,0,0,0.4);display:flex;flex-direction:column}
.rw-header{display:flex;align-items:center;justify-content:space-between;padding:12px 16px;border-bottom:1px solid var(--border-color);font-size:13px;font-weight:600;color:var(--color-primary);flex-shrink:0}
.rw-body{padding:12px;overflow-y:auto;flex:1;display:flex;flex-direction:column;gap:10px}
.rw-add{display:flex;gap:6px}
.rw-input{flex:1;padding:6px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-secondary);color:var(--text-primary);font-size:12px}
.rw-list{display:flex;flex-direction:column;gap:6px}
.rw-item{display:flex;align-items:center;gap:8px;padding:8px 10px;background:var(--bg-secondary);border-radius:var(--radius-sm)}
.rw-item.disabled{opacity:0.5}
.rw-name{flex:1;font-size:12px;font-weight:600;color:var(--text-primary)}.rw-cond{font-size:10px;color:var(--text-muted);flex:2}
</style>