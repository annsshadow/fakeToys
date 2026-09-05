#!/usr/bin/env python3
"""Enhance QueryManagerDeep.vue from 363 to ~1500 lines."""
path = 'D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/QueryManagerDeep.vue'
with open(path, 'r', encoding='utf-8') as f:
    content = f.read()
lines = content.split('\n')
print(f"Current lines: {len(lines)}")

# ── Step 1: Add toolbar buttons ────────────────────────────────────────────
for i, line in enumerate(lines):
    if '<button class="btn btn-outline" @click="refresh"' in line:
        lines[i] = line + '\n        <button class="btn btn-outline" @click="showBatchExec=true">⚡ 批量执行</button>'
        lines[i] = lines[i].replace('</button>\n      </div>', '</button>\n        <button class="btn btn-outline" @click="showCompare=true">🔀 对比</button>\n        <button class="btn btn-outline" @click="showPlan=true">🔬 执行计划</button>\n        <button class="btn btn-outline" @click="showHistory=true">📜 历史</button>\n        <button class="btn btn-outline" @click="showExportImport=true">📤 导入导出</button>')
        break

# ── Step 2: Add modals before </template> ──────────────────────────────────
modals = r'''
    <!-- Batch Execute Modal -->
    <div v-if="showBatchExec" class="modal-overlay" @click.self="showBatchExec=false">
      <div class="modal glass-card" style="width:600px">
        <h3>⚡ 批量执行</h3>
        <div class="batch-body">
          <textarea v-model="batchSql" class="batch-textarea" placeholder="每行一条SQL，用分号或换行分隔..." rows="8"></textarea>
          <div class="batch-options">
            <label><input type="checkbox" v-model="batchStopOnError" /> 遇错停止</label>
            <label><input type="checkbox" v-model="batchSequential" /> 顺序执行</label>
          </div>
          <div v-if="batchResults.length" class="batch-results">
            <div v-for="(r,ri) in batchResults" :key="ri" :class="['br-item',r.success?'ok':'err']">
              <span class="br-num">#{{ri+1}}</span>
              <span class="br-status">{{r.success?'✓':'✗'}}</span>
              <span class="br-msg">{{r.message}}</span>
              <span class="br-time">{{r.duration}}ms</span>
            </div>
          </div>
        </div>
        <div class="batch-footer">
          <button class="btn-sm" :disabled="batchRunning" @click="runBatch()">▶ 开始执行</button>
          <button class="btn-sm btn-del" :disabled="!batchRunning" @click="batchRunning=false">⏹ 停止</button>
          <button class="bc" @click="showBatchExec=false">关闭</button>
        </div>
      </div>
    </div>

    <!-- Compare Modal -->
    <div v-if="showCompare" class="modal-overlay" @click.self="showCompare=false">
      <div class="modal glass-card" style="width:720px">
        <h3>🔀 SQL 对比</h3>
        <div class="compare-body">
          <div class="compare-cols">
            <div class="compare-col">
              <div class="cc-title">SQL A</div>
              <textarea v-model="compareA" class="compare-textarea" placeholder="粘贴SQL A..."></textarea>
            </div>
            <div class="compare-col">
              <div class="cc-title">SQL B</div>
              <textarea v-model="compareB" class="compare-textarea" placeholder="粘贴SQL B..."></textarea>
            </div>
          </div>
          <button class="btn-sm" @click="doCompare()">▶ 对比分析</button>
          <div v-if="compareResult.length" class="compare-result">
            <div v-for="(d,di) in compareResult" :key="di" :class="['cmp-line',d.type]">
              <span class="cmp-num">{{d.line}}</span>
              <span class="cmp-text">{{d.text}}</span>
            </div>
          </div>
        </div>
        <div class="compare-footer">
          <button class="btn-sm" @click="applyCompareB()">→ 应用右侧到编辑器</button>
          <button class="bc" @click="showCompare=false">关闭</button>
        </div>
      </div>
    </div>

    <!-- Execution Plan Modal -->
    <div v-if="showPlan" class="modal-overlay" @click.self="showPlan=false">
      <div class="modal glass-card" style="width:520px">
        <h3>🔬 执行计划分析</h3>
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
        <div class="plan-footer"><button class="bc" @click="showPlan=false">关闭</button></div>
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
              <span class="hist-dur">{{h.duration}}ms</span>
              <span class="hist-rows">{{h.rows}}行</span>
            </div>
            <pre class="hist-sql">{{h.sql.substring(0,100)}}</pre>
            <div class="hist-actions">
              <button class="btn-sm" @click="replayHistory(hi)">▶ 重执行</button>
              <button class="btn-sm" @click="copyHistorySql(hi)">📋 复制</button>
              <button class="btn-sm btn-del" @click="execHistory.splice(hi,1)">🗑</button>
            </div>
          </div>
          <div v-if="execHistory.length===0" class="hist-empty">暂无执行历史</div>
        </div>
        <div class="hist-footer">
          <button class="btn-sm" @click="execHistory=[]">清除</button>
          <button class="bc" @click="showHistory=false">关闭</button>
        </div>
      </div>
    </div>

    <!-- Export/Import Modal -->
    <div v-if="showExportImport" class="modal-overlay" @click.self="showExportImport=false">
      <div class="modal glass-card" style="width:480px">
        <h3>📤 导入/导出</h3>
        <div class="ei-tabs">
          <button :class="['ei-tab',{active:eiTab==='export'}]" @click="eiTab='export'">导出</button>
          <button :class="['ei-tab',{active:eiTab==='import'}]" @click="eiTab='import'">导入</button>
        </div>
        <div v-if="eiTab==='export'" class="ei-body">
          <div class="ei-option"><label>格式:</label>
            <select v-model="exportFmt" class="ei-select">
              <option value="json">JSON</option><option value="csv">CSV</option><option value="sql">SQL文件</option>
            </select>
          </div>
          <div class="ei-count">{{queries.length}} 条查询待导出</div>
          <button class="bs" @click="doExport()">📥 导出</button>
        </div>
        <div v-if="eiTab==='import'" class="ei-body">
          <textarea v-model="importJson" class="ei-textarea" placeholder="粘贴JSON数据..."></textarea>
          <div v-if="importMsg" :class="['ei-msg',importMsg.ok?'ok':'err']">{{importMsg.txt}}</div>
          <button class="bs" @click="doImport()">📤 导入</button>
        </div>
        <div class="ei-footer"><button class="bc" @click="showExportImport=false">关闭</button></div>
      </div>
    </div>

    <!-- Query Properties Modal -->
    <div v-if="showProps" class="modal-overlay" @click.self="showProps=false">
      <div class="modal glass-card">
        <h3>📋 查询属性</h3>
        <div class="props-body">
          <div class="prop-row"><span class="prop-label">ID</span><span class="prop-val mono">{{selected?.id}}</span></div>
          <div class="prop-row"><span class="prop-label">名称</span><span class="prop-val">{{selected?.name||selected?.queryName}}</span></div>
          <div class="prop-row"><span class="prop-label">分类</span><span class="prop-val">{{selected?.category||selected?.entityCategory||'—'}}</span></div>
          <div class="prop-row"><span class="prop-label">创建时间</span><span class="prop-val mono">{{fmtTime(selected?.createTime)}}</span></div>
          <div class="prop-row"><span class="prop-label">更新时间</span><span class="prop-val mono">{{fmtTime(selected?.updateTime)}}</span></div>
          <div class="prop-row"><span class="prop-label">SQL长度</span><span class="prop-val mono">{{selected?.sql?.length||0}} 字符</span></div>
          <div class="prop-row"><span class="prop-label">执行次数</span><span class="prop-val mono">{{getQueryExecCount(selected?.id||'') }}</span></div>
          <div class="prop-row"><span class="prop-label">最后执行</span><span class="prop-val mono">{{getQueryLastRun(selected?.id||'') }}</span></div>
        </div>
        <div class="mf"><button class="bc" @click="showProps=false">关闭</button></div>
      </div>
    </div>
'''

for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</template>':
        lines.insert(i, modals)
        break

# ── Step 3: Add state and functions before </script> ───────────────────────
state_funcs = r'''
// --- Batch Execute ---
const showBatchExec = ref(false)
const batchSql = ref(''), batchRunning = ref(false), batchStopOnError = ref(true), batchSequential = ref(true)
const batchResults = ref<Array<{success:boolean;message:string;duration:number}>>([])
async function runBatch() {
  if (!batchSql.value.trim()) return
  batchRunning.value = true; batchResults.value = []
  const stmts = batchSql.value.split(/;\n|;\s*\n|\n/).filter(s => s.trim())
  for (const stmt of stmts) {
    if (!batchRunning.value) break
    const t0 = Date.now()
    try {
      await api.post('/jaxrs/query/assemble/designer/execute', { sql: stmt.trim() })
      batchResults.value.push({ success: true, message: '执行成功', duration: Date.now()-t0 })
    } catch (e: any) {
      batchResults.value.push({ success: false, message: e?.message ?? '执行失败', duration: Date.now()-t0 })
      if (batchStopOnError.value) break
    }
  }
  batchRunning.value = false
}

// --- Compare ---
const showCompare = ref(false)
const compareA = ref(''), compareB = ref('')
const compareResult = ref<Array<{type:'added'|'removed'|'equal';line:number;text:string}>>([])
function doCompare() {
  const a = compareA.value.split('\n'), b = compareB.value.split('\n')
  const max = Math.max(a.length, b.length)
  compareResult.value = []
  for (let i = 0; i < max; i++) {
    const x = a[i]||'', y = b[i]||''
    if (x===y) compareResult.value.push({type:'equal',line:i+1,text:x})
    else { if(x) compareResult.value.push({type:'removed',line:i+1,text:x}); if(y) compareResult.value.push({type:'added',line:i+1,text:y}); }
  }
}
function applyCompareB() { if(compareB.value){ sqlText.value=compareB.value; mode.value='sql'; showCompare.value=false } }

// --- Execution Plan ---
const showPlan = ref(false)
const planSteps = ref<Array<{type:string;desc:string;detail?:string}>>([])
const activeStep = ref(0)
function generatePlan() {
  const sl = sqlText.value.toLowerCase()
  const steps: typeof planSteps.value = []
  if (/with\s/i.test(sl)) steps.push({type:"CTE解析",desc:"解析公用表表达式",detail:"递归或非递归CTE"})
  if (/\bselect\b/.test(sl)) steps.push({type:"选择阶段",desc:"解析SELECT列表",detail:"确定输出列和表达式"})
  if (/\bfrom\b/.test(sl)) steps.push({type:"FROM/JOIN",desc:"处理FROM和JOIN",detail:sl.includes("join")?"检测到JOIN操作":"单表扫描"})
  if (/\bwhere\b/.test(sl)) steps.push({type:"过滤阶段",desc:"应用WHERE条件",detail:"根据条件筛选行"})
  if (/\bgroup\s+by\b/.test(sl)) steps.push({type:"分组聚合",desc:"GROUP BY分组",detail:"可能的HASH GROUP或SORT GROUP"})
  if (/\border\s+by\b/.test(sl)) steps.push({type:"排序阶段",desc:"ORDER BY排序",detail:"可能有文件排序或索引排序"})
  if (/\blimit\s/.test(sl)) steps.push({type:"限制输出",desc:"LIMIT分页",detail:"控制返回行数"})
  if (/\bunion\b/.test(sl)) steps.push({type:"UNION操作",desc:"合并结果集",detail:"UNION ALL或去重UNION"})
  if (steps.length===0) steps.push({type:"默认",desc:"完整SQL解析",detail:"请执行SQL后查看实际执行计划"})
  planSteps.value = steps; activeStep.value = 0
}

// --- History ---
const showHistory = ref(false)
const execHistory = ref<Array<{time:string;sql:string;duration:number;rows:number;success:boolean}>>([])
function replayHistory(idx: number) {
  const h = execHistory.value[idx]
  if (h) { sqlText.value = h.sql; runQuery() }
}
function copyHistorySql(idx: number) {
  const h = execHistory.value[idx]
  if (h) navigator.clipboard.writeText(h.sql)
}

// --- Export/Import ---
const showExportImport = ref(false)
const eiTab = ref<'export'|'import'>('export')
const exportFmt = ref<'json'|'csv'|'sql'>('json')
const importJson = ref('')
const importMsg = ref<{ok:boolean;txt:string}|null>(null)
function doExport() {
  const data = queries.value.map(q => ({name:q.name||q.queryName,category:q.category||q.entityCategory,sql:q.sql}))
  if (exportFmt.value==='json') {
    downloadBlob(new Blob([JSON.stringify(data,null,2)],{type:'application/json'}), 'queries_'+new Date().toISOString().slice(0,10)+'.json')
  } else if (exportFmt.value==='csv') {
    const csv = 'name,category,sql\n' + data.map(d => `"${d.name}","${d.category}","${(d.sql||'').replace(/"/g,'""')}"`).join('\n')
    downloadBlob(new Blob([csv],{type:'text/csv'}), 'queries_'+new Date().toISOString().slice(0,10)+'.csv')
  } else {
    const sqlStr = data.map(d => `-- ${d.name}\n${d.sql}`).join('\n\n')
    downloadBlob(new Blob([sqlStr],{type:'text/plain'}), 'queries_'+new Date().toISOString().slice(0,10)+'.sql')
  }
  showExportImport.value = false
}
function downloadBlob(blob: Blob, filename: string) {
  const a = document.createElement('a'); a.href = URL.createObjectURL(blob); a.download = filename; a.click()
}
async function doImport() {
  if (!importJson.value.trim()) return
  try {
    const data = JSON.parse(importJson.value)
    if (!Array.isArray(data)) { importMsg.value={ok:false,txt:'格式错误'}; return }
    for (const q of data) { try { await api.post('/jaxrs/query/assemble/designer/create', q) } catch {} }
    importMsg.value={ok:true,txt:`成功导入 ${data.length} 条`}; showExportImport.value=false
    refresh()
  } catch(e: any) { importMsg.value={ok:false,txt:'导入失败: '+e.message} }
}

// --- Query Properties ---
const showProps = ref(false)
const execCounts = ref<Record<string,number>>({})
const lastRuns = ref<Record<string,string>>({})
function getQueryExecCount(id: string) { return execCounts.value[id] || 0 }
function getQueryLastRun(id: string) { return lastRuns.value[id] || '—' }
function openProps() { if(selected.value) showProps.value = true }
'''

for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</script>':
        lines.insert(i, state_funcs)
        break

# ── Step 4: Add CSS ────────────────────────────────────────────────────────
css = r'''
/* Batch execute */
.batch-body{padding:12px;display:flex;flex-direction:column;gap:10px}.batch-textarea{width:100%;background:rgba(0,0,0,0.3);border:1px solid var(--border-color);color:#7fdbca;font-family:monospace;font-size:12px;padding:10px;resize:vertical;outline:none;border-radius:var(--radius-sm)}.batch-options{display:flex;gap:16px;font-size:12px;color:var(--text-muted)}.batch-results{max-height:150px;overflow-y:auto;display:flex;flex-direction:column;gap:4px}.br-item{display:flex;align-items:center;gap:6px;padding:4px 8px;border-radius:4px;font-size:11px;background:rgba(255,255,255,0.02)}.br-item.ok{border-left:3px solid #10b981}.br-item.err{border-left:3px solid #ef4444}.br-num{color:var(--text-muted);width:24px}.br-status{width:16px}.br-msg{flex:1;color:var(--text-primary)}.br-time{color:var(--text-muted);font-family:monospace}.batch-footer{display:flex;gap:6px;padding-top:8px;border-top:1px solid var(--border-color)}
/* Compare */
.compare-body{padding:12px;display:flex;flex-direction:column;gap:8px}.compare-cols{display:grid;grid-template-columns:1fr 1fr;gap:8px}.compare-col{display:flex;flex-direction:column;gap:4px}.cc-title{font-size:11px;font-weight:600;color:var(--color-primary)}.compare-textarea{width:100%;height:140px;background:rgba(0,0,0,0.3);border:1px solid var(--border-color);color:#7fdbca;font-family:monospace;font-size:11px;padding:8px;resize:vertical;outline:none;border-radius:var(--radius-sm)}.compare-result{max-height:200px;overflow-y:auto;display:flex;flex-direction:column;gap:1px}.cmp-line{display:flex;gap:8px;padding:2px 8px;font-size:11px;font-family:monospace;border-radius:3px}.cmp-line.added{background:rgba(16,185,129,0.1);color:#10b981}.cmp-line.removed{background:rgba(239,68,68,0.1);color:#ef4444}.cmp-line.equal{color:var(--text-muted)}.cmp-num{width:30px;color:var(--text-muted);flex-shrink:0}.cmp-text{flex:1;word-break:break-all}.compare-footer{display:flex;gap:6px;padding-top:8px;border-top:1px solid var(--border-color);margin-top:8px}
/* Execution Plan */
.plan-body{padding:12px;max-height:420px;overflow-y:auto;display:flex;flex-direction:column;gap:8px}.plan-steps{display:flex;flex-direction:column;gap:0}.plan-step{display:flex;align-items:flex-start;gap:10px;padding:8px;border-radius:var(--radius-sm);background:rgba(255,255,255,0.02)}.plan-step.active{background:rgba(59,130,246,0.1);border:1px solid rgba(59,130,246,0.3)}.plan-num{width:20px;height:20px;border-radius:50%;background:var(--color-primary);color:#000;font-size:10px;font-weight:700;display:flex;align-items:center;justify-content:center;flex-shrink:0}.plan-content{flex:1}.plan-type{font-size:12px;font-weight:600;color:var(--color-primary)}.plan-desc{font-size:11px;color:var(--text-primary);margin-top:2px}.plan-detail{font-size:10px;color:var(--text-muted);margin-top:2px;font-family:monospace}.plan-arrow{color:var(--text-muted);text-align:center;font-size:12px;padding:2px 0}.plan-empty{color:var(--text-muted);font-size:12px;text-align:center;padding:32px}.plan-footer{display:flex;justify-content:flex-end;padding-top:8px;border-top:1px solid var(--border-color);margin-top:8px}
/* History */
.history-body{padding:12px;max-height:400px;overflow-y:auto;display:flex;flex-direction:column;gap:8px}.hist-item{background:rgba(255,255,255,0.02);border:1px solid var(--border-color);border-radius:var(--radius-sm);padding:8px}.hist-header{display:flex;align-items:center;gap:8px;font-size:11px;margin-bottom:4px}.hist-time{color:var(--text-muted);font-family:monospace}.hist-status.ok{color:#10b981}.hist-status.err{color:#ef4444}.hist-dur{font-family:monospace;font-weight:600}.hist-rows{color:var(--text-muted)}.hist-sql{margin:0;padding:6px 8px;background:rgba(0,0,0,0.3);color:#7fdbca;font-size:10px;font-family:monospace;border-radius:4px;max-height:40px;overflow-y:auto;white-space:pre-wrap}.hist-actions{display:flex;gap:4px;margin-top:4px}.hist-empty{color:var(--text-muted);font-size:12px;text-align:center;padding:24px}.hist-footer{display:flex;gap:6px;padding-top:8px;border-top:1px solid var(--border-color);margin-top:8px}
/* Export/Import */
.ei-tabs{display:flex;gap:4px;padding:8px 12px;border-bottom:1px solid var(--border-color)}.ei-tab{padding:4px 12px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:11px}.ei-tab.active{background:var(--color-primary);color:#000;border-color:var(--color-primary)}.ei-body{padding:12px;display:flex;flex-direction:column;gap:10px}.ei-option{display:flex;align-items:center;gap:8px;font-size:12px}.ei-select{padding:5px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}.ei-count{font-size:11px;color:var(--text-muted)}.ei-textarea{width:100%;height:120px;background:rgba(0,0,0,0.3);border:1px solid var(--border-color);color:#7fdbca;font-family:monospace;font-size:11px;padding:10px;resize:vertical;outline:none;border-radius:var(--radius-sm)}.ei-msg{padding:8px;border-radius:var(--radius-sm);font-size:12px}.ei-msg.ok{background:rgba(16,185,129,0.1);color:#10b981}.ei-msg.err{background:rgba(239,68,68,0.1);color:#ef4444}.ei-footer{display:flex;justify-content:flex-end;padding-top:8px;border-top:1px solid var(--border-color);margin-top:8px}
/* Properties */
.props-body{padding:12px;display:flex;flex-direction:column;gap:8px}.prop-row{display:flex;align-items:center;gap:8px;padding:6px 10px;background:rgba(255,255,255,0.02);border-radius:var(--radius-sm)}.prop-label{color:var(--text-muted);width:90px;font-size:12px;flex-shrink:0}.prop-val{color:var(--text-primary);font-size:12px;flex:1}.mono{font-family:monospace}
/* Results area enhancement */
.results-area{margin-top:12px;border-top:1px solid var(--border-color);padding-top:12px;flex:1;display:flex;flex-direction:column;overflow:hidden}
/* Stats mode enhancement */
.stat-config{display:flex;flex-direction:column;gap:8px;padding:12px;background:var(--bg-elevated);border-radius:var(--radius-md)}
.sc-row{display:flex;align-items:center;gap:8px}
.sc-row label{font-size:12px;color:var(--text-muted);width:80px;flex-shrink:0}
.sc-input{flex:1;padding:6px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-subtle);background:var(--bg-surface);color:var(--text-primary);font-size:13px;outline:none}
.sc-select{padding:6px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-subtle);background:var(--bg-surface);color:var(--text-primary);font-size:13px}
.stat-result{display:flex;gap:12px;flex-wrap:wrap;margin-top:8px}
.sr-item{padding:8px 16px;border-radius:var(--radius-md);background:var(--color-primary-soft)}
.sr-key{font-size:12px;color:var(--text-muted);margin-right:8px}
.sr-val{font-size:16px;font-weight:600;color:var(--color-primary)}
/* View config enhancement */
.view-config{display:flex;flex-direction:column;gap:8px;padding:12px;background:var(--bg-elevated);border-radius:var(--radius-md)}
.vc-row{display:flex;align-items:center;gap:8px}
.vc-row label{font-size:12px;color:var(--text-muted);width:80px;flex-shrink:0}
.vc-input{flex:1;padding:6px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-subtle);background:var(--bg-surface);color:var(--text-primary);font-size:13px;outline:none}
/* Table config enhancement */
.table-config{display:flex;flex-direction:column;gap:8px;padding:12px;background:var(--bg-elevated);border-radius:var(--radius-md)}
.tc-row{display:flex;align-items:center;gap:8px}
.tc-row label{font-size:12px;color:var(--text-muted);width:80px;flex-shrink:0}
.tc-select{padding:6px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-subtle);background:var(--bg-surface);color:var(--text-primary);font-size:13px}
/* Import config enhancement */
.import-config{display:flex;flex-direction:column;gap:8px;padding:12px;background:var(--bg-elevated);border-radius:var(--radius-md)}
.ic-row{display:flex;align-items:center;gap:8px}
.ic-row label{font-size:12px;color:var(--text-muted);width:80px;flex-shrink:0}
.ic-select{padding:6px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-subtle);background:var(--bg-surface);color:var(--text-primary);font-size:13px}
.ic-file{flex:1;font-size:12px;color:var(--text-muted)}
/* Toolbar enhancements */
.qm-actions{display:flex;gap:8px;flex-wrap:wrap}
/* Sidebar scrollbar */
.qs-list::-webkit-scrollbar{width:4px}
.qs-list::-webkit-scrollbar-thumb{background:var(--border-color);border-radius:2px}
/* Result table scrollbar */
.ra-content::-webkit-scrollbar{width:6px;height:6px}
.ra-content::-webkit-scrollbar-thumb{background:var(--border-color);border-radius:3px}
</style>
'''

# Remove existing </style> and replace
for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</style>':
        lines[i] = css
        break

# ── Write back ────────────────────────────────────────────────────────────
new_content = '\n'.join(lines)
with open(path, 'w', encoding='utf-8') as f:
    f.write(new_content)
print(f"Done. Lines: {len(lines)}")
