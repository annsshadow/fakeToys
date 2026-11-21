<template>
  <view class="page-root">
    <view class="pad safe-bottom">
      <view v-if="!loading && myRank" class="r3-kpi r4-kpi r4-enter">
        <view class="r3-kpi-main">
          <text class="r3-kpi-num">{{ myRank.rank > 0 ? '第 ' + myRank.rank + ' 名' : '暂无成绩' }}</text>
          <text class="r3-kpi-label">{{ curLabel }} · 共 {{ myRank.total }} 人</text>
          <text class="r3-kpi-sub">{{ formatScore(myRank.score) }} 分 · 已加载 {{ items.length }} 名</text>
        </view>
        <view class="r3-kpi-cell"><text class="r3-kpi-num">{{ myRank.rank > 0 ? myRank.rank : '—' }}</text><text class="r3-kpi-label">当前名次</text></view>
        <view class="r3-kpi-cell"><text class="r3-kpi-num">{{ scoreBuckets.length }}</text><text class="r3-kpi-label">分数区间数</text></view>
      </view>
      <view v-if="!loading && r4AiRank" class="r3-ai r4-ai r4-enter" aria-label="AI 冲榜提示">
        <view class="r3-ai-head">
          <text class="r3-ai-badge">AI</text>
          <text class="r3-ai-title">AI 冲榜提示</text>
          <text class="r3-ai-engine">规则引擎 · 基于当前榜单</text>
        </view>
        <text class="r3-ai-conclusion">{{ r4AiRank.conclusion }}</text>
        <text class="r3-ai-basis">{{ r4AiRank.basis }}</text>
        <view class="r3-ai-cta" @click="r4GoRank()">去冲榜关卡 →</view>
      </view>
      <view class="tabs r3-filter r4-filter">
        <view
          v-for="t in tabs"
          :key="t.key"
          class="tab r3-chip"
          :class="{ active: curType === t.key }"
          @click="switchTab(t.key)"
        >
          {{ t.label }}
        </view>
      </view>
      <view class="rank-desc-row">
        <text class="muted">{{ curDesc }}</text>
        <!-- R79 榜单条数：切 tab 后立刻知道这一榜有多少人 -->
        <!-- 第六批 R59：榜单可能被服务端截断，「共 N 人」是假话；改成「显示前 N 名」 -->
        <text v-if="items.length" class="rank-count">显示前 {{ items.length }} 名</text>
      </view>

      <!-- G76 分数分布：前 N 名柱状图，我的那根高亮 -->
      <view v-if="!loading && scoreDistribution.length" class="card dist-card">
        <view class="card-title">分数分布<text class="dim">前 {{ scoreDistribution.length }} 名</text></view>
        <view class="dist-bars">
          <view v-for="b in scoreDistribution" :key="b.rank" class="dist-col" :class="{ mine: b.mine }">
            <view class="dist-bar" :style="{ height: b.h + '%' }" />
          </view>
        </view>
      </view>

      <!-- G106 分数区间直方图：我在哪个分数段 -->
      <view v-if="!loading && scoreBuckets.length" class="card bucket-card">
        <view class="card-title">分数区间</view>
        <view class="bucket-bars">
          <view v-for="b in scoreBuckets" :key="b.lo" class="bucket-col" :class="{ mine: b.mine }">
            <view class="bucket-bar" :style="{ height: b.h + '%' }" />
            <text class="dim">{{ b.label }}</text>
          </view>
        </view>
      </view>

      <view v-if="loading" class="card">
        <text class="muted">加载中…</text>
        <view v-for="n in 3" :key="n" class="skeleton-row">
          <view class="skeleton-bar" style="width: 48rpx; height: 48rpx; border-radius: 50%" />
          <view class="skeleton-bar" style="width: 38%" />
          <view class="skeleton-bar" style="width: 16%; margin-left: auto" />
        </view>
      </view>
      <view v-else-if="items.length === 0" class="r3-empty r4-empty empty-block" aria-live="polite" role="status">
        <text class="r3-empty-slot empty-glyph">🏆</text>
        <text class="r3-empty-title">暂无数据 · 还没有榜单数据</text>
        <text class="r3-empty-reason">{{ noScoreHint }}</text>
        <view class="r3-empty-action" @click="r4GoRank()">去冲榜关卡 →</view>
      </view>

      <view v-else class="card r3-data-grid r4-grid" style="padding: 8rpx 24rpx">
        <view
          v-for="(it, i) in items"
          :key="it.user_id"
          class="rank-row list-row"
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
        <!-- G137 追赶分差 -->
        <view v-if="chaseGap" class="chase-gap">
          <text v-if="chaseGap.toAbove !== null">距上一名还差 {{ chaseGap.toAbove }} 分</text>
          <text v-if="chaseGap.toBelow !== null">领先下一名 {{ chaseGap.toBelow }} 分</text>
        </view>
        <text v-else class="dim">{{ noScoreHint }}</text>
      </view>
      <!-- 第六批 R39：拉取失败与「没成绩」此前都让整块消失，用户分不清。
           显式占位说明「排名暂不可用」，避免把故障误读成自己没上榜。 -->
      <view v-else-if="!loading" class="card my-rank">
        <view class="my-rank-row">
          <text class="muted">我的排名</text>
          <text class="my-rank-none">排名暂不可用</text>
        </view>
        <text class="dim">榜单仍可正常查看，稍后重试即可。</text>
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

/* -------------------------------------------------------------------------- */
/* G76 榜单分数分布柱状图（第七批）                                             */
/* 已加载的每名玩家一根柱，最高分归一到满高；我的那根高亮。数据来自 items/myRank。 */
/* -------------------------------------------------------------------------- */
/** G137 追赶分差：距上一名还差多少、超越下一名需要多少（把名次换成可执行目标）。 */
const chaseGap = computed(() => {
  const my = myRank.value
  if (!my || my.rank <= 0) return null
  const idx = my.rank - 1
  const above = items.value[idx - 1]
  const below = items.value[idx + 1]
  const myScore = Number(my.score ?? 0)
  // 服务端 economy.go leaderboardOrderSQL：只有效率榜是 ASC（分数越低越好），
  // 其余榜 DESC。分差必须按**当前榜**的排序方向取绝对值，否则效率榜上
  // Math.max(0, ...) 会把两侧都夹成 0，显示「还差 0 分 / 领先 0 分」。
  const ascending = curType.value === 'efficiency'
  return {
    toAbove: above
      ? Math.max(0, ascending ? myScore - Number(above.score) : Number(above.score) - myScore)
      : null,
    toBelow: below
      ? Math.max(0, ascending ? Number(below.score) - myScore : myScore - Number(below.score))
      : null,
  }
})

/** G106 分数区间直方图：把前 N 名分数分桶，我的分数所在桶高亮。 */
const scoreBuckets = computed(() => {
  const scores = items.value.map((it: any) => Number(it.score) || 0)
  if (!scores.length) return []
  const max = Math.max(...scores)
  const bucketSize = Math.max(1, Math.ceil(max / 5))
  const buckets = Array.from({ length: 5 }, (_, i) => ({
    label: `${i * bucketSize}`,
    lo: i * bucketSize,
    hi: (i + 1) * bucketSize,
    count: 0,
    mine: false,
  }))
  const myScore = Number(myRank.value?.score ?? -1)
  for (const s of scores) {
    const idx = Math.min(4, Math.floor(s / bucketSize))
    buckets[idx]!.count++
  }
  if (myScore >= 0) {
    const idx = Math.min(4, Math.floor(myScore / bucketSize))
    buckets[idx]!.mine = true
  }
  const maxCount = Math.max(1, ...buckets.map((b) => b.count))
  return buckets.map((b) => ({ ...b, h: Math.max(4, Math.round(b.count / maxCount * 100)) }))
})

const scoreDistribution = computed(() => {
  const list = items.value.slice(0, 20)
  const max = Math.max(1, ...list.map((it: any) => Number(it.score) || 0))
  return list.map((it: any, i: number) => ({
    rank: i + 1,
    score: Number(it.score) || 0,
    h: Math.max(4, Math.round((Number(it.score) || 0) / max * 100)),
    mine: it.user_id === store.userId || (myRank.value?.rank === i + 1),
  }))
})

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

/** R4：把分桶/名次/追赶分差翻译成一句可执行的冲榜建议。 */
const r4AiRank = computed(() => {
  const mine = myRank.value
  const buckets = scoreBuckets.value
  if (!mine || !buckets.length) return null
  const band = buckets.find((b) => b.mine)
  const need = chaseGap.value?.toAbove
  return {
    conclusion: need !== null && need !== undefined
      ? `距上一名还差 ${need} 分，冲榜优先目标已锁定`
      : `当前第 ${mine.rank || '—'}/${mine.total} 名，处在 ${band ? band.label + ' 分' : '待定'} 分数段`,
    basis: `依据：${curLabel.value}分数分布 / 本人名次 · 更新于本次进入`,
  }
})

function r4GoRank() {
  uni.navigateTo({ url: '/pages/stage/stage' })
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
/* G137 追赶分差 */
.chase-gap { display: flex; flex-direction: row; gap: 16rpx; margin-top: 8rpx; font-size: 22rpx; color: var(--accent); }

/* G106 分数区间直方图 */
.bucket-bars { display: flex; flex-direction: row; align-items: flex-end; height: 200rpx; gap: 6rpx; padding-top: 12rpx; }
.bucket-col { flex: 1; height: 100%; display: flex; flex-direction: column; align-items: center; justify-content: flex-end; gap: 4rpx; }
.bucket-bar { width: 100%; background: var(--ice); opacity: 0.7; border-radius: 4rpx 4rpx 0 0; }
.bucket-col.mine .bucket-bar { background: var(--fire); opacity: 1; }
.bucket-col .dim { font-size: 18rpx; }

/* G76 分数分布柱状图 */
.dist-card .card-title .dim { font-size: 22rpx; font-weight: 400; }
.dist-bars {
  display: flex;
  flex-direction: row;
  align-items: flex-end;
  height: 200rpx;
  gap: 4rpx;
  padding-top: 12rpx;
}
.dist-col { flex: 1; height: 100%; display: flex; align-items: flex-end; }
.dist-bar {
  width: 100%;
  background: var(--ice);
  border-radius: 4rpx 4rpx 0 0;
  opacity: 0.7;
}
.dist-col.mine .dist-bar { background: var(--fire); opacity: 1; }

/* R79 榜单条数 */
.rank-desc-row {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  align-items: baseline;
  gap: 16rpx;
  margin-bottom: 16rpx;
}

.rank-count {
  flex-shrink: 0;
  font-size: 22rpx;
  color: var(--dim);
}

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
  /* R92 奖牌轻微浮动，前三名更有「领奖台」感 */
  animation: medal-float 2.4s ease-in-out infinite;
}

@keyframes medal-float {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-3rpx); }
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
