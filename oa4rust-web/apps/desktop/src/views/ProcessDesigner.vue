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
        <button class="btn" @click="zoomToFit" title="适配内容">⊞ 适配</button>
        <button class="btn" @click="clearCanvas" title="清空">🗑</button>
        <button class="btn" @click="createVersion()" title="创建版本快照">📌 快照</button>
        <button class="btn btn-outline" :class="{active: showVersionPanel}" @click="showVersionPanel=!showVersionPanel">📜 版本</button>
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
        <div class="pal-title">操作</div>
        <div class="pal-grid">
          <div class="pal-item" @click="autoLayout"><span class="ni">⊞</span><span class="nl">自动排列</span></div>
          <div class="pal-item" @click="clearCanvas"><span class="ni">🗑</span><span class="nl">清空</span></div>
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
          </g>

          <!-- Temp edge -->
          <path v-if="tempEdge" :d="tempEdgePath()" class="edge-temp" marker-end="url(#arrowhead-temp)" />

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
            <div class="pg"><label>条件表达式</label><input :value="getNodeProp('condition')" @input="_setNodeProp('condition',$event.target.value)" class="pi" placeholder="如: amount > 1000" /></div>
            <div class="pg"><label>超时(分钟)</label><input :value="getNodeProp('timeout')" type="number" @input="_setNodeProp('timeout',+$event.target.value)" class="pi" /></div>
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
          </div>
        </div>
        <div v-else-if="selectedEdge!==null" class="props-section">
          <div class="props-title"><span>连线属性</span></div>
          <div class="props-body">
            <div class="pg"><label>标签</label><input :value="getEdgeProp('label')" @input="_setEdgeProp('label',$event.target.value)" class="pi" /></div>
            <div class="pg"><label>流向</label><span class="pv">{{ getEdgeFromLabel() }} → {{ getEdgeToLabel() }}</span></div>
            <div class="pg"><label>条件</label><input :value="getEdgeProp('condition')" @input="_setEdgeProp('condition',$event.target.value)" class="pi" placeholder="如: amount > 1000" /></div>
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

    <!-- Subprocess Inline Editor -->
    <div v-if="subprocessEditing && processDef" class="subprocess-editor">
      <div class="sp-toolbar glass-card">
        <button class="btn" @click="exitSubprocess">← 返回主流程</button>
        <span class="sp-title">📦 子流程编辑 — {{ subprocessTitle }}</span>
        <button class="btn btn-outline" @click="addNode('start'); addNode('task')">+ 添加开始+任务</button>
        <button class="btn btn-primary" @click="saveSubprocess">💾 保存</button>
      </div>
      <div class="subprocess-canvas pd-canvas glass-card" ref="subprocessCanvasRef">
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
              <rect :class="['node-body', node.type]" :width="node.w||120" :height="node.h||50" rx="8" />
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
}
interface PDEdge { id: string; from: string; to: string; label?: string; condition?: string }
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
function toggleGroup(nodeIdx: number) {
  if (!processDef.value) return
  const node = processDef.value.nodes[nodeIdx]
  if (!groupedNodes.value.has(node.id)) {
    groupedNodes.value.add(node.id)
  } else {
    groupedNodes.value.delete(node.id)
  }
}

function createGroup() {
  if (groupedNodes.value.size < 2 || !processDef.value) return
  // Find bounding box
  let minX=Infinity, minY=Infinity, maxX=-Infinity, maxY=-Infinity
  for (const id of groupedNodes.value) {
    const n = processDef.value.nodes.find(nd => nd.id === id)
    if (!n) continue
    minX = Math.min(minX, n.x); minY = Math.min(minY, n.y)
    maxX = Math.max(maxX, n.x + (n.w||120)); maxY = Math.max(maxY, n.y + (n.h||50))
  }
  // Create group node
  const groupNode: PDNode = {
    id: genId(), type: 'subprocess', label: '分组',
    x: minX - 10, y: minY - 10,
    w: maxX - minX + 20, h: maxY - minY + 20
  }
  processDef.value.nodes.push(groupNode)
  groupedNodes.value.clear()
  selectedNode.value = processDef.value.nodes.length - 1
  pushHistory()
}

function ungroup(nodeIdx: number) {
  if (!processDef.value) return
  const node = processDef.value.nodes[nodeIdx]
  if (node.type !== 'subprocess') return
  // Could expand group back to individual nodes
  // For now just deselect
  selectedNode.value = null
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
function subSelectEdge(i: number) { subSelectedEdge.value = i; subSelectedNode.value = null }
function subPushHistory() {
  if (!subprocessDef.value) return
  subHistory.value = subHistory.value.slice(0, subHistIdx.value + 1)
  subHistory.value.push(JSON.parse(JSON.stringify(subprocessDef.value)))
  subHistIdx.value = subHistory.value.length - 1
}
function subUndo() { if (subHistIdx.value <= 0) return; subHistIdx.value--; subprocessDef.value = JSON.parse(JSON.stringify(subHistory.value[subHistIdx.value])); subSelectedNode.value = null }
function subRedo() { if (subHistIdx.value >= subHistory.value.length - 1) return; subHistIdx.value++; subprocessDef.value = JSON.parse(JSON.stringify(subHistory.value[subHistIdx.value])); subSelectedNode.value = null }

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
    if (e.key === 'g' && !e.ctrlKey && multiSelected.value.size >= 2) {
      e.preventDefault()
      createGroup()
    }
  })
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
</style>
