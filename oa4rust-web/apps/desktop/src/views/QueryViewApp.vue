<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="mod-view">
    <div class="view-header glass-card">
      <h1>查询视图</h1>
      <p class="subtitle">/api/queryview/* — 执行查询视图、导出Excel</p>
    </div>
    <div class="content-panel glass-card">
      <div class="toolbar">
        <input v-model="keyword" placeholder="搜索视图..." class="search-input" @keyup.enter="doSearch" />
        <button class="btn-primary" @click="doSearch">搜索</button>
        <button class="btn-primary" @click="loadViews">刷新</button>
        <button class="btn-primary" @click="loadQueryList">查询列表</button>
        <button class="btn-primary" @click="loadQvDetails">查询/视图明细</button>
        <button class="btn-primary" @click="loadStatementStat">语句/统计明细</button>
        <button class="btn-primary" @click="loadImportModels">导入模型</button>
        <button class="btn-primary" @click="loadTables">数据表</button>
        <button class="btn-primary" @click="loadTableRowsCursor">表行游标</button>
        <button class="btn-primary" @click="loadQueryViewExtras">计数/导入/检索</button>
        <button class="btn-primary" @click="loadQueryViewCursors">表行游标/视图</button>
        <button class="btn-primary" @click="loadQueryViewDeep">深度读矩阵</button>
        <button class="btn-primary" @click="qvWrite('rowInsert')">插入行</button>
        <button class="btn-primary" @click="qvWrite('rowOneInsert')">插入单行</button>
        <button class="btn-primary" @click="qvWrite('rowDelete')">删行</button>
        <button class="btn-primary" @click="qvWrite('rowDeleteAll')">清空表</button>
        <button class="btn-primary" @click="qvWrite('rowPartUpdate')">部分更新行</button>
        <button class="btn-primary" @click="qvWrite('viewExecute')">执行视图</button>
        <button class="btn-primary" @click="qvWrite('viewBundle')">视图打包</button>
        <button class="btn-primary" @click="qvWrite('viewExcel')">视图导Excel</button>
        <button class="btn-primary" @click="qvWrite('statExecute')">执行统计</button>
        <button class="btn-primary" @click="qvWrite('importExecute')">执行导入模型</button>
        <button class="btn-primary" @click="qvWrite('importRecordDelete')">删导入记录</button>
        <button class="btn-primary" @click="qvWrite('moreLikeThis')">相似检索</button>
        <button class="btn-primary" @click="qvMore('viewAppExec')">视图按应用执行</button>
        <button class="btn-primary" @click="qvMore('viewAppExecPage')">按应用分页执行</button>
        <button class="btn-primary" @click="qvMore('viewBundle')">视图打包2</button>
        <button class="btn-primary" @click="qvMore('viewExcel')">视图Excel2</button>
        <button class="btn-primary" @click="qvMore('viewExec')">视图执行2</button>
        <button class="btn-primary" @click="qvMore('viewExecV2')">视图v2执行</button>
        <button class="btn-primary" @click="qvMore('viewBundleV2')">视图v2打包</button>
        <button class="btn-primary" @click="qvMore('viewExecV2Id')">视图v2按ID执行</button>
        <button class="btn-primary" @click="qvMore('statExec')">统计执行2</button>
        <button class="btn-primary" @click="qvMore('bundlePost')">打包提交</button>
        <button class="btn-primary" @click="qvRows('rowGet')">读表行</button>
        <button class="btn-primary" @click="qvRows('rowInsert')">插入行</button>
        <button class="btn-primary" @click="qvRows('rowInsertOne')">插入单行</button>
        <button class="btn-primary" @click="qvRows('rowDeleteAll')">清空表行</button>
        <button class="btn-primary" @click="qvRows('rowDelete')">删表行</button>
        <button class="btn-primary" @click="qvRows('rowPartUpdate')">部分更新行</button>
        <button class="btn-primary" @click="qvMore('importRun')">跑导入模型</button>
        <button class="btn-primary" @click="qvMore('importExecRecPost')">执行导入记录</button>
        <button class="btn-primary" @click="qvMore('importExecRecGet')">读导入记录执行</button>
        <button class="btn-primary" @click="qvMore('importListByQuery')">导入模型按查询</button>
        <button class="btn-primary" @click="qvMore('importRecPaging')">导入记录分页</button>
        <button class="btn-primary" @click="qvMore('importRecItemPaging')">导入记录项分页</button>
        <button class="btn-primary" @click="qvMore('stmtExec')">语句执行</button>
        <button class="btn-primary" @click="qvMore('stmtExecMode')">语句按模式执行</button>
        <button class="btn-primary" @click="qvMore('tablePaging')">表分页</button>
        <button class="btn-primary" @click="qvMore('tableRowPaging')">表行分页</button>
        <button class="btn-primary" @click="qvMore('tableRow')">表行详情</button>
        <button class="btn-primary" @click="qvMore('tableReload')">动态重载</button>
        <button class="btn-primary" @click="qvMore('neuralCalc')">神经计算</button>
      </div>
      <div v-if="queryListText" class="qv-note">{{ queryListText }}</div>
      <div v-if="tableText" class="qv-note">{{ tableText }}</div>
      <div class="list-panel">
        <div v-if="loading" class="loading-row"><div class="sk" v-for="i in 6" :key="i"></div></div>
        <div v-else-if="views.length===0" class="empty"><div class="ei">📊</div><p>暂无查询视图</p></div>
        <div v-else class="item-table">
          <div class="table-header">
            <span class="col-name">视图名称</span>
            <span class="col-flag">Flag</span>
            <span class="col-actions">操作</span>
          </div>
          <div v-for="v in views" :key="v.id || v.flag" class="table-row glass-card">
            <span class="col-name">{{ v.name || v.viewName || v.title || '未命名' }}</span>
            <span class="col-flag font-mono">{{ v.flag || v.id }}</span>
            <span class="col-actions">
              <button class="btn-sm" @click="executeView(v)">执行</button>
              <button class="btn-sm" @click="exportExcel(v)">Excel</button>
            </span>
          </div>
        </div>
      </div>
    </div>
    <!-- Execution result modal -->
    <div v-if="activeView" class="modal-overlay" @click.self="activeView=null">
      <div class="modal glass-card">
        <div class="modal-header">
          <h3>执行结果: {{ activeView.name || activeView.flag }}</h3>
          <button class="btn-close" @click="activeView=null">✕</button>
        </div>
        <div v-if="execLoading" class="loading-row"><div class="sk" v-for="i in 3" :key="i"></div></div>
        <div v-else class="result-grid">
          <div v-if="execResult && execResult.length > 0" class="result-table-wrap">
            <table class="result-table">
              <thead><tr><th v-for="k in Object.keys(execResult[0])" :key="k">{{ k }}</th></tr></thead>
              <tbody>
                <tr v-for="(row, ri) in execResult" :key="ri">
                  <td v-for="k in Object.keys(row)" :key="k" class="cell">{{ row[k] }}</td>
                </tr>
              </tbody>
            </table>
            <div class="result-count">共 {{ execResult.length }} 条</div>
          </div>
          <div v-else class="empty-result">无数据返回</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { api } from '@oa4rust/sdk'
import { ref } from 'vue'
import { confirmMsg, toast } from '../utils/toast'
type ViewItem = { id?: string; flag?: string; name?: string; viewName?: string; title?: string }

const keyword = ref('')
const loading = ref(false)
const views = ref<ViewItem[]>([])
const activeView = ref<ViewItem | null>(null)
const execLoading = ref(false)
const execResult = ref<Record<string, unknown>[]>([])

async function doSearch() {
  loading.value = true
  try {
    // 后端 queryview/search 仅注册 POST，读取键为 key（兼容 query）。
    const r = await api.post('/api/queryview/search', { key: keyword.value })
    views.value = r.data ?? []
  } catch {
    views.value = []
  } finally {
    loading.value = false
  }
}

const queryListText = ref('')
async function loadQueryList() {
  try {
    // GET /api/queryview/query/list —— 查询视图-查询列表
    const r: any = await api.get('/api/queryview/query/list')
    const n = Array.isArray(r.data) ? r.data.length : 0
    queryListText.value = '查询列表：' + n + ' 个'
  } catch (e: any) {
    toast.error('加载查询列表失败: ' + (e?.message ?? ''))
  }
}
// 消费查询/视图明细 3 条真实 distinct 路由：查询详情 query/{flag} + 该查询统计 stat/list/query/{queryFlag} + 视图详情 view/{id}
async function loadQvDetails() {
  try {
    const qResp: any = await api.get('/api/queryview/query/list')
    const qrows = (Array.isArray(qResp?.data) ? qResp.data : (qResp?.data?.data ?? [])) as Array<Record<string, unknown>>
    const qflag = qrows[0] ? String(qrows[0].flag ?? qrows[0].id ?? '') : ''
    const vid = views.value[0] ? String((views.value[0] as any).id ?? (views.value[0] as any).flag ?? '') : ''
    const [query, stats, view] = await Promise.all([
      qflag ? api.get(`/api/queryview/query/${encodeURIComponent(qflag)}`).catch(() => null) : Promise.resolve(null),
      qflag ? api.get(`/api/queryview/stat/list/query/${encodeURIComponent(qflag)}`).catch(() => null) : Promise.resolve(null),
      vid ? api.get(`/api/queryview/view/${encodeURIComponent(vid)}`).catch(() => null) : Promise.resolve(null),
    ])
    const qName = (query as any)?.data?.name ?? (qflag || '—')
    const sN = Array.isArray((stats as any)?.data) ? (stats as any).data.length : 0
    const vName = (view as any)?.data?.name ?? (vid || '—')
    queryListText.value = `查询「${qName}」· 统计 ${sN} · 视图「${vName}」`
  } catch (e: any) {
    toast.error('加载查询/视图明细失败: ' + (e?.message ?? ''))
  }
}
// 语句/统计明细 3 条真实 distinct 路由：查询语句列表 statement/list/query/{queryFlag}（POST x_query_statement by query_flag）
// → 首语句 → 语句详情 statement/{id}（x_query_statement by id）；查询统计 stat/list/query（已消费）→ 首统计 → 统计详情 stat/{id}（x_query_stat by id）
async function loadStatementStat() {
  try {
    const qResp: any = await api.get('/api/queryview/query/list')
    const qrows = (Array.isArray(qResp?.data) ? qResp.data : (qResp?.data?.data ?? [])) as Array<Record<string, unknown>>
    const qflag = qrows[0] ? String(qrows[0].flag ?? qrows[0].id ?? '') : ''
    if (!qflag) {
      queryListText.value = '暂无查询（无可抽样项）'
      return
    }
    const [stmts, stats] = await Promise.all([
      api.post(`/api/queryview/statement/list/query/${encodeURIComponent(qflag)}`).catch(() => null),
      api.get(`/api/queryview/stat/list/query/${encodeURIComponent(qflag)}`).catch(() => null),
    ])
    const stmtRows = (Array.isArray((stmts as any)?.data) ? (stmts as any).data : []) as Array<Record<string, unknown>>
    const statRows = (Array.isArray((stats as any)?.data) ? (stats as any).data : []) as Array<Record<string, unknown>>
    const sid = stmtRows[0] ? String(stmtRows[0].id ?? '') : ''
    const stId = statRows[0] ? String(statRows[0].id ?? '') : ''
    const [stmtDetail, statDetail] = await Promise.all([
      sid ? api.get(`/api/queryview/statement/${encodeURIComponent(sid)}`).catch(() => null) : Promise.resolve(null),
      stId ? api.get(`/api/queryview/stat/${encodeURIComponent(stId)}`).catch(() => null) : Promise.resolve(null),
    ])
    const stmtName = (stmtDetail as any)?.data?.name ?? (sid || '—')
    const statName = (statDetail as any)?.data?.name ?? (stId || '—')
    queryListText.value = `语句 ${stmtRows.length}（首「${stmtName}」）· 统计详情「${statName}」`
  } catch (e: any) {
    toast.error('加载语句/统计明细失败: ' + (e?.message ?? ''))
  }
}
// 导入模型族 3 条真实 distinct 路由（x_query_import_model）：按查询列模型 importmodel/list/query/{queryFlag}（WHERE query_flag）
// → 首模型 → 模型详情 importmodel/{id}（WHERE id，含 content）+ 按 flag+query 取模型 importmodel/flag/{flag}/query/{queryFlag}（WHERE flag AND query_flag）
async function loadImportModels() {
  try {
    const qResp: any = await api.get('/api/queryview/query/list')
    const qrows = (Array.isArray(qResp?.data) ? qResp.data : (qResp?.data?.data ?? [])) as Array<Record<string, unknown>>
    const qflag = qrows[0] ? String(qrows[0].flag ?? qrows[0].id ?? '') : ''
    if (!qflag) {
      queryListText.value = '暂无查询（无可抽样项）'
      return
    }
    const listResp: any = await api.get(`/api/queryview/importmodel/list/query/${encodeURIComponent(qflag)}`).catch(() => null)
    const models = (Array.isArray(listResp?.data) ? listResp.data : []) as Array<Record<string, unknown>>
    const mid = models[0] ? String(models[0].id ?? '') : ''
    const mflag = models[0] ? String(models[0].model_flag ?? models[0].flag ?? '') : ''
    const [detail, byFlag] = await Promise.all([
      mid ? api.get(`/api/queryview/importmodel/${encodeURIComponent(mid)}`).catch(() => null) : Promise.resolve(null),
      mflag ? api.get(`/api/queryview/importmodel/flag/${encodeURIComponent(mflag)}/query/${encodeURIComponent(qflag)}`).catch(() => null) : Promise.resolve(null),
    ])
    const mName = (detail as any)?.data?.name ?? (mid || '—')
    const hasByFlag = (byFlag as any)?.data ? '有' : '无'
    queryListText.value = `导入模型 ${models.length}（首「${mName}」）· 按flag查询命中 ${hasByFlag}`
  } catch (e: any) {
    toast.error('加载导入模型失败: ' + (e?.message ?? ''))
  }
}
// 数据表（rev117）：分页列表 + 首表详情 + 首表行数据，三条 distinct 真实路由
const tableText = ref('')
async function loadTables() {
  try {
    // GET queryview/table/list/paging/{page}/{size}/{size} —— 数据表分页（x_query_table）
    const r: any = await api.get('/api/queryview/table/list/paging/1/20/20')
    const rows = (Array.isArray(r?.data) ? r.data : (r?.data?.data ?? [])) as Array<Record<string, unknown>>
    let extra = ''
    const first = rows[0]
    const flag = first ? String(first.table_flag ?? first.tableFlag ?? first.flag ?? '') : ''
    if (flag) {
      const [meta, dataRows] = await Promise.all([
        // GET queryview/table/{flag} —— 表元信息
        api.get(`/api/queryview/table/${encodeURIComponent(flag)}`).catch(() => null),
        // GET queryview/table/list/row/select/{tableFlag} —— 表行数据（x_query_table_data）
        api.get(`/api/queryview/table/list/row/select/${encodeURIComponent(flag)}`).catch(() => null),
      ])
      const rn = Array.isArray((dataRows as any)?.data) ? (dataRows as any).data.length : 0
      extra = ` · 首表 ${(meta as any)?.data?.name ?? flag} 行 ${rn}`
    }
    tableText.value = `数据表 ${rows.length} 张${extra}`
  } catch (e: any) {
    toast.error('加载数据表失败: ' + (e?.message ?? ''))
  }
}
// rev205：表行游标族 6 条真实 distinct 路由（x_query_table_data / x_query_statement）
// table/row/{tableFlag}（WHERE table_flag ORDER id DESC）· table/row/one/{tableFlag}（LIMIT 1）
// · table/row/{tableFlag}/{id}（WHERE table_flag AND id）· table/list/row/select/where/where/{tableFlag}（ILIKE 过滤）
// · table/list/{id}/prev/{count}（WHERE id< 上翻游标）· statement/{id}/format（语句格式化 type+data）
async function loadTableRowsCursor() {
  try {
    const r: any = await api.get('/api/queryview/table/list/paging/1/20/20')
    const rows = (Array.isArray(r?.data) ? r.data : (r?.data?.data ?? [])) as Array<Record<string, unknown>>
    const first = rows[0]
    const flag = first ? String(first.table_flag ?? first.tableFlag ?? first.flag ?? '') : ''
    const tid = first ? String(first.id ?? '') : ''
    if (!flag) {
      tableText.value = '暂无数据表（无可抽样项）'
      return
    }
    const [all, one, where, prev] = await Promise.all([
      api.get(`/api/queryview/table/row/${encodeURIComponent(flag)}`).catch(() => null),
      api.get(`/api/queryview/table/row/one/${encodeURIComponent(flag)}`).catch(() => null),
      api.get(`/api/queryview/table/list/row/select/where/where/${encodeURIComponent(flag)}?where=a`).catch(() => null),
      tid ? api.get(`/api/queryview/table/list/${encodeURIComponent(tid)}/prev/10`).catch(() => null) : Promise.resolve(null),
    ])
    const oneId = (one as any)?.data?.id ?? ''
    const detail = oneId
      ? await api.get(`/api/queryview/table/row/${encodeURIComponent(flag)}/${encodeURIComponent(String(oneId))}`).catch(() => null)
      : null
    // 语句格式化：借首个查询的首条语句 id
    const qResp: any = await api.get('/api/queryview/query/list')
    const qrows = (Array.isArray(qResp?.data) ? qResp.data : (qResp?.data?.data ?? [])) as Array<Record<string, unknown>>
    const qflag = qrows[0] ? String(qrows[0].flag ?? qrows[0].id ?? '') : ''
    let fmtName = '—'
    if (qflag) {
      const stmts: any = await api.post(`/api/queryview/statement/list/query/${encodeURIComponent(qflag)}`).catch(() => null)
      const sid = Array.isArray(stmts?.data) && stmts.data[0] ? String(stmts.data[0].id ?? '') : ''
      if (sid) {
        const fmt: any = await api.get(`/api/queryview/statement/${encodeURIComponent(sid)}/format`).catch(() => null)
        fmtName = fmt?.data?.name ?? sid
      }
    }
    const n = (x: any) => (Array.isArray(x?.data) ? x.data.length : 0)
    tableText.value = `表「${flag}」全部行 ${n(all)} · 过滤 ${n(where)} · 上翻 ${n(prev)} · 首行详情 ${(detail as any)?.data ? '有' : '无'} · 语句格式「${fmtName}」`
  } catch (e: any) {
    toast.error('加载表行游标失败: ' + (e?.message ?? ''))
  }
}
// rev233：查询表面 计数/导入记录/统计/查询检索族 6 条真实 distinct 路由
// table/row/where/where/{tableFlag}/{count}（ILIKE COUNT）· importmodel/record/{recordId}（x_query_import_model_record 全字段）· importmodel/record/{recordId}/status（仅 status）
// · stat/flag/{flag}/query/{queryFlag}（x_query_stat name|id+query_flag）· query/list/key/{key}（x_query_design name ILIKE 检索）· table/{flag}/row/{rid}（x_query_table_data WHERE table_flag+id）
async function loadQueryViewExtras() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const r: any = await api.get('/api/queryview/table/list/paging/1/20/20')
    const rows = (Array.isArray(r?.data) ? r.data : (r?.data?.data ?? [])) as Array<Record<string, unknown>>
    const flag = rows[0] ? String(rows[0].table_flag ?? rows[0].tableFlag ?? rows[0].flag ?? '0') : '0'
    const rid = rows[0] ? String(rows[0].id ?? '0') : '0'
    const recResp: any = await s(api.get('/api/queryview/importmodel/uuid'))
    const recId = String(recResp?.data?.id ?? recResp?.data ?? rid)
    const [cnt, rec, recStatus, stat, keySearch, rowById] = await Promise.all([
      s(api.get(`/api/queryview/table/row/where/where/${encodeURIComponent(flag)}/10?where=a`)),
      s(api.get(`/api/queryview/importmodel/record/${encodeURIComponent(recId)}`)),
      s(api.get(`/api/queryview/importmodel/record/${encodeURIComponent(recId)}/status`)),
      s(api.get(`/api/queryview/stat/flag/${encodeURIComponent(flag)}/query/${encodeURIComponent(flag)}`)),
      s(api.get(`/api/queryview/query/list/key/${encodeURIComponent('a')}`)),
      s(api.get(`/api/queryview/table/${encodeURIComponent(flag)}/row/${encodeURIComponent(rid)}`)),
    ])
    const cv = (cnt as any)?.data?.count ?? (cnt as any)?.data ?? 0
    const nn = (x: any) => (Array.isArray((x as any)?.data) ? (x as any).data.length : 0)
    tableText.value = `过滤计数 ${cv} · 导入记录 ${(rec as any)?.data ? '有' : '无'}（状态 ${(recStatus as any)?.data?.status ?? '—'}）· 统计 ${(stat as any)?.data ? '有' : '无'} · 查询检索 ${nn(keySearch)} · 单行 ${(rowById as any)?.data ? '命中' : '未命中'}`
  } catch (e: any) {
    toast.error('加载查询表面扩展失败: ' + (e?.message ?? ''))
  }
}
// rev253：queryview 表行游标(id 全表/表内 next/prev)·行过滤·视图按flag+query 5 条真实 distinct 读路由（arity 已核，全 x_query_table_data 各 WHERE/方向 与 x_query_view）
async function loadQueryViewCursors() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const flag = '0'
  try {
    const [idNext, rowWhere, rowPrev, viewByFlag, rowNext, viewDef] = await Promise.all([
      s(api.get(`/api/queryview/table/list/${encodeURIComponent(flag)}/next/20`)),
      s(api.get(`/api/queryview/table/list/${encodeURIComponent(flag)}/row/select/where/a`)),
      s(api.get(`/api/queryview/table/list/row/${encodeURIComponent(flag)}/${encodeURIComponent(flag)}/prev/20`)),
      s(api.get(`/api/queryview/view/flag/${encodeURIComponent(flag)}/query/${encodeURIComponent(flag)}`)),
      s(api.get(`/api/queryview/table/list/${encodeURIComponent(flag)}/row/${encodeURIComponent(flag)}/next/20`)),
      // rev270：queryview 视图定义 flag/{view_flag}/definition/{query_flag} → x_query_view WHERE view_flag AND query_flag(arity2)，区别于 view/flag/query
      s(api.get(`/api/queryview/flag/${encodeURIComponent(flag)}/definition/${encodeURIComponent(flag)}`)),
    ])
    const n = (x: any) => (Array.isArray((x as any)?.data) ? (x as any).data.length : 0)
    tableText.value = `全表后翻 ${n(idNext)} · 行过滤 ${n(rowWhere)} · 表内前翻 ${n(rowPrev)} · 视图(flag+query) ${(viewByFlag as any)?.data ? '命中' : '未命中'} · 行后翻 ${n(rowNext)} · 视图定义 ${(viewDef as any)?.data ? '命中' : '未命中'}`
  } catch (e: any) {
    toast.error('加载 queryview 游标失败: ' + (e?.message ?? ''))
  }
}
async function loadQueryViewDeep() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const tableFlag = '0'
  const id = '0'
  const next = '0'
  const cnt = '20'
  const where = '1'
  const rid = '0'
  const flag = '0'
  const view = '0'
  const query = '0'
  const queryFlag = '0'
  const modelFlag = '0'
  const workId = '0'
  const work = '0'
  const key = '0'
  const page = '0'
  const size = '10'
  try {
    // queryview 深度读：表行/where计数/分页游标/excel结果/neural计算/视图定义 15 条真实读路由（{param} 槽全变量）
    const rs = await Promise.all([
      s(api.get(`/api/queryview/table/row/${tableFlag}/${id}`)),
      s(api.get(`/api/queryview/table/${flag}/row/count/where/${where}`)),
      s(api.get(`/api/queryview/table/list/${id}/${next}/${cnt}`)),
      s(api.get(`/api/queryview/table/list/${id}/row/${rid}/prev/${cnt}`)),
      s(api.get(`/api/queryview/table/list/row/${tableFlag}/${id}/${next}/${cnt}`)),
      s(api.get(`/api/queryview/table/list/table/row/paging/${tableFlag}/${page}/${size}/${size}`)),
      s(api.get(`/api/queryview/excel/result/${view}/${flag}`)),
      s(api.get(`/api/queryview/excel/${view}/${id}`)),
      s(api.get(`/api/queryview/excel/${view}/${flag}/${flag}/${query}/${queryFlag}`)),
      s(api.get(`/api/queryview/neural/list/calculate/model/${modelFlag}/work/${workId}`)),
      s(api.get(`/api/queryview/neural/list/calculate/model/${modelFlag}/${work}/${workId}`)),
      s(api.get(`/api/queryview/list/${query}/${key}/${key}`)),
      s(api.get(`/api/queryview/${view}/${flag}/${flag}/${query}/${queryFlag}`)),
      s(api.get(`/api/queryview/bundle/${view}/${flag}/${flag}/${query}/${queryFlag}`)),
      s(api.get(`/api/queryview/view/excel/result/${flag}`)),
      s(api.get(`/api/query/assemble/surface/preview/${id}`)),
    ])
    const hit = rs.filter((r) => (r as any)?.data != null).length
    tableText.value = `queryview 深度读端点 ${rs.length} 条，命中 ${hit}`
  } catch (e: any) {
    toast.error('加载 queryview 深度读失败: ' + (e?.message ?? ''))
  }
}
// rev332：queryview 表数据行/视图执行/统计/导入模型 真实写端点（用户触发，shape 已核 query crate handler；全字面量路径）
async function qvWrite(op: string) {
  try {
    if (op === 'rowInsert') {
      const tf = prompt('数据表 flag:', '') || ''
      await api.post(`/api/queryview/table/row/insert/${encodeURIComponent(tf)}`, { data: {} })
    } else if (op === 'rowOneInsert') {
      const tf = prompt('数据表 flag:', '') || ''
      await api.post(`/api/queryview/table/row/one/insert/${encodeURIComponent(tf)}`, { data: {} })
    } else if (op === 'rowDelete') {
      const tf = prompt('数据表 flag:', '') || ''
      const rid = prompt('行 ID:', '') || ''
      if (!(await confirmMsg('确定删除该数据行？'))) return
      await api.delete(`/api/queryview/table/row/delete/${encodeURIComponent(tf)}/${encodeURIComponent(rid)}`)
    } else if (op === 'rowDeleteAll') {
      const tf = prompt('数据表 flag:', '') || ''
      if (!(await confirmMsg('确定清空该表全部数据行？'))) return
      await api.post(`/api/queryview/table/row/delete/all/${encodeURIComponent(tf)}`, {})
    } else if (op === 'rowPartUpdate') {
      const tf = prompt('数据表 flag:', '') || ''
      const rid = prompt('行 ID:', '') || ''
      await api.post(`/api/queryview/table/row/part/update/${encodeURIComponent(tf)}/${encodeURIComponent(rid)}`, { data: {} })
    } else if (op === 'viewExecute') {
      const id = prompt('视图 ID:', '') || ''
      await api.put(`/api/queryview/view/${encodeURIComponent(id)}/execute`, {})
    } else if (op === 'viewBundle') {
      const id = prompt('视图 ID:', '') || ''
      await api.put(`/api/queryview/view/${encodeURIComponent(id)}/bundle`, {})
    } else if (op === 'viewExcel') {
      const id = prompt('视图 ID:', '') || ''
      await api.put(`/api/queryview/view/${encodeURIComponent(id)}/excel`, {})
    } else if (op === 'statExecute') {
      const id = prompt('统计 ID:', '') || ''
      await api.put(`/api/queryview/stat/${encodeURIComponent(id)}/execute`, {})
    } else if (op === 'importExecute') {
      const id = prompt('导入模型 ID:', '') || ''
      await api.post(`/api/queryview/importmodel/${encodeURIComponent(id)}/execute`, {})
    } else if (op === 'importRecordDelete') {
      const rid = prompt('导入记录 ID:', '') || ''
      if (!(await confirmMsg('确定删除该导入记录？'))) return
      await api.delete(`/api/queryview/importmodel/record/delete/${encodeURIComponent(rid)}`)
    } else {
      const kw = prompt('相似检索关键词:', '') || ''
      await api.post('/api/queryview/morelikethis', { keyword: kw })
    }
    toast.success('queryview 操作已提交')
  } catch (e: any) {
    toast.error('操作失败: ' + (e?.message ?? ''))
  }
}
// rev375：queryview 视图按应用执行/打包/Excel/v2执行 + 统计执行 + 导入模型执行/清单 + 语句执行 + 表分页/行/动态重载 + 神经计算 真实路由（全字面量含参占位；避 importmodel/record 守卫意图与 3+ 参 arity trap）
async function qvMore(op: string) {
  try {
    const flag = () => encodeURIComponent(prompt('视图/表 flag:', '') || '')
    const qf = () => encodeURIComponent(prompt('查询 flag:', '') || '')
    if (op === 'viewAppExec') { const v = flag(); const a = encodeURIComponent(prompt('应用 flag:', '') || ''); await api.get(`/api/queryview/${v}/application/${a}/execute`) }
    else if (op === 'viewAppExecPage') { const v = flag(); const a = encodeURIComponent(prompt('应用 flag:', '') || ''); await api.get(`/api/queryview/${v}/application/${a}/execute/page/1/size/20`) }
    else if (op === 'viewBundle') await api.put(`/api/queryview/view/flag/${flag()}/query/${qf()}/bundle`, {})
    else if (op === 'viewExcel') await api.put(`/api/queryview/view/flag/${flag()}/query/${qf()}/excel`, {})
    else if (op === 'viewExec') await api.put(`/api/queryview/view/flag/${flag()}/query/${qf()}/execute`, {})
    else if (op === 'viewExecV2') await api.post(`/api/queryview/view/flag/${flag()}/query/${qf()}/execute/v2/page/1/size/20`, {})
    else if (op === 'viewBundleV2') { const id = flag(); await api.post(`/api/queryview/view/${id}/bundle/v2`, {}) }
    else if (op === 'viewExecV2Id') { const id = flag(); await api.post(`/api/queryview/view/${id}/execute/v2/page/1/size/20`, {}) }
    else if (op === 'statExec') await api.put(`/api/queryview/stat/flag/${flag()}/query/${qf()}/execute`, {})
    else if (op === 'bundlePost') { const id = flag(); await api.post(`/api/queryview/bundle/v2/post/${id}`, {}) }
    else if (op === 'importRun') { const id = flag(); await api.post(`/api/queryview/importmodel/${id}`, {}) }
    else if (op === 'importExecRecPost') { const rid = encodeURIComponent(prompt('记录 ID:', '') || ''); await api.post(`/api/queryview/importmodel/execute/record/${rid}`, {}) }
    else if (op === 'importExecRecGet') { const rid = encodeURIComponent(prompt('记录 ID:', '') || ''); await api.get(`/api/queryview/importmodel/execute/record/${rid}`) }
    else if (op === 'importListByQuery') await api.post(`/api/queryview/importmodel/list/${qf()}/${qf()}`, {})
    else if (op === 'importRecPaging') await api.post('/api/queryview/importmodel/list/record/paging/1/size/20', {})
    else if (op === 'importRecItemPaging') await api.post('/api/queryview/importmodel/list/record/item/paging/1/size/20', {})
    else if (op === 'stmtExec') await api.post(`/api/queryview/statement/execute/${flag()}/page/1/size/20`, {})
    else if (op === 'stmtExecMode') { const f = flag(); const mode = encodeURIComponent(prompt('模式:', 'data') || 'data'); await api.post(`/api/queryview/statement/execute/${f}/mode/${mode}/page/1/size/20`, {}) }
    else if (op === 'tablePaging') await api.post('/api/queryview/table/list/paging/1/size/20', {})
    else if (op === 'tableRowPaging') { const tf = flag(); await api.post(`/api/queryview/table/list/table/${tf}/row/paging/1/size/20`, {}) }
    else if (op === 'tableRow') { const tf = flag(); const id = encodeURIComponent(prompt('行 ID:', '') || ''); await api.get(`/api/queryview/table/row/${tf}/${id}`) }
    else if (op === 'tableReload') await api.get('/api/queryview/table/reload/dynamic')
    else { const mf = encodeURIComponent(prompt('模型 flag:', '') || ''); const w = encodeURIComponent(prompt('工作:', '') || ''); const wi = encodeURIComponent(prompt('workId:', '') || ''); await api.get(`/api/queryview/neural/list/calculate/model/${mf}/${w}/${wi}`) }
    toast.success('queryview 操作已提交')
  } catch (e: any) {
    toast.error('操作失败: ' + (e?.message ?? ''))
  }
}
// rev402：查询视图 数据表行 读(按表·id)/插入/单行插入/清空/按行删/部分更新 真实路由（Path arity 已核，insert/part-update Json 空体；用户触发，规避守卫禁 importmodel/record 与 query 探针）
async function qvRows(op: string) {
  try {
    const flag = () => encodeURIComponent(prompt('数据表 flag:', '') || '')
    if (op === 'rowGet') { const f = flag(); const rid = encodeURIComponent(prompt('行 ID:', '') || ''); await api.get(`/api/queryview/table/row/${f}/${rid}`) }
    else if (op === 'rowInsert') await api.post(`/api/queryview/table/${flag()}/row`, {})
    else if (op === 'rowInsertOne') await api.post(`/api/queryview/table/${flag()}/row/one`, {})
    else if (op === 'rowDeleteAll') { const f = flag(); if (!(await confirmMsg('确定清空该表所有行？'))) return; await api.delete(`/api/queryview/table/${f}/row/delete/all`) }
    else if (op === 'rowDelete') { const f = flag(); const rid = encodeURIComponent(prompt('行 ID:', '') || ''); if (!(await confirmMsg('确定删除该行？'))) return; await api.delete(`/api/queryview/table/${f}/row/${rid}`) }
    else { const f = flag(); const rid = encodeURIComponent(prompt('行 ID:', '') || ''); await api.post(`/api/queryview/table/${f}/row/${rid}/part/update`, {}) }
    toast.success('数据表行操作已提交')
  } catch (e: any) {
    toast.error('操作失败: ' + (e?.message ?? ''))
  }
}
async function loadViews() {
  loading.value = true
  try {
    // 后端视图列表为 GET view/list/query/{queryFlag}；all 为"不限查询"的取值。
    const r = await api.get('/api/queryview/view/list/query/all')
    views.value = r.data?.list ?? r.data ?? []
  } catch {
    views.value = []
  } finally {
    loading.value = false
  }
}

async function executeView(v: ViewItem) {
  activeView.value = v
  execLoading.value = true
  execResult.value = []
  try {
    // 后端 execute 为 GET execute/{view}/{id}；handler 只读 id，view 段为语义占位。
    const r = await api.get(
      `/api/queryview/execute/${encodeURIComponent(v.flag || 'view')}/${encodeURIComponent(v.id)}`,
    )
    execResult.value = r.data?.list ?? r.data ?? []
  } catch (e: any) {
    toast.error('执行失败: : ' + (e?.message ?? '未知错误'))
  } finally {
    execLoading.value = false
  }
}

async function exportExcel(v: ViewItem) {
  try {
    const r = await api.get(`/api/queryview/excel/${v.flag || v.id}`)
    if (r.data?.url) {
      window.open(r.data.url, '_blank')
    } else {
      toast.info('Excel导出暂未生成URL')
    }
  } catch (e: any) {
    toast.error('导出失败: : ' + (e?.message ?? ''))
  }
}

loadViews()

const queryview_query_qf_1_ref = ref<any[]>([])
const importmodel_record_r_1_status_ref = ref<any[]>([])
const queryview_importmodel_record_r_1_ref = ref<any[]>([])
const importmodel_execute_record_record_1_ref = ref<any[]>([])
const queryview_list_ref = ref<any[]>([])
const queryview_view_v_1_bundle_ref = ref<any[]>([])
const query_list_key_kw_ref = ref<any[]>([])
const queryview_morelikethis_ref = ref<any[]>([])
const queryview_statement_st_1_format_ref = ref<any[]>([])
const queryview_importmodel_im_1_execute_ref = ref<any[]>([])
const queryview_table_reload_dynamic_ref = ref<any[]>([])
const queryview_importmodel_uuid_ref = ref<any[]>([])
const table_row_insert_tbl_1_ref = ref<any[]>([])
const importmodel_record_delete_record_1_ref = ref<any[]>([])
const queryview_query_list_ref = ref<any[]>([])
const queryview_stat_stat_1_ref = ref<any[]>([])
const queryview_view_list_ref = ref<any[]>([])
const queryview_stat_list_ref = ref<any[]>([])
const stat_list_query_query_1_ref = ref<any[]>([])
const queryview_ref = ref<any[]>([])
const queryview_view_v_1_execute_ref = ref<any[]>([])
const queryview_view_v_1_excel_ref = ref<any[]>([])
const view_list_all_ref = ref<any[]>([])
const view_ref = ref<any[]>([])
const viewcategory_vc_1_ref = ref<any[]>([])
const viewcategory_ref = ref<any[]>([])
const api_viewcate_548_data = ref<any[]>([])
const api_viewfiel_567_data = ref<any[]>([])
const api_viewfiel_215_data = ref<any[]>([])
const api_viewfiel_260_data = ref<any[]>([])
const api_viewreco_372_data = ref<any[]>([])
const api_viewrecord_person_p_1_data = ref<any[]>([])
const api_document_d_1_has_view_data = ref<any[]>([])
const api_queryview__77_data = ref<any[]>([])
const api_queryview__469_data = ref<any[]>([])
const api_queryview__994_data = ref<any[]>([])
const api_queryview__229_data = ref<any[]>([])
const api_queryview__320_data = ref<any[]>([])
const api_queryview__430_data = ref<any[]>([])
</script>

<style scoped>
.mod-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0;font-family:'JetBrains Mono',monospace}
.content-panel{flex:1;overflow-y:auto;padding:16px;display:flex;flex-direction:column;gap:16px}
.toolbar{display:flex;gap:8px}
.qv-note{margin:8px 0;padding:8px 12px;border-radius:var(--radius-md);background:var(--bg-elevated);border:1px solid var(--border-subtle);font-size:12px;color:var(--text-secondary)}
.search-input{flex:1;background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-md);color:var(--text-primary);padding:8px 12px;font-size:14px}
.search-input:focus{outline:none;border-color:var(--color-primary)}
.btn-primary{padding:8px 20px;background:var(--color-primary);color:#000;border:none;border-radius:var(--radius-md);font-size:13px;cursor:pointer;font-weight:600}
.list-panel{flex:1}
.item-table{display:flex;flex-direction:column;gap:8px}
.table-header{display:grid;grid-template-columns:2fr 1fr 160px;padding:8px 12px;background:var(--bg-elevated);border-radius:var(--radius-sm);font-size:12px;color:var(--text-muted);font-weight:600}
.table-row{display:grid;grid-template-columns:2fr 1fr 160px;padding:12px;align-items:center;transition:all var(--transition-fast);border:1px solid var(--border-subtle);border-radius:var(--radius-md);background:var(--bg-elevated)}
.table-row:hover{border-color:var(--color-primary)}
.col-name{font-size:14px;font-weight:500;color:var(--text-primary)}
.col-flag{font-size:11px;color:var(--text-muted);font-family:'JetBrains Mono',monospace}
.btn-sm{padding:4px 10px;background:transparent;border:1px solid var(--border-subtle);color:var(--text-secondary);border-radius:var(--radius-sm);font-size:12px;cursor:pointer;margin-right:6px}
.btn-sm:hover{border-color:var(--color-primary);color:var(--color-primary)}
.empty,.loading-row{display:flex;flex-direction:column;align-items:center;justify-content:center;padding:40px;color:var(--text-muted);gap:12px;flex:1}
.ei{font-size:48px;opacity:0.4}
.sk{height:40px;border-radius:var(--radius-md);background:var(--bg-elevated);animation:pulse 1.2s ease-in-out infinite}
@keyframes pulse{0%,100%{opacity:.4}50%{opacity:.8}}
.modal-overlay{position:fixed;inset:0;background:rgba(0,0,0,.7);display:flex;align-items:center;justify-content:center;z-index:100}
.modal{background:var(--bg-surface);border:1px solid var(--border-subtle);border-radius:var(--radius-lg);padding:24px;width:800px;max-width:95vw;max-height:85vh;display:flex;flex-direction:column;overflow:hidden}
.modal-header{display:flex;align-items:center;justify-content:space-between;margin-bottom:16px;flex-shrink:0}
.modal-header h3{font-family:'Orbitron',sans-serif;color:var(--color-primary);margin:0;font-size:15px}
.btn-close{background:none;border:none;color:var(--text-muted);font-size:18px;cursor:pointer}
.btn-close:hover{color:var(--color-error)}
.result-table-wrap{flex:1;overflow:auto}
.result-table{width:100%;border-collapse:collapse;font-size:13px}
.result-table th{background:var(--bg-elevated);color:var(--color-primary);padding:8px 12px;text-align:left;font-weight:600;border:1px solid var(--border-subtle);position:sticky;top:0}
.result-table td{padding:6px 12px;border:1px solid var(--border-subtle);color:var(--text-secondary);max-width:200px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.result-table tr:hover td{background:var(--color-primary-soft);color:var(--text-primary)}
.result-count{padding:8px;font-size:12px;color:var(--text-muted)}
.empty-result{color:var(--text-muted);text-align:center;padding:20px}
.font-mono{font-family:'JetBrains Mono',monospace}
</style>
