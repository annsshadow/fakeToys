<template>
  <div class="qm">
    <!-- Header -->
    <div class="qm-header glass-card">
      <div class="qm-title">
        <h1>查询管理</h1>
        <p class="subtitle">/jaxrs/query/assemble/designer/* — 查询定义、视图、表格、SQL、统计、导入</p>
      </div>
      <div class="qm-actions">
        <button class="btn" @click="showCreate=true">+ 新建查询</button>
        <button class="btn btn-outline" @click="refresh">🔄 刷新</button>
      </div>
    </div>

    <div class="qm-body">
      <!-- Left: Query List -->
      <aside class="qm-sidebar glass-card">
        <div class="qs-search">
          <input v-model="search" placeholder="搜索查询..." class="qs-input" />
        </div>
        <div class="qs-tabs">
          <button :class="{active: qsTab==='all'}" @click="qsTab='all'">全部</button>
          <button :class="{active: qsTab==='view'}" @click="qsTab='view'">视图</button>
          <button :class="{active: qsTab==='stat'}" @click="qsTab='stat'">统计</button>
          <button :class="{active: qsTab==='statement'}" @click="qsTab='statement'">SQL</button>
        </div>
        <div class="qs-list">
          <div v-if="qsLoading" class="qs-loading">加载中...</div>
          <div v-else-if="qsFiltered.length===0" class="qs-empty">暂无查询定义</div>
          <div v-for="q in qsFiltered" :key="q.id" class="qs-item"
            :class="{active:selected?.id===q.id}"
            @click="selectQuery(q)">
            <div class="qi-icon">{{ queryIcon(q) }}</div>
            <div class="qi-info">
              <div class="qi-name">{{ q.name||q.queryName||'未命名' }}</div>
              <div class="qi-meta">{{ q.category||q.entityCategory||'通用' }}</div>
            </div>
            <div class="qi-time">{{ fmtTime(q.updateTime) }}</div>
          </div>
        </div>
      </aside>

      <!-- Right: Editor Area -->
      <main class="qm-main glass-card">
        <template v-if="selected">
          <!-- Mode Tabs -->
          <div class="mode-tabs">
            <button :class="{active: mode==='sql'}" @click="mode='sql'">📝 SQL编辑</button>
            <button :class="{active: mode==='view'}" @click="mode='view'">👁 视图配置</button>
            <button :class="{active: mode==='table'}" @click="mode='table'">📊 表格设计</button>
            <button :class="{active: mode==='stat'}" @click="mode='stat'">📈 统计分析</button>
            <button :class="{active: mode==='import'}" @click="mode='import'" v-if="selected.importModel">📥 导入配置</button>
          </div>

          <!-- SQL Editor Mode -->
          <div v-if="mode==='sql'" class="editor-mode">
            <div class="em-header">
              <span class="em-title">{{ selected.name||selected.queryName }}</span>
              <div class="em-actions">
                <button class="btn-sm" @click="runQuery">▶ 执行</button>
                <button class="btn-sm btn-del" @click="deleteQuery">🗑 删除</button>
              </div>
            </div>
            <textarea v-model="sqlText" class="sql-area" placeholder="输入SQL语句，支持SELECT查询..." spellcheck="false"></textarea>
            <div class="em-status">{{ sqlStatus }}</div>
          </div>

          <!-- View Config Mode -->
          <div v-if="mode==='view'" class="editor-mode">
            <div class="em-header"><span class="em-title">视图配置</span></div>
            <div class="view-config">
              <div class="vc-row"><label>视图列</label><input v-model="viewConfig.columns" placeholder="逗号分隔列名" class="vc-input" /></div>
              <div class="vc-row"><label>过滤条件</label><input v-model="viewConfig.filter" placeholder="WHERE条件" class="vc-input" /></div>
              <div class="vc-row"><label>排序</label><input v-model="viewConfig.sort" placeholder="ORDER BY" class="vc-input" /></div>
              <div class="vc-row"><label>分页</label><input v-model.number="viewConfig.pageSize" type="number" class="vc-input" min="10" max="500" /></div>
              <button class="btn-sm" @click="applyViewConfig">应用配置</button>
            </div>
          </div>

          <!-- Table Design Mode -->
          <div v-if="mode==='table'" class="editor-mode">
            <div class="em-header"><span class="em-title">表格设计</span></div>
            <div class="table-config">
              <div class="tc-row"><label>表格主题</label>
                <select v-model="tableConfig.theme" class="tc-select">
                  <option value="default">默认</option>
                  <option value="striped">条纹</option>
                  <option value="bordered">边框</option>
                </select>
              </div>
              <div class="tc-row"><label>可排序</label><input type="checkbox" v-model="tableConfig.sortable" /></div>
              <div class="tc-row"><label>可筛选</label><input type="checkbox" v-model="tableConfig.filterable" /></div>
              <div class="tc-row"><label>行选择</label><input type="checkbox" v-model="tableConfig.rowSelect" /></div>
            </div>
          </div>

          <!-- Stats Mode -->
          <div v-if="mode==='stat'" class="editor-mode">
            <div class="em-header"><span class="em-title">统计分析</span></div>
            <div class="stat-config">
              <div class="sc-row"><label>维度字段</label><input v-model="statConfig.dimension" placeholder="GROUP BY 字段" class="sc-input" /></div>
              <div class="sc-row"><label>统计指标</label><input v-model="statConfig.metric" placeholder="COUNT/SUM/AVG" class="sc-input" /></div>
              <div class="sc-row"><label>图表类型</label>
                <select v-model="statConfig.chartType" class="sc-select">
                  <option value="bar">柱状图</option>
                  <option value="pie">饼图</option>
                  <option value="line">折线图</option>
                  <option value="table">表格</option>
                </select>
              </div>
              <button class="btn-sm" @click="runStats">执行分析</button>
            </div>
            <div v-if="statResult" class="stat-result">
              <div v-for="(v,k) in statResult" :key="k" class="sr-item">
                <span class="sr-key">{{ k }}</span><span class="sr-val">{{ v }}</span>
              </div>
            </div>
          </div>

          <!-- Import Mode -->
          <div v-if="mode==='import'" class="editor-mode">
            <div class="em-header"><span class="em-title">导入配置</span></div>
            <div class="import-config">
              <div class="ic-row"><label>源文件</label><input type="file" class="ic-file" accept=".csv,.xlsx,.xls" /></div>
              <div class="ic-row"><label>分隔符</label>
                <select v-model="importConfig.delimiter" class="ic-select">
                  <option value=",">逗号(,)</option>
                  <option value="\t">制表符</option>
                  <option value=";">分号(；)</option>
                </select>
              </div>
              <button class="btn-sm" @click="importData">导入</button>
            </div>
          </div>

          <!-- Results -->
          <div class="results-area" v-if="resultData.length > 0 || resultLoading">
            <div class="ra-header">
              <span>执行结果</span>
              <span class="ra-count">{{ resultData.length }} 行</span>
              <button class="btn-sm" @click="exportResults">📥 导出</button>
            </div>
            <div class="ra-content" :class="{loading: resultLoading}">
              <div v-if="resultLoading" class="ra-loading">执行中...</div>
              <table v-else class="res-table">
                <thead><tr><th v-for="h in resultHeaders" :key="h">{{ h }}</th></tr></thead>
                <tbody>
                  <tr v-for="(row,i) in resultData" :key="i">
                    <td v-for="h in resultHeaders" :key="h" class="mono">{{ row[h] ?? '—' }}</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </template>
        <div v-else class="qm-empty">
          <div class="qe-icon">📊</div>
          <h2>选择查询定义</h2>
          <p>点击左侧查询列表开始编辑或执行</p>
        </div>
      </main>
    </div>

    <!-- Create Modal -->
    <div v-if="showCreate" class="modal-overlay" @click.self="showCreate=false">
      <div class="modal glass-card">
        <h3>新建查询</h3>
        <div class="form-group"><label>名称</label><input v-model="nform.name" class="form-input" placeholder="查询名称" /></div>
        <div class="form-group"><label>SQL</label><textarea v-model="nform.sql" class="form-textarea" rows="5" placeholder="SELECT * FROM ..."></textarea></div>
        <div class="form-group"><label>分类</label><input v-model="nform.category" class="form-input" placeholder="如：query, stat, admin" /></div>
        <div class="modal-actions">
          <button class="btn-cancel" @click="showCreate=false">取消</button>
          <button class="btn-save" :disabled="!nform.name" @click="createQuery">创建</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useQuery, useMutation, useQueryClient } from '@tanstack/vue-query'
import { api } from '@oa4rust/sdk'

interface QueryDef {
  id: string; name?: string; queryName?: string; icon?: string
  category?: string; entityCategory?: string; sql?: string
  updateTime?: string; createTime?: string; importModel?: boolean
}

const search = ref(''), qsTab = ref('all'), qsLoading = ref(false)
const selected = ref<QueryDef|null>(null), mode = ref('sql')
const sqlText = ref(''), sqlStatus = ref('')
const resultData = ref<any[]>([]), resultHeaders = ref<string[]>([]), resultLoading = ref(false)
const showCreate = ref(false), nform = ref({ name: '', sql: '', category: '' })

const viewConfig = ref({ columns: '', filter: '', sort: '', pageSize: 100 })
const tableConfig = ref({ theme: 'default', sortable: true, filterable: false, rowSelect: false })
const statConfig = ref({ dimension: '', metric: '', chartType: 'bar' })
const statResult = ref<Record<string,number>|null>(null)
const importConfig = ref({ delimiter: ',' })

const qc = useQueryClient()
const { data } = useQuery({ queryKey: ['qm','list'], queryFn: async () => { qsLoading.value = true; try { const r: any = await api.get('/jaxrs/query/assemble/designer/list'); return r?.data ?? [] } finally { qsLoading.value = false } } })
const queries = ref<QueryDef[]>(data.value ?? [])

const qsFiltered = computed(() => {
  let list = queries.value
  if (search.value) list = list.filter(q => (q.name||'').toLowerCase().includes(search.value.toLowerCase()))
  if (qsTab.value !== 'all') list = list.filter(q => (q.category||'').toLowerCase() === qsTab.value)
  return list
})

function queryIcon(q: QueryDef) {
  if (q.icon) return q.icon
  const cat = (q.category||'').toLowerCase()
  if (cat==='stat') return '📈'
  if (cat==='view') return '👁'
  if (cat==='statement') return '💻'
  return '📄'
}

function selectQuery(q: QueryDef) {
  selected.value = q
  sqlText.value = q.sql ?? ''
  resultData.value = []
  resultHeaders.value = []
  statResult.value = null
}

async function runQuery() {
  if (!sqlText.value.trim() || !selected.value) return
  resultLoading.value = true
  try {
    const r: any = await api.post('/jaxrs/query/assemble/designer/execute', { id: selected.value.id, sql: sqlText.value })
    resultData.value = r?.data?.list ?? r?.data ?? []
    resultHeaders.value = resultData.value.length > 0 ? Object.keys(resultData.value[0]) : []
    sqlStatus.value = `执行成功: ${resultData.value.length} 行`
  } catch (e: any) {
    sqlStatus.value = '执行失败: ' + (e?.message ?? '未知错误')
    resultData.value = []; resultHeaders.value = []
  } finally { resultLoading.value = false }
}

async function runStats() {
  if (!selected.value) return
  resultLoading.value = true
  try {
    const r: any = await api.post('/jaxrs/query/assemble/designer/stat/do', { id: selected.value.id, dimension: statConfig.value.dimension, metric: statConfig.value.metric })
    statResult.value = r?.data ?? null
    resultData.value = []
  } catch { statResult.value = null } finally { resultLoading.value = false }
}

const delM = useMutation({
  mutationFn: (id: string) => api.delete(`/jaxrs/query/assemble/designer/delete/${id}`),
  onSuccess: () => { qc.invalidateQueries({ queryKey: ['qm','list'] }); if (selected.value?.id) selected.value = null }
})
function deleteQuery() {
  if (!selected.value || !confirm('确定删除此查询？')) return
  delM.mutate(selected.value.id)
}

const cm = useMutation({
  mutationFn: () => api.post('/jaxrs/query/assemble/designer/create', { name: nform.value.name, sql: nform.value.sql, category: nform.value.category }),
  onSuccess: () => { showCreate.value = false; qc.invalidateQueries({ queryKey: ['qm','list'] }) }
})
function createQuery() { if (nform.value.name) cm.mutate() }

function refresh() { qc.invalidateQueries({ queryKey: ['qm','list'] }) }
function applyViewConfig() { /* apply config to current query */ }
function importData() { alert('导入功能开发中') }
function exportResults() {
  if (!resultData.value.length) return
  const header = resultHeaders.value.join(',')
  const rows = resultData.value.map(r =>
    resultHeaders.value.map(h => '"' + String(r[h] ?? '').replace(/"/g, '""') + '"').join(',')
  )
  const blob = new Blob([header + '\n' + rows.join('\n')], { type: 'text/csv;charset=utf-8' })
  const a = document.createElement('a')
  a.href = URL.createObjectURL(blob)
  a.download = 'query_result.csv'
  a.click()
}
function fmtTime(t?: string) { if (!t) return ''; try { return new Date(t).toLocaleString('zh-CN',{month:'2-digit',day:'2-digit',hour:'2-digit',minute:'2-digit'}) } catch { return String(t) } }
</script>

<style scoped>
.qm{display:flex;flex-direction:column;gap:0;height:100%}
.qm-header{display:flex;align-items:center;justify-content:space-between;padding:12px 20px;flex-shrink:0}
.qm-title h1{font-family:'Orbitron',sans-serif;font-size:18px;color:var(--color-primary);margin:0 0 2px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:11px;color:var(--text-muted);margin:0;font-family:'JetBrains Mono',monospace}
.qm-actions{display:flex;gap:8px}
.btn{padding:6px 14px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);cursor:pointer;font-size:13px}
.btn:hover{border-color:var(--color-primary);color:var(--color-primary)}
.btn-outline{background:transparent}
.qm-body{display:flex;flex:1;gap:0;min-height:0;overflow:hidden}
/* Sidebar */
.qm-sidebar{width:240px;flex-shrink:0;display:flex;flex-direction:column;border-right:1px solid var(--border-color)}
.qs-search{padding:8px}
.qs-input{width:100%;padding:6px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:12px;outline:none;box-sizing:border-box}
.qs-tabs{display:flex;gap:4px;padding:4px 8px;border-bottom:1px solid var(--border-color)}
.qs-tabs button{flex:1;padding:4px;font-size:11px;border-radius:var(--radius-sm);border:1px solid transparent;background:transparent;color:var(--text-muted);cursor:pointer}
.qs-tabs button.active{border-color:var(--color-primary);color:var(--color-primary);background:var(--color-primary-soft)}
.qs-list{flex:1;overflow-y:auto;padding:4px}
.qs-loading,.qs-empty{padding:16px;text-align:center;color:var(--text-muted);font-size:12px}
.qs-item{display:flex;align-items:center;gap:8px;padding:8px;border-radius:var(--radius-sm);cursor:pointer;margin-bottom:2px}
.qs-item:hover{background:var(--bg-hover)}
.qs-item.active{background:var(--color-primary-soft);border-left:3px solid var(--color-primary)}
.qi-icon{font-size:18px;flex-shrink:0}
.qi-info{flex:1;min-width:0}
.qi-name{font-size:13px;color:var(--text-primary);overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.qi-meta{font-size:10px;color:var(--text-muted);font-family:'JetBrains Mono',monospace}
.qi-time{font-size:10px;color:var(--text-muted);flex-shrink:0}
/* Main */
.qm-main{flex:1;display:flex;flex-direction:column;min-width:0;padding:12px;overflow:hidden}
.qm-empty{display:flex;flex-direction:column;align-items:center;justify-content:center;flex:1;color:var(--text-muted);gap:12px}
.qe-icon{font-size:64px;opacity:0.3}
.mode-tabs{display:flex;gap:4px;margin-bottom:12px;flex-wrap:wrap}
.mode-tabs button{padding:6px 14px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-muted);cursor:pointer;font-size:12px}
.mode-tabs button.active{background:var(--color-primary-soft);border-color:var(--color-primary);color:var(--color-primary)}
.editor-mode{flex:1;display:flex;flex-direction:column;gap:8px;overflow:hidden}
.em-header{display:flex;align-items:center;justify-content:space-between;padding-bottom:8px;border-bottom:1px solid var(--border-color)}
.em-title{font-size:14px;font-weight:600;color:var(--text-primary)}
.em-actions{display:flex;gap:6px}
.btn-sm{padding:4px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);cursor:pointer;font-size:12px}
.btn-del{border-color:var(--color-danger);color:var(--color-danger)}
.sql-area{flex:1;min-height:150px;padding:12px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-terminal);color:#7fdbca;font-family:'Fira Code','JetBrains Mono',monospace;font-size:13px;outline:none;resize:none;line-height:1.6}
.em-status{font-size:12px;color:var(--text-muted);padding:4px 0}
.view-config,.table-config,.stat-config,.import-config{display:flex;flex-direction:column;gap:8px;padding:12px;background:var(--bg-elevated);border-radius:var(--radius-md)}
.vc-row,.tc-row,.sc-row,.ic-row{display:flex;align-items:center;gap:8px}
.vc-row label,.tc-row label,.sc-row label,.ic-row label{font-size:12px;color:var(--text-muted);width:80px;flex-shrink:0}
.vc-input,.sc-input{flex:1;padding:6px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-surface);color:var(--text-primary);font-size:13px;outline:none}
.tc-select,.sc-select,.ic-select{padding:6px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-surface);color:var(--text-primary);font-size:13px}
.ic-file{flex:1;font-size:12px;color:var(--text-muted)}
.results-area{margin-top:12px;border-top:1px solid var(--border-color);padding-top:12px;flex:1;display:flex;flex-direction:column;overflow:hidden}
.ra-header{display:flex;align-items:center;gap:8px;margin-bottom:8px;font-size:13px;font-weight:600;color:var(--color-primary)}
.ra-count{font-size:11px;color:var(--text-muted);margin-left:auto}
.ra-content{flex:1;overflow:auto;position:relative}
.ra-content.loading .res-table{opacity:0.3}
.ra-loading{position:absolute;inset:0;display:flex;align-items:center;justify-content:center;color:var(--text-muted)}
.res-table{width:100%;border-collapse:collapse;font-size:12px}
.res-table th{padding:6px 10px;text-align:left;border-bottom:1px solid var(--border-color);color:var(--text-muted);font-weight:600;font-size:11px;text-transform:uppercase;position:sticky;top:0;background:var(--bg-surface)}
.res-table td{padding:5px 10px;border-bottom:1px solid var(--border-subtle);color:var(--text-primary);max-width:200px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.res-table tr:hover td{background:var(--bg-hover)}
.mono{font-family:'JetBrains Mono',monospace;font-size:11px}
.stat-result{display:flex;gap:12px;flex-wrap:wrap;margin-top:8px}
.sr-item{padding:8px 16px;border-radius:var(--radius-md);background:var(--color-primary-soft)}
.sr-key{font-size:12px;color:var(--text-muted);margin-right:8px}
.sr-val{font-size:16px;font-weight:600;color:var(--color-primary)}
/* Modal */
.modal-overlay{position:fixed;inset:0;background:rgba(0,0,0,0.6);display:flex;align-items:center;justify-content:center;z-index:200}
.modal{padding:24px;width:560px;max-width:90vw;display:flex;flex-direction:column;gap:12px}
.modal h3{font-size:16px;color:var(--color-primary);margin:0}
.form-group{display:flex;flex-direction:column;gap:4px}
.form-group label{font-size:12px;color:var(--text-muted)}
.form-input,.form-textarea{padding:8px 12px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);outline:none;font-size:13px;box-sizing:border-box}
.form-textarea{resize:vertical;font-family:'JetBrains Mono',monospace}
.modal-actions{display:flex;justify-content:flex-end;gap:8px;margin-top:8px}
.btn-cancel{padding:8px 16px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:transparent;color:var(--text-primary);cursor:pointer}
.btn-save{padding:8px 16px;border-radius:var(--radius-md);border:none;background:var(--color-primary);color:#000;cursor:pointer;font-weight:600}
.btn-save:disabled{opacity:0.4;cursor:not-allowed}
</style>
