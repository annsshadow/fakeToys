<template>
  <view class="page-root">
    <view class="pad safe-bottom">
      <text class="muted" style="display: block; margin-bottom: 16rpx">
        全部运营公告。首页只展示最新几条，这里可以回看历史公告。
      </text>

      <view v-if="loading" class="card"><text class="muted">加载中…</text></view>

      <view v-else-if="items.length === 0" class="card">
        <text class="card-title">暂无公告</text>
        <text class="muted" style="display: block; margin-top: 8rpx">
          新服或活动前公告为空是正常的。
        </text>
      </view>

      <view v-else>
        <view v-for="a in items" :key="a.id" class="card ann-card">
          <view class="ann-head">
            <text class="ann-title">{{ a.title }}</text>
            <text class="dim mono">{{ formatDate(a.published_at) }}</text>
          </view>
          <text class="ann-body">{{ a.body }}</text>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import * as api from '@/api/client'
import type { Announcement } from '@/api/client'

const items = ref<Announcement[]>([])
const loading = ref(false)

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
  try {
    // 服务端上限 20 条（玩家侧公网端点），传 20 即全量。
    const res = await api.fetchAnnouncements(20)
    items.value = res.items ?? []
  } catch {
    // 公告拉取失败：展示空态，不阻塞页面
  } finally {
    loading.value = false
  }
})
</script>

<style scoped>
.ann-card {
  margin-bottom: 16rpx;
}

.ann-head {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  align-items: center;
  gap: 16rpx;
  margin-bottom: 8rpx;
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