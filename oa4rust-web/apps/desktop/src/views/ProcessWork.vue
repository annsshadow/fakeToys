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

}

async function handleComment(item) {
  const comment = prompt('添加评论:');
  if(!comment) return;
  await api.post('/jaxrs/processplatform/assemble/surface/work/comment',{id:item.id,comment});
  query.refetch();
  toast.success('评论已提交');
}



// 工作管理扩展操作
const workManageM = useMutation({
  mutationFn: (id: string) => api.get(`/jaxrs/processplatform/assemble/surface/work/manage/${id}`),
  onSettled: () => { queryClient.invalidateQueries({ queryKey: ['process', 'tasks'] }); queryClient.invalidateQueries({ queryKey: ['process', 'counts'] }); },
});
function handleManage(item: TaskItem): void { workManageM.mutate(item.id); }

const workAssignmentM = useMutation({
  mutationFn: (id: string) => api.get(`/jaxrs/processplatform/assemble/surface/work/assignment/manage/${id}`),
  onSettled: () => { queryClient.invalidateQueries({ queryKey: ['process', 'tasks'] }); },
});
function handleAssignment(item: TaskItem): void { workAssignmentM.mutate(item.id); }

const workForceM = useMutation({
  mutationFn: (processFlag: string) => api.post(`/jaxrs/processplatform/assemble/surface/work/process/force/${processFlag}`, null),
  onSettled: () => { queryClient.invalidateQueries({ queryKey: ['process', 'tasks'] }); },
});
async function handleForce(item: TaskItem): void {
  const flag = prompt('输入流程Flag:', '');
  if (!flag) return;
  workForceM.mutate(flag);
}

const workRerouteM = useMutation({
  mutationFn: (id: string) => api.post(`/jaxrs/processplatform/assemble/surface/work/v2/reroute/${id}`, null),
  onSettled: () => { queryClient.invalidateQueries({ queryKey: ['process', 'tasks'] }); },
});
function handleReroute(item: TaskItem): void { if(confirmMsg('确认重路由该任务？')) workRerouteM.mutate(item.id); }

const workRetractM = useMutation({
  mutationFn: (id: string) => api.post(`/jaxrs/processplatform/assemble/surface/work/v2/retract/${id}`, null),
  onSettled: () => { queryClient.invalidateQueries({ queryKey: ['process', 'tasks'] }); },
});
function handleRetract(item: TaskItem): void { if(confirmMsg('确认撤回该任务？')) workRetractM.mutate(item.id); }
const review_v2_count_ref = ref<any[]>([]);
const review_v2_count_q = useQuery({
  queryKey: ['review_v2_count'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/review/v2/count"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const review_v2_search_ref = ref<any[]>([]);
const review_v2_search_q = useQuery({
  queryKey: ['review_v2_search'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/review/v2/search"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const task_processing_task_001_ref = ref<any[]>([]);
const task_processing_task_001_q = useQuery({
  queryKey: ['task_processing_task_001'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/task/processing/task-001"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});

const assemble_designer_save_flow_1_ref = ref<any[]>([]);
const assemble_designer_save_flow_1_q = useQuery({
  queryKey: ['assemble_designer_save_flow_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/designer/save/flow-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_surface_save_surface_1_ref = ref<any[]>([]);
const assemble_surface_save_surface_1_q = useQuery({
  queryKey: ['assemble_surface_save_surface_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/save/surface-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_designer_preview_flow_1_ref = ref<any[]>([]);
const assemble_designer_preview_flow_1_q = useQuery({
  queryKey: ['assemble_designer_preview_flow_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/designer/preview/flow-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_designer_delete_flow_1_ref = ref<any[]>([]);
const assemble_designer_delete_flow_1_q = useQuery({
  queryKey: ['assemble_designer_delete_flow_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/designer/delete/flow-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_surface_get_surface_1_ref = ref<any[]>([]);
const assemble_surface_get_surface_1_q = useQuery({
  queryKey: ['assemble_surface_get_surface_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/get/surface-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_surface_publish_surface_1_ref = ref<any[]>([]);
const assemble_surface_publish_surface_1_q = useQuery({
  queryKey: ['assemble_surface_publish_surface_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/publish/surface-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});

const surface_work_v2_list_ref = ref<any[]>([]);
const surface_work_v2_list_q = useQuery({
  queryKey: ['surface_work_v2_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/work/v2/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const processplatform_work_retract_work_001_ref = ref<any[]>([]);
const processplatform_work_retract_work_001_q = useQuery({
  queryKey: ['processplatform_work_retract_work_001'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/work/retract/work-001"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_surface_snap_snap_1_ref = ref<any[]>([]);
const assemble_surface_snap_snap_1_q = useQuery({
  queryKey: ['assemble_surface_snap_snap_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/snap/snap-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const surface_work_v3_retract_ref = ref<any[]>([]);
const surface_work_v3_retract_q = useQuery({
  queryKey: ['surface_work_v3_retract'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/work/v3/retract"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const surface_snap_snap_1_restore_ref = ref<any[]>([]);
const surface_snap_snap_1_restore_q = useQuery({
  queryKey: ['surface_snap_snap_1_restore'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/snap/snap-1/restore"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});

const assemble_designer_get_flow_1_ref = ref<any[]>([]);
const assemble_designer_get_flow_1_q = useQuery({
  queryKey: ['assemble_designer_get_flow_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/designer/get/flow-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const surface_taskcompleted_v2_list_ref = ref<any[]>([]);
const surface_taskcompleted_v2_list_q = useQuery({
  queryKey: ['surface_taskcompleted_v2_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/taskcompleted/v2/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const surface_attachment_att_1_available_ref = ref<any[]>([]);
const surface_attachment_att_1_available_q = useQuery({
  queryKey: ['surface_attachment_att_1_available'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/attachment/att-1/available"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_designer_mapping_m_1_ref = ref<any[]>([]);
const assemble_designer_mapping_m_1_q = useQuery({
  queryKey: ['assemble_designer_mapping_m_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/designer/mapping/m-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const surface_readcompleted_v2_list_ref = ref<any[]>([]);
const surface_readcompleted_v2_list_q = useQuery({
  queryKey: ['surface_readcompleted_v2_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/readcompleted/v2/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const designer_mapping_m_1_execute_ref = ref<any[]>([]);
const designer_mapping_m_1_execute_q = useQuery({
  queryKey: ['designer_mapping_m_1_execute'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/designer/mapping/m-1/execute"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});

const processplatform_work_processing_work_001_ref = ref<any[]>([]);
const processplatform_work_processing_work_001_q = useQuery({
  queryKey: ['processplatform_work_processing_work_001'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/work/processing/work-001"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const service_processing_instance_proc_1_ref = ref<any[]>([]);
const service_processing_instance_proc_1_q = useQuery({
  queryKey: ['service_processing_instance_proc_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/service/processing/instance/proc-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const surface_task_v2_list_ref = ref<any[]>([]);
const surface_task_v2_list_q = useQuery({
  queryKey: ['surface_task_v2_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/task/v2/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const processing_work_v3_retract_ref = ref<any[]>([]);
const processing_work_v3_retract_q = useQuery({
  queryKey: ['processing_work_v3_retract'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/service/processing/work/v3/retract"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const service_processing_cancel_proc_1_ref = ref<any[]>([]);
const service_processing_cancel_proc_1_q = useQuery({
  queryKey: ['service_processing_cancel_proc_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/service/processing/cancel/proc-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const service_processing_get_proc_1_ref = ref<any[]>([]);
const service_processing_get_proc_1_q = useQuery({
  queryKey: ['service_processing_get_proc_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/service/processing/get/proc-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const surface_taskcompleted_v2_count_ref = ref<any[]>([]);
const surface_taskcompleted_v2_count_q = useQuery({
  queryKey: ['surface_taskcompleted_v2_count'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/taskcompleted/v2/count"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_surface_delete_surface_1_ref = ref<any[]>([]);
const assemble_surface_delete_surface_1_q = useQuery({
  queryKey: ['assemble_surface_delete_surface_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/delete/surface-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const surface_task_v2_count_ref = ref<any[]>([]);
const surface_task_v2_count_q = useQuery({
  queryKey: ['surface_task_v2_count'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/task/v2/count"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});

const correlation_doc_d_1_ref = ref<any[]>([]);
const correlation_doc_d_1_q = useQuery({
  queryKey: ['correlation_doc_d_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/correlation/doc/d-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const processing_link_message_msg_1_ref = ref<any[]>([]);
const processing_link_message_msg_1_q = useQuery({
  queryKey: ['processing_link_message_msg_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/correlation/service/processing/link/message/msg-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const correlation_doc_doc_9_delete_ref = ref<any[]>([]);
const correlation_doc_doc_9_delete_q = useQuery({
  queryKey: ['correlation_doc_doc_9_delete'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/correlation/doc/doc-9/delete"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const core_entity_delete_corr_test_001_ref = ref<any[]>([]);
const core_entity_delete_corr_test_001_q = useQuery({
  queryKey: ['core_entity_delete_corr_test_001'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/correlation/core/entity/delete/corr-test-001"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});

const processplatform_work_terminate_work_001_ref = ref<any[]>([]);
const processplatform_work_terminate_work_001_q = useQuery({
  queryKey: ['processplatform_work_terminate_work_001'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/work/terminate/work-001"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const surface_readcompleted_v2_count_ref = ref<any[]>([]);
const surface_readcompleted_v2_count_q = useQuery({
  queryKey: ['surface_readcompleted_v2_count'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/readcompleted/v2/count"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const designer_application_app_1_permission_ref = ref<any[]>([]);
const designer_application_app_1_permission_q = useQuery({
  queryKey: ['designer_application_app_1_permission'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/designer/application/app-1/permission"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const count_with_person_person_001_ref = ref<any[]>([]);
const count_with_person_person_001_q = useQuery({
  queryKey: ['count_with_person_person_001'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/task/count/with/person/person-001"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});

const designer_application_app_1_icon_ref = ref<any[]>([]);
const designer_application_app_1_icon_q = useQuery({
  queryKey: ['designer_application_app_1_icon'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/designer/application/app-1/icon"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const surface_review_v2_list_ref = ref<any[]>([]);
const surface_review_v2_list_q = useQuery({
  queryKey: ['surface_review_v2_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/review/v2/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const processing_data_work_dw_1_ref = ref<any[]>([]);
const processing_data_work_dw_1_q = useQuery({
  queryKey: ['processing_data_work_dw_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/service/processing/data/work/dw-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const assemble_surface_preview_surface_1_ref = ref<any[]>([]);
const assemble_surface_preview_surface_1_q = useQuery({
  queryKey: ['assemble_surface_preview_surface_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/preview/surface-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const count_with_person_person_001_1_ref = ref<any[]>([]);
const count_with_person_person_001_1_q = useQuery({
  queryKey: ['count_with_person_person_001_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/work/count/with/person/person-001"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const surface_read_v2_count_ref = ref<any[]>([]);
const surface_read_v2_count_q = useQuery({
  queryKey: ['surface_read_v2_count'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/read/v2/count"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});

const correlation_update_doc_u3_cor_doc_ref = ref<any[]>([]);
const correlation_update_doc_u3_cor_doc_q = useQuery({
  queryKey: ['correlation_update_doc_u3_cor_doc'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/correlation/update/doc/u3-cor-doc"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const correlation_update_doc_d_1_ref = ref<any[]>([]);
const correlation_update_doc_d_1_q = useQuery({
  queryKey: ['correlation_update_doc_d_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/correlation/update/doc/d-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});

const service_processing_execute_proc_1_ref = ref<any[]>([]);
const service_processing_execute_proc_1_q = useQuery({
  queryKey: ['service_processing_execute_proc_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/service/processing/execute/proc-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const surface_read_v2_list_ref = ref<any[]>([]);
const surface_read_v2_list_q = useQuery({
  queryKey: ['surface_read_v2_list'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/read/v2/list"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});

const jaxrs_processplatform_assemble_bam_period_list_count_completed_task_application__92e84b_ref = ref<any[]>([]);
const jaxrs_processplatform_assemble_bam_period_list_count_completed_task_application__92e84b_q = useQuery({
  queryKey: ['jaxrs_processplatform_assemble_bam_period_list_count_completed_task_application__92e84b'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/bam/period/list/count/completed/task/application/app1/process/p1/activity/a1/by/unit"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_processplatform_assemble_bam_period_list_count_start_work_application_app1_31fe10_ref = ref<any[]>([]);
const jaxrs_processplatform_assemble_bam_period_list_count_start_work_application_app1_31fe10_q = useQuery({
  queryKey: ['jaxrs_processplatform_assemble_bam_period_list_count_start_work_application_app1_31fe10'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/bam/period/list/count/start/work/application/app1/process/p1/unit/u1/person/per1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_processplatform_assemble_surface_applicationdict_d1_application_a1_p0_data_ref = ref<any[]>([]);
const jaxrs_processplatform_assemble_surface_applicationdict_d1_application_a1_p0_data_q = useQuery({
  queryKey: ['jaxrs_processplatform_assemble_surface_applicationdict_d1_application_a1_p0_data'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/applicationdict/d1/application/a1/p0/data"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_processplatform_assemble_surface_attachment_att_1_work_work_1_ref = ref<any[]>([]);
const jaxrs_processplatform_assemble_surface_attachment_att_1_work_work_1_q = useQuery({
  queryKey: ['jaxrs_processplatform_assemble_surface_attachment_att_1_work_work_1'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/attachment/att-1/work/work-1"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_processplatform_assemble_surface_attachment_att_1_work_work_1_text_ref = ref<any[]>([]);
const jaxrs_processplatform_assemble_surface_attachment_att_1_work_work_1_text_q = useQuery({
  queryKey: ['jaxrs_processplatform_assemble_surface_attachment_att_1_work_work_1_text'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/attachment/att-1/work/work-1/text"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_processplatform_assemble_surface_attachment_v2_upload_workorworkcompleted__5df9f8_ref = ref<any[]>([]);
const jaxrs_processplatform_assemble_surface_attachment_v2_upload_workorworkcompleted__5df9f8_q = useQuery({
  queryKey: ['jaxrs_processplatform_assemble_surface_attachment_v2_upload_workorworkcompleted__5df9f8'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/attachment/v2/upload/workorworkcompleted/either-1/base64"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_jaxrs_pr_277_data = ref<any[]>([]);
const { data: api_jaxrs_pr_277_q } = useQuery({queryKey: ['api_jaxrs_pr_277', '/jaxrs/processplatform/assemble/surface/data/job/job/array/data'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/data/job/job/array/data"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_277_q, (v) => { api_jaxrs_pr_277_data.value = v ?? []; });
const api_jaxrs_pr_735_data = ref<any[]>([]);
const { data: api_jaxrs_pr_735_q } = useQuery({queryKey: ['api_jaxrs_pr_735', '/jaxrs/processplatform/assemble/surface/data/job/job/path0'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/data/job/job/path0"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_735_q, (v) => { api_jaxrs_pr_735_data.value = v ?? []; });
const jaxrs_processplatform_assemble_surface_data_job_job_path0_mockputtopost_ref = ref<any[]>([]);
const jaxrs_processplatform_assemble_surface_data_job_job_path0_mockputtopost_q = useQuery({
  queryKey: ['jaxrs_processplatform_assemble_surface_data_job_job_path0_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/data/job/job/path0/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_jaxrs_pr_401_data = ref<any[]>([]);
const { data: api_jaxrs_pr_401_q } = useQuery({queryKey: ['api_jaxrs_pr_401', '/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_401_q, (v) => { api_jaxrs_pr_401_data.value = v ?? []; });
const jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_mockputtopost_ref = ref<any[]>([]);
const jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_mockputtopost_q = useQuery({
  queryKey: ['jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_ref = ref<any[]>([]);
const jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_q = useQuery({
  queryKey: ['jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_mockputtopost_ref = ref<any[]>([]);
const jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_mockputtopost_q = useQuery({
  queryKey: ['jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_mockputtopost'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3_ref = ref<any[]>([]);
const jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3_q = useQuery({
  queryKey: ['jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3_mock_e5a3fd_ref = ref<any[]>([]);
const jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3_mock_e5a3fd_q = useQuery({
  queryKey: ['jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3_mock_e5a3fd'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3_path4_ref = ref<any[]>([]);
const jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3_path4_q = useQuery({
  queryKey: ['jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3_path4'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/path4"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3_path_df94bf_ref = ref<any[]>([]);
const jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3_path_df94bf_q = useQuery({
  queryKey: ['jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3_path_df94bf'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/path4/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3_path4_path5_ref = ref<any[]>([]);
const jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3_path4_path5_q = useQuery({
  queryKey: ['jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3_path4_path5'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/path4/path5"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3_path_ecbec3_ref = ref<any[]>([]);
const jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3_path_ecbec3_q = useQuery({
  queryKey: ['jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3_path_ecbec3'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/path4/path5/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3_path_1020aa_ref = ref<any[]>([]);
const jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3_path_1020aa_q = useQuery({
  queryKey: ['jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3_path_1020aa'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/path4/path5/path6"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3_path_a6df9d_ref = ref<any[]>([]);
const jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3_path_a6df9d_q = useQuery({
  queryKey: ['jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3_path_a6df9d'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/path4/path5/path6/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3_path_f5986a_ref = ref<any[]>([]);
const jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3_path_f5986a_q = useQuery({
  queryKey: ['jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3_path_f5986a'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/path4/path5/path6/path7"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3_path_80cd92_ref = ref<any[]>([]);
const jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3_path_80cd92_q = useQuery({
  queryKey: ['jaxrs_processplatform_assemble_surface_data_job_job_path0_path1_path2_path3_path_80cd92'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/path4/path5/path6/path7/mockputtopost"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_jaxrs_pr_319_data = ref<any[]>([]);
const { data: api_jaxrs_pr_319_q } = useQuery({queryKey: ['api_jaxrs_pr_319', '/jaxrs/processplatform/assemble/surface/datarecord/get/job/job/path/path'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/datarecord/get/job/job/path/path"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_319_q, (v) => { api_jaxrs_pr_319_data.value = v ?? []; });
const api_jaxrs_pr_674_data = ref<any[]>([]);
const { data: api_jaxrs_pr_674_q } = useQuery({queryKey: ['api_jaxrs_pr_674', '/jaxrs/processplatform/assemble/surface/datarecord/list/job/job'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/datarecord/list/job/job"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_674_q, (v) => { api_jaxrs_pr_674_data.value = v ?? []; });
const api_jaxrs_pr_950_data = ref<any[]>([]);
const { data: api_jaxrs_pr_950_q } = useQuery({queryKey: ['api_jaxrs_pr_950', '/jaxrs/processplatform/assemble/surface/documentversion/list/job/job'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/documentversion/list/job/job"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_950_q, (v) => { api_jaxrs_pr_950_data.value = v ?? []; });
const api_jaxrs_pr_561_data = ref<any[]>([]);
const { data: api_jaxrs_pr_561_q } = useQuery({queryKey: ['api_jaxrs_pr_561', '/jaxrs/processplatform/assemble/surface/form/v2/lookup/taskcompleted/taskcompleted'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/form/v2/lookup/taskcompleted/taskcompleted"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_561_q, (v) => { api_jaxrs_pr_561_data.value = v ?? []; });
const api_jaxrs_pr_701_data = ref<any[]>([]);
const { data: api_jaxrs_pr_701_q } = useQuery({queryKey: ['api_jaxrs_pr_701', '/jaxrs/processplatform/assemble/surface/form/v2/lookup/taskcompleted/taskcompleted/mobile'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/form/v2/lookup/taskcompleted/taskcompleted/mobile"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_701_q, (v) => { api_jaxrs_pr_701_data.value = v ?? []; });
const api_jaxrs_pr_26_data = ref<any[]>([]);
const { data: api_jaxrs_pr_26_q } = useQuery({queryKey: ['api_jaxrs_pr_26', '/jaxrs/processplatform/assemble/surface/job/job/allow/visit/person/person'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/job/job/allow/visit/person/person"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_26_q, (v) => { api_jaxrs_pr_26_data.value = v ?? []; });
const api_jaxrs_pr_824_data = ref<any[]>([]);
const { data: api_jaxrs_pr_824_q } = useQuery({queryKey: ['api_jaxrs_pr_824', '/jaxrs/processplatform/assemble/surface/job/job/find/work/workcompleted'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/job/job/find/work/workcompleted"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_824_q, (v) => { api_jaxrs_pr_824_data.value = v ?? []; });
const jaxrs_processplatform_assemble_surface_job_v2_job_projection_ref = ref<any[]>([]);
const jaxrs_processplatform_assemble_surface_job_v2_job_projection_q = useQuery({
  queryKey: ['jaxrs_processplatform_assemble_surface_job_v2_job_projection'],
  queryFn: async () => {
    try { const r = await api.get("/jaxrs/processplatform/assemble/surface/job/v2/job/projection"); return (r.data ?? []) as any[]; }
    catch { return []; }
  },
  staleTime: 60000,
});
const api_jaxrs_pr_183_data = ref<any[]>([]);
const { data: api_jaxrs_pr_183_q } = useQuery({queryKey: ['api_jaxrs_pr_183', '/jaxrs/processplatform/assemble/surface/read/filter/attribute/filter'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/read/filter/attribute/filter"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_183_q, (v) => { api_jaxrs_pr_183_data.value = v ?? []; });
const api_jaxrs_pr_939_data = ref<any[]>([]);
const { data: api_jaxrs_pr_939_q } = useQuery({queryKey: ['api_jaxrs_pr_939', '/jaxrs/processplatform/assemble/surface/read/list/count/application'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/read/list/count/application"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_939_q, (v) => { api_jaxrs_pr_939_data.value = v ?? []; });
const api_jaxrs_pr_56_data = ref<any[]>([]);
const { data: api_jaxrs_pr_56_q } = useQuery({queryKey: ['api_jaxrs_pr_56', '/jaxrs/processplatform/assemble/surface/read/list/date/date/manage'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/read/list/date/date/manage"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_56_q, (v) => { api_jaxrs_pr_56_data.value = v ?? []; });
const api_jaxrs_pr_897_data = ref<any[]>([]);
const { data: api_jaxrs_pr_897_q } = useQuery({queryKey: ['api_jaxrs_pr_897', '/jaxrs/processplatform/assemble/surface/read/list/job/job'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/read/list/job/job"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_897_q, (v) => { api_jaxrs_pr_897_data.value = v ?? []; });
const api_jaxrs_pr_48_data = ref<any[]>([]);
const { data: api_jaxrs_pr_48_q } = useQuery({queryKey: ['api_jaxrs_pr_48', '/jaxrs/processplatform/assemble/surface/read/list/person/person/manage'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/read/list/person/person/manage"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_48_q, (v) => { api_jaxrs_pr_48_data.value = v ?? []; });
const api_jaxrs_pr_952_data = ref<any[]>([]);
const { data: api_jaxrs_pr_952_q } = useQuery({queryKey: ['api_jaxrs_pr_952', '/jaxrs/processplatform/assemble/surface/readcompleted/filter/attribute/filter'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/readcompleted/filter/attribute/filter"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_952_q, (v) => { api_jaxrs_pr_952_data.value = v ?? []; });
const api_jaxrs_pr_640_data = ref<any[]>([]);
const { data: api_jaxrs_pr_640_q } = useQuery({queryKey: ['api_jaxrs_pr_640', '/jaxrs/processplatform/assemble/surface/readcompleted/list/count/application'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/readcompleted/list/count/application"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_640_q, (v) => { api_jaxrs_pr_640_data.value = v ?? []; });
const api_jaxrs_pr_154_data = ref<any[]>([]);
const { data: api_jaxrs_pr_154_q } = useQuery({queryKey: ['api_jaxrs_pr_154', '/jaxrs/processplatform/assemble/surface/readcompleted/list/date/date/manage'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/readcompleted/list/date/date/manage"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_154_q, (v) => { api_jaxrs_pr_154_data.value = v ?? []; });
const api_jaxrs_pr_661_data = ref<any[]>([]);
const { data: api_jaxrs_pr_661_q } = useQuery({queryKey: ['api_jaxrs_pr_661', '/jaxrs/processplatform/assemble/surface/readcompleted/list/job/job'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/readcompleted/list/job/job"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_661_q, (v) => { api_jaxrs_pr_661_data.value = v ?? []; });
const api_jaxrs_pr_951_data = ref<any[]>([]);
const { data: api_jaxrs_pr_951_q } = useQuery({queryKey: ['api_jaxrs_pr_951', '/jaxrs/processplatform/assemble/surface/readcompleted/list/work/work'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/readcompleted/list/work/work"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_951_q, (v) => { api_jaxrs_pr_951_data.value = v ?? []; });
const api_jaxrs_pr_769_data = ref<any[]>([]);
const { data: api_jaxrs_pr_769_q } = useQuery({queryKey: ['api_jaxrs_pr_769', '/jaxrs/processplatform/assemble/surface/readrecord/list/job/job'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/readrecord/list/job/job"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_769_q, (v) => { api_jaxrs_pr_769_data.value = v ?? []; });
const api_jaxrs_pr_567_data = ref<any[]>([]);
const { data: api_jaxrs_pr_567_q } = useQuery({queryKey: ['api_jaxrs_pr_567', '/jaxrs/processplatform/assemble/surface/record/job/job/manage'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/record/job/job/manage"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_567_q, (v) => { api_jaxrs_pr_567_data.value = v ?? []; });
const api_jaxrs_pr_91_data = ref<any[]>([]);
const { data: api_jaxrs_pr_91_q } = useQuery({queryKey: ['api_jaxrs_pr_91', '/jaxrs/processplatform/assemble/surface/record/list/job/job'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/record/list/job/job"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_91_q, (v) => { api_jaxrs_pr_91_data.value = v ?? []; });
const api_jaxrs_pr_864_data = ref<any[]>([]);
const { data: api_jaxrs_pr_864_q } = useQuery({queryKey: ['api_jaxrs_pr_864', '/jaxrs/processplatform/assemble/surface/review/filter/create/entry'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/review/filter/create/entry"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_864_q, (v) => { api_jaxrs_pr_864_data.value = v ?? []; });
const api_jaxrs_pr_803_data = ref<any[]>([]);
const { data: api_jaxrs_pr_803_q } = useQuery({queryKey: ['api_jaxrs_pr_803', '/jaxrs/processplatform/assemble/surface/review/list/job/job'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/review/list/job/job"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_803_q, (v) => { api_jaxrs_pr_803_data.value = v ?? []; });
const api_jaxrs_pr_449_data = ref<any[]>([]);
const { data: api_jaxrs_pr_449_q } = useQuery({queryKey: ['api_jaxrs_pr_449', '/jaxrs/processplatform/assemble/surface/sign/list/job/job'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/sign/list/job/job"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_449_q, (v) => { api_jaxrs_pr_449_data.value = v ?? []; });
const api_jaxrs_pr_220_data = ref<any[]>([]);
const { data: api_jaxrs_pr_220_q } = useQuery({queryKey: ['api_jaxrs_pr_220', '/jaxrs/processplatform/assemble/surface/task/filter/attribute/filter'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/task/filter/attribute/filter"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_220_q, (v) => { api_jaxrs_pr_220_data.value = v ?? []; });
const api_jaxrs_pr_227_data = ref<any[]>([]);
const { data: api_jaxrs_pr_227_q } = useQuery({queryKey: ['api_jaxrs_pr_227', '/jaxrs/processplatform/assemble/surface/task/list/count/application'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/task/list/count/application"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_227_q, (v) => { api_jaxrs_pr_227_data.value = v ?? []; });
const api_jaxrs_pr_729_data = ref<any[]>([]);
const { data: api_jaxrs_pr_729_q } = useQuery({queryKey: ['api_jaxrs_pr_729', '/jaxrs/processplatform/assemble/surface/task/list/date/2024-01-01/hour/09/exclude/draft/true/manage'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/task/list/date/2024-01-01/hour/09/exclude/draft/true/manage"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_729_q, (v) => { api_jaxrs_pr_729_data.value = v ?? []; });
const api_jaxrs_pr_664_data = ref<any[]>([]);
const { data: api_jaxrs_pr_664_q } = useQuery({queryKey: ['api_jaxrs_pr_664', '/jaxrs/processplatform/assemble/surface/task/list/person/p1/exclude/draft/true/manage'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/task/list/person/p1/exclude/draft/true/manage"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_664_q, (v) => { api_jaxrs_pr_664_data.value = v ?? []; });
const api_jaxrs_pr_547_data = ref<any[]>([]);
const { data: api_jaxrs_pr_547_q } = useQuery({queryKey: ['api_jaxrs_pr_547', '/jaxrs/processplatform/assemble/surface/task/list/work/work'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/task/list/work/work"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_547_q, (v) => { api_jaxrs_pr_547_data.value = v ?? []; });
const api_jaxrs_pr_273_data = ref<any[]>([]);
const { data: api_jaxrs_pr_273_q } = useQuery({queryKey: ['api_jaxrs_pr_273', '/jaxrs/processplatform/assemble/surface/taskcompleted/filter/attribute/filter'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/taskcompleted/filter/attribute/filter"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_273_q, (v) => { api_jaxrs_pr_273_data.value = v ?? []; });
const api_jaxrs_pr_413_data = ref<any[]>([]);
const { data: api_jaxrs_pr_413_q } = useQuery({queryKey: ['api_jaxrs_pr_413', '/jaxrs/processplatform/assemble/surface/taskcompleted/list/count/application'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/taskcompleted/list/count/application"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_413_q, (v) => { api_jaxrs_pr_413_data.value = v ?? []; });
const api_jaxrs_pr_731_data = ref<any[]>([]);
const { data: api_jaxrs_pr_731_q } = useQuery({queryKey: ['api_jaxrs_pr_731', '/jaxrs/processplatform/assemble/surface/taskcompleted/list/date/date/hour/hour/manage'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/taskcompleted/list/date/date/hour/hour/manage"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_731_q, (v) => { api_jaxrs_pr_731_data.value = v ?? []; });
const api_jaxrs_pr_102_data = ref<any[]>([]);
const { data: api_jaxrs_pr_102_q } = useQuery({queryKey: ['api_jaxrs_pr_102', '/jaxrs/processplatform/assemble/surface/taskcompleted/list/job/job'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/taskcompleted/list/job/job"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_102_q, (v) => { api_jaxrs_pr_102_data.value = v ?? []; });
const api_jaxrs_pr_531_data = ref<any[]>([]);
const { data: api_jaxrs_pr_531_q } = useQuery({queryKey: ['api_jaxrs_pr_531', '/jaxrs/processplatform/assemble/surface/taskcompleted/list/work/work'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/taskcompleted/list/work/work"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_531_q, (v) => { api_jaxrs_pr_531_data.value = v ?? []; });
const api_jaxrs_pr_617_data = ref<any[]>([]);
const { data: api_jaxrs_pr_617_q } = useQuery({queryKey: ['api_jaxrs_pr_617', '/jaxrs/processplatform/assemble/surface/taskcompleted/press/work/work'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/taskcompleted/press/work/work"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_617_q, (v) => { api_jaxrs_pr_617_data.value = v ?? []; });
const api_jaxrs_pr_710_data = ref<any[]>([]);
const { data: api_jaxrs_pr_710_q } = useQuery({queryKey: ['api_jaxrs_pr_710', '/jaxrs/processplatform/assemble/surface/work/v3/retract/stage/job/job'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/work/v3/retract/stage/job/job"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_710_q, (v) => { api_jaxrs_pr_710_data.value = v ?? []; });
const api_jaxrs_pr_241_data = ref<any[]>([]);
const { data: api_jaxrs_pr_241_q } = useQuery({queryKey: ['api_jaxrs_pr_241', '/jaxrs/processplatform/assemble/surface/workcompleted/list/count/application'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/workcompleted/list/count/application"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_241_q, (v) => { api_jaxrs_pr_241_data.value = v ?? []; });
const api_jaxrs_pr_360_data = ref<any[]>([]);
const { data: api_jaxrs_pr_360_q } = useQuery({queryKey: ['api_jaxrs_pr_360', '/jaxrs/processplatform/assemble/surface/worklog/list/job/job'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/assemble/surface/worklog/list/job/job"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_360_q, (v) => { api_jaxrs_pr_360_data.value = v ?? []; });
const api_jaxrs_pr_509_data = ref<any[]>([]);
const { data: api_jaxrs_pr_509_q } = useQuery({queryKey: ['api_jaxrs_pr_509', '/jaxrs/processplatform/service/processing/attachment/att-x/work/w-other'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/service/processing/attachment/att-x/work/w-other"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_509_q, (v) => { api_jaxrs_pr_509_data.value = v ?? []; });
const api_jaxrs_pr_752_data = ref<any[]>([]);
const { data: api_jaxrs_pr_752_q } = useQuery({queryKey: ['api_jaxrs_pr_752', '/jaxrs/processplatform/service/processing/attachment/copy/work/att-dst-w'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/service/processing/attachment/copy/work/att-dst-w"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_752_q, (v) => { api_jaxrs_pr_752_data.value = v ?? []; });
const api_jaxrs_pr_866_data = ref<any[]>([]);
const { data: api_jaxrs_pr_866_q } = useQuery({queryKey: ['api_jaxrs_pr_866', '/jaxrs/processplatform/service/processing/event/add/update/table'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/service/processing/event/add/update/table"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_866_q, (v) => { api_jaxrs_pr_866_data.value = v ?? []; });
const api_jaxrs_pr_604_data = ref<any[]>([]);
const { data: api_jaxrs_pr_604_q } = useQuery({queryKey: ['api_jaxrs_pr_604', '/jaxrs/processplatform/service/processing/snap/work/sus-w/type/suspend'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/service/processing/snap/work/sus-w/type/suspend"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_604_q, (v) => { api_jaxrs_pr_604_data.value = v ?? []; });
const api_jaxrs_pr_706_data = ref<any[]>([]);
const { data: api_jaxrs_pr_706_q } = useQuery({queryKey: ['api_jaxrs_pr_706', '/jaxrs/processplatform/service/processing/taskcompleted/next/task/identity'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/service/processing/taskcompleted/next/task/identity"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_706_q, (v) => { api_jaxrs_pr_706_data.value = v ?? []; });
const api_jaxrs_pr_329_data = ref<any[]>([]);
const { data: api_jaxrs_pr_329_q } = useQuery({queryKey: ['api_jaxrs_pr_329', '/jaxrs/processplatform/service/processing/taskcompleted/tc-press/press/work/tc-other'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/service/processing/taskcompleted/tc-press/press/work/tc-other"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_329_q, (v) => { api_jaxrs_pr_329_data.value = v ?? []; });
const api_jaxrs_pr_695_data = ref<any[]>([]);
const { data: api_jaxrs_pr_695_q } = useQuery({queryKey: ['api_jaxrs_pr_695', '/jaxrs/processplatform/service/processing/taskcompleted/tc-press/press/work/tc-w'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/service/processing/taskcompleted/tc-press/press/work/tc-w"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_695_q, (v) => { api_jaxrs_pr_695_data.value = v ?? []; });
const api_jaxrs_pr_440_data = ref<any[]>([]);
const { data: api_jaxrs_pr_440_q } = useQuery({queryKey: ['api_jaxrs_pr_440', '/jaxrs/processplatform/service/processing/work/process/pd-boot/name/boot/serial'], queryFn: async () => { try { const r = await api.get("/jaxrs/processplatform/service/processing/work/process/pd-boot/name/boot/serial"); return (r.data ?? []) as any[]; } catch { return []; } }, staleTime: 60000});
watch(api_jaxrs_pr_440_q, (v) => { api_jaxrs_pr_440_data.value = v ?? []; });
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
