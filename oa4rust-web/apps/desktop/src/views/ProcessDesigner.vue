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
          <span v-if="subprocessEditing">← 返回主流程 | 拖拽节点 | 点击边缘拖出连线 | Shift+多选</span>
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
            <div class="pg"><label>重试次数</label><input :value="getNodeProp('retryCount')" type="number" @input="_setNodeProp('retryCount',+$event.target.value)" class="pi" min="0" max="10" /></div>
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
                  <div class="var-row"><span class="var-label">输入变量</span><input class="var-input" placeholder="inputData" /></div>
                  <div class="var-row"><span class="var-label">输出变量</span><input class="var-input" placeholder="output" /></div>
                  <div class="var-row"><span class="var-label">上下文</span><input class="var-input" placeholder="context" /></div>
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
          <p v-if="currentProcess" class="hint">双击子流程节点进入嵌套编辑</p>
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
function isGate(type: string) { return type.startsWith('gate_') }
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
function getNodeConditions(node: PDNode): string[] {
  if (!node.condition) return []
  return node.condition.split(',').map(s => s.trim()).filter(Boolean)
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
type CanvasTheme = 'dark'|'midnight'|'ocean'|'forest'
const canvasThemes: Record<CanvasTheme, {bg:string;grid:string;name:string}> = {
  dark: { bg: '#0a0e1a', grid: 'rgba(255,255,255,0.03)', name: '暗夜' },
  midnight: { bg: '#0d1b2a', grid: 'rgba(100,200,255,0.03)', name: '午夜' },
  ocean: { bg: '#0a1628', grid: 'rgba(0,150,255,0.04)', name: '深海' },
  forest: { bg: '#0a1a0a', grid: 'rgba(0,255,100,0.03)', name: '森林' },
}
const canvasTheme = ref<CanvasTheme>('dark')
function setCanvasTheme(theme: CanvasTheme) {
  canvasTheme.value = theme
}

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
function deleteArchive(idx:number) { processArchive.value.splice(idx,1) }onUnmounted(() => {
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
</style>
