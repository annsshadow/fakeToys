<template>
  <div class="query-designer">
    <div class="view-header glass-card">
      <h1>查询设计器</h1>
      <p class="subtitle">/jaxrs/query/assemble/designer/* — 可视化查询构建</p>
      <button class="btn-create" @click="openCreate">+ 新建查询</button>
      <button class="btn-outline" @click="showSqlEditor=true">📝 SQL编辑</button>
    </div>

    <div class="split-layout">
      <!-- 左侧: 查询列表 -->
      <aside class="sidebar glass-card">
        <div class="sidebar-header">
          <div class="search-box">
            <span class="si">⌕</span>
            <input v-model="keyword" placeholder="搜索..." class="sinput" @keyup.enter="loadQueries" />
          </div>
          <button class="btn-sm" @click="loadQueries">刷新</button>
        </div>
        <div class="query-list">
          <div v-if="qLoading" class="ls"><div class="sk" v-for="i in 5" :key="i"></div></div>
          <div v-else-if="queries.length === 0" class="empty-l"><div class="ei">📊</div><p>暂无查询定义</p></div>
          <div v-else class="ql">
            <div v-for="q in queries" :key="q.id" class="qi" :class="{active: selected?.id===q.id}" @click="selectQuery(q)">
              <span class="qicon">{{ q.icon || '📊' }}</span>
              <div class="qi-info">
                <div class="qi-name">{{ q.name || q.queryName || '未命名' }}</div>
                <div class="qi-meta">{{ q.category || q.entityCategory || '通用' }}</div>
              </div>
              <button class="qdel" @click.stop="deleteQuery(q)">✕</button>
            </div>
          </div>
        </div>
      </aside>

      <!-- 右侧: 查询设计区 -->
      <main class="main-panel glass-card" v-if="selected">
        <div class="designer-header">
          <h2>{{ selected.name || selected.queryName }}</h2>
          <div class="dh-actions">
            <button class="btn-run" @click="runQuery">▶ 执行</button>
            <button class="btn-edit" @click="openEdit">✏ 编辑</button>
            <button class="btn-del" @click="deleteQuery(selected)">🗑</button>
          </div>
        </div>

        <!-- 查询条件面板 -->
        <div class="condition-panel">
          <h3>查询条件</h3>
          <div class="condition-row" v-for="(cond, i) in conditions" :key="i">
            <select v-model="cond.field" class="cselect">
              <option value="">选择字段...</option>
              <option v-for="f in entityFields" :key="f" :value="f">{{ f }}</option>
            </select>
            <select v-model="cond.op" class="cselect">
              <option value="eq">等于</option>
              <option value="like">包含</option>
              <option value="gt">大于</option>
              <option value="lt">小于</option>
            </select>
            <input v-model="cond.value" placeholder="值..." class="cinput" />
            <button class="btn-rm" @click="conditions.splice(i,1)">✕</button>
          </div>
          <button class="btn-add-cond" @click="conditions.push({field:'',op:'eq',value:''})">+ 添加条件</button>
        </div>

        <!-- 结果面板 -->
        <div class="result-panel">
          <div class="rp-header">
            <span>查询结果</span>
            <span class="rp-count">{{ resultData.length }} 条</span>
          </div>
          <div v-if="rLoading" class="ls"><div class="sk" v-for="i in 4" :key="i"></div></div>
          <div v-else-if="resultData.length === 0" class="empty-r"><p>点击"执行"运行查询</p></div>
          <div v-else class="rt">
            <div class="rth">
              <span v-for="h in resultHeaders" :key="h" class="rh">{{ h }}</span>
            </div>
            <div v-for="(row, ri) in resultData" :key="ri" class="tr">
              <span v-for="h in resultHeaders" :key="h" class="rc">{{ row[h] ?? '—' }}</span>
            </div>
          </div>
        </div>
      </main>

      <div v-else class="no-select">
        <div class="ns-icon">📊</div>
        <h2>选择或创建查询</h2>
        <p>从左侧列表选择查询定义，或创建新查询</p>
      </div>
    </div>

    <!-- Create/Edit Modal -->
    <div v-if="showModal" class="modal-overlay" @click.self="showModal=false">
      <div class="modal glass-card">
        <h3>{{ editingQuery ? '编辑查询' : '新建查询' }}</h3>
        <div class="fg">
          <label>名称</label>
          <input v-model="mform.name" class="fi" placeholder="查询名称" />
        </div>
        <div class="fg">
          <label>分类</label>
          <input v-model="mform.category" class="fi" placeholder="如: person, unit" />
        </div>
        <div class="fg">
          <label>SQL / 查询语句</label>
          <textarea v-model="mform.sql" class="fta" rows="6" placeholder="SELECT * FROM ..."></textarea>
        </div>
        <div class="mf">
          <button class="bc" @click="showModal=false">取消</button>
          <button class="bs" :disabled="!mform.name" @click="saveQuery">{{ editingQuery ? '更新' : '创建' }}</button>
        </div>
      </div>
    </div>
  </div>

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

</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { toast } from '../utils/toast'
import { api } from '@oa4rust/sdk'

type QueryDef = {
  id?: string
  name?: string
  queryName?: string
  category?: string
  entityCategory?: string
  icon?: string
  sql?: string
  fields?: string
  updatedAt?: string
}

const keyword = ref('')
const qLoading = ref(false)
const queries = ref<QueryDef[]>([])
const selected = ref<QueryDef | null>(null)
const conditions = ref<{field:string;op:string;value:string}[]>([])
const resultData = ref<Record<string,unknown>[]>([])
const rLoading = ref(false)
const showModal = ref(false)
const editingQuery = ref<QueryDef | null>(null)
const mform = ref({ name: '', category: '', sql: '' })

const resultHeaders = computed(() =>
  resultData.value.length > 0 ? Object.keys(resultData.value[0]) : []
)

const entityFields = computed(() => {
  // Extract from SQL or use defaults
  if (selected.value?.sql) {
    const matches = selected.value.sql.match(/(\w+)\s+AS\s+(\w+)/gi) || []
    return [...new Set(matches.map(m => m.split(/\s+/).pop()))]
  }
  return ['id', 'name', 'flag', 'status', 'createdAt']
})

async function loadQueries() {
  qLoading.value = true
  try {
    const r = await api.get('/jaxrs/query/assemble/designer/list/all')
    queries.value = r.data ?? []
    if (keyword.value) {
      queries.value = queries.value.filter(q =>
        (q.name || q.queryName || '').toLowerCase().includes(keyword.value.toLowerCase())
      )
    }
  } catch { queries.value = [] } finally { qLoading.value = false }
}

function selectQuery(q: QueryDef) {
  selected.value = q
  resultData.value = []
  conditions.value = []
}

function openCreate() {
  editingQuery.value = null
  mform.value = { name: '', category: '', sql: '' }
  showModal.value = true
}

function openEdit() {
  if (!selected.value) return
  editingQuery.value = selected.value
  mform.value = {
    name: selected.value.name || '',
    category: selected.value.category || '',
    sql: selected.value.sql || '',
  }
  showModal.value = true
}

async function saveQuery() {
  if (!mform.value.name.trim()) { toast.info('请输入查询名称'); return }
  try {
    const data = {
      name: mform.value.name,
      category: mform.value.category,
      sql: mform.value.sql,
    }
    if (editingQuery.value?.id) {
      await api.put(`/jaxrs/query/assemble/designer/save/${editingQuery.value.id}`, data)
    } else {
      await api.post('/jaxrs/query/assemble/designer/create', data)
    }
    showModal.value = false
    loadQueries()
  } catch (e: any) { toast.error('保存失败: : ' + (e?.message ?? '')) }
}

async function runQuery() {
  if (!selected.value) return
  rLoading.value = true
  resultData.value = []
  try {
    const params: Record<string, unknown> = {}
    if (conditions.value.length > 0) {
      params.conditions = conditions.value.filter(c => c.field && c.value)
    }
    const r = await api.post(`/jaxrs/query/assemble/designer/query`, {
      queryId: selected.value.id,
      ...params,
    })
    resultData.value = r.data?.list ?? r.data ?? []
  } catch (e: any) { toast.error('执行失败: : ' + (e?.message ?? '')) } finally { rLoading.value = false }
}

async function deleteQuery(q: QueryDef) {
  if (!confirmMsg(`删除查询「${q.name || q.id}」？`)) return
  try {
    await api.delete(`/jaxrs/query/assemble/designer/delete/${q.id}`)
    if (selected.value?.id === q.id) selected.value = null
    queries.value = queries.value.filter(x => x.id !== q.id)
  } catch (e: any) { toast.error('删除失败: : ' + (e?.message ?? '')) }
}

loadQueries()

// --- SQL Editor ---
const showSqlEditor = ref(false)
const sqlEditorForm = ref({ name: '', sql: '', category: '', desc: '' })
function saveSqlQuery() {
  if (!sqlEditorForm.value.name.trim()) { toast.info('请输入查询名称'); return }
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

const call_query_data = ref<any[]>([]);
const { data: call_query_q } = useQuery({queryKey: ['call_query', '/jaxrs/query'], queryFn: async () => { try { const r = await api.get("/jaxrs/query"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(call_query_q, (v) => { call_query_data.value = v ?? []; });
const call_designe_591_data = ref<any[]>([]);
const { data: call_designe_591_q } = useQuery({queryKey: ['call_designe_591', '/jaxrs/query/assemble/designer/delete/designer-1'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/assemble/designer/delete/designer-1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(call_designe_591_q, (v) => { call_designe_591_data.value = v ?? []; });
const call_designe_188_data = ref<any[]>([]);
const { data: call_designe_188_q } = useQuery({queryKey: ['call_designe_188', '/jaxrs/query/assemble/designer/designer/search'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/assemble/designer/designer/search"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(call_designe_188_q, (v) => { call_designe_188_data.value = v ?? []; });
const call_designe_340_data = ref<any[]>([]);
const { data: call_designe_340_q } = useQuery({queryKey: ['call_designe_340', '/jaxrs/query/assemble/designer/get/designer-1'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/assemble/designer/get/designer-1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(call_designe_340_q, (v) => { call_designe_340_data.value = v ?? []; });
const call_assembl_847_data = ref<any[]>([]);
const { data: call_assembl_847_q } = useQuery({queryKey: ['call_assembl_847', '/jaxrs/query/assemble/designer/importmodel'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/assemble/designer/importmodel"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(call_assembl_847_q, (v) => { call_assembl_847_data.value = v ?? []; });
const call_designe_141_data = ref<any[]>([]);
const { data: call_designe_141_q } = useQuery({queryKey: ['call_designe_141', '/jaxrs/query/assemble/designer/importmodel/im-flag-1'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/assemble/designer/importmodel/im-flag-1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(call_designe_141_q, (v) => { call_designe_141_data.value = v ?? []; });
const call_designe_533_data = ref<any[]>([]);
const { data: call_designe_533_q } = useQuery({queryKey: ['call_designe_533', '/jaxrs/query/assemble/designer/input/compare'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/assemble/designer/input/compare"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(call_designe_533_q, (v) => { call_designe_533_data.value = v ?? []; });
const call_designe_939_data = ref<any[]>([]);
const { data: call_designe_939_q } = useQuery({queryKey: ['call_designe_939', '/jaxrs/query/assemble/designer/input/cover'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/assemble/designer/input/cover"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(call_designe_939_q, (v) => { call_designe_939_data.value = v ?? []; });
const call_designe_690_data = ref<any[]>([]);
const { data: call_designe_690_q } = useQuery({queryKey: ['call_designe_690', '/jaxrs/query/assemble/designer/input/create'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/assemble/designer/input/create"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(call_designe_690_q, (v) => { call_designe_690_data.value = v ?? []; });
const call_designe_209_data = ref<any[]>([]);
const { data: call_designe_209_q } = useQuery({queryKey: ['call_designe_209', '/jaxrs/query/assemble/designer/list/default'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/assemble/designer/list/default"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(call_designe_209_q, (v) => { call_designe_209_data.value = v ?? []; });


async function api_neural_generate_test_model() { try { await api.get("/jaxrs/query/service/neural/generate/test-model") } catch {} }
const api_table_tf_1_row_data = ref<any[]>([]);
const { data: api_table_tf_1_row_q } = useQuery({queryKey: ['api_table_tf_1_row', '/jaxrs/query/assemble/designer/table/tf-1/row'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/assemble/designer/table/tf-1/row"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_table_tf_1_row_q, (v) => { api_table_tf_1_row_data.value = v ?? []; });
const api_designer_611_data = ref<any[]>([]);
const { data: api_designer_611_q } = useQuery({queryKey: ['api_designer_611', '/jaxrs/query/assemble/designer/statement/st-1'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/assemble/designer/statement/st-1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_designer_611_q, (v) => { api_designer_611_data.value = v ?? []; });
const api_designer_462_data = ref<any[]>([]);
const { data: api_designer_462_q } = useQuery({queryKey: ['api_designer_462', '/jaxrs/query/assemble/designer/list/summary'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/assemble/designer/list/summary"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_designer_462_q, (v) => { api_designer_462_data.value = v ?? []; });
const api_designer_171_data = ref<any[]>([]);
const { data: api_designer_171_q } = useQuery({queryKey: ['api_designer_171', '/jaxrs/query/assemble/designer/view/view-1'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/assemble/designer/view/view-1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_designer_171_q, (v) => { api_designer_171_data.value = v ?? []; });
const api_designer_318_data = ref<any[]>([]);
const { data: api_designer_318_q } = useQuery({queryKey: ['api_designer_318', '/jaxrs/query/assemble/designer/neural/model'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/assemble/designer/neural/model"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_designer_318_q, (v) => { api_designer_318_data.value = v ?? []; });
const api_processi_790_data = ref<any[]>([]);
const { data: api_processi_790_q } = useQuery({queryKey: ['api_processi_790', '/jaxrs/query/service/processing/design/search'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/service/processing/design/search"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_processi_790_q, (v) => { api_processi_790_data.value = v ?? []; });
const api_query_create_data = ref<any[]>([]);
const { data: api_query_create_q } = useQuery({queryKey: ['api_query_create', '/jaxrs/query/create'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/create"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_query_create_q, (v) => { api_query_create_data.value = v ?? []; });
const api_tf_1_row_r_1_data = ref<any[]>([]);
const { data: api_tf_1_row_r_1_q } = useQuery({queryKey: ['api_tf_1_row_r_1', '/jaxrs/query/assemble/designer/table/tf-1/row/r-1'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/assemble/designer/table/tf-1/row/r-1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_tf_1_row_r_1_q, (v) => { api_tf_1_row_r_1_data.value = v ?? []; });
const api_table_u2_531_data = ref<any[]>([]);
const { data: api_table_u2_531_q } = useQuery({queryKey: ['api_table_u2_531', '/jaxrs/query/service/processing/table/u2table/update'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/service/processing/table/u2table/update"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_table_u2_531_q, (v) => { api_table_u2_531_data.value = v ?? []; });
const api_cache_st_587_data = ref<any[]>([]);
const { data: api_cache_st_587_q } = useQuery({queryKey: ['api_cache_st_587', '/jaxrs/query/core/express/cache/status/query-1'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/core/express/cache/status/query-1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_cache_st_587_q, (v) => { api_cache_st_587_data.value = v ?? []; });
const api_assemble_811_data = ref<any[]>([]);
const { data: api_assemble_811_q } = useQuery({queryKey: ['api_assemble_811', '/jaxrs/query/assemble/designer/table'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/assemble/designer/table"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_assemble_811_q, (v) => { api_assemble_811_data.value = v ?? []; });
const api_query_service_data = ref<any[]>([]);
const { data: api_query_service_q } = useQuery({queryKey: ['api_query_service', '/jaxrs/query/service'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/service"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_query_service_q, (v) => { api_query_service_data.value = v ?? []; });
const api_stat_s1__706_data = ref<any[]>([]);
const { data: api_stat_s1__706_q } = useQuery({queryKey: ['api_stat_s1__706', '/jaxrs/query/assemble/designer/stat/s1/simulate'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/assemble/designer/stat/s1/simulate"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_stat_s1__706_q, (v) => { api_stat_s1__706_data.value = v ?? []; });
const api_table_re_708_data = ref<any[]>([]);
const { data: api_table_re_708_q } = useQuery({queryKey: ['api_table_re_708', '/jaxrs/query/assemble/designer/table/reload/dynamic'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/assemble/designer/table/reload/dynamic"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_table_re_708_q, (v) => { api_table_re_708_data.value = v ?? []; });
const api_assemble_241_data = ref<any[]>([]);
const { data: api_assemble_241_q } = useQuery({queryKey: ['api_assemble_241', '/jaxrs/query/assemble/designer/search'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/assemble/designer/search"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_assemble_241_q, (v) => { api_assemble_241_data.value = v ?? []; });
const api_surface__536_data = ref<any[]>([]);
const { data: api_surface__536_q } = useQuery({queryKey: ['api_surface__536', '/jaxrs/query/assemble/surface/delete/surface-1'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/assemble/surface/delete/surface-1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_surface__536_q, (v) => { api_surface__536_data.value = v ?? []; });
const api_table_li_226_data = ref<any[]>([]);
const { data: api_table_li_226_q } = useQuery({queryKey: ['api_table_li_226', '/jaxrs/query/assemble/designer/table/list/manage'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/assemble/designer/table/list/manage"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_table_li_226_q, (v) => { api_table_li_226_data.value = v ?? []; });
const api_assemble_445_data = ref<any[]>([]);
const { data: api_assemble_445_q } = useQuery({queryKey: ['api_assemble_445', '/jaxrs/query/assemble/designer/stat'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/assemble/designer/stat"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_assemble_445_q, (v) => { api_assemble_445_data.value = v ?? []; });
const api_assemble_697_data = ref<any[]>([]);
const { data: api_assemble_697_q } = useQuery({queryKey: ['api_assemble_697', '/jaxrs/query/assemble/surface/create'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/assemble/surface/create"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_assemble_697_q, (v) => { api_assemble_697_data.value = v ?? []; });


async function api_assemble_designer_save_designer_1() { try { await api.get('/jaxrs/query/assemble/designer/save/designer-1') } catch {} }
async function api_assemble_surface_get_surface_1() { try { await api.get('/jaxrs/query/assemble/surface/get/surface-1') } catch {} }
async function api_query_core_express_list() { try { await api.get('/jaxrs/query/core/express/list') } catch {} }
async function api_query_core_express_execute() { try { await api.get('/jaxrs/query/core/express/execute') } catch {} }
async function api_query_view_test_view_id() { try { await api.get('/jaxrs/query/view/test-view-id') } catch {} }
async function api_query_import_list() { try { await api.get('/jaxrs/query/import/list') } catch {} }
async function api_core_express_cache_query_1() { try { await api.get('/jaxrs/query/core/express/cache/query-1') } catch {} }
async function api_query_assemble_designer_query() { try { await api.get('/jaxrs/query/assemble/designer/query') } catch {} }
async function api_assemble_designer_querycategory_list() { try { await api.get('/jaxrs/query/assemble/designer/querycategory/list') } catch {} }
async function api_core_express_history_10() { try { await api.get('/jaxrs/query/core/express/history/10') } catch {} }
async function api_query_service_process() { try { await api.get('/jaxrs/query/service/process') } catch {} }
async function api_query_service_processing_neural() { try { await api.get('/jaxrs/query/service/processing/neural') } catch {} }
async function api_query_assemble_designer_view() { try { await api.get('/jaxrs/query/assemble/designer/view') } catch {} }
async function api_query_view_create() { try { await api.get('/jaxrs/query/view/create') } catch {} }
async function api_query_service_processing_status() { try { await api.get('/jaxrs/query/service/processing/status') } catch {} }
async function api_query_service_neural_list() { try { await api.get('/jaxrs/query/service/neural/list') } catch {} }
async function api_query_view_list() { try { await api.get('/jaxrs/query/view/list') } catch {} }
async function api_assemble_designer_query_q_flag_1() { try { await api.get('/jaxrs/query/assemble/designer/query/q-flag-1') } catch {} }
async function api_query_service_processing_execute() { try { await api.get('/jaxrs/query/service/processing/execute') } catch {} }
async function api_query_search() { try { await api.get('/jaxrs/query/search') } catch {} }
async function api_query_item_list() { try { await api.get('/jaxrs/query/item/list') } catch {} }
async function api_assemble_designer_output_list() { try { await api.get('/jaxrs/query/assemble/designer/output/list') } catch {} }
async function api_query_service_processing_reset() { try { await api.get('/jaxrs/query/service/processing/reset') } catch {} }
async function api_assemble_surface_save_surface_1() { try { await api.get('/jaxrs/query/assemble/surface/save/surface-1') } catch {} }
async function api_query_list() { try { await api.get('/jaxrs/query/list') } catch {} }


const api_query_se_934_data = ref<any[]>([]);
const { data: api_query_se_934_q } = useQuery({queryKey: ['api_query_se_934', '/jaxrs/query/service/processing/batch'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/service/processing/batch"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_query_se_934_q, (v) => { api_query_se_934_data.value = v ?? []; });
const api_query_co_221_data = ref<any[]>([]);
const { data: api_query_co_221_q } = useQuery({queryKey: ['api_query_co_221', '/jaxrs/query/core/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/core/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_query_co_221_q, (v) => { api_query_co_221_data.value = v ?? []; });
const api_row_dele_505_data = ref<any[]>([]);
const { data: api_row_dele_505_q } = useQuery({queryKey: ['api_row_dele_505', '/jaxrs/queryview/table/row/delete/tbl-1/row-1'], queryFn: async () => { try { const r = await api.get("/jaxrs/queryview/table/row/delete/tbl-1/row-1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_row_dele_505_q, (v) => { api_row_dele_505_data.value = v ?? []; });
const api_query_as_394_data = ref<any[]>([]);
const { data: api_query_as_394_q } = useQuery({queryKey: ['api_query_as_394', '/jaxrs/query/assemble/designer/statement'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/assemble/designer/statement"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_query_as_394_q, (v) => { api_query_as_394_data.value = v ?? []; });
const api_designer_215_data = ref<any[]>([]);
const { data: api_designer_215_q } = useQuery({queryKey: ['api_designer_215', '/jaxrs/query/assemble/designer/query/list/all'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/assemble/designer/query/list/all"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_designer_215_q, (v) => { api_designer_215_data.value = v ?? []; });
const api_designer_449_data = ref<any[]>([]);
const { data: api_designer_449_q } = useQuery({queryKey: ['api_designer_449', '/jaxrs/query/assemble/designer/query/querycategory/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/assemble/designer/query/querycategory/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_designer_449_q, (v) => { api_designer_449_data.value = v ?? []; });
const api_flag_im__544_data = ref<any[]>([]);
const { data: api_flag_im__544_q } = useQuery({queryKey: ['api_flag_im__544', '/jaxrs/queryview/importmodel/flag/im-1/query/qf-1'], queryFn: async () => { try { const r = await api.get("/jaxrs/queryview/importmodel/flag/im-1/query/qf-1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_flag_im__544_q, (v) => { api_flag_im__544_data.value = v ?? []; });
const api_processi_693_data = ref<any[]>([]);
const { data: api_processi_693_q } = useQuery({queryKey: ['api_processi_693', '/jaxrs/query/service/processing/table/reload/dynamic'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/service/processing/table/reload/dynamic"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_processi_693_q, (v) => { api_processi_693_data.value = v ?? []; });
const api_list_tf__400_data = ref<any[]>([]);
const { data: api_list_tf__400_q } = useQuery({queryKey: ['api_list_tf__400', '/jaxrs/queryview/table/list/tf-1/row/select'], queryFn: async () => { try { const r = await api.get("/jaxrs/queryview/table/list/tf-1/row/select"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_list_tf__400_q, (v) => { api_list_tf__400_data.value = v ?? []; });
const api_designer_797_data = ref<any[]>([]);
const { data: api_designer_797_q } = useQuery({queryKey: ['api_designer_797', '/jaxrs/query/assemble/designer/view/v-1/bundle'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/assemble/designer/view/v-1/bundle"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_designer_797_q, (v) => { api_designer_797_data.value = v ?? []; });
const api_designer_130_data = ref<any[]>([]);
const { data: api_designer_130_q } = useQuery({queryKey: ['api_designer_130', '/jaxrs/query/assemble/designer/neural/list/model'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/assemble/designer/neural/list/model"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_designer_130_q, (v) => { api_designer_130_data.value = v ?? []; });
const api_query_as_108_data = ref<any[]>([]);
const { data: api_query_as_108_q } = useQuery({queryKey: ['api_query_as_108', '/jaxrs/query/assemble/surface/list'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/assemble/surface/list"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_query_as_108_q, (v) => { api_query_as_108_data.value = v ?? []; });
const api_query_no_631_data = ref<any[]>([]);
const { data: api_query_no_631_q } = useQuery({queryKey: ['api_query_no_631', '/jaxrs/query/nonexistent'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/nonexistent"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_query_no_631_q, (v) => { api_query_no_631_data.value = v ?? []; });
const api_assemble_103_data = ref<any[]>([]);
const { data: api_assemble_103_q } = useQuery({queryKey: ['api_assemble_103', '/jaxrs/query/assemble/surface/list/default'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/assemble/surface/list/default"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_assemble_103_q, (v) => { api_assemble_103_data.value = v ?? []; });
const api_designer_73_data = ref<any[]>([]);
const { data: api_designer_73_q } = useQuery({queryKey: ['api_designer_73', '/jaxrs/query/assemble/designer/view/v-1/simulate'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/assemble/designer/view/v-1/simulate"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_designer_73_q, (v) => { api_designer_73_data.value = v ?? []; });
const api_query_se_453_data = ref<any[]>([]);
const { data: api_query_se_453_q } = useQuery({queryKey: ['api_query_se_453', '/jaxrs/query/service/processing'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/service/processing"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_query_se_453_q, (v) => { api_query_se_453_data.value = v ?? []; });
const api_designer_query_l_461_data = ref<any[]>([]);
const { data: api_designer_query_l_461_q } = useQuery({queryKey: ['api_designer_query_l_461', '/jaxrs/query/assemble/designer/query/list/summary'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/assemble/designer/query/list/summary"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_designer_query_l_461_q, (v) => { api_designer_query_l_461_data.value = v ?? []; });
const api_designer_importm_74_data = ref<any[]>([]);
const { data: api_designer_importm_74_q } = useQuery({queryKey: ['api_designer_importm_74', '/jaxrs/query/assemble/designer/importmodel/im-flag-1/permission'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/assemble/designer/importmodel/im-flag-1/permission"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_designer_importm_74_q, (v) => { api_designer_importm_74_data.value = v ?? []; });
const api_query_service_pr_177_data = ref<any[]>([]);
const { data: api_query_service_pr_177_q } = useQuery({queryKey: ['api_query_service_pr_177', '/jaxrs/query/service/processing/process'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/service/processing/process"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_query_service_pr_177_q, (v) => { api_query_service_pr_177_data.value = v ?? []; });
const api_designer_stateme_227_data = ref<any[]>([]);
const { data: api_designer_stateme_227_q } = useQuery({queryKey: ['api_designer_stateme_227', '/jaxrs/query/assemble/designer/statement/list/manage'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/assemble/designer/statement/list/manage"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_designer_stateme_227_q, (v) => { api_designer_stateme_227_data.value = v ?? []; });


// Confirmation dialog (replaces window.confirm)
function confirmMsg(msg: string): Promise<boolean> {
  return new Promise(resolve => {
    const overlay = document.createElement('div')
    overlay.style.cssText = 'position:fixed;inset:0;background:rgba(0,0,0,.6);z-index:10000;display:flex;align-items:center;justify-content:center'
    const box = document.createElement('div')
    box.style.cssText = 'background:var(--bg-surface);border:1px solid var(--border-color);border-radius:var(--radius-lg);padding:24px;max-width:360px;width:90%;display:flex;flex-direction:column;gap:16px'
    box.innerHTML = '<p style="margin:0;color:var(--text-primary);font-size:14px">' + msg + '</p>' +
      '<div style="display:flex;gap:8px;justify-content:flex-end">' +
      '<button class="tc-cancel" style="padding:6px 16px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:transparent;color:var(--text-primary);cursor:pointer">取消</button>' +
      '<button class="tc-ok" style="padding:6px 16px;border-radius:var(--radius-md);border:none;background:var(--color-primary);color:#000;cursor:pointer;font-weight:600">确认</button>' +
      '</div>'
    overlay.appendChild(box)
    document.body.appendChild(overlay)
    const ok = () => { overlay.remove(); resolve(true) }
    const cancel = () => { overlay.remove(); resolve(false) }
    box.querySelector('.tc-ok').addEventListener('click', ok)
    box.querySelector('.tc-cancel').addEventListener('click', cancel)
    overlay.addEventListener('click', e => { if (e.target === overlay) cancel() })
  })
}


const api_designer_input_p_186_data = ref<any[]>([]);
const { data: api_designer_input_p_186_q } = useQuery({queryKey: ['api_designer_input_p_186', '/jaxrs/query/assemble/designer/input/prepare/create'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/assemble/designer/input/prepare/create"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_designer_input_p_186_q, (v) => { api_designer_input_p_186_data.value = v ?? []; });
const api_designer_input_p_638_data = ref<any[]>([]);
const { data: api_designer_input_p_638_q } = useQuery({queryKey: ['api_designer_input_p_638', '/jaxrs/query/assemble/designer/input/prepare/cover'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/assemble/designer/input/prepare/cover"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_designer_input_p_638_q, (v) => { api_designer_input_p_638_data.value = v ?? []; });
const api_processing_table_695_data = ref<any[]>([]);
const { data: api_processing_table_695_q } = useQuery({queryKey: ['api_processing_table_695', '/jaxrs/query/service/processing/table/u2table/insert'], queryFn: async () => { try { const r = await api.get("/jaxrs/query/service/processing/table/u2table/insert"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_processing_table_695_q, (v) => { api_processing_table_695_data.value = v ?? []; });
const api_query_service_pr_699_data = ref<any[]>([]);
const { data: api_query_service_pr_699_q } = useQuery({queryKey: ['api_query_service_pr_699', '/jaxrs/query_service_processing'], queryFn: async () => { try { const r = await api.get("/jaxrs/query_service_processing"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_query_service_pr_699_q, (v) => { api_query_service_pr_699_data.value = v ?? []; });

</script>

<style scoped>
.query-designer { display: flex; flex-direction: column; gap: 16px; height: 100% }
.view-header { display: flex; align-items: center; justify-content: space-between; padding: 16px 24px }
.view-header h1 { font-family: 'Orbitron', sans-serif; font-size: 20px; color: var(--color-primary); margin: 0; text-shadow: 0 0 15px var(--color-primary-glow) }
.subtitle { font-size: 12px; color: var(--text-muted); margin: 4px 0 0; font-family: 'JetBrains Mono', monospace }
.btn-create { padding: 8px 20px; background: var(--color-primary); color: #000; border: none; border-radius: var(--radius-md); font-size: 13px; cursor: pointer; font-weight: 600 }
.split-layout { flex: 1; display: grid; grid-template-columns: 280px 1fr; gap: 16px; overflow: hidden }
.sidebar { padding: 16px; display: flex; flex-direction: column; gap: 12px; overflow: hidden }
.sidebar-header { display: flex; gap: 8px }
.search-box { flex: 1; display: flex; align-items: center; gap: 6px; background: var(--bg-elevated); border: 1px solid var(--border-subtle); border-radius: var(--radius-md); padding: 4px 10px }
.si { color: var(--text-muted); font-size: 14px }
.sinput { background: none; border: none; outline: none; color: var(--text-primary); font-size: 13px; flex: 1 }
.btn-sm { padding: 4px 10px; background: var(--bg-elevated); border: 1px solid var(--border-subtle); border-radius: var(--radius-sm); color: var(--text-secondary); font-size: 12px; cursor: pointer }
.query-list { flex: 1; overflow-y: auto; display: flex; flex-direction: column; gap: 6px }
.qi { display: flex; align-items: center; gap: 10px; padding: 10px 12px; border-radius: var(--radius-md); cursor: pointer; transition: all var(--transition-fast); border: 1px solid transparent }
.qi:hover { background: var(--color-primary-soft); border-color: var(--border-active) }
.qi.active { background: var(--color-primary-soft); border-color: var(--color-primary) }
.qicon { font-size: 18px; flex-shrink: 0 }
.qi-info { flex: 1; min-width: 0 }
.qi-name { font-size: 13px; font-weight: 500; color: var(--text-primary); overflow: hidden; text-overflow: ellipsis; white-space: nowrap }
.qi-meta { font-size: 11px; color: var(--text-muted); margin-top: 2px }
.qdel { background: none; border: none; color: var(--text-muted); cursor: pointer; font-size: 14px; padding: 2px 6px; border-radius: var(--radius-sm) }
.qdel:hover { background: rgba(239,68,68,.15); color: var(--color-error) }
.main-panel { padding: 16px; overflow-y: auto; display: flex; flex-direction: column; gap: 16px }
.designer-header { display: flex; justify-content: space-between; align-items: center; padding-bottom: 12px; border-bottom: 1px solid var(--border-subtle) }
.designer-header h2 { font-size: 16px; color: var(--text-primary); margin: 0; font-family: 'Orbitron', sans-serif }
.dh-actions { display: flex; gap: 8px }
.btn-run { padding: 6px 16px; background: var(--color-success); color: #000; border: none; border-radius: var(--radius-md); font-size: 13px; cursor: pointer; font-weight: 600 }
.btn-edit { padding: 6px 12px; background: transparent; border: 1px solid var(--color-primary); color: var(--color-primary); border-radius: var(--radius-md); font-size: 13px; cursor: pointer }
.btn-del { padding: 6px 12px; background: transparent; border: 1px solid var(--color-error); color: var(--color-error); border-radius: var(--radius-md); font-size: 13px; cursor: pointer }
.condition-panel { padding: 16px; background: var(--bg-elevated); border-radius: var(--radius-md); display: flex; flex-direction: column; gap: 10px }
.condition-panel h3 { font-size: 14px; color: var(--color-primary); margin: 0 }
.condition-row { display: flex; gap: 8px; align-items: center }
.cselect { background: var(--bg-surface); border: 1px solid var(--border-subtle); border-radius: var(--radius-sm); color: var(--text-primary); padding: 6px 8px; font-size: 13px; min-width: 120px }
.cinput { flex: 1; background: var(--bg-surface); border: 1px solid var(--border-subtle); border-radius: var(--radius-sm); color: var(--text-primary); padding: 6px 10px; font-size: 13px }
.cinput:focus, .cselect:focus { outline: none; border-color: var(--color-primary) }
.btn-rm { background: none; border: none; color: var(--text-muted); cursor: pointer; font-size: 16px }
.btn-rm:hover { color: var(--color-error) }
.btn-add-cond { padding: 6px 12px; background: transparent; border: 1px dashed var(--border-subtle); color: var(--text-muted); border-radius: var(--radius-sm); font-size: 12px; cursor: pointer; align-self: flex-start }
.btn-add-cond:hover { border-color: var(--color-primary); color: var(--color-primary) }
.result-panel { flex: 1; display: flex; flex-direction: column; overflow: hidden; min-height: 0 }
.rp-header { display: flex; justify-content: space-between; align-items: center; padding-bottom: 8px; border-bottom: 1px solid var(--border-subtle); margin-bottom: 8px }
.rp-header span:first-child { font-size: 14px; font-weight: 600; color: var(--text-primary) }
.rp-count { font-size: 12px; color: var(--text-muted) }
.rt { flex: 1; overflow: auto }
.rth { display: grid; gap: 0; font-size: 12px; font-weight: 600; color: var(--color-primary); background: var(--bg-elevated); border-radius: var(--radius-sm); overflow: hidden; margin-bottom: 4px }
.rh { padding: 8px 12px; border-right: 1px solid var(--border-subtle) }
.tr { display: grid; font-size: 13px; color: var(--text-secondary); border-bottom: 1px solid var(--border-subtle) }
.tr:hover { background: var(--color-primary-soft) }
.rc { padding: 6px 12px; border-right: 1px solid var(--border-subtle); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; max-width: 200px }
.no-select { flex: 1; display: flex; flex-direction: column; align-items: center; justify-content: center; color: var(--text-muted); gap: 12px }
.no-select .ns-icon { font-size: 64px; opacity: 0.4 }
.no-select h2 { font-family: 'Orbitron', sans-serif; color: var(--color-primary); font-size: 18px; margin: 0 }
.ls, .empty-l, .empty-r { display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 40px; color: var(--text-muted); gap: 12px }
.sk { height: 36px; border-radius: var(--radius-md); background: var(--bg-elevated); animation: pulse 1.2s ease-in-out infinite }
@keyframes pulse { 0%,100%{opacity:.4}50%{opacity:.8} }
.ei { font-size: 48px; opacity: 0.4 }
.modal-overlay { position: fixed; inset: 0; background: rgba(0,0,0,.7); display: flex; align-items: center; justify-content: center; z-index: 100 }
.modal { background: var(--bg-surface); border: 1px solid var(--border-subtle); border-radius: var(--radius-lg); padding: 24px; width: 500px; max-width: 90vw; display: flex; flex-direction: column; gap: 14px }
.modal h3 { font-family: 'Orbitron', sans-serif; color: var(--color-primary); margin: 0; font-size: 15px }
.fg { display: flex; flex-direction: column; gap: 6px }
.fg label { font-size: 13px; color: var(--text-muted) }
.fi, .fta { background: var(--bg-elevated); border: 1px solid var(--border-subtle); border-radius: var(--radius-md); color: var(--text-primary); padding: 10px 12px; font-size: 14px }
.fi:focus, .fta:focus { outline: none; border-color: var(--color-primary) }
.fta { resize: vertical; min-height: 100px; font-family: 'JetBrains Mono', monospace }
.mf { display: flex; justify-content: flex-end; gap: 8px }
.bc { padding: 8px 20px; background: transparent; border: 1px solid var(--border-subtle); color: var(--text-secondary); border-radius: var(--radius-md); cursor: pointer }
.bs { padding: 8px 20px; background: var(--color-primary); color: #000; border: none; border-radius: var(--radius-md); font-size: 13px; cursor: pointer; font-weight: 600 }
.bs:disabled { opacity: 0.5; cursor: not-allowed }
@media(max-width:768px){.split-layout{grid-template-columns:1fr}}

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

