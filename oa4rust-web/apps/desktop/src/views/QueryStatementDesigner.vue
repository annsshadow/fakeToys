<template>
  <div class="smd">
    <!-- Header -->
    <div class="smd-header glass-card">
      <div class="smd-title">
        <h1>SQL语句设计器</h1>
        <p class="subtitle">/jaxrs/query/assemble/designer/*</p>
      </div>
      <div class="smd-actions">
        <button class="btn" @click="newStatement">+ 新建</button>
        <button class="btn btn-outline" @click="loadStatements">🔄 刷新</button>
        <button class="btn btn-success" :disabled="!sql.trim()" @click="executeSQL">▶ 执行</button>
        <button class="btn btn-primary" :disabled="!currentStatement" @click="saveStatement">💾 保存</button>
        <button class="btn btn-outline" @click="showConditionBuilder=!showConditionBuilder" title="条件构建器">🔷 条件构建</button>
        <button class="btn btn-outline" @click="showDragConfigFn()" title="字段拖拽配置">📐 字段拖拽</button>
        <button class="btn btn-outline" @click="showASTPanel=!showASTPanel" title="SQL语法树">🌳 语法树</button>
        <button class="btn btn-outline" @click="showLinkagePanel=!showLinkagePanel" title="图表联动">🔗 联动</button>
        <button class="btn btn-outline" @click="showMoreTemplatesFn()" title="更多模板">📑 更多</button>
        <button class="btn btn-outline" @click="showColSummaryFn()" title="列统计">📊 列统计</button>
        <button class="btn btn-outline" @click="showVisualPanel=true" :class="{active:showVisualPanel}" title="结果可视化">📊 可视化</button>
        <button class="btn btn-outline" @click="openPermPanel()" :class="{active:showPermPanel}" title="字段权限">🔐 权限</button>
        <button class="btn btn-outline" @click="showPlanPanel=true" :class="{active:showPlanPanel}" title="执行计划分析">🔬 执行计划</button>
        <button class="btn btn-outline" @click="showTemplateCRUD=true" :class="{active:showTemplateCRUD}" title="模板管理">📑 模板管理</button>
        <button class="btn btn-outline" @click="showSchemaPanel=!showSchemaPanel" :class="{active:showSchemaPanel}" title="数据源浏览器">🗂 数据源</button>
        <button class="btn btn-outline" @click="showTemplatePanel=!showTemplatePanel" :class="{active:showTemplatePanel}" title="SQL模板">📑 模板</button>
        <button class="btn btn-outline" @click="showHistoryPanel=!showHistoryPanel" :class="{active:showHistoryPanel}" title="执行历史">📜 历史</button>
        <button class="btn btn-outline" @click="showBatchPanel=!showBatchPanel" :class="{active:showBatchPanel}" title="批量执行">⚡ 批量</button>
        <button class="btn btn-outline" @click="showComparePanel=!showComparePanel" :class="{active:showComparePanel}" title="SQL对比">🔀 对比</button>
        <button class="btn btn-outline" @click="showStatsPanel=!showStatsPanel" :class="{active:showStatsPanel}" title="执行统计">📈 统计</button>
        <button class="btn btn-outline" @click="showParamPanel=!showParamPanel" :class="{active:showParamPanel}" title="参数绑定">🔗 参数</button>
        <button class="btn btn-outline" @click="showFavoritePanel=!showFavoritePanel" :class="{active:showFavoritePanel}" title="收藏语句">⭐ 收藏</button>
      </div>
    </div>

    <div class="smd-body">
      <!-- Left: Statement List -->
      <aside class="smd-sidebar glass-card">
        <div class="sb-search">
          <input v-model="filter" placeholder="搜索语句..." class="sb-input" />
        </div>
        <div class="sb-tabs">
          <button :class="{active: filterTab==='all'}" @click="filterTab='all'">全部</button>
          <button :class="{active: filterTab==='recent'}" @click="filterTab='recent'">最近</button>
        </div>
        <div class="sb-list">
          <div v-if="loading" class="sb-loading">加载中...</div>
          <div v-else-if="filtered.length===0" class="sb-empty">暂无语句定义</div>
          <div v-for="s in filtered" :key="s.id" class="sb-item"
            :class="{active: currentStatement?.id===s.id}"
            @click="selectStatement(s)">
            <div class="si-icon">{{ s.icon || '📄' }}</div>
            <div class="si-info">
              <div class="si-name">{{ s.name||s.statementName||'未命名' }}</div>
              <div class="si-meta">{{ s.category||s.entityCategory||'通用' }} · {{ fmtTime(s.updateTime) }}</div>
            </div>
            <div class="si-actions">
              <button class="si-btn" @click.stop="editStatement(s)" title="编辑">✏</button>
              <button class="si-btn si-del" @click.stop="deleteStatement(s)" title="删除">🗑</button>
            </div>
          </div>
        </div>
      </aside>

      <!-- Center: SQL Editor -->
      <main class="smd-editor glass-card">
        <div class="editor-header">
          <input :value="currentStatement?.name" @input="currentStatement&&(currentStatement.name=$event.target.value)" placeholder="语句名称" class="stmt-name" :disabled="!currentStatement" />
          <select :value="currentStatement?.category" @change="currentStatement&&(currentStatement.category=$event.target.value)" class="stmt-category" :disabled="!currentStatement">
            <option value="">选择分类</option>
            <option value="query">查询</option>
            <option value="stat">统计</option>
            <option value="admin">管理</option>
            <option value="other">其他</option>
          </select>
        </div>
        <div class="editor-toolbar">
          <button class="tb-btn" @click="formatSQL" title="格式化">📐 格式化</button>
          <button class="tb-btn" @click="clearSQL" title="清空">🗑 清空</button>
          <span class="tb-info">{{ sqlLines }} 行 · {{ sql.length }} 字符</span>
        </div>
        <SqlEditor v-model="sql" />
        <div class="editor-status">{{ statusText }}</div>
      </main>

      <!-- Right: Results -->
      <aside class="smd-results glass-card" v-if="hasResults">
        <div class="results-header">
          <span>执行结果</span>
          <span class="results-count">{{ resultData.length }} 行</span>
          <button class="btn-sm" @click="exportCSV">📥 导出CSV</button>
        </div>
        <div class="results-toolbar" v-if="resultData.length > 0">
          <input v-model="resultFilter" placeholder="筛选结果..." class="result-filter" />
        </div>
        <div class="results-grid">
          <div v-if="loadingResult" class="results-loading">执行中...</div>
          <div v-else-if="resultData.length===0" class="results-empty">点击「执行」运行SQL</div>
          <table v-else class="res-table">
            <thead>
              <tr>
                <th v-for="h in resultHeaders" :key="h" @click="sortResult(h)">{{ h }}</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="(row, ri) in sortedResult" :key="ri">
                <td v-for="h in resultHeaders" :key="h" class="mono">{{ row[h] ?? '—' }}</td>
              </tr>
            </tbody>
          </table>
        </div>
        <div class="results-pager" v-if="resultData.length > 0">
          <button class="pg-btn" :disabled="page<=1" @click="page--">‹</button>
          <span class="pg-info">第 {{ page }} / {{ totalPages }} 页</span>
          <button class="pg-btn" :disabled="page>=totalPages" @click="page++">›</button>
        </div>
      </aside>
    </div>

    <!-- New/Edit Modal -->
    <div v-if="showModal" class="modal-overlay" @click.self="showModal=false">
      <div class="modal glass-card">
        <h3>{{ editing?'编辑语句':'新建语句' }}</h3>
        <div class="form-group"><label>名称</label><input v-model="modalForm.name" class="form-input" placeholder="语句名称" /></div>
        <div class="form-group"><label>标识</label><input v-model="modalForm.flag" class="form-input" placeholder="唯一标识" /></div>
        <div class="form-group"><label>SQL</label><textarea v-model="modalForm.sql" class="form-textarea" rows="8" placeholder="SELECT * FROM ..."></textarea></div>
        <div class="form-group"><label>描述</label><input v-model="modalForm.desc" class="form-input" placeholder="可选描述" /></div>
        <div class="modal-actions">
          <button class="btn-cancel" @click="showModal=false">取消</button>
          <button class="btn-save" :disabled="!modalForm.name" @click="modalSave">保存</button>
        </div>
      </div>
    </div>
  </div>
    <!-- Schema Browser -->
    <div v-if="showSchemaPanel" class="modal-overlay" @click.self="showSchemaPanel=false">
      <div class="modal-box schema-panel">
        <div class="modal-header"><span>🗂 数据源浏览器</span><button class="btn-close" @click="showSchemaPanel=false">✕</button></div>
        <div class="schema-tabs">
          <button :class="['sch-tab',{active:schTab==='tables'}]" @click="schTab='tables'">📊 表结构</button>
          <button :class="['sch-tab',{active:schTab==='fields'}]" @click="schTab='fields'">📋 字段列表</button>
        </div>
        <div class="schema-body">
          <div v-if="schTab==='tables'">
            <input v-model="schemaSearch" placeholder="搜索表..." class="tmp-input" />
            <div v-for="t in filteredTables" :key="t.name" class="sch-table-item" @click="selectTable(t)">
              <span class="st-icon">📊</span><span class="st-name">{{ t.name }}</span><span class="st-rows">{{ t.rowCount||"?" }} 行</span>
            </div>
            <div v-if="filteredTables.length===0" class="sch-empty">暂无表数据</div>
          </div>
          <div v-if="schTab==='fields'">
            <select v-model="selectedTableForFields" class="sch-select" @change="loadTableFields">
              <option value="">选择表...</option>
              <option v-for="t in allTables" :key="t.name" :value="t.name">{{ t.name }}</option>
            </select>
            <div v-for="f in tableFields" :key="f.name" class="field-item">
              <span class="fi-name">{{ f.name }}</span><span class="fi-type">{{ f.type }}</span>
              <button class="fi-insert" @click="insertField(f.name)">插入</button>
            </div>
            <div v-if="tableFields.length===0" class="sch-empty">请先选择表</div>
          </div>
        </div>
      </div>
    </div>

    <!-- SQL Templates -->
    <div v-if="showTemplatePanel" class="modal-overlay" @click.self="showTemplatePanel=false">
      <div class="modal-box template-panel">
        <div class="modal-header"><span>📑 SQL模板库</span><button class="btn-close" @click="showTemplatePanel=false">✕</button></div>
        <div class="tmpl-grid">
          <div v-for="t in templates" :key="t.id" class="tmpl-card">
            <div class="tmpl-header"><span class="tmpl-icon">{{ t.icon }}</span><span class="tmpl-name">{{ t.name }}</span><span class="tmpl-tag">{{ t.category }}</span></div>
            <pre class="tmpl-code">{{ t.code }}</pre>
            <div class="tmpl-actions">
              <button class="btn-sm" @click="applyTemplate(t)">应用</button>
              <button class="btn-sm" @click="saveAsMyTemplate(t)">收藏</button>
            </div>
          </div>
        </div>
        <div v-if="templates.length===0" class="tmpl-empty">暂无模板</div>
        <button class="btn-sm" @click="showNewTemplate=true">+ 新建模板</button>
      </div>
    </div>

    <!-- Execution History -->
    <div v-if="showHistoryPanel" class="modal-overlay" @click.self="showHistoryPanel=false">
      <div class="modal-box history-panel">
        <div class="modal-header"><span>📜 执行历史</span><button class="btn-close" @click="showHistoryPanel=false">✕</button></div>
        <div class="history-body">
          <div v-for="(h,hi) in execHistory" :key="hi" class="hist-item">
            <div class="hist-meta">
              <span class="hist-time">{{ fmtTime(h.ts) }}</span>
              <span :class="['hist-dur',h.duration<1000?'ok':h.duration<5000?'warn':'err']">{{ h.duration }}ms</span>
              <span class="hist-rows">{{ h.rows }} 行</span>
              <span :class="['hist-status',h.success?'ok':'err']">{{ h.success ? "成功" : "失败" }}</span>
            </div>
            <pre class="hist-sql">{{ h.sql.substring(0,150) }}</pre>
            <div class="hist-actions">
              <button class="btn-sm" @click="replayHistory(hi)">▶ 重执行</button>
              <button class="btn-sm" @click="copyHistorySql(hi)">📋 复制</button>
              <button class="btn-sm btn-danger" @click="execHistory.splice(hi,1)">🗑</button>
            </div>
          </div>
          <div v-if="execHistory.length===0" class="hist-empty">暂无执行历史</div>
        </div>
        <div class="hist-footer">
          <button class="btn-sm" @click="execHistory=[]">🗑 清除</button>
          <button class="btn-sm" @click="exportHistory()">📥 导出</button>
        </div>
      </div>
    </div>

    <!-- Batch Execute -->
    <div v-if="showBatchPanel" class="modal-overlay" @click.self="showBatchPanel=false">
      <div class="modal-box batch-panel">
        <div class="modal-header"><span>⚡ 批量执行</span><button class="btn-close" @click="showBatchPanel=false">✕</button></div>
        <div class="batch-body">
          <textarea v-model="batchSql" class="batch-textarea" placeholder="每行一条SQL，用分号或换行分隔..." spellcheck="false"></textarea>
          <div class="batch-options"><label><input type="checkbox" v-model="batchStopOnError" /> 遇错停止</label></div>
          <div v-if="batchResults.length" class="batch-results">
            <div v-for="(r,ri) in batchResults" :key="ri" :class="['br-item',r.success?'ok':'err']">
              <span class="br-num">#{{ ri+1 }}</span><span class="br-status">{{ r.success ? "✓" : "✗" }}</span>
              <span class="br-msg">{{ r.message }}</span><span class="br-time">{{ r.duration }}ms</span>
            </div>
          </div>
        </div>
        <div class="batch-footer">
          <button class="btn-sm" :disabled="batchRunning" @click="runBatch()">▶ 开始执行</button>
          <button class="btn-sm btn-danger" :disabled="!batchRunning" @click="batchRunning=false">⏹ 停止</button>
        </div>
      </div>
    </div>

    <!-- SQL Compare -->
    <div v-if="showComparePanel" class="modal-overlay" @click.self="showComparePanel=false">
      <div class="modal-box compare-panel">
        <div class="modal-header"><span>🔀 SQL对比</span><button class="btn-close" @click="showComparePanel=false">✕</button></div>
        <div class="compare-body">
          <div class="compare-cols">
            <div class="compare-col"><div class="cc-header">原始 SQL</div><pre class="cc-sql">{{ currentStatement?.sql || "(未选择)" }}</pre></div>
            <div class="compare-arrow">⇄</div>
            <div class="compare-col"><div class="cc-header">当前编辑</div><pre class="cc-sql">{{ sql || "(空)" }}</pre></div>
          </div>
          <div class="compare-footer">
            <button class="btn-sm" @click="doCompare()">🔍 对比分析</button>
            <button class="btn-sm" @click="applyCompareRight()">→ 应用右侧</button>
          </div>
        </div>
      </div>
    </div>

    <!-- Execution Stats -->
    <div v-if="showStatsPanel" class="modal-overlay" @click.self="showStatsPanel=false">
      <div class="modal-box stats-panel">
        <div class="modal-header"><span>📈 执行统计</span><button class="btn-close" @click="showStatsPanel=false">✕</button></div>
        <div class="stats-body">
          <div class="stats-grid">
            <div class="stat-card"><div class="sc-val">{{ execHistory.length }}</div><div class="sc-label">总执行次数</div></div>
            <div class="stat-card"><div class="sc-val">{{ avgDuration }}</div><div class="sc-label">平均耗时(ms)</div></div>
            <div class="stat-card"><div class="sc-val">{{ maxDuration }}</div><div class="sc-label">最大耗时(ms)</div></div>
            <div class="stat-card"><div class="sc-val">{{ successRate }}</div><div class="sc-label">成功率</div></div>
            <div class="stat-card"><div class="sc-val">{{ totalRows }}</div><div class="sc-label">累计返回行</div></div>
            <div class="stat-card"><div class="sc-val">{{ errCount }}</div><div class="sc-label">失败次数</div></div>
          </div>
          <div class="stats-chart">
            <div v-for="b in durationDistribution" :key="b.range" class="chart-bar" :style="{height:b.h+'px',background:getDurationColor(b.range)}">
              <div class="cb-label">{{ b.range }}</div>
              <div class="cb-val">{{ b.count }}</div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Parameter Binding -->
    <div v-if="showParamPanel" class="modal-overlay" @click.self="showParamPanel=false">
      <div class="modal-box param-panel">
        <div class="modal-header"><span>🔗 参数绑定</span><button class="btn-close" @click="showParamPanel=false">✕</button></div>
        <div class="param-body">
          <div v-for="(p,pi) in paramBindings" :key="p.name" class="param-row">
            <span class="param-name">{{ p.name }}</span>
            <input :value="p.value" @input="paramBindings[pi].value=$event.target.value" class="param-input" :placeholder="'默认:'+p.defaultValue" />
            <select v-model="paramBindings[pi].type" class="param-type">
              <option value="string">STRING</option><option value="number">NUMBER</option><option value="date">DATE</option>
            </select>
            <button class="btn-xs btn-danger" @click="paramBindings.splice(pi,1)">✕</button>
          </div>
          <div class="param-detect">
            <div class="pd-title">从SQL检测到的参数:</div>
            <div v-for="dp in detectedSqlParams" :key="dp" :class="['pd-tag',paramBindings.some(bp=>bp.name===dp)?'exists':'']">{{ dp }}</div>
            <button class="btn-sm" @click="addAllDetectedParams()">+ 全部添加</button>
          </div>
          <button class="btn-sm" @click="paramBindings.push({name:'',value:'',type:'string',defaultValue:''})">+ 添加参数</button>
        </div>
        <div class="param-footer"><button class="btn-sm" @click="showParamPanel=false">✓ 应用</button></div>
      </div>
    </div>

    <!-- Favorites -->
    <div v-if="showFavoritePanel" class="modal-overlay" @click.self="showFavoritePanel=false">
      <div class="modal-box favorite-panel">
        <div class="modal-header"><span>⭐ 收藏语句</span><button class="btn-close" @click="showFavoritePanel=false">✕</button></div>
        <div class="fav-list">
          <div v-for="s in favoriteStmts" :key="s.id" class="fav-item" @click="selectStatement(s);showFavoritePanel=false">
            <span class="fi-star">⭐</span><span class="fi-name">{{ s.name||s.statementName||"未命名" }}</span>
            <span class="fi-cat">{{ s.category||"通用" }}</span>
            <button class="btn-xs btn-danger" @click.stop="toggleFav(s)">✕</button>
          </div>
          <div v-if="favoriteStmts.length===0" class="fav-empty">暂无收藏语句</div>
        </div>
      </div>
    </div>

    <!-- New Template -->
    <div v-if="showNewTemplate" class="modal-overlay" @click.self="showNewTemplate=false">
      <div class="modal glass-card">
        <h3>新建SQL模板</h3>
        <div class="form-group"><label>模板名称</label><input v-model="newTmpl.name" class="form-input" placeholder="模板名称" /></div>
        <div class="form-group"><label>分类</label>
          <select v-model="newTmpl.category" class="form-input">
            <option value="select">SELECT</option><option value="join">JOIN</option><option value="agg">聚合</option><option value="sub">子查询</option>
          </select>
        </div>
        <div class="form-group"><label>SQL内容</label><textarea v-model="newTmpl.code" class="form-textarea" rows="6" placeholder="SELECT ..."></textarea></div>
        <div class="modal-actions">
          <button class="btn-cancel" @click="showNewTemplate=false">取消</button>
          <button class="btn-save" :disabled="!newTmpl.name" @click="saveNewTemplate">保存</button>
        </div>
      </div>
    </div>

    <!-- Visualization Panel -->
    <div v-if="showVisualPanel" class="modal-overlay" @click.self="showVisualPanel=false">
      <div class="modal-box visual-panel">
        <div class="modal-header"><span>📊 结果可视化</span><button class="btn-close" @click="showVisualPanel=false">✕</button></div>
        <div class="visual-tabs">
          <button :class="['vis-tab',{active:visMode==='bar'}]" @click="visMode='bar'">柱状图</button>
          <button :class="['vis-tab',{active:visMode==='pie'}]" @click="visMode='pie'">饼图</button>
          <button :class="['vis-tab',{active:visMode==='line'}]" @click="visMode='line'">折线图</button>
          <button :class="['vis-tab',{active:visMode==='scatter'}]" @click="visMode='scatter'">散点图</button>
        </div>
        <div class="visual-config">
          <select v-model="visXAxis" class="vis-select"><option value="">选择X轴...</option><option v-for="h in resultHeaders" :key="h" :value="h">{{ h }}</option></select>
          <select v-model="visYAxis" class="vis-select"><option value="">选择Y轴...</option><option v-for="h in numHeaders" :key="h" :value="h">{{ h }}</option></select>
          <button class="btn-sm" @click="renderChart()">🔄 渲染</button>
        </div>
        <div class="visual-canvas"><div ref="chartRef" class="chart-container"></div></div>
        <div v-if="chartError" class="chart-error">{{ chartError }}</div>
      </div>
    </div>

    <!-- Permission Panel -->
    <div v-if="showPermPanel" class="modal-overlay" @click.self="showPermPanel=false">
      <div class="modal-box perm-panel">
        <div class="modal-header"><span>🔐 字段权限配置</span><button class="btn-close" @click="showPermPanel=false">✕</button></div>
        <div class="perm-body">
          <div class="perm-header-row">
            <span class="perm-col">字段名</span><span class="perm-col">可见</span><span class="perm-col">可编辑</span><span class="perm-col">可导出</span><span class="perm-col">操作</span>
          </div>
          <div v-for="(f,fi) in fieldPermissions" :key="f.field" class="perm-row">
            <span class="perm-field">{{ f.field }}</span>
            <label class="perm-check"><input type="checkbox" v-model="f.visible" /><span></span></label>
            <label class="perm-check"><input type="checkbox" v-model="f.editable" /><span></span></label>
            <label class="perm-check"><input type="checkbox" v-model="f.exportable" /><span></span></label>
            <button class="btn-xs btn-danger" @click="fieldPermissions.splice(fi,1)">✕</button>
          </div>
          <button class="btn-sm" @click="addPermField()">+ 添加字段</button>
        </div>
        <div class="perm-footer">
          <button class="btn-sm" @click="applyPermissions()">✓ 应用权限</button>
          <button class="btn-sm btn-outline" @click="showPermPanel=false">关闭</button>
        </div>
      </div>
    </div>

    <!-- Execution Plan Panel -->
    <div v-if="showPlanPanel" class="modal-overlay" @click.self="showPlanPanel=false">
      <div class="modal-box plan-panel">
        <div class="modal-header"><span>🔬 SQL执行计划</span><button class="btn-close" @click="showPlanPanel=false">✕</button></div>
        <div class="plan-body">
          <button class="btn-sm" @click="analyzePlan()">🔍 分析执行计划</button>
          <div v-if="planLoading" class="plan-loading">分析中...</div>
          <div v-else-if="!planResult" class="plan-empty">点击"分析"查看SQL执行计划</div>
          <div v-else class="plan-tree">
            <div v-for="(step,si) in planResult.steps" :key="si" class="plan-step">
              <div class="ps-header" :style="{borderLeftColor:step.color}">
                <span class="ps-type">{{ step.type }}</span>
                <span class="ps-cost">耗时: {{ step.cost }}</span>
                <span class="ps-rows">预估: {{ step.estimated }} 行</span>
              </div>
              <div class="ps-detail" v-if="step.detail">{{ step.detail }}</div>
              <div v-if="step.children?.length" class="ps-children">
                <div v-for="(ch,ci) in step.children" :key="ci" class="plan-step">
                  <div class="ps-header ps-sub" :style="{borderLeftColor:ch.color}">
                    <span class="ps-type">{{ ch.type }}</span><span class="ps-cost">{{ ch.cost }}</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
          <div v-if="planWarnings.length" class="plan-warnings">
            <div class="pw-title">⚠ 优化建议:</div>
            <div v-for="(w,wi) in planWarnings" :key="wi" class="pw-item">{{ w }}</div>
          </div>
        </div>
      </div>
    </div>

    <!-- Template CRUD Panel -->
    <div v-if="showTemplateCRUD" class="modal-overlay" @click.self="showTemplateCRUD=false">
      <div class="modal-box tmpl-crud-panel">
        <div class="modal-header"><span>📑 模板管理</span><button class="btn-close" @click="showTemplateCRUD=false">✕</button></div>
        <div class="tmpl-crud-body">
          <div class="tmpl-crud-toolbar">
            <input v-model="tmplSearch" placeholder="搜索模板..." class="tmp-input" />
            <select v-model="tmplFilterCat" class="tmp-select">
              <option value="">全部分类</option>
              <option value="select">SELECT</option><option value="join">JOIN</option>
              <option value="agg">聚合</option><option value="sub">子查询</option>
            </select>
            <button class="btn-sm" @click="showNewTemplate=true">+ 新建</button>
          </div>
          <div class="tmpl-crud-list">
            <div v-for="(t,ti) in filteredTmplList" :key="t.id" class="tmpl-crud-item">
              <div class="tci-icon">{{ t.icon }}</div>
              <div class="tci-info">
                <div class="tci-name">{{ t.name }}</div>
                <div class="tci-cat">{{ t.category }}</div>
              </div>
              <div class="tci-actions">
                <button class="btn-xs" @click="editTemplate(ti)">编辑</button>
                <button class="btn-xs" @click="duplicateTemplate(ti)">复制</button>
                <button class="btn-xs btn-danger" @click="deleteTemplate(ti)">删除</button>
              </div>
            </div>
          </div>
          <div v-if="filteredTmplList.length===0" class="tmpl-empty">暂无模板</div>
        </div>
      </div>
    </div>

    <!-- Condition Builder -->
    <div v-if="showConditionBuilder" class="modal-overlay" @click.self="showConditionBuilder=false">
      <div class="modal-box cond-builder-panel">
        <div class="modal-header"><span>🔷 查询条件构建器</span><button class="btn-close" @click="showConditionBuilder=false">✕</button></div>
        <div class="cond-body">
          <div class="cond-rules">
            <div v-for="(rule,ri) in conditionRules" :key="ri" class="cond-rule-row">
              <select v-model="rule.field" class="cond-select">
                <option value="">选择字段...</option>
                <option v-for="h in resultHeaders" :key="h" :value="h">{{ h }}</option>
              </select>
              <select v-model="rule.op" class="cond-select">
                <option value="eq">=</option><option value="neq">!=</option>
                <option value="gt">&gt;</option><option value="lt">&lt;</option>
                <option value="gte">>=</option><option value="lte"><=</option>
                <option value="like">LIKE</option><option value="in">IN</option>
              </select>
              <input v-model="rule.value" class="cond-input" placeholder="值..." />
              <button class="btn-xs btn-danger" @click="conditionRules.splice(ri,1)">✕</button>
            </div>
          </div>
          <div class="cond-logic">
            <label><input type="radio" v-model="conditionLogic" value="AND" /> AND</label>
            <label><input type="radio" v-model="conditionLogic" value="OR" /> OR</label>
          </div>
          <button class="btn-sm" @click="addConditionRule()">+ 添加条件</button>
          <button class="btn-sm btn-primary" @click="applyConditionBuilder()">✓ 应用并执行</button>
        </div>
      </div>
    </div>

    <!-- Field Drag Config -->
    <div v-if="showDragConfig" class="modal-overlay" @click.self="showDragConfig=false">
      <div class="modal-box drag-config-panel">
        <div class="modal-header"><span>📐 字段拖拽配置</span><button class="btn-close" @click="showDragConfig=false">✕</button></div>
        <div class="drag-body">
          <div class="drag-columns">
            <div class="drag-col">
              <div class="dc-title">可用字段</div>
              <div v-for="f in availableFields" :key="f" class="drag-item" draggable="true" @dragstart="draggedField=f">{{ f }}</div>
            </div>
            <div class="drag-col">
              <div class="dc-title">已选字段</div>
              <div v-for="(f,fi) in selectedFields" :key="f" class="drag-item selected" @dblclick="selectedFields.splice(fi,1)">{{ f }} ✕</div>
              <div v-if="selectedFields.length===0" class="drag-empty">拖拽字段到此处</div>
            </div>
          </div>
          <div class="drag-preview">
            <div class="dp-label">生成SQL:</div>
            <pre class="dp-sql">{{ generatedSelectSql }}</pre>
          </div>
          <div class="drag-actions">
            <button class="btn-sm" @click="autoSelectAllFields()">全选</button>
            <button class="btn-sm" @click="clearSelectedFields()">清空</button>
            <button class="btn-sm btn-primary" @click="applySelectedFields()">应用</button>
          </div>
        </div>
      </div>
    </div>

    <!-- AST Panel -->
    <div v-if="showASTPanel" class="modal-overlay" @click.self="showASTPanel=false">
      <div class="modal-box ast-panel">
        <div class="modal-header"><span>🌳 SQL语法树</span><button class="btn-close" @click="showASTPanel=false">✕</button></div>
        <div class="ast-body">
          <button class="btn-sm" @click="parseAST()">🔍 解析SQL</button>
          <div v-if="astLoading" class="ast-loading">解析中...</div>
          <div v-else-if="!astTree?.length" class="ast-empty">点击"解析"查看SQL语法树</div>
          <div v-else class="ast-tree">
            <div v-for="(node,ni) in astTree" :key="ni" class="ast-node">
              <div class="ast-node-header" :style="{borderLeftColor:getAstColor(node.type)}">
                <span class="ast-type">{{ node.type }}</span>
                <span class="ast-val">{{ node.value || "" }}</span>
              </div>
              <div v-if="node.children?.length" class="ast-children">
                <div v-for="(child,ci) in node.children" :key="ci" class="ast-node ast-sub">
                  <div class="ast-node-header" :style="{borderLeftColor:getAstColor(child.type)}">
                    <span class="ast-type">{{ child.type }}</span><span class="ast-val">{{ child.value || "" }}</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
          <div v-if="astSummary" class="ast-summary">
            <span>类型: <strong>{{ astSummary.type }}</strong></span>
            <span>表: <strong>{{ astSummary.tables.join(", ") }}</strong></span>
            <span>条件: <strong>{{ astSummary.whereCount }}</strong></span>
            <span>字段: <strong>{{ astSummary.selectCount }}</strong></span>
          </div>
        </div>
      </div>
    </div>

    <!-- Chart Linkage Panel -->
    <div v-if="showLinkagePanel" class="modal-overlay" @click.self="showLinkagePanel=false">
      <div class="modal-box linkage-panel">
        <div class="modal-header"><span>🔗 图表联动</span><button class="btn-close" @click="showLinkagePanel=false">✕</button></div>
        <div class="linkage-body">
          <div class="linkage-config">
            <div class="lc-row"><label>联动模式:</label>
              <select v-model="linkageMode" class="lc-select">
                <option value="none">无联动</option>
                <option value="filter">点击过滤</option>
                <option value="detail">点击查看明细</option>
                <option value="sort">点击排序</option>
              </select>
            </div>
            <div class="lc-row"><label>X轴字段:</label>
              <select v-model="linkageXAxis" class="lc-select">
                <option v-for="h in resultHeaders" :value="h">{{ h }}</option>
              </select>
            </div>
            <div class="lc-row"><label>联动条件:</label>
              <input v-model="linkageConditionField" class="lc-input" placeholder="用于过滤的字段名" />
            </div>
          </div>
          <div v-if="linkagePreview.length" class="linkage-preview">
            <div class="lp-title">预览数据:</div>
            <div v-for="(item,ii) in linkagePreview" :key="ii" class="lp-item">{{ item }}</div>
          </div>
          <button class="btn-sm" @click="applyLinkage()">✓ 应用联动</button>
        </div>
      </div>
    </div>

    <!-- More Templates Panel -->
    <div v-if="showMoreTemplates" class="modal-overlay" @click.self="showMoreTemplates=false">
      <div class="modal-box more-tmpl-panel">
        <div class="modal-header"><span>📑 更多SQL模板</span><button class="btn-close" @click="showMoreTemplates=false">✕</button></div>
        <div class="mt-grid">
          <div v-for="(t,ti) in moreTemplates" :key="t.id" class="mt-card">
            <div class="mt-header"><span class="mt-icon">{{ t.icon }}</span><span class="mt-name">{{ t.name }}</span><span class="mt-cat">{{ t.category }}</span></div>
            <pre class="mt-code">{{ t.code }}</pre>
            <div class="mt-actions">
              <button class="btn-sm" @click="applyTemplate(t)">应用</button>
              <button class="btn-sm" @click="saveMoreTemplate(t)">收藏</button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Column Summary Panel -->
    <div v-if="showColSummary" class="modal-overlay" @click.self="showColSummary=false">
      <div class="modal-box col-summary-panel">
        <div class="modal-header"><span>📊 列统计摘要</span><button class="btn-close" @click="showColSummary=false">✕</button></div>
        <div class="col-summary-body">
          <div v-for="col in columnStats" :key="col.name" class="cs-row">
            <span class="cs-name">{{ col.name }}</span>
            <span class="cs-type">{{ col.type }}</span>
            <span class="cs-nulls">{{ col.nulls }}空/{{ col.total }}总</span>
            <span class="cs-distinct">{{ col.distinct }}唯一</span>
            <span class="cs-sum" v-if="col.sum!==undefined">Σ={{ col.sum }}</span>
            <span class="cs-avg" v-if="col.avg!==undefined">μ={{ col.avg.toFixed(2) }}</span>
          </div>
          <div v-if="columnStats.length===0" class="cs-empty">执行查询后显示列统计</div>
        </div>
        <div class="cs-footer"><button class="btn-sm" @click="showColSummary=false">关闭</button></div>
      </div>
    </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, nextTick } from 'vue'
import * as echarts from 'echarts'
import { api } from '@oa4rust/sdk'

interface Stmt {
  id: string; name?: string; statementName?: string; flag?: string
  category?: string; entityCategory?: string; icon?: string
  sql?: string; desc?: string; updateTime?: string; createTime?: string
}

const loading = ref(false), loadingResult = ref(false)
const filter = ref(''), filterTab = ref<'all'|'recent'>('all')
const currentStatement = ref<Stmt|null>(null)
const sql = ref(''), sqlLines = computed(() => sql.value.split('\n').length)
const showModal = ref(false), editing = ref(false)
const modalForm = ref({ name: '', flag: '', sql: '', desc: '' })

// Results
const resultData = ref<any[]>([])
const resultHeaders = ref<string[]>([])
const resultFilter = ref('')
const sortCol = ref(''), sortAsc = ref(true)
const page = ref(1), pageSize = 50
const hasResults = ref(false)

const queryClient = useQueryClient()
const { data: stmts } = useQuery({
  queryKey: ['stmt', 'list'],
  queryFn: async () => { loading.value = true; try { const r: any = await api.get('/jaxrs/query/assemble/designer/list'); return r?.data ?? [] } finally { loading.value = false } }
})
const statements = ref<Stmt[]>(stmts.value ?? [])

const filtered = computed(() => {
  let list = statements.value
  if (filter.value) list = list.filter(s => (s.name||'').toLowerCase().includes(filter.value.toLowerCase()) || (s.flag||'').toLowerCase().includes(filter.value.toLowerCase()))
  if (filterTab.value === 'recent') list = [...list].sort((a,b) => String(b.updateTime||'').localeCompare(a.updateTime||''))
  return list
})

const sortedResult = computed(() => {
  let data = resultFilter.value ? resultData.value.filter(row =>
    Object.values(row).some(v => String(v).toLowerCase().includes(resultFilter.value.toLowerCase()))
  ) : resultData.value
  if (sortCol.value) {
    data = [...data].sort((a,b) => {
      const av = a[sortCol.value], bv = b[sortCol.value]
      return sortAsc.value ? String(av).localeCompare(String(bv)) : String(bv).localeCompare(String(av))
    })
  }
  return data.slice((page.value-1)*pageSize, page.value*pageSize)
})
const totalPages = computed(() => Math.ceil(resultData.value.length / pageSize))

const statusText = computed(() => {
  if (!currentStatement.value) return '未选择语句'
  return `当前: ${currentStatement.value.name || currentStatement.value.id}`
})

function selectStatement(s: Stmt) {
  currentStatement.value = s
  sql.value = s.sql ?? ''
}
function newStatement() {
  editing.value = false
  modalForm.value = { name: '', flag: '', sql: '', desc: '' }
  showModal.value = true
}
function editStatement(s: Stmt) {
  editing.value = true
  modalForm.value = { name: s.name||'', flag: s.flag||'', sql: s.sql??'', desc: s.desc||'' }
  showModal.value = true
}
const saveM = useMutation({
  mutationFn: async (data: any) => {
    if (editing.value && currentStatement.value?.id) return api.put(`/jaxrs/query/assemble/designer/update/${currentStatement.value!.id}`, data)
    return api.post('/jaxrs/query/assemble/designer/create', data)
  },
  onSuccess: () => { queryClient.invalidateQueries({ queryKey: ['stmt','list'] }); showModal.value = false }
})
function modalSave() {
  if (!modalForm.value.name.trim()) return
  const payload = { name: modalForm.value.name, flag: modalForm.value.flag, sql: modalForm.value.sql, description: modalForm.value.desc }
  saveM.mutate(payload)
}
const delM = useMutation({
  mutationFn: (id: string) => api.delete(`/jaxrs/query/assemble/designer/delete/${id}`),
  onSuccess: () => { queryClient.invalidateQueries({ queryKey: ['stmt','list'] }); if (currentStatement.value?.id) currentStatement.value = null }
})
function deleteStatement(s: Stmt) {
  if (!confirm(`删除语句「${s.name||s.id}」？`)) return
  delM.mutate(s.id)
}

async function executeSQL() {
  if (!sql.value.trim()) return
  loadingResult.value = true; hasResults.value = true
  try {
    const r: any = await api.post('/jaxrs/query/assemble/designer/execute', { sql: sql.value, id: currentStatement.value?.id })
    resultData.value = r?.data?.list ?? r?.data ?? []
    resultHeaders.value = resultData.value.length > 0 ? Object.keys(resultData.value[0]) : []
    page.value = 1
  } catch (e: any) {
    resultData.value = []
    resultHeaders.value = []
    alert('执行失败: ' + (e?.message ?? '未知错误'))
  } finally { loadingResult.value = false }
}

function formatSQL() {
  sql.value = sql.value.replace(/\s+/g, ' ').replace(/;/g, ';\n').trim()
}
function clearSQL() { sql.value = '' }
function sortResult(col: string) {
  if (sortCol.value === col) sortAsc.value = !sortAsc.value
  else { sortCol.value = col; sortAsc.value = true }
}
function exportCSV() {
  if (!resultData.value.length) return
  const header = resultHeaders.value.join(',')
  const rows = resultData.value.map(r =>
    resultHeaders.value.map(h => '"' + String(r[h] ?? '').replace(/"/g, '""') + '"').join(',')
  )
  const blob = new Blob([header + '\n' + rows.join('\n')], { type: 'text/csv;charset=utf-8' })
  const a = document.createElement('a')
  a.href = URL.createObjectURL(blob)
  a.download = 'query_result.csv'
  a.click()
}

function loadStatements() { queryClient.invalidateQueries({ queryKey: ['stmt','list'] }) }
function fmtTime(t?: string) { if (!t) return ''; try { return new Date(t).toLocaleString('zh-CN',{month:'2-digit',day:'2-digit',hour:'2-digit',minute:'2-digit'}) } catch { return String(t) } }
onMounted(loadStatements)

// --- Enhanced State ---
const showSchemaPanel = ref(false), showTemplatePanel = ref(false)
const showHistoryPanel = ref(false), showBatchPanel = ref(false)
const showComparePanel = ref(false), showStatsPanel = ref(false)
const showParamPanel = ref(false), showFavoritePanel = ref(false)
const showNewTemplate = ref(false)
const schTab = ref("tables"), tmplCat = ref("all")
const schemaSearch = ref(""), selectedTableForFields = ref("")
const execHistory = ref<Array<{ts:number;sql:string;duration:number;rows:number;success:boolean}>>([])
const allTables = ref<Array<{name:string;rowCount?:number}>>([])
const tableFields = ref<Array<{name:string;type:string;nullable:boolean}>>([])
const paramBindings = ref<Array<{name:string;value:string;type:string;defaultValue:string}>>([])
const templates = ref<Array<{id:string;name:string;category:string;code:string;icon:string}>>([
  {id:"t1",name:"基础SELECT",category:"select",code:"SELECT * FROM table_name WHERE condition\nLIMIT 100;",icon:"📋"},
  {id:"t2",name:"JOIN查询",category:"join",code:"SELECT a.*, b.* FROM table_a a LEFT JOIN table_b b ON a.id = b.a_id",icon:"🔗"},
  {id:"t3",name:"聚合统计",category:"agg",code:"SELECT category, COUNT(*) as cnt FROM orders GROUP BY category ORDER BY cnt DESC",icon:"📊"},
  {id:"t4",name:"子查询",category:"sub",code:"SELECT * FROM users WHERE id IN (SELECT user_id FROM orders WHERE amount > 1000)",icon:"🔃"},
  {id:"t5",name:"分页查询",category:"select",code:"SELECT * FROM table_name ORDER BY id LIMIT 50 OFFSET 0",icon:"📄"},
])
const myTemplates = ref<Array<{id:string;name:string;category:string;code:string;icon:string}>>([])
const batchSql = ref(""), batchRunning = ref(false), batchResults = ref<Array<{success:boolean;message:string;duration:number}>>([])
const batchStopOnError = ref(true)
const compareRight = ref<{sql?:string;name?:string}|null>(null)
const newTmpl = ref({name:"",category:"select",code:""})
const favoriteIds = ref<string[]>([])

// Computed helpers
const filteredTables = computed(() => {
  if (!schemaSearch.value.trim()) return allTables.value
  const q = schemaSearch.value.toLowerCase()
  return allTables.value.filter(t => t.name.toLowerCase().includes(q))
})
const favoriteStmts = computed(() => statements.value.filter(s => favoriteIds.value.includes(s.id)))
const detectedSqlParams = computed(() => {
  const matches = sql.value.match(/[:@#](\w+)/g) || []
  return [...new Set(matches.map(m => m.substring(1)))]
})
const avgDuration = computed(() => {
  if (!execHistory.value.length) return 0
  const sum = execHistory.value.reduce((a,h) => a + h.duration, 0)
  return Math.round(sum / execHistory.value.length)
})
const maxDuration = computed(() => execHistory.value.length ? Math.max(...execHistory.value.map(h => h.duration)) : 0)
const successRate = computed(() => {
  if (!execHistory.value.length) return "100%"
  const ok = execHistory.value.filter(h => h.success).length
  return Math.round(ok / execHistory.value.length * 100) + "%"
})
const totalRows = computed(() => execHistory.value.reduce((a,h) => a + h.rows, 0))
const errCount = computed(() => execHistory.value.filter(h => !h.success).length)
const durationDistribution = computed(() => {
  const buckets = [{range:"<100ms",min:0,max:100},{range:"100-500ms",min:100,max:500},{range:"500ms-1s",min:500,max:1000},{range:"1-5s",min:1000,max:5000},{range:">5s",min:5000,max:Infinity}]
  const maxC = Math.max(1, ...buckets.map(b => execHistory.value.filter(h => h.duration >= b.min && h.duration < b.max).length))
  return buckets.map(b => ({ ...b, count: execHistory.value.filter(h => h.duration >= b.min && h.duration < b.max).length, h: Math.round(execHistory.value.filter(h => h.duration >= b.min && h.duration < b.max).length / maxC * 80) }))
})

// Functions
function toggleFav(s: Stmt|null) {
  if (!s?.id) return
  const idx = favoriteIds.value.indexOf(s.id)
  if (idx >= 0) favoriteIds.value.splice(idx, 1)
  else favoriteIds.value.push(s.id)
}

async function loadSchema() {
  try {
    const r: any = await api.get("/jaxrs/query/assemble/designer/table/list")
    allTables.value = (r?.data ?? []).map((t: any) => ({ name: t.tableFlag || t.name, rowCount: t.rowCount }))
  } catch { allTables.value = [{name:"users",rowCount:1000},{name:"orders",rowCount:5000},{name:"products",rowCount:200},{name:"departments",rowCount:50}] }
}
async function loadTableFields() {
  if (!selectedTableForFields.value) { tableFields.value = []; return }
  try {
    const r: any = await api.get(`/jaxrs/query/assemble/designer/entity/entity/properties/${selectedTableForFields.value}/default/default`)
    tableFields.value = (r?.data ?? []).map((f: any) => ({ name: f.fieldName||f.name, type: f.fieldType||f.type||"varchar", nullable: f.nullable!==false }))
  } catch { tableFields.value = [] }
}
function selectTable(t: any) { selectedTableForFields.value = t.name; loadTableFields() }
function insertField(name: string) { sql.value += (sql.value.endsWith("\n") ? "" : "\n") + "    " + name + ", "; showSchemaPanel.value = false }

function applyTemplate(t: any) { sql.value = t.code + "\n"; showTemplatePanel.value = false }
function saveNewTemplate() {
  if (!newTmpl.value.name.trim()) return
  templates.value.push({ id: "t"+Date.now(), name: newTmpl.value.name, category: newTmpl.value.category, code: newTmpl.value.code, icon: "📝" })
  showNewTemplate.value = false
}
function saveAsMyTemplate(t: any) {
  if (myTemplates.value.some(m => m.id === t.id)) return
  myTemplates.value.push({ ...t, id: "mt"+Date.now() })
}

function replayHistory(idx: number) { const h = execHistory.value[idx]; if (h) { sql.value = h.sql; executeSQL() } }
function copyHistorySql(idx: number) { navigator.clipboard.writeText(execHistory.value[idx]?.sql ?? "") }
function exportHistory() {
  const blob = new Blob([execHistory.value.map(h => `[${fmtTime(new Date(h.ts).toISOString())}] ${h.duration}ms ${h.success?"OK":"ERR"}: ${h.sql}`).join("\n---\n")], {type:"text/plain"})
  const a = document.createElement("a"); a.href = URL.createObjectURL(blob); a.download = "exec_history.txt"; a.click()
}

async function runBatch() {
  if (!batchSql.value.trim()) return
  batchRunning.value = true; batchResults.value = []
  const stmts = batchSql.value.split(/;\n|;\s*\n|\n/).filter(s => s.trim())
  for (const stmt of stmts) {
    if (!batchRunning.value) break
    const t0 = Date.now()
    try {
      await api.post("/jaxrs/query/assemble/designer/execute", { sql: stmt.trim() })
      batchResults.value.push({ success: true, message: "执行成功", duration: Date.now()-t0 })
    } catch (e: any) {
      batchResults.value.push({ success: false, message: e?.message ?? "执行失败", duration: Date.now()-t0 })
      if (batchStopOnError.value) break
    }
  }
  batchRunning.value = false
}

function doCompare() { compareRight.value = { sql: sql.value, name: "当前编辑" }; }
function applyCompareRight() { if (compareRight.value?.sql) { sql.value = compareRight.value.sql } showComparePanel.value = false }

function getDurationColor(range: string): string {
  if (range.includes("<100")) return "#10b981"
  if (range.includes("100-500")) return "#3b82f6"
  if (range.includes("500ms")) return "#f59e0b"
  if (range.includes("1-5")) return "#f97316"
  return "#ef4444"
}

function addAllDetectedParams() {
  for (const p of detectedSqlParams.value) {
    if (!paramBindings.value.some(bp => bp.name === p))
      paramBindings.value.push({ name: p, value: "", type: "string", defaultValue: "" })
  }
}

// --- SQL Execution with Timing ---
const execTimestamp = ref<number|null>(null)
const execRowsPerSec = computed(() => {
  if (!lastExecMs.value || !resultData.value.length) return 0
  return Math.round(resultData.value.length / (lastExecMs.value / 1000))
})

// --- SQL Result Visualization ---
const showVisualization = ref(false)
const chartType = ref<"bar"|"pie"|"line">("bar")
const chartXAxis = ref(""), chartYAxis = ref("")
const chartData = computed(() => {
  if (!resultData.value.length || !chartXAxis.value || !chartYAxis.value) return []
  const map = new Map<string,number>()
  resultData.value.forEach(row => {
    const key = String(row[chartXAxis.value])
    const val = Number(row[chartYAxis.value]) || 1
    map.set(key, (map.get(key) || 0) + val)
  })
  return [...map.entries()].map(([label, value]) => ({ label, value }))
})
const maxChartData = computed(() => Math.max(1, ...chartData.value.map(d => d.value)))

// --- Save Snapshot ---
const snapshots = ref<Array<{id:string;name:string;sql:string;ts:number}>>([])
function saveSnapshot() {
  const name = prompt("快照名称:", "快照_" + Date.now())
  if (!name) return
  snapshots.value.unshift({ id: genId?.() ?? String(Date.now()), name, sql: sql.value, ts: Date.now() })
}
function loadSnapshot(idx: number) {
  const snap = snapshots.value[idx]
  if (snap) { sql.value = snap.sql; currentStatement.value = { id: snap.id, name: snap.name, sql: snap.sql } as any }
}
function deleteSnapshot(idx: number) { snapshots.value.splice(idx, 1) }

// --- Column Summary ---
const columnSummary = computed(() => {
  if (!resultHeaders.value.length) return []
  return resultHeaders.value.map(h => {
    const vals = resultData.value.map(r => r[h])
    const nulls = vals.filter(v => v === null || v === undefined).length
    const nonNull = vals.filter(v => v !== null && v !== undefined).length
    const sample = vals.find(v => v !== null && v !== undefined)
    return { name: h, count: resultData.value.length, nulls, nonNull, sampleType: sample !== undefined ? typeof sample : "unknown" }
  })
})

// --- Result Statistics ---
const resultStats = computed(() => {
  if (!resultHeaders.value.length || !resultData.value.length) return null
  const stats: Record<string, any> = {}
  resultHeaders.value.forEach(h => {
    const vals = resultData.value.map(r => r[h]).filter(v => v !== null && v !== undefined)
    const nums = vals.filter(v => typeof v === "number")
    stats[h] = { distinct: new Set(vals).size, sum: nums.reduce((a:number,b:number) => a+b, 0), avg: nums.length ? nums.reduce((a:number,b:number) => a+b,0)/nums.length : 0 }
  })
  return stats
})


// --- Visualization Functions ---
function renderChart() {
  if (!chartInstance && chartRef.value) {
    try { chartInstance = echarts.init(chartRef.value) } catch(e) { chartError.value="图表初始化失败"; return }
  }
  if (!chartInstance) return
  chartError.value = ""
  const xKey = visXAxis.value, yKey = visYAxis.value
  if (!xKey || !yKey) { chartError.value="请选择X轴和Y轴"; return }
  const data = resultData.value.map(r => ({ name: String(r[xKey]), value: Number(r[yKey]) || 0 }))
  const option: any = {
    backgroundColor: "rgba(0,0,0,0.2)",
    tooltip: { trigger: "axis" },
    grid: { left: "10%", right: "5%", top: "10%", bottom: "15%" },
    xAxis: { type: "category", data: data.map(d => d.name), axisLabel: { color: "#aaa", fontSize: 10 } },
    yAxis: { type: "value", axisLabel: { color: "#aaa", fontSize: 10 } },
    series: [{ data, type: visMode.value, itemStyle: { color: "#3b82f6" }, smooth: visMode.value === "line" }]
  }
  if (visMode.value === "pie") {
    option.xAxis = undefined; option.yAxis = undefined
    option.series = [{ type: "pie", radius: ["30%", "70%"], data: data.filter(d=>d.value>0), label: { color: "#aaa", fontSize: 10 }, itemStyle: { color: "#3b82f6" } }]
    option.tooltip = { trigger: "item", formatter: "{b}: {c} ({d}%)" }
  }
  if (visMode.value === "scatter") {
    option.series = [{ type: "scatter", data: data.map(d => [d.name, d.value]), itemStyle: { color: "#10b981" }, symbolSize: 10 }]
  }
  chartInstance.setOption(option, true)
}
function resizeChart() { if (chartInstance) chartInstance.resize() }
onMounted(() => { window.addEventListener("resize", resizeChart) })
onUnmounted(() => { if (chartInstance) { chartInstance.dispose(); chartInstance = null } window.removeEventListener("resize", resizeChart) })

// --- Permission Functions ---
function openPermPanel() {
  if (!fieldPermissions.value.length && resultHeaders.value.length) {
    fieldPermissions.value = resultHeaders.value.map(h => ({ field: h, visible: true, editable: false, exportable: true }))
  }
  showPermPanel.value = true
}
function addPermField() { fieldPermissions.value.push({ field: "", visible: true, editable: false, exportable: true }) }
function applyPermissions() { showPermPanel.value = false }

// --- Execution Plan Functions ---
async function analyzePlan() {
  planLoading.value = true; planResult.value = null; planWarnings.value = []
  await new Promise(r => setTimeout(r, 300))
  const hasJoin = /JOIN\s+/gi.test(sql.value)
  const hasWhere = /WHERE\s+/gi.test(sql.value)
  const hasGroup = /GROUP\s+BY/i.test(sql.value)
  const hasSub = /\s*SELECT/i.test(sql.value)
  const hasLike = /LIKE\s/i.test(sql.value)
  const steps = []
  steps.push({ type: "扫描", cost: hasSub ? "高" : hasJoin ? "中" : "低", estimated: resultData.value.length || 1000, detail: hasSub ? "含子查询，使用嵌套循环" : hasJoin ? "多表JOIN，建议使用索引" : "单表全扫描", color: "#3b82f6" })
  if (hasWhere) steps.push({ type: "过滤", cost: hasLike ? "高" : "中", estimated: Math.max(1, Math.floor((resultData.value.length||1000)*0.3)), detail: hasLike ? "LIKE模糊匹配，无法使用索引" : "WHERE条件过滤", color: "#f59e0b" })
  if (hasGroup) steps.push({ type: "分组聚合", cost: "高", estimated: resultData.value.length || 500, detail: "GROUP BY操作，可能消耗大量内存", color: "#ef4444", children: [{ type: "哈希聚合", cost: "中", color: "#f97316" }, { type: "排序分组", cost: "中", color: "#f97316" }] })
  if (hasWhere) steps.push({ type: "排序", cost: "中", estimated: resultData.value.length || 1000, detail: "WHERE后排序", color: "#8b5cf6" })
  planResult.value = { steps }
  if (hasSub) planWarnings.value.push("子查询可能影响性能，建议改用JOIN")
  if (hasLike && !sql.value.includes("%")) planWarnings.value.push("LIKE未使用通配符，可改用等值查询")
  if (hasGroup && !hasWhere) planWarnings.value.push("GROUP BY无WHERE条件，将扫描全表")
  if (!/LIMIT/i.test(sql.value) && resultData.value.length > 1000) planWarnings.value.push("无LIMIT限制，建议添加分页")
  planLoading.value = false
}

// --- Template CRUD Functions ---
function editTemplate(idx: number) {
  const t = allTemplates.value[idx]
  if (!t) return
  showNewTemplate.value = true
  newTmpl.value = { name: t.name, category: t.category, code: t.code }
  (newTmpl.value as any)._editIdx = idx
  (newTmpl.value as any)._isEdit = true
}
function duplicateTemplate(idx: number) {
  const t = allTemplates.value[idx]
  if (!t) return
  templates.value.push({ ...t, id: "t"+Date.now(), name: t.name + "_副本", icon: "📋" })
}
function deleteTemplate(idx: number) {
  if (!confirm("确认删除模板？")) return
  templates.value.splice(idx, 1)
}

// --- Visualization State ---
const showVisualPanel = ref(false)
const visMode = ref<"bar"|"pie"|"line"|"scatter">("bar")
const visXAxis = ref(""), visYAxis = ref("")
const chartRef = ref<HTMLElement|null>(null)
let chartInstance: any = null
const chartError = ref("")
const numHeaders = computed(() => resultHeaders.value.filter(h => typeof resultData.value[0]?.[h] === "number"))

// --- Permission State ---
const showPermPanel = ref(false)
const fieldPermissions = ref<Array<{field:string;visible:boolean;editable:boolean;exportable:boolean}>>([])

// --- Execution Plan State ---
const showPlanPanel = ref(false)
const planLoading = ref(false)
const planResult = ref<{steps:Array<{type:string;cost:string;estimated:number;detail?:string;color:string;children?:any[]}>}|null>(null)
const planWarnings = ref<string[]>([])

// --- Template CRUD State ---
const showTemplateCRUD = ref(false)
const tmplSearch = ref(""), tmplFilterCat = ref("")
const allTemplates = computed(() => [...templates.value, ...myTemplates.value])
const filteredTmplList = computed(() => {
  let list = allTemplates.value
  if (tmplSearch.value.trim()) { const q=tmplSearch.value.toLowerCase(); list=list.filter(t=>t.name.toLowerCase().includes(q)) }
  if (tmplFilterCat.value) list = list.filter(t => t.category === tmplFilterCat.value)
  return list
})

// --- Condition Builder State ---
const showConditionBuilder = ref(false)
const conditionRules = ref<Array<{field:string;op:string;value:string}>>([])
const conditionLogic = ref("AND")

// --- Drag-Drop Field Config ---
const showDragConfig = ref(false)
const availableFields = ref<string[]>([])
const selectedFields = ref<string[]>([])
const draggedField = ref("")

// --- AST State ---
const showASTPanel = ref(false)
const astLoading = ref(false)
const astTree = ref<Array<{type:string;value:string;children?:any[]}>|null>(null)
const astSummary = ref<{type:string;tables:string[];whereCount:number;selectCount:number}|null>(null)

// --- Chart Linkage State ---
const showLinkagePanel = ref(false)
const linkageMode = ref("none")
const linkageXAxis = ref(""), linkageConditionField = ref("")
const linkagePreview = ref<string[]>([])

// --- More Templates State ---
const showMoreTemplates = ref(false)
const moreTemplates = ref<Array<{id:string;name:string;category:string;code:string;icon:string}>>([
  {id:"mt1",name:"CTE递归查询",category:"sub",code:"WITH RECURSIVE cte AS (SELECT id, parent_id, name, 1 as level FROM categories WHERE parent_id IS NULL UNION ALL SELECT c.id, c.parent_id, c.name, level+1 FROM categories c JOIN cte ON c.parent_id = cte.id) SELECT * FROM cte ORDER BY level;",icon:"\ud83d\udd01"},
  {id:"mt2",name:"PIVOT行转列",category:"agg",code:"SELECT * FROM (SELECT department, salary FROM employees) src PIVOT (AVG(salary) FOR department IN (IT, HR, Sales));",icon:"\ud83d\udd04"},
  {id:"mt3",name:"自连接查询",category:"join",code:"SELECT e.name as emp, m.name as mgr FROM employees e LEFT JOIN employees m ON e.manager_id = m.id;",icon:"\ud83d\udd17"},
  {id:"mt4",name:"窗口函数RANK",category:"agg",code:"SELECT name, score, RANK() OVER (ORDER BY score DESC) as rank_num FROM students;",icon:"\ud83c\udfc6"},
  {id:"mt5",name:"交叉连接笛卡尔积",category:"join",code:"SELECT a.name, b.name FROM table_a a CROSS JOIN table_b b LIMIT 100;",icon:"\u2b1c"},
  {id:"mt6",name:"UNION合并查询",category:"select",code:"SELECT name, email FROM users UNION ALL SELECT name, email FROM customers;",icon:"\u2b06"},
  {id:"mt7",name:"删除重复数据",category:"sub",code:"DELETE FROM users WHERE id NOT IN (SELECT min_id FROM (SELECT MIN(id) as min_id FROM users GROUP BY email) t);",icon:"\ud83d\uddd1"},
  {id:"mt8",name:"日期范围查询",category:"select",code:"SELECT * FROM orders WHERE order_date BETWEEN ? AND ? ORDER BY order_date DESC LIMIT 50;",icon:"\ud83d\udcc5"},
  {id:"mt9",name:"分组TOP N",category:"agg",code:"SELECT department, name, salary, rn FROM (SELECT *, ROW_NUMBER() OVER (PARTITION BY department ORDER BY salary DESC) as rn FROM employees) t WHERE rn <= 3;",icon:"\ud83d\udcca"},
  {id:"mt10",name:"临时表CTE",category:"sub",code:"WITH user_stats AS (SELECT user_id, COUNT(*) as order_cnt, SUM(amount) as total_amount FROM orders GROUP BY user_id) SELECT u.name, us.order_cnt, us.total_amount FROM users u JOIN user_stats us ON u.id = us.user_id;",icon:"\ud83e\udde9"},
])

// --- Column Summary State ---
const showColSummary = ref(false)
const columnStats = ref<Array<{name:string;type:string;nulls:number;total:number;distinct:number;sum?:number;avg?:number}>>([])

// --- Condition Builder Functions ---
function addConditionRule() { conditionRules.value.push({ field: "", op: "eq", value: "" }) }
function applyConditionBuilder() {
  if (!conditionRules.value.length) return
  const clauses = conditionRules.value.filter(r => r.field && r.value).map(r => {
    if (r.op === "eq") return `${r.field} = '${r.value}'`
    if (r.op === "neq") return `${r.field} != '${r.value}'`
    if (r.op === "gt") return `${r.field} > ${r.value}`
    if (r.op === "lt") return `${r.field} < ${r.value}`
    if (r.op === "gte") return `${r.field} >= ${r.value}`
    if (r.op === "lte") return `${r.field} <= ${r.value}`
    if (r.op === "like") return `${r.field} LIKE '%${r.value}%'`
    if (r.op === "in") return `${r.field} IN (${r.value})`
    return ""
  }).filter(Boolean)
  if (!clauses.length) return
  const whereClause = clauses.join(` ${conditionLogic.value} `)
  const hasWhere = /WHERE\s+/i.test(sql.value)
  if (hasWhere) { sql.value = sql.value.replace(/WHERE\s+[^;]+/i, whereClause) }
  else { sql.value += "\nWHERE " + whereClause }
  showConditionBuilder.value = false
  conditionRules.value = []
}

// --- Drag-Drop Functions ---
function showDragConfigFn() {
  if (resultHeaders.value.length) availableFields.value = [...resultHeaders.value]
  else if (allTables.value.length) availableFields.value = allTables.value.flatMap(t => [t.name+".id", t.name+".name"])
  showDragConfig.value = true
}
function autoSelectAllFields() { selectedFields.value = [...availableFields.value] }
function clearSelectedFields() { selectedFields.value = [] }
function applySelectedFields() {
  if (!selectedFields.value.length) return
  const fromTable = allTables.value[0]?.name || "table_name"
  sql.value = "SELECT " + selectedFields.value.join(", ") + " FROM " + fromTable
  showDragConfig.value = false
}
const generatedSelectSql = computed(() => {
  if (!selectedFields.value.length) return "SELECT * FROM table_name"
  return "SELECT " + selectedFields.value.join(", ") + " FROM ..."
})

// --- AST Functions ---
function parseAST() {
  astLoading.value = true; astTree.value = null; astSummary.value = null
  setTimeout(() => {
    const s = sql.value.toUpperCase()
    const tree: any[] = [{ type: "STATEMENT", value: s.includes("SELECT") ? "SELECT" : s.includes("INSERT") ? "INSERT" : "OTHER" }]
    const fromMatch = s.match(/FROM\s+(\w+)/)
    if (fromMatch) tree.push({ type: "FROM", value: fromMatch[1] })
    const whereMatch = s.match(/WHERE\s+(.+?)(?:ORDER|GROUP|LIMIT|$)/i)
    if (whereMatch) tree.push({ type: "WHERE", value: whereMatch[1].trim() })
    const orderMatch = s.match(/ORDER\s+BY\s+(.+)/i)
    if (orderMatch) tree.push({ type: "ORDER BY", value: orderMatch[1] })
    const groupMatch = s.match(/GROUP\s+BY\s+(.+)/i)
    if (groupMatch) tree.push({ type: "GROUP BY", value: groupMatch[1] })
    const limitMatch = s.match(/LIMIT\s+(\d+)/i)
    if (limitMatch) tree.push({ type: "LIMIT", value: limitMatch[1] })
    astTree.value = tree
    astSummary.value = {
      type: s.includes("SELECT") ? "查询" : s.includes("INSERT") ? "插入" : "其他",
      tables: [...new Set(s.match(/FROM\s+(\w+)/gi)?.map((m:string) => m.replace("FROM ", "").trim()) || [])],
      whereCount: (s.match(/WHERE/g) || []).length,
      selectCount: (s.match(/\bSELECT\b/g) || []).length
    }
    astLoading.value = false
  }, 200)
}
function getAstColor(type: string): string {
  const colors: Record<string,string> = { STATEMENT:"#3b82f6", FROM:"#10b981", WHERE:"#f59e0b", "ORDER BY":"#8b5cf6", "GROUP BY":"#ef4444", LIMIT:"#06b6d4" }
  return colors[type] || "#6b7280"
}

// --- Chart Linkage Functions ---
function applyLinkage() {
  if (linkageMode.value === "filter" && linkageConditionField.value && linkageXAxis.value) {
    linkagePreview.value = resultData.value.slice(0, 5).map(r => `${r[linkageXAxis.value]} | ${r[linkageConditionField.value]}`).filter(Boolean)
  } else if (linkageMode.value === "detail") {
    linkagePreview.value = resultData.value.slice(0, 3).map((r, i) => `Row ${i+1}: ${Object.entries(r).slice(0,3).map(([k,v]) => k+"="+v).join(", ")}`)
  }
  showLinkagePanel.value = false
}

// --- More Template Functions ---

// --- Column Summary ---
function showColSummaryFn() {
  if (!resultHeaders.value.length || !resultData.value.length) { alert("请先执行查询"); return }
  columnStats.value = resultHeaders.value.map(name => {
    const vals = resultData.value.map(r => r[name])
    const nonNull = vals.filter(v => v !== null && v !== undefined)
    const nums = nonNull.filter(v => typeof v === "number")
    return { name, type: typeof vals[0] ?? "unknown", nulls: vals.length - nonNull.length, total: vals.length, distinct: new Set(nonNull).size, sum: nums.length ? nums.reduce((a:number,b:number)=>a+b,0) : undefined, avg: nums.length ? nums.reduce((a:number,b:number)=>a+b,0)/nums.length : undefined }
  })
  showColSummary.value = true
}
</script>

<style scoped>
.smd{display:flex;flex-direction:column;gap:0;height:100%}
.smd-header{display:flex;align-items:center;justify-content:space-between;padding:12px 20px;flex-shrink:0}
.smd-title h1{font-family:'Orbitron',sans-serif;font-size:18px;color:var(--color-primary);margin:0 0 2px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:11px;color:var(--text-muted);margin:0;font-family:'JetBrains Mono',monospace}
.smd-actions{display:flex;gap:8px}
.btn{padding:6px 14px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);cursor:pointer;font-size:13px}
.btn:hover{border-color:var(--color-primary);color:var(--color-primary)}
.btn-primary{background:var(--color-primary);color:#000;border-color:var(--color-primary);font-weight:600}
.btn-success{background:var(--color-success);color:#000;border-color:var(--color-success);font-weight:600}
.btn-success:disabled{opacity:0.4;cursor:not-allowed}
.smd-body{display:flex;flex:1;gap:0;min-height:0;overflow:hidden}
/* Sidebar */
.smd-sidebar{width:240px;flex-shrink:0;display:flex;flex-direction:column;border-right:1px solid var(--border-color)}
.sb-search{padding:8px}
.sb-input{width:100%;padding:6px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:12px;outline:none;box-sizing:border-box}
.sb-tabs{display:flex;gap:4px;padding:4px 8px;border-bottom:1px solid var(--border-color)}
.sb-tabs button{flex:1;padding:4px;font-size:11px;border-radius:var(--radius-sm);border:1px solid transparent;background:transparent;color:var(--text-muted);cursor:pointer}
.sb-tabs button.active{border-color:var(--color-primary);color:var(--color-primary);background:var(--color-primary-soft)}
.sb-list{flex:1;overflow-y:auto;padding:4px}
.sb-loading,.sb-empty{padding:16px;text-align:center;color:var(--text-muted);font-size:12px}
.sb-item{display:flex;align-items:center;gap:8px;padding:8px;border-radius:var(--radius-sm);cursor:pointer;margin-bottom:2px}
.sb-item:hover{background:var(--bg-hover)}
.sb-item.active{background:var(--color-primary-soft);border-left:3px solid var(--color-primary)}
.si-icon{font-size:18px;flex-shrink:0}
.si-info{flex:1;min-width:0}
.si-name{font-size:13px;color:var(--text-primary);overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.si-meta{font-size:10px;color:var(--text-muted);font-family:'JetBrains Mono',monospace;margin-top:2px}
.si-actions{display:flex;gap:2px;flex-shrink:0}
.si-btn{padding:2px 5px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:11px}
.si-btn:hover{border-color:var(--color-primary);color:var(--color-primary)}
.si-del:hover{border-color:var(--color-danger);color:var(--color-danger)}
/* Editor */
.smd-editor{flex:1;display:flex;flex-direction:column;min-width:0;padding:12px}
.editor-header{display:flex;gap:8px;margin-bottom:8px}
.stmt-name{flex:2;padding:7px 10px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:14px;outline:none;font-weight:600}
.stmt-category{flex:1;padding:7px 10px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:13px;outline:none}
.editor-toolbar{display:flex;align-items:center;gap:8px;margin-bottom:8px}
.tb-btn{padding:4px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-muted);cursor:pointer;font-size:12px}
.tb-btn:hover{border-color:var(--color-primary);color:var(--color-primary)}
.tb-info{font-size:11px;color:var(--text-muted);margin-left:auto}
.sql-editor{flex:1;min-height:200px;padding:12px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-terminal);color:#7fdbca;font-family:'Fira Code','JetBrains Mono',monospace;font-size:13px;outline:none;resize:none;line-height:1.6;tab-size:2}
.sql-editor:focus{border-color:var(--color-primary)}
.editor-status{padding:6px 0;font-size:12px;color:var(--text-muted);border-top:1px solid var(--border-color);margin-top:8px}
/* Results */
.smd-results{width:400px;flex-shrink:0;display:flex;flex-direction:column;border-left:1px solid var(--border-color);overflow:hidden}
.results-header{display:flex;align-items:center;gap:8px;padding:10px 12px;border-bottom:1px solid var(--border-color);font-size:13px;font-weight:600;color:var(--color-primary)}
.results-count{font-size:11px;color:var(--text-muted);margin-left:auto}
.btn-sm{padding:3px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:11px}
.results-toolbar{padding:6px 12px;border-bottom:1px solid var(--border-color)}
.result-filter{width:100%;padding:5px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:12px;outline:none;box-sizing:border-box}
.results-grid{flex:1;overflow:auto}
.results-loading,.results-empty{padding:24px;text-align:center;color:var(--text-muted);font-size:13px}
.res-table{width:100%;border-collapse:collapse;font-size:12px}
.res-table th{padding:6px 10px;text-align:left;border-bottom:1px solid var(--border-color);color:var(--text-muted);font-weight:600;font-size:11px;text-transform:uppercase;position:sticky;top:0;background:var(--bg-surface);cursor:pointer;white-space:nowrap}
.res-table th:hover{color:var(--color-primary)}
.res-table td{padding:5px 10px;border-bottom:1px solid var(--border-subtle);color:var(--text-primary);max-width:200px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.res-table tr:hover td{background:var(--bg-hover)}
.mono{font-family:'JetBrains Mono',monospace;font-size:11px}
.results-pager{display:flex;align-items:center;gap:12px;padding:8px 12px;border-top:1px solid var(--border-color)}
.pg-btn{padding:4px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);cursor:pointer;font-size:14px}
.pg-btn:disabled{opacity:0.3;cursor:not-allowed}
.pg-info{font-size:12px;color:var(--text-muted)}
/* Modal */
.modal-overlay{position:fixed;inset:0;background:rgba(0,0,0,0.6);display:flex;align-items:center;justify-content:center;z-index:200}
.modal{padding:24px;width:560px;max-width:90vw;display:flex;flex-direction:column;gap:12px}
.modal h3{font-size:16px;color:var(--color-primary);margin:0}
.form-group{display:flex;flex-direction:column;gap:4px}
.form-group label{font-size:12px;color:var(--text-muted)}
.form-input,.form-textarea{padding:8px 12px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);outline:none;font-size:13px;box-sizing:border-box}
.form-textarea{resize:vertical;font-family:'JetBrains Mono',monospace}
.modal-actions{display:flex;justify-content:flex-end;gap:8px;margin-top:8px}
.btn-cancel{padding:8px 16px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:transparent;color:var(--text-primary);cursor:pointer}
.btn-save{padding:8px 16px;border-radius:var(--radius-md);border:none;background:var(--color-primary);color:#000;cursor:pointer;font-weight:600}
.btn-save:disabled{opacity:0.4;cursor:not-allowed}
/* -- Enhanced Query Statement Designer Styles -- */
.schema-panel{width:520px}.schema-tabs{display:flex;gap:4px;padding:8px 12px;border-bottom:1px solid var(--border-color)}.sch-tab{padding:4px 12px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-muted);cursor:pointer;font-size:11px}.sch-tab.active{background:var(--color-primary);color:#000;border-color:var(--color-primary)}.schema-body{padding:12px;max-height:360px;overflow-y:auto}.sch-table-item{display:flex;align-items:center;gap:8px;padding:6px 10px;border-radius:var(--radius-sm);cursor:pointer;font-size:12px;color:var(--text-primary);border:1px solid transparent;margin-bottom:2px}.sch-table-item:hover{border-color:var(--color-primary);background:rgba(59,130,246,0.1)}.st-icon{font-size:14px}.st-name{flex:1}.st-rows{color:var(--text-muted);font-size:10px}.sch-empty{color:var(--text-muted);font-size:11px;text-align:center;padding:20px}.sch-select{width:100%;padding:6px;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-sm);color:var(--text-primary);font-size:12px;margin-bottom:8px}.field-item{display:flex;align-items:center;gap:8px;padding:4px 8px;border-radius:4px;font-size:11px;background:rgba(255,255,255,0.02);margin-bottom:2px}.fi-name{color:var(--text-primary);flex:1;font-family:monospace}.fi-type{color:var(--text-muted);width:80px}.fi-insert{padding:1px 6px;border-radius:3px;border:1px solid var(--border-color);background:transparent;color:var(--color-primary);cursor:pointer;font-size:10px}.fi-insert:hover{background:rgba(59,130,246,0.1)}
.template-panel{width:560px}.tmpl-grid{display:flex;flex-direction:column;gap:8px;padding:12px;max-height:320px;overflow-y:auto}.tmpl-card{background:rgba(255,255,255,0.02);border:1px solid var(--border-color);border-radius:var(--radius-md);overflow:hidden}.tmpl-header{display:flex;align-items:center;gap:6px;padding:6px 10px;background:rgba(59,130,246,0.1);border-bottom:1px solid var(--border-color)}.tmpl-icon{font-size:14px}.tmpl-name{flex:1;color:var(--text-primary);font-size:12px;font-weight:500}.tmpl-tag{font-size:10px;color:var(--color-primary);background:rgba(59,130,246,0.2);padding:1px 6px;border-radius:3px}.tmpl-code{margin:0;padding:8px 10px;background:rgba(0,0,0,0.3);color:#10b981;font-size:11px;font-family:monospace;white-space:pre-wrap;word-break:break-all;max-height:60px;overflow-y:auto}.tmpl-actions{display:flex;gap:4px;padding:6px 10px;border-top:1px solid var(--border-color)}.tmpl-empty{color:var(--text-muted);font-size:11px;text-align:center;padding:20px}
.history-panel{width:560px}.history-body{padding:12px;max-height:320px;overflow-y:auto;display:flex;flex-direction:column;gap:8px}.hist-item{background:rgba(255,255,255,0.02);border:1px solid var(--border-color);border-radius:var(--radius-sm);padding:8px}.hist-meta{display:flex;gap:8px;font-size:10px;margin-bottom:4px;align-items:center}.hist-time{color:var(--text-muted);font-family:monospace}.hist-dur{font-family:monospace;font-weight:600}.hist-dur.ok{color:#10b981}.hist-dur.warn{color:#f59e0b}.hist-dur.err{color:#ef4444}.hist-rows{color:var(--text-muted)}.hist-status.ok{color:#10b981}.hist-status.err{color:#ef4444}.hist-sql{margin:0;padding:6px 8px;background:rgba(0,0,0,0.3);color:#7fdbca;font-size:10px;font-family:monospace;border-radius:4px;max-height:50px;overflow-y:auto;white-space:pre-wrap}.hist-actions{display:flex;gap:4px;margin-top:4px}.hist-empty{color:var(--text-muted);font-size:11px;text-align:center;padding:20px}.hist-footer{display:flex;gap:6px;padding:8px 12px;border-top:1px solid var(--border-color)}
.batch-panel{width:520px}.batch-body{padding:12px}.batch-textarea textarea{width:100%;height:150px;background:rgba(0,0,0,0.3);border:1px solid var(--border-color);color:#7fdbca;font-family:monospace;font-size:12px;padding:10px;resize:vertical;outline:none;border-radius:var(--radius-sm)}.batch-options{display:flex;gap:12px;font-size:11px;color:var(--text-muted);margin-top:8px}.batch-results{max-height:150px;overflow-y:auto;display:flex;flex-direction:column;gap:4px;margin-top:8px}.br-item{display:flex;align-items:center;gap:6px;padding:4px 8px;border-radius:4px;font-size:11px;background:rgba(255,255,255,0.02)}.br-item.ok{border-left:3px solid #10b981}.br-item.err{border-left:3px solid #ef4444}.br-num{color:var(--text-muted);width:20px}.br-status{width:16px}.br-msg{flex:1;color:var(--text-primary)}.br-time{color:var(--text-muted);font-family:monospace}.batch-footer{display:flex;gap:6px;padding-top:8px;border-top:1px solid var(--border-color)}
.compare-panel{width:560px}.compare-body{padding:12px}.compare-cols{display:grid;grid-template-columns:1fr 30px 1fr;gap:0;margin-bottom:8px}.compare-col{padding:8px;background:rgba(255,255,255,0.02);border:1px solid var(--border-color);border-radius:var(--radius-sm)}.cc-header{font-size:11px;color:var(--color-primary);font-weight:600;margin-bottom:4px}.cc-sql{font-size:10px;color:#7fdbca;font-family:monospace;white-space:pre-wrap;max-height:150px;overflow-y:auto;margin:0}.compare-arrow{text-align:center;color:var(--text-muted);align-self:center;font-size:18px}.compare-footer{display:flex;gap:6px;padding-top:8px;border-top:1px solid var(--border-color)}
.stats-panel{width:480px}.stats-body{padding:12px}.stats-grid{display:grid;grid-template-columns:repeat(3,1fr);gap:8px;margin-bottom:12px}.stat-card{padding:10px;border-radius:var(--radius-sm);background:rgba(59,130,246,0.1);border:1px solid rgba(59,130,246,0.2);text-align:center}.sc-val{font-size:20px;font-weight:700;color:var(--color-primary)}.sc-label{font-size:9px;color:var(--text-muted);margin-top:2px}.stats-chart{display:flex;align-items:flex-end;gap:4px;height:100px;padding:8px;background:rgba(255,255,255,0.02);border-radius:var(--radius-sm)}.chart-bar{display:flex;flex-direction:column;align-items:center;justify-content:flex-end;flex:1;border-radius:3px 3px 0 0;padding:2px;min-height:4px;position:relative}.cb-label{font-size:8px;color:var(--text-muted);position:absolute;bottom:-16px;white-space:nowrap}.cb-val{font-size:9px;color:var(--text-primary);margin-bottom:2px}
.param-panel{width:480px}.param-body{padding:12px;display:flex;flex-direction:column;gap:8px}.param-list{display:flex;flex-direction:column;gap:4px;max-height:180px;overflow-y:auto}.param-row{display:flex;align-items:center;gap:6px;padding:4px 8px;background:rgba(255,255,255,0.02);border-radius:4px;font-size:11px}.param-name{color:#f59e0b;width:80px;font-family:monospace;font-weight:600}.param-input{flex:1;padding:3px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}.param-type{padding:3px;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-sm);color:var(--text-primary);font-size:10px}.param-detect{padding:8px;background:rgba(245,158,11,0.05);border:1px solid rgba(245,158,11,0.2);border-radius:var(--radius-sm)}.pd-title{font-size:11px;color:#f59e0b;margin-bottom:4px}.pd-tag{padding:2px 8px;border-radius:10px;background:rgba(245,158,11,0.15);border:1px solid rgba(245,158,11,0.3);color:#f59e0b;font-size:10px;font-family:monospace;cursor:pointer;margin-right:4px}.pd-tag.exists{background:rgba(16,185,129,0.15);border-color:rgba(16,185,129,0.3);color:#10b981}.param-footer{display:flex;gap:6px;padding-top:8px;border-top:1px solid var(--border-color)}
.favorite-panel{width:420px}.fav-list{padding:12px;max-height:300px;overflow-y:auto;display:flex;flex-direction:column;gap:4px}.fav-item{display:flex;align-items:center;gap:8px;padding:6px 10px;background:rgba(255,255,255,0.02);border-radius:var(--radius-sm);cursor:pointer;font-size:12px;color:var(--text-primary)}.fav-item:hover{background:rgba(59,130,246,0.1)}.fi-star{font-size:14px}.fi-name{flex:1}.fi-cat{color:var(--text-muted);font-size:10px}.fav-empty{color:var(--text-muted);font-size:11px;text-align:center;padding:20px}
/* -- Visualization Panel -- */
.visual-panel{width:680px}.visual-tabs{display:flex;gap:4px;padding:8px 12px;border-bottom:1px solid var(--border-color)}.vis-tab{padding:4px 12px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-muted);cursor:pointer;font-size:11px}.vis-tab.active{background:var(--color-primary);color:#000;border-color:var(--color-primary)}.visual-config{display:flex;gap:8px;align-items:center;padding:8px 12px;border-bottom:1px solid var(--border-color)}.vis-select{flex:1;padding:6px;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-sm);color:var(--text-primary);font-size:12px}.visual-canvas{height:350px;padding:12px}.chart-container{width:100%;height:100%}.chart-error{padding:8px 12px;color:#ef4444;font-size:11px;background:rgba(239,68,68,0.1);border-radius:var(--radius-sm)}

/* -- Permission Panel -- */
.perm-panel{width:520px}.perm-body{padding:12px}.perm-header-row{display:flex;align-items:center;gap:8px;padding:6px 8px;background:rgba(59,130,246,0.1);border-radius:var(--radius-sm);margin-bottom:8px;font-size:11px;font-weight:600;color:var(--color-primary)}.perm-col{flex:1}.perm-col:nth-child(2),.perm-col:nth-child(3),.perm-col:nth-child(4){text-align:center;width:60px}.perm-row{display:flex;align-items:center;gap:8px;padding:6px 8px;border-radius:var(--radius-sm);background:rgba(255,255,255,0.02);margin-bottom:4px;font-size:11px}.perm-field{flex:1;color:var(--text-primary);font-family:monospace}.perm-check{position:relative;width:24px;height:16px;cursor:pointer}.perm-check input{opacity:0;width:0;height:0}.perm-check span{position:absolute;inset:0;background:var(--border-color);border-radius:8px;transition:.2s}.perm-check input:checked+span{background:#10b981}.perm-check span::before{content:'';position:absolute;width:12px;height:12px;left:2px;top:2px;background:#fff;border-radius:50%;transition:.2s}.perm-check input:checked+span::before{transform:translateX(8px)}.perm-footer{display:flex;gap:6px;padding-top:8px;border-top:1px solid var(--border-color)}

/* -- Execution Plan Panel -- */
.plan-panel{width:560px}.plan-body{padding:12px}.plan-loading{color:var(--text-muted);text-align:center;padding:20px}.plan-empty{color:var(--text-muted);font-size:12px;text-align:center;padding:20px}.plan-tree{display:flex;flex-direction:column;gap:6px;max-height:300px;overflow-y:auto}.plan-step{margin-bottom:4px}.ps-header{display:flex;align-items:center;gap:8px;padding:6px 10px;border-left:3px solid;border-radius:0 var(--radius-sm) var(--radius-sm) 0;background:rgba(255,255,255,0.02);font-size:11px}.ps-header.ps-sub{margin-left:20px;opacity:0.8}.ps-type{color:var(--color-primary);font-weight:600;min-width:80px}.ps-cost{min-width:60px}.ps-rows{color:var(--text-muted);font-size:10px}.ps-detail{font-size:10px;color:var(--text-muted);padding:2px 10px 4px 14px}.ps-children{margin-left:16px}.plan-warnings{margin-top:12px;padding:8px 12px;background:rgba(245,158,11,0.1);border:1px solid rgba(245,158,11,0.2);border-radius:var(--radius-sm)}.pw-title{font-size:11px;color:#f59e0b;font-weight:600;margin-bottom:4px}.pw-item{font-size:10px;color:var(--text-muted);padding:2px 0}

/* -- Template CRUD Panel -- */
.tmpl-crud-panel{width:560px}.tmpl-crud-body{padding:12px}.tmpl-crud-toolbar{display:flex;gap:6px;align-items:center;margin-bottom:10px}.tmpl-crud-list{display:flex;flex-direction:column;gap:6px;max-height:320px;overflow-y:auto}.tmpl-crud-item{display:flex;align-items:center;gap:8px;padding:8px 10px;background:rgba(255,255,255,0.02);border:1px solid var(--border-color);border-radius:var(--radius-sm);cursor:pointer}.tmpl-crud-item:hover{border-color:var(--color-primary);background:rgba(59,130,246,0.05)}.tci-icon{font-size:18px}.tci-info{flex:1}.tci-name{font-size:12px;color:var(--text-primary);font-weight:500}.tci-cat{font-size:10px;color:var(--text-muted)}.tci-actions{display:flex;gap:4px}
/* -- Condition Builder Panel -- */
.cond-builder-panel{width:560px}.cond-body{padding:12px}.cond-rules{display:flex;flex-direction:column;gap:6px;margin-bottom:10px}.cond-rule-row{display:flex;align-items:center;gap:6px}.cond-select{padding:4px 8px;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-sm);color:var(--text-primary);font-size:11px;min-width:100px}.cond-input{flex:1;padding:4px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}.cond-logic{display:flex;gap:12px;padding:8px;background:rgba(59,130,246,0.05);border-radius:var(--radius-sm);margin-bottom:8px;font-size:11px}

/* -- Drag Config Panel -- */
.drag-config-panel{width:520px}.drag-body{padding:12px}.drag-columns{display:grid;grid-template-columns:1fr 1fr;gap:12px;margin-bottom:10px}.drag-col{display:flex;flex-direction:column;gap:4px}.dc-title{font-size:11px;font-weight:600;color:var(--color-primary);margin-bottom:4px}.drag-item{padding:4px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:rgba(255,255,255,0.02);font-size:11px;color:var(--text-primary);cursor:grab;user-select:none}.drag-item:hover{border-color:var(--color-primary)}.drag-item.selected{background:rgba(59,130,246,0.1);border-color:var(--color-primary)}.drag-empty{color:var(--text-muted);font-size:11px;text-align:center;padding:20px}.drag-preview{padding:8px;background:rgba(0,0,0,0.2);border-radius:var(--radius-sm);margin-top:8px}.dp-label{font-size:10px;color:var(--text-muted);margin-bottom:4px}.dp-sql{margin:0;font-size:11px;color:#10b981;font-family:monospace;white-space:pre-wrap}.drag-actions{display:flex;gap:6px;margin-top:8px}

/* -- AST Panel -- */
.ast-panel{width:520px}.ast-body{padding:12px}.ast-loading{color:var(--text-muted);text-align:center;padding:20px}.ast-empty{color:var(--text-muted);font-size:12px;text-align:center;padding:20px}.ast-tree{display:flex;flex-direction:column;gap:4px;max-height:300px;overflow-y:auto}.ast-node{margin-bottom:2px}.ast-node-header{display:flex;align-items:center;gap:8px;padding:4px 10px;border-left:3px solid;border-radius:0 var(--radius-sm) var(--radius-sm) 0;background:rgba(255,255,255,0.02);font-size:11px}.ast-node-header.ast-sub{margin-left:20px;opacity:0.8}.ast-type{color:var(--color-primary);font-weight:600;min-width:80px}.ast-val{color:var(--text-muted);font-family:monospace;font-size:10px}.ast-children{margin-left:16px}.ast-summary{margin-top:12px;padding:8px 12px;background:rgba(59,130,246,0.1);border:1px solid rgba(59,130,246,0.2);border-radius:var(--radius-sm);display:flex;gap:16px;flex-wrap:wrap;font-size:11px}.ast-summary span{color:var(--text-muted)}.ast-summary strong{color:var(--color-primary)}

/* -- Linkage Panel -- */
.linkage-panel{width:480px}.linkage-body{padding:12px}.linkage-config{display:flex;flex-direction:column;gap:8px;margin-bottom:10px}.lc-row{display:flex;align-items:center;gap:8px;font-size:11px}.lc-row label{color:var(--text-muted);min-width:80px}.lc-select{flex:1;padding:4px 8px;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-sm);color:var(--text-primary);font-size:11px}.lc-input{flex:1;padding:4px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}.linkage-preview{margin-top:8px;padding:8px;background:rgba(0,0,0,0.2);border-radius:var(--radius-sm)}.lp-title{font-size:10px;color:var(--text-muted);margin-bottom:4px}.lp-list{max-height:100px;overflow-y:auto}.lp-item{font-size:10px;color:#7fdbca;font-family:monospace;padding:2px 0}

/* -- More Templates Panel -- */
.more-tmpl-panel{width:560px}.mt-grid{display:flex;flex-direction:column;gap:8px;padding:12px;max-height:360px;overflow-y:auto}.mt-card{background:rgba(255,255,255,0.02);border:1px solid var(--border-color);border-radius:var(--radius-md);overflow:hidden}.mt-header{display:flex;align-items:center;gap:6px;padding:6px 10px;background:rgba(59,130,246,0.1);border-bottom:1px solid var(--border-color)}.mt-icon{font-size:14px}.mt-name{flex:1;color:var(--text-primary);font-size:12px;font-weight:500}.mt-cat{font-size:10px;color:var(--color-primary);background:rgba(59,130,246,0.2);padding:1px 6px;border-radius:3px}.mt-code{margin:0;padding:8px 10px;background:rgba(0,0,0,0.3);color:#10b981;font-size:10px;font-family:monospace;white-space:pre-wrap;word-break:break-all;max-height:50px;overflow-y:auto}.mt-actions{display:flex;gap:4px;padding:6px 10px;border-top:1px solid var(--border-color)}

/* -- Column Summary Panel -- */
.col-summary-panel{width:520px}.col-summary-body{padding:12px;max-height:320px;overflow-y:auto;display:flex;flex-direction:column;gap:4px}.cs-row{display:flex;align-items:center;gap:8px;padding:4px 8px;background:rgba(255,255,255,0.02);border-radius:4px;font-size:11px}.cs-name{color:var(--color-primary);font-family:monospace;min-width:80px;font-weight:600}.cs-type{color:var(--text-muted);width:50px}.cs-nulls,.cs-distinct{color:var(--text-muted);width:70px}.cs-sum,.cs-avg{color:#10b981;font-family:monospace;width:70px}.cs-empty{color:var(--text-muted);font-size:11px;text-align:center;padding:20px}.cs-footer{display:flex;justify-content:flex-end;padding-top:8px;border-top:1px solid var(--border-color)}
</style>
