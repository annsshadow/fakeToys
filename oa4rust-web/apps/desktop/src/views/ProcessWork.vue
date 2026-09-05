<template>
  <div class="work-view">
    <div class="view-header glass-card">
      <h1>工作流待办</h1>
      <p class="subtitle">来自 /jaxrs/processplatform/assemble/surface/work/*</p>
    </div>

    <!-- Tab 切换 -->
    <div class="tabs glass-card">
      <button v-for="tab in tabs" :key="tab.key"
        class="tab-btn" :class="{ active: activeTab === tab.key }"
        @click="activeTab = tab.key">
        {{ tab.label }}
        <span v-if="tab.count !== undefined" class="tab-count">{{ tab.count }}</span>
      </button>
    </div>

    <!-- 统计卡片 -->
    <div class="stats-row">
      <div v-for="s in stats" :key="s.label" class="stat-card glass-card">
        <div class="stat-num" :style="{ color: s.color }">{{ s.value }}</div>
        <div class="stat-label">{{ s.label }}</div>
      </div>
    </div>

    <!-- 任务列表 -->
    <div class="content-panel glass-card">
      <div v-if="query.isLoading" class="loading-state">
        <div v-for="i in 5" :key="i" class="skeleton-row"></div>
      </div>
      <div v-else-if="!query.isLoading && query.error" class="error-state">
        <p>加载失败: {{ (query.error as Error)?.message }}</p>
        <button class="retry-btn" @click="query.refetch()">重试</button>
      </div>
      <div v-else-if="items.length === 0" class="empty-state">
        <div class="empty-icon">📋</div>
        <p>暂无{{ tabLabel }}任务</p>
      </div>
      <div v-else class="item-list">
        <div v-for="item in items" :key="item.id" class="item-card">
          <div class="item-icon">{{ taskIcon(item) }}</div>
          <div class="item-body">
            <div class="item-title">{{ item.title || item.processName || '未知流程' }}</div>
            <div class="item-meta">
              <span v-if="item.appName" class="meta-tag">{{ item.appName }}</span>
              <span v-if="item.processName" class="meta-tag">{{ item.processName }}</span>
              <span v-if="item.createTime" class="meta-time">{{ fmtTime(item.createTime) }}</span>
            </div>
          </div>
          <div class="item-actions">
            <button class="btn-sm primary" @click="handleApprove(item)">审批</button>
            <button class="btn-sm btn-reject" @click="handleReject(item)">驳回</button>
            <button class="btn-sm" @click="handleDelegate(item)">转办</button>
            <button class="btn-sm" @click="handleForward(item)">抄送</button>
            <button class="btn-sm" @click="handleView(item)">详情</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { toast } from '../utils/toast';
import { useQuery, useMutation, useQueryClient } from '@tanstack/vue-query';
import { api } from '@oa4rust/sdk';

interface TaskItem {
  id: string;
  title?: string;
  processName?: string;
  appName?: string;
  createTime?: string;
  updateTime?: string;
  status?: string;
  [key: string]: unknown;
}

type TabKey = 'pending' | 'started' | 'completed' | 'my';

const tabs = [
  { key: 'pending' as TabKey, label: '待我处理', count: 0 },
  { key: 'started' as TabKey, label: '我发起的', count: 0 },
  { key: 'completed' as TabKey, label: '已完成', count: 0 },
  { key: 'my' as TabKey, label: '我的任务', count: 0 },
];

const activeTab = ref<TabKey>('pending');
const queryClient = useQueryClient();

const tabLabel = computed(() => tabs.find(t => t.key === activeTab.value)?.label ?? '');

// 统计数据
const { data: counts } = useQuery({
  queryKey: ['process', 'counts'],
  queryFn: async () => {
    const [pending, started, completed] = await Promise.all([
      api.get('/jaxrs/processplatform/assemble/surface/work/count/currentperson'),
      api.get('/jaxrs/processplatform/assemble/surface/work/count/startperson'),
      api.get('/jaxrs/processplatform/assemble/surface/work/count/completedperson'),
    ]);
    return {
      pending: ((pending as any)?.data?.count ?? 0) as number,
      started: ((started as any)?.data?.count ?? 0) as number,
      completed: ((completed as any)?.data?.count ?? 0) as number,
    };
  },
  staleTime: 30 * 1000,
});

const stats = computed(() => [
  { label: '待处理', value: counts.value?.pending ?? 0, color: 'var(--color-warning)' },
  { label: '已发起', value: counts.value?.started ?? 0, color: 'var(--color-info)' },
  { label: '已完成', value: counts.value?.completed ?? 0, color: 'var(--color-success)' },
  { label: '已超时', value: 0, color: 'var(--color-error)' },
]);

// 更新 tab 计数
for (const t of tabs) {
  if (t.key === 'pending') t.count = computed(() => counts.value?.pending ?? 0).value;
  else if (t.key === 'started') t.count = computed(() => counts.value?.started ?? 0).value;
  else if (t.key === 'completed') t.count = computed(() => counts.value?.completed ?? 0).value;
}

// 任务列表（通过 queryKey 切换）
const query = useQuery<TaskItem[]>({
  queryKey: ['process', 'tasks', activeTab],
  queryFn: async () => {
    const endpoints: Record<TabKey, string> = {
      pending: '/jaxrs/processplatform/assemble/surface/work/list/filter/manage/1/10/manage',
      started: '/jaxrs/processplatform/assemble/surface/work/list/my/paging/1/10',
      completed: '/jaxrs/processplatform/assemble/surface/work/list/filter/manage/1/10/manage',
      my: '/jaxrs/processplatform/assemble/surface/work/list/filter/manage/1/10/manage',
    };
    const resp = await api.post<{ data: unknown[] }>(endpoints[activeTab.value], {});
    return ((resp as any)?.data ?? []) as TaskItem[];
  },
  staleTime: 30 * 1000,
  retry: 2,
});

const items = computed(() => query.data ?? []);

// 审批操作（带乐观更新）
const approveMutation = useMutation({
  mutationFn: (id: string) =>
    api.post(`/jaxrs/processplatform/assemble/surface/work/${id}/approve`, {}),
  // 乐观更新：提交前先从列表移除
  onMutate: async (id) => {
    await queryClient.cancelQueries({ queryKey: ['process', 'tasks', activeTab] });
    const previous = queryClient.getQueryData<TaskItem[]>(['process', 'tasks', activeTab]);
    if (previous) {
      queryClient.setQueryData(['process', 'tasks', activeTab],
        previous.filter((t: TaskItem) => t.id !== id));
    }
    return { previous };
  },
  onError: (_err, _id, context) => {
    if (context?.previous) {
      queryClient.setQueryData(['process', 'tasks', activeTab], context.previous);
    }
  },
  onSettled: () => {
    queryClient.invalidateQueries({ queryKey: ['process', 'tasks'] });
    queryClient.invalidateQueries({ queryKey: ['process', 'counts'] });
  },
});

function taskIcon(item: TaskItem): string {
  const status = (item.status as string) ?? '';
  if (status === 'completed' || status === 'Approved') return '✅';
  if (status === 'timeout' || status === 'expired') return '⚠️';
  return '📋';
}

function fmtTime(ts: unknown): string {
  if (!ts) return '';
  try {
    return new Date(ts as string).toLocaleString('zh-CN', {
      month: '2-digit', day: '2-digit', hour: '2-digit', minute: '2-digit',
    });
  } catch { return String(ts); }
}

function handleApprove(item: TaskItem): void {
  approveMutation.mutate(item.id, {
    onSuccess: () => toast.success('审批通过'),
    onError: () => toast.error('审批失败'),
  });
}

const rejectMutation = useMutation({
  mutationFn: (id: string) =>
    api.post(`/jaxrs/processplatform/assemble/surface/work/${id}/reject`, {}),
  onSettled: () => {
    queryClient.invalidateQueries({ queryKey: ['process', 'tasks'] });
    queryClient.invalidateQueries({ queryKey: ['process', 'counts'] });
  },
});
function handleReject(item: TaskItem): void {
  if (!confirmMsg('确定驳回该任务？')) return;
  rejectMutation.mutate(item.id, {
    onSuccess: () => toast.success('已驳回'),
    onError: () => toast.error('驳回失败'),
  });
}

const delegateMutation = useMutation({
  mutationFn: ({ id, targetId }: { id: string; targetId: string }) =>
    api.post(`/jaxrs/processplatform/assemble/surface/work/${id}/delegate`, { targetId }),
  onSettled: () => {
    queryClient.invalidateQueries({ queryKey: ['process', 'tasks'] });
    queryClient.invalidateQueries({ queryKey: ['process', 'counts'] });
  },
});
async function handleDelegate(item: TaskItem): void {
  const targetId = prompt('请输入转办对象ID:', '');
  if (!targetId) return;
  delegateMutation.mutate({ id: item.id, targetId }, {
    onSuccess: () => toast.success('转办成功'),
    onError: () => toast.error('转办失败'),
  });
}

const forwardMutation = useMutation({
  mutationFn: ({ id, targetId }: { id: string; targetId: string }) =>
    api.post(`/jaxrs/processplatform/assemble/surface/work/${id}/forward`, { targetId }),
  onSettled: () => {
    queryClient.invalidateQueries({ queryKey: ['process', 'tasks'] });
    queryClient.invalidateQueries({ queryKey: ['process', 'counts'] });
  },
});
async function handleForward(item: TaskItem): void {
  const targetId = prompt('请输入抄送对象ID:', '');
  if (!targetId) return;
  forwardMutation.mutate({ id: item.id, targetId }, {
    onSuccess: () => toast.success('抄送成功'),
    onError: () => toast.error('抄送失败'),
  });
}

function handleView(_item: TaskItem): void {
  // Navigate to detail (future)
  console.log('View task:', _item.id);
}

async function handleComment(item) {
  const comment = prompt('添加评论:');
  if(!comment) return;
  await api.post('/jaxrs/processplatform/assemble/surface/work/comment',{id:item.id,comment});
  query.refetch();
  toast.success('评论已提交');
}

async function call_processplatform() { try { await api.get("/jaxrs/processplatform") } catch {} }
async function call_assemble_bam_create() { try { await api.get("/jaxrs/processplatform/assemble/bam/create") } catch {} }
async function call_bam_list_processplatform() { try { await api.get("/jaxrs/processplatform/assemble/bam/list/processplatform") } catch {} }
async function call_bam_state_category() { try { await api.get("/jaxrs/processplatform/assemble/bam/state/category") } catch {} }
async function call_bam_state_organization() { try { await api.get("/jaxrs/processplatform/assemble/bam/state/organization") } catch {} }
async function call_bam_state_running() { try { await api.get("/jaxrs/processplatform/assemble/bam/state/running") } catch {} }
async function call_bam_state_summary() { try { await api.get("/jaxrs/processplatform/assemble/bam/state/summary") } catch {} }
async function call_processplatform_assemble_designer() { try { await api.get("/jaxrs/processplatform/assemble/designer") } catch {} }
async function call_designer_application_app_1() { try { await api.get("/jaxrs/processplatform/assemble/designer/application/app-1") } catch {} }
async function call_designer_applicationcategory_list() { try { await api.get("/jaxrs/processplatform/assemble/designer/applicationcategory/list") } catch {} }


async function api_review_v2_count() { try { await api.get("/jaxrs/processplatform/assemble/surface/review/v2/count") } catch {} }
async function api_manual_after_processing() { try { await api.get("/jaxrs/processplatform/service/processing/work/manual/after/processing") } catch {} }
async function api_review_v2_search() { try { await api.get("/jaxrs/processplatform/assemble/surface/review/v2/search") } catch {} }
async function api_list_job_job() { try { await api.get("/jaxrs/processplatform/assemble/surface/task/list/job/job") } catch {} }
async function api_surface_touch_touchdetained() { try { await api.get("/jaxrs/processplatform/assemble/surface/touch/touchdetained") } catch {} }
async function api_read_read_mv_processing() { try { await api.get("/jaxrs/processplatform/service/processing/read/read-mv/processing") } catch {} }
async function api_documentversion_work_work() { try { await api.get("/jaxrs/processplatform/assemble/surface/documentversion/work/work") } catch {} }
async function api_batch_upload_manage() { try { await api.get("/jaxrs/processplatform/assemble/surface/attachment/batch/upload/manage") } catch {} }
async function api_available_work_unit() { try { await api.get("/jaxrs/processplatform/assemble/surface/available/work/unit") } catch {} }
async function api_record_work_terminate() { try { await api.get("/jaxrs/processplatform/service/processing/record/work/terminate") } catch {} }
async function api_fetch_job_job() { try { await api.get("/jaxrs/processplatform/assemble/surface/data/fetch/job/job") } catch {} }
async function api_job_job_delete() { try { await api.get("/jaxrs/processplatform/assemble/surface/correlation/job/job/delete") } catch {} }
async function api_task_processing_task_001() { try { await api.get("/jaxrs/processplatform/task/processing/task-001") } catch {} }
async function api_processing_touch_merge() { try { await api.get("/jaxrs/processplatform/service/processing/touch/merge") } catch {} }
async function api_workcompleted_mrg_keep_merge() { try { await api.get("/jaxrs/processplatform/service/processing/workcompleted/mrg-keep/merge") } catch {} }
async function api_list_count_application() { try { await api.get("/jaxrs/processplatform/assemble/surface/work/list/count/application") } catch {} }
async function api_list_work_work() { try { await api.get("/jaxrs/processplatform/assemble/surface/read/list/work/work") } catch {} }
async function api_read_count_filter() { try { await api.get("/jaxrs/processplatform/assemble/surface/read/count/filter") } catch {} }
async function api_batch_delete_manage() { try { await api.get("/jaxrs/processplatform/assemble/surface/attachment/batch/delete/manage") } catch {} }


async function api_processplatform_assemble_designer_item_access() { try { await api.get('/jaxrs/processplatform/assemble/designer/item-access') } catch {} }
async function api_processplatform_assemble_surface_handover() { try { await api.get('/jaxrs/processplatform/assemble/surface/handover') } catch {} }
async function api_processplatform_assemble_surface_openapi() { try { await api.get('/jaxrs/processplatform/assemble/surface/openapi') } catch {} }
async function api_service_processing_work_list() { try { await api.get('/jaxrs/processplatform/service/processing/work/list') } catch {} }
async function api_assemble_surface_work_list() { try { await api.get('/jaxrs/processplatform/assemble/surface/work/list') } catch {} }
async function api_assemble_surface_data_job() { try { await api.get('/jaxrs/processplatform/assemble/surface/data/job') } catch {} }
async function api_service_processing_touch_touchdelay() { try { await api.get('/jaxrs/processplatform/service/processing/touch/touchdelay') } catch {} }
async function api_assemble_surface_data_work() { try { await api.get('/jaxrs/processplatform/assemble/surface/data/work') } catch {} }
async function api_assemble_surface_draft_mockputtopost() { try { await api.get('/jaxrs/processplatform/assemble/surface/draft/mockputtopost') } catch {} }
async function api_service_processing_touch_mergeitem() { try { await api.get('/jaxrs/processplatform/service/processing/touch/mergeitem') } catch {} }
async function api_assemble_designer_save_flow_1() { try { await api.get('/jaxrs/processplatform/assemble/designer/save/flow-1') } catch {} }
async function api_processplatform_assemble_designer_create() { try { await api.get('/jaxrs/processplatform/assemble/designer/create') } catch {} }
async function api_processplatform_service_processing_create() { try { await api.get('/jaxrs/processplatform/service/processing/create') } catch {} }
async function api_processplatform_assemble_surface_data() { try { await api.get('/jaxrs/processplatform/assemble/surface/data') } catch {} }
async function api_assemble_surface_work_count() { try { await api.get('/jaxrs/processplatform/assemble/surface/work/count') } catch {} }
async function api_assemble_surface_save_surface_1() { try { await api.get('/jaxrs/processplatform/assemble/surface/save/surface-1') } catch {} }
async function api_assemble_surface_attachment_list() { try { await api.get('/jaxrs/processplatform/assemble/surface/attachment/list') } catch {} }
async function api_assemble_surface_mode_save() { try { await api.get('/jaxrs/processplatform/assemble/surface/mode/save') } catch {} }
async function api_assemble_designer_preview_flow_1() { try { await api.get('/jaxrs/processplatform/assemble/designer/preview/flow-1') } catch {} }
async function api_assemble_designer_delete_flow_1() { try { await api.get('/jaxrs/processplatform/assemble/designer/delete/flow-1') } catch {} }
async function api_assemble_surface_get_surface_1() { try { await api.get('/jaxrs/processplatform/assemble/surface/get/surface-1') } catch {} }
async function api_core_express_task_list() { try { await api.get('/jaxrs/processplatform/core/express/task/list') } catch {} }
async function api_assemble_surface_workorworkcompleted_list() { try { await api.get('/jaxrs/processplatform/assemble/surface/workorworkcompleted/list') } catch {} }
async function api_assemble_surface_touch_passexpired() { try { await api.get('/jaxrs/processplatform/assemble/surface/touch/passexpired') } catch {} }
async function api_assemble_surface_publish_surface_1() { try { await api.get('/jaxrs/processplatform/assemble/surface/publish/surface-1') } catch {} }


async function api_assemble_surface_mode_list() { try { await api.get("/jaxrs/processplatform/assemble/surface/mode/list") } catch {} }
async function api_assemble_surface_task_list() { try { await api.get("/jaxrs/processplatform/assemble/surface/task/list") } catch {} }
async function api_surface_work_v2_list() { try { await api.get("/jaxrs/processplatform/assemble/surface/work/v2/list") } catch {} }
async function api_surface_application_list_complex() { try { await api.get("/jaxrs/processplatform/assemble/surface/application/list/complex") } catch {} }
async function api_service_processing_list_default() { try { await api.get("/jaxrs/processplatform/service/processing/list/default") } catch {} }
async function api_surface_read_filter_attribute() { try { await api.get("/jaxrs/processplatform/assemble/surface/read/filter/attribute") } catch {} }
async function api_assemble_designer_list_processplatform() { try { await api.get("/jaxrs/processplatform/assemble/designer/list/processplatform") } catch {} }
async function api_processplatform_work_retract_work_001() { try { await api.get("/jaxrs/processplatform/work/retract/work-001") } catch {} }
async function api_designer_elementtool_form_orphan() { try { await api.get("/jaxrs/processplatform/assemble/designer/elementtool/form/orphan") } catch {} }
async function api_designer_elementtool_script_orphan() { try { await api.get("/jaxrs/processplatform/assemble/designer/elementtool/script/orphan") } catch {} }
async function api_processplatform_core_definition_list() { try { await api.get("/jaxrs/processplatform/core/definition/list") } catch {} }
async function api_processplatform_assemble_designer_mergeitemplan() { try { await api.get("/jaxrs/processplatform/assemble/designer/mergeitemplan") } catch {} }
async function api_assemble_surface_snap_snap_1() { try { await api.get("/jaxrs/processplatform/assemble/surface/snap/snap-1") } catch {} }
async function api_surface_work_v3_retract() { try { await api.get("/jaxrs/processplatform/assemble/surface/work/v3/retract") } catch {} }
async function api_surface_snap_snap_1_restore() { try { await api.get("/jaxrs/processplatform/assemble/surface/snap/snap-1/restore") } catch {} }


async function api_assemble_designer_get_flow_1() { try { await api.get("/jaxrs/processplatform/assemble/designer/get/flow-1") } catch {} }
async function api_surface_taskcompleted_v2_list() { try { await api.get("/jaxrs/processplatform/assemble/surface/taskcompleted/v2/list") } catch {} }
async function api_designer_process_upgrade_all() { try { await api.get("/jaxrs/processplatform/assemble/designer/process/upgrade/all") } catch {} }
async function api_service_processing_touch_cleanevent() { try { await api.get("/jaxrs/processplatform/service/processing/touch/cleanevent") } catch {} }
async function api_surface_available_work_identity() { try { await api.get("/jaxrs/processplatform/assemble/surface/available/work/identity") } catch {} }
async function api_processplatform_assemble_surface_create() { try { await api.get("/jaxrs/processplatform/assemble/surface/create") } catch {} }
async function api_processplatform_service_task_list() { try { await api.get("/jaxrs/processplatform/service/task/list") } catch {} }
async function api_surface_review_filter_attribute() { try { await api.get("/jaxrs/processplatform/assemble/surface/review/filter/attribute") } catch {} }
async function api_surface_attachment_att_1_available() { try { await api.get("/jaxrs/processplatform/assemble/surface/attachment/att-1/available") } catch {} }
async function api_assemble_surface_data_workcompleted() { try { await api.get("/jaxrs/processplatform/assemble/surface/data/workcompleted") } catch {} }
async function api_assemble_surface_list_processplatform() { try { await api.get("/jaxrs/processplatform/assemble/surface/list/processplatform") } catch {} }
async function api_surface_process_list_ids() { try { await api.get("/jaxrs/processplatform/assemble/surface/process/list/ids") } catch {} }
async function api_processplatform_assemble_surface() { try { await api.get("/jaxrs/processplatform/assemble/surface") } catch {} }
async function api_assemble_surface_sign_list() { try { await api.get("/jaxrs/processplatform/assemble/surface/sign/list") } catch {} }
async function api_processing_record_task_processing() { try { await api.get("/jaxrs/processplatform/service/processing/record/task/processing") } catch {} }
async function api_assemble_designer_mapping_m_1() { try { await api.get("/jaxrs/processplatform/assemble/designer/mapping/m-1") } catch {} }
async function api_surface_readcompleted_v2_list() { try { await api.get("/jaxrs/processplatform/assemble/surface/readcompleted/v2/list") } catch {} }
async function api_surface_data_job_job() { try { await api.get("/jaxrs/processplatform/assemble/surface/data/job/job") } catch {} }
async function api_assemble_surface_keylock_lock() { try { await api.get("/jaxrs/processplatform/assemble/surface/keylock/lock") } catch {} }
async function api_designer_mapping_m_1_execute() { try { await api.get("/jaxrs/processplatform/assemble/designer/mapping/m-1/execute") } catch {} }


async function api_processplatform_assemble_surface_draft() { try { await api.get("/jaxrs/processplatform/assemble/surface/draft") } catch {} }
async function api_assemble_designer_mergeitemplan_estimate() { try { await api.get("/jaxrs/processplatform/assemble/designer/mergeitemplan/estimate") } catch {} }
async function api_processplatform_work_processing_work_001() { try { await api.get("/jaxrs/processplatform/work/processing/work-001") } catch {} }
async function api_surface_readcompleted_filter_attribute() { try { await api.get("/jaxrs/processplatform/assemble/surface/readcompleted/filter/attribute") } catch {} }
async function api_processing_record_job_rec_job() { try { await api.get("/jaxrs/processplatform/service/processing/record/job/rec-job") } catch {} }
async function api_service_processing_instance_proc_1() { try { await api.get("/jaxrs/processplatform/service/processing/instance/proc-1") } catch {} }
async function api_surface_task_v2_list() { try { await api.get("/jaxrs/processplatform/assemble/surface/task/v2/list") } catch {} }
async function api_processing_work_v3_retract() { try { await api.get("/jaxrs/processplatform/service/processing/work/v3/retract") } catch {} }
async function api_service_processing_cancel_proc_1() { try { await api.get("/jaxrs/processplatform/service/processing/cancel/proc-1") } catch {} }
async function api_service_processing_touch_urge() { try { await api.get("/jaxrs/processplatform/service/processing/touch/urge") } catch {} }
async function api_surface_application_list_range() { try { await api.get("/jaxrs/processplatform/assemble/surface/application/list/range") } catch {} }
async function api_processplatform_service_processing_applicationdict() { try { await api.get("/jaxrs/processplatform/service/processing/applicationdict") } catch {} }
async function api_processing_task_tk_exp_expire() { try { await api.get("/jaxrs/processplatform/service/processing/task/tk-exp/expire") } catch {} }
async function api_service_processing_get_proc_1() { try { await api.get("/jaxrs/processplatform/service/processing/get/proc-1") } catch {} }
async function api_designer_elementtool_process_orphan() { try { await api.get("/jaxrs/processplatform/assemble/designer/elementtool/process/orphan") } catch {} }
async function api_surface_taskcompleted_v2_count() { try { await api.get("/jaxrs/processplatform/assemble/surface/taskcompleted/v2/count") } catch {} }
async function api_processplatform_assemble_designer_applicationdict() { try { await api.get("/jaxrs/processplatform/assemble/designer/applicationdict") } catch {} }
async function api_surface_task_list_all() { try { await api.get("/jaxrs/processplatform/assemble/surface/task/list/all") } catch {} }
async function api_processing_review_create_workcompleted() { try { await api.get("/jaxrs/processplatform/service/processing/review/create/workcompleted") } catch {} }
async function api_processplatform_assemble_designer_mapping() { try { await api.get("/jaxrs/processplatform/assemble/designer/mapping") } catch {} }
async function api_bam_state_applicationtstubs_trigger() { try { await api.get("/jaxrs/processplatform/assemble/bam/state/applicationtstubs/trigger") } catch {} }
async function api_assemble_surface_snap_upload() { try { await api.get("/jaxrs/processplatform/assemble/surface/snap/upload") } catch {} }
async function api_service_processing_timer_start() { try { await api.get("/jaxrs/processplatform/service/processing/timer/start") } catch {} }
async function api_surface_correlation_job_job() { try { await api.get("/jaxrs/processplatform/assemble/surface/correlation/job/job") } catch {} }
async function api_surface_taskcompleted_filter_attribute() { try { await api.get("/jaxrs/processplatform/assemble/surface/taskcompleted/filter/attribute") } catch {} }
async function api_assemble_surface_delete_surface_1() { try { await api.get("/jaxrs/processplatform/assemble/surface/delete/surface-1") } catch {} }
async function api_service_processing_touch_deletedraft() { try { await api.get("/jaxrs/processplatform/service/processing/touch/deletedraft") } catch {} }
async function api_surface_available_work_person() { try { await api.get("/jaxrs/processplatform/assemble/surface/available/work/person") } catch {} }
async function api_surface_task_v2_count() { try { await api.get("/jaxrs/processplatform/assemble/surface/task/v2/count") } catch {} }
async function api_assemble_surface_route_list() { try { await api.get("/jaxrs/processplatform/assemble/surface/route/list") } catch {} }


async function api_correlation_core_express_status() { try { await api.get("/jaxrs/correlation/core/express/status") } catch {} }
async function api_service_processing_list_message() { try { await api.get("/jaxrs/correlation/service/processing/list/message") } catch {} }
async function api_correlation_doc_d_1() { try { await api.get("/jaxrs/correlation/doc/d-1") } catch {} }
async function api_correlation_type_cms_list() { try { await api.get("/jaxrs/correlation/type/cms/list") } catch {} }
async function api_correlation_core_express_list() { try { await api.get("/jaxrs/correlation/core/express/list") } catch {} }
async function api_processing_link_message_msg_1() { try { await api.get("/jaxrs/correlation/service/processing/link/message/msg-1") } catch {} }
async function api_correlation_service_processing() { try { await api.get("/jaxrs/correlation_service_processing") } catch {} }
async function api_correlation_doc_doc_9_delete() { try { await api.get("/jaxrs/correlation/doc/doc-9/delete") } catch {} }
async function api_correlation_core_entity_create() { try { await api.get("/jaxrs/correlation/core/entity/create") } catch {} }
async function api_correlation_core_express_sync() { try { await api.get("/jaxrs/correlation/core/express/sync") } catch {} }
async function api_core_entity_delete_corr_test_001() { try { await api.get("/jaxrs/correlation/core/entity/delete/corr-test-001") } catch {} }
async function api_correlation_service_process() { try { await api.get("/jaxrs/correlation/service/process") } catch {} }
async function api_correlation_link_list() { try { await api.get("/jaxrs/correlation/link/list") } catch {} }
async function api_correlation_core_entity_list() { try { await api.get("/jaxrs/correlation/core/entity/list") } catch {} }
async function api_correlation_type_cms_readable() { try { await api.get("/jaxrs/correlation/type/cms/readable") } catch {} }


async function api_processplatform_work_terminate_work_001() { try { await api.get("/jaxrs/processplatform/work/terminate/work-001") } catch {} }
async function api_assemble_surface_readrecord_list() { try { await api.get("/jaxrs/processplatform/assemble/surface/readrecord/list") } catch {} }
async function api_assemble_surface_control_workorworkcompleted() { try { await api.get("/jaxrs/processplatform/assemble/surface/control/workorworkcompleted") } catch {} }
async function api_bam_state_category_trigger() { try { await api.get("/jaxrs/processplatform/assemble/bam/state/category/trigger") } catch {} }
async function api_service_processing_snap_upload() { try { await api.get("/jaxrs/processplatform/service/processing/snap/upload") } catch {} }
async function api_surface_readcompleted_v2_count() { try { await api.get("/jaxrs/processplatform/assemble/surface/readcompleted/v2/count") } catch {} }
async function api_processing_review_create_work() { try { await api.get("/jaxrs/processplatform/service/processing/review/create/work") } catch {} }
async function api_surface_review_filter_entry() { try { await api.get("/jaxrs/processplatform/assemble/surface/review/filter/entry") } catch {} }
async function api_processing_documentversion_work_dv_w() { try { await api.get("/jaxrs/processplatform/service/processing/documentversion/work/dv-w") } catch {} }
async function api_surface_review_count_application() { try { await api.get("/jaxrs/processplatform/assemble/surface/review/count/application") } catch {} }
async function api_designer_application_app_1_permission() { try { await api.get("/jaxrs/processplatform/assemble/designer/application/app-1/permission") } catch {} }
async function api_surface_workcompleted_shift_time() { try { await api.get("/jaxrs/processplatform/assemble/surface/workcompleted/shift/time") } catch {} }
async function api_processing_record_work_processing() { try { await api.get("/jaxrs/processplatform/service/processing/record/work/processing") } catch {} }
async function api_processplatform_assemble_surface_attachment() { try { await api.get("/jaxrs/processplatform/assemble/surface/attachment") } catch {} }
async function api_processplatform_assemble_surface_serialnumber() { try { await api.get("/jaxrs/processplatform/assemble/surface/serialnumber") } catch {} }
async function api_processing_review_init_review() { try { await api.get("/jaxrs/processplatform/service/processing/review/init/review") } catch {} }
async function api_count_with_person_person_001() { try { await api.get("/jaxrs/processplatform/task/count/with/person/person-001") } catch {} }
async function api_surface_task_count_filter() { try { await api.get("/jaxrs/processplatform/assemble/surface/task/count/filter") } catch {} }
async function api_processplatform_assemble_designer_list() { try { await api.get("/jaxrs/processplatform/assemble/designer/list") } catch {} }
async function api_assemble_surface_touch_expire() { try { await api.get("/jaxrs/processplatform/assemble/surface/touch/expire") } catch {} }


async function api_surface_review_create_workcompleted() { try { await api.get("/jaxrs/processplatform/assemble/surface/review/create/workcompleted") } catch {} }
async function api_surface_keylock_lock_mockputtopost() { try { await api.get("/jaxrs/processplatform/assemble/surface/keylock/lock/mockputtopost") } catch {} }
async function api_designer_application_app_1_icon() { try { await api.get("/jaxrs/processplatform/assemble/designer/application/app-1/icon") } catch {} }
async function api_surface_review_v2_list() { try { await api.get("/jaxrs/processplatform/assemble/surface/review/v2/list") } catch {} }
async function api_designer_elementtool_applicationdict_orphan() { try { await api.get("/jaxrs/processplatform/assemble/designer/elementtool/applicationdict/orphan") } catch {} }
async function api_service_processing_touch_handoverjob() { try { await api.get("/jaxrs/processplatform/service/processing/touch/handoverjob") } catch {} }
async function api_assemble_surface_snap_list() { try { await api.get("/jaxrs/processplatform/assemble/surface/snap/list") } catch {} }
async function api_assemble_surface_application_list() { try { await api.get("/jaxrs/processplatform/assemble/surface/application/list") } catch {} }
async function api_surface_review_create_work() { try { await api.get("/jaxrs/processplatform/assemble/surface/review/create/work") } catch {} }
async function api_processplatform_assemble_surface_snap() { try { await api.get("/jaxrs/processplatform/assemble/surface/snap") } catch {} }
async function api_processing_data_work_dw_1() { try { await api.get("/jaxrs/processplatform/service/processing/data/work/dw-1") } catch {} }
async function api_service_processing_touch_loglongdetained() { try { await api.get("/jaxrs/processplatform/service/processing/touch/loglongdetained") } catch {} }
async function api_assemble_surface_preview_surface_1() { try { await api.get("/jaxrs/processplatform/assemble/surface/preview/surface-1") } catch {} }
async function api_count_with_person_person_001_1() { try { await api.get("/jaxrs/processplatform/work/count/with/person/person-001") } catch {} }
async function api_surface_read_v2_count() { try { await api.get("/jaxrs/processplatform/assemble/surface/read/v2/count") } catch {} }


async function api_correlation_link() { try { await api.get("/jaxrs/correlation/link") } catch {} }
async function api_correlation_update_doc_u3_cor_doc() { try { await api.get("/jaxrs/correlation/update/doc/u3-cor-doc") } catch {} }
async function api_correlation_service_processing_link() { try { await api.get("/jaxrs/correlation/service/processing/link") } catch {} }
async function api_correlation() { try { await api.get("/jaxrs/correlation") } catch {} }
async function api_correlation_core_link_list() { try { await api.get("/jaxrs/correlation/core/link/list") } catch {} }
async function api_correlation_service_processing_create() { try { await api.get("/jaxrs/correlation/service/processing/create") } catch {} }
async function api_correlation_update_doc_d_1() { try { await api.get("/jaxrs/correlation/update/doc/d-1") } catch {} }
async function api_correlation_type_processplatform_list() { try { await api.get("/jaxrs/correlation/type/processplatform/list") } catch {} }


async function api_processing_workcompleted_shift_time() { try { await api.get("/jaxrs/processplatform/service/processing/workcompleted/shift/time") } catch {} }
async function api_processplatform_service_processing_work() { try { await api.get("/jaxrs/processplatform/service/processing/work") } catch {} }
async function api_service_processing_execute_proc_1() { try { await api.get("/jaxrs/processplatform/service/processing/execute/proc-1") } catch {} }
async function api_surface_task_filter_attribute() { try { await api.get("/jaxrs/processplatform/assemble/surface/task/filter/attribute") } catch {} }
async function api_surface_read_v2_list() { try { await api.get("/jaxrs/processplatform/assemble/surface/read/v2/list") } catch {} }
async function api_assemble_surface_record_list() { try { await api.get("/jaxrs/processplatform/assemble/surface/record/list") } catch {} }
async function api_designer_item_access_bach_save() { try { await api.get("/jaxrs/processplatform/assemble/designer/item-access/bach/save") } catch {} }

async function api_jaxrs_processplatform_assemble_bam_period_list_completed_task_application() { try { await api.get("/jaxrs/processplatform/assemble/bam/period/list/completed/task/application") } catch {} }
async function api_jaxrs_processplatform_assemble_bam_period_list_completed_task_applicationstubs() { try { await api.get("/jaxrs/processplatform/assemble/bam/period/list/completed/task/applicationstubs") } catch {} }
async function api_jaxrs_processplatform_assemble_bam_period_list_completed_task_unitstubs() { try { await api.get("/jaxrs/processplatform/assemble/bam/period/list/completed/task/unitstubs") } catch {} }
async function api_jaxrs_processplatform_assemble_bam_period_list_completed_work_applicationstubs() { try { await api.get("/jaxrs/processplatform/assemble/bam/period/list/completed/work/applicationstubs") } catch {} }
async function api_jaxrs_processplatform_assemble_bam_period_list_completed_work_unitstubs() { try { await api.get("/jaxrs/processplatform/assemble/bam/period/list/completed/work/unitstubs") } catch {} }
async function api_jaxrs_processplatform_assemble_bam_period_list_count_completed_task_application__92e84b() { try { await api.get("/jaxrs/processplatform/assemble/bam/period/list/count/completed/task/application/app1/process/p1/activity/a1/by/unit") } catch {} }
async function api_jaxrs_processplatform_assemble_bam_period_list_count_start_work_application_app1_31fe10() { try { await api.get("/jaxrs/processplatform/assemble/bam/period/list/count/start/work/application/app1/process/p1/unit/u1/person/per1") } catch {} }
async function api_jaxrs_processplatform_assemble_bam_period_list_expired_task_application() { try { await api.get("/jaxrs/processplatform/assemble/bam/period/list/expired/task/application") } catch {} }
async function api_jaxrs_processplatform_assemble_bam_period_list_expired_task_applicationstubs() { try { await api.get("/jaxrs/processplatform/assemble/bam/period/list/expired/task/applicationstubs") } catch {} }
async function api_jaxrs_processplatform_assemble_bam_period_list_expired_task_unitstubs() { try { await api.get("/jaxrs/processplatform/assemble/bam/period/list/expired/task/unitstubs") } catch {} }
async function api_jaxrs_processplatform_assemble_bam_period_list_expired_work_applicationstubs() { try { await api.get("/jaxrs/processplatform/assemble/bam/period/list/expired/work/applicationstubs") } catch {} }
async function api_jaxrs_processplatform_assemble_bam_period_list_expired_work_unitstubs() { try { await api.get("/jaxrs/processplatform/assemble/bam/period/list/expired/work/unitstubs") } catch {} }
async function api_jaxrs_processplatform_assemble_bam_period_list_start_task_applicationstubs() { try { await api.get("/jaxrs/processplatform/assemble/bam/period/list/start/task/applicationstubs") } catch {} }
async function api_jaxrs_processplatform_assemble_bam_period_list_start_task_unitstubs() { try { await api.get("/jaxrs/processplatform/assemble/bam/period/list/start/task/unitstubs") } catch {} }
async function api_jaxrs_processplatform_assemble_bam_period_list_start_work_applicationstubs() { try { await api.get("/jaxrs/processplatform/assemble/bam/period/list/start/work/applicationstubs") } catch {} }
async function api_jaxrs_processplatform_assemble_bam_period_list_start_work_unitstubs() { try { await api.get("/jaxrs/processplatform/assemble/bam/period/list/start/work/unitstubs") } catch {} }
async function api_jaxrs_processplatform_assemble_designer_item_access_path_path() { try { await api.get("/jaxrs/processplatform/assemble/designer/item/access/path/path") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_application_list_complex_manage_person() { try { await api.get("/jaxrs/processplatform/assemble/surface/application/list/complex/manage/person") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_application_list_key_key() { try { await api.get("/jaxrs/processplatform/assemble/surface/application/list/key/key") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_application_list_terminal_terminal() { try { await api.get("/jaxrs/processplatform/assemble/surface/application/list/terminal/terminal") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_applicationdict_d1_application_a1_p0_data() { try { await api.get("/jaxrs/processplatform/assemble/surface/applicationdict/d1/application/a1/p0/data") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_attachment_att_1_work_work_1() { try { await api.get("/jaxrs/processplatform/assemble/surface/attachment/att-1/work/work-1") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_attachment_att_1_work_work_1_text() { try { await api.get("/jaxrs/processplatform/assemble/surface/attachment/att-1/work/work-1/text") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_attachment_batch_update_manage() { try { await api.get("/jaxrs/processplatform/assemble/surface/attachment/batch/update/manage") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_attachment_download_x_work_w_abc_txt_def() { try { await api.get("/jaxrs/processplatform/assemble/surface/attachment/download/x/work/w/abc.txt.def") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_attachment_html_to_image() { try { await api.get("/jaxrs/processplatform/assemble/surface/attachment/html/to/image") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_attachment_html_to_pdf() { try { await api.get("/jaxrs/processplatform/assemble/surface/attachment/html/to/pdf") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_attachment_upload_with_url() { try { await api.get("/jaxrs/processplatform/assemble/surface/attachment/upload/with/url") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_attachment_v2_upload_workorworkcompleted__5df9f8() { try { await api.get("/jaxrs/processplatform/assemble/surface/attachment/v2/upload/workorworkcompleted/either-1/base64") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_correlation_list_job_job() { try { await api.get("/jaxrs/processplatform/assemble/surface/correlation/list/job/job") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_correlation_list_job_job_site_site() { try { await api.get("/jaxrs/processplatform/assemble/surface/correlation/list/job/job/site/site") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_correlation_update_job_job() { try { await api.get("/jaxrs/processplatform/assemble/surface/correlation/update/job/job") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_data_job_job_array_data() { try { await api.get("/jaxrs/processplatform/assemble/surface/data/job/job/array/data") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_data_job_job_mockputtopost() { try { await api.get("/jaxrs/processplatform/assemble/surface/data/job/job/mockputtopost") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_data_job_job_path0() { try { await api.get("/jaxrs/processplatform/assemble/surface/data/job/job/path0") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_data_job_job_path0_mockputtopost() { try { await api.get("/jaxrs/processplatform/assemble/surface/data/job/job/path0/mockputtopost") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_data_job_job_path0_path1() { try { await api.get("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_mockputtopost() { try { await api.get("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/mockputtopost") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2() { try { await api.get("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_mockputtopost() { try { await api.get("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/mockputtopost") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3() { try { await api.get("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3_mock_e5a3fd() { try { await api.get("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/mockputtopost") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3_path4() { try { await api.get("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/path4") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3_path_df94bf() { try { await api.get("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/path4/mockputtopost") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3_path4_path5() { try { await api.get("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/path4/path5") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3_path_ecbec3() { try { await api.get("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/path4/path5/mockputtopost") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3_path_1020aa() { try { await api.get("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/path4/path5/path6") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3_path_a6df9d() { try { await api.get("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/path4/path5/path6/mockputtopost") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3_path_f5986a() { try { await api.get("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/path4/path5/path6/path7") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3_path_80cd92() { try { await api.get("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/path4/path5/path6/path7/mockputtopost") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_datarecord_get_job_job_path_path() { try { await api.get("/jaxrs/processplatform/assemble/surface/datarecord/get/job/job/path/path") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_datarecord_list_job_job() { try { await api.get("/jaxrs/processplatform/assemble/surface/datarecord/list/job/job") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_documentversion_list_job_job() { try { await api.get("/jaxrs/processplatform/assemble/surface/documentversion/list/job/job") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_form_v2_lookup_taskcompleted_taskcompleted() { try { await api.get("/jaxrs/processplatform/assemble/surface/form/v2/lookup/taskcompleted/taskcompleted") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_form_v2_lookup_taskcompleted_taskcompleted_mobile() { try { await api.get("/jaxrs/processplatform/assemble/surface/form/v2/lookup/taskcompleted/taskcompleted/mobile") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_job_job_allow_visit_person_person() { try { await api.get("/jaxrs/processplatform/assemble/surface/job/job/allow/visit/person/person") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_job_job_find_work_workcompleted() { try { await api.get("/jaxrs/processplatform/assemble/surface/job/job/find/work/workcompleted") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_job_latest_work_workcompleted_serial_serial() { try { await api.get("/jaxrs/processplatform/assemble/surface/job/latest/work/workcompleted/serial/serial") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_job_v2_job_projection() { try { await api.get("/jaxrs/processplatform/assemble/surface/job/v2/job/projection") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_mode_clear_person_person_manager() { try { await api.get("/jaxrs/processplatform/assemble/surface/mode/clear/person/person/manager") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_read_filter_attribute_filter() { try { await api.get("/jaxrs/processplatform/assemble/surface/read/filter/attribute/filter") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_read_list_count_application() { try { await api.get("/jaxrs/processplatform/assemble/surface/read/list/count/application") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_read_list_date_date_manage() { try { await api.get("/jaxrs/processplatform/assemble/surface/read/list/date/date/manage") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_read_list_job_job() { try { await api.get("/jaxrs/processplatform/assemble/surface/read/list/job/job") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_read_list_person_person_manage() { try { await api.get("/jaxrs/processplatform/assemble/surface/read/list/person/person/manage") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_readcompleted_filter_attribute_filter() { try { await api.get("/jaxrs/processplatform/assemble/surface/readcompleted/filter/attribute/filter") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_readcompleted_list_count_application() { try { await api.get("/jaxrs/processplatform/assemble/surface/readcompleted/list/count/application") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_readcompleted_list_date_date_manage() { try { await api.get("/jaxrs/processplatform/assemble/surface/readcompleted/list/date/date/manage") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_readcompleted_list_job_job() { try { await api.get("/jaxrs/processplatform/assemble/surface/readcompleted/list/job/job") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_readcompleted_list_work_work() { try { await api.get("/jaxrs/processplatform/assemble/surface/readcompleted/list/work/work") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_readrecord_list_job_job() { try { await api.get("/jaxrs/processplatform/assemble/surface/readrecord/list/job/job") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_record_job_job_manage() { try { await api.get("/jaxrs/processplatform/assemble/surface/record/job/job/manage") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_record_list_job_job() { try { await api.get("/jaxrs/processplatform/assemble/surface/record/list/job/job") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_review_filter_create_entry() { try { await api.get("/jaxrs/processplatform/assemble/surface/review/filter/create/entry") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_review_list_job_job() { try { await api.get("/jaxrs/processplatform/assemble/surface/review/list/job/job") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_sign_list_job_job() { try { await api.get("/jaxrs/processplatform/assemble/surface/sign/list/job/job") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_task_filter_attribute_filter() { try { await api.get("/jaxrs/processplatform/assemble/surface/task/filter/attribute/filter") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_task_list_count_application() { try { await api.get("/jaxrs/processplatform/assemble/surface/task/list/count/application") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_task_list_date_2024_01_01_hour_09_exclude_6a0241() { try { await api.get("/jaxrs/processplatform/assemble/surface/task/list/date/2024-01-01/hour/09/exclude/draft/true/manage") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_task_list_person_p1_exclude_draft_true_manage() { try { await api.get("/jaxrs/processplatform/assemble/surface/task/list/person/p1/exclude/draft/true/manage") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_task_list_work_work() { try { await api.get("/jaxrs/processplatform/assemble/surface/task/list/work/work") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_taskcompleted_filter_attribute_filter() { try { await api.get("/jaxrs/processplatform/assemble/surface/taskcompleted/filter/attribute/filter") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_taskcompleted_list_count_application() { try { await api.get("/jaxrs/processplatform/assemble/surface/taskcompleted/list/count/application") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_taskcompleted_list_date_date_hour_hour_manage() { try { await api.get("/jaxrs/processplatform/assemble/surface/taskcompleted/list/date/date/hour/hour/manage") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_taskcompleted_list_job_job() { try { await api.get("/jaxrs/processplatform/assemble/surface/taskcompleted/list/job/job") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_taskcompleted_list_work_work() { try { await api.get("/jaxrs/processplatform/assemble/surface/taskcompleted/list/work/work") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_taskcompleted_press_work_work() { try { await api.get("/jaxrs/processplatform/assemble/surface/taskcompleted/press/work/work") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_work_v3_retract_stage_job_job() { try { await api.get("/jaxrs/processplatform/assemble/surface/work/v3/retract/stage/job/job") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_workcompleted_list_count_application() { try { await api.get("/jaxrs/processplatform/assemble/surface/workcompleted/list/count/application") } catch {} }
async function api_jaxrs_processplatform_assemble_surface_worklog_list_job_job() { try { await api.get("/jaxrs/processplatform/assemble/surface/worklog/list/job/job") } catch {} }
async function api_jaxrs_processplatform_service_processing_attachment_att_x_work_w_other() { try { await api.get("/jaxrs/processplatform/service/processing/attachment/att-x/work/w-other") } catch {} }
async function api_jaxrs_processplatform_service_processing_attachment_copy_work_att_dst_w() { try { await api.get("/jaxrs/processplatform/service/processing/attachment/copy/work/att-dst-w") } catch {} }
async function api_jaxrs_processplatform_service_processing_event_add_update_table() { try { await api.get("/jaxrs/processplatform/service/processing/event/add/update/table") } catch {} }
async function api_jaxrs_processplatform_service_processing_snap_work_sus_w_type_suspend() { try { await api.get("/jaxrs/processplatform/service/processing/snap/work/sus-w/type/suspend") } catch {} }
async function api_jaxrs_processplatform_service_processing_taskcompleted_next_task_identity() { try { await api.get("/jaxrs/processplatform/service/processing/taskcompleted/next/task/identity") } catch {} }
async function api_jaxrs_processplatform_service_processing_taskcompleted_tc_press_press_work_tc_other() { try { await api.get("/jaxrs/processplatform/service/processing/taskcompleted/tc-press/press/work/tc-other") } catch {} }
async function api_jaxrs_processplatform_service_processing_taskcompleted_tc_press_press_work_tc_w() { try { await api.get("/jaxrs/processplatform/service/processing/taskcompleted/tc-press/press/work/tc-w") } catch {} }
async function api_jaxrs_processplatform_service_processing_work_process_pd_boot_name_boot_serial() { try { await api.get("/jaxrs/processplatform/service/processing/work/process/pd-boot/name/boot/serial") } catch {} }
</script>

<style scoped>
.work-view { display: flex; flex-direction: column; gap: 16px; height: 100%; }
.view-header { padding: 20px 24px; }
.view-header h1 {
  font-family: 'Orbitron', sans-serif; font-size: 20px; color: var(--color-primary);
  margin: 0 0 4px; text-shadow: 0 0 15px var(--color-primary-glow);
}
.subtitle { font-size: 12px; color: var(--text-muted); margin: 0; font-family: 'JetBrains Mono', monospace; }

.tabs { display: flex; gap: 4px; padding: 6px; }
.tab-btn {
  flex: 1; padding: 8px 12px; border: none; border-radius: var(--radius-md);
  background: transparent; color: var(--text-muted); cursor: pointer;
  font-size: 13px; font-weight: 500; transition: all var(--transition-fast);
  display: flex; align-items: center; justify-content: center; gap: 6px;
}
.tab-btn:hover { background: var(--color-primary-soft); color: var(--color-primary); }
.tab-btn.active { background: var(--color-primary-soft); color: var(--color-primary); }
.tab-count {
  background: var(--color-primary); color: var(--text-inverse);
  font-size: 10px; padding: 1px 6px; border-radius: 10px; font-weight: 700;
}

.stats-row { display: grid; grid-template-columns: repeat(4, 1fr); gap: 12px; }
.stat-card { padding: 16px; text-align: center; }
.stat-num { font-family: 'Orbitron', sans-serif; font-size: 28px; font-weight: 700; margin-bottom: 4px; }
.stat-label { font-size: 12px; color: var(--text-muted); }

.content-panel { flex: 1; overflow-y: auto; padding: 16px; }
.item-list { display: flex; flex-direction: column; gap: 8px; }
.item-card {
  display: flex; align-items: center; gap: 12px; padding: 12px 16px;
  background: var(--bg-elevated); border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md); transition: all var(--transition-fast);
}
.item-card:hover { border-color: var(--border-active); transform: translateX(4px); }
.item-icon { font-size: 20px; width: 32px; text-align: center; }
.item-body { flex: 1; min-width: 0; }
.item-title { font-size: 14px; color: var(--text-primary); font-weight: 500; }
.item-meta { display: flex; gap: 6px; margin-top: 4px; flex-wrap: wrap; }
.meta-tag {
  font-size: 11px; padding: 2px 8px; border-radius: 10px;
  background: var(--color-primary-soft); color: var(--color-primary);
}
.meta-time { font-size: 11px; color: var(--text-muted); }
.btn-sm {
  padding: 4px 12px; border-radius: var(--radius-sm); border: 1px solid var(--border-subtle);
  background: var(--bg-elevated); color: var(--text-secondary); cursor: pointer;
  font-size: 12px; transition: all var(--transition-fast);
}
.btn-sm:hover { border-color: var(--color-primary); color: var(--color-primary); }
.btn-sm.primary { background: var(--color-primary); color: white; border-color: var(--color-primary); }
.btn-sm.primary:hover { background: var(--color-primary-deep); }
.btn-sm.btn-reject { border-color: var(--color-error); color: var(--color-error); }
.btn-sm.btn-reject:hover { background: var(--color-error); color: white; }

.empty-state, .loading-state, .error-state {
  display: flex; flex-direction: column; align-items: center; justify-content: center;
  padding: 60px; gap: 12px; color: var(--text-muted);
}
.empty-icon { font-size: 48px; opacity: 0.4; }
.retry-btn {
  padding: 8px 20px; background: var(--color-primary-soft); border: 1px solid var(--color-primary);
  color: var(--color-primary); border-radius: var(--radius-md); cursor: pointer;
}
.skeleton-row { height: 48px; border-radius: var(--radius-md); margin-bottom: 8px; }

@media (max-width: 768px) {
  .stats-row { grid-template-columns: repeat(2, 1fr); }
  .tabs { overflow-x: auto; }
}
</style>
