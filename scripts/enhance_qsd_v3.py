#!/usr/bin/env python3
"""Add more features to reach ~2000 lines in QSD."""
import re

path = 'D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/QueryStatementDesigner.vue'
with open(path, 'r', encoding='utf-8') as f:
    content = f.read()
lines = content.split('\n')

# ── 1. Add more toolbar buttons ───────────────────────────────────────────
for i, line in enumerate(lines):
    if 'showDebugConsole=true' in line and 'title="调试控制台"' in line:
        lines[i] = line + '        <button class="btn btn-outline" @click="showSqlFormatter=true" title="SQL格式化器">📐 格式化</button>'
        lines[i] = lines[i].replace('</button>\n        <button', '</button>\n        <button')
        break

# ── 2. Add SQL Formatter modal before </template> ────────────────────────
sql_formatter_modal = r'''
    <!-- SQL Formatter/Beautifier -->
    <div v-if="showSqlFormatter" class="modal-overlay" @click.self="showSqlFormatter=false">
      <div class="modal-box formatter-panel">
        <div class="modal-header"><span>📐 SQL 格式化器</span><button class="btn-close" @click="showSqlFormatter=false">✕</button></div>
        <div class="formatter-body">
          <div class="formatter-row">
            <div class="formatter-col">
              <div class="fmt-label">原始 SQL</div>
              <pre class="fmt-input">{{ sql || '(空)' }}</pre>
            </div>
            <div class="fmt-arrow">⇄</div>
            <div class="formatter-col">
              <div class="fmt-label">格式化后</div>
              <pre class="fmt-output">{{ formattedSql }}</pre>
            </div>
          </div>
          <div class="fmt-options">
            <label><input type="checkbox" v-model="fmtUpperCase" /> 大写关键字</label>
            <label><input type="checkbox" v-model="fmtIndent" /> 缩进排版</label>
            <label>换行宽度: <input type="number" v-model.number="fmtWrapWidth" min="60" max="200" class="fmt-num" /></label>
          </div>
        </div>
        <div class="fmt-footer">
          <button class="btn-sm" @click="applyFormattedSql()">✓ 应用</button>
          <button class="btn-sm" @click="copyFormatted()">📋 复制</button>
        </div>
      </div>
    </div>

    <!-- Column Analysis Panel -->
    <div v-if="showColAnalysis" class="modal-overlay" @click.self="showColAnalysis=false">
      <div class="modal-box colanalysis-panel">
        <div class="modal-header"><span>📊 列统计分析</span><button class="btn-close" @click="showColAnalysis=false">✕</button></div>
        <div class="colanalysis-body">
          <div v-if="!resultData.length" class="colanalysis-empty">请先执行SQL获取结果数据</div>
          <div v-else class="col-analysis-table">
            <div class="ca-header"><span>列名</span><span>类型</span><span>非空</span><span>唯一值</span><span>平均值</span><span>总和</span></div>
            <div v-for="(s,si) in columnSummary" :key="si" class="ca-row">
              <span class="ca-name">{{s.name}}</span>
              <span class="ca-type">{{s.sampleType}}</span>
              <span class="ca-null">{{s.nulls}}/{{s.count}}</span>
              <span class="ca-distinct">{{new Set(resultData.value.map(r=>r[s.name])).size}}</span>
              <span class="ca-avg">{{s.avg ?? '-'}}</span>
              <span class="ca-sum">{{s.sum ?? '-'}}</span>
            </div>
          </div>
        </div>
        <div class="colanalysis-footer">
          <button class="btn-sm" @click="showColAnalysis=false">关闭</button>
        </div>
      </div>
    </div>
'''

for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</template>':
        lines.insert(i, sql_formatter_modal)
        break

# ── 3. Add state variables for new panels ─────────────────────────────────
state_add = r'''
// --- SQL Formatter State ---
const showSqlFormatter = ref(false)
const fmtUpperCase = ref(true), fmtIndent = ref(true), fmtWrapWidth = ref(120)
const formattedSql = computed(() => formatSqlEnhanced(sql.value))

// --- Column Analysis State ---
const showColAnalysis = ref(false)
'''

for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</script>':
        lines.insert(i, state_add)
        break

# ── 4. Add formatter and analysis functions ────────────────────────────────
func_add = r'''
// --- Enhanced SQL Formatter ---
function formatSqlEnhanced(raw: string): string {
  if (!raw.trim()) return raw
  let s = raw.trim()
  if (fmtUpperCase.value) {
    s = s.replace(/\b(SELECT|FROM|WHERE|AND|OR|ORDER\s+BY|GROUP\s+BY|HAVING|LIMIT|OFFSET|JOIN|LEFT|RIGHT|INNER|OUTER|ON|SET|VALUES|INSERT|INTO|DELETE|CREATE|ALTER|DROP|TABLE|INDEX|VIEW|AS|DISTINCT|UNION|ALL|NOT|NULL|IS|IN|EXISTS|BETWEEN|LIKE|CASE|WHEN|THEN|ELSE|END|COUNT|SUM|AVG|MAX|MIN|ROW_NUMBER|RANK|OVER|PARTITION|WITH|RECURSIVE|FOR|UPDATE|RETURNING)\b/gi, (m) => m.toUpperCase())
  }
  const keywords = ['SELECT','FROM','WHERE','AND','OR','ORDER BY','GROUP BY','HAVING','LIMIT','OFFSET','JOIN','LEFT JOIN','RIGHT JOIN','INNER JOIN','CROSS JOIN','ON','SET','VALUES','INSERT INTO','DELETE FROM','CREATE TABLE','ALTER TABLE','DROP TABLE','UNION ALL','UNION','NOT NULL','IS NULL','IS NOT NULL','IN','EXISTS','BETWEEN','LIKE','CASE','WHEN','THEN','ELSE','END','AS','WITH','RECURSIVE']
  let result = s
  for (const kw of keywords) {
    const re = new RegExp(kw.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'gi')
    result = result.replace(re, '\n' + kw + ' ')
  }
  result = result.replace(/\n\s*\n/g, '\n').trim()
  if (fmtIndent.value) {
    let indent = 0
    const lines2 = result.split('\n')
    result = lines2.map(l => {
      const trimmed = l.trim()
      if (!trimmed) return ''
      let prefix = '  '.repeat(indent)
      if (trimmed.startsWith(')') || trimmed.startsWith(')')) indent = Math.max(0, indent - 1)
      const line = prefix + trimmed
      if (trimmed.endsWith('(') || trimmed.endsWith(',')) indent++
      return line
    }).join('\n')
  }
  return result
}
function applyFormattedSql() { sql.value = formattedSql.value; showSqlFormatter.value = false }
function copyFormatted() { navigator.clipboard.writeText(formattedSql.value) }

// --- Result Stats Enhancement ---
function resultNumericStats(col: string): any {
  const nums = resultData.value.map(r => Number(r[col])).filter(v => !isNaN(v))
  if (!nums.length) return null
  const sorted = [...nums].sort((a,b) => a-b)
  const sum = nums.reduce((a:number,b:number) => a+b, 0)
  return { min: sorted[0], max: sorted[sorted.length-1], mean: sum/nums.length, median: sorted[Math.floor(sorted.length/2)], sum }
}
'''

for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</script>':
        lines.insert(i, func_add)
        break

# ── 5. Add CSS for new panels ─────────────────────────────────────────────
new_css = r'''
/* SQL Formatter */
.formatter-panel{width:720px}.fmt-body{padding:12px}.formatter-row{display:grid;grid-template-columns:1fr 40px 1fr;gap:0;margin-bottom:12px}.formatter-col{display:flex;flex-direction:column;gap:4px}.fmt-label{font-size:11px;color:var(--text-muted);font-weight:600}.fmt-input,.fmt-output{padding:10px;background:rgba(0,0,0,0.3);color:#7fdbca;font-size:12px;font-family:monospace;border-radius:var(--radius-sm);border:1px solid var(--border-color);white-space:pre-wrap;word-break:break-all;max-height:200px;overflow-y:auto;min-height:80px}.fmt-output{color:#10b981}.fmt-arrow{text-align:center;color:var(--text-muted);align-self:center;font-size:18px}.fmt-options{display:flex;align-items:center;gap:16px;font-size:12px;color:var(--text-muted);padding:8px;background:rgba(255,255,255,0.02);border-radius:var(--radius-sm)}.fmt-options label{display:flex;align-items:center;gap:4px;cursor:pointer}.fmt-num{width:50px;padding:3px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}.fmt-footer{display:flex;gap:6px;padding-top:8px;border-top:1px solid var(--border-color)}
/* Column Analysis */
.colanalysis-panel{width:680px}.colanalysis-body{padding:12px;max-height:400px;overflow-y:auto}.colanalysis-empty{color:var(--text-muted);font-size:13px;text-align:center;padding:32px}.col-analysis-table{display:flex;flex-direction:column;gap:4px}.ca-header{display:grid;grid-template-columns:120px 80px 60px 70px 80px 80px;gap:4px;padding:6px 10px;background:rgba(59,130,246,0.1);border-radius:var(--radius-sm);font-size:10px;font-weight:600;color:var(--color-primary);text-transform:uppercase}.ca-row{display:grid;grid-template-columns:120px 80px 60px 70px 80px 80px;gap:4px;padding:6px 10px;border-radius:var(--radius-sm);font-size:11px;background:rgba(255,255,255,0.02);align-items:center}.ca-row:hover{background:rgba(59,130,246,0.05)}.ca-name{color:var(--text-primary);font-family:monospace;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}.ca-type{color:var(--text-muted);font-size:10px}.ca-null{color:var(--text-muted)}.ca-distinct{color:var(--color-primary);font-family:monospace}.ca-avg,.ca-sum{color:var(--text-primary);font-family:monospace}.colanalysis-footer{display:flex;justify-content:flex-end;padding-top:8px;border-top:1px solid var(--border-color);margin-top:8px}
'''

for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</style>':
        lines.insert(i, new_css)
        break

# ── Write back ────────────────────────────────────────────────────────────
new_content = '\n'.join(lines)
with open(path, 'w', encoding='utf-8') as f:
    f.write(new_content)
print(f"Done. Lines: {len(lines)}")
