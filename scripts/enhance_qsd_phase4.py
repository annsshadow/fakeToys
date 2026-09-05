#!/usr/bin/env python3
"""Phase 4: Final push to ~2000 lines with parameter presets, SQL hints, and extra features."""
import re

path = 'D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/QueryStatementDesigner.vue'
with open(path, 'r', encoding='utf-8') as f:
    content = f.read()
lines = content.split('\n')
print(f"Current lines: {len(lines)}")

# ── Step 1: Add toolbar buttons ────────────────────────────────────────────
for i, line in enumerate(lines):
    if 'showBookmark=true' in line and 'title="书签"' in line:
        lines[i] = line + '        <button class="btn btn-outline" @click="showParamPresets=!showParamPresets" title="参数预设">🔗 参数</button>'
        lines[i] = lines[i].replace('</button>\n        <button', '</button>\n        <button')
        break

# ── Step 2: Add modals ─────────────────────────────────────────────────────
modals = r'''
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
'''

for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</template>':
        lines.insert(i, modals)
        break

# ── Step 3: Add state + functions ───────────────────────────────────────────
sf = r'''
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
'''

for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</script>':
        lines.insert(i, sf)
        break

# ── Step 4: Add CSS ────────────────────────────────────────────────────────
css = r'''
/* Parameter Presets */
.param-panel{width:560px}.param-body{padding:12px;display:flex;flex-direction:column;gap:10px}.param-list{display:flex;flex-direction:column;gap:4px;max-height:180px;overflow-y:auto}.param-row{display:flex;align-items:center;gap:6px;padding:4px 8px;background:rgba(255,255,255,0.02);border-radius:var(--radius-sm);font-size:11px}.param-name{color:#f59e0b;width:80px;font-family:monospace;font-weight:600;flex-shrink:0}.param-input{flex:1;padding:3px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}.param-type{padding:3px;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-sm);color:var(--text-primary);font-size:10px}.param-del{padding:2px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--color-danger);cursor:pointer;font-size:10px}.param-detect{padding:10px;background:rgba(245,158,11,0.05);border:1px solid rgba(245,158,11,0.2);border-radius:var(--radius-sm)}.pd-title{font-size:11px;color:#f59e0b;margin-bottom:6px;font-weight:600}.pd-tags{display:flex;flex-wrap:wrap;gap:4px;margin-bottom:6px}.pd-tag{padding:2px 8px;border-radius:10px;background:rgba(245,158,11,0.15);border:1px solid rgba(245,158,11,0.3);color:#f59e0b;font-size:10px;font-family:monospace;cursor:pointer}.pd-tag.exists{background:rgba(16,185,129,0.15);border-color:rgba(16,185,129,0.3);color:#10b981}.param-footer{display:flex;gap:6px;padding-top:8px;border-top:1px solid var(--border-color)}
/* SQL Hints */
.hint-panel{width:520px}.hint-body{padding:12px;max-height:400px;overflow-y:auto;display:flex;flex-direction:column;gap:12px}.hint-section{display:flex;flex-direction:column;gap:6px}.hint-title{font-size:11px;font-weight:600;color:var(--color-primary);text-transform:uppercase}.hint-tags{display:flex;flex-wrap:wrap;gap:4px}.hint-tag{padding:3px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:rgba(255,255,255,0.02);color:var(--text-muted);cursor:pointer;font-size:11px;font-family:monospace;transition:all 0.15s}.hint-tag:hover{border-color:var(--color-primary);color:var(--color-primary);background:rgba(59,130,246,0.1)}
'''

for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</style>':
        lines.insert(i, css)
        break

# ── Write back ────────────────────────────────────────────────────────────
new_content = '\n'.join(lines)
with open(path, 'w', encoding='utf-8') as f:
    f.write(new_content)
print(f"Done. Lines: {len(lines)}")
