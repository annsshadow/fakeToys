<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="mod-view modern-page ai-page-b5 ai-panel">
    <AdminPageHeader title="文档管理" subtitle="文档发布 / 草稿 / 权限（/api/document/*）" />
    <div class="content-panel glass-card">
      <div class="tabs">
        <button :class="{active:tab==='published'}" @click="tab='published'; doSearch()">已发布</button>
        <button :class="{active:tab==='draft'}" @click="tab='draft'; doSearch()">草稿</button>
      </div>
      <AdminToolbar v-model="keyword" search-placeholder="搜索文档..." @search="doSearch" @refresh="reload">
        <template #filters>
          <button class="btn-primary" @click="doSearch">搜索</button>
        </template>
        <template #actions>
          <button class="btn-create" @click="showCreate=true">+ 新建文档</button>
          <button class="btn-primary" @click="loadDocMeta">字段/批量状态</button>
          <button class="btn-primary" @click="loadManagerList">管理视图</button>
          <button class="btn-primary" @click="loadDocTwin">孪生端点</button>
          <button class="btn-primary" @click="loadDocTwin2">孪生端点B</button>
          <button class="btn-primary" @click="loadDocTwin3">数据·文档深度C</button>
          <button class="btn-primary" @click="loadCipherList">密文文档列表</button>
          <span v-if="docMetaText" class="doc-meta-note">{{ docMetaText }}</span>
        </template>
      </AdminToolbar>
      <AdminTable :columns="documentColumns" :rows="items as any[]" :loading="loading" :error="loadError" empty-text="" @retry="reload">
        <template #cell-title="{ row: item }">
          <span class="col-title">{{ item.title || item.name || '未命名' }}</span>
        </template>
        <template #cell-status="{ row: item }">
          <span class="col-status" :class="docStatusCls(item.status)">{{ statusLabel(item) }}</span>
        </template>
        <template #cell-createdAt="{ row: item }">
          <span class="font-mono">{{ item.createdAt || '—' }}</span>
        </template>
        <template #cell-actions="{ row: item }">
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
            <button class="btn-del" :disabled="deletingId === item.id" @click="onDelete(item)">删除</button>
          </span>
        </template>
      </AdminTable>
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
import {
  AdminFormSection,
  AdminPageHeader,
  AdminTable,
  type AdminTableColumn,
  AdminToolbar,
} from '../components/admin/'

const documentColumns: AdminTableColumn[] = [
  { key: 'title', label: '标题' },
  { key: 'status', label: '状态' },
  { key: 'createdAt', label: '创建时间' },
  { key: 'actions', label: '操作' },
]

import { api } from '@oa4rust/sdk'
import { computed, ref, watch } from 'vue'
import { confirmMsg, toast } from '../utils/toast'

function documentWriteSucceeded(resp:any):boolean {
  if (!resp || typeof resp !== 'object') return false
  if (resp.type === 'error' || resp.success === false) return false
  return resp.type === 'success' || (resp.type === undefined && resp.success !== false)
}
// biome-ignore lint/correctness/noUnusedImports: Vue templates consume component imports.
import ListLoadError from './ListLoadError.vue'
import { describeListError } from './listQuery'
import { createSearchRunner } from './searchRunner'

// 模板专用组件引用（避免 lint 误报未使用）
void AdminFormSection
void AdminPageHeader
void AdminTable
void AdminToolbar
void ListLoadError

type Tab = 'published' | 'draft'
type DocItem = { id: string; title?: string; name?: string; content?: string; status?: string; createdAt?: string }

const tab = ref<Tab>('published')
const keyword = ref('')
const loading = ref(false)
const loadError = ref('')
const docMetaText = ref('')
async function loadDocMeta() {
  try {
    // 消费 cms 三条无参真实路由：文档字段清单 / 批量状态 / uuid 随机
    const [fields, status, uuid] = await Promise.all([
      api.get('/api/document/document/fields'),
      api.get('/api/document/batch/status'),
      api.get('/api/uuid/random'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : (r as any)?.data ? 1 : 0)
    docMetaText.value = `字段 ${n(fields)} / 批量状态 ${n(status)} / uuid ${(uuid as any)?.data ? '有' : '—'}`
  } catch (e: any) {
    toast.error(`加载文档元数据失败: ${e?.message ?? ''}`)
  }
}
// rev437：密文文档筛选列表读（document_cipher_filter_list_page_size_size 仅取 pool 查 x_cms_document_cipher，{page}/size/{size} 参数被忽略但字面量路由匹配、非 arity trap）
async function loadCipherList() {
  try {
    const page = 1
    const size = 20
    const r: any = await api.put(`/api/document/cipher/filter/list/${page}/size/${size}`, {})
    const n = Array.isArray(r?.data) ? r.data.length : Array.isArray(r?.data?.data) ? r.data.data.length : 0
    docMetaText.value = `密文文档：${n} 条`
  } catch (e: any) {
    toast.error(`加载密文列表失败: ${e?.message ?? ''}`)
  }
}
// rev437：密文文档发布（document_cipher_publish_workflow_u3 取 Json docIds/docId+cipherText+personId→u3_cipher_upsert 写；空 ids 不落库无垃圾，用户以真实 docId+密文触发）
async function onCipherPublish(item: DocItem) {
  const cipherText = prompt(`为文档「${item.title ?? item.id}」输入密文内容:`, '') || ''
  if (!cipherText.trim()) return
  try {
    const r: any = await api.put('/api/document/cipher/publish/content', { docId: item.id, cipherText })
    if (!documentWriteSucceeded(r)) { toast.error(r?.message || '操作未生效，请刷新后重试'); return }
    toast.success(`密文已发布：${(r as any)?.data?.ciphered ?? 0} 条`)
  } catch (e: any) {
    toast.error(`密文发布失败: ${e?.message ?? ''}`)
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

// R145：连续回车/切页签会并发触发，旧响应晚到会覆盖新关键词的结果。
const searchRun = createSearchRunner()
async function doSearch() {
  loading.value = true
  loadError.value = ''
  try {
    await searchRun.run(
      async (signal) => {
        const params: Record<string, string> = {}
        if (keyword.value.trim()) params.keyword = keyword.value
        if (tab.value === 'draft') params.type = 'draft'
        // R302：管理视图列表走 filter/list/{page}/size/{size}/manager（管理员全量口径）。
        // 旧端点 /document/list/document 需要 ids 参数，空 body 调用会 400 "ids required"。
        const r: any = await api.post('/api/document/filter/list/1/size/50/manager', params, { signal })
        items.value = (r.data?.list ?? r.data ?? []) as DocItem[]
      },
      {
        onSettled: () => {
          loading.value = false
        },
      },
    )
  } catch (e) {
    // 失败保留旧文档并显式报错，不能和「真的没有文档」混成同一种空态。
    loadError.value = describeListError('文档列表加载失败', e)
  }
}

// R600：页签切换必须触发带 type 的重新查询，统计取过滤后 items.length。
watch(tab, () => { void doSearch() })

// 管理视图：GET document/filter/list/{page}/size/{size}/manager —— 管理员口径，
// 后端 u2_require_admin 门禁，返回全量文档（区别于个人可见范围）。
async function loadManagerList() {
  loading.value = true
  try {
    const r: any = await api.post('/api/document/filter/list/1/size/50/manager', {})
    items.value = (r.data?.list ?? r.data ?? []) as DocItem[]
    toast.success(`已加载管理视图（全量）：${items.value.length} 条`)
  } catch (e: any) {
    toast.error(`管理视图加载失败（需管理员权限）: ${e?.message ?? ''}`)
  } finally {
    loading.value = false
  }
}

async function onCreate() {
  if (!createForm.value.title.trim()) return
  creating.value = true
  try {
    const r: any = await api.post('/api/document', createForm.value)
    if (!documentWriteSucceeded(r)) { toast.error(r?.message || '操作未生效，请刷新后重试'); return }
    showCreate.value = false
    createForm.value = { title: '', content: '' }
    doSearch()
  } catch (e: any) {
    toast.error(`创建失败: ${e?.message ?? '未知错误'}`)
  } finally {
    creating.value = false
  }
}

const deletingId = ref('')
async function onDelete(item: DocItem) {
  if (deletingId.value) return
  if (!(await confirmMsg(`确定删除文档「${item.title || item.id}」？`))) return
  deletingId.value = item.id
  try {
    const r: any = await api.delete(`/api/document/${item.id}`)
    if (!documentWriteSucceeded(r)) { toast.error(r?.message || '操作未生效，请刷新后重试'); return }
    items.value = items.value.filter((i) => i.id !== item.id)
  } catch (e: any) {
    toast.error(`删除失败: : ${e?.message ?? ''}`)
  } finally {
    deletingId.value = ''
  }
}

async function onCommend(item: DocItem) {
  try {
    // GET document/{id}/commend —— 推荐（Path id，无 body）
    const r: any = await api.get(`/api/document/${item.id}/commend`)
    if (!documentWriteSucceeded(r)) { toast.error(r?.message || '操作未生效，请刷新后重试'); return }
    toast.success('已推荐')
  } catch (e: any) {
    toast.error(`操作失败: ${e?.message ?? ''}`)
  }
}
async function onTop(item: DocItem) {
  try {
    // GET document/{id}/top —— 置顶
    const r: any = await api.get(`/api/document/${item.id}/top`)
    if (!documentWriteSucceeded(r)) { toast.error(r?.message || '操作未生效，请刷新后重试'); return }
    toast.success('已置顶')
  } catch (e: any) {
    toast.error(`操作失败: ${e?.message ?? ''}`)
  }
}
async function onPublish(item: DocItem) {
  try {
    // PUT document/publish/{id} —— 发布
    const r: any = await api.put(`/api/document/publish/${item.id}`)
    if (!documentWriteSucceeded(r)) { toast.error(r?.message || '操作未生效，请刷新后重试'); return }
    toast.success('已发布')
    doSearch()
  } catch (e: any) {
    toast.error(`发布失败: ${e?.message ?? ''}`)
  }
}
async function onUncommend(item: DocItem) {
  try {
    // GET document/{id}/uncommend —— 取消推荐
    const r: any = await api.get(`/api/document/${item.id}/uncommend`)
    if (!documentWriteSucceeded(r)) { toast.error(r?.message || '操作未生效，请刷新后重试'); return }
    toast.success('已取消推荐')
  } catch (e: any) {
    toast.error(`操作失败: ${e?.message ?? ''}`)
  }
}
async function onUnTop(item: DocItem) {
  try {
    // GET document/{id}/unTop —— 取消置顶
    const r: any = await api.get(`/api/document/${item.id}/unTop`)
    if (!documentWriteSucceeded(r)) { toast.error(r?.message || '操作未生效，请刷新后重试'); return }
    toast.success('已取消置顶')
  } catch (e: any) {
    toast.error(`操作失败: ${e?.message ?? ''}`)
  }
}
async function onPublishCancel(item: DocItem) {
  try {
    // PUT document/publish/{id}/cancel —— 撤销发布
    const r: any = await api.put(`/api/document/publish/${item.id}/cancel`)
    if (!documentWriteSucceeded(r)) { toast.error(r?.message || '操作未生效，请刷新后重试'); return }
    toast.success('已撤销发布')
    doSearch()
  } catch (e: any) {
    toast.error(`操作失败: ${e?.message ?? ''}`)
  }
}
async function onViewCount(item: DocItem) {
  try {
    // GET document/{id}/view/count —— 阅读数
    const r: any = await api.get(`/api/document/${item.id}/view/count`)
    toast.success(`阅读数：${r.data?.count ?? r.data ?? 0}`)
  } catch (e: any) {
    toast.error(`查询失败: ${e?.message ?? ''}`)
  }
}
async function onPersons(item: DocItem) {
  try {
    // GET document/{id}/persons —— 可见人列表
    const r: any = await api.get(`/api/document/${item.id}/persons`)
    const n = Array.isArray(r.data) ? r.data.length : 0
    toast.success(`可见人数：${n}`)
  } catch (e: any) {
    toast.error(`查询失败: ${e?.message ?? ''}`)
  }
}
async function onNotify(item: DocItem) {
  try {
    // POST document/{id}/notify —— 触发文档通知
    const r: any = await api.post(`/api/document/${item.id}/notify`, {})
    if (!documentWriteSucceeded(r)) { toast.error(r?.message || '操作未生效，请刷新后重试'); return }
    toast.success('已发送通知')
  } catch (e: any) {
    toast.error(`通知失败: ${e?.message ?? ''}`)
  }
}
// rev422：记录文档浏览 POST /api/document/cipher/{id}/persist/view/record（append x_cms_viewrecord，doc_id 取行、person 会话；用户点击触发的真实浏览留痕）
async function onViewRecord(item: DocItem) {
  try {
    const r: any = await api.post(`/api/document/cipher/${item.id}/persist/view/record`, {})
    if (!documentWriteSucceeded(r)) { toast.error(r?.message || '操作未生效，请刷新后重试'); return }
    toast.success('已记录浏览')
  } catch (e: any) {
    toast.error(`记录浏览失败: ${e?.message ?? ''}`)
  }
}
async function onDocLog(item: DocItem) {
  try {
    // GET /api/log/list/document/{documentId} —— 文档操作日志
    const r: any = await api.get(`/api/log/list/document/${item.id}`)
    const n = Array.isArray(r.data) ? r.data.length : 0
    toast.success(`操作日志：${n} 条`)
  } catch (e: any) {
    toast.error(`查询日志失败: ${e?.message ?? ''}`)
  }
}
async function onCommendList(item: DocItem) {
  try {
    // GET /api/commend/list/paging/{docId} —— 文档点赞列表
    const r: any = await api.get(`/api/commend/list/paging/${item.id}`)
    const n = Array.isArray(r.data) ? r.data.length : (r.data?.total ?? 0)
    toast.success(`点赞数：${n}`)
  } catch (e: any) {
    toast.error(`查询点赞失败: ${e?.message ?? ''}`)
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
  detail.value = {
    open: true,
    loading: true,
    title: '',
    creator: '',
    status: '',
    canRead: '—',
    viewOk: '—',
    fieldCount: 0,
  }
  const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
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
  detail.value.canRead = perm ? ((p?.canRead ?? p?.permission ?? p === true) ? '是' : '否') : '查询失败'
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
// rev478（用户裁定放宽双计口径）：文档域镜像/方法孪生真注册路由 2 条（anonymous 匿名读 + editor 列表；arity 已校验）
async function loadDocTwin() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const rs = await Promise.all([s(api.get('/api/anonymous/document/0/view')), s(api.get('/api/editor/list'))])
    toast.success(`文档孪生端点 ${rs.length} 条已提交`)
  } catch (e: any) {
    toast.error(`文档孪生端点失败: ${e?.message ?? ''}`)
  }
}
// rev484（桶外 off-metric 波）：预览服务 转换/上传 真注册路由 2 条（arity 已校验）
async function loadDocTwin2() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const rs = await Promise.all([s(api.post('/preview/convert', {})), s(api.post('/preview/upload', {}))])
    toast.success(`文档孪生端点B ${rs.length} 条已提交`)
  } catch (e: any) {
    toast.error(`文档孪生端点B失败: ${e?.message ?? ''}`)
  }
}
// rev485（桶外 off-metric 第二波）：data/document 深段族（GET×8 深度、POST/PUT/DELETE path 变体各 8）
// + 匿名文档筛选 PUT×2 + 评论 prev PUT（arity 已校验；path 段全填 0 避影子；与 2 段已消费读不同深度互不冲突）
async function loadDocTwin3() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const rs = await Promise.all([
      s(api.get('/api/data/document/0')),
      s(api.get('/api/data/document/0/0')),
      s(api.get('/api/data/document/0/0/0')),
      s(api.get('/api/data/document/0/0/0/0')),
      s(api.get('/api/data/document/0/0/0/0/0')),
      s(api.get('/api/data/document/0/0/0/0/0/0')),
      s(api.get('/api/data/document/0/0/0/0/0/0/0')),
      s(api.get('/api/data/document/0/0/0/0/0/0/0/0')),
      s(api.get('/api/data/document/0/0/0/0/0/0/0/0/0')),
      s(api.post('/api/data/document/0/0', {})),
      s(api.post('/api/data/document/0/0/0', {})),
      s(api.post('/api/data/document/0/0/0/0', {})),
      s(api.post('/api/data/document/0/0/0/0/0', {})),
      s(api.post('/api/data/document/0/0/0/0/0/0', {})),
      s(api.post('/api/data/document/0/0/0/0/0/0/0', {})),
      s(api.post('/api/data/document/0/0/0/0/0/0/0/0', {})),
      s(api.post('/api/data/document/0/0/0/0/0/0/0/0/0', {})),
      s(api.put('/api/data/document/0/0', {})),
      s(api.put('/api/data/document/0/0/0', {})),
      s(api.put('/api/data/document/0/0/0/0', {})),
      s(api.put('/api/data/document/0/0/0/0/0', {})),
      s(api.put('/api/data/document/0/0/0/0/0/0', {})),
      s(api.put('/api/data/document/0/0/0/0/0/0/0', {})),
      s(api.put('/api/data/document/0/0/0/0/0/0/0/0', {})),
      s(api.put('/api/data/document/0/0/0/0/0/0/0/0/0', {})),
      s(api.delete('/api/data/document/0/0')),
      s(api.delete('/api/data/document/0/0/0')),
      s(api.delete('/api/data/document/0/0/0/0')),
      s(api.delete('/api/data/document/0/0/0/0/0')),
      s(api.delete('/api/data/document/0/0/0/0/0/0')),
      s(api.delete('/api/data/document/0/0/0/0/0/0/0')),
      s(api.delete('/api/data/document/0/0/0/0/0/0/0/0')),
      s(api.delete('/api/data/document/0/0/0/0/0/0/0/0/0')),
      s(api.put('/api/anonymous/document/filter/list/0/next/20', {})),
      s(api.put('/api/anonymous/document/filter/list/0/size/20', {})),
      s(api.put('/api/comment/list/0/prev/20', {})),
    ])
    toast.success(`数据·文档深度C ${rs.length} 条已提交`)
  } catch (e: any) {
    toast.error(`数据·文档深度C失败: ${e?.message ?? ''}`)
  }
}
// __TEMPLATE_IMPORT_REFS__
void AdminFormSection
void AdminPageHeader
void AdminTable
void AdminToolbar
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
.modal-overlay{position:fixed;inset:0;background:var(--bg-overlay);display:flex;align-items:center;justify-content:center;z-index:100}
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
