<template>
  <view class="page-root">
    <view class="pad safe-bottom">
      <text class="muted" style="display: block; margin-bottom: 16rpx">
        全部运营公告。首页只展示最新几条，这里可以回看历史公告。
      </text>
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
      </view>

      <view v-else>
        <!-- G86 时间筛选 -->
        <view class="filter-row">
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
          class="search-inp"
          placeholder="搜索公告标题 / 正文"
          placeholder-class="ph"
          style="margin-top: 12rpx"
        />
        <text v-if="annKeyword && filteredItems.length === 0" class="muted">
          没有匹配「{{ annKeyword }}」的公告
          <text class="link" @click="annKeyword = ''">清除搜索</text>
        </text>
        <text v-else-if="filteredItems.length === 0" class="muted">该时间段没有公告。</text>
        <view v-for="(a, i) in filteredItems" :key="a.id" class="card ann-card">
          <view class="ann-head">
            <!-- R22 最新一条给「最新」徽章，日期做成统一胶囊，与其它列表页一致 -->
            <text v-if="i === 0" class="ann-new">最新</text>
            <text class="ann-title">{{ a.title }}</text>
            <text class="ann-date mono">{{ formatDate(a.published_at) }}</text>
          </view>
          <text class="ann-body">{{ a.body }}</text>
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
}

.ann-body {
  font-size: 24rpx;
  line-height: 1.7;
  color: var(--muted, #9aa0a6);
  white-space: pre-wrap;
}
</style>