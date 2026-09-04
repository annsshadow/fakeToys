<template>
  <div class="smd">
    <!-- Header -->
    <div class="smd-header glass-card">
      <div class="smd-title">
        <h1>SQL语句设计器</h1>
        <p class="subtitle">/jaxrs/query/assemble/designer/*</p>
      </div>
      <div class="smd-actions">
        <button class="btn" @click="newStatement">+ 新建</button>
        <button class="btn btn-outline" @click="loadStatements">🔄 刷新</button>
        <button class="btn btn-success" :disabled="!sql.trim()" @click="executeSQL">▶ 执行</button>
        <button class="btn btn-primary" :disabled="!currentStatement" @click="saveStatement">💾 保存</button>
      </div>
    </div>

    <div class="smd-body">
      <!-- Left: Statement List -->
      <aside class="smd-sidebar glass-card">
        <div class="sb-search">
          <input v-model="filter" placeholder="搜索语句..." class="sb-input" />
        </div>
        <div class="sb-tabs">
          <button :class="{active: filterTab==='all'}" @click="filterTab='all'">全部</button>
          <button :class="{active: filterTab==='recent'}" @click="filterTab='recent'">最近</button>
        </div>
        <div class="sb-list">
          <div v-if="loading" class="sb-loading">加载中...</div>
          <div v-else-if="filtered.length===0" class="sb-empty">暂无语句定义</div>
          <div v-for="s in filtered" :key="s.id" class="sb-item"
            :class="{active: currentStatement?.id===s.id}"
            @click="selectStatement(s)">
            <div class="si-icon">{{ s.icon || '📄' }}</div>
            <div class="si-info">
              <div class="si-name">{{ s.name||s.statementName||'未命名' }}</div>
              <div class="si-meta">{{ s.category||s.entityCategory||'通用' }} · {{ fmtTime(s.updateTime) }}</div>
            </div>
            <div class="si-actions">
              <button class="si-btn" @click.stop="editStatement(s)" title="编辑">✏</button>
              <button class="si-btn si-del" @click.stop="deleteStatement(s)" title="删除">🗑</button>
            </div>
          </div>
        </div>
      </aside>

      <!-- Center: SQL Editor -->
      <main class="smd-editor glass-card">
        <div class="editor-header">
          <input :value="currentStatement?.name" @input="currentStatement&&(currentStatement.name=$event.target.value)" placeholder="语句名称" class="stmt-name" :disabled="!currentStatement" />
          <select :value="currentStatement?.category" @change="currentStatement&&(currentStatement.category=$event.target.value)" class="stmt-category" :disabled="!currentStatement">
            <option value="">选择分类</option>
            <option value="query">查询</option>
            <option value="stat">统计</option>
            <option value="admin">管理</option>
            <option value="other">其他</option>
          </select>
        </div>
        <div class="editor-toolbar">
          <button class="tb-btn" @click="formatSQL" title="格式化">📐 格式化</button>
          <button class="tb-btn" @click="clearSQL" title="清空">🗑 清空</button>
          <span class="tb-info">{{ sqlLines }} 行 · {{ sql.length }} 字符</span>
        </div>
        <textarea v-model="sql" class="sql-editor" placeholder="在此输入SQL语句..." spellcheck="false"
          @keydown.ctrl.enter="executeSQL" @keydown.meta.enter="executeSQL"></textarea>
        <div class="editor-status">{{ statusText }}</div>
      </main>

      <!-- Right: Results -->
      <aside class="smd-results glass-card" v-if="hasResults">
        <div class="results-header">
          <span>执行结果</span>
          <span class="results-count">{{ resultData.length }} 行</span>
          <button class="btn-sm" @click="exportCSV">📥 导出CSV</button>
        </div>
        <div class="results-toolbar" v-if="resultData.length > 0">
          <input v-model="resultFilter" placeholder="筛选结果..." class="result-filter" />
        </div>
        <div class="results-grid">
          <div v-if="loadingResult" class="results-loading">执行中...</div>
          <div v-else-if="resultData.length===0" class="results-empty">点击「执行」运行SQL</div>
          <table v-else class="res-table">
            <thead>
              <tr>
                <th v-for="h in resultHeaders" :key="h" @click="sortResult(h)">{{ h }}</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="(row, ri) in sortedResult" :key="ri">
                <td v-for="h in resultHeaders" :key="h" class="mono">{{ row[h] ?? '—' }}</td>
              </tr>
            </tbody>
          </table>
        </div>
        <div class="results-pager" v-if="resultData.length > 0">
          <button class="pg-btn" :disabled="page<=1" @click="page--">‹</button>
          <span class="pg-info">第 {{ page }} / {{ totalPages }} 页</span>
          <button class="pg-btn" :disabled="page>=totalPages" @click="page++">›</button>
        </div>
      </aside>
    </div>

    <!-- New/Edit Modal -->
    <div v-if="showModal" class="modal-overlay" @click.self="showModal=false">
      <div class="modal glass-card">
        <h3>{{ editing?'编辑语句':'新建语句' }}</h3>
        <div class="form-group"><label>名称</label><input v-model="modalForm.name" class="form-input" placeholder="语句名称" /></div>
        <div class="form-group"><label>标识</label><input v-model="modalForm.flag" class="form-input" placeholder="唯一标识" /></div>
        <div class="form-group"><label>SQL</label><textarea v-model="modalForm.sql" class="form-textarea" rows="8" placeholder="SELECT * FROM ..."></textarea></div>
        <div class="form-group"><label>描述</label><input v-model="modalForm.desc" class="form-input" placeholder="可选描述" /></div>
        <div class="modal-actions">
          <button class="btn-cancel" @click="showModal=false">取消</button>
          <button class="btn-save" :disabled="!modalForm.name" @click="modalSave">保存</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useQuery, useMutation, useQueryClient } from '@tanstack/vue-query'
import { api } from '@oa4rust/sdk'

interface Stmt {
  id: string; name?: string; statementName?: string; flag?: string
  category?: string; entityCategory?: string; icon?: string
  sql?: string; desc?: string; updateTime?: string; createTime?: string
}

const loading = ref(false), loadingResult = ref(false)
const filter = ref(''), filterTab = ref<'all'|'recent'>('all')
const currentStatement = ref<Stmt|null>(null)
const sql = ref(''), sqlLines = computed(() => sql.value.split('\n').length)
const showModal = ref(false), editing = ref(false)
const modalForm = ref({ name: '', flag: '', sql: '', desc: '' })

// Results
const resultData = ref<any[]>([])
const resultHeaders = ref<string[]>([])
const resultFilter = ref('')
const sortCol = ref(''), sortAsc = ref(true)
const page = ref(1), pageSize = 50
const hasResults = ref(false)

const queryClient = useQueryClient()
const { data: stmts } = useQuery({
  queryKey: ['stmt', 'list'],
  queryFn: async () => { loading.value = true; try { const r: any = await api.get('/jaxrs/query/assemble/designer/list'); return r?.data ?? [] } finally { loading.value = false } }
})
const statements = ref<Stmt[]>(stmts.value ?? [])

const filtered = computed(() => {
  let list = statements.value
  if (filter.value) list = list.filter(s => (s.name||'').toLowerCase().includes(filter.value.toLowerCase()) || (s.flag||'').toLowerCase().includes(filter.value.toLowerCase()))
  if (filterTab.value === 'recent') list = [...list].sort((a,b) => String(b.updateTime||'').localeCompare(a.updateTime||''))
  return list
})

const sortedResult = computed(() => {
  let data = resultFilter.value ? resultData.value.filter(row =>
    Object.values(row).some(v => String(v).toLowerCase().includes(resultFilter.value.toLowerCase()))
  ) : resultData.value
  if (sortCol.value) {
    data = [...data].sort((a,b) => {
      const av = a[sortCol.value], bv = b[sortCol.value]
      return sortAsc.value ? String(av).localeCompare(String(bv)) : String(bv).localeCompare(String(av))
    })
  }
  return data.slice((page.value-1)*pageSize, page.value*pageSize)
})
const totalPages = computed(() => Math.ceil(resultData.value.length / pageSize))

const statusText = computed(() => {
  if (!currentStatement.value) return '未选择语句'
  return `当前: ${currentStatement.value.name || currentStatement.value.id}`
})

function selectStatement(s: Stmt) {
  currentStatement.value = s
  sql.value = s.sql ?? ''
}
function newStatement() {
  editing.value = false
  modalForm.value = { name: '', flag: '', sql: '', desc: '' }
  showModal.value = true
}
function editStatement(s: Stmt) {
  editing.value = true
  modalForm.value = { name: s.name||'', flag: s.flag||'', sql: s.sql??'', desc: s.desc||'' }
  showModal.value = true
}
const saveM = useMutation({
  mutationFn: async (data: any) => {
    if (editing.value && currentStatement.value?.id) return api.put(`/jaxrs/query/assemble/designer/update/${currentStatement.value!.id}`, data)
    return api.post('/jaxrs/query/assemble/designer/create', data)
  },
  onSuccess: () => { queryClient.invalidateQueries({ queryKey: ['stmt','list'] }); showModal.value = false }
})
function modalSave() {
  if (!modalForm.value.name.trim()) return
  const payload = { name: modalForm.value.name, flag: modalForm.value.flag, sql: modalForm.value.sql, description: modalForm.value.desc }
  saveM.mutate(payload)
}
const delM = useMutation({
  mutationFn: (id: string) => api.delete(`/jaxrs/query/assemble/designer/delete/${id}`),
  onSuccess: () => { queryClient.invalidateQueries({ queryKey: ['stmt','list'] }); if (currentStatement.value?.id) currentStatement.value = null }
})
function deleteStatement(s: Stmt) {
  if (!confirm(`删除语句「${s.name||s.id}」？`)) return
  delM.mutate(s.id)
}

async function executeSQL() {
  if (!sql.value.trim()) return
  loadingResult.value = true; hasResults.value = true
  try {
    const r: any = await api.post('/jaxrs/query/assemble/designer/execute', { sql: sql.value, id: currentStatement.value?.id })
    resultData.value = r?.data?.list ?? r?.data ?? []
    resultHeaders.value = resultData.value.length > 0 ? Object.keys(resultData.value[0]) : []
    page.value = 1
  } catch (e: any) {
    resultData.value = []
    resultHeaders.value = []
    alert('执行失败: ' + (e?.message ?? '未知错误'))
  } finally { loadingResult.value = false }
}

function formatSQL() {
  sql.value = sql.value.replace(/\s+/g, ' ').replace(/;/g, ';\n').trim()
}
function clearSQL() { sql.value = '' }
function sortResult(col: string) {
  if (sortCol.value === col) sortAsc.value = !sortAsc.value
  else { sortCol.value = col; sortAsc.value = true }
}
function exportCSV() {
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

function loadStatements() { queryClient.invalidateQueries({ queryKey: ['stmt','list'] }) }
function fmtTime(t?: string) { if (!t) return ''; try { return new Date(t).toLocaleString('zh-CN',{month:'2-digit',day:'2-digit',hour:'2-digit',minute:'2-digit'}) } catch { return String(t) } }
onMounted(loadStatements)
</script>

<style scoped>
.smd{display:flex;flex-direction:column;gap:0;height:100%}
.smd-header{display:flex;align-items:center;justify-content:space-between;padding:12px 20px;flex-shrink:0}
.smd-title h1{font-family:'Orbitron',sans-serif;font-size:18px;color:var(--color-primary);margin:0 0 2px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:11px;color:var(--text-muted);margin:0;font-family:'JetBrains Mono',monospace}
.smd-actions{display:flex;gap:8px}
.btn{padding:6px 14px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);cursor:pointer;font-size:13px}
.btn:hover{border-color:var(--color-primary);color:var(--color-primary)}
.btn-primary{background:var(--color-primary);color:#000;border-color:var(--color-primary);font-weight:600}
.btn-success{background:var(--color-success);color:#000;border-color:var(--color-success);font-weight:600}
.btn-success:disabled{opacity:0.4;cursor:not-allowed}
.smd-body{display:flex;flex:1;gap:0;min-height:0;overflow:hidden}
/* Sidebar */
.smd-sidebar{width:240px;flex-shrink:0;display:flex;flex-direction:column;border-right:1px solid var(--border-color)}
.sb-search{padding:8px}
.sb-input{width:100%;padding:6px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:12px;outline:none;box-sizing:border-box}
.sb-tabs{display:flex;gap:4px;padding:4px 8px;border-bottom:1px solid var(--border-color)}
.sb-tabs button{flex:1;padding:4px;font-size:11px;border-radius:var(--radius-sm);border:1px solid transparent;background:transparent;color:var(--text-muted);cursor:pointer}
.sb-tabs button.active{border-color:var(--color-primary);color:var(--color-primary);background:var(--color-primary-soft)}
.sb-list{flex:1;overflow-y:auto;padding:4px}
.sb-loading,.sb-empty{padding:16px;text-align:center;color:var(--text-muted);font-size:12px}
.sb-item{display:flex;align-items:center;gap:8px;padding:8px;border-radius:var(--radius-sm);cursor:pointer;margin-bottom:2px}
.sb-item:hover{background:var(--bg-hover)}
.sb-item.active{background:var(--color-primary-soft);border-left:3px solid var(--color-primary)}
.si-icon{font-size:18px;flex-shrink:0}
.si-info{flex:1;min-width:0}
.si-name{font-size:13px;color:var(--text-primary);overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.si-meta{font-size:10px;color:var(--text-muted);font-family:'JetBrains Mono',monospace;margin-top:2px}
.si-actions{display:flex;gap:2px;flex-shrink:0}
.si-btn{padding:2px 5px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:11px}
.si-btn:hover{border-color:var(--color-primary);color:var(--color-primary)}
.si-del:hover{border-color:var(--color-danger);color:var(--color-danger)}
/* Editor */
.smd-editor{flex:1;display:flex;flex-direction:column;min-width:0;padding:12px}
.editor-header{display:flex;gap:8px;margin-bottom:8px}
.stmt-name{flex:2;padding:7px 10px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:14px;outline:none;font-weight:600}
.stmt-category{flex:1;padding:7px 10px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:13px;outline:none}
.editor-toolbar{display:flex;align-items:center;gap:8px;margin-bottom:8px}
.tb-btn{padding:4px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-muted);cursor:pointer;font-size:12px}
.tb-btn:hover{border-color:var(--color-primary);color:var(--color-primary)}
.tb-info{font-size:11px;color:var(--text-muted);margin-left:auto}
.sql-editor{flex:1;min-height:200px;padding:12px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-terminal);color:#7fdbca;font-family:'Fira Code','JetBrains Mono',monospace;font-size:13px;outline:none;resize:none;line-height:1.6;tab-size:2}
.sql-editor:focus{border-color:var(--color-primary)}
.editor-status{padding:6px 0;font-size:12px;color:var(--text-muted);border-top:1px solid var(--border-color);margin-top:8px}
/* Results */
.smd-results{width:400px;flex-shrink:0;display:flex;flex-direction:column;border-left:1px solid var(--border-color);overflow:hidden}
.results-header{display:flex;align-items:center;gap:8px;padding:10px 12px;border-bottom:1px solid var(--border-color);font-size:13px;font-weight:600;color:var(--color-primary)}
.results-count{font-size:11px;color:var(--text-muted);margin-left:auto}
.btn-sm{padding:3px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:11px}
.results-toolbar{padding:6px 12px;border-bottom:1px solid var(--border-color)}
.result-filter{width:100%;padding:5px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:12px;outline:none;box-sizing:border-box}
.results-grid{flex:1;overflow:auto}
.results-loading,.results-empty{padding:24px;text-align:center;color:var(--text-muted);font-size:13px}
.res-table{width:100%;border-collapse:collapse;font-size:12px}
.res-table th{padding:6px 10px;text-align:left;border-bottom:1px solid var(--border-color);color:var(--text-muted);font-weight:600;font-size:11px;text-transform:uppercase;position:sticky;top:0;background:var(--bg-surface);cursor:pointer;white-space:nowrap}
.res-table th:hover{color:var(--color-primary)}
.res-table td{padding:5px 10px;border-bottom:1px solid var(--border-subtle);color:var(--text-primary);max-width:200px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.res-table tr:hover td{background:var(--bg-hover)}
.mono{font-family:'JetBrains Mono',monospace;font-size:11px}
.results-pager{display:flex;align-items:center;gap:12px;padding:8px 12px;border-top:1px solid var(--border-color)}
.pg-btn{padding:4px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);cursor:pointer;font-size:14px}
.pg-btn:disabled{opacity:0.3;cursor:not-allowed}
.pg-info{font-size:12px;color:var(--text-muted)}
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
