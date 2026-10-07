<template>
  <view class="page-root">
    <view class="pad">
      <view class="tabs">
        <view
          v-for="t in tabs"
          :key="t.key"
          class="tab"
          :class="{ active: curType === t.key }"
          @click="switchTab(t.key)"
        >
          {{ t.label }}
        </view>
      </view>
      <text class="muted" style="display: block; margin-bottom: 16rpx">{{ curDesc }}</text>

      <view v-if="loading" class="card"><text class="muted">加载中…</text></view>
      <view v-else-if="items.length === 0" class="card"><text class="muted">暂无数据</text></view>

      <view v-else class="card" style="padding: 8rpx 24rpx">
        <view v-for="(it, i) in items" :key="it.user_id" class="rank-row" :class="{ me: it.user_id === store.userId }">
          <text class="rank-no mono" :class="'no-' + (i + 1)">{{ i + 1 }}</text>
          <text class="rank-name flex-1">{{ it.nickname || '玩家' }}</text>
          <text class="rank-score mono">{{ formatScore(it.score) }}</text>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useGameStore } from '@/store/game'
import * as api from '@/api/client'

const store = useGameStore()
const curType = ref('power')
const items = ref<any[]>([])
const loading = ref(false)
// 第 136 轮：请求序号（模块级 let，非 ref —— 无需响应式）。
let loadSeq = 0

const tabs = [
  { key: 'power', label: '战力榜' },
  { key: 'stage', label: '关卡榜' },
  { key: 'efficiency', label: '效率榜' },
]

const curDesc = computed(() => {
  switch (curType.value) {
    case 'stage':
      return '按最高通关关卡排名。'
    case 'efficiency':
      return '按通关时使用的最低战力排名 —— 低养成高技巧的玩家在这里有位置，不必和氪金榜挤在一起。'
    default:
      return '按累计成长值排名。'
  }
})

function formatScore(v: number): string {
  if (curType.value === 'efficiency') return String(v)
  return String(v)
}

async function load() {
  // 第 136 轮：记录本次请求发起时的 tab + 自增序号。
  // 若期间用户又切了 tab（发起更新的请求），本次的响应就是「陈旧」的，
  // 一律丢弃，避免旧的 fetchLeaderboard 后到、把新 tab 的列表覆盖掉。
  const seq = ++loadSeq
  const type = curType.value
  loading.value = true
  try {
    const res = await api.fetchLeaderboard(type)
    if (seq !== loadSeq) return // 已有更新的请求，丢弃本陈旧响应
    items.value = res.items ?? []
  } catch (e) {
    if (seq !== loadSeq) return
    uni.showToast({ title: (e as Error).message, icon: 'none' })
    items.value = []
  } finally {
    if (seq === loadSeq) loading.value = false
  }
}

function switchTab(key: string) {
  curType.value = key
  load()
}

onMounted(async () => {
  if (!store.loggedIn) await store.login()
  await load()
})
</script>

<style scoped>
.tabs {
  display: flex;
  flex-direction: row;
  gap: 12rpx;
  margin-bottom: 16rpx;
}

.tab {
  flex: 1;
  height: 68rpx;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 12rpx;
  background: var(--panel);
  border: 1rpx solid var(--border);
  font-size: 26rpx;
  color: var(--muted);
}

.tab.active {
  background: rgba(255, 107, 53, 0.14);
  border-color: var(--fire);
  color: var(--fire);
}

.rank-row {
  display: flex;
  flex-direction: row;
  align-items: center;
  padding: 22rpx 0;
  border-bottom: 1rpx solid var(--border);
}

.rank-row:last-child {
  border-bottom: none;
}

.rank-row.me {
  background: rgba(255, 107, 53, 0.08);
}

.rank-no {
  width: 60rpx;
  font-size: 28rpx;
  color: var(--muted);
  font-weight: 700;
}

.rank-no.no-1 { color: #ffd700; }
.rank-no.no-2 { color: #c0c0c0; }
.rank-no.no-3 { color: #cd7f32; }

.rank-name {
  font-size: 26rpx;
}

.rank-score {
  font-size: 26rpx;
  color: var(--ok);
  font-weight: 600;
}
</style>
