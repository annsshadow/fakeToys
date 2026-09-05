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
        <button class="btn btn-outline" @click="showVisualEditor=!showVisualEditor" title="SQL可视化编辑器">✏️ 可视化编辑器</button>
        <button class="btn btn-outline" @click="showRuleChain=!showRuleChain" title="规则链编辑器">🔗 规则链</button>
        <button class="btn btn-outline" @click="showFieldDrag=!showFieldDrag" title="字段配置器">📐 字段配置</button>
        <button class="btn btn-outline" @click="showChartLinkage=!showChartLinkage" title="图表联动配置">📊 图表联动</button>
        <button class="btn btn-outline" @click="showAdvancedTemplatesFn" title="高级模板库">📑 高级模板</button>
        <button class="btn btn-outline" @click="showSchemaPanel=!showSchemaPanel" :class="{active:showSchemaPanel}" title="数据源浏览器">🗂 数据源</button>
        <button class="btn btn-outline" @click="showTemplatePanel=!showTemplatePanel" :class="{active:showTemplatePanel}" title="SQL模板">📑 模板</button>
        <button class="btn btn-outline" @click="showHistoryPanel=!showHistoryPanel" :class="{active:showHistoryPanel}" title="执行历史">📜 历史</button>
        <button class="btn btn-outline" @click="showBatchPanel=!showBatchPanel" :class="{active:showBatchPanel}" title="批量执行">⚡ 批量</button>
        <button class="btn btn-outline" @click="showComparePanel=!showComparePanel" :class="{active:showComparePanel}" title="SQL对比">🔀 对比</button>
        <button class="btn btn-outline" @click="showStatsPanel=!showStatsPanel" :class="{active:showStatsPanel}" title="执行统计">📈 统计</button>
        <button class="btn btn-outline" @click="showParamPanel=!showParamPanel" :class="{active:showParamPanel}" title="参数绑定">🔗 参数</button>
        <button class="btn btn-outline" @click="showFavoritePanel=!showFavoritePanel" :class="{active:showFavoritePanel}" title="收藏语句">⭐ 收藏</button>
        <button class="btn btn-outline" @click="showDebugConsole=!showDebugConsole" title="调试控制台">🐛 调试</button>
        <button class="btn btn-outline" @click="showSqlFormatter=true" title="SQL格式化器">📐 格式化器</button>
        <button class="btn btn-outline" @click="showSqlValidator=true" title="SQL语法验证">✅ 验证</button>
        <button class="btn btn-outline" @click="showResultViz=true" title="结果可视化">📊 可视化</button>
        <button class="btn btn-outline" @click="showSnippetLibrary=true" title="SQL片段库">📝 片段</button>        <button class="btn btn-outline" @click="showExecPlan=true" title="执行计划">🔬 计划</button>        <button class="btn btn-outline" @click="showBookmark=!showBookmark" title="书签">⭐ 书签</button>        <button class="btn btn-outline" @click="showSqlHints=true" title="智能提示">💡 提示</button>
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

    <!-- Visual SQL Editor -->
    <div v-if="showVisualEditor" class="modal-overlay" @click.self="showVisualEditor=false">
      <div class="modal-box visual-editor-panel">
        <div class="modal-header"><span>✏️ SQL可视化编辑器</span><button class="btn-close" @click="showVisualEditor=false">✕</button></div>
        <div class="ve-body">
          <div class="ve-section">
            <div class="ve-section-title">SELECT 字段</div>
            <div class="ve-fields">
              <span v-for="(f,fi) in veSelectFields" :key="f" class="ve-field-tag" @click="veSelectFields.splice(fi,1)">✕ {{ f }}</span>
              <button class="ve-add-btn" @click="addVeSelectField()">+ 添加字段</button>
            </div>
          </div>
          <div class="ve-section">
            <div class="ve-section-title">FROM 表</div>
            <select v-model="veFromTable" class="ve-select">
              <option value="">选择表...</option>
              <option v-for="t in allTables" :key="t.name" :value="t.name">{{ t.name }}</option>
            </select>
          </div>
          <div class="ve-section">
            <div class="ve-section-title">WHERE 条件</div>
            <div v-for="(c,ci) in veWhereConditions" :key="ci" class="ve-condition-row">
              <select v-model="c.field" class="ve-select-sm"><option v-for="h in resultHeaders" :value="h">{{ h }}</option></select>
              <select v-model="c.op" class="ve-select-sm"><option value="eq">=</option><option value="gt">></option><option value="lt"><</option><option value="like">LIKE</option></select>
              <input v-model="c.value" class="ve-input-sm" placeholder="值" />
              <button class="ve-del-btn" @click="veWhereConditions.splice(ci,1)">✕</button>
            </div>
            <button class="ve-add-btn" @click="addVeWhereCondition()">+ 添加条件</button>
          </div>
          <div class="ve-section">
            <div class="ve-section-title">排序与分页</div>
            <select v-model="veOrderBy" class="ve-select-sm"><option value="">排序字段...</option><option v-for="h in resultHeaders" :value="h">{{ h }}</option></select>
            <select v-model="veOrderDir" class="ve-select-sm"><option value="ASC">升序</option><option value="DESC">降序</option></select>
            <input v-model.number="veLimit" type="number" class="ve-input-sm" placeholder="LIMIT" min="1" max="10000" />
          </div>
          <div class="ve-preview">
            <div class="ve-preview-label">预览SQL:</div>
            <pre class="ve-preview-sql">{{ generatedVisualSql }}</pre>
          </div>
          <div class="ve-actions">
            <button class="btn-sm" @click="applyVisualEditor()">✓ 应用到编辑器</button>
            <button class="btn-sm btn-outline" @click="clearVisualEditor()">清空</button>
          </div>
        </div>
      </div>
    </div>

    <!-- Rule Chain Editor -->
    <div v-if="showRuleChain" class="modal-overlay" @click.self="showRuleChain=false">
      <div class="modal-box rule-chain-panel">
        <div class="modal-header"><span>🔗 规则链编辑器</span><button class="btn-close" @click="showRuleChain=false">✕</button></div>
        <div class="rc-body">
          <div class="rc-chain">
            <div v-for="(rule,ri) in ruleChain" :key="ri" class="rc-rule">
              <div class="rc-rule-header">
                <span class="rc-rule-num">#{{ ri+1 }}</span>
                <span class="rc-rule-type">{{ rule.type }}</span>
                <span :class="{active:rule.enabled,inactive:!rule.enabled}">{{ rule.enabled ? "启用" : "禁用" }}</span>
                <button class="btn-xs" @click="toggleRule(ri)">{{ rule.enabled ? "禁用" : "启用" }}</button>
                <button class="btn-xs btn-danger" @click="ruleChain.splice(ri,1)">🗑</button>
              </div>
              <div class="rc-rule-body">
                <div class="rc-row"><label>字段:</label><select v-model="ruleChain[ri].field" class="rc-select"><option v-for="h in resultHeaders" :value="h">{{ h }}</option></select></div>
                <div class="rc-row"><label>操作:</label><select v-model="ruleChain[ri].op" class="rc-select"><option value="eq">=</option><option value="gt">></option><option value="lt"><</option><option value="like">LIKE</option></select></div>
                <div class="rc-row"><label>值:</label><input v-model="ruleChain[ri].value" class="rc-input" placeholder="值..." /></div>
              </div>
            </div>
          </div>
          <button class="btn-sm" @click="addRuleToChain()">+ 添加规则</button>
          <button class="btn-sm" @click="applyRuleChain()">✓ 应用规则链</button>
        </div>
      </div>
    </div>

    <!-- Field Drag Panel -->
    <div v-if="showFieldDrag" class="modal-overlay" @click.self="showFieldDrag=false">
      <div class="modal-box field-drag-panel">
        <div class="modal-header"><span>📐 字段配置器</span><button class="btn-close" @click="showFieldDrag=false">✕</button></div>
        <div class="fd-body">
          <div class="fd-columns">
            <div class="fd-col"><div class="fd-title">所有字段</div>
              <div v-for="f in allSchemaFields" :key="f" class="fd-item">{{ f }}</div>
            </div>
            <div class="fd-col"><div class="fd-title">SELECT 字段</div>
              <div class="fd-target">
                <div v-for="(f,fi) in fdSelectFields" :key="f" class="fd-target-item">{{ f }} <span class="fd-x" @click="fdSelectFields.splice(fi,1)">✕</span></div>
                <div v-if="fdSelectFields.length===0" class="fd-hint">拖拽到此处</div>
              </div>
            </div>
            <div class="fd-col"><div class="fd-title">WHERE 字段</div>
              <div class="fd-target">
                <div v-for="(f,fi) in fdWhereFields" :key="f" class="fd-target-item">{{ f }} <span class="fd-x" @click="fdWhereFields.splice(fi,1)">✕</span></div>
                <div v-if="fdWhereFields.length===0" class="fd-hint">拖拽到此处</div>
              </div>
            </div>
          </div>
          <div class="fd-preview">
            <div class="dp-label">生成SQL:</div>
            <pre class="dp-sql">{{ generatedFieldDragSql }}</pre>
          </div>
          <div class="fd-actions">
            <button class="btn-sm" @click="fdApply()">✓ 应用</button>
            <button class="btn-sm" @click="fdAutoFill()">智能填充</button>
            <button class="btn-sm btn-outline" @click="fdReset()">重置</button>
          </div>
        </div>
      </div>
    </div>

    <!-- Chart Linkage Panel -->
    <div v-if="showChartLinkage" class="modal-overlay" @click.self="showChartLinkage=false">
      <div class="modal-box chart-linkage-panel">
        <div class="modal-header"><span>📊 图表联动配置</span><button class="btn-close" @click="showChartLinkage=false">✕</button></div>
        <div class="cl-body">
          <div class="cl-cards">
            <div class="cl-card"><div class="cl-card-title">联动模式</div><select v-model="clMode" class="cl-select"><option value="filter">点击过滤</option><option value="detail">点击查看</option><option value="compare">点击对比</option></select></div>
            <div class="cl-card"><div class="cl-card-title">X轴字段</div><select v-model="clXAxis" class="cl-select"><option v-for="h in resultHeaders" :value="h">{{ h }}</option></select></div>
            <div class="cl-card"><div class="cl-card-title">Y轴字段</div><select v-model="clYAxis" class="cl-select"><option v-for="h in numHeaders" :value="h">{{ h }}</option></select></div>
            <div class="cl-card"><div class="cl-card-title">过滤字段</div><input v-model="clFilterField" class="cl-input" placeholder="用于过滤的字段名" /></div>
          </div>
          <div class="cl-preview">
            <div class="cl-preview-title">预览:</div>
            <div v-for="(item,pi) in clPreviewData" :key="pi" class="cl-preview-item">{{ item }}</div>
            <div v-if="clPreviewData.length===0" class="cl-empty">配置后点击测试</div>
          </div>
          <div class="cl-actions">
            <button class="btn-sm" @click="applyChartLinkage()">✓ 应用</button>
            <button class="btn-sm" @click="testChartLinkage()">测试</button>
          </div>
        </div>
      </div>
    </div>

    <!-- Advanced Templates Panel -->
    <div v-if="showAdvancedTemplates" class="modal-overlay" @click.self="showAdvancedTemplates=false">
      <div class="modal-box adv-tmpl-panel">
        <div class="modal-header"><span>📑 高级SQL模板库</span><button class="btn-close" @click="showAdvancedTemplates=false">✕</button></div>
        <div class="adv-grid">
          <div v-for="(t,ti) in advancedTemplates" :key="t.id" class="adv-card">
            <div class="adv-header"><span class="adv-icon">{{ t.icon }}</span><span class="adv-name">{{ t.name }}</span><span class="adv-diff">{{ t.difficulty }}</span></div>
            <pre class="adv-code">{{ t.code }}</pre>
            <div class="adv-desc">{{ t.description }}</div>
            <div class="adv-actions"><button class="btn-sm" @click="applyAdvancedTemplate(t)">应用</button><button class="btn-sm" @click="saveAdvancedTemplate(t)">收藏</button></div>
          </div>
        </div>
      </div>
    </div>

    <!-- Debug Console -->
    <div v-if="showDebugConsole" class="modal-overlay" @click.self="showDebugConsole=false">
      <div class="modal-box debug-panel">
        <div class="modal-header"><span>🐛 调试控制台</span><button class="btn-close" @click="showDebugConsole=false">✕</button></div>
        <div class="debug-body">
          <div class="debug-tabs">
            <button :class="['dbg-tab',{active:dbgTab==='logs'}]" @click="dbgTab='logs'">日志</button>
            <button :class="['dbg-tab',{active:dbgTab==='vars'}]" @click="dbgTab='vars'">变量</button>
            <button :class="['dbg-tab',{active:dbgTab==='perf'}]" @click="dbgTab='perf'">性能</button>
          </div>
          <div v-if="dbgTab==='logs'" class="dbg-logs">
            <div v-for="(log,li) in debugLogs" :key="li" :class="['dbg-log','dbg-'+log.type]">
              <span class="dbg-time">{{log.time}}</span>
              <span class="dbg-msg">{{log.msg}}</span>
            </div>
            <div v-if="debugLogs.length===0" class="dbg-empty">暂无日志</div>
          </div>
          <div v-if="dbgTab==='vars'" class="dbg-vars">
            <div v-for="(v,ki) in dbgVarList" :key="ki" class="dbg-var-row">
              <span class="dbg-var-name">{{ki}}</span>
              <span class="dbg-var-val">{{String(v).substring(0,100)}}</span>
            </div>
          </div>
          <div v-if="dbgTab==='perf'" class="dbg-perf">
            <div class="perf-row"><span>总执行次数</span><span>{{execHistory.length}}</span></div>
            <div class="perf-row"><span>平均耗时</span><span>{{avgDuration}}ms</span></div>
            <div class="perf-row"><span>最大耗时</span><span>{{maxDuration}}ms</span></div>
            <div class="perf-row"><span>成功率</span><span>{{successRate}}</span></div>
            <div class="perf-row"><span>累计行数</span><span>{{totalRows}}</span></div>
          </div>
        </div>
        <div class="dbg-footer">
          <button class="btn-sm" @click="debugLogs=[]">清除日志</button>
          <button class="btn-sm" @click="showDebugConsole=false">关闭</button>
        </div>
      </div>
    </div>

    <!-- SQL Formatter -->
    <div v-if="showSqlFormatter" class="modal-overlay" @click.self="showSqlFormatter=false">
      <div class="modal-box fmt-panel">
        <div class="modal-header"><span>📐 SQL 格式化器</span><button class="btn-close" @click="showSqlFormatter=false">✕</button></div>
        <div class="fmt-body">
          <div class="fmt-cols">
            <div class="fmt-col">
              <div class="fmt-label">原始 SQL</div>
              <pre class="fmt-src">{{ sql || "(空)" }}</pre>
            </div>
            <div class="fmt-arrow">⇄</div>
            <div class="fmt-col">
              <div class="fmt-label">格式化结果</div>
              <pre class="fmt-out">{{ formattedSql }}</pre>
            </div>
          </div>
          <div class="fmt-opts">
            <label><input type="checkbox" v-model="fmtUpper" /> 大写关键字</label>
            <label><input type="checkbox" v-model="fmtIndent" /> 缩进排版</label>
          </div>
        </div>
        <div class="fmt-footer">
          <button class="btn-sm" @click="applyFormatted">✓ 应用</button>
          <button class="btn-sm" @click="copyFormatted()">📋 复制</button>
        </div>
      </div>
    </div>

    <!-- SQL Validator -->
    <div v-if="showSqlValidator" class="modal-overlay" @click.self="showSqlValidator=false">
      <div class="modal-box val-panel">
        <div class="modal-header"><span>✅ SQL 语法验证</span><button class="btn-close" @click="showSqlValidator=false">✕</button></div>
        <div class="val-body">
          <div class="val-result" :class="valResult.status">
            <span class="val-icon">{{valResult.status==='valid'?'✓':'✗'}}</span>
            <span>{{valResult.message}}</span>
          </div>
          <div class="val-checks">
            <div v-for="(c,ci) in valChecks" :key="ci" :class="['val-check',c.pass?'pass':'fail']">
              <span>{{c.pass?'✓':'✗'}}</span><span class="val-name">{{c.name}}</span><span class="val-detail">{{c.detail}}</span>
            </div>
          </div>
          <div class="val-sug" v-if="valSuggestions.length">
            <div class="val-sug-title">优化建议:</div>
            <div v-for="(s,si) in valSuggestions" :key="si" class="val-sug-item">• {{s}}</div>
          </div>
        </div>
        <div class="val-footer">
          <button class="btn-sm" :disabled="!sql.trim()" @click="runValidation()">▶ 验证</button>
          <button class="btn-sm" @click="showSqlValidator=false">关闭</button>
        </div>
      </div>
    </div>

    <!-- Result Visualization -->
    <div v-if="showResultViz" class="modal-overlay" @click.self="showResultViz=false">
      <div class="modal-box viz-panel">
        <div class="modal-header"><span>📊 结果可视化</span><button class="btn-close" @click="showResultViz=false">✕</button></div>
        <div class="viz-body">
          <div class="viz-controls">
            <select v-model="vizType" class="viz-select"><option value="bar">柱状图</option><option value="line">折线图</option><option value="pie">饼图</option></select>
            <select v-model="vizXAxis" class="viz-select"><option value="">X轴...</option><option v-for="h in resultHeaders" :key="h" :value="h">{{h}}</option></select>
            <select v-model="vizYAxis" class="viz-select"><option value="">Y轴...</option><option v-for="h in resultHeaders" :key="h" :value="h">{{h}}</option></select>
            <button class="btn-sm" @click="renderChart()">▶ 渲染</button>
          </div>
          <div class="viz-chart" v-if="vizRendered">
            <div v-for="(d,di) in vizBars" :key="di" class="viz-bar-wrap">
              <div class="viz-bar" :style="{height:Math.max(4,d.h)+'px',background:vizColors[di%8]}" :title="d.label+': '+d.value"></div>
              <div class="viz-bar-label">{{d.label}}</div>
              <div class="viz-bar-val">{{d.value}}</div>
            </div>
          </div>
          <div v-else class="viz-empty">{{resultData.length?'选择字段后点击渲染':'请先执行SQL'}}</div>
          <div class="viz-stats" v-if="vizStats">
            <div class="viz-stat"><span>总数</span><span>{{vizStats.count}}</span></div>
            <div class="viz-stat"><span>最大值</span><span>{{vizStats.max}}</span></div>
            <div class="viz-stat"><span>最小值</span><span>{{vizStats.min}}</span></div>
            <div class="viz-stat"><span>平均值</span><span>{{vizStats.avg}}</span></div>
          </div>
        </div>
        <div class="viz-footer">
          <button class="btn-sm" @click="exportVizData()">📥 导出CSV</button>
          <button class="btn-sm" @click="showResultViz=false">关闭</button>
        </div>
      </div>
    </div>

    <!-- SQL Snippet Library -->
    <div v-if="showSnippetLibrary" class="modal-overlay" @click.self="showSnippetLibrary=false">
      <div class="modal-box snippet-panel">
        <div class="modal-header"><span>📝 SQL 片段库</span><button class="btn-close" @click="showSnippetLibrary=false">✕</button></div>
        <div class="snippet-toolbar">
          <input v-model="snippetSearch" class="tmp-input" placeholder="搜索片段..." />
          <select v-model="snippetCat" class="tmp-select">
            <option value="all">全部分类</option>
            <option value="filter">WHERE过滤</option>
            <option value="join">JOIN</option>
            <option value="agg">聚合</option>
            <option value="window">窗口函数</option>
            <option value="cte">CTE</option>
          </select>
        </div>
        <div class="snippet-grid">
          <div v-for="(s,si) in filteredSnippets" :key="si" class="snippet-card">
            <div class="snippet-head"><span class="snippet-name">{{s.name}}</span><span class="snippet-cat">{{s.category}}</span></div>
            <pre class="snippet-code">{{s.code}}</pre>
            <div class="snippet-foot">
              <button class="btn-sm" @click="insertSnippet(s)">📋 插入</button>
              <button class="btn-sm" @click="copySnip(s.code)">📄 复制</button>
            </div>
          </div>
        </div>
        <div v-if="filteredSnippets.length===0" class="tmpl-empty">暂无片段</div>
      </div>
    </div>


    <!-- Execution Plan -->
    <div v-if="showExecPlan" class="modal-overlay" @click.self="showExecPlan=false">
      <div class="modal-box plan-panel">
        <div class="modal-header"><span>🔬 执行计划分析</span><button class="btn-close" @click="showExecPlan=false">✕</button></div>
        <div class="plan-body">
          <div v-if="planSteps.length" class="plan-steps">
            <div v-for="(step,si) in planSteps" :key="si" :class="['plan-step',{active:si===activeStep}]">
              <div class="plan-num">{{si+1}}</div>
              <div class="plan-content">
                <div class="plan-type">{{step.type}}</div>
                <div class="plan-desc">{{step.desc}}</div>
                <div class="plan-detail" v-if="step.detail">{{step.detail}}</div>
              </div>
              <div class="plan-arrow" v-if="si<planSteps.length-1">↓</div>
            </div>
          </div>
          <div v-else class="plan-empty">点击「生成计划」分析当前SQL</div>
          <button class="btn-sm" @click="generatePlan()">🔍 生成执行计划</button>
        </div>
        <div class="plan-footer">
          <button class="btn-sm" @click="showExecPlan=false">关闭</button>
        </div>
      </div>
    </div>

    <!-- SQL Diff Tool -->
    <div v-if="showSqlDiff" class="modal-overlay" @click.self="showSqlDiff=false">
      <div class="modal-box diff-panel">
        <div class="modal-header"><span>🔀 SQL 对比工具</span><button class="btn-close" @click="showSqlDiff=false">✕</button></div>
        <div class="diff-body">
          <div class="diff-cols">
            <div class="diff-col">
              <div class="diff-title">原始 SQL</div>
              <textarea v-model="diffLeft" class="diff-textarea" placeholder="粘贴原始SQL..."></textarea>
            </div>
            <div class="diff-col">
              <div class="diff-title">当前 SQL</div>
              <textarea v-model="diffRight" class="diff-textarea" placeholder="粘贴修改后SQL..."></textarea>
            </div>
          </div>
          <button class="btn-sm" @click="computeDiff()">▶ 对比分析</button>
          <div v-if="diffLines.length" class="diff-result">
            <div v-for="(d,di) in diffLines" :key="di" :class="['diff-line',d.type]">
              <span class="diff-num">{{d.line}}</span>
              <span class="diff-text">{{d.text}}</span>
            </div>
          </div>
        </div>
        <div class="diff-footer">
          <button class="btn-sm" @click="applyDiffRight()">→ 应用右侧</button>
          <button class="btn-sm" @click="showSqlDiff=false">关闭</button>
        </div>
      </div>
    </div>

    <!-- Export/Import -->
    <div v-if="showExportImport" class="modal-overlay" @click.self="showExportImport=false">
      <div class="modal-box expimp-panel">
        <div class="modal-header"><span>📤 导入/导出</span><button class="btn-close" @click="showExportImport=false">✕</button></div>
        <div class="expimp-tabs">
          <button :class="['ei-tab',{active:eiTab==='export'}]" @click="eiTab='export'">导出</button>
          <button :class="['ei-tab',{active:eiTab==='import'}]" @click="eiTab='import'">导入</button>
        </div>
        <div v-if="eiTab==='export'" class="ei-body">
          <div class="ei-option"><label>格式:</label>
            <select v-model="exportFmt" class="ei-select">
              <option value="json">JSON</option><option value="sql">SQL文件</option><option value="csv">CSV</option>
            </select>
          </div>
          <div class="ei-count">{{statements.length}} 条语句待导出</div>
          <button class="btn-sm" @click="doExport()">📥 导出到文件</button>
        </div>
        <div v-if="eiTab==='import'" class="ei-body">
          <textarea v-model="importData" class="ei-textarea" placeholder="粘贴JSON数据..."></textarea>
          <div v-if="importMsg" :class="['ei-msg',importMsg.ok?'ok':'err']">{{importMsg.txt}}</div>
          <button class="btn-sm" @click="doImport()">📤 导入</button>
        </div>
        <div class="ei-footer"><button class="btn-sm" @click="showExportImport=false">关闭</button></div>
      </div>
    </div>

    <!-- Bulk Delete -->
    <div v-if="showBulkDelete" class="modal-overlay" @click.self="showBulkDelete=false">
      <div class="modal-box bulk-panel">
        <div class="modal-header"><span>🗑 批量删除确认</span><button class="btn-close" @click="showBulkDelete=false">✕</button></div>
        <div class="bulk-body">
          <p>确定删除选中的 <strong>{{bulkIds.length}}</strong> 条语句？此操作不可恢复。</p>
          <div class="bulk-list">
            <div v-for="id in bulkIds" class="bulk-item">{{statements.find(s=>s.id===id)?.name||id}}</div>
          </div>
        </div>
        <div class="bulk-footer">
          <button class="btn-sm btn-danger" @click="confirmBulkDelete()">✓ 确认删除</button>
          <button class="btn-sm" @click="showBulkDelete=false">取消</button>
        </div>
      </div>
    </div>


    <!-- Bookmark Panel -->
    <div v-if="showBookmark" class="modal-overlay" @click.self="showBookmark=false">
      <div class="modal-box bookmark-panel">
        <div class="modal-header"><span>⭐ SQL 书签</span><button class="btn-close" @click="showBookmark=false">✕</button></div>
        <div class="bookmark-body">
          <div class="bm-add">
            <input v-model="bmName" class="bm-input" placeholder="书签名称..." />
            <button class="btn-sm" @click="addBookmark()">+ 添加</button>
          </div>
          <div class="bm-list">
            <div v-for="(b,bi) in bookmarks" :key="bi" class="bm-item">
              <span class="bm-icon">⭐</span>
              <span class="bm-name">{{b.name}}</span>
              <span class="bm-time">{{fmtTime(b.ts)}}</span>
              <button class="bm-btn" @click="loadBookmark(bi)">加载</button>
              <button class="bm-btn bm-del" @click="deleteBookmark(bi)">✕</button>
            </div>
          </div>
          <div v-if="bookmarks.length===0" class="bm-empty">暂无书签</div>
        </div>
        <div class="bm-footer"><button class="btn-sm" @click="showBookmark=false">关闭</button></div>
      </div>
    </div>

    <!-- Template CRUD -->
    <div v-if="showTemplateCRUD" class="modal-overlay" @click.self="showTemplateCRUD=false">
      <div class="modal-box tplcrud-panel">
        <div class="modal-header"><span>📑 模板管理</span><button class="btn-close" @click="showTemplateCRUD=false">✕</button></div>
        <div class="tplcrud-toolbar">
          <input v-model="tplSearch" class="tmp-input" placeholder="搜索模板..." />
          <select v-model="tplCat" class="tmp-select">
            <option value="all">全部分类</option>
            <option v-for="c in tplCategories" :key="c" :value="c">{{c}}</option>
          </select>
          <button class="btn-sm" @click="openTplEditor(null)">+ 新建</button>
        </div>
        <div class="tpl-list">
          <div v-for="(t,ti) in filteredTpls" :key="t.id" class="tpl-card">
            <div class="tpl-head"><span class="tpl-icon">{{t.icon||'📋'}}</span><span class="tpl-name">{{t.name}}</span><span class="tpl-cat">{{t.category}}</span></div>
            <pre class="tpl-code">{{t.code}}</pre>
            <div class="tpl-foot">
              <button class="btn-sm" @click="applyTemplate(t)">应用</button>
              <button class="btn-sm" @click="editTemplate(t)">编辑</button>
              <button class="btn-sm btn-danger" @click="deleteTpl(ti)">删除</button>
            </div>
          </div>
        </div>
        <div v-if="filteredTpls.length===0" class="tmpl-empty">暂无模板</div>
      </div>
    </div>

    <!-- Template Editor Modal -->
    <div v-if="showTplEditor" class="modal-overlay" @click.self="showTplEditor=false">
      <div class="modal glass-card">
        <h3>{{tplEditingId?'编辑模板':'新建模板'}}</h3>
        <div class="form-group"><label>名称</label><input v-model="tplForm.name" class="form-input" placeholder="模板名称" /></div>
        <div class="form-group"><label>分类</label>
          <select v-model="tplForm.category" class="form-input">
            <option value="select">SELECT</option><option value="join">JOIN</option><option value="agg">聚合</option><option value="window">窗口函数</option><option value="cte">CTE</option><option value="admin">管理</option>
          </select>
        </div>
        <div class="form-group"><label>图标</label><input v-model="tplForm.icon" class="form-input" placeholder="emoji" /></div>
        <div class="form-group"><label>SQL</label><textarea v-model="tplForm.code" class="form-textarea" rows="8" placeholder="SELECT ..."></textarea></div>
        <div class="modal-actions">
          <button class="btn-cancel" @click="showTplEditor=false">取消</button>
          <button class="btn-save" :disabled="!tplForm.name" @click="saveTpl">保存</button>
        </div>
      </div>
    </div>


    <!-- Parameter Presets -->
    <div v-if="showParamPresets" class="modal-overlay" @click.self="showParamPresets=false">
      <div class="modal-box param-panel">
        <div class="modal-header"><span>🔗 参数预设管理</span><button class="btn-close" @click="showParamPresets=false">✕</button></div>
        <div class="param-body">
          <div class="param-list">
            <div v-for="(p,pi) in paramPresets" :key="p.id" class="param-row">
              <span class="param-name">{{p.name}}</span>
              <input :value="p.value" @input="paramPresets[pi].value=$event.target.value" class="param-input" :placeholder="'默认:'+p.defaultValue" />
              <select v-model="paramPresets[pi].type" class="param-type">
                <option value="string">STRING</option><option value="number">NUMBER</option><option value="date">DATE</option>
              </select>
              <button class="param-del" @click="paramPresets.splice(pi,1)">✕</button>
            </div>
          </div>
          <div class="param-detect">
            <div class="pd-title">从当前SQL检测:</div>
            <div class="pd-tags">
              <span v-for="dp in detectedParams" :key="dp" :class="['pd-tag',paramPresets.some(pp=>pp.name===dp)?'exists':'']" @click="addParamPreset(dp)">{{dp}}</span>
            </div>
            <button class="btn-sm" @click="addAllParams()">+ 全部添加</button>
          </div>
          <button class="btn-sm" @click="paramPresets.push({id:'p'+Date.now(),name:'',value:'',type:'string',defaultValue:''})">+ 添加参数</button>
        </div>
        <div class="param-footer">
          <button class="btn-sm" @click="applyParamPresets()">✓ 应用到SQL</button>
          <button class="btn-sm" @click="showParamPresets=false">关闭</button>
        </div>
      </div>
    </div>

    <!-- SQL Auto-Hint Panel -->
    <div v-if="showSqlHints" class="modal-overlay" @click.self="showSqlHints=false">
      <div class="modal-box hint-panel">
        <div class="modal-header"><span>💡 SQL 智能提示</span><button class="btn-close" @click="showSqlHints=false">✕</button></div>
        <div class="hint-body">
          <div class="hint-section">
            <div class="hint-title">常用表</div>
            <div class="hint-tags">
              <span v-for="t in allTables" :key="t.name" class="hint-tag" @click="insertHint(t.name)">{{t.name}}</span>
            </div>
          </div>
          <div class="hint-section" v-if="selectedTableForHints">
            <div class="hint-title">{{selectedTableForHints}} 字段</div>
            <div class="hint-tags">
              <span v-for="f in tableFieldsByTable(selectedTableForHints)||[]" :key="f.name" class="hint-tag" @click="insertHint(f.name)">{{f.name}}</span>
            </div>
          </div>
          <div class="hint-section">
            <div class="hint-title">常用关键字</div>
            <div class="hint-tags">
              <span v-for="kw in sqlKeywords" :key="kw" class="hint-tag" @click="insertHint(kw)">{{kw}}</span>
            </div>
          </div>
          <div class="hint-section">
            <div class="hint-title">常用函数</div>
            <div class="hint-tags">
              <span v-for="fn in sqlFunctions" :key="fn" class="hint-tag" @click="insertHint(fn)">{{fn}}</span>
            </div>
          </div>
        </div>
        <div class="hint-footer">
          <button class="btn-sm" @click="showSqlHints=false">关闭</button>
        </div>
      </div>
    </div>

</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useQuery, useMutation, useQueryClient } from '@tanstack/vue-query'
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
  if (!confirmMsg(`删除语句「${s.name||s.id}」？`)) return
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
    toast.error('执行失败: : ' + (e?.message ?? '未知错误'))
    lastExecDuration.value = Date.now() - t0
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
const lastExecDuration = ref(0)
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

// --- Enhanced Result Statistics ---
const resultNumericStats = computed(() => {
  if (!resultHeaders.value.length || !resultData.value.length) return {}
  const stats: Record<string, any> = {}
  resultHeaders.value.forEach(h => {
    const nums = resultData.value.map(r => Number(r[h])).filter(v => !isNaN(v))
    if (nums.length) {
      const sorted = [...nums].sort((a:number,b:number) => a-b)
      stats[h] = { min: sorted[0], max: sorted[sorted.length-1], mean: nums.reduce((a:number,b:number)=>a+b,0)/nums.length, median: sorted[Math.floor(sorted.length/2)] }
    }
  })
  return stats
})
const numResultHeaders = computed(() => resultHeaders.value.filter(h => {
  if (!resultData.value.length) return false
  const v = resultData.value[0][h]
  return typeof v === "number" || (!isNaN(Number(v)) && v !== null && v !== undefined)
}))
const stringResultHeaders = computed(() => resultHeaders.value.filter(h => {
  if (!resultData.value.length) return false
  const v = resultData.value[0][h]
  return typeof v === "string"
}))
const resultSizeKB = computed(() => {
  if (!resultData.value.length) return 0
  const str = JSON.stringify(resultData.value)
  return Math.round(str.length / 1024 * 10) / 10
})
  return stats
})


// --- Visual Editor State ---
const showVisualEditor = ref(false)
const veSelectFields = ref<string[]>(["id", "name"])
const veFromTable = ref(""), veOrderBy = ref(""), veOrderDir = ref("DESC")
const veWhereConditions = ref<Array<{field:string;op:string;value:string}>>([])
const veLimit = ref(100)

// --- Rule Chain State ---
const showRuleChain = ref(false)
const ruleChain = ref<Array<{type:string;field:string;op:string;value:string;enabled:boolean}>>([])

// --- Field Drag State ---
const showFieldDrag = ref(false)
const allSchemaFields = ref<string[]>([])
const fdSelectFields = ref<string[]>([])
const fdWhereFields = ref<string[]>([])

// --- Chart Linkage State ---
const showChartLinkage = ref(false)
const clMode = ref("filter"), clXAxis = ref(""), clYAxis = ref(""), clFilterField = ref("")
const clPreviewData = ref<string[]>([])

// --- Advanced Templates State ---
const showAdvancedTemplates = ref(false)

// Computed
const generatedVisualSql = computed(() => {
  let s = "SELECT " + (veSelectFields.value.length ? veSelectFields.value.join(", ") : "*")
  if (veFromTable.value) s += " FROM " + veFromTable.value
  if (veWhereConditions.value.length) {
    const wh = veWhereConditions.value.filter(c => c.field && c.value).map(c => c.field + " " + c.op + " " + String.fromCharCode(39) + c.value + String.fromCharCode(39)).join(" AND ")
    if (wh) s += "\nWHERE " + wh
  }
  if (veOrderBy.value) s += "\nORDER BY " + veOrderBy.value + " " + veOrderDir.value
  if (veLimit.value) s += "\nLIMIT " + veLimit.value
  return s
})
const generatedFieldDragSql = computed(() => {
  let s = "SELECT " + (fdSelectFields.value.length ? fdSelectFields.value.join(", ") : "*")
  if (allSchemaFields.value[0]) s += " FROM " + allSchemaFields.value[0].split(".")[0]
  if (fdWhereFields.value.length) s += "\nWHERE " + fdWhereFields.value.map(f => f + " IS NOT NULL").join(" AND ")
  return s
})

// Functions
function addVeSelectField() { veSelectFields.value.push("") }
function addVeWhereCondition() { veWhereConditions.value.push({ field: "", op: "eq", value: "" }) }
function applyVisualEditor() { sql.value = generatedVisualSql.value; showVisualEditor.value = false }
function clearVisualEditor() { veSelectFields.value = []; veFromTable.value = ""; veWhereConditions.value = []; veOrderBy.value = ""; veLimit.value = 100 }
function addRuleToChain() { ruleChain.value.push({ type: "过滤", field: "", op: "eq", value: "", enabled: true }) }
function toggleRule(idx: number) { ruleChain.value[idx].enabled = !ruleChain.value[idx].enabled }
function applyRuleChain() {
  const r = ruleChain.value.filter(x => x.enabled && x.field && x.value)
  if (!r.length) return
  const w = r.map(x => x.field + " " + x.op + " " + String.fromCharCode(39) + x.value + String.fromCharCode(39)).join(" AND ")
  if (/WHERE/i.test(sql.value)) sql.value = sql.value.replace(/WHEREs+[^;]+/i, w)
  else sql.value += "\nWHERE " + w
  showRuleChain.value = false
}
function fdApply() { sql.value = generatedFieldDragSql.value; showFieldDrag.value = false }
function fdAutoFill() { fdSelectFields.value = resultHeaders.value.slice(0,5); fdWhereFields.value = resultHeaders.value.filter(h=>h.includes("status")||h.includes("flag")).slice(0,2) }
function fdReset() { fdSelectFields.value = []; fdWhereFields.value = [] }
function applyChartLinkage() { showChartLinkage.value = false }
function testChartLinkage() {
  if (clXAxis.value && clFilterField.value) clPreviewData.value = resultData.value.slice(0,3).map(r => r[clXAxis.value] + " | " + r[clFilterField.value]).filter(Boolean)
  else clPreviewData.value = ["请先选择X轴和过滤字段"]
}
function showAdvancedTemplatesFn() { showAdvancedTemplates.value = true }
function applyAdvancedTemplate(t: any) { sql.value = t.code + "\n"; showAdvancedTemplates.value = false }
function saveAdvancedTemplate(t: any) { templates.value.push({id:"t"+Date.now(),name:t.name,category:t.category,code:t.code,icon:t.icon}); showAdvancedTemplates.value = false }

// --- Debug Console ---
const showDebugConsole = ref(false)
const dbgTab = ref("logs")
const debugLogs = ref<Array<{type:'info'|'warn'|'error';msg:string;time:string}>>([])
const dbgVarList = computed(() => ({
  sqlLength: sql.value.length, rowCount: resultData.value.length,
  filter: filter.value, hasResults: hasResults.value, loading: loading.value
}))
function dbgLog(type: 'info'|'warn'|'error', msg: string) {
  const now = new Date().toLocaleTimeString('zh-CN')
  debugLogs.value.unshift({type, msg, time: now})
}

// --- SQL Formatter ---
const showSqlFormatter = ref(false)
const fmtUpper = ref(true), fmtIndent = ref(true)
const formattedSql = computed(() => formatSql(sql.value))
function formatSql(raw: string): string {
  if (!raw.trim()) return raw
  let s = raw.trim()
  if (fmtUpper.value) s = s.replace(/\b(SELECT|FROM|WHERE|AND|OR|ORDER BY|GROUP BY|HAVING|LIMIT|JOIN|LEFT|RIGHT|INNER|ON|SET|VALUES|INSERT|INTO|DELETE|UNION|NOT|NULL|IN|LIKE|CASE|WHEN|THEN|ELSE|END)\b/gi, m => m.toUpperCase())
  const kw = ['SELECT','FROM','WHERE','AND','OR','ORDER BY','GROUP BY','HAVING','LIMIT','JOIN','LEFT JOIN','RIGHT JOIN','INNER JOIN','ON','SET','VALUES','INSERT INTO','DELETE FROM','UNION ALL','UNION']
  for (const k of kw) {
    const re = new RegExp(k.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'gi')
    s = s.replace(re, '\n' + k + ' ')
  }
  s = s.replace(/\n\s*\n/g, '\n').trim()
  if (fmtIndent.value) {
    let indent = 0
    s = s.split('\n').map(l => {
      const t = l.trim()
      if (!t) return ''
      let p = '  '.repeat(indent)
      if (t.startsWith(')')) indent = Math.max(0, indent - 1)
      const line = p + t
      if (t.endsWith('(') || t.endsWith(',')) indent++
      return line
    }).join('\n')
  }
  return s
}
function applyFormatted() { sql.value = formattedSql.value; showSqlFormatter.value = false }
function copyFormatted() { navigator.clipboard.writeText(formattedSql.value) }

// --- SQL Validator ---
const showSqlValidator = ref(false)
const valResult = ref<{status:'valid'|'error';message:string}>({status:'pending',message:'点击下方按钮验证'})
const valChecks = ref<Array<{name:string;pass:boolean;detail:string}>>([])
const valSuggestions = ref<string[]>([])
function runValidation() {
  const checks: typeof valChecks.value = []
  const sug: typeof valSuggestions.value = []
  const sl = sql.value.toLowerCase().trim()
  checks.push({ name: 'SQL非空', pass: !!sql.value.trim(), detail: sql.value.trim() ? '有内容' : '无内容' })
  checks.push({ name: 'SELECT关键字', pass: /\bselect\b/.test(sl), detail: sl.includes('select') ? '已包含' : '缺失' })
  checks.push({ name: 'FROM子句', pass: /\bfrom\b/.test(sl), detail: sl.includes('from') ? '已包含' : '缺失' })
  checks.push({ name: '括号匹配', pass: (sql.value.match(/\(/g)||[]).length === (sql.value.match(/\)/g)||[]).length, detail: `左${(sql.value.match(/\(/g)||[]).length} 右${(sql.value.match(/\)/g)||[]).length}` })
  checks.push({ name: '分号结尾', pass: sl.endsWith(';'), detail: sl.endsWith(';') ? '有分号' : '建议加分号' })
  if (!/\blimit\s/i.test(sl)) sug.push('缺少LIMIT，建议限制返回行数')
  if (/select\s+\*/.test(sl) && !/from\s+\w+\s+join/i.test(sl)) sug.push('使用SELECT *可能影响性能')
  if (!/\bwhere\s/i.test(sl) && !/\blimit\s/i.test(sl)) sug.push('无WHERE和LIMIT，可能返回大量数据')
  const hasErr = !/\bselect\b/.test(sl) || !/\bfrom\b/.test(sl)
  valResult.value = { status: hasErr ? 'error' : 'valid', message: hasErr ? '存在语法问题' : '语法验证通过' }
  valChecks.value = checks
  valSuggestions.value = sug
  dbgLog(hasErr ? 'error' : 'info', '验证结果: ' + (hasErr ? '失败' : '通过'))
}

// --- Result Visualization ---
const showResultViz = ref(false)
const vizType = ref("bar"), vizXAxis = ref(""), vizYAxis = ref("")
const vizRendered = ref(false)
const vizColors = ["#3b82f6","#10b981","#f59e0b","#ef4444","#8b5cf6","#ec4899","#06b6d4","#f97316"]
const vizBars = ref<Array<{label:string;value:number;h:number}>>([])
const vizStats = ref<{count:number;max:number;min:number;avg:number}|null>(null)
function renderChart() {
  if (!resultData.value.length || !vizXAxis.value || !vizYAxis.value) return
  const map = new Map<string,number>()
  resultData.value.forEach(r => {
    const key = String(r[vizXAxis.value])
    const val = Number(r[vizYAxis.value]) || 0
    map.set(key, (map.get(key) || 0) + val)
  })
  const entries = [...map.entries()].sort((a,b) => b[1]-a[1]).slice(0, 20)
  const maxVal = Math.max(1, ...entries.map(([,v]) => v))
  const nums = entries.map(([,v]) => v)
  vizBars.value = entries.map(([label, value], i) => ({ label, value, h: Math.round(value/maxVal*140) }))
  vizStats.value = { count: resultData.value.length, max: Math.max(...nums), min: Math.min(...nums), avg: Math.round(nums.reduce((a:number,b:number)=>a+b,0)/nums.length) }
  vizRendered.value = true
  dbgLog('info', '图表已渲染: ' + entries.length + ' 个数据点')
}
function exportVizData() {
  if (!vizBars.value.length) return
  const csv = 'label,value\n' + vizBars.value.map(d => d.label+','+d.value).join('\n')
  const blob = new Blob([csv], {type:'text/csv'})
  const a = document.createElement('a'); a.href = URL.createObjectURL(blob)
  a.download = 'chart_data.csv'; a.click()
}

// --- Snippet Library ---
const showSnippetLibrary = ref(false)
const snippetSearch = ref(""), snippetCat = ref("all")
const snippetLibrary = ref<Array<{name:string;category:string;code:string}>>([
  {name:"日期范围过滤",category:"filter",code:"WHERE created_at BETWEEN '2024-01-01' AND '2024-12-31'\n  AND status IN ('active','pending')"},
  {name:"模糊搜索",category:"filter",code:"WHERE name LIKE '%关键词%'\n  OR description ILIKE '%关键词%'"},
  {name:"左连接防重复",category:"join",code:"LEFT JOIN orders o ON u.id = o.user_id\n  AND o.status != 'cancelled'"},
  {name:"计数聚合",category:"agg",code:"SELECT dept_id,\n  COUNT(*) as total,\n  SUM(amount) as total_amount,\n  AVG(amount) as avg_amount\nFROM orders GROUP BY dept_id"},
  {name:"排名分析",category:"window",code:"SELECT *,\n  RANK() OVER (PARTITION BY dept_id ORDER BY salary DESC) as rank,\n  LAG(salary, 1) OVER (ORDER BY salary) as prev_sal\nFROM employees"},
  {name:"递归层级",category:"cte",code:"WITH RECURSIVE tree AS (\n  SELECT id, name, manager_id, 1 as lvl\n  FROM employees WHERE manager_id IS NULL\n  UNION ALL\n  SELECT e.id, e.name, e.manager_id, t.lvl+1\n  FROM employees e JOIN tree t ON e.manager_id = t.id\n)\nSELECT * FROM tree ORDER BY lvl"},
  {name:"累计求和",category:"window",code:"SELECT date, amount,\n  SUM(amount) OVER (ORDER BY date\n    ROWS UNBOUNDED PRECEDING) as cumulative\nFROM daily_sales"},
  {name:"TOP-N每组",category:"agg",code:"WITH ranked AS (\n  SELECT dept_id, name, salary,\n    RANK() OVER (PARTITION BY dept_id ORDER BY salary DESC) as r\n  FROM employees\n)\nSELECT * FROM ranked WHERE r <= 3"},
  {id:"at19",name:"并行查询优化",category:"optimize",icon:"⚡",code:"-- 并行查询优化示例\nSET max_parallel_workers_per_gather = 4;\nSELECT /*+ PARALLEL(4) */\n  dept_id, COUNT(*) as cnt\nFROM employees\nGROUP BY dept_id\nORDER BY cnt DESC"},
  {id:"at20",name:"物化视图刷新",category:"admin",icon:"🗄️",code:"-- 物化视图刷新\nREFRESH MATERIALIZED VIEW CONCURRENTLY\n  mv_sales_summary;\n\n-- 查看刷新时间\nSELECT * FROM pg_matviews\nWHERE matviewname = 'mv_sales_summary'"},
  {id:"at21",name:"JSON聚合查询",category:"analytics",icon:"🔑",code:"-- JSON聚合查询\nSELECT user_id,\n  jsonb_object_agg(key, value) as attrs,\n  array_agg(tag) as tags\nFROM user_metadata\nGROUP BY user_id\nHAVING count(*) > 1"},
  {id:"at22",name:"增量数据同步",category:"admin",icon:"🔄",code:"-- 增量数据同步\nINSERT INTO target_table\nSELECT * FROM source_table\nWHERE updated_at > :last_sync_time\nON CONFLICT (id)\nDO UPDATE SET\n  name = EXCLUDED.name,\n  updated_at = EXCLUDED.updated_at"},
  {id:"at23",name:"数据归档策略",category:"admin",icon:"📦",code:"-- 数据归档到历史表\nINSERT INTO orders_archive\nSELECT * FROM orders\nWHERE created_at < '2023-01-01'\nRETURNING id;\n\nDELETE FROM orders\nWHERE created_at < '2023-01-01';"},
  {id:"at24",name:"CAGR复合增长率",category:"report",icon:"📈",code:"-- CAGR复合年增长率\nSELECT product_name,\n  start_value, end_value,\n  years,\n  ROUND(\n    (POWER(end_value/start_value, 1.0/years) - 1) * 100, 2\n  ) as cagr_pct\nFROM product_growth"},
  {id:"at25",name:"用户留存分析",category:"analytics",icon:"👥",code:"-- 用户留存率分析\nWITH first_login AS (\n  SELECT user_id, MIN(created_at) as first_day\n  FROM user_events\n  WHERE action = 'signup'\n  GROUP BY user_id\n),\nlogins AS (\n  SELECT user_id, DATE(created_at) as login_day\n  FROM user_events\n  WHERE action = 'login'\n)\nSELECT \n  DATE_PART('day', l.login_day - f.first_day) as day_offset,\n  COUNT(DISTINCT l.user_id) as retained_users\nFROM first_login f\nJOIN logins l ON f.user_id = l.user_id\nGROUP BY 1 ORDER BY 1"},
  {id:"at26",name:"时间序列插值",category:"analytics",icon:"📉",code:"-- 时间序列数据插值\nSELECT time_bucket,\n  value,\n  COALESCE(value,\n    AVG(value) OVER (\n      ORDER BY time_bucket\n      ROWS BETWEEN 2 PRECEDING AND 2 FOLLOWING\n    )\n  ) as interpolated\nFROM time_series_data\nORDER BY time_bucket"},
])
const filteredSnippets = computed(() => {
  let list = snippetLibrary.value
  if (snippetSearch.value) {
    const q = snippetSearch.value.toLowerCase()
    list = list.filter(s => s.name.toLowerCase().includes(q) || s.code.toLowerCase().includes(q))
  }
  if (snippetCat.value !== "all") list = list.filter(s => s.category === snippetCat.value)
  return list
})
function insertSnippet(s: any) {
  sql.value += (sql.value.endsWith("\n") ? "" : "\n") + s.code + "\n"
  showSnippetLibrary.value = false
}
function copySnip(code: string) { navigator.clipboard.writeText(code); dbgLog('info', '片段已复制') }


// --- Execution Plan ---
const showExecPlan = ref(false)
const planSteps = ref<Array<{type:string;desc:string;detail?:string}>>([])
const activeStep = ref(0)
function generatePlan() {
  const sl = sql.value.toLowerCase()
  const steps: typeof planSteps.value = []
  if (/with\s/i.test(sl)) steps.push({type:"CTE解析",desc:"解析公用表表达式",detail:"递归或非递归CTE"})
  if (/\bselect\b/.test(sl)) steps.push({type:"选择阶段",desc:"解析SELECT列表",detail:"确定输出列和表达式"})
  if (/\bfrom\b/.test(sl)) steps.push({type:"FROM/JOIN",desc:"处理FROM和JOIN",detail:sl.includes("join")?"检测到JOIN操作":"单表扫描"})
  if (/\bwhere\b/.test(sl)) steps.push({type:"过滤阶段",desc:"应用WHERE条件",detail:"根据条件筛选行"})
  if (/\bgroup\s+by\b/.test(sl)) steps.push({type:"分组聚合",desc:"GROUP BY分组",detail:"可能的HASH GROUP或SORT GROUP"})
  if (/\bhaving\b/.test(sl)) steps.push({type:"HAVING过滤",desc:"HAVING二次筛选",detail:"对聚合结果进行筛选"})
  if (/\border\s+by\b/.test(sl)) steps.push({type:"排序阶段",desc:"ORDER BY排序",detail:"可能有文件排序或索引排序"})
  if (/\blimit\s/.test(sl) || /\boffset\b/.test(sl)) steps.push({type:"限制输出",desc:"LIMIT/OFFSET分页",detail:"控制返回行数"})
  if (/\bunion\b/.test(sl)) steps.push({type:"UNION操作",desc:"合并多个结果集",detail:"UNION ALL或去重UNION"})
  if (steps.length===0) steps.push({type:"默认",desc:"完整SQL解析",detail:"请执行SQL后查看实际执行计划"})
  planSteps.value = steps
  activeStep.value = 0
  dbgLog('info', '执行计划已生成: '+steps.length+' 个步骤')
}

// --- SQL Diff ---
const showSqlDiff = ref(false)
const diffLeft = ref(""), diffRight = ref("")
const diffLines = ref<Array<{type:'added'|'removed'|'equal';line:number;text:string}>>([])
function computeDiff() {
  const l = diffLeft.value.split('\n'), r = diffRight.value.split('\n')
  const max = Math.max(l.length, r.length)
  diffLines.value = []
  for (let i = 0; i < max; i++) {
    const a = l[i]||'', b = r[i]||''
    if (a===b) diffLines.value.push({type:'equal',line:i+1,text:a})
    else { if(a) diffLines.value.push({type:'removed',line:i+1,text:a}); if(b) diffLines.value.push({type:'added',line:i+1,text:b}); }
  }
}
function applyDiffRight() { if(diffRight.value){ sql.value=diffRight.value; showSqlDiff.value=false; } }

// --- Export/Import ---
const showExportImport = ref(false)
const eiTab = ref<"export"|"import">("export")
const exportFmt = ref<"json"|"sql"|"csv">("json")
const importData = ref(""), importMsg = ref<{ok:boolean;txt:string}|null>(null)
function doExport() {
  const data = statements.value.map(s => ({name:s.name,flag:s.flag,sql:s.sql,description:s.desc,category:s.category}))
  if (exportFmt.value==='json') {
    const blob = new Blob([JSON.stringify(data,null,2)], {type:'application/json'})
    downloadBlob(blob, 'statements_'+new Date().toISOString().slice(0,10)+'.json')
  } else if (exportFmt.value==='sql') {
    const sqlStr = data.map(d => `-- ${d.name}\n${d.sql}`).join('\n\n')
    downloadBlob(new Blob([sqlStr],{type:'text/plain'}), 'statements_'+new Date().toISOString().slice(0,10)+'.sql')
  } else {
    const csv = 'name,flag,sql,category\n' + data.map(d => `"${d.name}","${d.flag||''}","${(d.sql||'').replace(/"/g,'""')}","${d.category||''}"`).join('\n')
    downloadBlob(new Blob([csv],{type:'text/csv'}), 'statements_'+new Date().toISOString().slice(0,10)+'.csv')
  }
  showExportImport.value = false
}
function downloadBlob(blob: Blob, filename: string) {
  const a = document.createElement('a'); a.href = URL.createObjectURL(blob)
  a.download = filename; a.click()
}
async function doImport() {
  if (!importData.value.trim()) return
  try {
    const data = JSON.parse(importData.value)
    if (!Array.isArray(data)) { importMsg.value={ok:false,txt:'数据格式错误: 期望数组'}; return }
    for (const stmt of data) {
      try { await api.post('/jaxrs/query/assemble/designer/create', stmt) } catch {}
    }
    importMsg.value={ok:true,txt:`成功导入 ${data.length} 条语句`}; showExportImport.value=false
    queryClient.invalidateQueries({queryKey:['stmt','list']})
  } catch(e: any) { importMsg.value={ok:false,txt:'导入失败: '+e.message} }
}

// --- Bulk Delete ---
const showBulkDelete = ref(false)
const bulkIds = ref<string[]>([])
const bulkSelectAll = computed(() => bulkIds.value.length === filtered.value.length && filtered.value.length > 0)
function toggleBulk(id: string) {
  const idx = bulkIds.value.indexOf(id)
  if (idx >= 0) bulkIds.value.splice(idx, 1); else bulkIds.value.push(id)
}
function selectAllBulk() { bulkIds.value = filtered.value.map(s => s.id) }
function clearBulk() { bulkIds.value = [] }
function confirmBulkDelete() {
  showBulkDelete.value = true
}
async function executeBulkDelete() {
  if (!bulkIds.value.length) return
  for (const id of bulkIds.value) { try { await api.delete(`/jaxrs/query/assemble/designer/delete/${id}`) } catch {} }
  bulkIds.value = []; showBulkDelete.value = false
  queryClient.invalidateQueries({queryKey:['stmt','list']})
}


// --- Bookmark ---
const showBookmark = ref(false)
const bookmarks = ref<Array<{id:string;name:string;sql:string;ts:number}>>([])
const bmName = ref("")
function addBookmark() {
  if (!bmName.value.trim() || !sql.value.trim()) return
  bookmarks.value.unshift({ id: "bm"+Date.now(), name: bmName.value, sql: sql.value, ts: Date.now() })
  bmName.value = ""
}
function loadBookmark(idx: number) { const b = bookmarks.value[idx]; if (b) { sql.value = b.sql; showBookmark.value = false } }
function deleteBookmark(idx: number) { bookmarks.value.splice(idx, 1) }

// --- Template CRUD ---
const showTemplateCRUD = ref(false)
const tplSearch = ref(""), tplCat = ref("all")
const tplCategories = computed(() => [...new Set(templates.value.map(t => t.category))])
const filteredTpls = computed(() => {
  let list = templates.value
  if (tplSearch.value) { const q = tplSearch.value.toLowerCase(); list = list.filter(t => t.name.toLowerCase().includes(q) || t.code.toLowerCase().includes(q)) }
  if (tplCat.value !== "all") list = list.filter(t => t.category === tplCat.value)
  return list
})
const showTplEditor = ref(false)
const tplEditingId = ref<string|null>(null)
const tplForm = ref({name:"",category:"select",icon:"📋",code:""})
function openTplEditor(t: any) {
  if (t) { tplEditingId.value = t.id; tplForm.value = {name:t.name,category:t.category,icon:t.icon,code:t.code} }
  else { tplEditingId.value = null; tplForm.value = {name:"",category:"select",icon:"📋",code:""} }
  showTplEditor.value = true
}
function editTemplate(t: any) { openTplEditor(t) }
function saveTpl() {
  if (!tplForm.value.name.trim()) return
  if (tplEditingId.value) {
    const t = templates.value.find(x => x.id === tplEditingId.value)
    if (t) Object.assign(t, tplForm.value)
  } else {
    templates.value.push({ id: "t"+Date.now(), ...tplForm.value })
  }
  showTplEditor.value = false
}
function deleteTpl(idx: number) {
  if (!confirmMsg("确定删除此模板？")) return
  templates.value.splice(idx, 1)
}


// --- Parameter Presets ---
const showParamPresets = ref(false)
const paramPresets = ref<Array<{id:string;name:string;value:string;type:string;defaultValue:string}>>([])
const detectedParams = computed(() => {
  const matches = sql.value.match(/[:@#](\w+)/g) || []
  return [...new Set(matches.map(m => m.substring(1)))]
})
function addParamPreset(name: string) {
  if (!paramPresets.value.some(p => p.name === name))
    paramPresets.value.push({ id: "p"+Date.now(), name, value: "", type: "string", defaultValue: "" })
}
function addAllParams() { detectedParams.value.forEach(addParamPreset) }
function applyParamPresets() {
  let s = sql.value
  paramPresets.value.forEach(p => {
    if (p.name && p.value) s = s.replace(new RegExp(':'+p.name+'|@'+p.name+'|#'+p.name, 'g'), p.value)
  })
  sql.value = s; showParamPresets.value = false
}

// --- SQL Auto-Hint ---
const showSqlHints = ref(false)
const selectedTableForHints = ref("")
const sqlKeywords = ["SELECT","FROM","WHERE","AND","OR","ORDER BY","GROUP BY","HAVING","LIMIT","OFFSET","JOIN","LEFT JOIN","RIGHT JOIN","INNER JOIN","CROSS JOIN","ON","SET","VALUES","INSERT INTO","DELETE FROM","CREATE TABLE","ALTER TABLE","DROP TABLE","UNION ALL","UNION","NOT NULL","IS NULL","IS NOT NULL","IN","EXISTS","BETWEEN","LIKE","CASE","WHEN","THEN","ELSE","END","DISTINCT","AS","WITH","RECURSIVE"]
const sqlFunctions = ["COUNT","SUM","AVG","MAX","MIN","ROW_NUMBER","RANK","DENSE_RANK","LAG","LEAD","FIRST_VALUE","LAST_VALUE","COALESCE","NULLIF","CAST","CONVERT","SUBSTRING","LENGTH","TRIM","UPPER","LOWER","REPLACE","NOW","CURRENT_DATE","DATE_TRUNC","DATE_PART","ABS","ROUND","FLOOR","CEIL","MOD","POWER","SQRT"]
function insertHint(text: string) {
  sql.value += text + " "
  showSqlHints.value = false
}

// --- Additional helper: copy SQL to clipboard with timestamp ---
function copySqlWithTimestamp() {
  const ts = new Date().toLocaleString('zh-CN')
  navigator.clipboard.writeText(`-- ${ts}\n${sql.value}`)
  dbgLog('success', 'SQL已复制（含时间戳）')
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
/* -- Visual Editor Panel -- */
.visual-editor-panel{width:620px}.ve-body{padding:12px;display:flex;flex-direction:column;gap:10px}.ve-section{padding:10px;background:rgba(255,255,255,0.02);border:1px solid var(--border-color);border-radius:var(--radius-sm)}.ve-section-title{font-size:11px;color:var(--color-primary);font-weight:600;margin-bottom:6px}.ve-fields{display:flex;flex-wrap:wrap;gap:4px;align-items:center}.ve-field-tag{padding:2px 8px;border-radius:10px;background:rgba(59,130,246,0.2);border:1px solid rgba(59,130,246,0.4);color:#3b82f6;font-size:11px;cursor:pointer}.ve-add-btn{padding:2px 8px;border-radius:var(--radius-sm);border:1px dashed var(--border-color);background:transparent;color:var(--color-primary);font-size:11px;cursor:pointer}.ve-select{width:100%;padding:6px;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-sm);color:var(--text-primary);font-size:12px}.ve-select-sm{padding:4px 8px;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-sm);color:var(--text-primary);font-size:11px}.ve-input-sm{flex:1;padding:4px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}.ve-condition-row{display:flex;align-items:center;gap:6px;margin-bottom:4px}.ve-del-btn{padding:2px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:10px}.ve-preview{padding:10px;background:rgba(0,0,0,0.3);border-radius:var(--radius-sm)}.ve-preview-label{font-size:10px;color:var(--text-muted);margin-bottom:4px}.ve-preview-sql{margin:0;font-size:11px;color:#10b981;font-family:monospace;white-space:pre-wrap;max-height:120px;overflow-y:auto}.ve-actions{display:flex;gap:6px}
/* -- Rule Chain Panel -- */
.rule-chain-panel{width:560px}.rc-body{padding:12px}.rc-chain{display:flex;flex-direction:column;gap:4px;margin-bottom:10px}.rc-rule{background:rgba(255,255,255,0.02);border:1px solid var(--border-color);border-radius:var(--radius-sm)}.rc-rule-header{display:flex;align-items:center;gap:6px;padding:6px 10px;background:rgba(59,130,246,0.05)}.rc-rule-num{color:var(--text-muted);font-size:10px;width:20px}.rc-rule-type{padding:2px 8px;border-radius:10px;background:rgba(59,130,246,0.2);color:#3b82f6;font-size:10px;font-weight:600}.rc-status.active{color:#10b981}.rc-status.inactive{color:#ef4444}.rc-rule-body{padding:8px 10px;display:flex;flex-direction:column;gap:4px}.rc-row{display:flex;align-items:center;gap:8px;font-size:11px}.rc-row label{color:var(--text-muted);min-width:30px}.rc-select{flex:1;padding:4px 8px;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-sm);color:var(--text-primary);font-size:11px}.rc-input{flex:1;padding:4px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}
/* -- Field Drag Panel -- */
.field-drag-panel{width:680px}.fd-body{padding:12px}.fd-columns{display:grid;grid-template-columns:1fr 1fr 1fr;gap:10px;margin-bottom:10px}.fd-col{display:flex;flex-direction:column;gap:4px}.fd-title{font-size:10px;font-weight:600;color:var(--color-primary);margin-bottom:4px}.fd-item{padding:4px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:rgba(255,255,255,0.02);font-size:11px;color:var(--text-primary);cursor:grab}.fd-item:hover{border-color:var(--color-primary)}.fd-target{min-height:40px;padding:4px;border:1px dashed var(--border-color);border-radius:var(--radius-sm)}.fd-target-item{display:flex;align-items:center;gap:4px;padding:3px 6px;background:rgba(59,130,246,0.1);border:1px solid rgba(59,130,246,0.3);border-radius:3px;font-size:10px;color:var(--text-primary)}.fd-x{color:#ef4444;cursor:pointer;margin-left:auto;font-size:9px}.fd-hint{color:var(--text-muted);font-size:10px;text-align:center;padding:8px}.fd-preview{padding:8px;background:rgba(0,0,0,0.2);border-radius:var(--radius-sm);margin-bottom:8px}.dp-label{font-size:10px;color:var(--text-muted);margin-bottom:4px}.dp-sql{margin:0;font-size:11px;color:#10b981;font-family:monospace;white-space:pre-wrap}.fd-actions{display:flex;gap:6px}
/* -- Chart Linkage Panel -- */
.chart-linkage-panel{width:560px}.cl-body{padding:12px}.cl-cards{display:grid;grid-template-columns:1fr 1fr;gap:8px;margin-bottom:10px}.cl-card{padding:10px;background:rgba(255,255,255,0.02);border:1px solid var(--border-color);border-radius:var(--radius-sm)}.cl-card-title{font-size:10px;color:var(--text-muted);margin-bottom:4px;font-weight:600}.cl-select{width:100%;padding:4px;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-sm);color:var(--text-primary);font-size:11px}.cl-input{width:100%;padding:4px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}.cl-preview{padding:8px;background:rgba(0,0,0,0.2);border-radius:var(--radius-sm);margin-bottom:8px}.cl-preview-title{font-size:10px;color:var(--text-muted);margin-bottom:4px}.cl-preview-item{font-size:10px;color:#7fdbca;font-family:monospace;padding:2px 0}.cl-empty{color:var(--text-muted);font-size:11px;text-align:center;padding:12px}.cl-actions{display:flex;gap:6px}
/* -- Advanced Templates Panel -- */
.adv-tmpl-panel{width:640px}.adv-grid{display:flex;flex-direction:column;gap:8px;padding:12px;max-height:400px;overflow-y:auto}.adv-card{background:rgba(255,255,255,0.02);border:1px solid var(--border-color);border-radius:var(--radius-md);overflow:hidden}.adv-header{display:flex;align-items:center;gap:6px;padding:6px 10px;background:rgba(59,130,246,0.1);border-bottom:1px solid var(--border-color)}.adv-icon{font-size:14px}.adv-name{flex:1;color:var(--text-primary);font-size:12px;font-weight:500}.adv-diff{font-size:10px;padding:1px 6px;border-radius:10px;background:rgba(245,158,11,0.2);color:#f59e0b}.adv-code{margin:0;padding:8px 10px;background:rgba(0,0,0,0.3);color:#10b981;font-size:10px;font-family:monospace;white-space:pre-wrap;word-break:break-all;max-height:60px;overflow-y:auto}.adv-desc{padding:4px 10px;font-size:10px;color:var(--text-muted);border-top:1px solid var(--border-color)}.adv-actions{display:flex;gap:4px;padding:6px 10px;border-top:1px solid var(--border-color)}

/* Debug Console */
.debug-panel{width:560px}.dbg-body{padding:12px;max-height:420px;overflow-y:auto;display:flex;flex-direction:column;gap:8px}.debug-tabs{display:flex;gap:4px;border-bottom:1px solid var(--border-color);padding-bottom:8px}.dbg-tab{padding:4px 12px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:11px}.dbg-tab.active{background:var(--color-primary);color:#000;border-color:var(--color-primary)}.dbg-logs{flex:1;overflow-y:auto;display:flex;flex-direction:column;gap:3px}.dbg-log{display:flex;align-items:center;gap:8px;padding:4px 8px;border-radius:var(--radius-sm);font-size:11px;font-family:monospace}.dbg-log.info{background:rgba(59,130,246,0.08)}.dbg-log.warn{background:rgba(245,158,11,0.08)}.dbg-log.error{background:rgba(239,68,68,0.08)}.dbg-time{color:var(--text-muted);width:60px;flex-shrink:0}.dbg-msg{flex:1;color:var(--text-primary);word-break:break-all}.dbg-empty{color:var(--text-muted);font-size:12px;text-align:center;padding:24px}.dbg-vars{flex:1;overflow-y:auto;display:flex;flex-direction:column;gap:4px}.dbg-var-row{display:flex;align-items:center;gap:8px;padding:4px 8px;background:rgba(255,255,255,0.02);border-radius:4px;font-size:11px}.dbg-var-name{color:var(--color-primary);width:100px;font-family:monospace;flex-shrink:0}.dbg-var-val{color:var(--text-primary);font-family:monospace;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}.dbg-perf{display:flex;flex-direction:column;gap:6px}.perf-row{display:flex;justify-content:space-between;padding:6px 10px;background:rgba(59,130,246,0.08);border-radius:var(--radius-sm);font-size:12px}.perf-row span:first-child{color:var(--text-muted)}.perf-row span:last-child{color:var(--color-primary);font-weight:600;font-family:monospace}.dbg-footer{display:flex;gap:6px;padding-top:8px;border-top:1px solid var(--border-color)}
/* SQL Formatter */
.fmt-panel{width:720px}.fmt-body{padding:12px}.fmt-cols{display:grid;grid-template-columns:1fr 40px 1fr;gap:0;margin-bottom:12px}.fmt-col{display:flex;flex-direction:column;gap:4px}.fmt-label{font-size:11px;color:var(--text-muted);font-weight:600}.fmt-src,.fmt-out{padding:10px;background:rgba(0,0,0,0.3);color:#7fdbca;font-size:12px;font-family:monospace;border-radius:var(--radius-sm);border:1px solid var(--border-color);white-space:pre-wrap;word-break:break-all;max-height:220px;overflow-y:auto;min-height:80px}.fmt-out{color:#10b981}.fmt-arrow{text-align:center;color:var(--text-muted);align-self:center;font-size:18px}.fmt-opts{display:flex;gap:16px;font-size:12px;color:var(--text-muted);padding:8px;background:rgba(255,255,255,0.02);border-radius:var(--radius-sm)}.fmt-opts label{display:flex;align-items:center;gap:4px;cursor:pointer}.fmt-footer{display:flex;gap:6px;padding-top:8px;border-top:1px solid var(--border-color)}
/* SQL Validator */
.val-panel{width:520px}.val-body{padding:12px;display:flex;flex-direction:column;gap:10px}.val-result{padding:12px;border-radius:var(--radius-md);display:flex;align-items:center;gap:10px;font-size:13px}.val-result.valid{background:rgba(16,185,129,0.1);border:1px solid rgba(16,185,129,0.3);color:#10b981}.val-result.error{background:rgba(239,68,68,0.1);border:1px solid rgba(239,68,68,0.3);color:#ef4444}.val-icon{font-size:20px;font-weight:700}.val-checks{display:flex;flex-direction:column;gap:4px;max-height:180px;overflow-y:auto}.val-check{display:flex;align-items:center;gap:8px;padding:4px 8px;border-radius:var(--radius-sm);font-size:11px;background:rgba(255,255,255,0.02)}.val-check.pass{border-left:3px solid #10b981}.val-check.fail{border-left:3px solid #ef4444}.val-name{flex:1;color:var(--text-primary)}.val-detail{color:var(--text-muted);font-family:monospace;font-size:10px}.val-sug{padding:10px;background:rgba(245,158,11,0.05);border:1px solid rgba(245,158,11,0.2);border-radius:var(--radius-sm)}.val-sug-title{font-size:11px;color:#f59e0b;font-weight:600;margin-bottom:4px}.val-sug-item{font-size:11px;color:var(--text-primary);padding:2px 0}.val-footer{display:flex;gap:6px;padding-top:8px;border-top:1px solid var(--border-color)}
/* Result Visualization */
.viz-panel{width:640px}.viz-body{padding:12px;max-height:480px;overflow-y:auto;display:flex;flex-direction:column;gap:10px}.viz-controls{display:flex;gap:8px;flex-wrap:wrap;align-items:center}.viz-select{padding:5px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}.viz-chart{padding:16px;background:rgba(255,255,255,0.02);border:1px solid var(--border-color);border-radius:var(--radius-md);min-height:160px;display:flex;align-items:flex-end;gap:4px;flex-wrap:wrap;justify-content:center}.viz-bar-wrap{display:flex;flex-direction:column;align-items:center;gap:2px;flex:1;max-width:50px}.viz-bar{width:100%;border-radius:3px 3px 0 0;transition:opacity 0.15s;cursor:pointer;min-height:4px}.viz-bar:hover{opacity:0.8}.viz-bar-label{font-size:8px;color:var(--text-muted);text-align:center;max-width:50px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}.viz-bar-val{font-size:9px;color:var(--text-primary);font-family:monospace}.viz-empty{color:var(--text-muted);font-size:12px;text-align:center;width:100%;padding:24px}.viz-stats{display:grid;grid-template-columns:repeat(4,1fr);gap:8px}.viz-stat{padding:8px;border-radius:var(--radius-sm);background:rgba(59,130,246,0.08);border:1px solid rgba(59,130,246,0.15);text-align:center;font-size:11px}.viz-stat span:first-child{color:var(--text-muted);display:block}.viz-stat span:last-child{color:var(--color-primary);font-weight:700;font-family:monospace;font-size:13px}.viz-footer{display:flex;gap:6px;padding-top:8px;border-top:1px solid var(--border-color);margin-top:8px}
/* Snippet Library */
.snippet-panel{width:620px}.snippet-toolbar{display:flex;gap:8px;padding:8px 12px;border-bottom:1px solid var(--border-color)}.snippet-grid{padding:12px;max-height:380px;overflow-y:auto;display:flex;flex-direction:column;gap:8px}.snippet-card{background:rgba(255,255,255,0.02);border:1px solid var(--border-color);border-radius:var(--radius-md);overflow:hidden}.snippet-head{display:flex;align-items:center;gap:6px;padding:6px 10px;background:rgba(139,92,246,0.08);border-bottom:1px solid var(--border-color)}.snippet-name{flex:1;color:var(--text-primary);font-size:12px;font-weight:500}.snippet-cat{font-size:10px;padding:1px 6px;border-radius:10px;background:rgba(139,92,246,0.2);color:#8b5cf6}.snippet-code{margin:0;padding:8px 10px;background:rgba(0,0,0,0.3);color:#7fdbca;font-size:11px;font-family:monospace;white-space:pre-wrap;word-break:break-all;max-height:70px;overflow-y:auto}.snippet-foot{display:flex;gap:6px;padding:6px 10px;border-top:1px solid var(--border-color)}
/* Toolbar enhancements */
.smd-actions{display:flex;gap:8px;flex-wrap:wrap}


/* Execution Plan */
.plan-panel{width:520px}.plan-body{padding:12px;max-height:420px;overflow-y:auto;display:flex;flex-direction:column;gap:8px}.plan-steps{display:flex;flex-direction:column;gap:0}.plan-step{display:flex;align-items:flex-start;gap:10px;padding:8px;border-radius:var(--radius-sm);background:rgba(255,255,255,0.02);position:relative}.plan-step.active{background:rgba(59,130,246,0.1);border:1px solid rgba(59,130,246,0.3)}.plan-num{width:20px;height:20px;border-radius:50%;background:var(--color-primary);color:#000;font-size:10px;font-weight:700;display:flex;align-items:center;justify-content:center;flex-shrink:0}.plan-content{flex:1}.plan-type{font-size:12px;font-weight:600;color:var(--color-primary)}.plan-desc{font-size:11px;color:var(--text-primary);margin-top:2px}.plan-detail{font-size:10px;color:var(--text-muted);margin-top:2px;font-family:monospace}.plan-arrow{color:var(--text-muted);text-align:center;font-size:12px;padding:2px 0}.plan-empty{color:var(--text-muted);font-size:12px;text-align:center;padding:32px}.plan-footer{display:flex;justify-content:flex-end;padding-top:8px;border-top:1px solid var(--border-color);margin-top:8px}
/* SQL Diff */
.diff-panel{width:720px}.diff-body{padding:12px;display:flex;flex-direction:column;gap:8px}.diff-cols{display:grid;grid-template-columns:1fr 1fr;gap:8px}.diff-col{display:flex;flex-direction:column;gap:4px}.diff-title{font-size:11px;font-weight:600;color:var(--color-primary)}.diff-textarea{width:100%;height:140px;background:rgba(0,0,0,0.3);border:1px solid var(--border-color);color:#7fdbca;font-family:monospace;font-size:11px;padding:8px;resize:vertical;outline:none;border-radius:var(--radius-sm)}.diff-result{max-height:200px;overflow-y:auto;display:flex;flex-direction:column;gap:1px}.diff-line{display:flex;gap:8px;padding:2px 8px;font-size:11px;font-family:monospace;border-radius:3px}.diff-line.added{background:rgba(16,185,129,0.1);color:#10b981}.diff-line.removed{background:rgba(239,68,68,0.1);color:#ef4444}.diff-line.equal{color:var(--text-muted)}.diff-num{width:30px;color:var(--text-muted);flex-shrink:0}.diff-text{flex:1;word-break:break-all}.diff-footer{display:flex;gap:6px;padding-top:8px;border-top:1px solid var(--border-color);margin-top:8px}
/* Export/Import */
.expimp-panel{width:480px}.ei-tabs{display:flex;gap:4px;padding:8px 12px;border-bottom:1px solid var(--border-color)}.ei-tab{padding:4px 12px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:11px}.ei-tab.active{background:var(--color-primary);color:#000;border-color:var(--color-primary)}.ei-body{padding:12px;display:flex;flex-direction:column;gap:10px}.ei-option{display:flex;align-items:center;gap:8px;font-size:12px}.ei-select{padding:5px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}.ei-count{font-size:11px;color:var(--text-muted)}.ei-textarea{width:100%;height:120px;background:rgba(0,0,0,0.3);border:1px solid var(--border-color);color:#7fdbca;font-family:monospace;font-size:11px;padding:10px;resize:vertical;outline:none;border-radius:var(--radius-sm)}.ei-msg{padding:8px;border-radius:var(--radius-sm);font-size:12px}.ei-msg.ok{background:rgba(16,185,129,0.1);color:#10b981}.ei-msg.err{background:rgba(239,68,68,0.1);color:#ef4444}.ei-footer{display:flex;justify-content:flex-end;padding-top:8px;border-top:1px solid var(--border-color);margin-top:8px}
/* Bulk Delete */
.bulk-panel{width:420px}.bulk-body{padding:12px;display:flex;flex-direction:column;gap:8px}.bulk-body p{font-size:13px;color:var(--text-primary)}.bulk-body strong{color:var(--color-danger)}.bulk-list{max-height:200px;overflow-y:auto;display:flex;flex-direction:column;gap:4px}.bulk-item{padding:4px 8px;background:rgba(239,68,68,0.05);border-radius:var(--radius-sm);font-size:11px;color:var(--text-primary);font-family:monospace}.bulk-footer{display:flex;gap:6px;padding-top:8px;border-top:1px solid var(--border-color);margin-top:8px}
/* Sidebar bulk bar */
.bulk-bar{display:flex;align-items:center;gap:8px;padding:6px 8px;background:rgba(239,68,68,0.08);border-bottom:1px solid var(--border-color);font-size:12px;color:var(--color-danger)}
.bulk-btn{padding:2px 8px;border-radius:var(--radius-sm);border:1px solid currentColor;background:transparent;cursor:pointer;font-size:11px;color:inherit}.bulk-btn:hover{background:rgba(239,68,68,0.1)}
/* Result stats enhancement */
.result-stats-bar{display:flex;align-items:center;gap:12px;padding:6px 12px;background:rgba(59,130,246,0.05);border:1px solid rgba(59,130,246,0.15);border-radius:var(--radius-sm);font-size:11px;color:var(--text-muted)}
.rs-item{display:flex;align-items:center;gap:4px}
.rs-val{color:var(--color-primary);font-weight:600;font-family:monospace}


/* Bookmark */
.bookmark-panel{width:420px}.bm-body{padding:12px;display:flex;flex-direction:column;gap:10px}.bm-add{display:flex;gap:8px}.bm-input{flex:1;padding:6px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:12px;outline:none}.bm-list{display:flex;flex-direction:column;gap:4px;max-height:280px;overflow-y:auto}.bm-item{display:flex;align-items:center;gap:8px;padding:6px 10px;background:rgba(255,255,255,0.02);border-radius:var(--radius-sm);font-size:12px}.bm-icon{font-size:14px}.bm-name{flex:1;color:var(--text-primary)}.bm-time{color:var(--text-muted);font-size:10px;font-family:monospace}.bm-btn{padding:2px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:10px}.bm-btn:hover{border-color:var(--color-primary);color:var(--color-primary)}.bm-del:hover{border-color:var(--color-danger);color:var(--color-danger)}.bm-empty{color:var(--text-muted);font-size:12px;text-align:center;padding:24px}.bm-footer{display:flex;justify-content:flex-end;padding-top:8px;border-top:1px solid var(--border-color);margin-top:8px}
/* Template CRUD */
.tplcrud-panel{width:640px}.tplcrud-toolbar{display:flex;gap:8px;padding:8px 12px;border-bottom:1px solid var(--border-color)}.tpl-list{padding:12px;max-height:380px;overflow-y:auto;display:flex;flex-direction:column;gap:8px}.tpl-card{background:rgba(255,255,255,0.02);border:1px solid var(--border-color);border-radius:var(--radius-md);overflow:hidden}.tpl-head{display:flex;align-items:center;gap:6px;padding:6px 10px;background:rgba(59,130,246,0.08);border-bottom:1px solid var(--border-color)}.tpl-icon{font-size:14px}.tpl-name{flex:1;color:var(--text-primary);font-size:12px;font-weight:500}.tpl-cat{font-size:10px;padding:1px 6px;border-radius:10px;background:rgba(59,130,246,0.15);color:var(--color-primary)}.tpl-code{margin:0;padding:8px 10px;background:rgba(0,0,0,0.3);color:#7fdbca;font-size:11px;font-family:monospace;white-space:pre-wrap;word-break:break-all;max-height:60px;overflow-y:auto}.tpl-foot{display:flex;gap:6px;padding:6px 10px;border-top:1px solid var(--border-color)}
/* Result stats bar */
.result-stats-bar{display:flex;align-items:center;gap:12px;padding:6px 12px;background:rgba(59,130,246,0.05);border:1px solid rgba(59,130,246,0.15);border-radius:var(--radius-sm);font-size:11px;color:var(--text-muted);margin-top:4px}
.rs-item{display:flex;align-items:center;gap:4px}.rs-val{color:var(--color-primary);font-weight:600;font-family:monospace}
/* Bulk selection in sidebar */
.sb-item{cursor:pointer}.sb-item input[type="checkbox"]{cursor:pointer}


/* Parameter Presets */
.param-panel{width:560px}.param-body{padding:12px;display:flex;flex-direction:column;gap:10px}.param-list{display:flex;flex-direction:column;gap:4px;max-height:180px;overflow-y:auto}.param-row{display:flex;align-items:center;gap:6px;padding:4px 8px;background:rgba(255,255,255,0.02);border-radius:var(--radius-sm);font-size:11px}.param-name{color:#f59e0b;width:80px;font-family:monospace;font-weight:600;flex-shrink:0}.param-input{flex:1;padding:3px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}.param-type{padding:3px;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-sm);color:var(--text-primary);font-size:10px}.param-del{padding:2px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--color-danger);cursor:pointer;font-size:10px}.param-detect{padding:10px;background:rgba(245,158,11,0.05);border:1px solid rgba(245,158,11,0.2);border-radius:var(--radius-sm)}.pd-title{font-size:11px;color:#f59e0b;margin-bottom:6px;font-weight:600}.pd-tags{display:flex;flex-wrap:wrap;gap:4px;margin-bottom:6px}.pd-tag{padding:2px 8px;border-radius:10px;background:rgba(245,158,11,0.15);border:1px solid rgba(245,158,11,0.3);color:#f59e0b;font-size:10px;font-family:monospace;cursor:pointer}.pd-tag.exists{background:rgba(16,185,129,0.15);border-color:rgba(16,185,129,0.3);color:#10b981}.param-footer{display:flex;gap:6px;padding-top:8px;border-top:1px solid var(--border-color)}
/* SQL Hints */
.hint-panel{width:520px}.hint-body{padding:12px;max-height:400px;overflow-y:auto;display:flex;flex-direction:column;gap:12px}.hint-section{display:flex;flex-direction:column;gap:6px}.hint-title{font-size:11px;font-weight:600;color:var(--color-primary);text-transform:uppercase}.hint-tags{display:flex;flex-wrap:wrap;gap:4px}.hint-tag{padding:3px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:rgba(255,255,255,0.02);color:var(--text-muted);cursor:pointer;font-size:11px;font-family:monospace;transition:all 0.15s}.hint-tag:hover{border-color:var(--color-primary);color:var(--color-primary);background:rgba(59,130,246,0.1)}


/* Extra layout polish */
.smd-header{display:flex;align-items:center;justify-content:space-between;padding:12px 20px;flex-shrink:0;flex-wrap:wrap;gap:8px}
.smd-actions{display:flex;gap:8px;flex-wrap:wrap}
.sb-tabs{display:flex;gap:4px;padding:4px 8px;border-bottom:1px solid var(--border-color)}
.sb-item{display:flex;align-items:center;gap:8px;padding:8px;border-radius:var(--radius-sm);cursor:pointer;margin-bottom:2px}
.sb-item:hover{background:var(--bg-hover)}
.sb-item.active{background:var(--color-primary-soft);border-left:3px solid var(--color-primary)}
/* SQL Editor enhancement */
.sql-editor{flex:1;min-height:200px;padding:12px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-terminal);color:#7fdbca;font-family:'Fira Code','JetBrains Mono',monospace;font-size:13px;outline:none;resize:none;line-height:1.6;tab-size:2}
.sql-editor:focus{border-color:var(--color-primary)}
/* Result export bar */
.result-export-bar{display:flex;align-items:center;gap:8px;padding:6px 12px;background:rgba(59,130,246,0.05);border:1px solid rgba(59,130,246,0.15);border-radius:var(--radius-sm);font-size:11px;color:var(--text-muted);margin-top:4px}
.rap-btn{padding:2px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:10px}
.rap-btn:hover{border-color:var(--color-primary);color:var(--color-primary)}
/* Query param input bar */
.query-param-bar{display:flex;align-items:center;gap:8px;padding:6px 12px;background:rgba(245,158,11,0.05);border:1px solid rgba(245,158,11,0.15);border-radius:var(--radius-sm);font-size:11px;flex-wrap:wrap}
.param-chip{display:inline-flex;align-items:center;gap:4px;padding:2px 8px;background:rgba(245,158,11,0.15);border:1px solid rgba(245,158,11,0.3);border-radius:10px;font-size:10px;color:#f59e0b;font-family:monospace}
.query-param-input{padding:3px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none;width:100px}
/* Editor mode tabs */
.editor-mode-tabs{display:flex;gap:4px;margin-bottom:8px}
.emt{padding:4px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:12px}
.emt.active{background:var(--color-primary);color:#000;border-color:var(--color-primary)}
/* Gen SQL bar */
.gen-sql-bar{display:flex;align-items:center;gap:8px;padding:8px 12px;background:rgba(16,185,129,0.05);border:1px solid rgba(16,185,129,0.2);border-radius:var(--radius-sm);margin-top:8px}
.gen-sql-label{font-size:11px;color:#10b981;font-weight:600}
.gen-sql-btn{padding:3px 8px;border-radius:var(--radius-sm);border:1px solid var(--color-success);background:rgba(16,185,129,0.1);color:#10b981;cursor:pointer;font-size:11px}
.gen-sql-btn:hover{background:rgba(16,185,129,0.2)}
/* Template panel search */
.tmp-input{flex:1;padding:5px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}
.tmp-select{padding:5px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}
/* SQL generator section */
.sql-gen-section{padding:12px;background:rgba(0,0,0,0.2);border-radius:var(--radius-md);margin-top:8px}
.sql-gen-title{font-size:12px;color:var(--color-primary);font-weight:600;margin-bottom:8px;display:flex;align-items:center;gap:6px}
/* Enhanced result stats */
.result-stats-bar{display:flex;align-items:center;gap:12px;padding:6px 12px;background:rgba(59,130,246,0.05);border:1px solid rgba(59,130,246,0.15);border-radius:var(--radius-sm);font-size:11px;color:var(--text-muted)}
.rs-item{display:flex;align-items:center;gap:4px}
.rs-val{color:var(--color-primary);font-weight:600;font-family:monospace}
/* Bulk bar in sidebar */
.bulk-bar{display:flex;align-items:center;gap:8px;padding:6px 8px;background:rgba(239,68,68,0.08);border-bottom:1px solid var(--border-color);font-size:12px;color:var(--color-danger)}
.bulk-btn{padding:2px 8px;border-radius:var(--radius-sm);border:1px solid currentColor;background:transparent;cursor:pointer;font-size:11px;color:inherit}.bulk-btn:hover{background:rgba(239,68,68,0.1)}
/* Modal glass effect enhancement */
.modal{backdrop-filter:blur(10px)}
/* Snippet card hover */
.snippet-card:hover{border-color:var(--color-primary)}
/* Debug log scrollbar */
.dbg-logs::-webkit-scrollbar,.dbg-vars::-webkit-scrollbar,.snippet-grid::-webkit-scrollbar,.tpl-list::-webkit-scrollbar{width:6px}
.dbg-logs::-webkit-scrollbar-thumb,.dbg-vars::-webkit-scrollbar-thumb,.snippet-grid::-webkit-scrollbar-thumb,.tpl-list::-webkit-scrollbar-thumb{background:var(--border-color);border-radius:3px}
/* Performance stat highlights */
.perf-stat{display:flex;justify-content:space-between;padding:6px 10px;background:rgba(59,130,246,0.08);border-radius:var(--radius-sm);font-size:12px}
.perf-stat span:first-child{color:var(--text-muted)}
.perf-stat span:last-child{color:var(--color-primary);font-weight:600;font-family:monospace}
/* Viz chart tooltip style */
.viz-bar-wrap:hover .viz-bar{opacity:0.7}
/* Execution plan step highlight */
.plan-step.active{box-shadow:0 0 0 2px rgba(59,130,246,0.3)}
/* SQL diff result styling */
.diff-line.added{background:rgba(16,185,129,0.08)}
.diff-line.removed{background:rgba(239,68,68,0.08)}
/* Parameter row focus */
.param-row:focus-within{border:1px solid var(--color-primary);background:rgba(59,130,246,0.05)}
/* Bookmark item hover */
.bm-item:hover{background:rgba(59,130,246,0.05)}
/* Template card hover */
.tpl-card:hover{border-color:var(--color-primary)}
/* Hint tag hover animation */
.hint-tag:hover{transform:translateY(-1px)}


/* Result stats bar */
.result-stats-bar{display:flex;align-items:center;gap:12px;padding:6px 12px;background:rgba(59,130,246,0.05);border:1px solid rgba(59,130,246,0.15);border-radius:var(--radius-sm);font-size:11px;color:var(--text-muted);margin-top:4px}
.rs-item{display:flex;align-items:center;gap:4px}.rs-val{color:var(--color-primary);font-weight:600;font-family:monospace}
.rap-btn{padding:2px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:10px}
.rap-btn:hover{border-color:var(--color-primary);color:var(--color-primary)}
/* Enhanced editor toolbar */
.editor-toolbar{display:flex;align-items:center;gap:8px;margin-bottom:8px;flex-wrap:wrap}
/* Query param bar */
.query-param-bar{display:flex;align-items:center;gap:8px;padding:6px 12px;background:rgba(245,158,11,0.05);border:1px solid rgba(245,158,11,0.15);border-radius:var(--radius-sm);font-size:11px;flex-wrap:wrap;margin-top:4px}
.param-chip{display:inline-flex;align-items:center;gap:4px;padding:2px 8px;background:rgba(245,158,11,0.15);border:1px solid rgba(245,158,11,0.3);border-radius:10px;font-size:10px;color:#f59e0b;font-family:monospace}
.query-param-input{padding:3px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none;width:100px}
/* Gen SQL bar */
.gen-sql-bar{display:flex;align-items:center;gap:8px;padding:8px 12px;background:rgba(16,185,129,0.05);border:1px solid rgba(16,185,129,0.2);border-radius:var(--radius-sm);margin-top:8px}
.gen-sql-label{font-size:11px;color:#10b981;font-weight:600}
.gen-sql-btn{padding:3px 8px;border-radius:var(--radius-sm);border:1px solid var(--color-success);background:rgba(16,185,129,0.1);color:#10b981;cursor:pointer;font-size:11px}
.gen-sql-btn:hover{background:rgba(16,185,129,0.2)}
/* Editor mode tabs */
.editor-mode-tabs{display:flex;gap:4px;margin-bottom:8px}
.emt{padding:4px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:12px}
.emt.active{background:var(--color-primary);color:#000;border-color:var(--color-primary)}
/* Snippet card hover */
.snippet-card:hover{border-color:var(--color-primary)}
/* Visual editor fields */
.ve-panel{width:680px}.ve-body{padding:12px;max-height:440px;overflow-y:auto;display:flex;flex-direction:column;gap:10px}
.ve-fields-row{display:flex;gap:8px}.ve-field-group{display:flex;flex-direction:column;gap:4px;flex:1}
.ve-field-group label{font-size:11px;color:var(--text-muted);font-weight:600}
.ve-tags{display:flex;flex-wrap:wrap;gap:4px;padding:6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);min-height:32px}
.ve-tag{display:inline-flex;align-items:center;gap:4px;padding:2px 8px;background:rgba(59,130,246,0.15);border:1px solid rgba(59,130,246,0.3);border-radius:10px;font-size:11px;color:var(--color-primary);font-family:monospace}
.ve-tag-del{cursor:pointer;opacity:0.7;font-size:10px}.ve-tag-input{border:none;background:transparent;color:var(--text-primary);font-size:11px;outline:none;flex:1;min-width:80px}
.ve-row{display:flex;gap:8px}.ve-select{flex:1;padding:6px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:12px;outline:none}
.ve-select-sm{padding:5px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}
.ve-input{flex:1;padding:6px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:12px;outline:none}
.ve-input-sm{width:80px;padding:5px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}
.ve-conditions{display:flex;flex-direction:column;gap:4px;max-height:120px;overflow-y:auto}
.ve-cond-row{display:flex;align-items:center;gap:4px}
.ve-sel-sm{padding:3px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none;min-width:80px}
.ve-del-btn{padding:2px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--color-danger);cursor:pointer;font-size:10px}
.ve-add-btn{padding:3px 8px;border-radius:var(--radius-sm);border:1px dashed var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:11px;margin-top:4px}
.ve-add-btn:hover{border-color:var(--color-primary);color:var(--color-primary)}
.ve-preview{background:rgba(0,0,0,0.3);border-radius:var(--radius-sm);padding:10px}
.ve-preview-label{font-size:11px;color:var(--text-muted);margin-bottom:4px}
.ve-sql-out{margin:0;padding:8px;background:rgba(0,0,0,0.4);color:#10b981;font-size:12px;font-family:monospace;border-radius:4px;white-space:pre-wrap;word-break:break-all;max-height:100px;overflow-y:auto}
.ve-footer{display:flex;gap:6px;padding-top:8px;border-top:1px solid var(--border-color)}
/* Rule chain editor */
.rc-panel{width:600px}.rc-body{padding:12px;max-height:440px;overflow-y:auto;display:flex;flex-direction:column;gap:8px}
.rc-rule-list{display:flex;flex-direction:column;gap:6px}.rc-rule{background:rgba(255,255,255,0.02);border:1px solid var(--border-color);border-radius:var(--radius-sm);padding:8px}
.rc-rule-disabled{opacity:0.5}.rc-rule-header{display:flex;align-items:center;gap:6px;flex-wrap:wrap}
.rc-rule-type{padding:2px 6px;border-radius:3px;font-size:10px;font-weight:700;background:rgba(59,130,246,0.2);color:var(--color-primary)}
.rc-rule-field{color:var(--text-primary);font-family:monospace;font-size:11px;min-width:80px}
.rc-rule-op{color:var(--color-primary);font-size:11px;font-weight:600}
.rc-rule-val{flex:1;padding:3px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}
.rc-rule-del{padding:2px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--color-danger);cursor:pointer;font-size:10px}
.rc-rule-children{margin-left:16px;padding-left:8px;border-left:2px solid var(--border-color);display:flex;flex-direction:column;gap:4px;margin-top:4px}
.rc-rule-inner{display:flex;align-items:center;gap:6px;font-size:11px;color:var(--text-muted)}
.rc-btn-sm{padding:2px 6px;border-radius:3px;border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:10px}
.rc-btn-sm:hover{border-color:var(--color-primary);color:var(--color-primary)}
.rc-add-main{padding:6px 12px;border-radius:var(--radius-sm);border:1px dashed var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:12px;width:100%}
.rc-add-main:hover{border-color:var(--color-primary);color:var(--color-primary)}
.rc-preview{background:rgba(0,0,0,0.3);border-radius:var(--radius-sm);padding:10px}
.rc-preview-label{font-size:11px;color:var(--text-muted);margin-bottom:4px}
.rc-sql-out{margin:0;padding:8px;background:rgba(0,0,0,0.4);color:#10b981;font-size:11px;font-family:monospace;border-radius:4px;white-space:pre-wrap;word-break:break-all;max-height:80px;overflow-y:auto}
.rc-footer{display:flex;gap:6px;padding-top:8px;border-top:1px solid var(--border-color)}
/* Field drag config */
.fd-panel{width:680px}.fd-body{padding:12px;max-height:460px;overflow-y:auto}
.fd-layout{display:grid;grid-template-columns:1fr 1fr;gap:12px}
.fd-col-title{font-size:12px;font-weight:600;color:var(--color-primary);margin-bottom:6px;display:flex;align-items:center;gap:6px}
.fd-count{font-size:10px;color:var(--text-muted);font-weight:400}
.fd-search{width:100%;padding:5px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none;margin-bottom:6px;box-sizing:border-box}
.fd-available{max-height:180px;overflow-y:auto;display:flex;flex-direction:column;gap:2px}
.fd-item{padding:4px 8px;border-radius:var(--radius-sm);font-size:11px;font-family:monospace;cursor:pointer;color:var(--text-primary);border:1px solid transparent}
.fd-item:hover{border-color:var(--color-primary);background:rgba(59,130,246,0.1)}
.fd-item.fd-used{background:rgba(16,185,129,0.1);border-color:rgba(16,185,129,0.3);color:#10b981}
.fd-selected-list{display:flex;flex-direction:column;gap:2px;max-height:150px;overflow-y:auto}
.fd-selected-item{display:flex;align-items:center;gap:6px;padding:4px 8px;background:rgba(16,185,129,0.08);border:1px solid rgba(16,185,129,0.2);border-radius:var(--radius-sm);font-size:11px;font-family:monospace;color:#10b981}
.fd-selected-item.fd-wf{background:rgba(245,158,11,0.08);border-color:rgba(245,158,11,0.2);color:#f59e0b}
.fd-remove{padding:1px 4px;border-radius:3px;border:none;background:transparent;color:inherit;cursor:pointer;font-size:10px;opacity:0.7}
.fd-remove:hover{opacity:1}.fd-empty-hint{font-size:11px;color:var(--text-muted);text-align:center;padding:12px}
.fd-sql-preview{margin-top:12px;background:rgba(0,0,0,0.3);border-radius:var(--radius-sm);padding:10px}
.fd-preview-label{font-size:11px;color:var(--text-muted);margin-bottom:4px}
.fd-sql-out{margin:0;padding:8px;background:rgba(0,0,0,0.4);color:#10b981;font-size:11px;font-family:monospace;border-radius:4px;white-space:pre-wrap;word-break:break-all;max-height:80px;overflow-y:auto}
.fd-footer{display:flex;gap:6px;padding-top:8px;border-top:1px solid var(--border-color);margin-top:8px}
/* Chart linkage */
.cl-panel{width:640px}.cl-body{padding:12px;max-height:480px;overflow-y:auto}
.cl-config{display:flex;flex-direction:column;gap:10px}.cl-row{display:flex;gap:8px;flex-wrap:wrap}
.cl-group{display:flex;flex-direction:column;gap:4px;flex:1;min-width:120px}
.cl-group label{font-size:11px;color:var(--text-muted);font-weight:600}
.cl-select{padding:5px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}
.cl-chart-area{display:flex;align-items:flex-end;gap:4px;height:140px;padding:12px;background:rgba(255,255,255,0.02);border-radius:var(--radius-sm);border:1px solid var(--border-color)}
.cl-chart-bar{display:flex;flex-direction:column;align-items:center;justify-content:flex-end;flex:1;border-radius:3px 3px 0 0;cursor:pointer;transition:opacity 0.15s;min-width:20px;padding:2px}
.cl-chart-bar:hover{opacity:0.8}.cl-bar-label{font-size:8px;color:var(--text-muted);position:absolute;bottom:-16px;white-space:nowrap;max-width:60px;overflow:hidden;text-overflow:ellipsis}
.cl-bar-val{font-size:9px;color:var(--text-primary);margin-bottom:2px}
.cl-chart-empty{color:var(--text-muted);font-size:12px;text-align:center;width:100%;align-self:center}
.cl-filters{display:flex;align-items:center;gap:6px;flex-wrap:wrap;padding:8px;background:rgba(59,130,246,0.05);border:1px solid rgba(59,130,246,0.2);border-radius:var(--radius-sm)}
.cl-filter-tag{display:flex;align-items:center;gap:4px;padding:2px 8px;background:rgba(59,130,246,0.15);border:1px solid rgba(59,130,246,0.3);border-radius:10px;font-size:10px;color:var(--color-primary);font-family:monospace}
.cl-filter-tag button{background:none;border:none;color:var(--color-primary);cursor:pointer;font-size:10px}
.cl-clear-all{padding:2px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:10px}
.cl-footer{display:flex;gap:6px;padding-top:8px;border-top:1px solid var(--border-color);margin-top:8px}
/* Advanced templates panel */
.at-panel{width:720px}.at-tabs{display:flex;gap:4px;padding:8px 12px;border-bottom:1px solid var(--border-color)}
.at-tab{padding:4px 12px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:11px}
.at-tab.active{background:var(--color-primary);color:#000;border-color:var(--color-primary)}
.at-grid{padding:12px;max-height:400px;overflow-y:auto;display:flex;flex-direction:column;gap:8px}
.at-card{background:rgba(255,255,255,0.02);border:1px solid var(--border-color);border-radius:var(--radius-md);overflow:hidden}
.at-card-head{display:flex;align-items:center;gap:6px;padding:8px 12px;background:rgba(59,130,246,0.08);border-bottom:1px solid var(--border-color)}
.at-icon{font-size:16px}.at-name{flex:1;color:var(--text-primary);font-size:13px;font-weight:500}
.at-cat-tag{font-size:10px;padding:1px 8px;border-radius:10px;background:rgba(245,158,11,0.15);color:#f59e0b}
.at-code{margin:0;padding:8px 12px;background:rgba(0,0,0,0.3);color:#10b981;font-size:11px;font-family:monospace;white-space:pre-wrap;word-break:break-all;max-height:80px;overflow-y:auto}
.at-card-foot{display:flex;gap:6px;padding:8px 12px;border-top:1px solid var(--border-color)}
.at-empty{color:var(--text-muted);font-size:12px;text-align:center;padding:32px}
/* Button utilities */
.btn-xs{padding:2px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:10px}
.btn-danger{border-color:var(--color-danger);color:var(--color-danger)}.btn-danger:hover{background:rgba(239,68,68,0.1)}
/* Scrollbar polish */
::-webkit-scrollbar{width:6px;height:6px}
::-webkit-scrollbar-track{background:transparent}
::-webkit-scrollbar-thumb{background:var(--border-color);border-radius:3px}
::-webkit-scrollbar-thumb:hover{background:var(--text-muted)}


/* Enhanced stats display */
.enhanced-stats{display:grid;grid-template-columns:repeat(3,1fr);gap:8px;padding:12px;background:rgba(0,0,0,0.2);border-radius:var(--radius-md);margin-top:8px}
.stat-cell{padding:8px;border-radius:var(--radius-sm);background:rgba(59,130,246,0.06);border:1px solid rgba(59,130,246,0.12)}
.stat-cell-label{font-size:10px;color:var(--text-muted);margin-bottom:2px}
.stat-cell-value{font-size:14px;font-weight:600;color:var(--color-primary);font-family:monospace}
.stat-cell-unit{font-size:10px;color:var(--text-muted)}
/* Column stats table */
.col-stats-table{width:100%;border-collapse:collapse;font-size:11px;margin-top:8px}
.col-stats-table th{padding:6px 10px;text-align:left;border-bottom:1px solid var(--border-color);color:var(--text-muted);font-weight:600;font-size:10px;text-transform:uppercase;position:sticky;top:0;background:var(--bg-surface)}
.col-stats-table td{padding:5px 10px;border-bottom:1px solid var(--border-subtle);color:var(--text-primary)}
.col-stats-table tr:hover td{background:var(--bg-hover)}
.col-stats-table .num-val{font-family:monospace;color:var(--color-primary)}
.col-stats-table .str-val{color:var(--text-muted);font-size:10px}
/* SQL editor scroll enhancement */
.sql-editor::-webkit-scrollbar{width:8px}
.sql-editor::-webkit-scrollbar-track{background:var(--bg-surface)}
.sql-editor::-webkit-scrollbar-thumb{background:var(--border-color);border-radius:4px}
.sql-editor::-webkit-scrollbar-thumb:hover{background:var(--text-muted)}
/* Sidebar list scroll */
.sb-list::-webkit-scrollbar{width:4px}
.sb-list::-webkit-scrollbar-thumb{background:var(--border-color);border-radius:2px}
/* Results table enhancement */
.res-table tbody tr{transition:background 0.1s}
.res-table tbody tr:nth-child(even){background:rgba(255,255,255,0.01)}
.res-table tbody tr:nth-child(odd){background:transparent}
.res-table tbody tr:hover td{background:rgba(59,130,246,0.08)}
/* Editor status bar */
.editor-status{padding:6px 0;font-size:12px;color:var(--text-muted);border-top:1px solid var(--border-color);margin-top:8px;display:flex;align-items:center;justify-content:space-between}
/* Header glass effect */
.smd-header{backdrop-filter:blur(10px)}
/* Sidebar glass effect */
.smd-sidebar{backdrop-filter:blur(5px)}
/* Results panel glass */
.smd-results{backdrop-filter:blur(5px)}
/* Modal overlay smooth transition */
.modal-overlay{animation:fadeIn 0.15s ease}
@keyframes fadeIn{from{opacity:0}to{opacity:1}}
.modal-box{animation:slideUp 0.2s ease}
@keyframes slideUp{from{transform:translateY(20px);opacity:0}to{transform:translateY(0);opacity:1}}
/* Button focus ring */
button:focus-visible{outline:2px solid var(--color-primary);outline-offset:2px}
/* Selection color */
::selection{background:rgba(59,130,246,0.3)}
/* Loading spinner */
@keyframes spin{from{transform:rotate(0deg)}to{transform:rotate(360deg)}}
.loading-spinner{animation:spin 1s linear infinite}
/* Tooltip style */
[data-tooltip]{position:relative}
[data-tooltip]:hover::after{content:attr(data-tooltip);position:absolute;bottom:100%;left:50%;transform:translateX(-50%);padding:4px 8px;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-sm);font-size:10px;white-space:nowrap;z-index:100;color:var(--text-primary)}
/* Badge style */
.badge{display:inline-flex;align-items:center;justify-content:center;padding:1px 6px;border-radius:10px;font-size:10px;font-weight:600}
.badge-primary{background:rgba(59,130,246,0.15);color:var(--color-primary)}
.badge-success{background:rgba(16,185,129,0.15);color:#10b981}
.badge-warning{background:rgba(245,158,11,0.15);color:#f59e0b}
.badge-danger{background:rgba(239,68,68,0.15);color:#ef4444}
/* Tag cloud style */
.tag-cloud{display:flex;flex-wrap:wrap;gap:4px}
.tag-item{padding:2px 8px;border-radius:10px;background:rgba(255,255,255,0.05);border:1px solid var(--border-color);font-size:10px;color:var(--text-muted);cursor:pointer;transition:all 0.15s}
.tag-item:hover{border-color:var(--color-primary);color:var(--color-primary)}
/* Grid layout helper */
.grid-2{display:grid;grid-template-columns:1fr 1fr;gap:8px}
.grid-3{display:grid;grid-template-columns:repeat(3,1fr);gap:8px}
.grid-4{display:grid;grid-template-columns:repeat(4,1fr);gap:8px}
/* Flex utilities */
.flex-center{display:flex;align-items:center;justify-content:center}
.flex-between{display:flex;align-items:center;justify-content:space-between}
.flex-wrap{display:flex;flex-wrap:wrap;gap:8px}
/* Text utilities */
.text-muted{color:var(--text-muted)}
.text-primary{color:var(--color-primary)}
.text-success{color:#10b981}
.text-warning{color:#f59e0b}
.text-danger{color:#ef4444}
.text-mono{font-family:'JetBrains Mono',monospace}
.text-sm{font-size:11px}
.text-xs{font-size:10px}
.font-mono{font-family:'JetBrains Mono',monospace}
.font-semibold{font-weight:600}
.font-bold{font-weight:700}
/* Spacing utilities */
.gap-1{gap:4px}.gap-2{gap:8px}.gap-3{gap:12px}.gap-4{gap:16px}
.p-1{padding:4px}.p-2{padding:8px}.p-3{padding:12px}.p-4{padding:16px}
.m-1{margin:4px}.m-2{margin:8px}.m-3{margin:12px}.m-4{margin:16px}
/* Border utilities */
.border-dashed{border-style:dashed}
.border-primary{border-color:var(--color-primary)}
.rounded-sm{border-radius:var(--radius-sm)}
.rounded-md{border-radius:var(--radius-md)}
.rounded-full{border-radius:9999px}
/* Display utilities */
.hidden{display:none}
.block{display:block}
.inline-block{display:inline-block}
.flex{display:flex}
.grid{display:grid}
.relative{position:relative}
.absolute{position:absolute}
/* Overflow utilities */
.overflow-auto{overflow:auto}
.overflow-hidden{overflow:hidden}
.overflow-y-auto{overflow-y:auto}
.overflow-x-auto{overflow-x:auto}
/* Width/Height utilities */
.w-full{width:100%}.h-full{height:100%}
.min-h-0{min-height:0}
.flex-1{flex:1}
/* Max height utilities */
.max-h-20{max-height:80px}
.max-h-32{max-height:128px}
.max-h-48{max-height:192px}
.max-h-64{max-height:256px}
.max-h-80{max-height:320px}
/* Transition utilities */
.transition{transition:all 0.15s ease}
.transition-fast{transition:all 0.1s ease}
.transition-slow{transition:all 0.3s ease}
/* Cursor utilities */
.cursor-pointer{cursor:pointer}
.cursor-default{cursor:default}
/* Opacity utilities */
.opacity-50{opacity:0.5}
.opacity-75{opacity:0.75}
.opacity-100{opacity:1}
/* Z-index utilities */
.z-0{z-index:0}.z-10{z-index:10}.z-20{z-index:20}.z-50{z-index:50}
/* Whitespace utilities */
.whitespace-nowrap{white-space:nowrap}
.whitespace-pre{white-space:pre}
.whitespace-pre-wrap{white-space:pre-wrap}
/* Text overflow utilities */
.truncate{overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.overflow-ellipsis{text-overflow:ellipsis}
/* Flex direction utilities */
.flex-col{flex-direction:column}
.flex-row{flex-direction:row}
/* Align utilities */
.items-start{align-items:flex-start}
.items-center{align-items:center}
.items-end{align-items:flex-end}
.justify-start{justify-content:flex-start}
.justify-center{justify-content:center}
.justify-end{justify-content:flex-end}
.justify-between{justify-content:space-between}
/* Gap utilities */
.gap-x-1{column-gap:4px}.gap-x-2{column-gap:8px}
.gap-y-1{row-gap:4px}.gap-y-2{row-gap:8px}
/* Padding utilities for panels */
.p-6{padding:24px}.p-8{padding:32px}
.pt-2{padding-top:8px}.pb-2{padding-bottom:8px}
.pl-2{padding-left:8px}.pr-2{padding-right:8px}
/* Margin utilities for panels */
.mb-2{margin-bottom:8px}.mb-4{margin-bottom:16px}
.mt-2{margin-top:8px}.mt-4{margin-top:16px}
.ml-2{margin-left:8px}.mr-2{margin-right:8px}
/* Font size utilities */
.text-xs{font-size:10px}.text-sm{font-size:11px}
.text-base{font-size:12px}.text-lg{font-size:14px}
.text-xl{font-size:16px}.text-2xl{font-size:18px}
/* Font weight utilities */
.font-light{font-weight:300}.font-normal{font-weight:400}
.font-medium{font-weight:500}.font-semibold{font-weight:600}
.font-bold{font-weight:700}.font-extrabold{font-weight:800}
/* Line height utilities */
.leading-tight{line-height:1.25}
.leading-normal{line-height:1.5}
.leading-relaxed{line-height:1.75}
/* Letter spacing utilities */
.tracking-tight{letter-spacing:-0.025em}
.tracking-normal{letter-spacing:0}
.tracking-wide{letter-spacing:0.025em}
/* Transform utilities */
.hover\\:scale-105:hover{transform:scale(1.05)}
.hover\\:brightness-110:hover{filter:brightness(1.1)}
/* Active state */
.active\\:bg-primary-active:active{background:rgba(59,130,246,0.2)}
.active\\:border-primary-active:active{border-color:var(--color-primary)}
/* Disabled state */
.disabled\\:opacity-50:disabled{opacity:0.5}
.disabled\\:cursor-not-allowed:disabled{cursor:not-allowed}
/* Focus state */
.focus\\:ring-2:focus{outline:none;box-shadow:0 0 0 2px var(--color-primary)}
.focus\\:border-primary:focus{border-color:var(--color-primary)}
/* Print utilities */
@media print{.no-print{display:none!important}}
/* Reduced motion */
@media(prefers-reduced-motion:reduce){*{animation-duration:0.01ms!important;transition-duration:0.01ms!important}}
/* High contrast */
@media(hcontrasts){.high-contrast{border-width:2px}}
/* Dark mode enhancement */
.dark .glass-card{background:rgba(15,23,42,0.6)}
/* Mobile responsive */
@media(max-width:768px){.smd-body{flex-direction:column}.smd-sidebar{width:100%!important;max-height:200px}.smd-results{width:100%!important;max-height:300px}.modal-box{width:95vw!important}}
/* Tablet responsive */
@media(min-width:769px) and (max-width:1024px){.smd-sidebar{width:200px}.smd-results{width:320px}}
/* Large screen */
@media(min-width:1400px){.smd-results{width:480px}}
/* Print styles */
@media print{.smd{height:auto!important}.smd-header,.smd-sidebar,.smd-results{break-inside:avoid}}
/* Performance optimization */
.smd *{will-change:auto}
.smd-editor{contain:layout style}
.smd-results{contain:layout style}
/* Accessibility */
.sr-only{position:absolute;width:1px;height:1px;padding:0;margin:-1px;overflow:hidden;clip:rect(0,0,0,0);white-space:nowrap;border-width:0}
/* Focus visible for keyboard navigation */
:focus-visible{outline:2px solid var(--color-primary);outline-offset:2px;border-radius:var(--radius-sm)}
/* Smooth scrolling */
html{scroll-behavior:smooth}
body{scroll-padding-top:80px}
/* Custom scrollbar for webkit */
::-webkit-scrollbar{width:6px;height:6px}
::-webkit-scrollbar-track{background:transparent}
::-webkit-scrollbar-thumb{background:rgba(148,163,184,0.2);border-radius:3px}
::-webkit-scrollbar-thumb:hover{background:rgba(148,163,184,0.4)}
/* Selection color */
::selection{background:rgba(59,130,246,0.3);color:#fff}
::-moz-selection{background:rgba(59,130,246,0.3);color:#fff}

</style>
