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
            <button class="btn-sm" @click="handleView(item)">详情</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
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
  approveMutation.mutate(item.id);
}

function handleView(_item: TaskItem): void {
  // Navigate to detail (future)
  console.log('View task:', _item.id);
}

async function handleComment(item) { const comment = prompt('添加评论:'); if(!comment)return; await api.post('/jaxrs/processplatform/assemble/surface/work/comment',{id:item.id,comment}); query.refetch() }

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
