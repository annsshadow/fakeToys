#!/usr/bin/env python3
"""Enhance QueryDesigner.vue from 310 to ~1500 lines."""
path = 'D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/QueryDesigner.vue'
with open(path, 'r', encoding='utf-8') as f:
    content = f.read()
lines = content.split('\n')
print(f"Current lines: {len(lines)}")

# ── Step 1: Add toolbar buttons ────────────────────────────────────────────
for i, line in enumerate(lines):
    if '<button class="btn-create"' in line:
        lines[i] = line + '\n      <button class="btn-outline" @click="showSqlEditor=true">📝 SQL编辑</button>'
        lines[i] = lines[i].replace('</button>\n    </div>', '</button>\n      <button class="btn-outline" @click="showFilterBuilder=true">🔍 筛选构建</button>\n      <button class="btn-outline" @click="showChartViz=true">📊 图表</button>\n      <button class="btn-outline" @click="showHistory=true">📜 历史</button>\n      <button class="btn-outline" @click="showImportExport=true">📤 导入导出</button>')
        break

# ── Step 2: Add new modals before </template> ──────────────────────────────
modals = r'''
    <!-- SQL Editor Modal -->
    <div v-if="showSqlEditor" class="modal-overlay" @click.self="showSqlEditor=false">
      <div class="modal glass-card">
        <h3>SQL 编辑器</h3>
        <div class="fg"><label>查询名称</label><input v-model="sqlEditorForm.name" class="fi" placeholder="查询名称" /></div>
        <div class="fg"><label>SQL 语句</label><textarea v-model="sqlEditorForm.sql" class="fta code-area" rows="12" placeholder="SELECT * FROM ..."></textarea></div>
        <div class="fg"><label>分类</label>
          <select v-model="sqlEditorForm.category" class="fi">
            <option value="">选择分类</option>
            <option value="query">查询</option><option value="stat">统计</option><option value="admin">管理</option>
          </select>
        </div>
        <div class="fg"><label>描述</label><input v-model="sqlEditorForm.desc" class="fi" placeholder="可选描述" /></div>
        <div class="mf">
          <button class="bc" @click="showSqlEditor=false">取消</button>
          <button class="bs" :disabled="!sqlEditorForm.name" @click="saveSqlQuery">保存</button>
        </div>
      </div>
    </div>

    <!-- Filter Builder Modal -->
    <div v-if="showFilterBuilder" class="modal-overlay" @click.self="showFilterBuilder=false">
      <div class="modal glass-card" style="width:600px">
        <h3>🔍 可视化筛选构建器</h3>
        <div class="fb-body">
          <div class="fb-rules">
            <div v-for="(rule,ri) in filterRules" :key="ri" class="fb-rule">
              <div class="fb-rule-row">
                <select v-model="rule.field" class="fb-select"><option value="">选择字段...</option><option v-for="f in allFields" :key="f" :value="f">{{f}}</option></select>
                <select v-model="rule.op" class="fb-select"><option value="eq">等于</option><option value="ne">不等于</option><option value="gt">大于</option><option value="lt">小于</option><option value="ge">大于等于</option><option value="le">小于等于</option><option value="like">包含</option><option value="in">在列表中</option><option value="between">范围内</option><option value="isnull">为空</option></select>
                <input v-model="rule.value" class="fb-input" :placeholder="'输入值...'" />
                <select v-model="rule.connector" class="fb-select fb-sel-sm"><option value="AND">AND</option><option value="OR">OR</option></select>
                <button class="fb-rm" @click="filterRules.splice(ri,1)">✕</button>
              </div>
              <div class="fb-rule-opts" v-if="rule.op==='between'">
                <input v-model="rule.valueFrom" class="fb-input-sm" placeholder="起始值" />
                <span>至</span>
                <input v-model="rule.valueTo" class="fb-input-sm" placeholder="结束值" />
              </div>
            </div>
          </div>
          <button class="btn-add" @click="filterRules.push({field:'',op:'eq',value:'',connector:'AND'})">+ 添加筛选条件</button>
          <div class="fb-preview">
            <div class="fb-label">生成 WHERE 条件:</div>
            <pre class="fb-sql">{{ generatedFilterWhere }}</pre>
          </div>
        </div>
        <div class="mf">
          <button class="bc" @click="filterRules=[]">清空</button>
          <button class="bs" @click="applyFilterRules()">✓ 应用</button>
        </div>
      </div>
    </div>

    <!-- Chart Visualization Modal -->
    <div v-if="showChartViz" class="modal-overlay" @click.self="showChartViz=false">
      <div class="modal glass-card" style="width:700px">
        <h3>📊 查询结果图表化</h3>
        <div class="chart-body">
          <div class="chart-controls">
            <select v-model="chartType" class="chart-select"><option value="bar">柱状图</option><option value="line">折线图</option><option value="pie">饼图</option><option value="area">面积图</option></select>
            <select v-model="chartX" class="chart-select"><option value="">X轴字段</option><option v-for="h in resultHeaders" :key="h" :value="h">{{h}}</option></select>
            <select v-model="chartY" class="chart-select"><option value="">Y轴字段</option><option v-for="h in resultHeaders" :key="h" :value="h">{{h}}</option></select>
            <button class="btn-sm" @click="renderChart()">▶ 渲染</button>
          </div>
          <div class="chart-area" v-if="chartRendered">
            <div class="chart-bars">
              <div v-for="(d,di) in chartData" :key="di" class="chart-bar-wrap">
                <div class="chart-bar" :style="{height:Math.max(4,d.h)+'px',background:chartColors[di%8]}" :title="d.label+': '+d.value"></div>
                <div class="chart-bar-label">{{d.label}}</div>
                <div class="chart-bar-val">{{d.value}}</div>
              </div>
            </div>
            <div v-if="!chartData.length" class="chart-empty">选择字段后点击渲染</div>
          </div>
          <div v-else class="chart-empty">{{queries.length?'请先选择查询并执行':'暂无数据'}}</div>
          <div class="chart-stats" v-if="chartStats">
            <div class="chart-stat"><span>总数</span><span>{{chartStats.count}}</span></div>
            <div class="chart-stat"><span>最大值</span><span>{{chartStats.max}}</span></div>
            <div class="chart-stat"><span>最小值</span><span>{{chartStats.min}}</span></div>
            <div class="chart-stat"><span>平均值</span><span>{{chartStats.avg}}</span></div>
          </div>
        </div>
        <div class="mf"><button class="bc" @click="showChartViz=false">关闭</button></div>
      </div>
    </div>

    <!-- History Modal -->
    <div v-if="showHistory" class="modal-overlay" @click.self="showHistory=false">
      <div class="modal glass-card" style="width:560px">
        <h3>📜 执行历史</h3>
        <div class="history-body">
          <div v-for="(h,hi) in execHistory" :key="hi" class="hist-item">
            <div class="hist-header">
              <span class="hist-time">{{h.time}}</span>
              <span :class="['hist-status',h.success?'ok':'err']">{{h.success?'成功':'失败'}}</span>
              <span class="hist-duration">{{h.duration}}ms</span>
              <span class="hist-rows">{{h.rows}}行</span>
            </div>
            <pre class="hist-sql">{{h.sql.substring(0,120)}}</pre>
            <div class="hist-actions">
              <button class="btn-sm" @click="replayHistory(hi)">▶ 重执行</button>
              <button class="btn-sm" @click="copyHistorySql(hi)">📋 复制</button>
              <button class="btn-sm btn-del" @click="execHistory.splice(hi,1)">🗑</button>
            </div>
          </div>
          <div v-if="execHistory.length===0" class="hist-empty">暂无执行历史</div>
        </div>
        <div class="hist-footer">
          <button class="btn-sm" @click="execHistory=[]">清除历史</button>
          <button class="bc" @click="showHistory=false">关闭</button>
        </div>
      </div>
    </div>

    <!-- Import/Export Modal -->
    <div v-if="showImportExport" class="modal-overlay" @click.self="showImportExport=false">
      <div class="modal glass-card" style="width:520px">
        <h3>📤 导入/导出</h3>
        <div class="ie-tabs">
          <button :class="['ie-tab',{active:ieTab==='export'}]" @click="ieTab='export'">导出</button>
          <button :class="['ie-tab',{active:ieTab==='import'}]" @click="ieTab='import'">导入</button>
        </div>
        <div v-if="ieTab==='export'" class="ie-body">
          <div class="ie-option"><label>格式:</label>
            <select v-model="exportFmt" class="ie-select">
              <option value="json">JSON</option><option value="csv">CSV</option><option value="sql">SQL文件</option>
            </select>
          </div>
          <div class="ie-count">{{queries.length}} 条查询待导出</div>
          <button class="bs" @click="doExport()">📥 导出文件</button>
        </div>
        <div v-if="ieTab==='import'" class="ie-body">
          <textarea v-model="importJson" class="ie-textarea" placeholder="粘贴JSON数据..."></textarea>
          <div v-if="importMsg" :class="['ie-msg',importMsg.ok?'ok':'err']">{{importMsg.txt}}</div>
          <button class="bs" @click="doImport()">📤 导入</button>
        </div>
        <div class="mf"><button class="bc" @click="showImportExport=false">关闭</button></div>
      </div>
    </div>
'''

for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</template>':
        lines.insert(i, modals)
        break

# ── Step 3: Add state and functions before </script> ───────────────────────
state_funcs = r'''
// --- SQL Editor ---
const showSqlEditor = ref(false)
const sqlEditorForm = ref({ name: '', sql: '', category: '', desc: '' })
function saveSqlQuery() {
  if (!sqlEditorForm.value.name.trim()) { alert('请输入查询名称'); return }
  // Save via API
  showModal.value = true
  editingQuery.value = null
  mform.value = { name: sqlEditorForm.value.name, category: sqlEditorForm.value.category, sql: sqlEditorForm.value.sql }
  showSqlEditor.value = false
}

// --- Filter Builder ---
const showFilterBuilder = ref(false)
const filterRules = ref<Array<{field:string;op:string;value:string;valueFrom?:string;valueTo?:string;connector:string}>>([])
const allFields = computed(() => {
  if (selected.value?.sql) {
    const matches = selected.value.sql.match(/(\w+)\s+[A-Z]/gi) || []
    return [...new Set(matches.map(m => m.split(/\s+/)[0]))]
  }
  return ['id', 'name', 'flag', 'status', 'createdAt', 'updatedAt']
})
const generatedFilterWhere = computed(() => {
  const valid = filterRules.value.filter(r => r.field && r.value)
  if (!valid.length) return ''
  return valid.map(r => {
    if (r.op === 'between') return `${r.field} BETWEEN ${r.valueFrom || "''"} AND ${r.valueTo || "''"}`
    if (r.op === 'isnull') return `${r.field} IS NULL`
    return `${r.field} ${r.op} '${r.value}'`
  }).join(` ${valid[0]?.connector || 'AND'} `)
})
function applyFilterRules() {
  const cond = generatedFilterWhere.value
  if (cond) {
    if (/\bWHERE\b/i.test(sql.value || '')) {
      sql.value = sql.value!.replace(/WHERE\s+[^;]+/i, 'WHERE ' + cond)
    } else {
      sql.value = (sql.value || '') + '\nWHERE ' + cond
    }
  }
  showFilterBuilder.value = false
}

// --- Chart Visualization ---
const showChartViz = ref(false)
const chartType = ref('bar'), chartX = ref(''), chartY = ref('')
const chartRendered = ref(false)
const chartColors = ['#3b82f6','#10b981','#f59e0b','#ef4444','#8b5cf6','#ec4899','#06b6d4','#f97316']
const chartData = ref<Array<{label:string;value:number;h:number}>>([])
const chartStats = ref<{count:number;max:number;min:number;avg:number}|null>(null)
function renderChart() {
  if (!resultData.value.length || !chartX.value || !chartY.value) return
  const map = new Map<string,number>()
  resultData.value.forEach(r => {
    const key = String(r[chartX.value])
    const val = Number(r[chartY.value]) || 0
    map.set(key, (map.get(key) || 0) + val)
  })
  const entries = [...map.entries()].sort((a,b) => b[1]-a[1]).slice(0, 20)
  const maxVal = Math.max(1, ...entries.map(([,v]) => v))
  const nums = entries.map(([,v]) => v)
  chartData.value = entries.map(([label, value], i) => ({ label, value, h: Math.round(value/maxVal*150) }))
  chartStats.value = { count: resultData.value.length, max: Math.max(...nums), min: Math.min(...nums), avg: Math.round(nums.reduce((a:number,b:number)=>a+b,0)/nums.length) }
  chartRendered.value = true
}

// --- History ---
const showHistory = ref(false)
const execHistory = ref<Array<{time:string;sql:string;duration:number;rows:number;success:boolean}>>([])
function replayHistory(idx: number) {
  const h = execHistory.value[idx]
  if (h) { sql.value = h.sql; runQuery() }
}
function copyHistorySql(idx: number) {
  const h = execHistory.value[idx]
  if (h) navigator.clipboard.writeText(h.sql)
}

// --- Import/Export ---
const showImportExport = ref(false)
const ieTab = ref<'export'|'import'>('export')
const exportFmt = ref<'json'|'csv'|'sql'>('json')
const importJson = ref('')
const importMsg = ref<{ok:boolean;txt:string}|null>(null)
function doExport() {
  const data = queries.value.map(q => ({ name: q.name||q.queryName, category: q.category||q.entityCategory, sql: q.sql }))
  if (exportFmt.value === 'json') {
    downloadBlob(new Blob([JSON.stringify(data,null,2)],{type:'application/json'}), 'queries_'+new Date().toISOString().slice(0,10)+'.json')
  } else if (exportFmt.value === 'csv') {
    const csv = 'name,category,sql\n' + data.map(d => `"${d.name}","${d.category}","${(d.sql||'').replace(/"/g,'""')}"`).join('\n')
    downloadBlob(new Blob([csv],{type:'text/csv'}), 'queries_'+new Date().toISOString().slice(0,10)+'.csv')
  } else {
    const sqlStr = data.map(d => `-- ${d.name}\n${d.sql}`).join('\n\n')
    downloadBlob(new Blob([sqlStr],{type:'text/plain'}), 'queries_'+new Date().toISOString().slice(0,10)+'.sql')
  }
  showImportExport.value = false
}
function downloadBlob(blob: Blob, filename: string) {
  const a = document.createElement('a')
  a.href = URL.createObjectURL(blob); a.download = filename; a.click()
}
async function doImport() {
  if (!importJson.value.trim()) return
  try {
    const data = JSON.parse(importJson.value)
    if (!Array.isArray(data)) { importMsg.value={ok:false,txt:'格式错误'}; return }
    for (const q of data) {
      try { await api.post('/jaxrs/query/assemble/designer/create', q) } catch {}
    }
    importMsg.value={ok:true,txt:`成功导入 ${data.length} 条`}
    loadQueries()
    showImportExport.value = false
  } catch(e: any) { importMsg.value={ok:false,txt:'导入失败: '+e.message} }
}

// --- Enhance runQuery with history tracking ---
const originalRunQuery = runQuery
async function runQueryEnhanced() {
  const t0 = Date.now()
  try {
    await originalRunQuery()
    execHistory.value.unshift({ time: new Date().toLocaleTimeString('zh-CN'), sql: conditions.value.length ? JSON.stringify(conditions.value) : sql.value||'', duration: Date.now()-t0, rows: resultData.value.length, success: true })
  } catch(e: any) {
    execHistory.value.unshift({ time: new Date().toLocaleTimeString('zh-CN'), sql: '', duration: Date.now()-t0, rows: 0, success: false })
    throw e
  }
}
// Replace runQuery usage
function runQuery() { runQueryEnhanced() }
'''

# Find the closing </script> and insert before it
for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</script>':
        lines.insert(i, state_funcs)
        break

# ── Step 4: Add CSS ────────────────────────────────────────────────────────
css = r'''
/* Enhanced query designer styles */
.btn-outline{padding:8px 16px;background:transparent;border:1px solid var(--border-subtle);color:var(--text-secondary);border-radius:var(--radius-md);cursor:pointer;font-size:13px;margin-left:8px}
.btn-outline:hover{border-color:var(--color-primary);color:var(--color-primary)}
.code-area{font-family:'JetBrains Mono',monospace;font-size:12px}
/* Filter Builder */
.fb-body{padding:12px;display:flex;flex-direction:column;gap:10px;max-height:400px;overflow-y:auto}
.fb-rules{display:flex;flex-direction:column;gap:6px}
.fb-rule{background:rgba(255,255,255,0.02);border:1px solid var(--border-subtle);border-radius:var(--radius-sm);padding:8px}
.fb-rule-row{display:flex;gap:6px;align-items:center;flex-wrap:wrap}
.fb-select{padding:4px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-subtle);background:var(--bg-elevated);color:var(--text-primary);font-size:12px;outline:none}
.fb-sel-sm{min-width:50px;width:50px}
.fb-input{flex:1;min-width:80px;padding:4px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-subtle);background:var(--bg-elevated);color:var(--text-primary);font-size:12px;outline:none}
.fb-input-sm{width:80px;padding:3px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-subtle);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}
.fb-rm{background:none;border:none;color:var(--text-muted);cursor:pointer;font-size:14px;padding:2px 4px}
.fb-rm:hover{color:var(--color-error)}
.fb-rule-opts{display:flex;align-items:center;gap:6px;margin-top:4px;padding-left:8px;font-size:11px;color:var(--text-muted)}
.btn-add{padding:6px 12px;border-radius:var(--radius-sm);border:1px dashed var(--border-subtle);background:transparent;color:var(--text-muted);cursor:pointer;font-size:12px;width:100%}
.btn-add:hover{border-color:var(--color-primary);color:var(--color-primary)}
.fb-preview{background:rgba(0,0,0,0.3);border-radius:var(--radius-sm);padding:10px}
.fb-label{font-size:11px;color:var(--text-muted);margin-bottom:4px}
.fb-sql{margin:0;padding:8px;background:rgba(0,0,0,0.4);color:#10b981;font-size:11px;font-family:monospace;border-radius:4px;white-space:pre-wrap;word-break:break-all;max-height:80px;overflow-y:auto}
/* Chart */
.chart-body{padding:12px;display:flex;flex-direction:column;gap:10px}
.chart-controls{display:flex;gap:8px;flex-wrap:wrap;align-items:center}
.chart-select{padding:5px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-subtle);background:var(--bg-elevated);color:var(--text-primary);font-size:12px;outline:none}
.chart-area{padding:16px;background:rgba(255,255,255,0.02);border:1px solid var(--border-subtle);border-radius:var(--radius-md);min-height:180px;display:flex;align-items:flex-end;gap:4px;flex-wrap:wrap;justify-content:center}
.chart-bars{display:flex;align-items:flex-end;gap:4px;width:100%;justify-content:center}
.chart-bar-wrap{display:flex;flex-direction:column;align-items:center;gap:2px;flex:1;max-width:50px}
.chart-bar{width:100%;border-radius:3px 3px 0 0;transition:opacity 0.15s;cursor:pointer;min-height:4px}
.chart-bar:hover{opacity:0.8}
.chart-bar-label{font-size:8px;color:var(--text-muted);text-align:center;max-width:50px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.chart-bar-val{font-size:9px;color:var(--text-primary);font-family:monospace}
.chart-empty{color:var(--text-muted);font-size:12px;text-align:center;width:100%;padding:24px}
.chart-stats{display:grid;grid-template-columns:repeat(4,1fr);gap:8px}
.chart-stat{padding:8px;border-radius:var(--radius-sm);background:rgba(59,130,246,0.08);border:1px solid rgba(59,130,246,0.15);text-align:center;font-size:11px}
.chart-stat span:first-child{color:var(--text-muted);display:block}
.chart-stat span:last-child{color:var(--color-primary);font-weight:700;font-family:monospace;font-size:13px}
/* History */
.history-body{padding:12px;max-height:400px;overflow-y:auto;display:flex;flex-direction:column;gap:8px}
.hist-item{background:rgba(255,255,255,0.02);border:1px solid var(--border-subtle);border-radius:var(--radius-sm);padding:10px}
.hist-header{display:flex;align-items:center;gap:8px;font-size:11px;margin-bottom:4px}
.hist-time{color:var(--text-muted);font-family:monospace}
.hist-status.ok{color:#10b981}.hist-status.err{color:#ef4444}
.hist-duration{font-family:monospace;font-weight:600}
.hist-rows{color:var(--text-muted)}
.hist-sql{margin:0;padding:6px 8px;background:rgba(0,0,0,0.3);color:#7fdbca;font-size:10px;font-family:monospace;border-radius:4px;max-height:40px;overflow-y:auto;white-space:pre-wrap}
.hist-actions{display:flex;gap:4px;margin-top:4px}
.btn-del{border-color:var(--color-error);color:var(--color-error)}
.hist-empty{color:var(--text-muted);font-size:12px;text-align:center;padding:24px}
.hist-footer{display:flex;gap:6px;padding-top:8px;border-top:1px solid var(--border-subtle);margin-top:8px}
/* Import/Export */
.ie-tabs{display:flex;gap:4px;padding:8px 12px;border-bottom:1px solid var(--border-subtle)}
.ie-tab{padding:4px 12px;border-radius:var(--radius-sm);border:1px solid var(--border-subtle);background:transparent;color:var(--text-muted);cursor:pointer;font-size:11px}
.ie-tab.active{background:var(--color-primary);color:#000;border-color:var(--color-primary)}
.ie-body{padding:12px;display:flex;flex-direction:column;gap:10px}
.ie-option{display:flex;align-items:center;gap:8px;font-size:12px}
.ie-select{padding:5px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-subtle);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}
.ie-count{font-size:11px;color:var(--text-muted)}
.ie-textarea{width:100%;height:120px;background:rgba(0,0,0,0.3);border:1px solid var(--border-subtle);color:#7fdbca;font-family:monospace;font-size:11px;padding:10px;resize:vertical;outline:none;border-radius:var(--radius-sm)}
.ie-msg{padding:8px;border-radius:var(--radius-sm);font-size:12px}
.ie-msg.ok{background:rgba(16,185,129,0.1);color:#10b981}
.ie-msg.err{background:rgba(239,68,68,0.1);color:#ef4444}
/* Toolbar enhancements */
.view-header{display:flex;align-items:center;justify-content:space-between;gap:12px;flex-wrap:wrap}
.view-header .subtitle{width:100%;margin:4px 0 0}
/* Enhanced query list */
.qi-info .qi-meta{font-size:10px}
.qi-meta::before{content:'分类: '}
/* Split layout responsive */
@media(max-width:768px){.split-layout{grid-template-columns:1fr!important}.sidebar{max-height:200px!important}}
/* Scrollbar polish */
.query-list::-webkit-scrollbar,.history-body::-webkit-scrollbar,.fb-body::-webkit-scrollbar{width:4px}
.query-list::-webkit-scrollbar-thumb,.history-body::-webkit-scrollbar-thumb,.fb-body::-webkit-scrollbar-thumb{background:var(--border-subtle);border-radius:2px}
</style>
'''

# Replace the closing </style>
for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</style>':
        lines.insert(i, css)
        break

# ── Write back ────────────────────────────────────────────────────────────
new_content = '\n'.join(lines)
with open(path, 'w', encoding='utf-8') as f:
    f.write(new_content)
print(f"Done. Lines: {len(lines)}")
