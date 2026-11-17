<template>
  <view class="page-root">
    <view class="pad safe-bottom">
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

      <view v-if="loading" class="card">
        <text class="muted">加载中…</text>
        <view v-for="n in 3" :key="n" class="skeleton-row">
          <view class="skeleton-bar" style="width: 48rpx; height: 48rpx; border-radius: 50%" />
          <view class="skeleton-bar" style="width: 38%" />
          <view class="skeleton-bar" style="width: 16%; margin-left: auto" />
        </view>
      </view>
      <view v-else-if="items.length === 0" class="card empty-block">
        <text class="empty-glyph">🏆</text>
        <text class="muted">暂无数据</text>
        <text class="dim">打一局战斗积累成长值，就能出现在这个榜上。</text>
      </view>

      <view v-else class="card" style="padding: 8rpx 24rpx">
        <view
          v-for="(it, i) in items"
          :key="it.user_id"
          class="rank-row"
          :class="[rankRowClass(i, it), { me: it.user_id === store.userId }]"
        >
          <text class="rank-no mono" :class="'no-' + (i + 1)">{{ i + 1 }}</text>
          <text class="rank-name flex-1">{{ it.nickname || '玩家' }}</text>
          <!-- R49 我的行给「我」标记：榜单很长时不用找高亮在哪 -->
          <text v-if="it.user_id === store.userId" class="me-chip">我</text>
          <!-- R18 前三名奖牌：名次徽章之外再给一枚一眼可辨的贵金属标记 -->
          <text v-if="i < 3" class="medal">{{ MEDALS[i] }}</text>
          <text class="rank-score mono">{{ formatScore(it.score) }}</text>
        </view>
      </view>

      <!--
        我的排名（2026-10-23）。
        此前只回前 50 条，而 `me` 高亮只在前 50 内有效 ——
        未进前 50 的玩家在榜上找不到自己，也不知道总共有多少人。
      -->
      <view v-if="myRank" class="card my-rank">
        <view class="my-rank-row">
          <text class="muted">我的排名</text>
          <text v-if="myRank.rank > 0" class="my-rank-no mono">
            第 {{ myRank.rank }} / 共 {{ myRank.total }} 人
          </text>
          <text v-else class="my-rank-none">暂无成绩</text>
        </view>
        <text v-if="myRank.rank > 0" class="dim">我的{{ curLabel }}：{{ formatScore(myRank.score) }}</text>
        <text v-else class="dim">{{ noScoreHint }}</text>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { onLoad, onShareAppMessage } from '@dcloudio/uni-app'
import { useGameStore } from '@/store/game'
import * as api from '@/api/client'
import type { MyRank } from '@/api/client'
import { rankShare } from '@/utils/share'

const store = useGameStore()
const curType = ref('power')
const items = ref<any[]>([])
const loading = ref(false)
/** 我的排名。null 表示还没拿到（加载中或失败）—— 此时不渲染那一块，不显示假名次。 */
const myRank = ref<MyRank | null>(null)
// 第 136 轮：请求序号（模块级 let，非 ref —— 无需响应式）。
let loadSeq = 0

const tabs = [
  { key: 'power', label: '战力榜' },
  { key: 'stage', label: '关卡榜' },
  { key: 'efficiency', label: '效率榜' },
]

const curLabel = computed(() => tabs.find((t) => t.key === curType.value)?.label ?? '排名')

/** 无成绩时的提示：三种榜的原因不同，不能一句话打发。 */
const noScoreHint = computed(() => {
  switch (curType.value) {
    case 'stage':
      return '通关任意一关后就会出现在关卡榜上。'
    case 'efficiency':
      return '通关一次并记录战力后才会进效率榜。'
    default:
      return '打一局战斗积累成长值后就会出现在战力榜上。'
  }
})

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
  // 2026-11-16：删掉同值死分支 —— 两个分支返回的都是 String(v)，
  // 「效率榜将来会格式化」的意图用这条注释保留，真需要时再写。
  return String(v)
}

/** R18 前三名奖牌字形（与 --rank-* 令牌配色对应）。 */
const MEDALS = ['🥇', '🥈', '🥉'] as const

/**
 * R18 榜单视觉分层：
 *  - 前三名给 podium 底 + 左侧贵金属描边；
 *  - 0 分玩家给 zero 降权 —— 有分与没分不再是一模一样的行。
 * 「我」的高亮仍由模板里独立的 me 类负责，两者互不覆盖。
 */
function rankRowClass(i: number, it: { score: number }): string {
  const cls: string[] = []
  if (i < 3) cls.push(`podium podium-${i + 1}`)
  if (Number(it.score) <= 0) cls.push('zero')
  return cls.join(' ')
}

async function load() {
  // 第 136 轮：记录本次请求发起时的 tab + 自增序号。
  // 若期间用户又切了 tab（发起更新的请求），本次的响应就是「陈旧」的，
  // 一律丢弃，避免旧的 fetchLeaderboard 后到、把新 tab 的列表覆盖掉。
  const seq = ++loadSeq
  const type = curType.value
  loading.value = true
  try {
    // 两个请求并发：榜单是公开的、我的排名需要鉴权，互不依赖。
    // ⚠️ 我的排名**失败不能连累榜单** —— 榜照样能看，只是少一条「我在哪」。
    const [res, mine] = await Promise.all([
      api.fetchLeaderboard(type),
      api.fetchMyRank(type).catch(() => null),
    ])
    if (seq !== loadSeq) return // 已有更新的请求，丢弃本陈旧响应
    items.value = res.items ?? []
    myRank.value = mine
  } catch (e) {
    if (seq !== loadSeq) return
    uni.showToast({ title: (e as Error).message, icon: 'none' })
    items.value = []
    myRank.value = null
  } finally {
    if (seq === loadSeq) loading.value = false
  }
}

function switchTab(key: string) {
  curType.value = key
  // 先清掉旧榜的名次：否则切 tab 的一瞬间会显示**上一个榜**的名次，
  // 而那两个数字看起来都合理，玩家不会察觉是错的。
  myRank.value = null
  load()
}

/**
 * 深链（2026-10-23）。
 *
 * 分享出去的 `?type=stage` 必须真的选中对应 tab —— 否则「分享了一个
 * 到不了的位置」：点进来看到的仍是战力榜。
 * 未识别的 type 静默回落到默认榜（分享链接可能来自旧版本）。
 */
onLoad((q?: Record<string, string>) => {
  const t = q?.type
  if (t && tabs.some((x) => x.key === t)) curType.value = t
})

/**
 * 分享（2026-10-23）。
 *
 * 名次取自 `myRank`；没拿到时 `rank=0`，文案退回邀请式 ——
 * 不能写「排第 0」。
 */
onShareAppMessage(() =>
  rankShare({
    label: curLabel.value,
    rank: myRank.value?.rank ?? 0,
    type: curType.value,
  }),
)

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
  animation: me-glow 2s ease-in-out infinite;
}

@keyframes me-glow {
  0%, 100% { box-shadow: inset 0 0 0 rgba(255, 107, 53, 0); }
  50% { box-shadow: inset 0 0 16rpx rgba(255, 107, 53, 0.18); }
}

.me-chip {
  flex-shrink: 0;
  font-size: 20rpx;
  padding: 2rpx 12rpx;
  border-radius: 999rpx;
  color: var(--fire);
  background: var(--fire-soft);
  margin-right: 8rpx;
}

/* R18 前三名：底色 + 左侧贵金属描边（inset 阴影不撑破布局） */
.rank-row.podium {
  background: linear-gradient(90deg, var(--panel-2), transparent 78%);
  border-radius: 10rpx;
}

.rank-row.podium-1 { box-shadow: inset 5rpx 0 0 var(--rank-gold); }
.rank-row.podium-2 { box-shadow: inset 5rpx 0 0 var(--rank-silver); }
.rank-row.podium-3 { box-shadow: inset 5rpx 0 0 var(--rank-bronze); }

.medal {
  font-size: 26rpx;
  margin-right: 8rpx;
}

/* R18 0 分玩家降权：有分/没分一眼可分 */
.rank-row.zero {
  opacity: 0.55;
}

.rank-row.zero .rank-score {
  color: var(--dim);
}

.rank-no {
  width: 64rpx;
  font-size: 28rpx;
  color: var(--muted);
  font-weight: 700;
  font-variant-numeric: tabular-nums;
}

/* 前三名次做成圆形徽章（色由 --rank-* 令牌下发） */
.rank-row .rank-no.no-1,
.rank-row .rank-no.no-2,
.rank-row .rank-no.no-3 {
  height: 56rpx;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--panel-3);
  border: 1rpx solid currentColor;
}

/* 名次色走主题令牌（common.css 的 --rank-*），不要在页面里写死 */
.rank-no.no-1 { color: var(--rank-gold); }
.rank-no.no-2 { color: var(--rank-silver); }
.rank-no.no-3 { color: var(--rank-bronze); }

.rank-name {
  font-size: 26rpx;
}

.rank-score {
  font-size: 26rpx;
  color: var(--ok);
  font-weight: 600;
}

.my-rank {
  margin-top: 16rpx;
}

.my-rank-row {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  align-items: baseline;
  margin-bottom: 8rpx;
}

.my-rank-no {
  font-size: 28rpx;
  color: var(--fire);
  font-weight: 700;
}

.my-rank-none {
  font-size: 26rpx;
  color: var(--muted);
}
</style>
