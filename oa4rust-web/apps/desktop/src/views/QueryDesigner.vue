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
import { api } from '@oa4rust/sdk'
import { computed, ref } from 'vue'
import { toast } from '../utils/toast'

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
const conditions = ref<{ field: string; op: string; value: string }[]>([])
const resultData = ref<Record<string, unknown>[]>([])
const rLoading = ref(false)
const showModal = ref(false)
const editingQuery = ref<QueryDef | null>(null)
const mform = ref({ name: '', category: '', sql: '' })

const resultHeaders = computed(() => (resultData.value.length > 0 ? Object.keys(resultData.value[0]) : []))

const entityFields = computed(() => {
  // Extract from SQL or use defaults
  if (selected.value?.sql) {
    const matches = selected.value.sql.match(/(\w+)\s+AS\s+(\w+)/gi) || []
    return [...new Set(matches.map((m) => m.split(/\s+/).pop()))]
  }
  return ['id', 'name', 'flag', 'status', 'createdAt']
})

async function loadQueries() {
  qLoading.value = true
  try {
    const r = await api.get('/jaxrs/query/assemble/designer/list/all')
    queries.value = r.data ?? []
    if (keyword.value) {
      queries.value = queries.value.filter((q) =>
        (q.name || q.queryName || '').toLowerCase().includes(keyword.value.toLowerCase()),
      )
    }
  } catch {
    queries.value = []
  } finally {
    qLoading.value = false
  }
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
  if (!mform.value.name.trim()) {
    toast.info('请输入查询名称')
    return
  }
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
  } catch (e: any) {
    toast.error('保存失败: : ' + (e?.message ?? ''))
  }
}

async function runQuery() {
  if (!selected.value) return
  rLoading.value = true
  resultData.value = []
  try {
    const params: Record<string, unknown> = {}
    if (conditions.value.length > 0) {
      params.conditions = conditions.value.filter((c) => c.field && c.value)
    }
    const r = await api.post(`/jaxrs/query/assemble/designer/query`, {
      queryId: selected.value.id,
      ...params,
    })
    resultData.value = r.data?.list ?? r.data ?? []
  } catch (e: any) {
    toast.error('执行失败: : ' + (e?.message ?? ''))
  } finally {
    rLoading.value = false
  }
}

async function deleteQuery(q: QueryDef) {
  if (!confirmMsg(`删除查询「${q.name || q.id}」？`)) return
  try {
    await api.delete(`/jaxrs/query/assemble/designer/delete/${q.id}`)
    if (selected.value?.id === q.id) selected.value = null
    queries.value = queries.value.filter((x) => x.id !== q.id)
  } catch (e: any) {
    toast.error('删除失败: : ' + (e?.message ?? ''))
  }
}

loadQueries()

// --- SQL Editor ---
const showSqlEditor = ref(false)
const sqlEditorForm = ref({ name: '', sql: '', category: '', desc: '' })
function saveSqlQuery() {
  if (!sqlEditorForm.value.name.trim()) {
    toast.info('请输入查询名称')
    return
  }
  // Save via API
  showModal.value = true
  editingQuery.value = null
  mform.value = { name: sqlEditorForm.value.name, category: sqlEditorForm.value.category, sql: sqlEditorForm.value.sql }
  showSqlEditor.value = false
}

// --- Filter Builder ---
const showFilterBuilder = ref(false)
const filterRules = ref<
  Array<{ field: string; op: string; value: string; valueFrom?: string; valueTo?: string; connector: string }>
>([])
const allFields = computed(() => {
  if (selected.value?.sql) {
    const matches = selected.value.sql.match(/(\w+)\s+[A-Z]/gi) || []
    return [...new Set(matches.map((m) => m.split(/\s+/)[0]))]
  }
  return ['id', 'name', 'flag', 'status', 'createdAt', 'updatedAt']
})
const generatedFilterWhere = computed(() => {
  const valid = filterRules.value.filter((r) => r.field && r.value)
  if (!valid.length) return ''
  return valid
    .map((r) => {
      if (r.op === 'between') return `${r.field} BETWEEN ${r.valueFrom || "''"} AND ${r.valueTo || "''"}`
      if (r.op === 'isnull') return `${r.field} IS NULL`
      return `${r.field} ${r.op} '${r.value}'`
    })
    .join(` ${valid[0]?.connector || 'AND'} `)
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
const chartType = ref('bar'),
  chartX = ref(''),
  chartY = ref('')
const chartRendered = ref(false)
const chartColors = ['#3b82f6', '#10b981', '#f59e0b', '#ef4444', '#8b5cf6', '#ec4899', '#06b6d4', '#f97316']
const chartData = ref<Array<{ label: string; value: number; h: number }>>([])
const chartStats = ref<{ count: number; max: number; min: number; avg: number } | null>(null)
function renderChart() {
  if (!resultData.value.length || !chartX.value || !chartY.value) return
  const map = new Map<string, number>()
  resultData.value.forEach((r) => {
    const key = String(r[chartX.value])
    const val = Number(r[chartY.value]) || 0
    map.set(key, (map.get(key) || 0) + val)
  })
  const entries = [...map.entries()].sort((a, b) => b[1] - a[1]).slice(0, 20)
  const maxVal = Math.max(1, ...entries.map(([, v]) => v))
  const nums = entries.map(([, v]) => v)
  chartData.value = entries.map(([label, value], i) => ({ label, value, h: Math.round((value / maxVal) * 150) }))
  chartStats.value = {
    count: resultData.value.length,
    max: Math.max(...nums),
    min: Math.min(...nums),
    avg: Math.round(nums.reduce((a: number, b: number) => a + b, 0) / nums.length),
  }
  chartRendered.value = true
}

// --- History ---
const showHistory = ref(false)
const execHistory = ref<Array<{ time: string; sql: string; duration: number; rows: number; success: boolean }>>([])
function replayHistory(idx: number) {
  const h = execHistory.value[idx]
  if (h) {
    sql.value = h.sql
    runQuery()
  }
}
function copyHistorySql(idx: number) {
  const h = execHistory.value[idx]
  if (h) navigator.clipboard.writeText(h.sql)
}

// --- Import/Export ---
const showImportExport = ref(false)
const ieTab = ref<'export' | 'import'>('export')
const exportFmt = ref<'json' | 'csv' | 'sql'>('json')
const importJson = ref('')
const importMsg = ref<{ ok: boolean; txt: string } | null>(null)
function doExport() {
  const data = queries.value.map((q) => ({
    name: q.name || q.queryName,
    category: q.category || q.entityCategory,
    sql: q.sql,
  }))
  if (exportFmt.value === 'json') {
    downloadBlob(
      new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' }),
      'queries_' + new Date().toISOString().slice(0, 10) + '.json',
    )
  } else if (exportFmt.value === 'csv') {
    const csv =
      'name,category,sql\n' +
      data.map((d) => `"${d.name}","${d.category}","${(d.sql || '').replace(/"/g, '""')}"`).join('\n')
    downloadBlob(new Blob([csv], { type: 'text/csv' }), 'queries_' + new Date().toISOString().slice(0, 10) + '.csv')
  } else {
    const sqlStr = data.map((d) => `-- ${d.name}\n${d.sql}`).join('\n\n')
    downloadBlob(
      new Blob([sqlStr], { type: 'text/plain' }),
      'queries_' + new Date().toISOString().slice(0, 10) + '.sql',
    )
  }
  showImportExport.value = false
}
function downloadBlob(blob: Blob, filename: string) {
  const a = document.createElement('a')
  a.href = URL.createObjectURL(blob)
  a.download = filename
  a.click()
}
async function doImport() {
  if (!importJson.value.trim()) return
  try {
    const data = JSON.parse(importJson.value)
    if (!Array.isArray(data)) {
      importMsg.value = { ok: false, txt: '格式错误' }
      return
    }
    for (const q of data) {
      try {
        await api.post('/jaxrs/query/assemble/designer/create', q)
      } catch {}
    }
    importMsg.value = { ok: true, txt: `成功导入 ${data.length} 条` }
    loadQueries()
    showImportExport.value = false
  } catch (e: any) {
    importMsg.value = { ok: false, txt: '导入失败: ' + e.message }
  }
}

// --- Enhance runQuery with history tracking ---
const originalRunQuery = runQuery
async function runQueryEnhanced() {
  const t0 = Date.now()
  try {
    await originalRunQuery()
    execHistory.value.unshift({
      time: new Date().toLocaleTimeString('zh-CN'),
      sql: conditions.value.length ? JSON.stringify(conditions.value) : sql.value || '',
      duration: Date.now() - t0,
      rows: resultData.value.length,
      success: true,
    })
  } catch (e: any) {
    execHistory.value.unshift({
      time: new Date().toLocaleTimeString('zh-CN'),
      sql: '',
      duration: Date.now() - t0,
      rows: 0,
      success: false,
    })
    throw e
  }
}
</script>

<style scoped>
/* optimized */
</style>
