<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="work-view">
    <div class="view-header glass-card">
      <div>
        <h1>工作流待办</h1>
        <p class="subtitle">真实任务、表单定义与流程数据闭环</p>
      </div>
      <div class="header-actions">
        <button class="btn-sm" @click="loadDrafts">草稿箱</button>
        <button class="btn-sm" @click="loadHandovers">工作交接</button>
        <button class="btn-sm" @click="loadReviews">审阅记录</button>
        <button class="btn-sm" @click="loadSerials">流水号</button>
        <button class="btn-sm" @click="loadWorkV2">全部工作</button>
        <button class="btn-sm" @click="loadReadLists">待阅/已阅</button>
        <button class="btn-sm primary" @click="openStart">发起流程</button>
      </div>
    </div>
    <p v-if="draftText" class="subtitle draft-note">{{ draftText }}</p>
    <p v-if="handoverText" class="subtitle draft-note">{{ handoverText }}</p>
    <p v-if="reviewText" class="subtitle draft-note">{{ reviewText }}</p>
    <p v-if="serialText" class="subtitle draft-note">{{ serialText }}</p>
    <p v-if="workV2Text" class="subtitle draft-note">{{ workV2Text }}</p>
    <p v-if="readListText" class="subtitle draft-note">{{ readListText }}</p>
    <div class="tabs glass-card">
      <button
        v-for="tab in tabs"
        :key="tab.key"
        class="tab-btn"
        :class="{ active: activeTab === tab.key }"
        @click="activeTab = tab.key"
      >
        {{ tab.label }}
      </button>
    </div>
    <div class="content-panel glass-card">
      <div v-if="query.isLoading.value" class="state">加载中...</div>
      <div v-else-if="query.error.value" class="state error-state">
        加载失败: {{ (query.error.value as Error)?.message }}
        <button class="btn-sm" @click="query.refetch()">重试</button>
      </div>
      <div v-else-if="!items.length" class="state">暂无任务</div>
      <div v-else class="item-list">
        <article v-for="item in items" :key="item.id" class="item-card">
          <div class="item-body" @click="openWork(item)">
            <div class="item-title">{{ item.title || item.processName || item.id }}</div>
            <div class="item-meta">
              <span>{{ item.appName || item.applicationName }}</span>
              <span>{{ item.processName }}</span>
              <span>{{ fmtTime(item.createTime) }}</span>
            </div>
          </div>
          <button class="btn-sm primary" @click="openWork(item)">{{ activeTab === 'pending' ? '办理' : '查看' }}</button>
        </article>
      </div>
    </div>

    <div v-if="opened" class="modal-overlay" @click.self="closeWork">
      <section class="work-dialog glass-card">
        <header>
          <div><h2>{{ opened.title || opened.processName || '流程办理' }}</h2><p>{{ opened.id }}</p></div>
          <button class="btn-sm" @click="closeWork">关闭</button>
        </header>
        <div v-if="detailLoading" class="state">正在加载表单...</div>
        <div v-else-if="detailError" class="state error-state">{{ detailError }}</div>
        <XformRuntime
          v-else-if="formDefinition"
          v-model="formValues"
          :definition="formDefinition"
          :errors="formErrors"
          :readonly="activeTab !== 'pending' && !handleTaskId"
        />
        <div v-else class="state">当前工作没有可渲染的表单定义</div>

        <section v-if="!detailLoading && !detailError" class="detail-panels">
          <div class="detail-block">
            <h3>附件 ({{ attachments.length }})</h3>
            <ul v-if="attachments.length" class="detail-list">
              <li v-for="att in attachments" :key="att.id" class="clickable" @click="viewAttachment(att.id)">
                <span class="name">{{ att.name || att.id }}</span>
                <span class="muted">{{ att.extension }} · {{ fmtSize(att.length) }}</span>
              </li>
            </ul>
            <p v-else class="muted">无附件</p>
            <p v-if="attachDetailText" class="muted">{{ attachDetailText }}</p>
          </div>
          <div class="detail-block">
            <h3>流转记录 ({{ records.length }})</h3>
            <ul v-if="records.length" class="detail-list">
              <li v-for="rec in records" :key="rec.id">
                <span class="name">{{ rec.title || rec.id }}</span>
                <span class="muted">{{ fmtTime(rec.createTime) }}</span>
              </li>
            </ul>
            <p v-else class="muted">无流转记录</p>
          </div>
          <div class="detail-block">
            <h3>工作日志 ({{ worklogs.length }})</h3>
            <ul v-if="worklogs.length" class="detail-list">
              <li v-for="log in worklogs" :key="log.id">
                <span class="name">{{ log.activityName || log.title || log.id }}</span>
                <span class="muted">{{ log.person }} · {{ fmtTime(log.createTime) }}</span>
              </li>
            </ul>
            <p v-else class="muted">无工作日志</p>
          </div>
          <div class="detail-block">
            <h3>待阅 ({{ reads.length }})</h3>
            <ul v-if="reads.length" class="detail-list">
              <li v-for="rd in reads" :key="rd.id" class="clickable" @click="viewRead(rd.id)">
                <span class="name">{{ rd.person || rd.id }}</span>
                <span class="muted">{{ fmtTime(rd.createTime) }}</span>
              </li>
            </ul>
            <p v-else class="muted">无待阅记录</p>
            <p v-if="readDetailText" class="muted">{{ readDetailText }}</p>
          </div>
          <div v-if="engineText" class="detail-block">
            <h3>引擎明细</h3>
            <p class="muted">{{ engineText }}</p>
          </div>
          <div v-if="surfaceExtraText" class="detail-block">
            <h3>附件/文档版本</h3>
            <p class="muted">{{ surfaceExtraText }}</p>
          </div>
          <div v-if="effectiveTaskId" class="detail-block">
            <h3>任务信息</h3>
            <ul class="detail-list">
              <li><span class="name">任务</span><span class="muted">{{ (taskInfo?.title as string) || (taskInfo?.activity as string) || effectiveTaskId }}</span></li>
              <li v-if="taskInfo?.person"><span class="name">处理人</span><span class="muted">{{ taskInfo?.person }}</span></li>
              <li v-if="taskExpireText"><span class="name">超时</span><span class="muted">{{ taskExpireText }}</span></li>
              <li v-if="taskV2Status"><span class="name">v2 状态</span><span class="muted">{{ taskV2Status }}</span></li>
            </ul>
            <button class="btn-sm" :disabled="pressing" @click="pressTask">{{ pressing ? '催办中…' : '催办' }}</button>
            <button class="btn-sm" :disabled="v2Busy" @click="taskV2Action('pause')">暂停</button>
            <button class="btn-sm" :disabled="v2Busy" @click="taskV2Action('resume')">恢复</button>
            <button class="btn-sm" :disabled="v2Busy" @click="taskV2Action('reset')">重置</button>
          </div>
        </section>

        <textarea v-if="canHandle" v-model="opinion" class="opinion" placeholder="处理意见" aria-label="处理意见" />
        <footer v-if="canHandle">
          <button class="btn-sm reject" :disabled="submitting" @click="submit('reject')">驳回</button>
          <button class="btn-sm primary" :disabled="submitting" @click="submit('approve')">审批通过</button>
        </footer>
      </section>
    </div>

    <div v-if="showStart" class="modal-overlay" @click.self="showStart = false">
      <section class="work-dialog glass-card">
        <header>
          <div><h2>发起流程</h2><p>从零创建工作实例并填报表单</p></div>
          <button class="btn-sm" @click="closeStart">关闭</button>
        </header>
        <div v-if="startLoading" class="state">加载流程列表...</div>
        <template v-else>
          <div class="start-row">
            <label for="start-process">流程</label>
            <select id="start-process" v-model="startProcessId" @change="onStartProcessChange">
              <option value="">请选择流程</option>
              <option v-for="proc in startProcesses" :key="proc.id" :value="proc.id">{{ proc.name }}</option>
            </select>
          </div>
          <div class="start-row">
            <label for="start-title">标题</label>
            <input id="start-title" v-model="startTitle" placeholder="工作标题" />
          </div>
          <XformRuntime
            v-if="startDefinition"
            v-model="startValues"
            :definition="startDefinition"
            :errors="startErrors"
          />
          <div v-else-if="startProcessId" class="state">当前流程未绑定表单，可直接发起</div>
          <footer>
            <button class="btn-sm primary" :disabled="!startProcessId || !startTitle.trim() || startSubmitting" @click="submitStart">
              {{ startSubmitting ? '发起中…' : '发起' }}
            </button>
          </footer>
        </template>
      </section>
    </div>
  </div>
</template>

<script setup lang="ts">
import { api } from '@oa4rust/sdk'
import { useQuery, useQueryClient } from '@tanstack/vue-query'
import { computed, ref } from 'vue'
// biome-ignore lint/correctness/noUnusedImports: Vue templates consume component imports.
import XformRuntime from '../components/XformRuntime.vue'
import {
  type FormValue,
  initialFormValues,
  parseFormDefinition,
  validateFormValues,
  type XformDefinition,
} from '../contracts/xform'
import { toast } from '../utils/toast'

interface TaskItem {
  id: string
  work?: string
  workId?: string
  title?: string
  processName?: string
  appName?: string
  applicationName?: string
  createTime?: string
  [key: string]: unknown
}

type TabKey = 'pending' | 'completed' | 'started'
const tabs = [
  { key: 'pending' as const, label: '待我处理' },
  { key: 'completed' as const, label: '已完成' },
  { key: 'started' as const, label: '我发起的' },
]
const activeTab = ref<TabKey>('pending')
const queryClient = useQueryClient()
const endpoints: Record<TabKey, string> = {
  pending: '/api/processplatform/assemble/surface/task/list/my/paging/1/size/20',
  completed: '/api/processplatform/assemble/surface/taskcompleted/list/my/paging/1/size/20',
  started: '/api/processplatform/assemble/surface/work/list/my/paging/1/size/20',
}
const query = useQuery({
  queryKey: ['process-work', activeTab],
  queryFn: async () => {
    // 「我发起的」work 列表在 o2server 契约为 POST（task/taskcompleted 为 GET），按 tab 分流
    const response: any =
      activeTab.value === 'started'
        ? await api.post(endpoints[activeTab.value])
        : await api.get(endpoints[activeTab.value])
    return (response?.data?.data ?? response?.data ?? []) as TaskItem[]
  },
  staleTime: 30_000,
})
const items = computed(() => query.data.value ?? [])
const opened = ref<TaskItem | null>(null)
const formDefinition = ref<XformDefinition | null>(null)
const formValues = ref<Record<string, FormValue>>({})
const formErrors = ref<Record<string, string>>({})
const detailLoading = ref(false)
const detailError = ref('')
const opinion = ref('')
const submitting = ref(false)
const handleTaskId = ref('')

interface AttachmentItem { id: string; name?: string; extension?: string; length?: number }
interface RecordItem { id: string; title?: string; createTime?: string }
interface WorklogItem { id: string; title?: string; activityName?: string; person?: string; createTime?: string }
interface ReadItem { id: string; person?: string; createTime?: string }
const attachments = ref<AttachmentItem[]>([])
const records = ref<RecordItem[]>([])
const worklogs = ref<WorklogItem[]>([])
const reads = ref<ReadItem[]>([])
const engineText = ref('')
const surfaceExtraText = ref('')
const draftText = ref('')

// 草稿箱（rev175，surface 域 3 条真实 distinct）：draft/list/my/paging/{page}/{size}/{size}
// （PP_C_DRAFT 分页）→ 首草稿 id → draft/{id}（xid 详情 query_opt）+ draft/list/next/{id}/{count}（xid 游标）。
async function loadDrafts(): Promise<void> {
  draftText.value = ''
  const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const listRes = await settle(api.get('/api/processplatform/assemble/surface/draft/list/my/paging/1/20/20'))
  const rows = asRows(listRes)
  const firstId = rows[0] ? String(rows[0].id ?? '') : ''
  let detailText = '—'
  if (firstId) {
    const [detail, next] = await Promise.all([
      settle(api.get(`/api/processplatform/assemble/surface/draft/${firstId}`)),
      settle(api.get(`/api/processplatform/assemble/surface/draft/list/next/${firstId}/20`)),
    ])
    const dTitle = (detail as any)?.data?.title ?? firstId
    const nextN = asRows(next).length
    detailText = `首草稿「${dTitle}」· 后续 ${nextN}`
  }
  draftText.value = `我的草稿 ${rows.length} · ${detailText}`
}

const handoverText = ref('')
// 工作交接（rev176，surface 域 3 条真实 distinct，PP_C_HANDOVER）：handover/list/paging/{page}/{size}/{size}
// → 首交接 id → handover/{id}（xid 详情）+ handover/process/{id}（xid 处理信息）。
async function loadHandovers(): Promise<void> {
  handoverText.value = ''
  const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const listRes = await settle(api.get('/api/processplatform/assemble/surface/handover/list/paging/1/20/20'))
  const rows = asRows(listRes)
  const firstId = rows[0] ? String(rows[0].id ?? '') : ''
  let detailText = '—'
  if (firstId) {
    const [detail, proc] = await Promise.all([
      settle(api.get(`/api/processplatform/assemble/surface/handover/${firstId}`)),
      settle(api.get(`/api/processplatform/assemble/surface/handover/process/${firstId}`)),
    ])
    const dTitle = (detail as any)?.data?.title ?? firstId
    const hasProc = (proc as any)?.data?.id ? '有' : '无'
    detailText = `首交接「${dTitle}」· 处理信息 ${hasProc}`
  }
  handoverText.value = `工作交接 ${rows.length} · ${detailText}`
}

const reviewText = ref('')
// 审阅记录（rev177，surface 域 3 条真实 distinct，PP_C_REVIEW）：review/v2/list/paging/{page}/{size}/{size}
// → 首审阅 id → review/{id}（xid 详情）+ review/v2/list/next/{id}/{count}（xid 游标）。
async function loadReviews(): Promise<void> {
  reviewText.value = ''
  const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const listRes = await settle(api.get('/api/processplatform/assemble/surface/review/v2/list/paging/1/20/20'))
  const rows = asRows(listRes)
  const firstId = rows[0] ? String(rows[0].id ?? '') : ''
  let detailText = '—'
  if (firstId) {
    const [detail, next] = await Promise.all([
      settle(api.get(`/api/processplatform/assemble/surface/review/${firstId}`)),
      settle(api.get(`/api/processplatform/assemble/surface/review/v2/list/next/${firstId}/20`)),
    ])
    const dTitle = (detail as any)?.data?.title ?? firstId
    const nextN = asRows(next).length
    detailText = `首审阅「${dTitle}」· 后续 ${nextN}`
  }
  reviewText.value = `审阅记录 ${rows.length} · ${detailText}`
}

const serialText = ref('')
// 流水号（rev178，surface 域 3 条真实 distinct，PP_C_SERIALNUMBER）：serialnumber/list/paging/{page}/{size}/{size}
// → 首流水号 id/application → serialnumber/{id}（xid 详情）+ serialnumber/list/application/{applicationFlag}（xapplication 列表）。
async function loadSerials(): Promise<void> {
  serialText.value = ''
  const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const listRes = await settle(api.get('/api/processplatform/assemble/surface/serialnumber/list/paging/1/20/20'))
  const rows = asRows(listRes)
  const first = rows[0] ?? null
  const firstId = first ? String(first.id ?? '') : ''
  const appFlag = first ? String((first.application as string) ?? '') : ''
  let detailText = '—'
  if (firstId) {
    const [detail, byApp] = await Promise.all([
      settle(api.get(`/api/processplatform/assemble/surface/serialnumber/${firstId}`)),
      appFlag
        ? settle(api.get(`/api/processplatform/assemble/surface/serialnumber/list/application/${encodeURIComponent(appFlag)}`))
        : Promise.resolve(null),
    ])
    const dName = (detail as any)?.data?.name ?? firstId
    const appN = asRows(byApp).length
    detailText = `首流水号「${dName}」· 同应用 ${appN}`
  }
  serialText.value = `流水号 ${rows.length} · ${detailText}`
}

const workV2Text = ref('')
// 全部工作 v2（rev179，surface 域 3 条真实 distinct，PP_C_WORK）：work/v2/list/paging/{page}/{size}/{size}
// → 首工作 id → work/v2/list/next/{id}/{count}（xid 游标）+ work/v2/workorworkcompleted/{flag}（按工作 id 取详情）。
async function loadWorkV2(): Promise<void> {
  workV2Text.value = ''
  const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const listRes = await settle(api.get('/api/processplatform/assemble/surface/work/v2/list/paging/1/20/20'))
  const rows = asRows(listRes)
  const firstId = rows[0] ? String(rows[0].id ?? '') : ''
  let detailText = '—'
  if (firstId) {
    const [next, detail] = await Promise.all([
      settle(api.get(`/api/processplatform/assemble/surface/work/v2/list/next/${firstId}/20`)),
      settle(api.get(`/api/processplatform/assemble/surface/work/v2/workorworkcompleted/${firstId}`)),
    ])
    const nextN = asRows(next).length
    const dTitle = (detail as any)?.data?.title ?? firstId
    detailText = `后续 ${nextN} · 首工作「${dTitle}」`
  }
  workV2Text.value = `全部工作 ${rows.length} · ${detailText}`
}

const readListText = ref('')
// 待阅/已阅列表 v2（rev180，surface 域 4 条真实 distinct）：read/v2/list/paging（PP_C_READ 分页）+ read/v2/list/next/{id}/{count}
// + readcompleted/v2/list/paging（PP_C_READCOMPLETED 分页）+ readcompleted/v2/list/next/{id}/{count}。next 与 prev 为孪生只取 next。
async function loadReadLists(): Promise<void> {
  readListText.value = ''
  const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const [readList, readcList] = await Promise.all([
    settle(api.get('/api/processplatform/assemble/surface/read/v2/list/paging/1/20/20')),
    settle(api.get('/api/processplatform/assemble/surface/readcompleted/v2/list/paging/1/20/20')),
  ])
  const readRows = asRows(readList)
  const readcRows = asRows(readcList)
  const rId = readRows[0] ? String(readRows[0].id ?? '') : ''
  const rcId = readcRows[0] ? String(readcRows[0].id ?? '') : ''
  const [readNext, readcNext] = await Promise.all([
    rId ? settle(api.get(`/api/processplatform/assemble/surface/read/v2/list/next/${rId}/20`)) : Promise.resolve(null),
    rcId ? settle(api.get(`/api/processplatform/assemble/surface/readcompleted/v2/list/next/${rcId}/20`)) : Promise.resolve(null),
  ])
  readListText.value = `待阅 ${readRows.length}(后续 ${asRows(readNext).length}) · 已阅 ${readcRows.length}(后续 ${asRows(readcNext).length})`
}

function asRows(response: unknown): Record<string, unknown>[] {
  const payload = (response as { data?: unknown })?.data
  const rows = Array.isArray(payload)
    ? payload
    : ((payload as { data?: unknown })?.data ?? [])
  return Array.isArray(rows) ? (rows as Record<string, unknown>[]) : []
}

function workId(item: TaskItem): string {
  return String(item.work || item.workId || item.id)
}

async function openWork(item: TaskItem): Promise<void> {
  opened.value = item
  detailLoading.value = true
  detailError.value = ''
  formErrors.value = {}
  opinion.value = ''
  handleTaskId.value = ''
  attachments.value = []
  records.value = []
  worklogs.value = []
  reads.value = []
  attachDetailText.value = ''
  readDetailText.value = ''
  try {
    const id = workId(item)
    const [formResponse, dataResponse] = await Promise.all([
      api.get(`/api/processplatform/assemble/surface/form/v2/lookup/workorworkcompleted/${id}`),
      api.get(`/api/processplatform/assemble/surface/data/work/${id}`),
    ])
    formDefinition.value = parseFormDefinition((formResponse as any)?.data)
    const payload = (dataResponse as any)?.data
    const values = Array.isArray(payload)
      ? (payload[0] ?? {})
      : ((payload?.data as Record<string, FormValue>) ?? payload ?? {})
    formValues.value = initialFormValues(formDefinition.value, values)
    void loadDetailPanels(id)
    void loadEngineRecords(id)
    void loadSurfaceExtras(id)
    // “我发起的”详情：若本人有该工作的活动任务，允许在此办理（发起人 begin 环节）
    if (activeTab.value === 'started') {
      const pending: any = await api.get(endpoints.pending)
      const tasks = (pending?.data?.data ?? pending?.data ?? []) as TaskItem[]
      const mine = tasks.find((task) => workId(task) === id)
      handleTaskId.value = mine?.id ?? ''
    }
    void loadTaskInfo(activeTab.value === 'pending' ? String(item.id ?? '') : handleTaskId.value)
    void loadTaskV2(activeTab.value === 'pending' ? String(item.id ?? '') : handleTaskId.value)
  } catch (error: any) {
    formDefinition.value = null
    detailError.value = error?.message || '加载表单失败'
  } finally {
    detailLoading.value = false
  }
}

// 详情侧栏：附件 / 流转记录 / 工作日志 / 待阅 —— 均按 workOrWorkCompleted 维度拉取。
// 单个子列表失败不阻断其它面板（各自静默降级为空数组）。
async function loadDetailPanels(id: string): Promise<void> {
  const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const [attRes, recRes, logRes, readRes] = await Promise.all([
    settle(api.get(`/api/processplatform/assemble/surface/attachment/list/work/${id}`)),
    settle(api.get(`/api/processplatform/assemble/surface/record/list/workorworkcompleted/${id}`)),
    settle(api.get(`/api/processplatform/assemble/surface/worklog/list/workorworkcompleted/${id}`)),
    settle(api.get(`/api/processplatform/assemble/surface/read/list/workorworkcompleted/${id}`)),
  ])
  attachments.value = asRows(attRes).map((r) => ({
    id: String(r.id ?? ''),
    name: r.name as string | undefined,
    extension: r.extension as string | undefined,
    length: typeof r.length === 'number' ? r.length : undefined,
  }))
  records.value = asRows(recRes).map((r) => ({
    id: String(r.id ?? ''),
    title: r.title as string | undefined,
    createTime: r.createTime as string | undefined,
  }))
  worklogs.value = asRows(logRes).map((r) => ({
    id: String(r.id ?? ''),
    title: r.title as string | undefined,
    activityName: r.activityName as string | undefined,
    person: r.person as string | undefined,
    createTime: r.createTime as string | undefined,
  }))
  reads.value = asRows(readRes).map((r) => ({
    id: String(r.id ?? ''),
    person: r.person as string | undefined,
    createTime: r.createTime as string | undefined,
  }))
}

// 引擎层明细（processplatform/service/processing 域，rev148）：以工作 id 消费 3 条真实 distinct 路由——
// work/{id}（work_get x_work 详情）+ record/processing/{work}（record_work_processing x_record processing 记录，POST）
// + record/terminate/{work}（record_work_terminate x_record terminate 记录，GET）。子请求各自静默降级。
async function loadEngineRecords(id: string): Promise<void> {
  engineText.value = ''
  const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const [work, processing, terminate] = await Promise.all([
    settle(api.get(`/api/processplatform/service/processing/work/${id}`)),
    settle(api.post(`/api/processplatform/service/processing/record/processing/${id}`)),
    settle(api.get(`/api/processplatform/service/processing/record/terminate/${id}`)),
  ])
  const wTitle = (work as any)?.data?.title ?? id
  const pN = Array.isArray((processing as any)?.data) ? (processing as any).data.length : 0
  const tN = Array.isArray((terminate as any)?.data) ? (terminate as any).data.length : 0
  engineText.value = `引擎工作「${wTitle}」· 处理记录 ${pN} · 终止记录 ${tN}`
}

// 表面附件/文档版本扩展（rev174，surface 域 4 条真实 distinct）：以工作 id 按 workOrWorkCompleted 维度拉
// attachment/list/workorworkcompleted/{flag}（xwork=$1 OR xworkCompleted=$1）+ documentversion/list/workorworkcompleted/{flag}
// （PP_C_DOCUMENTVERSION）；再取首个附件 id 查 attachment/{id}/available（pp_c_attachment xstorage/xlength 可用性）
// + attachment/{id}/online/info（在线编辑信息）。子请求各自静默降级。
async function loadSurfaceExtras(id: string): Promise<void> {
  surfaceExtraText.value = ''
  const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const [attList, docVer] = await Promise.all([
    settle(api.get(`/api/processplatform/assemble/surface/attachment/list/workorworkcompleted/${id}`)),
    settle(api.get(`/api/processplatform/assemble/surface/documentversion/list/workorworkcompleted/${id}`)),
  ])
  const attRows = asRows(attList)
  const docN = asRows(docVer).length
  const attId = attRows[0] ? String(attRows[0].id ?? '') : ''
  let availText = '—'
  if (attId) {
    const [avail, online] = await Promise.all([
      settle(api.get(`/api/processplatform/assemble/surface/attachment/${attId}/available`)),
      settle(api.get(`/api/processplatform/assemble/surface/attachment/${attId}/online/info`)),
    ])
    const ok = (avail as any)?.data?.available === true ? '可用' : '不可用'
    const editable = (online as any)?.data?.onlineEditable === true ? '可在线编辑' : '不可在线编辑'
    availText = `${ok}·${editable}`
  }
  surfaceExtraText.value = `附件(含已完成) ${attRows.length} · 文档版本 ${docN} · 首附件 ${availText}`
}

function closeWork(): void {
  opened.value = null
  formDefinition.value = null
  formValues.value = {}
  formErrors.value = {}
  attachments.value = []
  records.value = []
  worklogs.value = []
  reads.value = []
  engineText.value = ''
  surfaceExtraText.value = ''
}

const canHandle = computed(() => {
  if (!opened.value) return false
  if (activeTab.value === 'pending') return true
  return Boolean(handleTaskId.value)
})

// ── 任务信息 + 催办（processplatform/service/processing/task 族，rev105）──
// 有效任务 id：待办页取列表项 id（即 task id），其它页取本人活动任务 handleTaskId。
const effectiveTaskId = computed(() =>
  activeTab.value === 'pending' ? String(opened.value?.id ?? '') : handleTaskId.value,
)
const taskInfo = ref<Record<string, unknown> | null>(null)
const taskExpireText = ref('')
const pressing = ref(false)
async function loadTaskInfo(taskId: string): Promise<void> {
  taskInfo.value = null
  taskExpireText.value = ''
  if (!taskId) return
  const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const [info, expire] = await Promise.all([
    // GET service/processing/task/{id} —— 任务详情（x_task）
    settle(api.get(`/api/processplatform/service/processing/task/${taskId}`)),
    // GET service/processing/task/expire/{id} —— 超时信息
    settle(api.get(`/api/processplatform/service/processing/task/expire/${taskId}`)),
  ])
  taskInfo.value = ((info as { data?: Record<string, unknown> } | null)?.data ?? null)
  const ed = (expire as { data?: unknown } | null)?.data
  taskExpireText.value =
    ed && typeof ed === 'object' ? JSON.stringify(ed).slice(0, 80) : ed != null ? String(ed) : ''
}
async function pressTask(): Promise<void> {
  const taskId = effectiveTaskId.value
  if (!taskId || pressing.value) return
  pressing.value = true
  try {
    // POST service/processing/task/press/{id} —— 催办
    await api.post(`/api/processplatform/service/processing/task/press/${taskId}`, {})
    toast.success('已催办')
  } catch (e: any) {
    toast.error('催办失败: ' + (e?.message ?? ''))
  } finally {
    pressing.value = false
  }
}

// ── 附件详情 / 待阅详情（rev110，均按 id 拉单条 distinct handler）────
const attachDetailText = ref('')
const readDetailText = ref('')
async function viewAttachment(id: string): Promise<void> {
  if (!id) return
  try {
    // GET service/processing/attachment/{id} —— 附件详情（x_attachment）
    const r: any = await api.get(`/api/processplatform/service/processing/attachment/${id}`)
    const d = r?.data ?? {}
    attachDetailText.value = `附件：${d.name ?? id} · 创建人 ${d.creator ?? '—'}`
  } catch (e: any) {
    toast.error('加载附件详情失败: ' + (e?.message ?? ''))
  }
}
async function viewRead(id: string): Promise<void> {
  if (!id) return
  try {
    // GET assemble/surface/read/{id} —— 待阅详情（PP_C_READ）
    const r: any = await api.get(`/api/processplatform/assemble/surface/read/${id}`)
    const d = r?.data ?? {}
    readDetailText.value = `待阅：${d.xtitle ?? d.title ?? id} · ${d.xperson ?? d.person ?? '—'}`
  } catch (e: any) {
    toast.error('加载待阅详情失败: ' + (e?.message ?? ''))
  }
}

// ── 任务 v2 生命周期（暂停/恢复/重置，rev107）──────────────────
const taskV2Status = ref('')
const v2Busy = ref(false)
async function loadTaskV2(taskId: string): Promise<void> {
  taskV2Status.value = ''
  if (!taskId) return
  try {
    // GET service/processing/task/v2/{id} —— v2 任务详情（含 task_status）
    const r: any = await api.get(`/api/processplatform/service/processing/task/v2/${taskId}`)
    taskV2Status.value = String((r?.data as { task_status?: string; taskStatus?: string })?.task_status ?? (r?.data as any)?.taskStatus ?? '')
  } catch {
    taskV2Status.value = ''
  }
}
async function taskV2Action(kind: 'pause' | 'resume' | 'reset'): Promise<void> {
  const taskId = effectiveTaskId.value
  if (!taskId || v2Busy.value) return
  v2Busy.value = true
  try {
    if (kind === 'pause') {
      // GET task/v2/pause/{id} —— 暂停
      await api.get(`/api/processplatform/service/processing/task/v2/pause/${taskId}`)
    } else if (kind === 'resume') {
      // POST task/v2/resume/{id} —— 恢复
      await api.post(`/api/processplatform/service/processing/task/v2/resume/${taskId}`, {})
    } else {
      // POST task/v2/reset/{id} —— 重置
      await api.post(`/api/processplatform/service/processing/task/v2/reset/${taskId}`, {})
    }
    toast.success('操作成功')
    await loadTaskV2(taskId)
  } catch (e: any) {
    toast.error('操作失败: ' + (e?.message ?? ''))
  } finally {
    v2Busy.value = false
  }
}

// ── 发起流程（从零创建工作实例）────────────────────────────────
const showStart = ref(false)
const startLoading = ref(false)
const startProcesses = ref<Array<{ id: string; name: string }>>([])
const startProcessId = ref('')
const startTitle = ref('')
const startDefinition = ref<XformDefinition | null>(null)
const startValues = ref<Record<string, FormValue>>({})
const startErrors = ref<Record<string, FormValue>>({})
const startSubmitting = ref(false)

async function openStart(): Promise<void> {
  showStart.value = true
  startLoading.value = true
  startProcessId.value = ''
  startTitle.value = ''
  startDefinition.value = null
  startValues.value = {}
  startErrors.value = {}
  try {
    const r: any = await api.get('/api/processplatform/assemble/designer/list/all')
    // 该端点返回分页包裹 {count, data:[rows], page, size}（兼容直接数组形态）
    const payload = r?.data ?? {}
    const rows: unknown[] = Array.isArray(payload) ? payload : (payload.data ?? [])
    startProcesses.value = (rows as Array<Record<string, unknown>>)
      .map((row) => ({ id: String(row.id ?? ''), name: String(row.name ?? row.id ?? '') }))
      .filter((proc) => proc.id)
  } catch (error: any) {
    toast.error(`加载流程失败: ${error?.message || ''}`)
  } finally {
    startLoading.value = false
  }
}

function closeStart(): void {
  showStart.value = false
  startProcessId.value = ''
  startTitle.value = ''
  startDefinition.value = null
  startValues.value = {}
  startErrors.value = {}
}

async function onStartProcessChange(): Promise<void> {
  startDefinition.value = null
  startValues.value = {}
  startErrors.value = {}
  if (!startProcessId.value) return
  try {
    const detail: any = await api.get(
      `/api/processplatform/assemble/designer/get/${encodeURIComponent(startProcessId.value)}`,
    )
    const definition = detail?.data?.processDefinition as Record<string, unknown> | undefined
    const formFlag = (() => {
      const begin = definition?.begin as Record<string, unknown> | undefined
      const manualList = definition?.manualList as Array<Record<string, unknown>> | undefined
      const fromBegin = typeof begin?.form === 'string' ? begin.form : ''
      const fromManual = typeof manualList?.[0]?.form === 'string' ? manualList[0].form : ''
      return fromBegin || fromManual
    })()
    if (!formFlag) return
    const form: any = await api.get(`/api/form/${encodeURIComponent(formFlag)}`)
    startDefinition.value = parseFormDefinition(form?.data)
    startValues.value = initialFormValues(startDefinition.value, {})
  } catch (error: any) {
    toast.error(`加载流程表单失败: ${error?.message || ''}`)
  }
}

async function submitStart(): Promise<void> {
  if (!startProcessId.value || !startTitle.value.trim()) return
  if (startDefinition.value) {
    startErrors.value = validateFormValues(startDefinition.value, startValues.value)
    if (Object.keys(startErrors.value).length) {
      toast.error('请先修正表单校验错误')
      return
    }
  }
  startSubmitting.value = true
  try {
    const created: any = await api.post('/api/processplatform/service/processing/work', {
      process: startProcessId.value,
      title: startTitle.value.trim(),
    })
    const workId = String(created?.data?.id ?? '')
    if (!workId) throw new Error('后端未返回工作 ID')
    if (startDefinition.value && Object.keys(startValues.value).length) {
      await api.put(`/api/processplatform/service/processing/data/work/${workId}`, startValues.value)
    }
    toast.success('流程已发起')
    closeStart()
    activeTab.value = 'started'
    await queryClient.invalidateQueries({ queryKey: ['process-work'] })
  } catch (error: any) {
    toast.error(`发起失败: ${error?.message || ''}`)
  } finally {
    startSubmitting.value = false
  }
}

async function submit(action: 'approve' | 'reject'): Promise<void> {
  if (!opened.value || !formDefinition.value) return
  formErrors.value = validateFormValues(formDefinition.value, formValues.value)
  if (Object.keys(formErrors.value).length) {
    toast.error('请先修正表单校验错误')
    return
  }
  submitting.value = true
  const id = workId(opened.value)
  // 表单数据经 data/work/{id} 落库；complete/reject 仅需 opinion（后端只读 opinion）。
  const opinionPayload = { opinion: opinion.value }
  try {
    await api.put(`/api/processplatform/service/processing/data/work/${id}`, formValues.value)
    const taskId = activeTab.value === 'started' ? handleTaskId.value : opened.value.id
    if (action === 'approve') {
      await api.post(`/api/task/${taskId}/complete`, opinionPayload)
    } else {
      await api.post(`/api/task/${taskId}/reject`, opinionPayload)
    }
    toast.success(action === 'approve' ? '审批通过' : '已驳回')
    closeWork()
    await queryClient.invalidateQueries({ queryKey: ['process-work'] })
  } catch (error: any) {
    toast.error(`提交失败: ${error?.message || ''}`)
  } finally {
    submitting.value = false
  }
}

function fmtTime(value: unknown): string {
  if (!value) return ''
  const date = new Date(String(value))
  return Number.isNaN(date.valueOf()) ? String(value) : date.toLocaleString('zh-CN')
}

function fmtSize(bytes: unknown): string {
  const n = typeof bytes === 'number' ? bytes : Number(bytes)
  if (!Number.isFinite(n) || n <= 0) return '0 B'
  const units = ['B', 'KB', 'MB', 'GB']
  const i = Math.min(units.length - 1, Math.floor(Math.log(n) / Math.log(1024)))
  return `${(n / 1024 ** i).toFixed(i ? 1 : 0)} ${units[i]}`
}
</script>

<style scoped>
.work-view { display: flex; flex-direction: column; gap: 16px; height: 100%; }
.view-header, .content-panel { padding: 20px 24px; }
.view-header h1, .work-dialog h2 { margin: 0; color: var(--color-primary); }
.subtitle, .work-dialog p { margin: 4px 0 0; color: var(--text-muted); font-size: 12px; }
.tabs { display: flex; gap: 4px; padding: 6px; }
.tab-btn { flex: 1; padding: 9px; border: 0; border-radius: var(--radius-md); background: transparent; color: var(--text-muted); cursor: pointer; }
.tab-btn.active { background: var(--color-primary-soft); color: var(--color-primary); }
.content-panel { flex: 1; overflow: auto; }
.item-list { display: flex; flex-direction: column; gap: 8px; }
.item-card { display: flex; align-items: center; gap: 12px; padding: 14px; border: 1px solid var(--border-subtle); border-radius: var(--radius-md); background: var(--bg-elevated); }
.item-body { flex: 1; min-width: 0; }
.item-title { color: var(--text-primary); font-weight: 600; }
.item-meta { display: flex; gap: 10px; margin-top: 5px; color: var(--text-muted); font-size: 11px; }
.btn-sm { padding: 6px 12px; border: 1px solid var(--border-subtle); border-radius: var(--radius-sm); background: var(--bg-elevated); color: var(--text-secondary); cursor: pointer; }
.btn-sm.primary { border-color: var(--color-primary); background: var(--color-primary); color: white; }
.btn-sm.reject { border-color: var(--color-error); color: var(--color-error); }
.state { display: flex; justify-content: center; align-items: center; gap: 10px; padding: 50px; color: var(--text-muted); }
.error-state { color: var(--color-error); }
.modal-overlay { position: fixed; inset: 0; z-index: 1000; display: grid; place-items: center; background: rgba(0,0,0,.62); }
.work-dialog { width: min(860px, 92vw); max-height: 88vh; overflow: auto; padding: 22px; }
.work-dialog header, .work-dialog footer { display: flex; justify-content: space-between; align-items: center; gap: 12px; }
.work-dialog header { margin-bottom: 22px; }
.work-dialog footer { justify-content: flex-end; margin-top: 18px; }
.opinion { box-sizing: border-box; width: 100%; min-height: 80px; margin-top: 18px; padding: 10px; border: 1px solid var(--border-subtle); border-radius: var(--radius-sm); background: var(--bg-elevated); color: var(--text-primary); }
.detail-panels { display: grid; grid-template-columns: repeat(2, 1fr); gap: 12px; margin-top: 18px; }
.detail-block { padding: 12px; border: 1px solid var(--border-subtle); border-radius: var(--radius-sm); background: var(--bg-elevated); }
.detail-block h3 { margin: 0 0 8px; font-size: 13px; color: var(--color-primary); }
.detail-list { display: flex; flex-direction: column; gap: 6px; margin: 0; padding: 0; list-style: none; max-height: 160px; overflow: auto; }
.detail-list li { display: flex; justify-content: space-between; gap: 8px; font-size: 12px; color: var(--text-secondary); }
.detail-list .name { min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.detail-list li.clickable { cursor: pointer; }
.detail-list li.clickable:hover .name { color: var(--color-primary); }
.detail-block .muted { margin: 0; color: var(--text-muted); font-size: 12px; }
</style>
