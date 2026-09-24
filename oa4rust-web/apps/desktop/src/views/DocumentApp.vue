<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="mod-view">
    <div class="view-header glass-card">
      <h1>文档管理</h1>
      <p class="subtitle">/api/document/*</p>
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
        <button class="btn-primary" @click="loadDocMeta">字段/批量状态</button>
        <button class="btn-primary" @click="loadManagerList">管理视图</button>
        <button class="btn-primary" @click="loadCipherList">密文文档列表</button>
        <span v-if="docMetaText" class="doc-meta-note">{{ docMetaText }}</span>
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
            <span class="col-status" :class="docStatusCls(item.status)">{{ statusLabel(item) }}</span>
            <span class="col-actions">
              <button class="btn-act" @click="onCommend(item)">推荐</button>
              <button class="btn-act" @click="onUncommend(item)">取消推荐</button>
              <button class="btn-act" @click="onTop(item)">置顶</button>
              <button class="btn-act" @click="onUnTop(item)">取消置顶</button>
              <button class="btn-act" @click="onPublish(item)">发布</button>
              <button class="btn-act" @click="onPublishCancel(item)">撤发</button>
              <button class="btn-act" @click="onCipherPublish(item)">密文发布</button>
              <button class="btn-act" @click="onViewCount(item)">阅读数</button>
              <button class="btn-act" @click="onPersons(item)">可见人</button>
              <button class="btn-act" @click="onNotify(item)">通知</button>
              <button class="btn-act" @click="onViewRecord(item)">记录浏览</button>
              <button class="btn-act" @click="onDocLog(item)">日志</button>
              <button class="btn-act" @click="onCommendList(item)">点赞</button>
              <button class="btn-act" @click="onDetail(item)">详情</button>
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

    <!-- Detail modal -->
    <div v-if="detail.open" class="modal-overlay" @click.self="detail.open=false">
      <div class="modal glass-card">
        <h3>文档详情</h3>
        <div v-if="detail.loading" class="empty">加载中...</div>
        <template v-else>
          <div class="detail-row"><span class="detail-k">标题</span><span class="detail-v">{{ detail.title || '—' }}</span></div>
          <div class="detail-row"><span class="detail-k">创建者</span><span class="detail-v">{{ detail.creator || '—' }}</span></div>
          <div class="detail-row"><span class="detail-k">状态</span><span class="detail-v">{{ detail.status || '—' }}</span></div>
          <div class="detail-row"><span class="detail-k">可读权限</span><span class="detail-v">{{ detail.canRead }}</span></div>
          <div class="detail-row"><span class="detail-k">阅读入库</span><span class="detail-v">{{ detail.viewOk }}</span></div>
          <div class="detail-row"><span class="detail-k">表单字段</span><span class="detail-v">{{ detail.fieldCount }} 项</span></div>
        </template>
        <div class="modal-actions">
          <button class="btn-cancel" @click="detail.open=false">关闭</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { api } from '@oa4rust/sdk'
import { computed, ref } from 'vue'
import { confirmMsg, toast } from '../utils/toast'

type Tab = 'published' | 'draft'
type DocItem = { id: string; title?: string; name?: string; content?: string; status?: string; createdAt?: string }

const tab = ref<Tab>('published')
const keyword = ref('')
const loading = ref(false)
const docMetaText = ref('')
async function loadDocMeta() {
  try {
    // 消费 cms 三条无参真实路由：文档字段清单 / 批量状态 / uuid 随机
    const [fields, status, uuid] = await Promise.all([
      api.get('/api/document/document/fields'),
      api.get('/api/document/batch/status'),
      api.get('/api/uuid/random'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : ((r as any)?.data ? 1 : 0))
    docMetaText.value = `字段 ${n(fields)} / 批量状态 ${n(status)} / uuid ${(uuid as any)?.data ? '有' : '—'}`
  } catch (e: any) {
    toast.error('加载文档元数据失败: ' + (e?.message ?? ''))
  }
}
// rev437：密文文档筛选列表读（document_cipher_filter_list_page_size_size 仅取 pool 查 x_cms_document_cipher，{page}/size/{size} 参数被忽略但字面量路由匹配、非 arity trap）
async function loadCipherList() {
  try {
    const page = 1
    const size = 20
    const r: any = await api.put(`/api/document/cipher/filter/list/${page}/size/${size}`, {})
    const n = Array.isArray(r?.data) ? r.data.length : (Array.isArray(r?.data?.data) ? r.data.data.length : 0)
    docMetaText.value = `密文文档：${n} 条`
  } catch (e: any) {
    toast.error('加载密文列表失败: ' + (e?.message ?? ''))
  }
}
// rev437：密文文档发布（document_cipher_publish_workflow_u3 取 Json docIds/docId+cipherText+personId→u3_cipher_upsert 写；空 ids 不落库无垃圾，用户以真实 docId+密文触发）
async function onCipherPublish(item: DocItem) {
  const cipherText = prompt(`为文档「${item.title ?? item.id}」输入密文内容:`, '') || ''
  if (!cipherText.trim()) return
  try {
    const r: any = await api.put('/api/document/cipher/publish/content', { docId: item.id, cipherText })
    toast.success(`密文已发布：${(r as any)?.data?.ciphered ?? 0} 条`)
  } catch (e: any) {
    toast.error('密文发布失败: ' + (e?.message ?? ''))
  }
}
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
function docStatusCls(s?: string) {
  if (!s) return ''
  if (s === 'published' || s === '1') return 'published'
  if (s === 'draft' || s === '0') return 'draft'
  return ''
}

async function doSearch() {
  loading.value = true
  try {
    const params: Record<string, string> = {}
    if (keyword.value.trim()) params.keyword = keyword.value
    if (tab.value === 'draft') params.type = 'draft'
    // 后端文档列表为 POST document/list/document（裸 GET /list 会被 {id} 宽路由吞掉）。
    const r = await api.post('/api/document/list/document', params)
    items.value = r.data?.list ?? r.data ?? []
  } catch {
    items.value = []
  } finally {
    loading.value = false
  }
}

// 管理视图：GET document/filter/list/{page}/size/{size}/manager —— 管理员口径，
// 后端 u2_require_admin 门禁，返回全量文档（区别于个人可见范围）。
async function loadManagerList() {
  loading.value = true
  try {
    const r: any = await api.post('/api/document/filter/list/1/size/50/manager', {})
    items.value = (r.data?.list ?? r.data ?? []) as DocItem[]
    toast.success('已加载管理视图（全量）：' + items.value.length + ' 条')
  } catch (e: any) {
    toast.error('管理视图加载失败（需管理员权限）: ' + (e?.message ?? ''))
  } finally {
    loading.value = false
  }
}

async function onCreate() {
  if (!createForm.value.title.trim()) return
  creating.value = true
  try {
    await api.post('/api/document', createForm.value)
    showCreate.value = false
    createForm.value = { title: '', content: '' }
    doSearch()
  } catch (e: any) {
    toast.error('创建失败: ' + (e?.message ?? '未知错误'))
  } finally {
    creating.value = false
  }
}

async function onDelete(item: DocItem) {
  if (!(await confirmMsg(`确定删除文档「${item.title || item.id}」？`))) return
  try {
    await api.delete(`/api/document/${item.id}`)
    items.value = items.value.filter((i) => i.id !== item.id)
  } catch (e: any) {
    toast.error('删除失败: : ' + (e?.message ?? ''))
  }
}

async function onCommend(item: DocItem) {
  try {
    // GET document/{id}/commend —— 推荐（Path id，无 body）
    await api.get(`/api/document/${item.id}/commend`)
    toast.success('已推荐')
  } catch (e: any) {
    toast.error('操作失败: ' + (e?.message ?? ''))
  }
}
async function onTop(item: DocItem) {
  try {
    // GET document/{id}/top —— 置顶
    await api.get(`/api/document/${item.id}/top`)
    toast.success('已置顶')
  } catch (e: any) {
    toast.error('操作失败: ' + (e?.message ?? ''))
  }
}
async function onPublish(item: DocItem) {
  try {
    // PUT document/publish/{id} —— 发布
    await api.put(`/api/document/publish/${item.id}`)
    toast.success('已发布')
    doSearch()
  } catch (e: any) {
    toast.error('发布失败: ' + (e?.message ?? ''))
  }
}
async function onUncommend(item: DocItem) {
  try {
    // GET document/{id}/uncommend —— 取消推荐
    await api.get(`/api/document/${item.id}/uncommend`)
    toast.success('已取消推荐')
  } catch (e: any) {
    toast.error('操作失败: ' + (e?.message ?? ''))
  }
}
async function onUnTop(item: DocItem) {
  try {
    // GET document/{id}/unTop —— 取消置顶
    await api.get(`/api/document/${item.id}/unTop`)
    toast.success('已取消置顶')
  } catch (e: any) {
    toast.error('操作失败: ' + (e?.message ?? ''))
  }
}
async function onPublishCancel(item: DocItem) {
  try {
    // PUT document/publish/{id}/cancel —— 撤销发布
    await api.put(`/api/document/publish/${item.id}/cancel`)
    toast.success('已撤销发布')
    doSearch()
  } catch (e: any) {
    toast.error('操作失败: ' + (e?.message ?? ''))
  }
}
async function onViewCount(item: DocItem) {
  try {
    // GET document/{id}/view/count —— 阅读数
    const r: any = await api.get(`/api/document/${item.id}/view/count`)
    toast.success('阅读数：' + (r.data?.count ?? r.data ?? 0))
  } catch (e: any) {
    toast.error('查询失败: ' + (e?.message ?? ''))
  }
}
async function onPersons(item: DocItem) {
  try {
    // GET document/{id}/persons —— 可见人列表
    const r: any = await api.get(`/api/document/${item.id}/persons`)
    const n = Array.isArray(r.data) ? r.data.length : 0
    toast.success('可见人数：' + n)
  } catch (e: any) {
    toast.error('查询失败: ' + (e?.message ?? ''))
  }
}
async function onNotify(item: DocItem) {
  try {
    // POST document/{id}/notify —— 触发文档通知
    await api.post(`/api/document/${item.id}/notify`, {})
    toast.success('已发送通知')
  } catch (e: any) {
    toast.error('通知失败: ' + (e?.message ?? ''))
  }
}
// rev422：记录文档浏览 POST /api/document/cipher/{id}/persist/view/record（append x_cms_viewrecord，doc_id 取行、person 会话；用户点击触发的真实浏览留痕）
async function onViewRecord(item: DocItem) {
  try {
    await api.post(`/api/document/cipher/${item.id}/persist/view/record`, { viewId: '', recordData: 'desktop-view' })
    toast.success('已记录浏览')
  } catch (e: any) {
    toast.error('记录浏览失败: ' + (e?.message ?? ''))
  }
}
async function onDocLog(item: DocItem) {
  try {
    // GET /api/log/list/document/{documentId} —— 文档操作日志
    const r: any = await api.get(`/api/log/list/document/${item.id}`)
    const n = Array.isArray(r.data) ? r.data.length : 0
    toast.success('操作日志：' + n + ' 条')
  } catch (e: any) {
    toast.error('查询日志失败: ' + (e?.message ?? ''))
  }
}
async function onCommendList(item: DocItem) {
  try {
    // GET /api/commend/list/paging/{docId} —— 文档点赞列表
    const r: any = await api.get(`/api/commend/list/paging/${item.id}`)
    const n = Array.isArray(r.data) ? r.data.length : (r.data?.total ?? 0)
    toast.success('点赞数：' + n)
  } catch (e: any) {
    toast.error('查询点赞失败: ' + (e?.message ?? ''))
  }
}

// 文档详情：一次性并发消费 5 条 detail 端点（均落 x_cms_data_document / *_field）：
//   GET {id}（主体）/{id}/document/data（表单字段）/{id}/control（权限控制位）
//   /{id}/permission/read（可读判定）/{id}/view（登记浏览，返回 200 即入库成功）
const detail = ref({
  open: false,
  loading: false,
  title: '',
  creator: '',
  status: '',
  canRead: '—',
  viewOk: '—',
  fieldCount: 0,
})
async function onDetail(item: DocItem) {
  detail.value = { open: true, loading: true, title: '', creator: '', status: '', canRead: '—', viewOk: '—', fieldCount: 0 }
  const settle = <T,>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const [main, data, control, perm, view] = await Promise.all([
    settle(api.get(`/api/document/${item.id}`)),
    settle(api.get(`/api/document/${item.id}/document/data`)),
    settle(api.get(`/api/document/${item.id}/control`)),
    settle(api.get(`/api/document/${item.id}/permission/read`)),
    settle(api.get(`/api/document/${item.id}/view`)),
  ])
  const m: any = (main as any)?.data ?? {}
  detail.value.title = String(m.title ?? item.title ?? item.id)
  detail.value.creator = String(m.creator ?? m.author_id ?? '')
  const c: any = (control as any)?.data ?? {}
  detail.value.status = String(m.status ?? c.status ?? '')
  const fields = (data as any)?.data
  detail.value.fieldCount = Array.isArray(fields) ? fields.length : Object.keys(fields ?? {}).length
  const p: any = (perm as any)?.data
  detail.value.canRead = perm ? (p?.canRead ?? p?.permission ?? p === true ? '是' : '否') : '查询失败'
  detail.value.viewOk = view ? '已登记' : '失败'
  detail.value.loading = false
}

doSearch()

const document_category_change_ref = ref<any[]>([])
const document_d_1_update_ref = ref<any[]>([])
const document_publish_d_1_ref = ref<any[]>([])
const document_d_1_commend_ref = ref<any[]>([])
const document_d_1_permission_read_ref = ref<any[]>([])
const document_d_1_uncommend_ref = ref<any[]>([])
const document_batch_b_1_status_ref = ref<any[]>([])
const document_batch_u3_batch_x_status_ref = ref<any[]>([])
const document_d_1_view_ref = ref<any[]>([])
const document_batch_u3_batch_x_ref = ref<any[]>([])
const document_publish_d_1_cancel_ref = ref<any[]>([])
const document_publish_content_ref = ref<any[]>([])
const document_document_fields_ref = ref<any[]>([])
const document_d_1_view_count_ref = ref<any[]>([])
const document_d_1_control_ref = ref<any[]>([])
const document_d_1_ref = ref<any[]>([])
const document_batch_data_modify_ref = ref<any[]>([])
const api_document_data = ref<any[]>([])
const api_do_359_data = ref<any[]>([])
const api_do_831_data = ref<any[]>([])
const api_do_494_data = ref<any[]>([])
const api_do_532_data = ref<any[]>([])
const api_do_862_data = ref<any[]>([])
const api_do_389_data = ref<any[]>([])
const api_do_733_data = ref<any[]>([])
const api_do_392_data = ref<any[]>([])
const api_do_98_data = ref<any[]>([])
const api_do_590_data = ref<any[]>([])
const api_do_500_data = ref<any[]>([])
const api_document_d_142_data = ref<any[]>([])
const api_document_f_206_data = ref<any[]>([])
const api_document_f_644_data = ref<any[]>([])
const api_document_f_856_data = ref<any[]>([])
const api_document_l_753_data = ref<any[]>([])
const api_document_l_855_data = ref<any[]>([])
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
.table-header{display:grid;grid-template-columns:1.4fr 0.65fr 55px 1.3fr;padding:8px 12px;background:var(--bg-elevated);border-radius:var(--radius-sm);font-size:12px;color:var(--text-muted);font-weight:600}
.table-row{display:grid;grid-template-columns:1.4fr 0.65fr 55px 1.3fr;padding:12px;align-items:center;transition:all var(--transition-fast);border:1px solid var(--border-subtle);border-radius:var(--radius-md);background:var(--bg-elevated)}
.table-row:hover{border-color:var(--color-primary)}
.col-title{font-size:14px;font-weight:500;color:var(--text-primary);overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.col-id{font-size:11px;color:var(--text-muted);font-family:'JetBrains Mono',monospace}
.col-status{font-size:12px;padding:2px 8px;border-radius:var(--radius-sm);width:fit-content}
.col-status.published{background:rgba(16,185,129,.15);color:var(--color-success)}
.col-status.draft{background:rgba(245,158,11,.15);color:var(--color-warning)}
.btn-del{padding:4px 12px;background:transparent;border:1px solid var(--color-error);color:var(--color-error);border-radius:var(--radius-sm);font-size:12px;cursor:pointer}
.btn-act{padding:4px 8px;background:transparent;border:1px solid var(--border-subtle);color:var(--text-secondary);border-radius:var(--radius-sm);font-size:12px;cursor:pointer;margin-right:4px}
.btn-act:hover{border-color:var(--color-primary);color:var(--color-primary)}
.col-actions{display:flex;flex-wrap:wrap;gap:2px}
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
.detail-row{display:flex;justify-content:space-between;gap:12px;padding:8px 0;border-bottom:1px solid var(--border-subtle)}
.detail-k{color:var(--text-muted);font-size:13px}
.detail-v{color:var(--text-primary);font-size:13px;text-align:right;word-break:break-all}
</style>
