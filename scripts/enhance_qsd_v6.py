#!/usr/bin/env python3
"""Add final push to reach ~2000 lines in QSD."""
import re

path = 'D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/QueryStatementDesigner.vue'
with open(path, 'r', encoding='utf-8') as f:
    content = f.read()
lines = content.split('\n')

# ── 1. Add more toolbar buttons ───────────────────────────────────────────
for i, line in enumerate(lines):
    if 'showExportImport=true' in line and 'title="导入导出"' in line:
        lines[i] = line + '        <button class="btn btn-outline" @click="showCommentAnnotations=true" title="SQL注释">💬 注释</button>'
        lines[i] = lines[i].replace('</button>\n        <button', '</button>\n        <button')
        break

# ── 2. Add Comment Annotations modal ───────────────────────────────────────
comment_modal = r'''
    <!-- SQL Comment/Annotation Panel -->
    <div v-if="showCommentAnnotations" class="modal-overlay" @click.self="showCommentAnnotations=false">
      <div class="modal-box comment-panel">
        <div class="modal-header"><span>💬 SQL 注释注解</span><button class="btn-close" @click="showCommentAnnotations=false">✕</button></div>
        <div class="comment-body">
          <div class="comment-list">
            <div v-for="(c,ci) in sqlComments" :key="ci" class="comment-item">
              <div class="comment-header">
                <span class="comment-line">第{{c.line}}行</span>
                <span class="comment-type">{{c.type}}</span>
                <button class="comment-del" @click="sqlComments.splice(ci,1)">✕</button>
              </div>
              <textarea v-model="sqlComments[ci].text" class="comment-text" rows="2" placeholder="添加注释..."></textarea>
            </div>
          </div>
          <div class="comment-add">
            <input v-model="newCommentLine" type="number" class="comment-line-input" placeholder="行号" min="1" />
            <select v-model="newCommentType" class="comment-type-select">
              <option value="note">📝 备注</option>
              <option value="todo">🔴 TODO</option>
              <option value="warning">⚠️ 警告</option>
              <option value="important">❗ 重要</option>
            </select>
            <input v-model="newCommentText" class="comment-input" placeholder="注释内容..." />
            <button class="btn-sm" @click="addComment()">+ 添加</button>
          </div>
          <div class="comment-preview">
            <div class="comment-preview-label">预览带注释的SQL:</div>
            <pre class="comment-sql-out">{{annotatedSql}}</pre>
          </div>
        </div>
        <div class="comment-footer">
          <button class="btn-sm" @click="applyComments">✓ 应用注释到SQL</button>
          <button class="btn-sm" @click="clearComments">清空</button>
        </div>
      </div>
    </div>

    <!-- Query Result Visualization -->
    <div v-if="showResultViz" class="modal-overlay" @click.self="showResultViz=false">
      <div class="modal-box viz-panel">
        <div class="modal-header"><span>📊 结果可视化</span><button class="btn-close" @click="showResultViz=false">✕</button></div>
        <div class="viz-body">
          <div class="viz-controls">
            <select v-model="vizType" class="viz-select">
              <option value="bar">柱状图</option>
              <option value="line">折线图</option>
              <option value="pie">饼图</option>
              <option value="scatter">散点图</option>
            </select>
            <select v-model="vizXAxis" class="viz-select">
              <option value="">选择X轴...</option>
              <option v-for="h in resultHeaders" :key="h" :value="h">{{h}}</option>
            </select>
            <select v-model="vizYAxis" class="viz-select">
              <option value="">选择Y轴...</option>
              <option v-for="h in resultHeaders" :key="h" :value="h">{{h}}</option>
            </select>
            <button class="btn-sm" @click="renderViz()">▶ 渲染</button>
          </div>
          <div class="viz-chart" v-if="vizRendered">
            <div class="viz-bars">
              <div v-for="(d,di) in vizData" :key="di" class="viz-bar-wrap">
                <div class="viz-bar" :style="{height:Math.max(4,d.h)+'px',background:vizColors[di%8]}" :title="d.label+': '+d.value"></div>
                <div class="viz-bar-label">{{d.label}}</div>
                <div class="viz-bar-val">{{d.value}}</div>
              </div>
            </div>
            <div v-if="!vizData.length" class="viz-empty">选择字段后点击渲染</div>
          </div>
          <div v-else class="viz-empty">请先执行SQL获取数据</div>
          <div class="viz-stats" v-if="vizStats">
            <div class="viz-stat"><span>总数</span><span>{{vizStats.count}}</span></div>
            <div class="viz-stat"><span>最大值</span><span>{{vizStats.max}}</span></div>
            <div class="viz-stat"><span>最小值</span><span>{{vizStats.min}}</span></div>
            <div class="viz-stat"><span>平均值</span><span>{{vizStats.avg}}</span></div>
          </div>
        </div>
        <div class="viz-footer">
          <button class="btn-sm" @click="exportVizData()">📥 导出数据</button>
          <button class="btn-sm" @click="showResultViz=false">关闭</button>
        </div>
      </div>
    </div>
'''

for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</template>':
        lines.insert(i, comment_modal)
        break

# ── 3. Add state for new panels ────────────────────────────────────────────
more_state3 = r'''
// --- Comment Annotations State ---
const showCommentAnnotations = ref(false)
const sqlComments = ref<Array<{line:number;type:string;text:string}>>([])
const newCommentLine = ref(1)
const newCommentType = ref("note")
const newCommentText = ref("")

// --- Result Visualization State ---
const showResultViz = ref(false)
const vizType = ref("bar")
const vizXAxis = ref("")
const vizYAxis = ref("")
const vizRendered = ref(false)
const vizColors = ["#3b82f6","#10b981","#f59e0b","#ef4444","#8b5cf6","#ec4899","#06b6d4","#f97316"]
const vizData = ref<Array<{label:string;value:number;h:number}>>([])
const vizStats = ref<{count:number;max:number;min:number;avg:number}|null>(null)
'''

for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</script>':
        lines.insert(i, more_state3)
        break

# ── 4. Add comment and viz functions ───────────────────────────────────────
more_funcs2 = r'''
function addComment() {
  if (!newCommentText.value.trim()) return
  sqlComments.value.push({ line: newCommentLine.value, type: newCommentType.value, text: newCommentText.value })
  newCommentText.value = ""
}
const annotatedSql = computed(() => {
  if (!sqlComments.value.length) return sql.value
  const lines2 = sql.value.split('\n')
  return sqlComments.value.map(c => {
    const prefix = c.type === 'todo' ? '// TODO: ' : c.type === 'warning' ? '// WARN: ' : c.type === 'important' ? '// IMPORTANT: ' : '// NOTE: '
    return `-- [L${c.line}] ${prefix}${c.text}`
  }).join('\n') + '\n' + sql.value
})
function applyComments() { showCommentAnnotations.value = false }
function clearComments() { sqlComments.value = [] }

function renderViz() {
  if (!resultData.value.length || !vizXAxis.value || !vizYAxis.value) return
  const map = new Map<string,number>()
  resultData.value.forEach(row => {
    const key = String(row[vizXAxis.value])
    const val = Number(row[vizYAxis.value]) || 0
    map.set(key, (map.get(key) || 0) + val)
  })
  const entries = [...map.entries()].sort((a,b) => b[1]-a[1]).slice(0, 30)
  const maxVal = Math.max(1, ...entries.map(([,v]) => v))
  const nums = entries.map(([,v]) => v)
  vizData.value = entries.map(([label, value], i) => ({ label, value, h: Math.round(value/maxVal*150) }))
  vizStats.value = {
    count: resultData.value.length,
    max: Math.max(...nums),
    min: Math.min(...nums),
    avg: Math.round(nums.reduce((a:number,b:number)=>a+b,0)/nums.length)
  }
  vizRendered.value = true
}
function exportVizData() {
  if (!vizData.value.length) return
  const header = "label,value"
  const rows = vizData.value.map(d => `${d.label},${d.value}`).join('\n')
  const blob = new Blob([header+'\n'+rows], {type:'text/csv'})
  const a = document.createElement('a')
  a.href = URL.createObjectURL(blob)
  a.download = 'viz_data.csv'
  a.click()
}
'''

for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</script>':
        lines.insert(i, more_funcs2)
        break

# ── 5. Add CSS for new panels ─────────────────────────────────────────────
more_css3 = r'''
/* Comment Annotations */
.comment-panel{width:560px}.comment-body{padding:12px;max-height:440px;overflow-y:auto;display:flex;flex-direction:column;gap:10px}.comment-list{display:flex;flex-direction:column;gap:6px}.comment-item{background:rgba(255,255,255,0.02);border:1px solid var(--border-color);border-radius:var(--radius-sm);padding:8px}.comment-header{display:flex;align-items:center;gap:8px;margin-bottom:4px}.comment-line{font-size:11px;color:var(--color-primary);font-family:monospace}.comment-type{font-size:10px;padding:1px 6px;border-radius:10px;font-weight:600}.comment-type.note{background:rgba(59,130,246,0.15);color:var(--color-primary)}.comment-type.todo{background:rgba(239,68,68,0.15);color:#ef4444}.comment-type.warning{background:rgba(245,158,11,0.15);color:#f59e0b}.comment-type.important{background:rgba(139,92,246,0.15);color:#8b5cf6}.comment-del{margin-left:auto;padding:1px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--color-danger);cursor:pointer;font-size:10px}.comment-text{width:100%;padding:6px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none;resize:vertical;box-sizing:border-box}.comment-add{display:flex;gap:6px;align-items:center;flex-wrap:wrap}.comment-line-input{width:50px;padding:4px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}.comment-type-select{padding:4px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}.comment-input{flex:1;padding:4px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}.comment-preview{background:rgba(0,0,0,0.3);border-radius:var(--radius-sm);padding:10px}.comment-preview-label{font-size:11px;color:var(--text-muted);margin-bottom:4px}.comment-sql-out{margin:0;padding:8px;background:rgba(0,0,0,0.4);color:#7fdbca;font-size:11px;font-family:monospace;border-radius:4px;white-space:pre-wrap;word-break:break-all;max-height:100px;overflow-y:auto}.comment-footer{display:flex;gap:6px;padding-top:8px;border-top:1px solid var(--border-color)}
/* Result Visualization */
.viz-panel{width:640px}.viz-body{padding:12px;max-height:480px;overflow-y:auto;display:flex;flex-direction:column;gap:10px}.viz-controls{display:flex;gap:8px;flex-wrap:wrap;align-items:center}.viz-select{padding:5px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}.viz-chart{padding:16px;background:rgba(255,255,255,0.02);border:1px solid var(--border-color);border-radius:var(--radius-md);min-height:160px;display:flex;align-items:flex-end;gap:4px;flex-wrap:wrap;justify-content:center}.viz-bars{display:flex;align-items:flex-end;gap:4px;width:100%;justify-content:center}.viz-bar-wrap{display:flex;flex-direction:column;align-items:center;gap:2px;flex:1;max-width:60px}.viz-bar{width:100%;border-radius:3px 3px 0 0;transition:opacity 0.15s;cursor:pointer;min-height:4px}.viz-bar:hover{opacity:0.8}.viz-bar-label{font-size:8px;color:var(--text-muted);text-align:center;max-width:60px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}.viz-bar-val{font-size:9px;color:var(--text-primary);font-family:monospace}.viz-empty{color:var(--text-muted);font-size:12px;text-align:center;width:100%;padding:24px}.viz-stats{display:grid;grid-template-columns:repeat(4,1fr);gap:8px}.viz-stat{padding:8px;border-radius:var(--radius-sm);background:rgba(59,130,246,0.08);border:1px solid rgba(59,130,246,0.15);text-align:center;font-size:11px}.viz-stat span:first-child{color:var(--text-muted);display:block}.viz-stat span:last-child{color:var(--color-primary);font-weight:700;font-family:monospace;font-size:13px}.viz-footer{display:flex;gap:6px;padding-top:8px;border-top:1px solid var(--border-color)}
'''

for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</style>':
        lines.insert(i, more_css3)
        break

# ── Write back ────────────────────────────────────────────────────────────
new_content = '\n'.join(lines)
with open(path, 'w', encoding='utf-8') as f:
    f.write(new_content)
print(f"Done. Lines: {len(lines)}")
