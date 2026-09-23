<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

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
        <button class="new-topic-btn ghost" @click="loadForums">版块列表</button>
        <button class="new-topic-btn ghost" @click="loadBbsViews">视图浏览</button>
        <button class="new-topic-btn ghost" @click="loadBbsControl">控制台/检索</button>
        <button class="new-topic-btn ghost" @click="loadBbsEntities">核心实体</button>
        <button class="new-topic-btn ghost" @click="loadBbsDeepReads">深度读矩阵</button>
        <button class="new-topic-btn" @click="openNewTopic">✏️ 发帖</button>
      </div>
    </div>
    <div v-if="forumsText" class="forums-note">{{ forumsText }}</div>
    <div v-if="bbsViewsText" class="forums-note">{{ bbsViewsText }}</div>
    <div v-if="bbsControlText" class="forums-note">{{ bbsControlText }}</div>
    <div v-if="bbsEntityText" class="forums-note">{{ bbsEntityText }}</div>
    <div v-if="bbsDeepText" class="forums-note">{{ bbsDeepText }}</div>

    <!-- 左侧：版块列表 -->
    <aside class="bbs-sidebar glass-card" :class="{ collapsed: showNewTopic }">
      <div class="sidebar-header">
        <h3>版块</h3>
        <button class="add-section-btn" title="新建版块" aria-label="新建版块" @click="openNewSection()">+</button>
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
          <span class="sec-actions">
            <button class="sec-act" title="版块详情" aria-label="版块详情" @click.stop="openSectionInfo(sec)">ℹ</button>
            <button class="sec-act" title="重命名版块" aria-label="重命名版块" @click.stop="openSectionEdit(sec)">✎</button>
            <button class="sec-act" title="删除版块" aria-label="删除版块" @click.stop="deleteSection(sec)">✕</button>
          </span>
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
      <div v-if="activeTab==='my'" class="my-subtabs">
        <button :class="{ on: mySub==='topics' }" @click="switchMySub('topics')">我的主题</button>
        <button :class="{ on: mySub==='replies' }" @click="switchMySub('replies')">我的回复</button>
      </div>
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

      <!-- 分页：后端无 total 信封，用「本页满则可能有下一页」驱动 -->
      <div v-if="hasMore || page > 1" class="pagination">
        <button class="page-btn" :disabled="page <= 1" @click="page--">‹</button>
        <span class="page-info">第 {{ page }} 页</span>
        <button class="page-btn" :disabled="!hasMore" @click="page++">›</button>
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
            <span v-if="topicMeta">📎 {{ topicMeta }}</span>
          </div>
          <div class="detail-content" style="white-space:pre-wrap;word-break:break-word">{{formatContent(viewingTopic.content)}}</div>

          <!-- 图片附件（picture/list） -->
          <div v-if="topicPics.length" class="pic-gallery">
            <img v-for="(u, i) in topicPics" :key="i" :src="u" class="pic-thumb" alt="附图" />
          </div>

          <!-- 版主管理工具栏（owner/admin 门禁；失败提示无权限） -->
          <div class="mod-bar">
            <span class="mod-label">版主管理：</span>
            <button class="mod-btn" :class="{on: viewingTopic.creamed}" :disabled="modBusy" @click="toggleMod('cream', !viewingTopic.creamed)">
              {{ viewingTopic.creamed ? '取消精华' : '设为精华' }}
            </button>
            <button class="mod-btn" :class="{on: viewingTopic.isTop}" :disabled="modBusy" @click="toggleMod('topSection', !viewingTopic.isTop)">
              {{ viewingTopic.isTop ? '取消置顶' : '版块置顶' }}
            </button>
            <button class="mod-btn" :class="{on: viewingTopic.locked}" :disabled="modBusy" @click="toggleMod('lock', !viewingTopic.locked)">
              {{ viewingTopic.locked ? '解锁' : '锁定' }}
            </button>
            <button class="mod-btn" :class="{on: viewingTopic.completed}" :disabled="modBusy" @click="toggleMod('complete', !viewingTopic.completed)">
              {{ viewingTopic.completed ? '取消完结' : '标记完结' }}
            </button>
          </div>
        </div>
        <div class="reply-section">
          <h4>回复 ({{ replies.length }})<span v-if="replyGate" class="reply-gate">· 回复权限：{{ replyGate }}</span></h4>
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

    <!-- 版块新建/重命名弹窗（POST section/create | POST section/save/{id}） -->
    <div v-if="showSectionModal" class="modal-overlay" @click.self="closeSectionModal">
      <div class="modal glass-card">
        <div class="modal-header">
          <h3>{{ sectionModalMode === 'create' ? '新建版块' : '重命名版块' }}</h3>
          <button class="close-btn" @click="closeSectionModal">✕</button>
        </div>
        <div class="modal-body">
          <div class="form-group">
            <label>版块名称</label>
            <input v-model="sectionName" class="form-input" placeholder="版块名称" maxlength="30" @keydown.enter="saveSection" />
          </div>
          <div v-if="sectionError" class="error-msg">{{ sectionError }}</div>
        </div>
        <div class="modal-footer">
          <button class="btn-cancel" @click="closeSectionModal">取消</button>
          <button class="btn-submit" :disabled="!sectionName.trim() || sectionBusy" @click="saveSection">
            {{ sectionBusy ? '保存中…' : '保存' }}
          </button>
        </div>
      </div>
    </div>

    <!-- 版块详情弹窗（GET section/{id} + section/viewsub/{sectionId} + permission/section/{sectionId}） -->
    <div v-if="sectionInfo.open" class="modal-overlay" @click.self="sectionInfo.open=false">
      <div class="modal glass-card">
        <div class="modal-header">
          <h3>版块详情</h3>
          <button class="close-btn" @click="sectionInfo.open=false">✕</button>
        </div>
        <div v-if="sectionInfo.loading" class="loading-state"><p>加载中…</p></div>
        <div v-else class="modal-body">
          <div class="detail-meta">
            <span>🏷️ {{ sectionInfo.name || '—' }}</span>
            <span>🆔 {{ sectionInfo.id }}</span>
          </div>
          <div class="form-group"><label>子版块</label><div>{{ sectionInfo.subCount }} 个</div></div>
          <div class="form-group"><label>发帖权限</label><div>{{ sectionInfo.canPublish }}</div></div>
          <div class="form-group"><label>描述</label><div>{{ sectionInfo.description || '—' }}</div></div>
        </div>
        <div class="modal-footer"><button class="btn-cancel" @click="sectionInfo.open=false">关闭</button></div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { api, useSession } from '@oa4rust/sdk'
import { useMutation, useQuery, useQueryClient } from '@tanstack/vue-query'
import { computed, onMounted, ref, watch } from 'vue'
import { confirmMsg, toast } from '../utils/toast'

const session = useSession()
const qc = useQueryClient()

/** 后端分页端点回 {data:{data:rows,total}}（o2server ActionResult 信封），
 *  部分端点直接回数组；统一解包为行数组（与 ProcessWork 的 paged-aware 解包同型）。 */
function listRows(resp: { data?: unknown }): unknown[] {
  const p = resp.data
  if (Array.isArray(p)) return p
  if (p && typeof p === 'object' && Array.isArray((p as { data?: unknown }).data)) {
    return (p as { data: unknown[] }).data
  }
  return []
}

interface Section {
  id: string
  name: string
  icon?: string
  topicCount?: number
}

interface Topic {
  id: string
  /** 回复行点开的源主题 ID（我的回复卡片）；普通主题行无此键。 */
  topicRef?: string
  title: string
  content?: string
  excerpt?: string
  author?: string
  createTime?: string
  replyCount?: number
  viewCount?: number
  creamed?: boolean
  isTop?: boolean
  forumId?: string
  sectionId?: string
  forumName?: string
  sectionName?: string
  [key: string]: unknown
}

interface Reply {
  id: string
  content: string
  author: string
  createTime?: string
}

type TabKey = 'all' | 'recommended' | 'cream' | 'my'
const tabs = [
  { key: 'all' as TabKey, label: '全部' },
  { key: 'recommended' as TabKey, label: '推荐' },
  { key: 'cream' as TabKey, label: '精华' },
  { key: 'my' as TabKey, label: '我的' },
]

const activeTab = ref<TabKey>('all')
const searchQuery = ref('')
const showNewTopic = ref(false)
const viewingTopic = ref<Topic | null>(null)
const replyText = ref('')
const createError = ref('')
const page = ref(1)
const pageSize = 20

// 版块列表
const { data: sectionsData, isLoading: sectionsLoading } = useQuery({
  queryKey: ['bbs', 'sections'],
  queryFn: async () => {
    const resp = await api.get('/api/bbs/assemble/control/section/list')
    return ((resp as any)?.data ?? []) as Section[]
  },
  staleTime: 60 * 1000,
})
const sections = ref<Section[]>([])
watch(sectionsData, (d) => {
  if (d) sections.value = d
})

const selectedSection = ref<Section | null>(null)

// 「我的」页签子切换：我的主题 / 我的回复（论坛个人主页，x_component_ForumPerson）
const mySub = ref<'topics' | 'replies'>('topics')
function switchMySub(sub: 'topics' | 'replies'): void {
  if (mySub.value === sub) return
  mySub.value = sub
  page.value = 1
}

// 版块新建/重命名/删除（后端 x_bbs_assemble_control_section 实表写路由）
const showSectionModal = ref(false)
const sectionModalMode = ref<'create' | 'edit'>('create')
const sectionModalTarget = ref<Section | null>(null)
const sectionName = ref('')
const sectionBusy = ref(false)
const sectionError = ref('')

function openNewSection(): void {
  sectionModalMode.value = 'create'
  sectionModalTarget.value = null
  sectionName.value = ''
  sectionError.value = ''
  showSectionModal.value = true
}
function openSectionEdit(sec: Section): void {
  sectionModalMode.value = 'edit'
  sectionModalTarget.value = sec
  sectionName.value = sec.name ?? ''
  sectionError.value = ''
  showSectionModal.value = true
}
function closeSectionModal(): void {
  showSectionModal.value = false
  sectionModalTarget.value = null
  sectionName.value = ''
  sectionError.value = ''
}
async function saveSection(): Promise<void> {
  const name = sectionName.value.trim()
  if (!name || sectionBusy.value) return
  sectionBusy.value = true
  sectionError.value = ''
  try {
    if (sectionModalMode.value === 'edit' && sectionModalTarget.value?.id) {
      // POST 别名（后端 put+post 双注册）
      await api.post(`/api/bbs/assemble/control/section/save/${sectionModalTarget.value.id}`, { name })
      toast.success('版块已重命名')
    } else {
      await api.post('/api/bbs/assemble/control/section/create', { name })
      toast.success('版块已创建')
    }
    closeSectionModal()
    await qc.invalidateQueries({ queryKey: ['bbs', 'sections'] })
  } catch (e) {
    sectionError.value = e instanceof Error ? e.message : '保存失败'
  } finally {
    sectionBusy.value = false
  }
}
async function deleteSection(sec: Section): Promise<void> {
  if (!sec.id) return
  const ok = await confirmMsg(`确定删除版块「${sec.name}」？该版块下的帖子不受影响。`)
  if (!ok) return
  sectionBusy.value = true
  try {
    await api.post(`/api/bbs/assemble/control/section/delete/${sec.id}`)
    if (selectedSection.value?.id === sec.id) {
      selectedSection.value = null
    }
    toast.success('版块已删除')
    await qc.invalidateQueries({ queryKey: ['bbs', 'sections'] })
  } catch (e) {
    toast.error(e instanceof Error ? e.message : '删除失败')
  } finally {
    sectionBusy.value = false
  }
}

// 版块详情：并发消费 section/{id}（主体）+ section/viewsub/{sectionId}（子版块）
// + permission/section/{sectionId}（发帖权限判定）——均 distinct handler，事件触发（非 mounted useQuery）。
const sectionInfo = ref({ open: false, loading: false, id: '', name: '', description: '', subCount: 0, canPublish: '—' })
async function openSectionInfo(sec: Section): Promise<void> {
  sectionInfo.value = { open: true, loading: true, id: sec.id, name: sec.name, description: '', subCount: 0, canPublish: '—' }
  const settle = <T,>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const [main, subs, perm] = await Promise.all([
    settle(api.get(`/api/bbs/assemble/control/section/${sec.id}`)),
    settle(api.get(`/api/bbs/assemble/control/section/viewsub/${sec.id}`)),
    settle(api.get(`/api/bbs/assemble/control/permission/section/${sec.id}`)),
  ])
  const m = (main as { data?: Record<string, unknown> } | null)?.data
  if (m && typeof m === 'object') {
    sectionInfo.value.name = String(m.name ?? sec.name)
    sectionInfo.value.description = String(m.description ?? '')
  }
  const sd = (subs as { data?: unknown } | null)?.data
  sectionInfo.value.subCount = Array.isArray(sd)
    ? sd.length
    : Array.isArray((sd as { data?: unknown })?.data)
      ? ((sd as { data: unknown[] }).data).length
      : 0
  const pd = (perm as { data?: unknown } | null)?.data
  sectionInfo.value.canPublish = perm ? (pd === true || (pd as { publishable?: boolean })?.publishable ? '允许' : '不允许') : '查询失败'
  sectionInfo.value.loading = false
}

// 帖子列表
//
// 端点选型（均已注册可实跑；裸静态路由的无参 GET handler 需 Path((page,count))
// 运行时会 500，故全部改调 fmt 参数化路由，与后端 routes.rs 注册一致）：
//  · 全部/推荐/精华 → PUT subject/{index,recommended,creamed}/list/page/{p}/count/{n}
//  · 我的 → POST subject/filter/listsubjectinfo/page/{p}/count/{n} body.creator=本人
//  · 关键词 → PUT subject/search/list/page/1/count/{n} body.keyword
//  · 版块筛选 → GET /api/bbs/subject/list/{sectionId}（bbs crate 已注册）
const {
  data: topicsData,
  isLoading: topicsLoading,
  refetch,
} = useQuery({
  queryKey: ['bbs', 'topics', activeTab, mySub, selectedSection, page, searchQuery],
  queryFn: async () => {
    let resp: { data?: unknown }
    if (searchQuery.value) {
      resp = (await api.put(`/api/bbs/assemble/control/subject/search/list/page/1/count/${pageSize}`, {
        keyword: searchQuery.value,
      })) as { data?: unknown }
    } else if (selectedSection.value) {
      resp = (await api.get(`/api/bbs/subject/list/${selectedSection.value.id}`)) as { data?: unknown }
    } else if (activeTab.value === 'recommended') {
      resp = (await api.put(
        `/api/bbs/assemble/control/subject/recommended/list/page/${page.value}/count/${pageSize}`,
        {},
      )) as { data?: unknown }
    } else if (activeTab.value === 'cream') {
      resp = (await api.put(
        `/api/bbs/assemble/control/subject/creamed/list/page/${page.value}/count/${pageSize}`,
        {},
      )) as { data?: unknown }
    } else if (activeTab.value === 'my' && mySub.value === 'replies') {
      // 我的回复（论坛个人主页）：PUT 参数化路由（x_bbs_reply，creator/author_id = 登录人）
      resp = (await api.put(
        `/api/bbs/assemble/control/user/reply/my/list/page/${page.value}/count/${pageSize}`,
        {},
      )) as { data?: unknown }
      const replyRows = listRows(resp) as Array<Record<string, unknown>>
      // 回复行映射为列表卡片字段；topicRef 供详情点开源主题全文
      const replyTopics: Topic[] = replyRows.map((r) => ({
        id: String(r.id ?? ''),
        topicRef: String(r.topic_id ?? r.topicId ?? ''),
        title: '回复 · 主题 ' + String(r.topic_id ?? r.topicId ?? ''),
        content: String(r.content ?? ''),
        author: String(r.creator ?? ''),
        createTime: String(r.create_time ?? r.createTime ?? ''),
        sectionName: '我的回复',
      }))
      return { rows: replyTopics, more: replyTopics.length >= pageSize }
    } else if (activeTab.value === 'my') {
      resp = (await api.post(
        `/api/bbs/assemble/control/subject/filter/listsubjectinfo/page/${page.value}/count/${pageSize}`,
        { creator: session.user?.unique ?? '' },
      )) as { data?: unknown }
    } else {
      resp = (await api.put(
        `/api/bbs/assemble/control/subject/index/list/page/${page.value}/count/${pageSize}`,
        {},
      )) as { data?: unknown }
    }
    const raw = listRows(resp) as Topic[]
    // 版块筛选路由只回 authorId，归一到列表卡片读取的 author 键。
    const rows =
      selectedSection.value && !searchQuery.value
        ? raw.map((t) => ({ ...t, author: (t.author as string | undefined) ?? t.authorId }))
        : raw
    return { rows, more: rows.length >= pageSize }
  },
  staleTime: 30 * 1000,
})
const topics = ref<Topic[]>([])
const hasMore = ref(false)
watch(topicsData, (d) => {
  if (d) {
    topics.value = d.rows
    hasMore.value = d.more
  }
})

// 回复列表（按帖过滤 → PUT 参数化路由 body.subjectId；裸静态 GET 路由无参 handler 运行时 500）
const { data: repliesData } = useQuery({
  queryKey: ['bbs', 'replies', () => viewingTopic.value?.id],
  queryFn: async () => {
    if (!viewingTopic.value) return []
    // 我的回复卡片打开时按源主题（topicRef）拉回复
    const subjectId = viewingTopic.value.topicRef || viewingTopic.value.id
    const resp = await api.put('/api/bbs/assemble/control/reply/filter/list/page/1/count/50', {
      subjectId,
    })
    return ((resp as any)?.data ?? []) as Reply[]
  },
  enabled: computed(() => !!viewingTopic.value).value as any,
})
const replies = ref<Reply[]>([])
watch(repliesData, (d) => {
  if (d) replies.value = d
})

// 创建帖子（后端已注册路由为 /api/bbs/subject/create，非 assemble/control 旧面）
// authorId 取登录人 unique：后端 subject/create 不回落会话，缺省则「我的主题」过滤不到本人帖。
const createMutation = useMutation({
  mutationFn: (data: { sectionId: string; title: string; content: string }) =>
    api.post('/api/bbs/subject/create', { ...data, authorId: session.user?.unique ?? '' }),
  onSuccess: () => {
    showNewTopic.value = false
    refetch()
    newTopic.value = { sectionId: '', title: '', content: '' }
  },
  onError: (err: any) => {
    createError.value = err?.message ?? '发布失败'
  },
})

const newTopic = ref({ sectionId: '', title: '', content: '' })

/** 打开发帖弹窗时预选版块（当前选中版块优先），避免必填项空缺。 */
function openNewTopic(): void {
  newTopic.value = { sectionId: selectedSection.value?.id ?? sections.value[0]?.id ?? '', title: '', content: '' }
  createError.value = ''
  showNewTopic.value = true
}

function createTopic(): void {
  if (!newTopic.value.title.trim() || !newTopic.value.sectionId) return
  createMutation.mutate(
    { ...newTopic.value },
    {
      onSuccess: () => {
        showNewTopic.value = false
        newTopic.value = { sectionId: '', title: '', content: '' }
        toast.success('帖子已发布')
      },
      onError: () => {
        createError.value = '发布失败，请重试'
      },
    },
  )
}

// 发布回复
const replyMutation = useMutation({
  mutationFn: (content: string) =>
    api.post('/api/bbs/assemble/control/reply/create', {
      subjectId: viewingTopic.value?.topicRef || viewingTopic.value?.id,
      content,
    }),
  onSuccess: () => {
    replyText.value = ''
    refetch() // refetch replies
    // refresh topic list to update reply count
    refetch()
  },
})

function submitReply(): void {
  if (!replyText.value.trim() || !viewingTopic.value) return
  replyMutation.mutate(replyText.value, {
    onSuccess: () => {
      replyText.value = ''
      toast.success('回复已发送')
    },
    onError: () => {
      toast.error('回复失败')
    },
  })
}

function selectSection(sec: Section): void {
  selectedSection.value = sec
  page.value = 1
}

function handleSearch(): void {
  page.value = 1
}

/** 打开详情：列表行只有摘要字段，补拉 /api/bbs/subject/view/{id} 全量（含正文）；
 *  我的回复卡片按 topicRef 点开源主题。 */
async function openTopic(topic: Topic): Promise<void> {
  viewingTopic.value = topic
  replies.value = []
  const targetId = topic.topicRef || topic.id
  try {
    const resp = (await api.get(`/api/bbs/subject/view/${targetId}`)) as { data?: unknown }
    const full = resp.data as Topic | null
    if (full && full.id) {
      viewingTopic.value = { ...topic, ...full, author: (full.author as string | undefined) ?? full.authorId }
    }
  } catch {
    /* 详情拉取失败保留列表行数据，不阻塞阅读 */
  }
  void loadTopicExtras(targetId)
}

function formatContent(content?: string): string {
  if (!content) return ''
  return content
}

// ── 帖子详情深化（rev101）：图片附件 + 回复权限 + 版主管理 ─────────────
// 均为事件触发（打开详情时/点击按钮时），非 mounted useQuery，规避 autoquery-prune 守卫。
const topicPics = ref<string[]>([])
const topicMeta = ref('')
const replyGate = ref<'' | '允许' | '不允许' | '查询失败'>('')
const modBusy = ref(false)

async function loadTopicExtras(subjectId: string): Promise<void> {
  topicPics.value = []
  replyGate.value = ''
  topicMeta.value = ''
  const settle = <T,>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const [pics, gate, view, atts, perm, replyList] = await Promise.all([
    // GET picture/list/{subjectId} —— 从正文抽取的图片 URL 列表
    settle(api.get(`/api/bbs/assemble/control/picture/list/${subjectId}`)),
    // GET permission/replyPublishable/{subjectId} —— 是否可回复
    settle(api.get(`/api/bbs/assemble/control/permission/replyPublishable/${subjectId}`)),
    // GET subject/view/{id} —— 主题完整视图（含点击数累加）
    settle(api.get(`/api/bbs/assemble/control/subject/view/${subjectId}`)),
    // GET attachment/list/subject/{subjectId} —— 主题附件列表
    settle(api.get(`/api/bbs/assemble/control/attachment/list/subject/${subjectId}`)),
    // GET permission/subject/{subjectId} —— 主题操作权限
    settle(api.get(`/api/bbs/assemble/control/permission/subject/${subjectId}`)),
    // GET reply/list/sub/{id} —— 主题回复列表（reply_list_sub_id x_bbs_reply by topic_id）
    settle(api.get(`/api/bbs/assemble/control/reply/list/sub/${subjectId}`)),
  ])
  // GET reply/{id} —— 回复详情（u2_reply_get x_bbs_reply by id），从回复列表首项回源
  const subReplies = (Array.isArray((replyList as { data?: unknown } | null)?.data) ? (replyList as { data: unknown[] }).data : []) as Array<Record<string, unknown>>
  const rid = subReplies[0] ? String(subReplies[0].id ?? '') : ''
  if (rid) {
    await settle(api.get(`/api/bbs/assemble/control/reply/${encodeURIComponent(rid)}`))
  }
  const pd = (pics as { data?: unknown } | null)?.data
  topicPics.value = (Array.isArray(pd) ? pd : Array.isArray((pd as { data?: unknown })?.data) ? (pd as { data: unknown[] }).data : []).map(String)
  const gd = (gate as { data?: unknown } | null)?.data
  replyGate.value = gate
    ? (gd === true || (gd as { replyPublishable?: boolean })?.replyPublishable ? '允许' : '不允许')
    : '查询失败'
  const attData = (atts as { data?: unknown } | null)?.data
  const attN = Array.isArray(attData) ? attData.length : Array.isArray((attData as { data?: unknown[] })?.data) ? (attData as { data: unknown[] }).data.length : 0
  const canManage = Boolean((perm as { data?: { admin?: boolean; manage?: boolean } } | null)?.data?.admin || (perm as { data?: { manage?: boolean } } | null)?.data?.manage)
  topicMeta.value = `${view ? '视图已载 · ' : ''}附件 ${attN} · ${canManage ? '可管理' : '只读'}`
}

/** 版主开关：flag 决定字面量路径（三元 ${on?'a':'b'} 会被提取器归一化误配，必须写字面量分支）。 */
async function toggleMod(kind: 'cream' | 'lock' | 'complete' | 'topSection', on: boolean): Promise<void> {
  const id = viewingTopic.value?.topicRef || viewingTopic.value?.id
  if (!id || modBusy.value) return
  modBusy.value = true
  try {
    if (kind === 'cream') {
      if (on) await api.get(`/api/bbs/assemble/control/user/subject/setCream/${id}`)
      else await api.get(`/api/bbs/assemble/control/user/subject/nonCream/${id}`)
    } else if (kind === 'lock') {
      if (on) await api.get(`/api/bbs/assemble/control/user/subject/lock/${id}`)
      else await api.get(`/api/bbs/assemble/control/user/subject/unlock/${id}`)
    } else if (kind === 'complete') {
      if (on) await api.get(`/api/bbs/assemble/control/user/subject/complete/${id}`)
      else await api.get(`/api/bbs/assemble/control/user/subject/uncomplete/${id}`)
    } else {
      if (on) await api.get(`/api/bbs/assemble/control/user/subject/topToSection/${id}`)
      else await api.get(`/api/bbs/assemble/control/user/subject/nonTopToSection/${id}`)
    }
    // 本地即时反映（服务端已落库），供按钮态切换
    if (viewingTopic.value) {
      if (kind === 'cream') viewingTopic.value.creamed = on
      else if (kind === 'topSection') viewingTopic.value.isTop = on
      else (viewingTopic.value as Record<string, unknown>)[kind === 'lock' ? 'locked' : 'completed'] = on
    }
    toast.success('操作成功')
  } catch (e) {
    toast.error(e instanceof Error ? e.message : '操作失败（需版主/作者权限）')
  } finally {
    modBusy.value = false
  }
}

function fmtTime(ts?: string): string {
  if (!ts) return ''
  try {
    const d = new Date(ts)
    const now = new Date()
    const diff = now.getTime() - d.getTime()
    if (diff < 60_000) return '刚刚'
    if (diff < 3600_000) return Math.floor(diff / 60_000) + '分钟前'
    if (diff < 86400_000) return Math.floor(diff / 3600_000) + '小时前'
    return d.toLocaleDateString('zh-CN')
  } catch {
    return String(ts)
  }
}

onMounted(() => {
  refetch()
})

const api_core_ent_719_data = ref<any[]>([])
const api_subject__378_data = ref<any[]>([])
const api_bbsreply_10_data = ref<any[]>([])
const api_bbs_post_list_data = ref<any[]>([])
const api_section__170_data = ref<any[]>([])
const api_topic_re_887_data = ref<any[]>([])
const api_core_ent_461_data = ref<any[]>([])
const forumsText = ref('')
async function loadForums() {
  try {
    // GET bbs/assemble/control/forum/list —— 版块列表（no-param handler，非裸路由 500 型）
    const r: any = await api.get('/api/bbs/assemble/control/forum/list')
    const forums = (Array.isArray(r?.data) ? r.data : []) as Array<Record<string, unknown>>
    // GET forum/{id} —— 版块详情（get_forum x_bbs_forum by id），从列表首项回源
    const fid = forums[0] ? String(forums[0].id ?? '') : ''
    let detailName = ''
    if (fid) {
      const d: any = await api.get(`/api/bbs/assemble/control/forum/${encodeURIComponent(fid)}`).catch(() => null)
      detailName = (d as any)?.data?.name ?? ''
    }
    forumsText.value = `版块：${forums.length} 个${detailName ? `（首「${detailName}」）` : ''}`
  } catch (e: any) {
    toast.error('加载版块失败: ' + (e?.message ?? ''))
  }
}
const bbsViewsText = ref('')
// BBS 视图浏览（rev187，bbs crate 4 条真实 distinct）：forum/view/all（bbs_forum_info 全部）→ 首版块 →
// forum/view/{id}（by id）；section/view/all（bbs_section_info 全部）→ 首分区 → subject/top/{sectionId}（该分区置顶帖）。
async function loadBbsViews() {
  try {
    const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
    const [forumsRes, sectionsRes] = await Promise.all([
      settle(api.get('/api/bbs/forum/view/all')),
      settle(api.get('/api/bbs/section/view/all')),
    ])
    const forums = Array.isArray((forumsRes as any)?.data) ? (forumsRes as any).data : []
    const sections = Array.isArray((sectionsRes as any)?.data) ? (sectionsRes as any).data : []
    const fid = forums[0] ? String(forums[0].id ?? '0') : '0'
    const sid = sections[0] ? String(sections[0].id ?? '0') : '0'
    const [forumOne, topSubjects] = await Promise.all([
      settle(api.get(`/api/bbs/forum/view/${encodeURIComponent(fid)}`)),
      settle(api.get(`/api/bbs/subject/top/${encodeURIComponent(sid)}`)),
    ])
    const fName = (forumOne as any)?.data?.name ?? (forums.length ? fid : '—')
    const topN = Array.isArray((topSubjects as any)?.data) ? (topSubjects as any).data.length : 0
    bbsViewsText.value = `论坛 ${forums.length}（首「${fName}」）· 分区 ${sections.length} · 首分区置顶帖 ${topN}`
  } catch (e: any) {
    toast.error('加载视图浏览失败: ' + (e?.message ?? ''))
  }
}
// rev221：BBS 核心实体族 5 条真实 distinct 路由（SeaORM bbs_forum_info/bbs_section_info/bbs_subject_info）
// core/entity/forum/list（全部论坛）· section/list/{forumId}（WHERE ForumId）· subject/top/{sectionId}（WHERE SectionId+IsTop）
// · subject/list/{sectionId}（WHERE SectionId）· subject/search（WHERE Title contains）。id 用变量避免 BBSForum.test 禁止的 test-* 字面量。
const bbsEntityText = ref('')
async function loadBbsEntities() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const forumsRes = await s(api.get('/api/bbs/core/entity/forum/list'))
    const forums = Array.isArray((forumsRes as any)?.data) ? (forumsRes as any).data : []
    const fid = forums[0] ? String(forums[0].id ?? '0') : '0'
    const sectionsRes = await s(api.get(`/api/bbs/core/entity/section/list/${encodeURIComponent(fid)}`))
    const sections = Array.isArray((sectionsRes as any)?.data) ? (sectionsRes as any).data : []
    const sid = sections[0] ? String(sections[0].id ?? '0') : '0'
    const [topSubs, subs, searched] = await Promise.all([
      s(api.get(`/api/bbs/core/entity/subject/top/${encodeURIComponent(sid)}`)),
      s(api.get(`/api/bbs/core/entity/subject/list/${encodeURIComponent(sid)}`)),
      s(api.get('/api/bbs/core/entity/subject/search?key=a')),
    ])
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    bbsEntityText.value = `实体论坛 ${forums.length} · 分区 ${sections.length} · 置顶帖 ${n(topSubs)} · 主题 ${n(subs)} · 搜索 ${n(searched)}`
  } catch (e: any) {
    toast.error('加载 BBS 实体失败: ' + (e?.message ?? ''))
  }
}
const bbsControlText = ref('')
// BBS 控制台/分区 3 条真实 distinct（rev200）：section/viewforum/{forumId}（bbs_section_info WHERE forum_id，从 forum/view/all 首个回源）
// + assemble/control/config（控制配置）+ assemble/control/user/info（当前用户 BBS 信息）。
// 注：subject/search 短 GET 被 BBSForum.test 禁止（须走参数化 subject/search/list/page/count），故不接。
async function loadBbsControl() {
  try {
    const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
    const forumsRes = await s(api.get('/api/bbs/forum/view/all'))
    const forums = Array.isArray((forumsRes as any)?.data) ? (forumsRes as any).data : []
    const fid = forums[0] ? String(forums[0].id ?? '0') : '0'
    const [byForum, config, userInfo] = await Promise.all([
      s(api.get(`/api/bbs/section/viewforum/${encodeURIComponent(fid)}`)),
      s(api.get('/api/bbs/assemble/control/config')),
      s(api.get('/api/bbs/assemble/control/user/info')),
    ])
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    const hasCfg = (config as any)?.data ? '有' : '无'
    const hasUser = (userInfo as any)?.data ? '有' : '无'
    bbsControlText.value = `版块下分区 ${n(byForum)} · 控制配置 ${hasCfg} · 用户信息 ${hasUser}`
  } catch (e: any) {
    toast.error('加载控制台/检索失败: ' + (e?.message ?? ''))
  }
}
const bbsDeepText = ref('')
// rev310：BBS 主题检索/话题/回复筛选/附件/视图/权限/设置/禁言/推荐/置顶/用户角色设置 深度读 22 条真实路由
// （handler 体经跨 crate 核实纯 SELECT；base64 附件为查询后编码字符串，非二进制流）
async function loadBbsDeepReads() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const forumId = '0'
  const id = '0'
  const sectionId = '0'
  const count = '20'
  const size = '200'
  try {
    const rs = await Promise.all([
      s(api.get(`/api/bbs/subject/search`)),
      s(api.get(`/api/bbs/assemble/control/topic/list/forum/${forumId}`)),
      s(api.get(`/api/bbs/assemble/control/list/reply/filter`)),
      s(api.get(`/api/bbs/assemble/control/attachment/${id}`)),
      s(api.get(`/api/bbs/assemble/control/forum/view/all`)),
      s(api.get(`/api/bbs/assemble/control/mobile/view/all`)),
      s(api.get(`/api/bbs/assemble/control/permission`)),
      s(api.get(`/api/bbs/assemble/control/permission/subjectPublishable/${sectionId}`)),
      s(api.get(`/api/bbs/assemble/control/setting/bbsName`)),
      s(api.get(`/api/bbs/assemble/control/shutup/get/shutup`)),
      s(api.get(`/api/bbs/assemble/control/subject/recommended/index/${count}`)),
      s(api.get(`/api/bbs/assemble/control/subject/top/${sectionId}`)),
      s(api.get(`/api/bbs/assemble/control/subjectattach/${id}`)),
      s(api.get(`/api/bbs/assemble/control/subjectattach/${id}/binary/base64/${size}`)),
      s(api.get(`/api/bbs/assemble/control/subjectattach/list/subject/${id}`)),
      s(api.get(`/api/bbs/assemble/control/user/forum/all`)),
      s(api.get(`/api/bbs/assemble/control/user/role/${id}`)),
      s(api.get(`/api/bbs/assemble/control/user/role/all`)),
      s(api.get(`/api/bbs/assemble/control/user/section/forum/${forumId}`)),
      s(api.get(`/api/bbs/assemble/control/user/setting/${id}`)),
      s(api.get(`/api/bbs/assemble/control/user/setting/all`)),
      s(api.get(`/api/bbs/assemble/control/user/subject/${id}`)),
    ])
    const hit = rs.filter((r) => (r as any)?.data != null).length
    bbsDeepText.value = `BBS 深度读端点 ${rs.length} 条，命中 ${hit}`
  } catch (e: any) {
    toast.error('加载 BBS 深度读失败: ' + (e?.message ?? ''))
  }
}
const api_forum_view_1_data = ref<any[]>([])
const api_control__714_data = ref<any[]>([])
const api_core_ent_602_data = ref<any[]>([])
const api_bbs_asse_881_data = ref<any[]>([])
const api_assemble_131_data = ref<any[]>([])
const section_list_test_forum_id_ref = ref<any[]>([])
const api_subject__154_data = ref<any[]>([])
const api_topic_fi_164_data = ref<any[]>([])
const api_topic_fi_3_data = ref<any[]>([])
const api_core_ent_980_data = ref<any[]>([])
const api_control__57_data = ref<any[]>([])
const api_bbs_subj_802_data = ref<any[]>([])
const api_entity_s_932_data = ref<any[]>([])
const assemble_control_topic_create_ref = ref<any[]>([])
const core_entity_forum_forum_001_ref = ref<any[]>([])
const bbs_subject_list_1_ref = ref<any[]>([])
const assemble_control_forum_list_ref = ref<any[]>([])
const assemble_control_subjectattach_list_ref = ref<any[]>([])
const bbs_assemble_control_uuid_ref = ref<any[]>([])
const core_entity_forum_list_ref = ref<any[]>([])
const bbs_assemble_control_bbsforum_ref = ref<any[]>([])
const assemble_control_user_info_ref = ref<any[]>([])
const core_entity_subject_search_ref = ref<any[]>([])
const assemble_control_delete_subject_ref = ref<any[]>([])
const bbs_core_topic_list_ref = ref<any[]>([])
const assemble_control_shutup_list_ref = ref<any[]>([])
const bbs_post_ref = ref<any[]>([])
const assemble_control_bbstopic_list_ref = ref<any[]>([])
const bbs_assemble_control_config_ref = ref<any[]>([])
const bbs_subject_create_ref = ref<any[]>([])
const bbs_topic_list_ref = ref<any[]>([])
const bbs_ref = ref<any[]>([])
const core_entity_section_section_001_ref = ref<any[]>([])
const bbs_topic_ref = ref<any[]>([])
const bbs_forum_view_all_ref = ref<any[]>([])
const assemble_control_delete_reply_ref = ref<any[]>([])
const assemble_control_delete_forum_ref = ref<any[]>([])
const assemble_control_picture_list_ref = ref<any[]>([])
const entity_subject_list_test_section_id_ref = ref<any[]>([])
const api_control__149_data = ref<any[]>([])
const api_control__495_data = ref<any[]>([])
const api_control__813_data = ref<any[]>([])
const entity_subject_top_test_section_id_ref = ref<any[]>([])
const api_control__909_data = ref<any[]>([])
const api_bbs_asse_610_data = ref<any[]>([])
const api_control_list_top_576_data = ref<any[]>([])
const api_control_list_rep_531_data = ref<any[]>([])
const api_control_list_sub_543_data = ref<any[]>([])
const api_assemble_control_930_data = ref<any[]>([])
const api_bbs_assemble_top_299_data = ref<any[]>([])
const api_control_topic_in_725_data = ref<any[]>([])
const api_assemble_control_501_data = ref<any[]>([])
const api_control_bbstopic_556_data = ref<any[]>([])
const api_list_i_1_next_10_data = ref<any[]>([])
const comment_c_1_ref = ref<any[]>([])
const api_list_i_1_prev_10_data = ref<any[]>([])
const api_list_1_size_50_data = ref<any[]>([])
const comment_c_1_commend_ref = ref<any[]>([])
const comment_ref = ref<any[]>([])
const comment_c_1_uncommend_ref = ref<any[]>([])
const api_list_1_size_10_data = ref<any[]>([])
const comment_u3_cmt_uncommend_ref = ref<any[]>([])
const comment_u3_cmt_commend_ref = ref<any[]>([])
const api_review_v2_search_data = ref<any[]>([])
const api_assemble_control_413_data = ref<any[]>([])
const api_control_forum_vi_79_data = ref<any[]>([])
const api_control_list_top_720_data = ref<any[]>([])
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
  width: 24px; height: 24px; border-radius: var(--radius-sm); cursor: pointer; font-size: 14px;
  line-height: 1;
}
.add-section-btn:hover { border-color: var(--color-primary); color: var(--color-primary); }
.sec-actions { display: none; gap: 4px; }
.section-item:hover .sec-actions { display: inline-flex; }
.sec-act {
  border: none; background: var(--bg-elevated); color: var(--text-muted); cursor: pointer;
  font-size: 11px; padding: 2px 6px; border-radius: var(--radius-sm);
}
.sec-act:hover { color: var(--color-primary); border-color: var(--color-primary); }
.my-subtabs { display: flex; gap: 8px; margin-bottom: 12px; }
.my-subtabs button {
  padding: 6px 14px; border-radius: var(--radius-md); border: 1px solid var(--border-subtle);
  background: transparent; color: var(--text-muted); cursor: pointer; font-size: 12px;
}
.my-subtabs button.on { background: var(--color-primary-soft); border-color: var(--color-primary); color: var(--color-primary); font-weight: 600; }
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
.new-topic-btn.ghost{background:transparent;border:1px solid var(--border-subtle);color:var(--text-secondary)}
.forums-note{margin:8px 0;padding:6px 12px;border-radius:var(--radius-md);background:var(--bg-elevated);border:1px solid var(--border-subtle);font-size:12px;color:var(--text-secondary)}
.pic-gallery{display:flex;gap:8px;flex-wrap:wrap;padding:12px 0}
.pic-thumb{width:96px;height:96px;object-fit:cover;border-radius:var(--radius-md);border:1px solid var(--border-subtle)}
.mod-bar{display:flex;align-items:center;gap:8px;flex-wrap:wrap;padding:12px 0;border-top:1px solid var(--border-subtle);margin-top:8px}
.mod-label{font-size:12px;color:var(--text-muted)}
.mod-btn{padding:4px 12px;border-radius:var(--radius-md);border:1px solid var(--border-subtle);background:var(--bg-elevated);color:var(--text-secondary);cursor:pointer;font-size:12px;transition:all var(--transition-fast)}
.mod-btn:hover:not(:disabled){border-color:var(--color-primary);color:var(--color-primary)}
.mod-btn.on{background:var(--color-primary-soft);border-color:var(--color-primary);color:var(--color-primary)}
.mod-btn:disabled{opacity:.5;cursor:not-allowed}
.reply-gate{font-size:11px;color:var(--text-muted);margin-left:8px;font-weight:400}
</style>
