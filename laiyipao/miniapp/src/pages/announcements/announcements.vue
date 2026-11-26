<template>
  <view class="page-root">
    <view class="pad safe-bottom">
      <text class="muted" style="display: block; margin-bottom: 16rpx">
        全部运营公告。首页只展示最新几条，这里可以回看历史公告。
      </text>
      <!-- R5：公告与版本眉题 + 摘要指标带 + 规则引擎结论 -->
      <view class="section-kicker r5-section r5-enter">
        <text class="section-kicker-cn">公告与版本</text>
        <text class="section-kicker-en">RELEASE NOTES · RULE ENGINE</text>
        <text class="r3-ai-badge">AI</text>
      </view>
      <view v-if="!loading && !loadFailed && items.length" class="r3-kpi r5-enter">
        <view class="r3-kpi-main">
          <text class="r3-kpi-num">{{ items.length }}</text>
          <text class="r3-kpi-label">公告总数</text>
          <text class="r3-kpi-sub">近 7 日新增 {{ r5AnnRecent7 }} 条 · 未读 0（列表无未读字段）</text>
        </view>
        <view class="r3-kpi-cell">
          <text class="r3-kpi-num">{{ groupedByMonth.length }}</text>
          <text class="r3-kpi-label">归档月份</text>
        </view>
        <view class="r3-kpi-cell">
          <text class="r3-kpi-num">{{ r5AnnRecent7 }}</text>
          <text class="r3-kpi-label">近 7 日新增</text>
        </view>
      </view>
      <view v-if="r5AiAnn.visible" class="r3-ai r5-enter">
        <view class="r3-ai-head">
          <text class="r3-ai-badge">AI</text>
          <text class="r3-ai-title">AI 公告摘要</text>
          <text class="r3-ai-engine">规则引擎</text>
        </view>
        <text class="r3-ai-conclusion">{{ r5AiAnn.conclusion }}</text>
        <text class="r3-ai-basis">依据：公告列表 · 近 30 天 · 更新于本次进入</text>
        <text class="r3-ai-cta" role="button" @click="r5GoBattles()" tabindex="0" @keydown.enter="r5GoBattles()">去看变更影响 →</text>
      </view>
      <!-- 第六批 R41：总数一眼可见（首页入口有「全部 N 条」，本页此前没有） -->
      <text v-if="!loading && !loadFailed && items.length" class="result-count">
        共 {{ items.length }} 条公告
      </text>

      <view v-if="loading" class="card">
        <text class="muted">加载中…</text>
        <view v-for="n in 3" :key="n" class="skeleton-row">
          <view class="skeleton-bar" style="width: 55%" />
          <view class="skeleton-bar" style="width: 18%; margin-left: auto" />
        </view>
      </view>

      <!-- 第六批 R42：拉取失败与「确实没公告」此前同一副空态，
           失败时说「新服公告为空是正常的」是假话 —— 分开表述。 -->
      <view v-else-if="loadFailed" class="card empty-block">
        <text class="empty-glyph">📡</text>
        <text class="card-title">公告加载失败</text>
        <text class="muted" style="display: block; margin-top: 8rpx">
          网络或服务器暂时不可用，稍后再来重试。
        </text>
      </view>

      <view v-else-if="items.length === 0" class="card empty-block">
        <text class="empty-glyph">📢</text>
        <text class="card-title">暂无公告</text>
        <text class="muted" style="display: block; margin-top: 8rpx">
          新服或活动前公告为空是正常的。
        </text>
        <view class="r3-empty" aria-live="polite" role="status">
          <view class="r3-empty-slot">AI</view>
          <text class="r3-empty-title">近 30 天没有公告，去看看战报</text>
          <text class="r3-empty-reason">公告列表为空或均超出 30 天，无法从版本记录推导更新影响。</text>
          <text class="r3-empty-action" role="button" @click="r5GoBattles()" tabindex="0" @keydown.enter="r5GoBattles()">去看战报 →</text>
        </view>
      </view>

      <view v-else>
        <!-- G86 时间筛选 -->
        <view class="filter-row r3-filter">
          <view
            v-for="f in annFilters"
            :key="f.key"
            class="filter-chip"
            :class="{ active: annFilter === f.key }"
            @click="annFilter = f.key"
          >{{ f.label }}</view>
        </view>
        <!-- G112 公告关键词搜索 -->
        <input
          v-model="annKeyword"
          v-aria-native="'搜索公告标题 / 正文'"
          aria-label="搜索公告标题 / 正文"
          class="search-inp"
          placeholder="搜索公告标题 / 正文"
          placeholder-class="ph"
          style="margin-top: 12rpx"
        />
        <text v-if="annKeyword && filteredItems.length === 0" class="muted search-empty-text">
          没有匹配「{{ annKeyword }}」的公告
          <text class="link" @click="annKeyword = ''">清除搜索</text>
        </text>
        <text v-else-if="filteredItems.length === 0" class="muted">该时间段没有公告。</text>
        <!-- G131 按月份分组归档 -->
        <view v-for="grp in groupedByMonth" :key="grp[0]" class="month-group r3-data-grid">
          <text class="month-label">{{ grp[0] }}</text>
          <view v-for="(a, i) in grp[1]" :key="a.id" class="card ann-card">
            <view class="ann-head">
              <text v-if="i === 0" class="ann-new" :class="{ hidden: !isFirstGroup(grp[0]) }">最新</text>
              <text class="ann-title">{{ a.title }}</text>
              <text class="ann-date mono">{{ formatDate(a.published_at) }}</text>
            </view>
            <text class="ann-body">{{ a.body }}</text>
          </view>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import * as api from '@/api/client'
import type { Announcement } from '@/api/client'

const items = ref<Announcement[]>([])
const loading = ref(false)
/** 第六批 R42：区分「拉取失败」与「确实没公告」。 */
const loadFailed = ref(false)

/* -------------------------------------------------------------------------- */
/* G86 公告时间筛选（第七批）                                                   */
/* 按 published_at 过滤最近 7 天 / 30 天 / 全部，帮玩家在长公告里快速定位新内容。   */
/* -------------------------------------------------------------------------- */
const annFilters = [
  { key: 'all', label: '全部' },
  { key: '7d', label: '近 7 天' },
  { key: '30d', label: '近 30 天' },
] as const
type AnnFilter = (typeof annFilters)[number]['key']
const annFilter = ref<AnnFilter>('all')
/** G112 关键词搜索：在时间筛选基础上叠加标题/正文关键词（数据来自 items）。 */
/** G131 公告按月份分组：把卡片列表变成有脉络的归档（数据来自 filteredItems）。 */
/** 是否最早分组（用于「最新」徽章只标全局第一条）。 */
function isFirstGroup(key: string): boolean {
  return groupedByMonth.value[0]?.[0] === key
}

const groupedByMonth = computed(() => {
  const m = new Map<string, typeof items.value>()
  for (const a of filteredItems.value) {
    const key = monthKey(a.published_at)
    const arr = m.get(key) ?? []
    arr.push(a)
    m.set(key, arr)
  }
  return [...m.entries()].sort((a, b) => b[0].localeCompare(a[0]))
})

const annKeyword = ref('')
const filteredItems = computed(() => {
  const kw = annKeyword.value.trim().toLowerCase()
  let list = items.value
  if (annFilter.value !== 'all') {
    const days = annFilter.value === '7d' ? 7 : 30
    const cutoff = Date.now() - days * 86400000
    list = list.filter((a) => {
      const t = a.published_at ? new Date(a.published_at).getTime() : 0
      return t >= cutoff
    })
  }
  if (!kw) return list
  return list.filter((a) => a.title.toLowerCase().includes(kw) || a.body.toLowerCase().includes(kw))
})

/**
 * 归档分组键必须和 formatDate 用同一套（本地）日历。
 * 服务端 published_at 是 UTC 的 RFC3339 字符串，直接 slice(0,7) 取到的
 * 是 UTC 年月；UTC 月末在本地已是下月初时，组标题会和卡片日期自相矛盾。
 */
function monthKey(iso: string | null): string {
  if (!iso) return '未标注日期'
  const d = new Date(iso)
  if (Number.isNaN(d.getTime())) return iso.slice(0, 7) || '未标注日期'
  const y = d.getFullYear()
  const mm = String(d.getMonth() + 1).padStart(2, '0')
  return `${y}-${mm}`
}

function formatDate(iso: string | null): string {
  if (!iso) return ''
  const d = new Date(iso)
  if (Number.isNaN(d.getTime())) return iso
  const y = d.getFullYear()
  const mm = String(d.getMonth() + 1).padStart(2, '0')
  const dd = String(d.getDate()).padStart(2, '0')
  const hh = String(d.getHours()).padStart(2, '0')
  const mi = String(d.getMinutes()).padStart(2, '0')
  return `${y}-${mm}-${dd} ${hh}:${mi}`
}

/** R5：近 7 日公告数与摘要，全部来自已拉取的 items / groupedByMonth。 */
const r5AnnRecent7 = computed(() => {
  const cutoff = Date.now() - 7 * 86400000
  return items.value.filter((a) => {
    const t = a.published_at ? new Date(a.published_at).getTime() : 0
    return t >= cutoff
  }).length
})
const r5AiAnn = computed(() => {
  const latest = items.value[0]
  if (!latest) return { visible: false, conclusion: '' }
  const monthGroup = groupedByMonth.value[0]
  const monthCount = monthGroup ? monthGroup[1].length : 0
  const title = latest.title.length > 18 ? latest.title.slice(0, 18) + '…' : latest.title
  return { visible: true, conclusion: `本月更新 ${monthCount} 条，最新：${title}` }
})
function r5GoBattles(): void {
  uni.navigateTo({ url: '/pages/battles/battles' })
}

onMounted(async () => {
  loading.value = true
  loadFailed.value = false
  try {
    // 服务端上限 20 条（玩家侧公网端点），传 20 即全量。
    const res = await api.fetchAnnouncements(20)
    items.value = res.items ?? []
  } catch {
    // R42：失败要如实显示，不能再说「新服公告为空是正常的」
    loadFailed.value = true
  } finally {
    loading.value = false
  }
})
</script>

<style scoped>
/* G131 月份分组 */
.month-group { margin-bottom: 16rpx; }
.month-label { display: block; font-size: 24rpx; color: var(--accent); font-weight: 600; margin: 12rpx 0 8rpx; padding-left: 8rpx; border-left: 4rpx solid var(--accent); }
.ann-new.hidden { display: none; }

.ann-card {
  margin-bottom: 16rpx;
  /* R22 左侧强调条：与其它列表页的卡片视觉语言对齐 */
  border-left: 6rpx solid var(--fire);
}

.ann-head {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  align-items: center;
  gap: 16rpx;
  margin-bottom: 8rpx;
}

.ann-new {
  flex-shrink: 0;
  font-size: 20rpx;
  padding: 4rpx 12rpx;
  border-radius: 999rpx;
  color: var(--fire);
  background: var(--fire-soft);
}

.ann-date {
  flex-shrink: 0;
  font-size: 20rpx;
  padding: 4rpx 12rpx;
  border-radius: 999rpx;
  color: var(--muted);
  background: var(--panel-2);
}

.ann-title {
  font-size: 28rpx;
  font-weight: 600;
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.ann-body {
  font-size: 24rpx;
  line-height: 1.7;
  color: var(--muted);
  white-space: pre-wrap;
}
</style>