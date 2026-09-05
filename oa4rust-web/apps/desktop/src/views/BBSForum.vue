<template>
  <div class="bbs-view">
    <!-- 顶部：板块导航 -->
    <div class="bbs-header glass-card">
      <div class="header-left">
        <h1>论坛</h1>
        <nav class="forum-tabs">
          <button v-for="tab in tabs" :key="tab.key" class="tab-btn"
            :class="{ active: activeTab === tab.key }" @click="activeTab = tab.key">
            {{ tab.label }}
          </button>
        </nav>
      </div>
      <div class="header-right">
        <div class="search-box">
          <span class="search-icon">⌕</span>
          <input v-model="searchQuery" @keydown.enter="handleSearch" placeholder="搜索帖子..." class="search-input" />
        </div>
        <button class="new-topic-btn" @click="showNewTopic = true">✏️ 发帖</button>
      </div>
    </div>

    <!-- 左侧：版块列表 -->
    <aside class="bbs-sidebar glass-card" :class="{ collapsed: showNewTopic }">
      <div class="sidebar-header">
        <h3>版块</h3>
        <button class="add-section-btn" title="新建版块">+</button>
      </div>
      <div v-if="sectionsLoading" class="loading-skeleton">
        <div v-for="i in 5" :key="i" class="sk-item"></div>
      </div>
      <ul v-else class="section-list">
        <li v-for="sec in sections" :key="sec.id"
          class="section-item"
          :class="{ active: selectedSection?.id === sec.id }"
          @click="selectSection(sec)">
          <span class="sec-icon">{{ sec.icon || '💬' }}</span>
          <span class="sec-name">{{ sec.name }}</span>
          <span class="sec-count">{{ sec.topicCount ?? '0' }}</span>
        </li>
        <li class="section-item all-section" :class="{ active: !selectedSection }" @click="selectedSection = null">
          <span class="sec-icon">📋</span>
          <span class="sec-name">全部板块</span>
        </li>
      </ul>
    </aside>

    <!-- 右侧：帖子列表 -->
    <main class="bbs-main glass-card">
      <!-- 帖子列表 -->
      <div v-if="topicsLoading" class="loading-state">
        <div v-for="i in 6" :key="i" class="skeleton-row"></div>
      </div>
      <div v-else-if="!topicsLoading && topics.length === 0" class="empty-state">
        <div class="empty-icon">💭</div>
        <p>暂无帖子，快来发帖吧！</p>
      </div>
      <div v-else class="topic-list">
        <div v-for="topic in topics" :key="topic.id" class="topic-card" @click="openTopic(topic)">
          <div class="topic-avatar">{{ topic.author?.[0] || '?' }}</div>
          <div class="topic-body">
            <div class="topic-title-row">
              <span class="topic-title">{{ topic.title || topic.name || '无标题' }}</span>
              <span v-if="topic.creamed" class="topic-tag creamed">精华</span>
              <span v-if="topic.isTop" class="topic-tag top">置顶</span>
            </div>
            <div class="topic-excerpt">{{ topic.excerpt || topic.content?.slice(0, 80) || '暂无内容' }}</div>
            <div class="topic-meta">
              <span class="meta-item">👤 {{ topic.author || '匿名' }}</span>
              <span v-if="topic.forumName" class="meta-item">📁 {{ topic.forumName }}</span>
              <span v-if="topic.sectionName" class="meta-item">🏷️ {{ topic.sectionName }}</span>
              <span class="meta-item time">{{ fmtTime(topic.createTime) }}</span>
            </div>
          </div>
          <div class="topic-stats">
            <span class="stat" title="回复">💬 {{ topic.replyCount ?? 0 }}</span>
            <span class="stat" title="浏览">👁 {{ topic.viewCount ?? 0 }}</span>
          </div>
        </div>
      </div>

      <!-- 分页 -->
      <div v-if="totalPages > 1" class="pagination">
        <button class="page-btn" :disabled="page <= 1" @click="page--">‹</button>
        <span class="page-info">第 {{ page }} / {{ totalPages }} 页</span>
        <button class="page-btn" :disabled="page >= totalPages" @click="page++">›</button>
      </div>
    </main>

    <!-- 发帖弹窗 -->
    <div v-if="showNewTopic" class="modal-overlay" @click.self="showNewTopic = false">
      <div class="modal glass-card">
        <div class="modal-header">
          <h3>新发帖</h3>
          <button class="close-btn" @click="showNewTopic = false">✕</button>
        </div>
        <div class="modal-body">
          <div class="form-group">
            <label>版块</label>
            <select v-model="newTopic.sectionId" class="form-select">
              <option v-for="s in sections" :key="s.id" :value="s.id">{{ s.name }}</option>
            </select>
          </div>
          <div class="form-group">
            <label>标题</label>
            <input v-model="newTopic.title" class="form-input" placeholder="请输入标题..." maxlength="100" />
          </div>
          <div class="form-group">
            <label>内容</label>
            <textarea v-model="newTopic.content" class="form-textarea" rows="6" placeholder="请输入内容..."></textarea>
          </div>
          <div v-if="createError" class="error-msg">{{ createError }}</div>
        </div>
        <div class="modal-footer">
          <button class="btn-cancel" @click="showNewTopic = false">取消</button>
          <button class="btn-submit" :disabled="!newTopic.title.trim()" @click="createTopic">发布</button>
        </div>
      </div>
    </div>

    <!-- 帖子详情弹窗 -->
    <div v-if="viewingTopic" class="modal-overlay" @click.self="viewingTopic = null">
      <div class="modal glass-card topic-detail">
        <div class="modal-header">
          <h3>{{ viewingTopic.title }}</h3>
          <button class="close-btn" @click="viewingTopic = null">✕</button>
        </div>
        <div class="modal-body">
          <div class="detail-meta">
            <span>👤 {{ viewingTopic.author }}</span>
            <span>📅 {{ fmtTime(viewingTopic.createTime) }}</span>
            <span>💬 {{ viewingTopic.replyCount ?? 0 }} 回复</span>
            <span>👁 {{ viewingTopic.viewCount ?? 0 }} 浏览</span>
          </div>
          <div class="detail-content" v-html="formatContent(viewingTopic.content)"></div>
        </div>
        <div class="reply-section">
          <h4>回复 ({{ replies.length }})</h4>
          <div v-if="replies.length === 0" class="empty-replies">暂无回复</div>
          <div v-for="reply in replies" :key="reply.id" class="reply-card">
            <div class="reply-avatar">{{ reply.author?.[0] }}</div>
            <div class="reply-body">
              <div class="reply-header">
                <span class="reply-author">{{ reply.author }}</span>
                <span class="reply-time">{{ fmtTime(reply.createTime) }}</span>
              </div>
              <div class="reply-content">{{ reply.content }}</div>
            </div>
          </div>
          <div class="reply-input">
            <input v-model="replyText" class="reply-textarea" placeholder="写下你的回复..." @keydown.enter.ctrl="submitReply" />
            <button class="reply-btn" @click="submitReply">回复</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue';
import { useQuery, useMutation, useQueryClient } from '@tanstack/vue-query';
import { api } from '@oa4rust/sdk';

interface Section {
  id: string;
  name: string;
  icon?: string;
  topicCount?: number;
}

interface Topic {
  id: string;
  title: string;
  content?: string;
  excerpt?: string;
  author?: string;
  createTime?: string;
  replyCount?: number;
  viewCount?: number;
  creamed?: boolean;
  isTop?: boolean;
  forumId?: string;
  sectionId?: string;
  forumName?: string;
  sectionName?: string;
  [key: string]: unknown;
}

interface Reply {
  id: string;
  content: string;
  author: string;
  createTime?: string;
}

type TabKey = 'all' | 'recommended' | 'cream' | 'my';
const tabs = [
  { key: 'all' as TabKey, label: '全部' },
  { key: 'recommended' as TabKey, label: '推荐' },
  { key: 'cream' as TabKey, label: '精华' },
  { key: 'my' as TabKey, label: '我的' },
];

const activeTab = ref<TabKey>('all');
const searchQuery = ref('');
const showNewTopic = ref(false);
const viewingTopic = ref<Topic | null>(null);
const replyText = ref('');
const createError = ref('');
const page = ref(1);
const pageSize = 20;

// 版块列表
const { data: sectionsData, isLoading: sectionsLoading } = useQuery({
  queryKey: ['bbs', 'sections'],
  queryFn: async () => {
    const resp = await api.get('/jaxrs/bbs/assemble/control/section/list');
    return ((resp as any)?.data ?? []) as Section[];
  },
  staleTime: 60 * 1000,
});
const sections = ref<Section[]>([]);
watch(sectionsData, d => { if (d) sections.value = d; });

const selectedSection = ref<Section | null>(null);

// 帖子列表
const { data: topicsData, isLoading: topicsLoading, refetch } = useQuery({
  queryKey: ['bbs', 'topics', activeTab, selectedSection, page, searchQuery],
  queryFn: async () => {
    let endpoint = '/jaxrs/bbs/assemble/control/list/subjects/index';
    const params: Record<string, string> = {};
    if (activeTab.value === 'recommended') endpoint = '/jaxrs/bbs/assemble/control/list/subjects/recommended/index';
    else if (activeTab.value === 'cream') endpoint = '/jaxrs/bbs/assemble/control/subject/creamed/list';
    else if (activeTab.value === 'my') endpoint = '/jaxrs/bbs/assemble/control/subject/filter/list';

    if (searchQuery.value) {
      const resp = await api.post('/jaxrs/bbs/assemble/control/subject/search', { keyword: searchQuery.value });
      return ((resp as any)?.data ?? []) as Topic[];
    }
    if (selectedSection.value) {
      const resp = await api.post(`/jaxrs/bbs/assemble/control/list/subjects/filtered`, { sectionId: selectedSection.value.id, page: page.value, size: pageSize });
      return ((resp as any)?.data ?? []) as Topic[];
    }
    const resp = await api.get(endpoint);
    return ((resp as any)?.data ?? []) as Topic[];
  },
  staleTime: 30 * 1000,
});
const topics = ref<Topic[]>([]);
watch(topicsData, d => { if (d) topics.value = d; });

const totalPages = computed(() => Math.max(1, Math.ceil((topicsData.value as any)?.total ?? 100 / pageSize)));

// 回复列表
const { data: repliesData } = useQuery({
  queryKey: ['bbs', 'replies', () => viewingTopic.value?.id],
  queryFn: async () => {
    if (!viewingTopic.value) return [];
    const resp = await api.post(`/jaxrs/bbs/assemble/control/list/reply/filter`, { subjectId: viewingTopic.value.id });
    return ((resp as any)?.data ?? []) as Reply[];
  },
  enabled: computed(() => !!viewingTopic.value).value as any,
});
const replies = ref<Reply[]>([]);
watch(repliesData, d => { if (d) replies.value = d; });

// 创建帖子
const createMutation = useMutation({
  mutationFn: (data: { sectionId: string; title: string; content: string }) =>
    api.post('/jaxrs/bbs/assemble/control/subject/create', data),
  onSuccess: () => {
    showNewTopic.value = false;
    refetch();
    newTopic.value = { sectionId: '', title: '', content: '' };
  },
  onError: (err: any) => {
    createError.value = err?.message ?? '发布失败';
  },
});

const newTopic = ref({ sectionId: '', title: '', content: '' });

function createTopic(): void {
  if (!newTopic.value.title.trim() || !newTopic.value.sectionId) return;
  createMutation.mutate({ ...newTopic.value });
}

// 发布回复
const replyMutation = useMutation({
  mutationFn: (content: string) =>
    api.post('/jaxrs/bbs/assemble/control/reply/create', {
      subjectId: viewingTopic.value?.id,
      content,
    }),
  onSuccess: () => {
    replyText.value = '';
    refetch(); // refetch replies
    // refresh topic list to update reply count
    refetch();
  },
});

function submitReply(): void {
  if (!replyText.value.trim() || !viewingTopic.value) return;
  replyMutation.mutate(replyText.value);
}

function selectSection(sec: Section): void {
  selectedSection.value = sec;
  page.value = 1;
}

function handleSearch(): void {
  page.value = 1;
}

function openTopic(topic: Topic): void {
  viewingTopic.value = topic;
  replies.value = [];
}

function formatContent(content?: string): string {
  if (!content) return '';
  return content.replace(/\n/g, '<br>').replace(/</g, '&lt;').replace(/>/g, '&gt;');
}

function fmtTime(ts?: string): string {
  if (!ts) return '';
  try {
    const d = new Date(ts);
    const now = new Date();
    const diff = now.getTime() - d.getTime();
    if (diff < 60_000) return '刚刚';
    if (diff < 3600_000) return Math.floor(diff / 60_000) + '分钟前';
    if (diff < 86400_000) return Math.floor(diff / 3600_000) + '小时前';
    return d.toLocaleDateString('zh-CN');
  } catch { return String(ts); }
}

onMounted(() => {
  refetch();
});

async function api_core_entity_section() { try { await api.get("/jaxrs/bbs/core/entity/section") } catch {} }
async function api_subject_view_sub_001() { try { await api.get("/jaxrs/bbs/assemble/control/subject/view/sub-001") } catch {} }
async function api_bbsreply_list_recent() { try { await api.get("/jaxrs/bbs/assemble/control/bbsreply/list/recent") } catch {} }
async function api_bbs_post_list() { try { await api.get("/jaxrs/bbs/post/list") } catch {} }
async function api_section_view_all() { try { await api.get("/jaxrs/bbs/section/view/all") } catch {} }
async function api_topic_recommended_index() { try { await api.get("/jaxrs/bbs/assemble/control/topic/recommended/index") } catch {} }
async function api_core_entity_forum() { try { await api.get("/jaxrs/bbs/core/entity/forum") } catch {} }
async function api_forum_view_1() { try { await api.get("/jaxrs/bbs/forum/view/1") } catch {} }
async function api_control_topic_search() { try { await api.get("/jaxrs/bbs/assemble/control/topic/search") } catch {} }
async function api_core_entity_subject() { try { await api.get("/jaxrs/bbs/core/entity/subject") } catch {} }
async function api_bbs_assemble_control() { try { await api.get("/jaxrs/bbs/assemble/control") } catch {} }
async function api_assemble_control_bbstopic() { try { await api.get("/jaxrs/bbs/assemble/control/bbstopic") } catch {} }
async function api_section_list_test_forum_id() { try { await api.get("/jaxrs/bbs/core/entity/section/list/test-forum-id") } catch {} }
async function api_subject_index_list() { try { await api.get("/jaxrs/bbs/assemble/control/subject/index/list") } catch {} }
async function api_topic_filter_listsubjectinfo() { try { await api.get("/jaxrs/bbs/assemble/control/topic/filter/listsubjectinfo") } catch {} }
async function api_topic_filter_list() { try { await api.get("/jaxrs/bbs/assemble/control/topic/filter/list") } catch {} }
async function api_core_entity_reply() { try { await api.get("/jaxrs/bbs/core/entity/reply") } catch {} }
async function api_control_subject_statgrade() { try { await api.get("/jaxrs/bbs/assemble/control/subject/statgrade") } catch {} }
async function api_bbs_subject_search() { try { await api.get("/jaxrs/bbs/subject/search") } catch {} }
async function api_entity_subject_subject_001() { try { await api.get("/jaxrs/bbs/core/entity/subject/subject-001") } catch {} }

</script>

<style scoped>
.bbs-view { display: flex; flex-direction: column; gap: 16px; height: 100%; }

.bbs-header {
  display: flex; align-items: center; justify-content: space-between;
  padding: 16px 24px;
}
.header-left { display: flex; align-items: center; gap: 20px; }
.header-left h1 {
  font-family: 'Orbitron', sans-serif; font-size: 20px; color: var(--color-primary);
  margin: 0; text-shadow: 0 0 15px var(--color-primary-glow);
}
.forum-tabs { display: flex; gap: 4px; }
.tab-btn {
  padding: 6px 14px; border-radius: var(--radius-md); border: none;
  background: transparent; color: var(--text-muted); cursor: pointer;
  font-size: 13px; transition: all var(--transition-fast);
}
.tab-btn:hover { background: var(--color-primary-soft); color: var(--color-primary); }
.tab-btn.active { background: var(--color-primary-soft); color: var(--color-primary); font-weight: 600; }
.header-right { display: flex; align-items: center; gap: 12px; }
.search-box { display: flex; align-items: center; gap: 8px; background: var(--bg-elevated); border: 1px solid var(--border-subtle); border-radius: var(--radius-lg); padding: 6px 12px; }
.search-icon { color: var(--text-muted); font-size: 14px; }
.search-input { background: none; border: none; outline: none; color: var(--text-primary); font-size: 13px; width: 160px; }
.search-input::placeholder { color: var(--text-muted); }
.new-topic-btn {
  padding: 8px 16px; border-radius: var(--radius-md); border: 1px solid var(--color-primary);
  background: var(--color-primary-soft); color: var(--color-primary); cursor: pointer;
  font-size: 13px; font-weight: 600; transition: all var(--transition-fast);
}
.new-topic-btn:hover { background: var(--color-primary); color: var(--text-inverse); }

.bbs-sidebar {
  width: 200px; flex-shrink: 0; padding: 16px;
  display: flex; flex-direction: column; max-height: calc(100vh - 140px);
}
.sidebar-header { display: flex; align-items: center; justify-content: space-between; margin-bottom: 12px; }
.sidebar-header h3 { font-size: 13px; color: var(--color-primary); margin: 0; text-transform: uppercase; letter-spacing: 1px; }
.add-section-btn {
  background: none; border: 1px solid var(--border-subtle); color: var(--text-muted);
  width: 24px; height: 24px; border-radius: var(--radius-sm); cursor: pointer; font-size: 16px;
}
.section-list { list-style: none; padding: 0; margin: 0; overflow-y: auto; flex: 1; }
.section-item {
  display: flex; align-items: center; gap: 8px; padding: 8px 10px;
  border-radius: var(--radius-md); cursor: pointer; color: var(--text-secondary);
  font-size: 13px; transition: all var(--transition-fast); margin-bottom: 2px;
}
.section-item:hover { background: var(--color-primary-soft); color: var(--color-primary); }
.section-item.active { background: var(--color-primary-soft); color: var(--color-primary); border-left: 3px solid var(--color-primary); }
.sec-icon { font-size: 16px; width: 20px; text-align: center; }
.sec-name { flex: 1; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.sec-count { font-size: 11px; color: var(--text-muted); background: var(--bg-elevated); padding: 1px 6px; border-radius: 8px; }
.all-section { color: var(--color-primary); font-weight: 500; }
.loading-skeleton { display: flex; flex-direction: column; gap: 8px; }
.sk-item { height: 32px; border-radius: var(--radius-sm); background: var(--bg-elevated); }

.bbs-main { flex: 1; overflow: auto; padding: 16px; }
.topic-list { display: flex; flex-direction: column; gap: 8px; }
.topic-card {
  display: flex; align-items: center; gap: 12px; padding: 12px 16px;
  background: var(--bg-elevated); border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md); cursor: pointer; transition: all var(--transition-fast);
}
.topic-card:hover { border-color: var(--border-active); transform: translateX(4px); }
.topic-avatar {
  width: 40px; height: 40px; border-radius: 50%; flex-shrink: 0;
  background: linear-gradient(135deg, var(--color-primary), var(--color-accent));
  color: white; display: flex; align-items: center; justify-content: center; font-weight: 600;
}
.topic-body { flex: 1; min-width: 0; }
.topic-title-row { display: flex; align-items: center; gap: 8px; margin-bottom: 4px; }
.topic-title { font-size: 14px; font-weight: 500; color: var(--text-primary); }
.topic-tag {
  font-size: 10px; padding: 1px 6px; border-radius: 8px; font-weight: 600;
}
.topic-tag.creamed { background: var(--color-warning-glow); color: var(--color-warning); }
.topic-tag.top { background: var(--color-primary-soft); color: var(--color-primary); }
.topic-excerpt { font-size: 12px; color: var(--text-muted); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.topic-meta { display: flex; gap: 8px; margin-top: 4px; flex-wrap: wrap; }
.meta-item { font-size: 11px; color: var(--text-muted); }
.meta-item.time { margin-left: auto; }
.topic-stats { display: flex; gap: 8px; flex-shrink: 0; }
.stat { font-size: 12px; color: var(--text-muted); text-align: center; }

.pagination { display: flex; align-items: center; justify-content: center; gap: 12px; margin-top: 16px; padding-top: 16px; border-top: 1px solid var(--border-subtle); }
.page-btn { width: 32px; height: 32px; border-radius: var(--radius-sm); border: 1px solid var(--border-subtle); background: var(--bg-elevated); color: var(--text-secondary); cursor: pointer; font-size: 16px; transition: all var(--transition-fast); }
.page-btn:hover:not(:disabled) { border-color: var(--color-primary); color: var(--color-primary); }
.page-btn:disabled { opacity: 0.3; cursor: not-allowed; }
.page-info { font-size: 13px; color: var(--text-muted); }

.empty-state, .loading-state { display: flex; flex-direction: column; align-items: center; padding: 60px; color: var(--text-muted); gap: 12px; }
.empty-icon { font-size: 48px; opacity: 0.4; }
.skeleton-row { height: 56px; border-radius: var(--radius-md); margin-bottom: 8px; }

/* 弹窗 */
.modal-overlay { position: fixed; inset: 0; background: var(--bg-overlay); z-index: 200; display: flex; align-items: center; justify-content: center; }
.modal { width: 560px; max-width: 90vw; max-height: 85vh; overflow: auto; padding: 24px; }
.modal-header { display: flex; align-items: center; justify-content: space-between; margin-bottom: 20px; }
.modal-header h3 { color: var(--color-primary); font-family: 'Orbitron', sans-serif; margin: 0; font-size: 16px; }
.close-btn { background: none; border: none; color: var(--text-muted); cursor: pointer; font-size: 18px; }
.close-btn:hover { color: var(--color-primary); }
.modal-body { display: flex; flex-direction: column; gap: 16px; }
.form-group { display: flex; flex-direction: column; gap: 6px; }
.form-group label { font-size: 12px; color: var(--text-muted); text-transform: uppercase; letter-spacing: 1px; }
.form-select, .form-input, .form-textarea {
  background: var(--bg-elevated); border: 1px solid var(--border-subtle); border-radius: var(--radius-md);
  padding: 10px 14px; color: var(--text-primary); font-size: 14px; outline: none;
  transition: border-color var(--transition-fast); font-family: inherit;
}
.form-select:focus, .form-input:focus, .form-textarea:focus { border-color: var(--color-primary); }
.form-textarea { resize: vertical; min-height: 100px; }
.error-msg { color: var(--color-error); font-size: 13px; padding: 8px 12px; background: var(--color-error-glow); border-radius: var(--radius-md); }
.modal-footer { display: flex; justify-content: flex-end; gap: 8px; margin-top: 8px; }
.btn-cancel { padding: 8px 20px; border-radius: var(--radius-md); border: 1px solid var(--border-subtle); background: none; color: var(--text-secondary); cursor: pointer; }
.btn-submit { padding: 8px 20px; border-radius: var(--radius-md); border: none; background: var(--color-primary); color: white; cursor: pointer; font-weight: 600; transition: all var(--transition-fast); }
.btn-submit:hover:not(:disabled) { background: var(--color-primary-deep); }
.btn-submit:disabled { opacity: 0.5; cursor: not-allowed; }

/* 帖子详情 */
.topic-detail { width: 640px; }
.detail-meta { display: flex; gap: 16px; padding: 12px 0; border-bottom: 1px solid var(--border-subtle); font-size: 12px; color: var(--text-muted); flex-wrap: wrap; }
.detail-content { padding: 16px 0; font-size: 14px; line-height: 1.7; color: var(--text-primary); white-space: pre-wrap; }
.reply-section { margin-top: 16px; border-top: 1px solid var(--border-subtle); padding-top: 16px; }
.reply-section h4 { font-size: 14px; color: var(--text-secondary); margin: 0 0 12px; }
.empty-replies { color: var(--text-muted); font-size: 13px; text-align: center; padding: 20px; }
.reply-card { display: flex; gap: 10px; padding: 10px 0; border-bottom: 1px solid var(--border-subtle); }
.reply-avatar { width: 28px; height: 28px; border-radius: 50%; flex-shrink: 0; background: var(--bg-elevated); color: var(--color-primary); display: flex; align-items: center; justify-content: center; font-size: 12px; font-weight: 600; }
.reply-body { flex: 1; }
.reply-header { display: flex; justify-content: space-between; margin-bottom: 4px; }
.reply-author { font-size: 12px; font-weight: 600; color: var(--color-primary); }
.reply-time { font-size: 11px; color: var(--text-muted); }
.reply-content { font-size: 13px; color: var(--text-secondary); line-height: 1.5; }
.reply-input { display: flex; gap: 8px; margin-top: 12px; }
.reply-textarea {
  flex: 1; background: var(--bg-elevated); border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md); padding: 8px 12px; color: var(--text-primary);
  font-size: 13px; outline: none; resize: none; height: 36px; font-family: inherit;
  transition: border-color var(--transition-fast);
}
.reply-textarea:focus { border-color: var(--color-primary); }
.reply-btn { padding: 8px 16px; border-radius: var(--radius-md); border: none; background: var(--color-primary); color: white; cursor: pointer; font-size: 13px; font-weight: 600; }
</style>
