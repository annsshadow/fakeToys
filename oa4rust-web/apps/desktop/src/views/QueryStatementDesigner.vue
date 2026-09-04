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
    if (wh) s += "
WHERE " + wh
  }
  if (veOrderBy.value) s += "
ORDER BY " + veOrderBy.value + " " + veOrderDir.value
  if (veLimit.value) s += "
LIMIT " + veLimit.value
  return s
})
const generatedFieldDragSql = computed(() => {
  let s = "SELECT " + (fdSelectFields.value.length ? fdSelectFields.value.join(", ") : "*")
  if (allSchemaFields.value[0]) s += " FROM " + allSchemaFields.value[0].split(".")[0]
  if (fdWhereFields.value.length) s += "
WHERE " + fdWhereFields.value.map(f => f + " IS NOT NULL").join(" AND ")
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
  else sql.value += "
WHERE " + w
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
function applyAdvancedTemplate(t: any) { sql.value = t.code + "
"; showAdvancedTemplates.value = false }
function saveAdvancedTemplate(t: any) { templates.value.push({id:"t"+Date.now(),name:t.name,category:t.category,code:t.code,icon:t.icon}); showAdvancedTemplates.value = false }
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
</style>
