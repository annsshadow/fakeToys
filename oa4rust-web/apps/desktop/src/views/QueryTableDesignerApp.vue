<template>
  <div class="designer-shell">
    <header class="designer-header glass-card">
      <div><h1>数据表设计器</h1><p>typed columns → 后端生成并执行受限 DDL</p></div>
      <div class="actions"><button class="btn" @click="newTable">新建表</button><button class="btn primary" :disabled="!canSave || saving" @click="saveTable">{{ saving ? '保存中…' : '保存定义' }}</button><button class="btn success" :disabled="!activeFlag || executing" @click="executeTable">{{ executing ? '执行中…' : '执行建表/加列' }}</button></div>
    </header>
    <main class="main-grid">
      <aside class="list glass-card">
        <div class="list-head"><h2>数据表</h2><button class="link" @click="loadTables">刷新</button></div>
        <button v-for="item in tables" :key="item.tableFlag" class="table-item" :class="{ active: item.tableFlag === activeFlag }" @click="openTable(item)">
          <strong>{{ item.name }}</strong><span>{{ item.tableFlag }}</span><small>{{ item.status || 'draft' }}</small>
        </button>
        <p v-if="loading" class="muted">加载中…</p><p v-else-if="!tables.length" class="muted">暂无数据表</p>
      </aside>
      <section class="editor glass-card">
        <div class="form-row"><label>名称<input v-model="form.name" /></label><label>所属 Query Flag<input v-model="form.queryFlag" :disabled="!!activeFlag" /></label><label>Table Flag<input :value="activeFlag || '保存后由后端生成'" disabled /></label></div>
        <div class="column-head"><h2>列定义</h2><button class="btn" @click="addColumn">添加列</button></div>
        <table class="columns"><thead><tr><th>列名</th><th>类型</th><th>可空</th><th></th></tr></thead><tbody>
          <tr v-for="(column,index) in form.columns" :key="index"><td><input v-model="column.name" placeholder="lower_snake_case" /></td><td><select v-model="column.type"><option v-for="type in columnTypes" :key="type" :value="type">{{ type }}</option></select></td><td><input v-model="column.nullable" type="checkbox" /></td><td><button class="remove" @click="removeColumn(index)">删除</button></td></tr>
        </tbody></table>
        <p class="notice">已有物理表仅允许追加列；删除列、改类型或可空性会由后端拒绝，避免破坏数据。</p>
        <section v-if="executeResult" class="result"><h2>执行结果</h2><pre>{{ JSON.stringify(executeResult, null, 2) }}</pre></section>
      </section>
    </main>
  </div>
</template>
<script setup lang="ts">
import { api } from '@oa4rust/sdk'
import { computed, ref } from 'vue'
import { designerPaths, extractList, type TableColumnDefinition, tablePayload } from '../contracts/designer'
import { toast } from '../utils/toast'

interface TableSummary {
  id: string
  name: string
  tableFlag: string
  queryFlag?: string
  columns?: TableColumnDefinition[]
  status?: string
}
const columnTypes: TableColumnDefinition['type'][] = [
  'text',
  'integer',
  'decimal',
  'boolean',
  'date',
  'datetime',
  'json',
]
const tables = ref<TableSummary[]>([])
const activeFlag = ref('')
const loading = ref(false)
const saving = ref(false)
const executing = ref(false)
const executeResult = ref<unknown>(null)
const form = ref<{ name: string; queryFlag: string; columns: TableColumnDefinition[] }>({
  name: '',
  queryFlag: '',
  columns: [],
})
const canSave = computed(
  () =>
    form.value.name.trim() &&
    form.value.columns.length &&
    form.value.columns.every((column) => /^[a-z_][a-z0-9_]{0,62}$/.test(column.name) && column.name !== 'id'),
)
function addColumn() {
  form.value.columns.push({ name: '', type: 'text', nullable: true })
}
function removeColumn(index: number) {
  form.value.columns.splice(index, 1)
}
function newTable() {
  activeFlag.value = ''
  form.value = { name: '', queryFlag: '', columns: [{ name: 'title', type: 'text', nullable: false }] }
  executeResult.value = null
}
function openTable(item: TableSummary) {
  activeFlag.value = item.tableFlag
  form.value = {
    name: item.name,
    queryFlag: item.queryFlag ?? '',
    columns: (item.columns ?? []).map((column) => ({ ...column })),
  }
  executeResult.value = null
}
async function loadTables() {
  loading.value = true
  try {
    const response = await api.get<unknown>(designerPaths.tableList)
    tables.value = extractList<TableSummary>(response.data)
  } catch (error: any) {
    toast.error(`加载数据表失败: ${error?.message ?? '未知错误'}`)
  } finally {
    loading.value = false
  }
}
async function saveTable() {
  if (!canSave.value) return
  saving.value = true
  const payload = tablePayload(form.value.name, form.value.queryFlag, form.value.columns)
  try {
    const response = activeFlag.value
      ? await api.put<any>(designerPaths.tableSave(activeFlag.value), payload)
      : await api.post<any>(designerPaths.tableCreate, payload)
    activeFlag.value = response.data?.tableFlag ?? activeFlag.value
    toast.success('表定义已保存为草稿')
    await loadTables()
  } catch (error: any) {
    toast.error(`保存失败: ${error?.message ?? '未知错误'}`)
  } finally {
    saving.value = false
  }
}
async function executeTable() {
  if (!activeFlag.value) return
  executing.value = true
  try {
    const response = await api.post<unknown>(designerPaths.tableExecute(activeFlag.value), {})
    executeResult.value = response.data
    toast.success('受限 DDL 已执行')
    await loadTables()
  } catch (error: any) {
    toast.error(`执行失败: ${error?.message ?? '未知错误'}`)
  } finally {
    executing.value = false
  }
}
loadTables()
</script>
<style scoped>
.designer-shell{display:flex;flex-direction:column;gap:14px;height:100%}.designer-header{display:flex;align-items:center;justify-content:space-between;padding:16px 22px}.designer-header h1,.list h2,.editor h2{margin:0;color:var(--color-primary);font-size:17px}.designer-header p{margin:4px 0 0;color:var(--text-muted);font-size:12px}.actions{display:flex;gap:8px}.main-grid{display:grid;grid-template-columns:240px 1fr;gap:14px;min-height:0;flex:1}.list,.editor{padding:14px;overflow:auto}.list-head,.column-head{display:flex;align-items:center;justify-content:space-between}.link,.remove{border:0;background:transparent;color:var(--color-primary);cursor:pointer}.table-item{display:grid;grid-template-columns:1fr auto;gap:4px;width:100%;padding:10px;margin-top:8px;text-align:left;border:1px solid var(--border-color);border-radius:8px;background:var(--bg-elevated);color:var(--text-primary);cursor:pointer}.table-item.active{border-color:var(--color-primary)}.table-item span{grid-column:1/-1;font:10px monospace;color:var(--text-muted)}.table-item small{grid-row:1;grid-column:2;color:var(--color-secondary)}.form-row{display:grid;grid-template-columns:1fr 1fr 1fr;gap:12px}.form-row label{font-size:12px;color:var(--text-muted)}input,select{box-sizing:border-box;width:100%;padding:8px;border:1px solid var(--border-color);border-radius:6px;background:var(--bg-elevated);color:var(--text-primary)}.column-head{margin-top:24px}.columns{width:100%;margin-top:10px;border-collapse:collapse}.columns th,.columns td{padding:8px;border-bottom:1px solid var(--border-color);text-align:left}.columns th{color:var(--text-muted);font-size:11px}.columns input[type=checkbox]{width:auto}.notice,.muted{color:var(--text-muted);font-size:12px}.result{margin-top:20px}.result pre{max-height:240px;overflow:auto;padding:12px;border-radius:8px;background:#0005;color:var(--text-primary)}.btn{padding:8px 13px;border:1px solid var(--border-color);border-radius:7px;background:var(--bg-elevated);color:var(--text-primary);cursor:pointer}.btn.primary{background:var(--color-primary);color:#fff}.btn.success{border-color:var(--color-success);color:var(--color-success)}.btn:disabled{opacity:.45}@media(max-width:850px){.main-grid{grid-template-columns:1fr}.form-row{grid-template-columns:1fr}}
</style>
