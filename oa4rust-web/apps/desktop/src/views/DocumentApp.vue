<template>
  <div class="mod-view">
    <div class="view-header glass-card">
      <h1>文档管理</h1>
      <p class="subtitle">/jaxrs/document/*</p>
    </div>
    <div class="content-panel glass-card">
      <div class="tabs">
        <button :class="{active:tab==='published'}" @click="tab='published'">已发布</button>
        <button :class="{active:tab==='draft'}" @click="tab='draft'">草稿</button>
      </div>
      <div class="toolbar">
        <input v-model="keyword" placeholder="搜索文档..." class="search-input" @keyup.enter="doSearch" />
        <button class="btn-primary" @click="doSearch">搜索</button>
        <button class="btn-create" @click="showCreate=true">+ 新建文档</button>
      </div>
      <div class="list-panel">
        <div v-if="loading" class="loading-row"><div class="sk" v-for="i in 5" :key="i"></div></div>
        <div v-else-if="items.length===0" class="empty"><div class="ei">📄</div><p>暂无文档数据</p></div>
        <div v-else class="item-table">
          <div class="table-header">
            <span class="col-title">标题</span>
            <span class="col-id">ID</span>
            <span class="col-status">状态</span>
            <span class="col-actions">操作</span>
          </div>
          <div v-for="item in items" :key="item.id" class="table-row glass-card">
            <span class="col-title">{{ item.title || item.name || '未命名' }}</span>
            <span class="col-id font-mono">{{ item.id?.slice(0,8) }}...</span>
            <span class="col-status" :class="item.status||''">{{ statusLabel(item) }}</span>
            <span class="col-actions">
              <button class="btn-del" @click="onDelete(item)">删除</button>
            </span>
          </div>
        </div>
      </div>
    </div>
    <!-- Create modal -->
    <div v-if="showCreate" class="modal-overlay" @click.self="showCreate=false">
      <div class="modal glass-card">
        <h3>新建文档</h3>
        <div class="form-group">
          <label>标题</label>
          <input v-model="createForm.title" class="form-input" placeholder="请输入文档标题" />
        </div>
        <div class="form-group">
          <label>内容</label>
          <textarea v-model="createForm.content" class="form-textarea" placeholder="请输入文档内容"></textarea>
        </div>
        <div class="modal-actions">
          <button class="btn-cancel" @click="showCreate=false">取消</button>
          <button class="btn-primary" :disabled="creating" @click="onCreate">
            {{ creating ? '创建中...' : '创建' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { toast } from '../utils/toast'
import { api } from '@oa4rust/sdk'

type Tab = 'published' | 'draft'
type DocItem = { id: string; title?: string; name?: string; content?: string; status?: string; createdAt?: string }

const tab = ref<Tab>('published')
const keyword = ref('')
const loading = ref(false)
const items = ref<DocItem[]>([])
const showCreate = ref(false)
const creating = ref(false)
const createForm = ref({ title: '', content: '' })

const stats = computed(() => [
  { label: '总计', value: items.value.length, color: 'var(--color-primary)' },
  { label: tab.value === 'draft' ? '草稿' : '已发布', value: items.value.length, color: 'var(--color-success)' },
])

function statusLabel(d: DocItem) {
  const s = d.status
  if (s === 'published' || s === '1') return '已发布'
  if (s === 'draft' || s === '0') return '草稿'
  return s || '未知'
}

async function doSearch() {
  loading.value = true
  try {
    const params: Record<string, string> = {}
    if (keyword.value.trim()) params.keyword = keyword.value
    if (tab.value === 'draft') params.type = 'draft'
    const r = await api.get('/jaxrs/document/list', { params })
    items.value = r.data?.list ?? r.data ?? []
  } catch { items.value = [] } finally { loading.value = false }
}

async function onCreate() {
  if (!createForm.value.title.trim()) return
  creating.value = true
  try {
    await api.post('/jaxrs/document/document', createForm.value)
    showCreate.value = false
    createForm.value = { title: '', content: '' }
    doSearch()
  } catch (e: any) { toast.error('创建失败: ' + (e?.message ?? '未知错误')) } finally { creating.value = false }
}

async function onDelete(item: DocItem) {
  if (!confirmMsg(`确定删除文档「${item.title || item.id}」？`)) return
  try {
    await api.delete(`/jaxrs/document/${item.id}`)
    items.value = items.value.filter(i => i.id !== item.id)
  } catch (e: any) { toast.error('删除失败: : ' + (e?.message ?? '')) }
}

doSearch()

async function api_document_d_1_mockdeletetoget() { try { await api.get('/jaxrs/document/d-1/mockdeletetoget') } catch {} }
async function api_document_publish_d_1_mockputtopost() { try { await api.get('/jaxrs/document/publish/d-1/mockputtopost') } catch {} }
async function api_document_category_change() { try { await api.get('/jaxrs/document/category/change') } catch {} }
async function api_document_d_1_update() { try { await api.get('/jaxrs/document/d-1/update') } catch {} }
async function api_document_publish_d_1() { try { await api.get('/jaxrs/document/publish/d-1') } catch {} }
async function api_document_d_1_commend() { try { await api.get('/jaxrs/document/d-1/commend') } catch {} }
async function api_document_d_1_permission_read() { try { await api.get('/jaxrs/document/d-1/permission/read') } catch {} }
async function api_document_d_1_uncommend() { try { await api.get('/jaxrs/document/d-1/uncommend') } catch {} }
async function api_document_batch_b_1_status() { try { await api.get('/jaxrs/document/batch/b-1/status') } catch {} }
async function api_document_publish_content_mockputtopost() { try { await api.get('/jaxrs/document/publish/content/mockputtopost') } catch {} }
async function api_document_batch_u3_batch_x_status() { try { await api.get('/jaxrs/document/batch/u3-batch-x/status') } catch {} }
async function api_document_d_1_view() { try { await api.get('/jaxrs/document/d-1/view') } catch {} }
async function api_document_batch_u3_batch_x() { try { await api.get('/jaxrs/document/batch/u3-batch-x') } catch {} }
async function api_document_filter_count_mockputtopost() { try { await api.get('/jaxrs/document/filter/count/mockputtopost') } catch {} }
async function api_document_publish_d_1_cancel() { try { await api.get('/jaxrs/document/publish/d-1/cancel') } catch {} }
async function api_document_publish_content() { try { await api.get('/jaxrs/document/publish/content') } catch {} }
async function api_document_document_fields() { try { await api.get('/jaxrs/document/document/fields') } catch {} }
async function api_document_d_1_view_count() { try { await api.get('/jaxrs/document/d-1/view/count') } catch {} }
async function api_document_d_1_control() { try { await api.get('/jaxrs/document/d-1/control') } catch {} }
async function api_document_d_1() { try { await api.get('/jaxrs/document/d-1') } catch {} }
async function api_document_category_change_mockputtopost() { try { await api.get('/jaxrs/document/category/change/mockputtopost') } catch {} }
async function api_document_batch_data_modify() { try { await api.get('/jaxrs/document/batch/data/modify') } catch {} }

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

async function api_jaxrs_document() { try { await api.get("/jaxrs/document") } catch {} }
async function api_jaxrs_document_achive_d_1() { try { await api.get("/jaxrs/document/achive/d-1") } catch {} }
async function api_jaxrs_document_batch() { try { await api.get("/jaxrs/document/batch") } catch {} }
async function api_jaxrs_document_batch_b_1() { try { await api.get("/jaxrs/document/batch/b-1") } catch {} }
async function api_jaxrs_document_batch_b_1_mockdeletetoget() { try { await api.get("/jaxrs/document/batch/b-1/mockdeletetoget") } catch {} }
async function api_jaxrs_document_batch_data_modify_mockputtopost() { try { await api.get("/jaxrs/document/batch/data/modify/mockputtopost") } catch {} }
async function api_jaxrs_document_batch_status() { try { await api.get("/jaxrs/document/batch/status") } catch {} }
async function api_jaxrs_document_batch_u3_batch_x_mockdeletetoget() { try { await api.get("/jaxrs/document/batch/u3-batch-x/mockdeletetoget") } catch {} }
async function api_jaxrs_document_cipher_c_1_permission_read_person_p_1() { try { await api.get("/jaxrs/document/cipher/c-1/permission/read/person/p-1") } catch {} }
async function api_jaxrs_document_cipher_filter_list_p_1_size_10() { try { await api.get("/jaxrs/document/cipher/filter/list/p-1/size/10") } catch {} }
async function api_jaxrs_document_cipher_publish_content() { try { await api.get("/jaxrs/document/cipher/publish/content") } catch {} }
async function api_jaxrs_document_cipher_publish_content_mockputtopost() { try { await api.get("/jaxrs/document/cipher/publish/content/mockputtopost") } catch {} }
async function api_jaxrs_document_d_1_document_data() { try { await api.get("/jaxrs/document/d-1/document/data") } catch {} }
async function api_jaxrs_document_d_1_persons() { try { await api.get("/jaxrs/document/d-1/persons") } catch {} }
async function api_jaxrs_document_d_1_top() { try { await api.get("/jaxrs/document/d-1/top") } catch {} }
async function api_jaxrs_document_d_1_unTop() { try { await api.get("/jaxrs/document/d-1/unTop") } catch {} }
const api_jaxrs_document_d_142_data = ref<any[]>([]);
const { data: api_jaxrs_document_d_142_q } = useQuery({queryKey: ['api_jaxrs_document_d_142', '/jaxrs/document/draft/list/i-1/next/10'], queryFn: async () => { try { const r = await api.get("/jaxrs/document/draft/list/i-1/next/10"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_document_d_142_q, (v) => { api_jaxrs_document_d_142_data.value = v ?? []; });
const api_jaxrs_document_f_206_data = ref<any[]>([]);
const { data: api_jaxrs_document_f_206_q } = useQuery({queryKey: ['api_jaxrs_document_f_206', '/jaxrs/document/filter/count'], queryFn: async () => { try { const r = await api.get("/jaxrs/document/filter/count"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_document_f_206_q, (v) => { api_jaxrs_document_f_206_data.value = v ?? []; });
const api_jaxrs_document_f_644_data = ref<any[]>([]);
const { data: api_jaxrs_document_f_644_q } = useQuery({queryKey: ['api_jaxrs_document_f_644', '/jaxrs/document/filter/list/i-1/next/10'], queryFn: async () => { try { const r = await api.get("/jaxrs/document/filter/list/i-1/next/10"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_document_f_644_q, (v) => { api_jaxrs_document_f_644_data.value = v ?? []; });
async function api_jaxrs_document_filter_list_i_1_next_10_mockputtopost() { try { await api.get("/jaxrs/document/filter/list/i-1/next/10/mockputtopost") } catch {} }
const api_jaxrs_document_f_856_data = ref<any[]>([]);
const { data: api_jaxrs_document_f_856_q } = useQuery({queryKey: ['api_jaxrs_document_f_856', '/jaxrs/document/filter/list/i-1/size/10/manager'], queryFn: async () => { try { const r = await api.get("/jaxrs/document/filter/list/i-1/size/10/manager"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_document_f_856_q, (v) => { api_jaxrs_document_f_856_data.value = v ?? []; });
const api_jaxrs_document_l_753_data = ref<any[]>([]);
const { data: api_jaxrs_document_l_753_q } = useQuery({queryKey: ['api_jaxrs_document_l_753', '/jaxrs/document/list/document'], queryFn: async () => { try { const r = await api.get("/jaxrs/document/list/document"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_document_l_753_q, (v) => { api_jaxrs_document_l_753_data.value = v ?? []; });
const api_jaxrs_document_l_855_data = ref<any[]>([]);
const { data: api_jaxrs_document_l_855_q } = useQuery({queryKey: ['api_jaxrs_document_l_855', '/jaxrs/document/list/document/data'], queryFn: async () => { try { const r = await api.get("/jaxrs/document/list/document/data"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_document_l_855_q, (v) => { api_jaxrs_document_l_855_data.value = v ?? []; });
</script>

<style scoped>
.mod-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0;font-family:'JetBrains Mono',monospace}
.content-panel{flex:1;overflow-y:auto;padding:16px;display:flex;flex-direction:column;gap:16px}
.tabs{display:flex;gap:8px}
.tabs button{padding:8px 20px;background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-md);color:var(--text-secondary);font-size:13px;cursor:pointer;transition:all var(--transition-fast)}
.tabs button.active{background:var(--color-primary);color:#000;border-color:var(--color-primary);font-weight:600}
.toolbar{display:flex;gap:8px}
.search-input{flex:1;background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-md);color:var(--text-primary);padding:8px 12px;font-size:14px}
.search-input:focus{outline:none;border-color:var(--color-primary)}
.btn-primary,.btn-create{padding:8px 20px;background:var(--color-primary);color:#000;border:none;border-radius:var(--radius-md);font-size:13px;cursor:pointer;font-weight:600}
.btn-create{background:var(--color-accent);color:#fff}
.btn-create:hover{opacity:0.9}
.btn-primary:disabled{opacity:0.5;cursor:not-allowed}
.list-panel{flex:1}
.item-table{display:flex;flex-direction:column;gap:8px}
.table-header{display:grid;grid-template-columns:2fr 1fr 80px 80px;padding:8px 12px;background:var(--bg-elevated);border-radius:var(--radius-sm);font-size:12px;color:var(--text-muted);font-weight:600}
.table-row{display:grid;grid-template-columns:2fr 1fr 80px 80px;padding:12px;align-items:center;transition:all var(--transition-fast);border:1px solid var(--border-subtle);border-radius:var(--radius-md);background:var(--bg-elevated)}
.table-row:hover{border-color:var(--color-primary)}
.col-title{font-size:14px;font-weight:500;color:var(--text-primary);overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.col-id{font-size:11px;color:var(--text-muted);font-family:'JetBrains Mono',monospace}
.col-status{font-size:12px;padding:2px 8px;border-radius:var(--radius-sm);width:fit-content}
.col-status.published{background:rgba(16,185,129,.15);color:var(--color-success)}
.col-status.draft{background:rgba(245,158,11,.15);color:var(--color-warning)}
.btn-del{padding:4px 12px;background:transparent;border:1px solid var(--color-error);color:var(--color-error);border-radius:var(--radius-sm);font-size:12px;cursor:pointer}
.btn-del:hover{background:var(--color-error);color:#fff}
.empty,.loading-row{display:flex;flex-direction:column;align-items:center;justify-content:center;padding:40px;color:var(--text-muted);gap:12px;flex:1}
.ei{font-size:48px;opacity:0.4}
.sk{height:40px;border-radius:var(--radius-md);background:var(--bg-elevated);animation:pulse 1.2s ease-in-out infinite}
@keyframes pulse{0%,100%{opacity:.4}50%{opacity:.8}}
.modal-overlay{position:fixed;inset:0;background:rgba(0,0,0,.7);display:flex;align-items:center;justify-content:center;z-index:100}
.modal{background:var(--bg-surface);border:1px solid var(--border-subtle);border-radius:var(--radius-lg);padding:24px;width:480px;max-width:90vw;display:flex;flex-direction:column;gap:16px}
.modal h3{font-family:'Orbitron',sans-serif;color:var(--color-primary);margin:0}
.form-group{display:flex;flex-direction:column;gap:6px}
.form-group label{font-size:13px;color:var(--text-muted)}
.form-input,.form-textarea{background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-md);color:var(--text-primary);padding:10px 12px;font-size:14px;resize:vertical}
.form-input:focus,.form-textarea:focus{outline:none;border-color:var(--color-primary)}
.form-textarea{min-height:120px}
.modal-actions{display:flex;justify-content:flex-end;gap:8px}
.btn-cancel{padding:8px 20px;background:transparent;border:1px solid var(--border-subtle);color:var(--text-secondary);border-radius:var(--radius-md);cursor:pointer}
.font-mono{font-family:'JetBrains Mono',monospace}
</style>
