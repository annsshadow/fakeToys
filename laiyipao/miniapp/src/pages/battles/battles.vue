<template>
  <view class="page-root">
    <view class="pad safe-bottom">
      <text class="muted" style="display: block; margin-bottom: 16rpx">
        最近战斗的记录。点开一行可以回顾单局掉落与元素/反应/地形使用。
      </text>

      <!-- 结果筛选（2026-11-01 新一轮第 74 轮） -->
      <view class="filter-row">
        <view
          v-for="f in filters"
          :key="f.key"
          class="filter-chip"
          :class="{ active: filter === f.key }"
          @click="filter = f.key"
        >
          {{ f.label }}
        </view>
      </view>

      <view v-if="loading" class="card"><text class="muted">加载中…</text></view>

      <view v-else-if="items.length === 0" class="card">
        <text class="card-title">暂无战报</text>
        <text class="muted" style="display: block; margin-top: 8rpx">
          打一局战斗后，胜负、星级、分数和击杀都会出现在这里。
        </text>
      </view>

      <view v-else class="card" style="padding: 8rpx 24rpx">
        <view
          v-for="it in items"
          :key="it.id"
          class="battle-row"
          :class="{ expanded: expandedId === it.id }"
          @click="toggleDetail(it)"
        >
          <view class="battle-head">
            <text class="mono" :style="{ color: it.result === 'win' ? 'var(--ok)' : 'var(--bad)' }">
              {{ it.result === 'win' ? '胜' : '负' }}
            </text>
            <text class="battle-stars" :class="{ dim: it.stars === 0 }">
              {{ '★'.repeat(it.stars) || '☆☆☆' }}
            </text>
            <text class="flex-1 ellipsis">第 {{ it.level_id }} 关</text>
            <text class="mono battle-score">{{ it.score }}</text>
          </view>
          <view class="battle-meta">
            <text class="dim">击杀 {{ it.kills }} · 漏怪 {{ it.leaked }}</text>
            <text class="dim">{{ fmtTime(it.created_at) }}</text>
          </view>

          <!-- 详情（2026-11-01 新一轮第 41 轮）：展开时按需拉取单局完整回顾 -->
          <view v-if="expandedId === it.id" class="battle-detail">
            <view v-if="detailLoading" class="dim" style="padding: 8rpx 0">加载中…</view>
            <template v-else-if="detail">
              <view class="kv">
                <text class="muted">发射 / 命中</text>
                <text class="mono">{{ detail.shots }} / {{ detail.hits }}</text>
              </view>
              <view class="kv"><text class="muted">波次</text><text class="mono">{{ detail.wave_reached }}</text></view>
              <view class="kv"><text class="muted">峰值热量</text><text class="mono">{{ detail.heat_max }}</text></view>
              <view class="kv"><text class="muted">剩余血量</text><text class="mono">{{ detail.hp_left }}</text></view>
              <view class="kv">
                <text class="muted">掉落</text>
                <text class="mono" style="color: var(--ok)">{{ lootText(detail.loot) || '—' }}</text>
              </view>
              <view v-if="elementText" class="kv"><text class="muted">元素</text><text class="flex-1">{{ elementText }}</text></view>
              <view v-if="reactionText" class="kv"><text class="muted">反应</text><text class="flex-1">{{ reactionText }}</text></view>
              <view v-if="terrainText" class="kv"><text class="muted">地形</text><text class="flex-1">{{ terrainText }}</text></view>
              <view class="kv"><text class="muted">回放哈希</text><text class="mono hash">{{ detail.replay_hash }}</text></view>
              <!-- 直达验真（2026-11-01 新一轮第 47 轮）：验真页已支持 battle_id 深链，
                   这里一键跳转，玩家无需手动复制/输入 battle_id。 -->
              <view class="btn verify-btn" @click="goVerify(it.id)">去验真此局</view>
            </template>
          </view>
        </view>
      </view>

      <!-- 加载更早战报（2026-11-01 新一轮第 55 轮） -->
      <view v-if="hasMore" class="load-more" @click="loadMore">
        {{ loadingMore ? '加载中…' : '加载更多' }}
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { onLoad } from '@dcloudio/uni-app'
import { useGameStore } from '@/store/game'
import * as api from '@/api/client'
import type { BattleHistoryItem, BattleDetail } from '@/api/client'
import { ELEMENT_NAME } from '@/game/elements'
import { TERRAIN_NAME } from '@/game/terrain'

const store = useGameStore()

const items = ref<BattleHistoryItem[]>([])
const loading = ref(false)
const loadingMore = ref(false)
/** 是否还有更早的战报（上次返回满一页 20 条即视为还有更多）。 */
const hasMore = ref(false)
const expandedId = ref(0)

/**
 * 结果筛选（2026-11-01 新一轮第 74 轮）：全部 / 胜 / 负。
 * 筛选作用于已加载的列表（最近 N 局内过滤）。
 */
const filters = [
  { key: 'all', label: '全部' },
  { key: 'win', label: '胜利' },
  { key: 'lose', label: '失败' },
] as const
type BattleFilter = (typeof filters)[number]['key']
const filter = ref<BattleFilter>('all')

/**
 * 结果筛选（2026-11-01 新一轮第 80 轮）：切换时**重新从第一页加载**，
 * 服务端按 result 过滤（配合分页，筛选对全部历史生效，而非仅已加载部分）。
 */
watch(filter, () => {
  void reload()
})

async function reload(): Promise<void> {
  loading.value = true
  try {
    const res = await api.myBattleHistory(20, 0, filter.value === 'all' ? '' : filter.value)
    items.value = res.items ?? []
    hasMore.value = (res.items?.length ?? 0) >= 20
    expandedId.value = 0
    // 深链（2026-11-01 新一轮第 43 轮）：结算页跳转过来时自动展开刚打的那一局
    if (focusBattleId.value > 0) {
      const target = items.value.find((it) => it.id === focusBattleId.value)
      if (target) {
        expandedId.value = target.id
        void loadDetail(target)
      }
    }
  } catch {
    // 战报加载失败：展示空态，不阻塞页面
  } finally {
    loading.value = false
  }
}
const details = ref<Record<number, BattleDetail>>({})
const detailLoading = ref(false)

/** 深链目标战报 id（2026-11-01 新一轮第 43 轮）：结算页跳转时携带。 */
const focusBattleId = ref(0)

onLoad((query) => {
  const id = Number(query?.battle_id ?? 0)
  if (Number.isInteger(id) && id > 0) focusBattleId.value = id
})

/** 当前展开行的详情（未拉取时为 undefined）。 */
const detail = computed<BattleDetail | undefined>(() =>
  expandedId.value ? details.value[expandedId.value] : undefined,
)

const CURRENCY_NAME: Record<string, string> = {
  coin: '金币',
  gem: '钻石',
  energy: '体力',
  keys: '钥匙',
}

function currencyName(k: string): string {
  return CURRENCY_NAME[k] ?? k
}

function lootText(loot: Record<string, number> | undefined): string {
  if (!loot) return ''
  const entries = Object.entries(loot)
  if (!entries.length) return ''
  return entries.map(([k, v]) => `${v} ${currencyName(k)}`).join(' + ')
}

/** 反应名以服务端 /config.reactions 为准（与图鉴页同源）。 */
const reactionNameMap = computed(() => {
  const m = new Map<string, string>()
  for (const r of store.config?.reactions ?? []) m.set(r.key, r.name)
  return m
})

const elementText = computed(() => {
  const d = detail.value
  if (!d) return ''
  return Object.entries(d.elements_used)
    .map(([k, v]) => `${ELEMENT_NAME[k as keyof typeof ELEMENT_NAME] ?? k} ×${v}`)
    .join('、')
})

const reactionText = computed(() => {
  const d = detail.value
  if (!d) return ''
  return Object.entries(d.reactions_used)
    .map(([k, v]) => `${reactionNameMap.value.get(k) ?? k} ×${v}`)
    .join('、')
})

const terrainText = computed(() => {
  const d = detail.value
  if (!d) return ''
  return (d.terrain_used ?? [])
    .map((k) => TERRAIN_NAME[k as keyof typeof TERRAIN_NAME] ?? k)
    .join('、')
})

function fmtTime(iso: string): string {
  const d = new Date(iso)
  if (Number.isNaN(d.getTime())) return iso
  const mm = String(d.getMonth() + 1).padStart(2, '0')
  const dd = String(d.getDate()).padStart(2, '0')
  const hh = String(d.getHours()).padStart(2, '0')
  const mi = String(d.getMinutes()).padStart(2, '0')
  return `${mm}/${dd} ${hh}:${mi}`
}

/**
 * 展开/收起战报详情。详情按需拉取并缓存：同一场只请求一次。
 */
async function toggleDetail(it: BattleHistoryItem) {
  if (expandedId.value === it.id) {
    expandedId.value = 0
    return
  }
  expandedId.value = it.id
  await loadDetail(it)
}

/** 拉取单局详情（已缓存则直接返回）。 */
async function loadDetail(it: BattleHistoryItem): Promise<void> {
  if (details.value[it.id]) return
  detailLoading.value = true
  try {
    details.value[it.id] = await api.myBattleDetail(it.id)
  } catch {
    // 详情加载失败：保留展开态，可再次点击重试
  } finally {
    detailLoading.value = false
  }
}

/**
 * 直达验真（2026-11-01 新一轮第 47 轮）。
 * 验真页支持 `?battle_id=` 深链并自动拉取复现信息，
 * 玩家从战报详情一键跳转即可验真，无需手动复制哈希。
 */
function goVerify(battleId: number) {
  uni.navigateTo({ url: `/pages/verify/verify?battle_id=${battleId}` })
}

/**
 * 加载更早的战报（2026-11-01 新一轮第 55 轮）。
 * 服务端按时间倒序分页（limit=20/offset），追加到列表并去重。
 */
async function loadMore() {
  if (loadingMore.value || !hasMore.value) return
  loadingMore.value = true
  try {
    const res = await api.myBattleHistory(20, items.value.length)
    const more = res.items ?? []
    const existing = new Set(items.value.map((it) => it.id))
    items.value = items.value.concat(more.filter((it) => !existing.has(it.id)))
    hasMore.value = more.length >= 20
  } catch {
    // 加载更多失败：保留按钮可重试
  } finally {
    loadingMore.value = false
  }
}

onMounted(async () => {
  if (!store.loggedIn) await store.login()
  await reload()
})
</script>

<style scoped>
.battle-row {
  padding: 12rpx 0;
  border-bottom: 1rpx solid var(--border);
}

.battle-row:last-child {
  border-bottom: none;
}

.battle-row.expanded {
  background: rgba(88, 166, 255, 0.06);
  margin: 0 -12rpx;
  padding: 12rpx;
  border-radius: 12rpx;
}

.battle-head {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 12rpx;
  font-size: 26rpx;
}

.battle-stars {
  color: var(--warn);
  font-size: 22rpx;
}

.battle-score {
  font-size: 26rpx;
}

.battle-meta {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  margin-top: 4rpx;
  font-size: 22rpx;
}

.battle-detail {
  margin-top: 10rpx;
  padding-top: 8rpx;
  border-top: 1rpx dashed var(--border);
}

.kv {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  gap: 16rpx;
  padding: 6rpx 0;
  font-size: 24rpx;
}

.hash {
  font-size: 20rpx;
  word-break: break-all;
}

/* 加载更多（2026-11-01 新一轮第 55 轮） */
.load-more {
  margin-top: 16rpx;
  padding: 16rpx 0;
  text-align: center;
  font-size: 26rpx;
  color: var(--fire);
  background: var(--panel);
  border: 1rpx solid var(--border);
  border-radius: 12rpx;
}

/* 结果筛选（2026-11-01 新一轮第 74 轮） */
.filter-row {
  display: flex;
  flex-direction: row;
  gap: 12rpx;
  margin-bottom: 16rpx;
}

.filter-chip {
  padding: 10rpx 28rpx;
  border-radius: 999rpx;
  background: var(--panel);
  border: 1rpx solid var(--border);
  font-size: 24rpx;
  color: var(--muted);
}

.filter-chip.active {
  background: rgba(255, 107, 53, 0.16);
  border-color: var(--fire);
  color: var(--fire);
}
</style>