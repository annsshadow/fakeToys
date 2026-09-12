<template>
  <div class="work-view">
    <div class="view-header glass-card">
      <h1>工作流待办</h1>
      <p class="subtitle">真实任务、表单定义与流程数据闭环</p>
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
          <div class="item-body">
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
          :readonly="activeTab !== 'pending'"
        />
        <div v-else class="state">当前工作没有可渲染的表单定义</div>
        <textarea v-if="activeTab === 'pending'" v-model="opinion" class="opinion" placeholder="处理意见" />
        <footer v-if="activeTab === 'pending'">
          <button class="btn-sm reject" :disabled="submitting" @click="submit('reject')">驳回</button>
          <button class="btn-sm primary" :disabled="submitting" @click="submit('approve')">审批通过</button>
        </footer>
      </section>
    </div>
  </div>
</template>

<script setup lang="ts">
import { api } from '@oa4rust/sdk'
import { useQuery, useQueryClient } from '@tanstack/vue-query'
import { computed, ref } from 'vue'
import XformRuntime from '../components/XformRuntime.vue'
import {
  initialFormValues,
  parseFormDefinition,
  validateFormValues,
  type FormValue,
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
    const response: any = await api.get(endpoints[activeTab.value])
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

function workId(item: TaskItem): string {
  return String(item.work || item.workId || item.id)
}

async function openWork(item: TaskItem): Promise<void> {
  opened.value = item
  detailLoading.value = true
  detailError.value = ''
  formErrors.value = {}
  opinion.value = ''
  try {
    const id = workId(item)
    const [formResponse, dataResponse] = await Promise.all([
      api.get(`/jaxrs/processplatform/assemble/surface/form/v2/lookup/workorworkcompleted/${id}`),
      api.get(`/jaxrs/processplatform/assemble/surface/data/work/${id}`),
    ])
    formDefinition.value = parseFormDefinition((formResponse as any)?.data)
    const data = (dataResponse as any)?.data
    const values = Array.isArray(data) ? data[0] ?? {} : data ?? {}
    formValues.value = initialFormValues(formDefinition.value, values)
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
    if (action === 'approve') {
      await api.post(`/jaxrs/task/${opened.value.id}/complete`, payload)
    } else {
      await api.post(`/jaxrs/task/${opened.value.id}/reject`, payload)
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
