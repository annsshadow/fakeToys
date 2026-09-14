<template>
  <div class="work-view">
    <div class="view-header glass-card">
      <div>
        <h1>工作流待办</h1>
        <p class="subtitle">真实任务、表单定义与流程数据闭环</p>
      </div>
      <button class="btn-sm primary" @click="openStart">发起流程</button>
    </div>
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
  pending: '/jaxrs/processplatform/assemble/surface/task/list/my/paging/1/size/20',
  completed: '/jaxrs/processplatform/assemble/surface/taskcompleted/list/my/paging/1/size/20',
  started: '/jaxrs/processplatform/assemble/surface/work/list/my/paging/1/size/20',
}
const query = useQuery({
  queryKey: ['process-work', activeTab],
  queryFn: async () => {
    // 「我发起的」work 列表在 Java 契约为 POST（task/taskcompleted 为 GET），按 tab 分流
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
  try {
    const id = workId(item)
    const [formResponse, dataResponse] = await Promise.all([
      api.get(`/jaxrs/processplatform/assemble/surface/form/v2/lookup/workorworkcompleted/${id}`),
      api.get(`/jaxrs/processplatform/assemble/surface/data/work/${id}`),
    ])
    formDefinition.value = parseFormDefinition((formResponse as any)?.data)
    const payload = (dataResponse as any)?.data
    const values = Array.isArray(payload)
      ? (payload[0] ?? {})
      : ((payload?.data as Record<string, FormValue>) ?? payload ?? {})
    formValues.value = initialFormValues(formDefinition.value, values)
    // “我发起的”详情：若本人有该工作的活动任务，允许在此办理（发起人 begin 环节）
    if (activeTab.value === 'started') {
      const pending: any = await api.get(endpoints.pending)
      const tasks = (pending?.data?.data ?? pending?.data ?? []) as TaskItem[]
      const mine = tasks.find((task) => workId(task) === id)
      handleTaskId.value = mine?.id ?? ''
    }
  } catch (error: any) {
    formDefinition.value = null
    detailError.value = error?.message || '加载表单失败'
  } finally {
    detailLoading.value = false
  }
}

function closeWork(): void {
  opened.value = null
  formDefinition.value = null
  formValues.value = {}
  formErrors.value = {}
}

const canHandle = computed(() => {
  if (!opened.value) return false
  if (activeTab.value === 'pending') return true
  return Boolean(handleTaskId.value)
})

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
    const r: any = await api.get('/jaxrs/processplatform/assemble/designer/list/all')
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
      `/jaxrs/processplatform/assemble/designer/get/${encodeURIComponent(startProcessId.value)}`,
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
    const form: any = await api.get(`/jaxrs/form/${encodeURIComponent(formFlag)}`)
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
    const created: any = await api.post('/jaxrs/processplatform/service/processing/work', {
      process: startProcessId.value,
      title: startTitle.value.trim(),
    })
    const workId = String(created?.data?.id ?? '')
    if (!workId) throw new Error('后端未返回工作 ID')
    if (startDefinition.value && Object.keys(startValues.value).length) {
      await api.put(`/jaxrs/processplatform/service/processing/data/work/${workId}`, startValues.value)
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
  const payload = { data: formValues.value, opinion: opinion.value, action }
  try {
    await api.put(`/jaxrs/processplatform/service/processing/data/work/${id}`, formValues.value)
    const taskId = activeTab.value === 'started' ? handleTaskId.value : opened.value.id
    if (action === 'approve') {
      await api.post(`/jaxrs/task/${taskId}/complete`, payload)
    } else {
      await api.post(`/jaxrs/task/${taskId}/reject`, payload)
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
</style>
