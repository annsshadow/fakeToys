<template>
  <view class="page-root">
    <view class="pad safe-bottom">
      <view class="section-kicker section-kicker--ai r3-enter">
        <text class="section-kicker-cn">宝石收集雷达</text>
        <text class="section-kicker-en">GEM COVERAGE</text>
        <text class="r3-ai-badge">AI</text>
      </view>

      <!-- R12 跨系统成长优先级：与首页 aiInsight 同源口径，四页统一短板结论 -->
      <view v-if="r12Priority" class="r12-priority r12-priority-enter">
        <view class="r12-priority-head">
          <text class="r12-priority-badge">AI</text>
          <text class="r12-priority-title">当前最该强化 · 跨系统优先级</text>
        </view>
        <text class="r12-priority-text">{{ r12Priority.label }} {{ r12Priority.value }}（{{ r12Priority.pct }}%，权重 {{ r12Priority.weightPct }}%）：补满可涨约 {{ r12Priority.gain }} 分，优先补它</text>
        <text class="r12-priority-basis">{{ r12Priority.basis }}</text>
        <view class="r12-priority-cta" @click="r12Go()">{{ r12Cta.label }} →</view>
      </view>
      <view class="r3-kpi r3-enter">
        <view class="r3-kpi-main">
          <text class="r3-kpi-num">{{ ownedCount }}/{{ gems.length }}</text>
          <text class="r3-kpi-label">已拥有 / 图鉴种类</text>
          <text class="r3-kpi-sub">数据来自服务端 /config 与 /me/gems</text>
        </view>
        <view class="r3-kpi-cell">
          <text class="r3-kpi-num">{{ affixCoverage.length }}</text>
          <text class="r3-kpi-label">已覆盖词条</text>
        </view>
        <view class="r3-kpi-cell">
          <text class="r3-kpi-num">{{ r3GemPct }}%</text>
          <text class="r3-kpi-label">收集完成度</text>
        </view>
      </view>
      <view class="r3-progress-row is-ai r3-enter">
        <text class="r3-progress-label">收集进度</text>
        <view class="r3-progress"><view class="r3-progress-fill" :style="{ width: r3GemPct + '%' }" /></view>
        <text class="r3-progress-num">已拥有 {{ ownedCount }}/{{ gems.length }}</text>
      </view>
      <view class="r7-causal r7-causal-enter">
        <text class="r7-causal-title">AI 因果链 · 本局靠什么拿分 → 换上后预计提升</text>
        <text class="r7-causal-text">{{ r7Causal.text }}</text>
        <view class="r7-causal-empty" aria-live="polite" role="status">
          <text class="r7-causal-next">宝石页只提供收集线索：{{ r3AiGem ? r3AiGem.conclusion : '图鉴数据不足' }}</text>
          <text class="r7-causal-src">依据：{{ r7Causal.source }} · 本地规则引擎不写出战配置，不展示虚构收益。</text>
        </view>
      </view>
      <view class="r10-gem-gain r10-enter">
        <text class="r10-gem-title">AI · 宝石佩戴收益（服务端属性差分）</text>
        <template v-if="gemGain.available">
          <text class="r10-gem-text">{{ gemGain.text }}</text>
          <view v-for="row in gemGain.rows" :key="row.key" class="r10-gem-row">
            <text class="r10-gem-key">{{ row.label }} {{ fmt(row.before) }} → {{ fmt(row.after) }}</text>
            <text class="r10-gem-delta">{{ fmtSigned(row.delta) }}</text>
          </view>
        </template>
        <text v-else class="r10-gem-empty" aria-live="polite" role="status">佩戴收益未上报，先看词条描述</text>
        <!-- r10GemGainCacheKey(store.userId, gemId) 只用于同账号本地缓存定位，不渲染给玩家 -->
        <text class="r10-basis-note">依据：{{ gemGain.basis }} · 结果来自本机同账号佩戴前后的权威属性快照</text>
      </view>
      <view v-if="r3AiGem && gems.length" class="r3-ai r3-enter">
        <view class="r3-ai-head">
          <text class="r3-ai-badge">AI</text>
          <text class="r3-ai-title">AI 建议（本地规则引擎） · 收集缺口</text>
          <text class="r3-ai-engine">规则引擎</text>
        </view>
        <text class="r3-ai-conclusion">{{ r3AiGem.conclusion }}</text>
        <text class="r3-ai-basis">依据：{{ r3AiGem.basis }}</text>
        <text class="r3-ai-cta" role="button" @click="r3GoShop">去商城找宝石 →</text>
        <text class="r3-ai-cta secondary" role="button" @click="r3GoBag">去背包镶嵌 →</text>
      </view>
      <view v-else-if="!store.configLoading && gems.length === 0" class="r3-empty" aria-live="polite" role="status">
        <view class="r3-empty-slot">AI</view>
        <text class="r3-empty-title">还没有宝石图鉴数据</text>
        <text class="r3-empty-reason">配置尚未到达或网络请求未完成，暂时无法判断收集缺口。</text>
        <text class="r3-empty-action" role="button" @click="loadPage()">重新加载图鉴</text>
      </view>

      <!-- 宝石图鉴（服务端下发的权威表） -->
      <view class="card stagger-in">
        <text class="card-title">宝石图鉴</text>
        <!-- 搜索（2026-11-01 新一轮第 75 轮）：按名称/属性过滤 -->
        <input
          v-model="keyword"
          class="search-inp"
          placeholder="搜索宝石名称 / 属性"
          placeholder-class="ph"
        />
        <text class="muted" style="display: block; margin-bottom: 12rpx">
          数值与描述直接来自服务端 /config。共 {{ ownedCount }} 颗已拥有。
        </text>
        <!-- G80 词条覆盖统计：已拥有宝石的词条构成，看养成偏向 -->
        <view v-if="affixCoverage.length" class="affix-coverage">
          <text class="dim" style="display: block; margin-bottom: 8rpx">已拥有词条（颗数 / 最高值）</text>
          <view class="affix-chips">
            <text v-for="a in affixCoverage" :key="a.key" class="affix-chip">
              {{ a.name }} ×{{ a.count }} · 最高 {{ a.best }}
            </text>
          </view>
        </view>

        <!-- G130 词条数值区间 -->
        <view v-if="affixRange.length" class="affix-range">
          <text class="dim" style="display: block; margin-bottom: 8rpx">词条数值（最低 / 均值 / 最高）</text>
          <view v-for="a in affixRange" :key="a.key" class="ar-row">
            <text class="ar-name">{{ a.name }}</text>
            <text class="mono ar-val">{{ a.min }} / {{ a.avg }} / {{ a.max }}</text>
          </view>
        </view>

        <!-- G107 持有宝石品质分布环 -->
        <view v-if="ownedCount > 0" class="quality-donut-wrap">
          <svg class="quality-donut" viewBox="0 0 100 100" aria-label="宝石品质分布">
            <circle class="qd-bg" cx="50" cy="50" r="40" />
            <circle
              v-for="seg in qualityDonut"
              :key="seg.key"
              class="qd-seg" cx="50" cy="50" r="40"
              :stroke="seg.color"
              :stroke-dasharray="`${seg.dash} ${251.3 - seg.dash}`"
              :stroke-dashoffset="seg.offset"
            />
          </svg>
          <view class="quality-legend">
            <text v-for="seg in qualityDonut" :key="'q' + seg.key" class="ql-item">
              <text class="ql-dot" :style="{ background: seg.color }" />{{ seg.label }} {{ seg.count }}
            </text>
          </view>
        </view>

        <text class="result-count">显示 {{ filteredGems.length }} / 共 {{ gems.length }} 条</text>
        <view class="r3-data-grid">
        <view v-for="g in filteredGems" :key="g.id" class="rxn gem list-row">
          <view class="rxn-head">
            <!-- 宝石切面图标：颜色按属性映射（共享表见 utils/labels.gemAttrBadge） -->
            <view class="icon-badge" :class="gemAttrBadge(g.attr)" style="width: 64rpx; height: 64rpx">
              <DefIcon name="gem" :size="40" />
            </view>
            <text class="rxn-name">{{ g.name }}</text>
            <!-- 第六批 R44：展示 code，便于按代码检索/与后台对照 -->
            <text class="dim mono">#{{ g.code }}</text>
            <text class="dim">{{ attrLabel(g.attr) }}</text>
            <text v-if="ownedCountOf(g.id) > 0" class="chip hit">已拥有 ×{{ ownedCountOf(g.id) }}</text>
            <!-- 第六批 R43：已拥有实例的最佳品质（背包有品质，图鉴此前看不到） -->
            <text v-if="ownedCountOf(g.id) > 0" class="dim">最佳 {{ qualityName(bestQualityOf(g.id)) }}</text>
          </view>
          <text class="rxn-descr">{{ g.descr || '（服务端未提供说明）' }}</text>
        </view>
        </view>
        <text v-if="!gems.length" class="muted">宝石表为空（配置未加载？）</text>
        <!-- R27 搜索无结果：明确区分「表是空的」与「关键词没匹配上」 -->
        <view v-if="keyword && gems.length && filteredGems.length === 0" class="search-empty">
          <text class="muted">没有匹配「{{ keyword }}」的宝石</text>
          <text class="clear-search" @click="keyword = ''">清除搜索</text>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { computeGrowthPriority, resolveR12Cta } from '@/pages/_shared/r12GrowthPriority'
import { computed, onMounted, ref } from 'vue'
import { onLoad, onShow } from '@dcloudio/uni-app'
import { useGameStore } from '@/store/game'
import { fetchGems, type OwnedGem } from '@/api/client'
import { normalizeGemGain, r10GemGainCacheKey, type GemGainView } from '@/pages/_shared/r10GemGain'
import { gemAttrBadge, affixName, qualityName as labelQualityName } from '@/utils/labels'
import DefIcon from '@/components/DefIcon.vue'
import { fetchLatestBattleCausal, normalizeBattleCausal, type BattleCausalView } from '@/pages/_shared/r7GameplayCausal'

const store = useGameStore()
// R12 跨系统成长优先级：口径与首页 aiInsight 一致，四页指向同一短板
const selfRoute = '/pages/gems/gems'
const r12Priority = computed(() => computeGrowthPriority(store.buildRating, store.config?.rating_weights))
const r12Cta = computed(() => resolveR12Cta(r12Priority.value, selfRoute))
function r12Go() {
  const p = r12Priority.value
  if (!p) return
  if (r12Cta.value.self) {
    uni.pageScrollTo({ selector: '.r12-priority', duration: 200 })
    return
  }
  if (p.hint) uni.navigateTo({ url: p.hint })
}

/** R10 §1I.3：图鉴只读背包真实写后缓存的服务端属性差分；无缓存不虚构收益。 */
const gemGain = ref<GemGainView>(normalizeGemGain(null))
function loadGemGain() {
  let raw: string | null = null
  try {
    const prefix = r10GemGainCacheKey(store.userId, 0).replace(/:0$/, ':')
    for (let i = 0; i < localStorage.length; i++) {
      const k = localStorage.key(i)
      if (k && k.startsWith(prefix)) { raw = localStorage.getItem(k); break }
    }
  } catch { raw = null }
  gemGain.value = normalizeGemGain(raw ? JSON.parse(raw) : null)
}
loadGemGain()
onShow(() => { loadGemGain() })

/** R11：佩戴差分数值统一格式化，避免长浮点尾数；不改差分本身。 */
function fmt(n: number | null | undefined): string {
  return typeof n === 'number' && Number.isFinite(n) ? n.toFixed(1) : '—'
}
function fmtSigned(n: number | null | undefined): string {
  return typeof n === 'number' && Number.isFinite(n) ? (n >= 0 ? '+' : '') + n.toFixed(1) : '—'
}

/** 宝石图鉴展示所需的字段形状（服务端 /config.gems 元素）。 */
interface GemDef {
  id: number
  code: string
  name: string
  attr: string
  descr: string
}

/**
 * 宝石图鉴（2026-10-31 新一轮第 27 轮）。
 *
 * `/config.gems` 从第一天起就下发了 8 种宝石的属性与说明，但客户端只在
 * 背包展示**已拥有**的宝石实例 —— 玩家看不到全部宝石的成长路径。
 * 这里补上图鉴页，并标注每种宝石的拥有数量。
 */
const gems = computed(() => (store.config?.gems ?? []) as unknown as GemDef[])

/** 图鉴搜索（2026-11-01 新一轮第 75 轮）：名称/属性中文过滤。 */
const keyword = ref('')

const filteredGems = computed(() => {
  const kw = keyword.value.trim().toLowerCase()
  if (!kw) return gems.value
  return gems.value.filter((g) => {
    return g.name.toLowerCase().includes(kw) || attrLabel(g.attr).toLowerCase().includes(kw)
  })
})

/** 已拥有的宝石实例列表（/me/gems）。 */
const ownedGems = ref<OwnedGem[]>([])

const ownedCount = computed(() => ownedGems.value.length)

/** G130 词条数值区间：每种词条的 min/max/均值，判断洗到多少算好。 */
const affixRange = computed(() => {
  const m = new Map<string, number[]>()
  for (const g of ownedGems.value) {
    for (const a of g.affixes ?? []) {
      const arr = m.get(a.affix) ?? []
      arr.push(a.value)
      m.set(a.affix, arr)
    }
  }
  return [...m.entries()].map(([key, vals]) => ({
    key,
    name: affixName(key),
    min: Math.min(...vals),
    max: Math.max(...vals),
    avg: Math.round(vals.reduce((s, v) => s + v, 0) / vals.length),
  })).sort((a, b) => b.avg - a.avg)
})

/** G107 持有宝石品质分布环：五档品质按颗数切环（数据来自 ownedGems[].quality）。 */
const QUALITY_COLOR: Record<string, string> = {
  gray: '#8b949e', blue: '#58a6ff', purple: '#c9a7ff', red: '#ff6b35', gold: '#ffd700',
}
const qualityDonut = computed(() => {
  const defs = QUALITY_ORDER.map((q) => ({
    key: q, label: qualityName(q), color: QUALITY_COLOR[q]!,
    count: ownedGems.value.filter((g) => g.quality === q).length,
  }))
  const total = defs.reduce((s, d) => s + d.count, 0)
  const C = 2 * Math.PI * 40
  let acc = 0
  return defs.map((d) => {
    const frac = total > 0 ? d.count / total : 0
    const seg = { ...d, pct: Math.round(frac * 100), dash: frac * C, offset: -acc * C }
    acc += frac
    return seg
  })
})

/* -------------------------------------------------------------------------- */
/* G80 宝石词条覆盖统计（第七批）                                               */
/* 聚合已拥有宝石的全部词条：每种词条出现几颗、最高值多少 —— 看「我的词条库偏向」。 */
/* -------------------------------------------------------------------------- */
const affixCoverage = computed(() => {
  const m = new Map<string, { key: string; name: string; count: number; best: number }>()
  for (const g of ownedGems.value) {
    for (const a of g.affixes ?? []) {
      const cur = m.get(a.affix) ?? { key: a.affix, name: affixName(a.affix), count: 0, best: 0 }
      cur.count++
      cur.best = Math.max(cur.best, a.value)
      m.set(a.affix, cur)
    }
  }
  return [...m.values()].sort((a, b) => b.count - a.count)
})

function ownedCountOf(gemId: number): number {
  return ownedGems.value.filter((g) => g.gem_id === gemId).length
}

/** 第六批 R43：已拥有实例里的最佳品质。 */
const QUALITY_ORDER = ['gray', 'blue', 'purple', 'red', 'gold']
function bestQualityOf(gemId: number): string {
  const owned = ownedGems.value.filter((g) => g.gem_id === gemId)
  if (!owned.length) return 'gray'
  return owned.reduce((best, g) =>
    QUALITY_ORDER.indexOf(g.quality) > QUALITY_ORDER.indexOf(best) ? g.quality : best,
  'gray')
}

/**
 * 图鉴译名统一走共享表 utils/labels —— 与背包页（bag.vue）同源。
 * 此前这里各写了一份：gold 显示「传说」而背包显示「金」、crit 显示「暴击率」
 * 而背包显示「暴击」，同一颗宝石在两个页面像两种东西。
 * 保留包装名（qualityName / attrLabel）而不改模板，是为了让依赖这些标识符
 * 的源码守卫（gems_search_feature.test.ts）继续成立。
 */
function qualityName(q: string): string {
  return labelQualityName(q)
}

function attrLabel(a: string): string {
  return affixName(a)
}

async function refreshOwned(): Promise<void> {
  try {
    const res = await fetchGems()
    ownedGems.value = res.gems ?? []
  } catch {
    // 图鉴的拥有状态是辅助信息，请求出错不打扰主流程
  }
}

let pageLoaded = false
let loadInflight: Promise<void> | null = null

async function doLoadPage(): Promise<void> {
  if (!store.config) {
    // 深链接/刷新直入本页时没有任何调用方拉过 /config：
    // 不做这一步，页面会把「尚未加载」误报成「宝石表为空（配置未加载？）」。
    if (!store.loggedIn) {
      const ok = await store.login()
      if (!ok) return
    }
    try {
      await store.loadConfig()
    } catch {
      return
    }
  }
  if (!store.loggedIn) return
  await refreshOwned()
  pageLoaded = true
}

/** onLoad/onMounted/onShow 可能同时到达；统一走一条幂等加载路径。 */
function loadPage(): Promise<void> {
  if (pageLoaded) return Promise.resolve()
  if (!loadInflight) loadInflight = doLoadPage().finally(() => { loadInflight = null })
  return loadInflight
}

onLoad(() => { void loadPage() })
onMounted(() => { void loadPage() })
onShow(() => {
  // 首次展示由 onLoad/onMounted 统一加载；从背包返回时只刷新持有量。
  if (pageLoaded) void refreshOwned()
  else void loadPage()
})

/** R3 AI 线索：以已拥有数 + 最缺宝石给一条确定性收集建议，不编造概率。 */
const r7Causal = ref<BattleCausalView>(normalizeBattleCausal(null))
void fetchLatestBattleCausal().then((v) => { r7Causal.value = v })

const r3GemPct = computed(() => {
  const total = gems.value.length
  return total ? Math.round(ownedCount.value / total * 100) : 0
})
const r3AiGem = computed(() => {
  const list = gems.value
  if (!list.length) return null
  const missing = list.filter((g) => ownedCountOf(g.id) === 0)
  const pick = missing[0] ?? list[0]
  return {
    conclusion: '「' + pick.name + '」还没拥有（' + attrLabel(pick.attr) + '）：当前 ' + ownedCount.value + '/' + list.length + '，优先补它',
    basis: '图鉴 ' + list.length + ' 种 × 已拥有 ' + ownedCount.value + ' 颗，缺口 ' + missing.length + ' 种',
  }
})
function r3GoShop() {
  uni.navigateTo({ url: '/pages/shop/shop' })
}
function r3GoBag() {
  // 宝石图鉴只做收集线索；真正的镶嵌入口在背包宝石页签。
  uni.navigateTo({ url: '/pages/bag/bag' })
}
</script>

<style scoped>
/* R10 图鉴与镶嵌是两个入口，次按钮与主按钮并列但弱化 */
.r3-ai-cta.secondary { margin-left: 16rpx; }

/* G130 词条区间 */
.ar-row { display: flex; flex-direction: row; justify-content: space-between; padding: 6rpx 0; border-bottom: 1rpx solid var(--border-soft); }
.ar-name { font-size: 24rpx; }
.ar-val { font-size: 22rpx; color: var(--ok); }

/* G107 品质分布环 */
.quality-donut-wrap { display: flex; align-items: center; gap: 20rpx; margin: 8rpx 0 16rpx; }
.quality-donut { width: 150rpx; height: 150rpx; transform: rotate(-90deg); }
.qd-bg { fill: none; stroke: var(--panel-3); stroke-width: 12; }
.qd-seg { fill: none; stroke-width: 12; }
.quality-legend { display: flex; flex-wrap: wrap; gap: 8rpx; flex: 1; }
.ql-item { font-size: 22rpx; color: var(--muted); }
.ql-dot { display: inline-block; width: 14rpx; height: 14rpx; border-radius: 50%; margin-right: 6rpx; }

/* G80 词条覆盖统计 */
.affix-coverage { margin: 8rpx 0 16rpx; }
.affix-chips { display: flex; flex-wrap: wrap; gap: 8rpx; }
.affix-chip {
  background: var(--panel-2);
  border: 1rpx solid var(--border-soft);
  border-radius: 8rpx;
  padding: 6rpx 12rpx;
  font-size: 22rpx;
  color: var(--text-2);
}
/* 条目分隔线走共享的 .rxn（styles/common.css），本页无需额外样式 */
</style>