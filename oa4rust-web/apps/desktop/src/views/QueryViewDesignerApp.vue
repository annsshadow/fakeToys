<template>
  <div class="designer-shell">
    <header class="designer-header glass-card"><div><h1>查询视图设计器</h1><p>可视化过滤、排序、分页与真实 simulate/bundle</p></div><div class="actions"><button class="btn" @click="newView">新建视图</button><button class="btn primary" :disabled="!canSave||saving" @click="saveView">{{saving?'保存中…':'保存'}}</button><button class="btn" :disabled="!activeId" @click="simulate">模拟</button><button class="btn" :disabled="!activeId" @click="bundle">Bundle</button></div></header>
    <div class="query-picker glass-card"><label>Query Flag<input v-model="queryFlag" placeholder="输入后加载视图" @change="loadViews" /></label><button class="btn" :disabled="!queryFlag.trim()" @click="loadViews">加载</button><span>后端只提供按 Query Flag 列表，未提供全量视图列表。</span></div>
    <main class="main-grid"><aside class="list glass-card"><h2>视图</h2><button v-for="item in views" :key="item.id" class="view-item" :class="{active:item.id===activeId}" @click="openView(item.id)"><strong>{{item.name}}</strong><span>{{item.viewFlag}}</span></button><p v-if="!views.length" class="muted">该 Query 下暂无视图</p></aside>
      <section class="editor glass-card"><div class="form-row"><label>名称<input v-model="form.name" /></label><label>视图 Flag<input :value="form.viewFlag||'保存后生成'" disabled /></label></div><label class="sql-label">只读 SQL<textarea v-model="form.definition.sql" rows="5" placeholder="SELECT ..." /></label>
        <div class="section-head"><h2>过滤条件</h2><button class="btn" @click="addFilter">添加过滤</button></div><div v-for="(filter,index) in form.definition.filters" :key="index" class="rule-row"><input v-model="filter.field" placeholder="字段" /><select v-model="filter.operator"><option v-for="operator in operators" :key="operator" :value="operator">{{operator}}</option></select><input v-model="filter.value" placeholder="值" /><button class="remove" @click="form.definition.filters.splice(index,1)">删除</button></div>
        <div class="section-head"><h2>排序</h2><button class="btn" @click="addSort">添加排序</button></div><div v-for="(sort,index) in form.definition.sorts" :key="index" class="rule-row sort-row"><input v-model="sort.field" placeholder="字段" /><select v-model="sort.direction"><option value="asc">升序</option><option value="desc">降序</option></select><button class="remove" @click="form.definition.sorts.splice(index,1)">删除</button></div>
        <div class="paging"><label>页码<input v-model.number="form.definition.paging.page" type="number" min="1" /></label><label>每页<input v-model.number="form.definition.paging.size" type="number" min="1" max="500" /></label></div>
        <section class="lookup"><h2>Lookup</h2><p>当前后端 query designer/surface 无 lookup 路由或存储契约，因此不展示伪造配置；可在 SQL 与过滤条件中表达后端实际支持的查询。</p></section>
        <section v-if="result" class="result"><h2>{{resultTitle}}</h2><pre>{{JSON.stringify(result,null,2)}}</pre></section>
      </section></main>
  </div>
</template>
<script setup lang="ts">
import { api } from '@oa4rust/sdk'
import { computed, ref } from 'vue'
import {
  designerPaths,
  extractList,
  parseViewDefinition,
  type QueryViewDefinition,
  type QueryViewFilter,
  serializeViewDefinition,
  viewRuntimePayload,
} from '../contracts/designer'
import { toast } from '../utils/toast'

interface ViewSummary {
  id: string
  name: string
  viewFlag: string
  queryFlag?: string
}
const operators: QueryViewFilter['operator'][] = ['eq', 'ne', 'contains', 'gt', 'gte', 'lt', 'lte']
const queryFlag = ref('')
const views = ref<ViewSummary[]>([])
const activeId = ref('')
const saving = ref(false)
const result = ref<unknown>(null)
const resultTitle = ref('')
const emptyDefinition = (): QueryViewDefinition => ({
  version: 1,
  sql: '',
  filters: [],
  sorts: [],
  paging: { page: 1, size: 20 },
})
const form = ref<{ name: string; viewFlag: string; definition: QueryViewDefinition }>({
  name: '',
  viewFlag: '',
  definition: emptyDefinition(),
})
const canSave = computed(() => form.value.name.trim() && queryFlag.value.trim() && form.value.definition.sql.trim())
function newView() {
  activeId.value = ''
  form.value = { name: '', viewFlag: '', definition: emptyDefinition() }
  result.value = null
}
function addFilter() {
  form.value.definition.filters.push({ field: '', operator: 'eq', value: '' })
}
function addSort() {
  form.value.definition.sorts.push({ field: '', direction: 'asc' })
}
async function loadViews() {
  if (!queryFlag.value.trim()) return
  try {
    const response = await api.get<unknown>(designerPaths.viewList(queryFlag.value.trim()))
    views.value = extractList<ViewSummary>(response.data)
  } catch (error: any) {
    toast.error(`加载视图失败: ${error?.message ?? '未知错误'}`)
  }
}
async function openView(id: string) {
  try {
    const response = await api.get<any>(designerPaths.viewGet(id))
    activeId.value = id
    queryFlag.value = response.data?.queryFlag ?? queryFlag.value
    form.value = {
      name: response.data?.name ?? '',
      viewFlag: response.data?.viewFlag ?? '',
      definition: parseViewDefinition(response.data?.content),
    }
    result.value = null
  } catch (error: any) {
    toast.error(`加载视图失败: ${error?.message ?? '未知错误'}`)
  }
}
async function saveView() {
  if (!canSave.value) return
  saving.value = true
  const payload = {
    name: form.value.name.trim(),
    queryFlag: queryFlag.value.trim(),
    data: serializeViewDefinition(form.value.definition),
  }
  try {
    const response = activeId.value
      ? await api.put<any>(designerPaths.viewSave(activeId.value), payload)
      : await api.post<any>(designerPaths.viewCreate, payload)
    activeId.value = response.data?.id ?? activeId.value
    form.value.viewFlag = response.data?.viewFlag ?? form.value.viewFlag
    toast.success('视图已保存')
    await loadViews()
  } catch (error: any) {
    toast.error(`保存失败: ${error?.message ?? '未知错误'}`)
  } finally {
    saving.value = false
  }
}
async function simulate() {
  if (!activeId.value) return
  try {
    await saveView()
    const response = await api.put<unknown>(
      designerPaths.viewSimulate(activeId.value),
      viewRuntimePayload(form.value.definition),
    )
    result.value = response.data
    resultTitle.value = '模拟结果'
  } catch (error: any) {
    toast.error(`模拟失败: ${error?.message ?? '未知错误'}`)
  }
}
async function bundle() {
  if (!activeId.value) return
  try {
    const response = await api.put<unknown>(
      designerPaths.viewBundle(activeId.value),
      viewRuntimePayload(form.value.definition),
    )
    result.value = response.data
    resultTitle.value = 'Bundle 结果'
  } catch (error: any) {
    toast.error(`Bundle 失败: ${error?.message ?? '未知错误'}`)
  }
}
</script>
<style scoped>
.designer-shell{display:flex;flex-direction:column;gap:14px;height:100%}.designer-header,.query-picker{display:flex;align-items:center;justify-content:space-between;padding:15px 20px}.designer-header h1,.list h2,.editor h2{margin:0;color:var(--color-primary);font-size:17px}.designer-header p{margin:4px 0 0;color:var(--text-muted);font-size:12px}.actions{display:flex;gap:8px}.query-picker{justify-content:flex-start;gap:10px}.query-picker label{display:flex;align-items:center;gap:8px}.query-picker span,.muted,.lookup p{color:var(--text-muted);font-size:11px}.main-grid{display:grid;grid-template-columns:230px 1fr;gap:14px;min-height:0;flex:1}.list,.editor{padding:14px;overflow:auto}.view-item{display:flex;flex-direction:column;gap:4px;width:100%;padding:10px;margin-top:8px;border:1px solid var(--border-color);border-radius:8px;background:var(--bg-elevated);color:var(--text-primary);text-align:left;cursor:pointer}.view-item.active{border-color:var(--color-primary)}.view-item span{font:10px monospace;color:var(--text-muted)}.form-row,.paging{display:grid;grid-template-columns:1fr 1fr;gap:12px}.editor label{font-size:12px;color:var(--text-muted)}input,select,textarea{box-sizing:border-box;width:100%;padding:8px;border:1px solid var(--border-color);border-radius:6px;background:var(--bg-elevated);color:var(--text-primary)}.sql-label{display:block;margin-top:14px}.section-head{display:flex;align-items:center;justify-content:space-between;margin-top:18px}.rule-row{display:grid;grid-template-columns:1fr 130px 1fr auto;gap:8px;margin-top:8px}.sort-row{grid-template-columns:1fr 130px auto}.paging{width:330px;margin-top:18px}.lookup,.result{margin-top:18px;padding-top:12px;border-top:1px solid var(--border-color)}.result pre{max-height:280px;overflow:auto;padding:12px;border-radius:8px;background:#0005}.btn{padding:8px 13px;border:1px solid var(--border-color);border-radius:7px;background:var(--bg-elevated);color:var(--text-primary);cursor:pointer}.btn.primary{background:var(--color-primary);color:#fff}.btn:disabled{opacity:.45}.remove{border:0;background:transparent;color:var(--color-danger);cursor:pointer}@media(max-width:850px){.main-grid{grid-template-columns:1fr}.rule-row{grid-template-columns:1fr}}
</style>
