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

const document_d_1_mockdeletetoget_ref = ref<any[]>([]);
const document_d_1_mockdeletetoget_q = useQuery({
  queryKey: ['document_d_1_mockdeletetoget'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/document/d-1/mockdeletetoget"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const document_publish_d_1_mockputtopost_ref = ref<any[]>([]);
const document_publish_d_1_mockputtopost_q = useQuery({
  queryKey: ['document_publish_d_1_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/document/publish/d-1/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const document_category_change_ref = ref<any[]>([]);
const document_category_change_q = useQuery({
  queryKey: ['document_category_change'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/document/category/change"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const document_d_1_update_ref = ref<any[]>([]);
const document_d_1_update_q = useQuery({
  queryKey: ['document_d_1_update'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/document/d-1/update"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const document_publish_d_1_ref = ref<any[]>([]);
const document_publish_d_1_q = useQuery({
  queryKey: ['document_publish_d_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/document/publish/d-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const document_d_1_commend_ref = ref<any[]>([]);
const document_d_1_commend_q = useQuery({
  queryKey: ['document_d_1_commend'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/document/d-1/commend"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const document_d_1_permission_read_ref = ref<any[]>([]);
const document_d_1_permission_read_q = useQuery({
  queryKey: ['document_d_1_permission_read'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/document/d-1/permission/read"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const document_d_1_uncommend_ref = ref<any[]>([]);
const document_d_1_uncommend_q = useQuery({
  queryKey: ['document_d_1_uncommend'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/document/d-1/uncommend"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const document_batch_b_1_status_ref = ref<any[]>([]);
const document_batch_b_1_status_q = useQuery({
  queryKey: ['document_batch_b_1_status'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/document/batch/b-1/status"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const document_publish_content_mockputtopost_ref = ref<any[]>([]);
const document_publish_content_mockputtopost_q = useQuery({
  queryKey: ['document_publish_content_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/document/publish/content/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const document_batch_u3_batch_x_status_ref = ref<any[]>([]);
const document_batch_u3_batch_x_status_q = useQuery({
  queryKey: ['document_batch_u3_batch_x_status'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/document/batch/u3-batch-x/status"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const document_d_1_view_ref = ref<any[]>([]);
const document_d_1_view_q = useQuery({
  queryKey: ['document_d_1_view'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/document/d-1/view"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const document_batch_u3_batch_x_ref = ref<any[]>([]);
const document_batch_u3_batch_x_q = useQuery({
  queryKey: ['document_batch_u3_batch_x'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/document/batch/u3-batch-x"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const document_filter_count_mockputtopost_ref = ref<any[]>([]);
const document_filter_count_mockputtopost_q = useQuery({
  queryKey: ['document_filter_count_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/document/filter/count/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const document_publish_d_1_cancel_ref = ref<any[]>([]);
const document_publish_d_1_cancel_q = useQuery({
  queryKey: ['document_publish_d_1_cancel'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/document/publish/d-1/cancel"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const document_publish_content_ref = ref<any[]>([]);
const document_publish_content_q = useQuery({
  queryKey: ['document_publish_content'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/document/publish/content"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const document_document_fields_ref = ref<any[]>([]);
const document_document_fields_q = useQuery({
  queryKey: ['document_document_fields'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/document/document/fields"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const document_d_1_view_count_ref = ref<any[]>([]);
const document_d_1_view_count_q = useQuery({
  queryKey: ['document_d_1_view_count'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/document/d-1/view/count"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const document_d_1_control_ref = ref<any[]>([]);
const document_d_1_control_q = useQuery({
  queryKey: ['document_d_1_control'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/document/d-1/control"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const document_d_1_ref = ref<any[]>([]);
const document_d_1_q = useQuery({
  queryKey: ['document_d_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/document/d-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const document_category_change_mockputtopost_ref = ref<any[]>([]);
const document_category_change_mockputtopost_q = useQuery({
  queryKey: ['document_category_change_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/document/category/change/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const document_batch_data_modify_ref = ref<any[]>([]);
const document_batch_data_modify_q = useQuery({
  queryKey: ['document_batch_data_modify'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/document/batch/data/modify"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});

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

const api_jaxrs_document_data = ref<any[]>([]);
const { data: api_jaxrs_document_q } = useQuery({queryKey: ['api_jaxrs_document', '/jaxrs/document'], queryFn: async () => { try { const r = await api.get("/jaxrs/document"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_document_q, (v) => { api_jaxrs_document_data.value = v ?? []; });
const api_jaxrs_do_359_data = ref<any[]>([]);
const { data: api_jaxrs_do_359_q } = useQuery({queryKey: ['api_jaxrs_do_359', '/jaxrs/document/achive/d-1'], queryFn: async () => { try { const r = await api.get("/jaxrs/document/achive/d-1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_do_359_q, (v) => { api_jaxrs_do_359_data.value = v ?? []; });
const api_jaxrs_do_831_data = ref<any[]>([]);
const { data: api_jaxrs_do_831_q } = useQuery({queryKey: ['api_jaxrs_do_831', '/jaxrs/document/batch'], queryFn: async () => { try { const r = await api.get("/jaxrs/document/batch"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_do_831_q, (v) => { api_jaxrs_do_831_data.value = v ?? []; });
const api_jaxrs_do_494_data = ref<any[]>([]);
const { data: api_jaxrs_do_494_q } = useQuery({queryKey: ['api_jaxrs_do_494', '/jaxrs/document/batch/b-1'], queryFn: async () => { try { const r = await api.get("/jaxrs/document/batch/b-1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_do_494_q, (v) => { api_jaxrs_do_494_data.value = v ?? []; });
const jaxrs_document_batch_b_1_mockdeletetoget_ref = ref<any[]>([]);
const jaxrs_document_batch_b_1_mockdeletetoget_q = useQuery({
  queryKey: ['jaxrs_document_batch_b_1_mockdeletetoget'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/document/batch/b-1/mockdeletetoget"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_document_batch_data_modify_mockputtopost_ref = ref<any[]>([]);
const jaxrs_document_batch_data_modify_mockputtopost_q = useQuery({
  queryKey: ['jaxrs_document_batch_data_modify_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/document/batch/data/modify/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_jaxrs_do_532_data = ref<any[]>([]);
const { data: api_jaxrs_do_532_q } = useQuery({queryKey: ['api_jaxrs_do_532', '/jaxrs/document/batch/status'], queryFn: async () => { try { const r = await api.get("/jaxrs/document/batch/status"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_do_532_q, (v) => { api_jaxrs_do_532_data.value = v ?? []; });
const jaxrs_document_batch_u3_batch_x_mockdeletetoget_ref = ref<any[]>([]);
const jaxrs_document_batch_u3_batch_x_mockdeletetoget_q = useQuery({
  queryKey: ['jaxrs_document_batch_u3_batch_x_mockdeletetoget'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/document/batch/u3-batch-x/mockdeletetoget"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_jaxrs_do_862_data = ref<any[]>([]);
const { data: api_jaxrs_do_862_q } = useQuery({queryKey: ['api_jaxrs_do_862', '/jaxrs/document/cipher/c-1/permission/read/person/p-1'], queryFn: async () => { try { const r = await api.get("/jaxrs/document/cipher/c-1/permission/read/person/p-1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_do_862_q, (v) => { api_jaxrs_do_862_data.value = v ?? []; });
const api_jaxrs_do_389_data = ref<any[]>([]);
const { data: api_jaxrs_do_389_q } = useQuery({queryKey: ['api_jaxrs_do_389', '/jaxrs/document/cipher/filter/list/p-1/size/10'], queryFn: async () => { try { const r = await api.get("/jaxrs/document/cipher/filter/list/p-1/size/10"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_do_389_q, (v) => { api_jaxrs_do_389_data.value = v ?? []; });
const api_jaxrs_do_733_data = ref<any[]>([]);
const { data: api_jaxrs_do_733_q } = useQuery({queryKey: ['api_jaxrs_do_733', '/jaxrs/document/cipher/publish/content'], queryFn: async () => { try { const r = await api.get("/jaxrs/document/cipher/publish/content"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_do_733_q, (v) => { api_jaxrs_do_733_data.value = v ?? []; });
const jaxrs_document_cipher_publish_content_mockputtopost_ref = ref<any[]>([]);
const jaxrs_document_cipher_publish_content_mockputtopost_q = useQuery({
  queryKey: ['jaxrs_document_cipher_publish_content_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/document/cipher/publish/content/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_jaxrs_do_392_data = ref<any[]>([]);
const { data: api_jaxrs_do_392_q } = useQuery({queryKey: ['api_jaxrs_do_392', '/jaxrs/document/d-1/document/data'], queryFn: async () => { try { const r = await api.get("/jaxrs/document/d-1/document/data"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_do_392_q, (v) => { api_jaxrs_do_392_data.value = v ?? []; });
const api_jaxrs_do_98_data = ref<any[]>([]);
const { data: api_jaxrs_do_98_q } = useQuery({queryKey: ['api_jaxrs_do_98', '/jaxrs/document/d-1/persons'], queryFn: async () => { try { const r = await api.get("/jaxrs/document/d-1/persons"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_do_98_q, (v) => { api_jaxrs_do_98_data.value = v ?? []; });
const api_jaxrs_do_590_data = ref<any[]>([]);
const { data: api_jaxrs_do_590_q } = useQuery({queryKey: ['api_jaxrs_do_590', '/jaxrs/document/d-1/top'], queryFn: async () => { try { const r = await api.get("/jaxrs/document/d-1/top"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_do_590_q, (v) => { api_jaxrs_do_590_data.value = v ?? []; });
const api_jaxrs_do_500_data = ref<any[]>([]);
const { data: api_jaxrs_do_500_q } = useQuery({queryKey: ['api_jaxrs_do_500', '/jaxrs/document/d-1/unTop'], queryFn: async () => { try { const r = await api.get("/jaxrs/document/d-1/unTop"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_do_500_q, (v) => { api_jaxrs_do_500_data.value = v ?? []; });
const api_jaxrs_document_d_142_data = ref<any[]>([]);
const { data: api_jaxrs_document_d_142_q } = useQuery({queryKey: ['api_jaxrs_document_d_142', '/jaxrs/document/draft/list/i-1/next/10'], queryFn: async () => { try { const r = await api.get("/jaxrs/document/draft/list/i-1/next/10"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_document_d_142_q, (v) => { api_jaxrs_document_d_142_data.value = v ?? []; });
const api_jaxrs_document_f_206_data = ref<any[]>([]);
const { data: api_jaxrs_document_f_206_q } = useQuery({queryKey: ['api_jaxrs_document_f_206', '/jaxrs/document/filter/count'], queryFn: async () => { try { const r = await api.get("/jaxrs/document/filter/count"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_document_f_206_q, (v) => { api_jaxrs_document_f_206_data.value = v ?? []; });
const api_jaxrs_document_f_644_data = ref<any[]>([]);
const { data: api_jaxrs_document_f_644_q } = useQuery({queryKey: ['api_jaxrs_document_f_644', '/jaxrs/document/filter/list/i-1/next/10'], queryFn: async () => { try { const r = await api.get("/jaxrs/document/filter/list/i-1/next/10"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_document_f_644_q, (v) => { api_jaxrs_document_f_644_data.value = v ?? []; });
const jaxrs_document_filter_list_i_1_next_10_mockputtopost_ref = ref<any[]>([]);
const jaxrs_document_filter_list_i_1_next_10_mockputtopost_q = useQuery({
  queryKey: ['jaxrs_document_filter_list_i_1_next_10_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/document/filter/list/i-1/next/10/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
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
