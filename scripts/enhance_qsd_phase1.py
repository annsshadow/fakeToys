#!/usr/bin/env python3
"""Phase 1: Add debug console, SQL formatter, validator, chart viz panels to QSD."""
import re

path = 'D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/QueryStatementExporter.vue'
# Actually read the correct file
path = 'D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/QueryStatementDesigner.vue'
with open(path, 'r', encoding='utf-8') as f:
    content = f.read()
lines = content.split('\n')
print(f"Current lines: {len(lines)}")

# ── Step 1: Add toolbar buttons ────────────────────────────────────────────
button_additions = [
    '        <button class="btn btn-outline" @click="showDebugConsole=!showDebugConsole" title="调试控制台">🐛 调试</button>',
    '        <button class="btn btn-outline" @click="showSqlFormatter=true" title="SQL格式化器">📐 格式化器</button>',
    '        <button class="btn btn-outline" @click="showSqlValidator=true" title="SQL语法验证">✅ 验证</button>',
    '        <button class="btn btn-outline" @click="showResultViz=true" title="结果可视化">📊 可视化</button>',
    '        <button class="btn btn-outline" @click="showSnippetLibrary=true" title="SQL片段库">📝 片段</button>',
]
for i, line in enumerate(lines):
    if 'showFavoritePanel' in line and 'title="收藏语句"' in line:
        for j, btn in enumerate(button_additions):
            lines.insert(i + 1 + j, btn)
        break

# ── Step 2: Add modals before </template> ──────────────────────────────────
modals = r'''
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
'''

for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</template>':
        lines.insert(i, modals)
        break

# ── Step 3: Add state + functions before </script> ────────────────────────
state_funcs = r'''
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
'''

for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</script>':
        lines.insert(i, state_funcs)
        break

# ── Step 4: Add CSS before </style> ────────────────────────────────────────
css_add = r'''
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
'''

for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</style>':
        lines.insert(i, css_add)
        break

# ── Write back ────────────────────────────────────────────────────────────
new_content = '\n'.join(lines)
with open(path, 'w', encoding='utf-8') as f:
    f.write(new_content)
print(f"Done. Lines: {len(lines)}")
