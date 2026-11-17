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
        <!-- R78 筛选结果计数：筛选后还剩几场一眼可见 -->
        <!-- 第六批 R60：分页未拉完时「共 N 场」是错的，改为「已加载 N 场」 -->
        <text class="filter-count">{{ hasMore ? '已加载' : '共' }} {{ items.length }} 场</text>
      </view>

      <!-- G96 战绩总账：击杀/漏怪/分数累计与均分 -->
      <view v-if="!loading && items.length" class="card total-card">
        <view class="card-title">战绩总账</view>
        <view class="total-grid">
          <view class="total-item"><text class="mono total-val">{{ battleTotals.count }}</text><text class="dim">场数</text></view>
          <view class="total-item"><text class="mono total-val">{{ battleTotals.kills }}</text><text class="dim">总击杀</text></view>
          <view class="total-item"><text class="mono total-val">{{ battleTotals.leaked }}</text><text class="dim">总漏怪</text></view>
          <view class="total-item"><text class="mono total-val">{{ battleTotals.avgScore }}</text><text class="dim">均分</text></view>
        </view>
        <!-- G140 连胜/连败 -->
        <text v-if="streak.count > 0" class="streak-note">
          当前{{ streak.type === 'win' ? '连胜' : '连败' }} {{ streak.count }} 场 · 最长连胜 {{ streak.maxWin }}
        </text>
      </view>

      <!-- G70 战绩趋势折线：分数走势 + 胜负色点，趋势一眼可见 -->
      <view v-if="trend.count >= 2" class="card trend-card">
        <view class="card-title">
          <text>战绩趋势</text>
          <text class="dim">近 {{ trend.count }} 局 · 均分 {{ trend.avg }}</text>
        </view>
        <svg class="trend-svg" viewBox="0 0 300 96" aria-label="战绩分数趋势">
          <polyline class="trend-line" :points="trend.line" />
          <polyline v-if="leakRateTrend.pts.length >= 2" class="leak-line" :points="leakRateTrend.line" />
          <circle
            v-for="(p, i) in trend.pts"
            :key="'pt' + i"
            :cx="p.x" :cy="p.y" r="3"
            :class="p.win ? 'trend-win' : 'trend-lose'"
          />
        </svg>
        <view class="trend-legend">
          <text class="dim">● 分数（蓝）　● 漏怪率（红）　越低越好</text>
        </view>
      </view>

      <!-- G83 关卡胜率分布：每关打赢比例，找出还卡着的关 -->
      <view v-if="!loading && winRateByLevel.length" class="card winrate-card">
        <view class="card-title">关卡胜率</view>
        <view v-for="w in winRateByLevel" :key="w.level" class="wr-row">
          <text class="dim wr-label">第 {{ w.level }} 关</text>
          <view class="wr-track">
            <view class="wr-fill" :style="{ width: w.rate + '%' }" />
          </view>
          <text class="mono wr-val">{{ w.rate }}%</text>
        </view>
      </view>

      <!-- G88 星级分布：0-3 星各占比，看通关质量 -->
      <view v-if="!loading && items.length" class="card stardist-card">
        <view class="card-title">星级分布</view>
        <view v-for="s in starDistribution" :key="s.stars" class="sd-row">
          <text class="dim sd-label">{{ s.stars }} 星</text>
          <view class="sd-track">
            <view class="sd-fill" :style="{ width: s.pct + '%' }" />
          </view>
          <text class="mono sd-val">{{ s.n }}</text>
        </view>
      </view>

      <!-- G118 关卡交战热度 -->
      <view v-if="!loading && battleHeat.length" class="card heat-card">
        <view class="card-title">交战热度<text class="dim">反复挑战的关卡</text></view>
        <view v-for="h in battleHeat" :key="h.level" class="heat-row">
          <text class="dim heat-label">第 {{ h.level }} 关</text>
          <view class="heat-track"><view class="heat-fill" :style="{ width: h.pct + '%' }" /></view>
          <text class="mono heat-val">{{ h.n }}</text>
        </view>
      </view>

      <!-- G132 命中率效率卡 -->
      <view v-if="efficiencyStats" class="card eff-card">
        <view class="card-title">战斗效率<text class="dim">已展开 {{ efficiencyStats.count }} 局</text></view>
        <text class="dim">命中率 {{ efficiencyStats.accuracy }}% · 平均最高热量 {{ efficiencyStats.avgHeat }}</text>
      </view>
      <view v-else-if="!loading && items.length" class="card">
        <text class="dim">点开任意战报后，这里会统计命中率与热量效率。</text>
      </view>

      <view v-if="loading" class="card">
        <text class="muted">加载中…</text>
        <view v-for="n in 3" :key="n" class="skeleton-row">
          <view class="skeleton-bar" style="width: 44rpx; height: 44rpx; border-radius: 8rpx" />
          <view class="skeleton-bar" style="width: 50%" />
          <view class="skeleton-bar" style="width: 14%; margin-left: auto" />
        </view>
      </view>

      <view v-else-if="items.length === 0" class="card empty-state">
        <!-- R19 空态不再是一行孤零零的文字：给一个锚点 + 明确下一步 -->
        <text class="empty-glyph">⚔</text>
        <text class="card-title">还没有战报</text>
        <text class="muted" style="display: block; margin-top: 8rpx">
          打一局战斗后，胜负、星级、分数和击杀都会出现在这里。
        </text>
        <!-- 第六批 R36：此前这行看起来像链接却点不动（假入口）。接上真实跳转。 -->
        <text class="empty-hint link" @click="goStage">去关卡页挑一关开打 →</text>
      </view>

      <view v-else class="card" style="padding: 8rpx 24rpx">
        <view
          v-for="it in items"
          :key="it.id"
          class="battle-row tap"
          :class="[it.result === 'win' ? 'win' : 'lose', { expanded: expandedId === it.id }]"
          @click="toggleDetail(it)"
        >
          <view class="battle-head">
            <!-- R19 胜负做成实心徽章，整行再给淡色底，扫一眼就能分胜败 -->
            <text class="result-badge" :class="it.result === 'win' ? 'win' : 'lose'">
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
              <!-- G78 元素使用频次构成条（不是伤害量，服务端无按元素伤害拆分） -->
              <view v-if="elementMix.length" class="mix-bar">
                <view v-for="(m, i) in elementMix" :key="i" class="mix-seg" :style="{ width: m.pct + '%' }" :title="m.name" />
              </view>
              <view v-if="reactionText" class="kv"><text class="muted">反应</text><text class="flex-1">{{ reactionText }}</text></view>
              <view v-if="terrainText" class="kv"><text class="muted">地形</text><text class="flex-1">{{ terrainText }}</text></view>
              <view class="kv">
                <text class="muted">回放哈希</text>
                <text class="mono hash">{{ detail.replay_hash }}</text>
              </view>
              <!-- 复制哈希（2026-11-01 新一轮第 87 轮）：用外部工具验真时无需手选 -->
              <view class="btn copy-hash-btn" @click="copyHash(detail.replay_hash)">复制回放哈希</view>
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
/** 服务端提供的 keyset 游标（上一页末行 id）；0 表示没有更多页。 */
const nextCursor = ref(0)
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

/* -------------------------------------------------------------------------- */
/* G70 战绩趋势折线（第七批）                                                   */
/* 已加载战报按时间正序（列表最新在前，这里反转）画分数折线 + 胜负色点。         */
/* 数据来自 items[].score/result，纯客户端派生，零契约改动。                     */
/* -------------------------------------------------------------------------- */
const trend = computed(() => {
  const seq = [...items.value].reverse().slice(-20) // 最近 20 局，时间正序
  const n = seq.length
  const maxScore = Math.max(1, ...seq.map((it) => it.score))
  const W = 300
  const H = 90
  const pts = seq.map((it, i) => ({
    x: n <= 1 ? W / 2 : (i / (n - 1)) * W,
    y: H - (it.score / maxScore) * (H - 12) - 6,
    // 胜负只认 result（服务端 resultStr 只有 win/lose）；stars 由分数门槛
    // 独立算出、与胜负无关（lose+多星是合法数据），不能拿 stars>0 判胜。
    win: it.result === 'win',
    score: it.score,
  }))
  return {
    pts,
    line: pts.map((p) => `${p.x.toFixed(1)},${p.y.toFixed(1)}`).join(' '),
    count: n,
    maxScore,
    avg: n ? Math.round(seq.reduce((s, it) => s + it.score, 0) / n) : 0,
  }
})

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
    // 服务端 MyBattleHistoryPage 先多取 1 行判断 hasMore：总数恰好满一页时
    // hasMore=false → next_cursor=0。以 next_cursor 为准，避免满页时按钮无效常驻。
    hasMore.value = (res.next_cursor ?? 0) > 0
    nextCursor.value = res.next_cursor ?? 0
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

/* -------------------------------------------------------------------------- */
/* G140 连胜/连败：当前连续胜负状态 + 最长连胜，给连续作战的节奏反馈。             */
/* -------------------------------------------------------------------------- */
const streak = computed(() => {
  const seq = [...items.value].reverse() // 时间正序
  let cur = 0
  let curType = ''
  let maxWin = 0
  let tmp = 0
  for (const it of seq) {
    const win = it.result === 'win'
    if (curType === (win ? 'win' : 'lose')) cur++
    else { curType = win ? 'win' : 'lose'; cur = 1 }
    if (win) { tmp++; maxWin = Math.max(maxWin, tmp) } else tmp = 0
  }
  return { type: curType, count: seq.length ? cur : 0, maxWin }
})

/* -------------------------------------------------------------------------- */
/* G132 命中率效率卡：只聚合已展开过的局（details 缓存），不额外发请求。           */
/* -------------------------------------------------------------------------- */
const efficiencyStats = computed(() => {
  const ds = Object.values(details.value)
  if (!ds.length) return null
  const shots = ds.reduce((s, d) => s + (d.shots ?? 0), 0)
  const hits = ds.reduce((s, d) => s + (d.hits ?? 0), 0)
  return {
    count: ds.length,
    accuracy: shots > 0 ? Math.round(hits / shots * 100) : 0,
    avgHeat: Math.round(ds.reduce((s, d) => s + (d.heat_max ?? 0), 0) / ds.length),
  }
})

/* -------------------------------------------------------------------------- */
/* G118 关卡交战热度：按 level_id 聚合次数 Top8 —— 看出反复卡在哪关。             */
/* -------------------------------------------------------------------------- */
const battleHeat = computed(() => {
  const m = new Map<number, number>()
  for (const it of items.value) m.set(it.level_id, (m.get(it.level_id) ?? 0) + 1)
  const arr = [...m.entries()].map(([level, n]) => ({ level, n })).sort((a, b) => b.n - a.n).slice(0, 8)
  const max = Math.max(1, ...arr.map((a) => a.n))
  return arr.map((a) => ({ ...a, pct: Math.round(a.n / max * 100) }))
})

/* -------------------------------------------------------------------------- */
/* G102 漏怪率走势（第七批）                                                    */
/* 每局 leaked/(kills+leaked) 的防守质量百分比，序列折线 —— 看出是靠输出硬堆还是守得稳。 */
/* -------------------------------------------------------------------------- */
const leakRateTrend = computed(() => {
  const seq = [...items.value].reverse().slice(-20)
  const pts = seq.map((it, i) => {
    const k = it.kills ?? 0
    const l = it.leaked ?? 0
    const rate = k + l > 0 ? Math.round(l / (k + l) * 100) : 0
    return {
      x: seq.length <= 1 ? 150 : (i / (seq.length - 1)) * 300,
      y: 80 - rate * 0.7,
      rate,
    }
  })
  return { pts, line: pts.map((p) => `${p.x.toFixed(1)},${p.y.toFixed(1)}`).join(' ') }
})

/* -------------------------------------------------------------------------- */
/* G96 战绩总量统计（第七批）                                                   */
/* 已加载战报的击杀/漏怪/反应/分数总和与均值 —— 一个「我的战斗生涯」总账。        */
/* -------------------------------------------------------------------------- */
const battleTotals = computed(() => {
  const n = items.value.length || 1
  const sum = (k: string) => items.value.reduce((s, it) => s + (Number((it as any)[k]) || 0), 0)
  return {
    count: items.value.length,
    kills: sum('kills'),
    leaked: sum('leaked'),
    score: sum('score'),
    avgScore: Math.round(sum('score') / n),
  }
})

/* -------------------------------------------------------------------------- */
/* G88 星级分布（第七批）                                                       */
/* 已加载战报按 0/1/2/3 星聚合，看「我的通关质量」。数据来自 items[].stars。      */
/* -------------------------------------------------------------------------- */
const starDistribution = computed(() => {
  const counts = [0, 0, 0, 0]
  for (const it of items.value) {
    const s = Math.max(0, Math.min(3, it.stars ?? 0))
    counts[s]++
  }
  const total = counts.reduce((a, b) => a + b, 0) || 1
  return counts.map((n, stars) => ({ stars, n, pct: Math.round(n / total * 100) }))
})

/* -------------------------------------------------------------------------- */
/* G83 关卡胜率分布（第七批）                                                   */
/* 已加载战报按 level_id 聚合胜/负，展示每关打赢比例 —— 看「哪些关还卡着」。       */
/* 数据来自 items[].level_id/result，纯客户端派生。                               */
/* -------------------------------------------------------------------------- */
const winRateByLevel = computed(() => {
  const m = new Map<number, { level: number; win: number; lose: number }>()
  for (const it of items.value) {
    const win = it.result === 'win'
    const cur = m.get(it.level_id) ?? { level: it.level_id, win: 0, lose: 0 }
    if (win) cur.win++
    else cur.lose++
    m.set(it.level_id, cur)
  }
  return [...m.values()]
    .map((v) => ({ ...v, rate: Math.round(v.win / (v.win + v.lose) * 100) }))
    .sort((a, b) => a.level - b.level)
    .slice(-8) // 最近 8 个关卡，避免长榜
})

/* G78 元素使用构成（第七批）：注意 BattleDetail 只有「使用次数」，无按元素伤害数值，
   所以这里诚实标注是「使用频次构成」，绝不声称伤害量。 */
const elementMix = computed(() => {
  const d = detail.value
  if (!d) return []
  const entries = Object.entries(d.elements_used).filter(([, v]) => v > 0)
  const total = entries.reduce((s, [, v]) => s + v, 0) || 1
  return entries.map(([k, v]) => ({
    name: ELEMENT_NAME[k as keyof typeof ELEMENT_NAME] ?? k,
    pct: Math.round(v / total * 100),
  }))
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

/** 第六批 R36：空态引导接上真实跳转（此前是点不动的假链接）。 */
function goStage() {
  uni.navigateTo({ url: '/pages/stage/stage' })
}

/**
 * 复制回放哈希（2026-11-01 新一轮第 87 轮）。
 * 玩家用外部工具重放验真时，无需手动选中一长串字符。
 */
function copyHash(hash: string) {
  if (!hash) return
  uni.setClipboardData({
    data: hash,
    success: () => {
      uni.showToast({ title: '已复制回放哈希', icon: 'success' })
    },
  })
}

/**
 * 加载更早的战报（2026-11-01 新一轮第 55 轮）。
 * 服务端按时间倒序分页，响应给出 next_cursor（上一页末行 id）作 keyset 游标：
 * 翻页途中新插入的战报不会把旧行挤到下一页造成重复/漏行。
 */
async function loadMore() {
  if (loadingMore.value || !hasMore.value) return
  loadingMore.value = true
  try {
    const res = await api.myBattleHistory(20, 0, filter.value === 'all' ? '' : filter.value, nextCursor.value)
    const more = res.items ?? []
    const existing = new Set(items.value.map((it) => it.id))
    items.value = items.value.concat(more.filter((it) => !existing.has(it.id)))
    // 同上：末页恰好满 20 条时 next_cursor=0，不应继续显示加载更多。
    hasMore.value = (res.next_cursor ?? 0) > 0
    nextCursor.value = res.next_cursor ?? 0
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
/* G140 连胜连败 */
.streak-note { display: block; text-align: center; font-size: 22rpx; color: var(--warn); margin-top: 8rpx; }

/* G118 交战热度 */
.heat-row { display: flex; flex-direction: row; align-items: center; gap: 12rpx; margin: 8rpx 0; }
.heat-label { width: 140rpx; flex-shrink: 0; }
.heat-track { flex: 1; height: 12rpx; background: var(--panel-3); border-radius: 999rpx; overflow: hidden; }
.heat-fill { height: 100%; background: var(--accent); border-radius: 999rpx; }
.heat-val { width: 60rpx; text-align: right; font-size: 22rpx; color: var(--accent); }

/* G96 战绩总账 */
.total-grid { display: flex; flex-direction: row; }
.total-item { flex: 1; display: flex; flex-direction: column; align-items: center; gap: 4rpx; }
.total-val { font-size: 36rpx; font-weight: 700; color: var(--fire); }

/* G88 星级分布 */
.sd-row { display: flex; flex-direction: row; align-items: center; gap: 12rpx; margin: 8rpx 0; }
.sd-label { width: 80rpx; flex-shrink: 0; }
.sd-track { flex: 1; height: 12rpx; background: var(--panel-3); border-radius: 999rpx; overflow: hidden; }
.sd-fill { height: 100%; background: var(--warn); border-radius: 999rpx; }
.sd-val { width: 60rpx; text-align: right; font-size: 22rpx; color: var(--warn); }

/* G83 关卡胜率分布 */
.wr-row { display: flex; flex-direction: row; align-items: center; gap: 12rpx; margin: 8rpx 0; }
.wr-label { width: 140rpx; flex-shrink: 0; }
.wr-track { flex: 1; height: 12rpx; background: var(--panel-3); border-radius: 999rpx; overflow: hidden; }
.wr-fill { height: 100%; background: var(--ok); border-radius: 999rpx; }
.wr-val { width: 72rpx; text-align: right; font-size: 22rpx; color: var(--ok); }

/* G78 元素使用频次构成条 */
.mix-bar { display: flex; flex-direction: row; height: 12rpx; border-radius: 6rpx; overflow: hidden; margin: 6rpx 0 10rpx; }
.mix-seg { height: 100%; background: var(--ice); }
.mix-seg:nth-child(5n+2) { background: var(--fire); }
.mix-seg:nth-child(5n+3) { background: var(--lightning); }
.mix-seg:nth-child(5n+4) { background: var(--corrosion); }
.mix-seg:nth-child(5n+5) { background: var(--kinetic); }

/* G70 战绩趋势折线 */
.trend-card .card-title { margin-bottom: 8rpx; }
.trend-svg { width: 100%; height: 192rpx; overflow: visible; }
.trend-line {
  fill: none;
  stroke: var(--ice);
  stroke-width: 2;
  stroke-linejoin: round;
}
.leak-line { fill: none; stroke: var(--bad); stroke-width: 1.5; stroke-dasharray: 4 3; }
.trend-legend { text-align: center; font-size: 20rpx; }
.trend-win { fill: var(--ok); }
.trend-lose { fill: var(--bad); }

.battle-row {
  padding: 12rpx 0;
  border-bottom: 1rpx solid var(--border);
}

/* R19 胜负色差：整行淡底 + 左侧色条，不必逐字读「胜/负」 */
.battle-row.win {
  background: linear-gradient(90deg, rgba(126, 231, 135, 0.08), transparent 60%);
  box-shadow: inset 4rpx 0 0 rgba(126, 231, 135, 0.55);
}

.battle-row.lose {
  background: linear-gradient(90deg, rgba(255, 123, 114, 0.08), transparent 60%);
  box-shadow: inset 4rpx 0 0 rgba(255, 123, 114, 0.45);
}

/* R93 胜负徽章弹入，扫列表时胜负结论更醒目 */
.result-badge {
  animation: badge-pop 0.3s cubic-bezier(0.2, 0.9, 0.3, 1.3);
}

@keyframes badge-pop {
  from { transform: scale(0.7); opacity: 0; }
  to { transform: scale(1); opacity: 1; }
}

.result-badge {
  min-width: 44rpx;
  text-align: center;
  padding: 6rpx 0;
  border-radius: 8rpx;
  font-size: 24rpx;
  font-weight: 700;
}

.result-badge.win {
  color: var(--ok);
  background: rgba(126, 231, 135, 0.14);
}

.result-badge.lose {
  color: var(--bad);
  background: rgba(255, 123, 114, 0.14);
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
}

.empty-glyph {
  font-size: 64rpx;
  opacity: 0.6;
  margin-bottom: 8rpx;
}

.empty-hint {
  margin-top: 16rpx;
  font-size: 24rpx;
  color: var(--fire);
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
  font-size: 24rpx;
}

.battle-score {
  font-size: 26rpx;
}

.battle-meta {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  margin-top: 4rpx;
  font-size: 24rpx;
}

.battle-detail {
  margin-top: 10rpx;
  padding-top: 8rpx;
  border-top: 1rpx dashed var(--border);
}



/* 复制回放哈希（2026-11-01 新一轮第 87 轮） */
.copy-hash-btn {
  margin-top: 10rpx;
  padding: 12rpx 0;
  text-align: center;
  font-size: 24rpx;
  color: var(--fire);
  background: rgba(255, 107, 53, 0.08);
  border: 1rpx solid rgba(255, 107, 53, 0.3);
  border-radius: 10rpx;
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

/* R78 筛选结果计数 */
.filter-count {
  margin-left: auto;
  align-self: center;
  font-size: 22rpx;
  color: var(--dim);
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