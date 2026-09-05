#!/usr/bin/env python3
"""Phase 2: Add execution plan, diff tool, export/import, bulk delete, result stats to QSD."""
import re

path = 'D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/QueryStatementDesigner.vue'
with open(path, 'r', encoding='utf-8') as f:
    content = f.read()
lines = content.split('\n')
print(f"Current lines: {len(lines)}")

# ── Step 1: Add toolbar buttons ────────────────────────────────────────────
for i, line in enumerate(lines):
    if 'showSnippetLibrary=true' in line and 'title="SQL片段库"' in line:
        lines[i] = line + '        <button class="btn btn-outline" @click="showExecPlan=true" title="执行计划">🔬 计划</button>'
        lines[i] = lines[i].replace('</button>\n        <button', '</button>\n        <button')
        break

# ── Step 2: Add modals ─────────────────────────────────────────────────────
modals = r'''
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
'''

for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</template>':
        lines.insert(i, modals)
        break

# ── Step 3: Add state + functions ───────────────────────────────────────────
state_funcs = r'''
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
'''

for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</script>':
        lines.insert(i, state_funcs)
        break

# ── Step 4: Add CSS ────────────────────────────────────────────────────────
css = r'''
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
