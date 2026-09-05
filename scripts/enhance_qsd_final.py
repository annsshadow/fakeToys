#!/usr/bin/env python3
"""Enhance QueryStatementDesigner.vue from ~740 to ~2000 lines."""
import re

path = 'D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/QueryStatementDesigner.vue'
with open(path, 'r', encoding='utf-8') as f:
    content = f.read()
lines = content.split('\n')

# ── Step 1: Fix ]`}]]` patterns on lines 98-100 ──────────────────────────
for i in range(len(lines)):
    lines[i] = lines[i].replace(
        ":class=\"['emt',{active:editorMode==='write']}]\"",
        ":class=\"{active:editorMode==='write'}\"")
    lines[i] = lines[i].replace(
        ":class=\"['emt',{active:editorMode==='preview']}]\"",
        ":class=\"{active:editorMode==='preview'}\"")
    lines[i] = lines[i].replace(
        ":class=\"['emt',{active:editorMode==='explain']}]\"",
        ":class=\"{active:editorMode==='explain'}\"")

# ── Step 2: Add new toolbar buttons after existing ones ───────────────────
# Insert new buttons before the last `</div>` of smd-actions
for i, line in enumerate(lines):
    if line.strip() == '<button class="btn btn-outline" @click="showFavoritePanel=!showFavoritePanel" :class="{active:showFavoritePanel}" title="收藏语句">⭐ 收藏</button>':
        # Add new buttons after this line
        new_buttons = [
            '        <button class="btn btn-outline" @click="showVisualEditor=true" title="SQL可视化编辑器">🌳 语法树</button>',
            '        <button class="btn btn-outline" @click="showRuleChain=true" title="条件规则链">📋 规则链</button>',
            '        <button class="btn btn-outline" @click="showFieldDrag=true" title="字段拖拽配置">🧩 字段配置</button>',
            '        <button class="btn btn-outline" @click="showChartLinkage=true" title="执行结果图表联动">📊 图表联动</button>',
            '        <button class="btn btn-outline" @click="showAdvancedTemplates=true" title="高级模板">🚀 高级模板</button>',
        ]
        lines[i] = line + '\n' + '\n'.join(new_buttons)
        break

# ── Step 3: Add new modal panels before the closing </template> ───────────
new_modals = r'''
    <!-- SQL Syntax Tree Visual Editor -->
    <div v-if="showVisualEditor" class="modal-overlay" @click.self="showVisualEditor=false">
      <div class="modal-box ve-panel">
        <div class="modal-header"><span>🌳 SQL语法树编辑器</span><button class="btn-close" @click="showVisualEditor=false">✕</button></div>
        <div class="ve-body">
          <div class="ve-fields-row">
            <div class="ve-field-group"><label>SELECT 字段</label>
              <div class="ve-tags">
                <span v-for="(f,fi) in veSelectFields" :key="fi" class="ve-tag">{{f}} <span class="ve-tag-del" @click="veSelectFields.splice(fi,1)">✕</span></span>
                <input v-model="veNewField" @keydown.enter="addVeField" class="ve-tag-input" placeholder="输入字段名+回车" />
              </div>
            </div>
          </div>
          <div class="ve-row">
            <div class="ve-field-group"><label>FROM 表名</label>
              <select v-model="veFromTable" class="ve-select"><option value="">选择表...</option>
                <option v-for="t in allTables" :key="t.name" :value="t.name">{{t.name}}</option>
              </select></div>
            <div class="ve-field-group"><label>WHERE 条件</label>
              <div class="ve-conditions">
                <div v-for="(c,ci) in veWhereConditions" :key="ci" class="ve-cond-row">
                  <select v-model="c.field" class="ve-sel-sm"><option v-for="t in allTables" :key="t.name" v-for="f in (tableFieldsByTable(t.name)||[])" :value="f.name">{{f.name}}</option></select>
                  <select v-model="c.op" class="ve-sel-sm"><option>=</option><option>!=</option><option>&lt;</option><option>&gt;</option><option>LIKE</option><option>IN</option><option>IS NULL</option></select>
                  <input v-model="c.value" class="ve-input-sm" placeholder="值" />
                  <button class="ve-del-btn" @click="veWhereConditions.splice(ci,1)">✕</button>
                </div>
                <button class="ve-add-btn" @click="veWhereConditions.push({field:'',op:'=',value:''})">+ 添加条件</button>
              </div>
            </div>
          </div>
          <div class="ve-row">
            <div class="ve-field-group"><label>ORDER BY</label><input v-model="veOrderBy" class="ve-input" placeholder="字段名" /></div>
            <div class="ve-field-group"><label>方向</label>
              <select v-model="veOrderDir" class="ve-select-sm"><option>ASC</option><option>DESC</option></select></div>
            <div class="ve-field-group"><label>LIMIT</label><input v-model.number="veLimit" class="ve-input-sm" type="number" min="1" max="10000" /></div>
          </div>
          <div class="ve-preview">
            <div class="ve-preview-label">生成 SQL:</div>
            <pre class="ve-sql-out">{{generatedVisualSql}}</pre>
          </div>
        </div>
        <div class="ve-footer">
          <button class="btn-sm" @click="applyVisualEditor">✓ 应用到编辑器</button>
          <button class="btn-sm" @click="clearVisualEditor">清空</button>
        </div>
      </div>
    </div>

    <!-- Rule Chain Editor -->
    <div v-if="showRuleChain" class="modal-overlay" @click.self="showRuleChain=false">
      <div class="modal-box rc-panel">
        <div class="modal-header"><span>📋 查询条件规则链</span><button class="btn-close" @click="showRuleChain=false">✕</button></div>
        <div class="rc-body">
          <div class="rc-rule-list">
            <div v-for="(r,ri) in ruleChain" :key="ri" :class="['rc-rule',!r.enabled?'rc-rule-disabled':'']">
              <div class="rc-rule-header">
                <input type="checkbox" v-model="r.enabled" />
                <span class="rc-rule-type">{{r.type==='AND'?'∧ AND':'∨ OR'}}</span>
                <span class="rc-rule-field">{{r.field}}</span>
                <span class="rc-rule-op">{{r.op}}</span>
                <input :value="r.value" @input="ruleChain[ri].value=$event.target.value" class="rc-rule-val" :placeholder="'值...'"/>
                <button class="rc-rule-del" @click="ruleChain.splice(ri,1)">✕</button>
              </div>
              <div class="rc-rule-children" v-if="r.children?.length">
                <div v-for="(cr,cari) in r.children" :key="cari" class="rc-rule-inner">
                  <span class="rc-rule-type">{{cr.type==='AND'?'∧':'∨'}}</span>
                  <span>{{cr.field}} {{cr.op}} {{cr.value}}</span>
                  <button @click="r.children.splice(cari,1)">✕</button>
                </div>
              </div>
              <div class="rc-rule-actions">
                <button class="rc-btn-sm" @click="addNestedRule(ri,'AND')">+ AND子条件</button>
                <button class="rc-btn-sm" @click="addNestedRule(ri,'OR')">+ OR子条件</button>
              </div>
            </div>
          </div>
          <button class="rc-add-main" @click="ruleChain.push({type:'AND',field:'',op:'=',value:'',enabled:true,children:[]})">+ 添加主条件</button>
          <div class="rc-preview">
            <div class="rc-preview-label">生成条件:</div>
            <pre class="rc-sql-out">{{generatedRuleChainCondition}}</pre>
          </div>
        </div>
        <div class="rc-footer">
          <button class="btn-sm" @click="applyRuleChain">✓ 应用到SQL</button>
          <button class="btn-sm" @click="ruleChain=[]">清空</button>
        </div>
      </div>
    </div>

    <!-- Field Drag Config Panel -->
    <div v-if="showFieldDrag" class="modal-overlay" @click.self="showFieldDrag=false">
      <div class="modal-box fd-panel">
        <div class="modal-header"><span>🧩 字段拖拽配置</span><button class="btn-close" @click="showFieldDrag=false">✕</button></div>
        <div class="fd-body">
          <div class="fd-layout">
            <div class="fd-schema-col">
              <div class="fd-col-title">可用字段</div>
              <input v-model="fdSchemaSearch" class="fd-search" placeholder="搜索字段..." />
              <div class="fd-available">
                <div v-for="f in filteredFdSchema" :key="f" :class="['fd-item',fdSelectFields.includes(f)?'fd-used':'']" draggable="true"
                  @dragstart="fdDragStart(f)" @click="toggleFdField(f)">
                  {{f}}
                </div>
              </div>
            </div>
            <div class="fd-target-col">
              <div class="fd-col-title">SELECT 字段 <span class="fd-count">{{fdSelectFields.length}}</span></div>
              <div class="fd-selected-list">
                <div v-for="(f,fi) in fdSelectFields" :key="f" class="fd-selected-item">
                  <span>{{f}}</span>
                  <button class="fd-remove" @click="fdSelectFields.splice(fi,1)">✕</button>
                </div>
                <div v-if="!fdSelectFields.length" class="fd-empty-hint">拖拽或点击添加字段</div>
              </div>
              <div class="fd-col-title" style="margin-top:12px">WHERE 过滤字段 <span class="fd-count">{{fdWhereFields.length}}</span></div>
              <div class="fd-selected-list">
                <div v-for="(f,fi) in fdWhereFields" :key="f" class="fd-selected-item fd-wf">
                  <span>{{f}}</span>
                  <button class="fd-remove" @click="fdWhereFields.splice(fi,1)">✕</button>
                </div>
                <div v-if="!fdWhereFields.length" class="fd-empty-hint">拖拽或点击添加过滤字段</div>
              </div>
            </div>
          </div>
          <div class="fd-sql-preview">
            <div class="fd-preview-label">生成 SQL:</div>
            <pre class="fd-sql-out">{{generatedFieldDragSql}}</pre>
          </div>
        </div>
        <div class="fd-footer">
          <button class="btn-sm" @click="fdApply">✓ 应用到编辑器</button>
          <button class="btn-sm" @click="fdAutoFill">自动填充</button>
          <button class="btn-sm" @click="fdReset">重置</button>
        </div>
      </div>
    </div>

    <!-- Chart Linkage Panel -->
    <div v-if="showChartLinkage" class="modal-overlay" @click.self="showChartLinkage=false">
      <div class="modal-box cl-panel">
        <div class="modal-header"><span>📊 执行结果图表联动</span><button class="btn-close" @click="showChartLinkage=false">✕</button></div>
        <div class="cl-body">
          <div class="cl-config">
            <div class="cl-row">
              <div class="cl-group"><label>模式</label>
                <select v-model="clMode" class="cl-select">
                  <option value="filter">筛选联动</option>
                  <option value="aggregate">聚合分析</option>
                  <option value="crossfilter">交叉过滤</option>
                </select>
              </div>
              <div class="cl-group"><label>X 轴字段</label>
                <select v-model="clXAxis" class="cl-select">
                  <option v-for="h in resultHeaders" :key="h" :value="h">{{h}}</option>
                </select>
              </div>
              <div class="cl-group"><label>Y 轴字段</label>
                <select v-model="clYAxis" class="cl-select">
                  <option v-for="h in resultHeaders" :key="h" :value="h">{{h}}</option>
                </select>
              </div>
              <div class="cl-group" v-if="clMode==='filter'"><label>联动筛选字段</label>
                <select v-model="clFilterField" class="cl-select">
                  <option v-for="h in resultHeaders" :key="h" :value="h">{{h}}</option>
                </select>
              </div>
            </div>
            <div class="cl-chart-area">
              <div class="cl-chart-bar" v-for="d in clChartData" :key="d.label"
                :style="{height:Math.max(4,d.h)+'px',background:d.color}"
                @click="clMode==='filter'&&clFilterByLabel(d.label)">
                <div class="cl-bar-label">{{d.label}}</div>
                <div class="cl-bar-val">{{d.value}}</div>
              </div>
              <div v-if="!clChartData.length" class="cl-chart-empty">请先执行SQL获取数据</div>
            </div>
            <div class="cl-filters" v-if="clActiveFilters.length">
              <div class="cl-filter-tag" v-for="f in clActiveFilters" :key="f">{{f}}=... <button @click="clActiveFilters.splice(clActiveFilters.indexOf(f),1)">✕</button></div>
              <button class="cl-clear-all" @click="clActiveFilters=[]">清除全部筛选</button>
            </div>
          </div>
        </div>
        <div class="cl-footer">
          <button class="btn-sm" @click="applyChartLinkage">✓ 应用</button>
          <button class="btn-sm" @click="testChartLinkage">▶ 测试预览</button>
        </div>
      </div>
    </div>

    <!-- Advanced Templates Panel -->
    <div v-if="showAdvancedTemplates" class="modal-overlay" @click.self="showAdvancedTemplates=false">
      <div class="modal-box at-panel">
        <div class="modal-header"><span>🚀 高级SQL模板库</span><button class="btn-close" @click="showAdvancedTemplates=false">✕</button></div>
        <div class="at-tabs">
          <button :class="['at-tab',{active:atTab==='all'}]" @click="atTab='all'">全部</button>
          <button :class="['at-tab',{active:atTab==='analytics'}]" @click="atTab='analytics'">分析</button>
          <button :class="['at-tab',{active:atTab==='report'}]" @click="atTab='report'">报表</button>
          <button :class="['at-tab',{active:atTab==='admin'}]" @click="atTab='admin'">管理</button>
          <button :class="['at-tab',{active:atTab==='optimize'}]" @click="atTab='optimize'">优化</button>
        </div>
        <div class="at-grid">
          <div v-for="(t,ti) in filteredAdvancedTpls" :key="t.id" class="at-card">
            <div class="at-card-head">
              <span class="at-icon">{{t.icon}}</span>
              <span class="at-name">{{t.name}}</span>
              <span class="at-cat-tag">{{t.category}}</span>
            </div>
            <pre class="at-code">{{t.code}}</pre>
            <div class="at-card-foot">
              <button class="btn-sm" @click="applyAdvancedTemplate(t)">应用</button>
              <button class="btn-sm" @click="saveAdvancedTemplate(t)">收藏</button>
            </div>
          </div>
        </div>
        <div v-if="filteredAdvancedTpls.length===0" class="at-empty">该分类暂无模板</div>
      </div>
    </div>
'''

# Find insertion point: after the </div> closing showNewTemplate modal, before </template>
for i in range(len(lines)-1, -1, -1):
    if '</template>' in lines[i]:
        lines.insert(i, new_modals)
        break

# ── Step 4: Add state variables before the </script> closing tag ──────────
state_additions = r'''
// --- Visual SQL Editor State ---
const showVisualEditor = ref(false)
const veSelectFields = ref<string[]>(["id", "name"])
const veFromTable = ref(""), veOrderBy = ref(""), veOrderDir = ref("DESC")
const veWhereConditions = ref<Array<{field:string;op:string;value:string}>>([])
const veLimit = ref(100)
const veNewField = ref("")

// --- Rule Chain State ---
const showRuleChain = ref(false)
const ruleChain = ref<Array<{type:string;field:string;op:string;value:string;enabled:boolean;children?:Array<{type:string;field:string;op:string;value:string}>}>>([])

// --- Field Drag Config State ---
const showFieldDrag = ref(false)
const fdSchemaSearch = ref("")
const fdSelectFields = ref<string[]>([])
const fdWhereFields = ref<string[]>([])

// --- Chart Linkage State ---
const showChartLinkage = ref(false)
const clMode = ref("filter"), clXAxis = ref(""), clYAxis = ref(""), clFilterField = ref("")
const clActiveFilters = ref<string[]>([])
const clPreviewData = ref<string[]>([])

// --- Advanced Templates State ---
const showAdvancedTemplates = ref(false)
const atTab = ref("all")
'''

# Insert before </script>
for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</script>':
        lines.insert(i, state_additions)
        break

# ── Step 5: Add functions before </script> ────────────────────────────────
functions_addition = r'''
// --- Visual Editor Functions ---
function addVeField() {
  const f = veNewField.value.trim()
  if (f && !veSelectFields.value.includes(f)) veSelectFields.value.push(f)
  veNewField.value = ""
}
function generateVeSql(): string {
  let s = "SELECT " + (veSelectFields.value.length ? veSelectFields.value.join(", ") : "*")
  if (veFromTable.value) s += " FROM " + veFromTable.value
  const wh = veWhereConditions.value.filter(c => c.field && c.value).map(c => c.field + " " + c.op + " " + String.fromCharCode(39) + c.value + String.fromCharCode(39)).join(" AND ")
  if (wh) s += "\nWHERE " + wh
  if (veOrderBy.value) s += "\nORDER BY " + veOrderBy.value + " " + veOrderDir.value
  if (veLimit.value) s += "\nLIMIT " + veLimit.value
  return s
}
const generatedVisualSql = computed(() => generateVeSql())
function applyVisualEditor() { sql.value = generateVeSql(); showVisualEditor.value = false }
function clearVisualEditor() { veSelectFields.value = []; veFromTable.value = ""; veOrderBy.value = ""; veWhereConditions.value = []; veLimit.value = 100; veNewField.value = "" }

// --- Rule Chain Functions ---
function addNestedRule(ri: number, type: string) {
  const parent = ruleChain.value[ri]
  if (!parent.children) parent.children = []
  parent.children!.push({ type, field: '', op: '=', value: '', children: [] })
}
function generateRuleChainCondition(): string {
  if (!ruleChain.value.length) return ""
  const parts = ruleChain.value.filter(r => r.enabled && r.field).map(r => r.field + " " + r.op + " " + String.fromCharCode(39) + r.value + String.fromCharCode(39))
  const prefix = ruleChain.value[0]?.type === 'OR' ? " OR " : " AND "
  return parts.join(prefix)
}
const generatedRuleChainCondition = computed(() => generateRuleChainCondition())
function applyRuleChain() {
  const cond = generateRuleChainCondition()
  if (cond && /WHERE/i.test(sql.value)) {
    sql.value = sql.value.replace(/WHERE\s+[^;]+/i, "WHERE " + cond)
  } else if (cond) {
    sql.value += "\nWHERE " + cond
  }
  showRuleChain.value = false
}
function toggleRule(ri: number) { ruleChain.value[ri].enabled = !ruleChain.value[ri].enabled }

// --- Field Drag Functions ---
function toggleFdField(f: string) {
  const idx = fdSelectFields.value.indexOf(f)
  if (idx >= 0) fdSelectFields.value.splice(idx, 1)
  else fdSelectFields.value.push(f)
}
function fdDragStart(f: string) { fdDraggedField = f }
let fdDraggedField = ""
function fdApply() {
  const tbl = allTables.value[0]?.name || "table_name"
  let s = "SELECT " + (fdSelectFields.value.length ? fdSelectFields.value.join(", ") : "*")
  s += " FROM " + tbl
  if (fdWhereFields.value.length) s += "\nWHERE " + fdWhereFields.value.map(f => f + " IS NOT NULL").join(" AND ")
  sql.value = s; showFieldDrag.value = false
}
function fdAutoFill() {
  const all = allTables.value.flatMap(t => tableFieldsByTable(t.name) || [])
  fdSelectFields.value = all.slice(0, 10).map(f => f.name)
  fdWhereFields.value = all.slice(0, 3).map(f => f.name)
}
function fdReset() { fdSelectFields.value = []; fdWhereFields.value = []; fdSchemaSearch.value = "" }
function getFilteredFdSchema(): string[] {
  if (!fdSchemaSearch.value.trim()) return allTables.value.flatMap(t => (tableFieldsByTable(t.name)||[]).map(f => t.name+'.'+f.name))
  const q = fdSchemaSearch.value.toLowerCase()
  return allTables.value.flatMap(t => (tableFieldsByTable(t.name)||[]).filter(f => f.name.toLowerCase().includes(q) || t.name.toLowerCase().includes(q)).map(f => t.name+'.'+f.name))
}
const filteredFdSchema = computed(() => getFilteredFdSchema())
function generatedFieldDragSql(): string {
  const tbl = allTables.value[0]?.name || "table_name"
  let s = "SELECT " + (fdSelectFields.value.length ? fdSelectFields.value.join(", ") : "*")
  s += " FROM " + tbl
  if (fdWhereFields.value.length) s += " WHERE " + fdWhereFields.value.map(f => f + " IS NOT NULL").join(" AND ")
  return s
}

// --- Chart Linkage Functions ---
function getClChartData(): Array<{label:string;value:number;h:number;color:string}> {
  if (!resultData.value.length || !clXAxis.value || !clYAxis.value) return []
  const map = new Map<string,number>()
  resultData.value.forEach(row => {
    const key = String(row[clXAxis.value])
    const val = Number(row[clYAxis.value]) || 1
    map.set(key, (map.get(key) || 0) + val)
  })
  const entries = [...map.entries()].sort((a,b) => b[1]-a[1]).slice(0, 20)
  const maxVal = Math.max(1, ...entries.map(([,v]) => v))
  const colors = ["#3b82f6","#10b981","#f59e0b","#ef4444","#8b5cf6","#ec4899","#06b6d4","#f97316"]
  return entries.map(([label, value], i) => ({ label, value, h: Math.round(value/maxVal*120), color: colors[i%colors.length] }))
}
const clChartData = computed(() => getClChartData())
function clFilterByLabel(label: string) {
  if (!clFilterField.value) return
  const cond = clFilterField.value + " = " + String.fromCharCode(39) + label + String.fromCharCode(39)
  if (!clActiveFilters.value.includes(label)) clActiveFilters.value.push(label)
  if (/WHERE/i.test(sql.value)) {
    sql.value = sql.value.replace(/WHERE\s+[^;]+/i, "WHERE " + cond)
  } else {
    sql.value += "\nWHERE " + cond
  }
}
function applyChartLinkage() { clActiveFilters.value = []; clXAxis.value = resultHeaders.value[0] || ""; clYAxis.value = resultHeaders.value[1] || ""; showChartLinkage.value = false }
function testChartLinkage() { clChartData.value = getClChartData() }

// --- Advanced Templates ---
const advancedTemplates = ref<Array<{id:string;name:string;category:string;code:string;icon:string}>>([
  {id:"at1",name:"分页排名子查询",category:"analytics",icon:"🏆",code:"SELECT * FROM (\n  SELECT *, ROW_NUMBER() OVER (PARTITION BY dept_id ORDER BY salary DESC) as rn\n  FROM employees\n) t WHERE t.rn <= 10\nORDER BY dept_id, rn"},
  {id:"at2",name:"交叉表透视",category:"report",icon:"📊",code:"SELECT category,\n  SUM(CASE WHEN status='pending' THEN 1 ELSE 0 END) as pending,\n  SUM(CASE WHEN status='done' THEN 1 ELSE 0 END) as done,\n  SUM(CASE WHEN status='failed' THEN 1 ELSE 0 END) as failed\nFROM orders GROUP BY category ORDER BY pending DESC"},
  {id:"at3",name:"递归CTE层级",category:"analytics",icon:"🌲",code:"WITH RECURSIVE emp_hierarchy AS (\n  SELECT id, name, manager_id, 1 as level\n  FROM employees WHERE manager_id IS NULL\n  UNION ALL\n  SELECT e.id, e.name, e.manager_id, eh.level + 1\n  FROM employees e JOIN emp_hierarchy eh ON e.manager_id = eh.id\n)\nSELECT * FROM emp_hierarchy ORDER BY level, name"},
  {id:"at4",name:"窗口函数移动平均",category:"analytics",icon:"📈",code:"SELECT order_date, amount,\n  AVG(amount) OVER (ORDER BY order_date ROWS BETWEEN 6 PRECEDING AND CURRENT ROW) as moving_avg_7d,\n  LAG(amount, 1) OVER (ORDER BY order_date) as prev_amount,\n  ROW_NUMBER() OVER (PARTITION BY DATE(order_date) ORDER BY amount DESC) as day_rank\nFROM orders"},
  {id:"at5",name:"同比环比计算",category:"report",icon:"🔄",code:"WITH monthly AS (\n  SELECT DATE_TRUNC('month', order_date) as month, SUM(amount) as total\n  FROM orders GROUP BY 1\n)\nSELECT month, total,\n  total - LAG(total) OVER (ORDER BY month) as mom_change,\n  total - LAG(total, 12) OVER (ORDER BY month) as yoy_change,\n  ROUND((total - LAG(total) OVER (ORDER BY month)) / NULLIF(LAG(total) OVER (ORDER BY month),0) * 100, 2) as mom_pct\nFROM monthly ORDER BY month"},
  {id:"at6",name:"数据去重保留最新",category:"admin",icon:"🧹",code:"DELETE FROM users WHERE id NOT IN (\n  SELECT DISTINCT ON (email) id FROM users ORDER BY email, created_at DESC\n) RETURNING id, email, created_at"},
  {id:"at7",name:"慢查询日志分析",category:"optimize",icon:"🐌",code:"SELECT query, calls, total_exec_time, mean_exec_time, rows,\n  ROUND(100.0 * calls / SUM(calls) OVER (), 2) as pct_calls\nFROM pg_stat_statements\nWHERE dbid = (SELECT oid FROM pg_database WHERE datname = current_database())\nORDER BY mean_exec_time DESC LIMIT 20"},
  {id:"at8",name:"缺失数据分析",category:"analytics",icon:"❓",code:"SELECT column_name, table_name,\n  COUNT(*) as total_rows,\n  SUM(CASE WHEN column_name IS NULL THEN 1 ELSE 0 END) as null_count,\n  ROUND(100.0 * SUM(CASE WHEN column_name IS NULL THEN 1 ELSE 0 END) / COUNT(*), 2) as null_pct\nFROM (\n  SELECT id, name, email, phone FROM users\n) sub\nUNPIVOT (column_name, column_value) FOR col IN (id, name, email, phone)\nGROUP BY column_name, table_name"},
  {id:"at9",name:"索引使用率统计",category:"optimize",icon:"🔍",code:"SELECT schemaname, relname as table_name, indexrelname as index_name,\n  idx_scan as times_used, idx_tup_read, idx_tup_fetch,\n  pg_size_pretty(pg_relation_size(indexrelid)) as index_size\nFROM pg_stat_user_indexes\nORDER BY idx_scan ASC LIMIT 20"},
  {id:"at10",name:"TOP-N每组排序",category:"report",icon:"🎯",code:"WITH ranked AS (\n  SELECT dept_id, name, salary,\n    RANK() OVER (PARTITION BY dept_id ORDER BY salary DESC) as rank\n  FROM employees\n)\nSELECT * FROM ranked WHERE rank <= 3\nORDER BY dept_id, rank"},
  {id:"at11",name:"数据迁移批量更新",category:"admin",icon:"📦",code:"WITH updated AS (\n  UPDATE orders SET status = 'archived', updated_at = NOW()\n  WHERE created_at < '2023-01-01' AND status = 'completed'\n  RETURNING id, status, updated_at\n)\nSELECT COUNT(*) as archived_count FROM updated"},
  {id:"at12",name:"JSON字段查询",category:"analytics",icon:"🔑",code:"SELECT id, name,\n  data->>'email' as email,\n  data->'tags'->0 as first_tag,\n  jsonb_array_length(data->'permissions') as perm_count\nFROM users\nWHERE data->>'status' = 'active'\n  AND data->'tags' ? 'premium'\nORDER BY perm_count DESC"},
])
const myAdvancedTemplates = ref<Array<{id:string;name:string;category:string;code:string;icon:string}>>([])
function applyAdvancedTemplate(t: any) { sql.value = t.code + "\n"; showAdvancedTemplates.value = false }
function saveAdvancedTemplate(t: any) { myAdvancedTemplates.value.push({...t, id:"mat"+Date.now()}) }
const filteredAdvancedTpls = computed(() => {
  if (atTab.value === 'all') return advancedTemplates.value
  return advancedTemplates.value.filter(t => t.category === atTab.value)
})
'''

# Insert before </script> (find the position after state additions)
script_end_idx = None
for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</script>':
        script_end_idx = i
        break
if script_end_idx:
    lines.insert(script_end_idx, functions_addition)

# ── Step 6: Add styles before </style> ────────────────────────────────────
new_styles = r'''
/* Visual SQL Editor */
.ve-panel{width:680px}.ve-body{padding:12px;max-height:440px;overflow-y:auto;display:flex;flex-direction:column;gap:10px}.ve-fields-row{display:flex;gap:8px}.ve-field-group{display:flex;flex-direction:column;gap:4px;flex:1}.ve-field-group label{font-size:11px;color:var(--text-muted);font-weight:600}.ve-tags{display:flex;flex-wrap:wrap;gap:4px;padding:6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);min-height:32px}.ve-tag{display:inline-flex;align-items:center;gap:4px;padding:2px 8px;background:rgba(59,130,246,0.15);border:1px solid rgba(59,130,246,0.3);border-radius:10px;font-size:11px;color:var(--color-primary);font-family:monospace}.ve-tag-del{cursor:pointer;opacity:0.7;font-size:10px}.ve-tag-input{border:none;background:transparent;color:var(--text-primary);font-size:11px;outline:none;flex:1;min-width:80px}.ve-row{display:flex;gap:8px}.ve-select{flex:1;padding:6px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:12px;outline:none}.ve-select-sm{padding:5px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}.ve-input{flex:1;padding:6px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:12px;outline:none}.ve-input-sm{width:80px;padding:5px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}.ve-conditions{display:flex;flex-direction:column;gap:4px;max-height:120px;overflow-y:auto}.ve-cond-row{display:flex;align-items:center;gap:4px}.ve-sel-sm{padding:3px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none;min-width:80px}.ve-input-sm-wide{flex:1;padding:3px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none;min-width:80px}.ve-del-btn{padding:2px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--color-danger);cursor:pointer;font-size:10px}.ve-add-btn{padding:3px 8px;border-radius:var(--radius-sm);border:1px dashed var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:11px;margin-top:4px}.ve-add-btn:hover{border-color:var(--color-primary);color:var(--color-primary)}.ve-preview{background:rgba(0,0,0,0.3);border-radius:var(--radius-sm);padding:10px}.ve-preview-label{font-size:11px;color:var(--text-muted);margin-bottom:4px}.ve-sql-out{margin:0;padding:8px;background:rgba(0,0,0,0.4);color:#10b981;font-size:12px;font-family:monospace;border-radius:4px;white-space:pre-wrap;word-break:break-all;max-height:100px;overflow-y:auto}.ve-footer{display:flex;gap:6px;padding-top:8px;border-top:1px solid var(--border-color)}
/* Rule Chain Editor */
.rc-panel{width:600px}.rc-body{padding:12px;max-height:440px;overflow-y:auto;display:flex;flex-direction:column;gap:8px}.rc-rule-list{display:flex;flex-direction:column;gap:6px}.rc-rule{background:rgba(255,255,255,0.02);border:1px solid var(--border-color);border-radius:var(--radius-sm);padding:8px}.rc-rule-disabled{opacity:0.5}.rc-rule-header{display:flex;align-items:center;gap:6px;flex-wrap:wrap}.rc-rule-type{padding:2px 6px;border-radius:3px;font-size:10px;font-weight:700;background:rgba(59,130,246,0.2);color:var(--color-primary)}.rc-rule-field{color:var(--text-primary);font-family:monospace;font-size:11px;min-width:80px}.rc-rule-op{color:var(--color-primary);font-size:11px;font-weight:600}.rc-rule-val{flex:1;padding:3px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}.rc-rule-del{padding:2px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--color-danger);cursor:pointer;font-size:10px}.rc-rule-children{margin-left:16px;padding-left:8px;border-left:2px solid var(--border-color);display:flex;flex-direction:column;gap:4px;margin-top:4px}.rc-rule-inner{display:flex;align-items:center;gap:6px;font-size:11px;color:var(--text-muted)}.rc-btn-sm{padding:2px 6px;border-radius:3px;border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:10px}.rc-btn-sm:hover{border-color:var(--color-primary);color:var(--color-primary)}.rc-add-main{padding:6px 12px;border-radius:var(--radius-sm);border:1px dashed var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:12px;width:100%}.rc-add-main:hover{border-color:var(--color-primary);color:var(--color-primary)}.rc-preview{background:rgba(0,0,0,0.3);border-radius:var(--radius-sm);padding:10px}.rc-preview-label{font-size:11px;color:var(--text-muted);margin-bottom:4px}.rc-sql-out{margin:0;padding:8px;background:rgba(0,0,0,0.4);color:#10b981;font-size:11px;font-family:monospace;border-radius:4px;white-space:pre-wrap;word-break:break-all;max-height:80px;overflow-y:auto}.rc-footer{display:flex;gap:6px;padding-top:8px;border-top:1px solid var(--border-color)}
/* Field Drag Config */
.fd-panel{width:680px}.fd-body{padding:12px;max-height:460px;overflow-y:auto}.fd-layout{display:grid;grid-template-columns:1fr 1fr;gap:12px}.fd-col-title{font-size:12px;font-weight:600;color:var(--color-primary);margin-bottom:6px;display:flex;align-items:center;gap:6px}.fd-count{font-size:10px;color:var(--text-muted);font-weight:400}.fd-search{width:100%;padding:5px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none;margin-bottom:6px;box-sizing:border-box}.fd-available{max-height:180px;overflow-y:auto;display:flex;flex-direction:column;gap:2px}.fd-item{padding:4px 8px;border-radius:var(--radius-sm);font-size:11px;font-family:monospace;cursor:pointer;color:var(--text-primary);border:1px solid transparent}.fd-item:hover{border-color:var(--color-primary);background:rgba(59,130,246,0.1)}.fd-item.fd-used{background:rgba(16,185,129,0.1);border-color:rgba(16,185,129,0.3);color:#10b981}.fd-selected-list{display:flex;flex-direction:column;gap:2px;max-height:150px;overflow-y:auto}.fd-selected-item{display:flex;align-items:center;gap:6px;padding:4px 8px;background:rgba(16,185,129,0.08);border:1px solid rgba(16,185,129,0.2);border-radius:var(--radius-sm);font-size:11px;font-family:monospace;color:#10b981}.fd-selected-item.fd-wf{background:rgba(245,158,11,0.08);border-color:rgba(245,158,11,0.2);color:#f59e0b}.fd-remove{padding:1px 4px;border-radius:3px;border:none;background:transparent;color:inherit;cursor:pointer;font-size:10px;opacity:0.7}.fd-remove:hover{opacity:1}.fd-empty-hint{font-size:11px;color:var(--text-muted);text-align:center;padding:12px}.fd-sql-preview{margin-top:12px;background:rgba(0,0,0,0.3);border-radius:var(--radius-sm);padding:10px}.fd-preview-label{font-size:11px;color:var(--text-muted);margin-bottom:4px}.fd-sql-out{margin:0;padding:8px;background:rgba(0,0,0,0.4);color:#10b981;font-size:11px;font-family:monospace;border-radius:4px;white-space:pre-wrap;word-break:break-all;max-height:80px;overflow-y:auto}.fd-footer{display:flex;gap:6px;padding-top:8px;border-top:1px solid var(--border-color);margin-top:8px}
/* Chart Linkage */
.cl-panel{width:640px}.cl-body{padding:12px;max-height:480px;overflow-y:auto}.cl-config{display:flex;flex-direction:column;gap:10px}.cl-row{display:flex;gap:8px;flex-wrap:wrap}.cl-group{display:flex;flex-direction:column;gap:4px;flex:1;min-width:120px}.cl-group label{font-size:11px;color:var(--text-muted);font-weight:600}.cl-select{padding:5px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}.cl-chart-area{display:flex;align-items:flex-end;gap:4px;height:140px;padding:12px;background:rgba(255,255,255,0.02);border-radius:var(--radius-sm);border:1px solid var(--border-color)}.cl-chart-bar{display:flex;flex-direction:column;align-items:center;justify-content:flex-end;flex:1;border-radius:3px 3px 0 0;cursor:pointer;transition:opacity 0.15s;min-width:20px;padding:2px}.cl-chart-bar:hover{opacity:0.8}.cl-bar-label{font-size:8px;color:var(--text-muted);position:absolute;bottom:-16px;white-space:nowrap;max-width:60px;overflow:hidden;text-overflow:ellipsis}.cl-bar-val{font-size:9px;color:var(--text-primary);margin-bottom:2px}.cl-chart-empty{color:var(--text-muted);font-size:12px;text-align:center;width:100%;align-self:center}.cl-filters{display:flex;align-items:center;gap:6px;flex-wrap:wrap;padding:8px;background:rgba(59,130,246,0.05);border:1px solid rgba(59,130,246,0.2);border-radius:var(--radius-sm)}.cl-filter-tag{display:flex;align-items:center;gap:4px;padding:2px 8px;background:rgba(59,130,246,0.15);border:1px solid rgba(59,130,246,0.3);border-radius:10px;font-size:10px;color:var(--color-primary);font-family:monospace}.cl-filter-tag button{background:none;border:none;color:var(--color-primary);cursor:pointer;font-size:10px}.cl-clear-all{padding:2px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:10px}.cl-footer{display:flex;gap:6px;padding-top:8px;border-top:1px solid var(--border-color);margin-top:8px}
/* Advanced Templates */
.at-panel{width:720px}.at-tabs{display:flex;gap:4px;padding:8px 12px;border-bottom:1px solid var(--border-color)}.at-tab{padding:4px 12px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:11px}.at-tab.active{background:var(--color-primary);color:#000;border-color:var(--color-primary)}.at-grid{padding:12px;max-height:400px;overflow-y:auto;display:flex;flex-direction:column;gap:8px}.at-card{background:rgba(255,255,255,0.02);border:1px solid var(--border-color);border-radius:var(--radius-md);overflow:hidden}.at-card-head{display:flex;align-items:center;gap:6px;padding:8px 12px;background:rgba(59,130,246,0.08);border-bottom:1px solid var(--border-color)}.at-icon{font-size:16px}.at-name{flex:1;color:var(--text-primary);font-size:13px;font-weight:500}.at-cat-tag{font-size:10px;padding:1px 8px;border-radius:10px;background:rgba(245,158,11,0.15);color:#f59e0b}.at-code{margin:0;padding:8px 12px;background:rgba(0,0,0,0.3);color:#10b981;font-size:11px;font-family:monospace;white-space:pre-wrap;word-break:break-all;max-height:80px;overflow-y:auto}.at-card-foot{display:flex;gap:6px;padding:8px 12px;border-top:1px solid var(--border-color)}.at-empty{color:var(--text-muted);font-size:12px;text-align:center;padding:32px}
.btn-xs{padding:2px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:10px}.btn-danger{border-color:var(--color-danger);color:var(--color-danger)}.btn-danger:hover{background:rgba(239,68,68,0.1)}
'''

# Insert before </style>
for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</style>':
        lines.insert(i, new_styles)
        break

# ── Step 7: Fix insertField to use proper newline ────────────────────────
for i, line in enumerate(lines):
    if 'function insertField(name: string)' in line:
        lines[i] = line.replace('\\n', '" + "\\n" + "')

# ── Write back ────────────────────────────────────────────────────────────
new_content = '\n'.join(lines)
with open(path, 'w', encoding='utf-8') as f:
    f.write(new_content)

print(f"Done. New line count: {len(lines)}")
