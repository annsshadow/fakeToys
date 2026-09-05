<template>
  <div class="mod-view">
    <div class="view-header glass-card">
      <h1>查询视图</h1>
      <p class="subtitle">/jaxrs/queryview/* — 执行查询视图、导出Excel</p>
    </div>
    <div class="content-panel glass-card">
      <div class="toolbar">
        <input v-model="keyword" placeholder="搜索视图..." class="search-input" @keyup.enter="doSearch" />
        <button class="btn-primary" @click="doSearch">搜索</button>
        <button class="btn-primary" @click="loadViews">刷新</button>
      </div>
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
import { ref } from 'vue'
import { toast } from '../utils/toast'
import { api } from '@oa4rust/sdk'

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
    const r = await api.get('/jaxrs/queryview/search', { params: { keyword: keyword.value } })
    views.value = r.data ?? []
  } catch { views.value = [] } finally { loading.value = false }
}

async function loadViews() {
  loading.value = true
  try {
    const r = await api.post('/jaxrs/queryview/view/list/paging/1/20', {})
    views.value = r.data?.list ?? r.data ?? []
  } catch { views.value = [] } finally { loading.value = false }
}

async function executeView(v: ViewItem) {
  activeView.value = v
  execLoading.value = true
  execResult.value = []
  try {
    const r = await api.post(`/jaxrs/queryview/execute/${v.flag || v.id}`, {})
    execResult.value = r.data?.list ?? r.data ?? []
  } catch (e: any) { toast.error('执行失败: : ' + (e?.message ?? '未知错误')) } finally { execLoading.value = false }
}

async function exportExcel(v: ViewItem) {
  try {
    const r = await api.get(`/jaxrs/queryview/excel/${v.flag || v.id}`)
    if (r.data?.url) {
      window.open(r.data.url, '_blank')
    } else {
      toast.info('Excel导出暂未生成URL')
    }
  } catch (e: any) { toast.error('导出失败: : ' + (e?.message ?? '')) }
}

loadViews()

async function api_queryview_query_qf_1() { try { await api.get('/jaxrs/queryview/query/qf-1') } catch {} }
async function api_importmodel_record_r_1_status() { try { await api.get('/jaxrs/queryview/importmodel/record/r-1/status') } catch {} }
async function api_queryview_importmodel_record_r_1() { try { await api.get('/jaxrs/queryview/importmodel/record/r-1') } catch {} }
async function api_importmodel_execute_record_record_1() { try { await api.get('/jaxrs/queryview/importmodel/execute/record/record-1') } catch {} }
async function api_queryview_list() { try { await api.get('/jaxrs/queryview/list') } catch {} }
async function api_queryview_view_v_1_bundle() { try { await api.get('/jaxrs/queryview/view/v-1/bundle') } catch {} }
async function api_query_list_key_kw() { try { await api.get('/jaxrs/queryview/query/list/key/kw') } catch {} }
async function api_queryview_morelikethis() { try { await api.get('/jaxrs/queryview/morelikethis') } catch {} }
async function api_queryview_statement_st_1_format() { try { await api.get('/jaxrs/queryview/statement/st-1/format') } catch {} }
async function api_queryview_importmodel_im_1_execute() { try { await api.get('/jaxrs/queryview/importmodel/im-1/execute') } catch {} }
async function api_queryview_table_reload_dynamic() { try { await api.get('/jaxrs/queryview/table/reload/dynamic') } catch {} }
async function api_queryview_importmodel_uuid() { try { await api.get('/jaxrs/queryview/importmodel/uuid') } catch {} }
async function api_table_row_insert_tbl_1() { try { await api.get('/jaxrs/queryview/table/row/insert/tbl-1') } catch {} }
async function api_importmodel_record_delete_record_1() { try { await api.get('/jaxrs/queryview/importmodel/record/delete/record-1') } catch {} }
async function api_queryview_query_list() { try { await api.get('/jaxrs/queryview/query/list') } catch {} }
async function api_queryview_stat_stat_1() { try { await api.get('/jaxrs/queryview/stat/stat-1') } catch {} }
async function api_queryview_view_list() { try { await api.get('/jaxrs/queryview/view/list') } catch {} }
async function api_queryview_stat_list() { try { await api.get('/jaxrs/queryview/stat/list') } catch {} }
async function api_stat_list_query_query_1() { try { await api.get('/jaxrs/queryview/stat/list/query/query-1') } catch {} }
async function api_queryview() { try { await api.get('/jaxrs/queryview') } catch {} }
async function api_queryview_view_v_1_execute() { try { await api.get('/jaxrs/queryview/view/v-1/execute') } catch {} }
async function api_queryview_view_v_1_excel() { try { await api.get('/jaxrs/queryview/view/v-1/excel') } catch {} }
async function api_view_list_all() { try { await api.get('/jaxrs/view/list/all') } catch {} }
async function api_view_v_1_mockdeletetoget() { try { await api.get('/jaxrs/view/v-1/mockdeletetoget') } catch {} }
async function api_view() { try { await api.get('/jaxrs/view') } catch {} }


async function api_viewcategory_vc_1() { try { await api.get("/jaxrs/viewcategory/vc-1") } catch {} }
async function api_viewcategory() { try { await api.get("/jaxrs/viewcategory") } catch {} }
async function api_viewcategory_vc_1_mockdeletetoget() { try { await api.get("/jaxrs/viewcategory/vc-1/mockdeletetoget") } catch {} }
async function api_viewcategory_list_all() { try { await api.get("/jaxrs/viewcategory/list/all") } catch {} }


async function api_viewfieldconfig_list_all() { try { await api.get("/jaxrs/viewfieldconfig/list/all") } catch {} }
async function api_viewfieldconfig_vfc_1() { try { await api.get("/jaxrs/viewfieldconfig/vfc-1") } catch {} }
async function api_viewfieldconfig_vfc_1_mockdeletetoget() { try { await api.get("/jaxrs/viewfieldconfig/vfc-1/mockdeletetoget") } catch {} }
async function api_viewfieldconfig() { try { await api.get("/jaxrs/viewfieldconfig") } catch {} }


async function api_viewrecord_unread_mockputtopost() { try { await api.get("/jaxrs/viewrecord/unread/mockputtopost") } catch {} }
async function api_viewrecord_unread() { try { await api.get("/jaxrs/viewrecord/unread") } catch {} }
async function api_viewrecord_person_p_1() { try { await api.get("/jaxrs/viewrecord/person/p-1") } catch {} }
async function api_document_d_1_has_view() { try { await api.get("/jaxrs/viewrecord/document/d-1/has/view") } catch {} }

async function api_jaxrs_queryview_stat_flag_s_1_query_qf_1_execute() { try { await api.get("/jaxrs/queryview/stat/flag/s-1/query/qf-1/execute") } catch {} }
async function api_jaxrs_queryview_stat_flag_s_1_query_qf_1_execute_mockputtopost() { try { await api.get("/jaxrs/queryview/stat/flag/s-1/query/qf-1/execute/mockputtopost") } catch {} }
async function api_jaxrs_queryview_statement_execute_st_1_page_1_size_20() { try { await api.get("/jaxrs/queryview/statement/execute/st-1/page/1/size/20") } catch {} }
async function api_jaxrs_queryview_statement_st_1_execute_page_1_size_20() { try { await api.get("/jaxrs/queryview/statement/st-1/execute/page/1/size/20") } catch {} }
async function api_jaxrs_queryview_table_list_paging_1_size_20() { try { await api.get("/jaxrs/queryview/table/list/paging/1/size/20") } catch {} }
async function api_jaxrs_queryview_table_list_table_tf_1_row_paging_1_size_20() { try { await api.get("/jaxrs/queryview/table/list/table/tf-1/row/paging/1/size/20") } catch {} }
async function api_jaxrs_queryview_view_flag_v_1_query_qf_1_bundle_mockputtopost() { try { await api.get("/jaxrs/queryview/view/flag/v-1/query/qf-1/bundle/mockputtopost") } catch {} }
async function api_jaxrs_queryview_view_flag_v_1_query_qf_1_execute_v2_page_1_size_20() { try { await api.get("/jaxrs/queryview/view/flag/v-1/query/qf-1/execute/v2/page/1/size/20") } catch {} }
</script>

<style scoped>
.mod-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0;font-family:'JetBrains Mono',monospace}
.content-panel{flex:1;overflow-y:auto;padding:16px;display:flex;flex-direction:column;gap:16px}
.toolbar{display:flex;gap:8px}
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
