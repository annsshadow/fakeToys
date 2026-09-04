<template>
  <div class="query-designer">
    <div class="view-header glass-card">
      <h1>查询设计器</h1>
      <p class="subtitle">/jaxrs/query/assemble/designer/* — 可视化查询构建</p>
      <button class="btn-create" @click="openCreate">+ 新建查询</button>
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
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
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
  if (!mform.value.name.trim()) { alert('请输入查询名称'); return }
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
  } catch (e: any) { alert('保存失败: ' + (e?.message ?? '')) }
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
  } catch (e: any) { alert('执行失败: ' + (e?.message ?? '')) } finally { rLoading.value = false }
}

async function deleteQuery(q: QueryDef) {
  if (!confirm(`删除查询「${q.name || q.id}」？`)) return
  try {
    await api.delete(`/jaxrs/query/assemble/designer/delete/${q.id}`)
    if (selected.value?.id === q.id) selected.value = null
    queries.value = queries.value.filter(x => x.id !== q.id)
  } catch (e: any) { alert('删除失败: ' + (e?.message ?? '')) }
}

loadQueries()
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
</style>
