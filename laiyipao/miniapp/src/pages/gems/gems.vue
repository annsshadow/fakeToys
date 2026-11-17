<template>
  <view class="page-root">
    <view class="pad safe-bottom">
      <!-- 加载中 / 空态（与其它图鉴同款）-->
      <view v-if="store.configLoading" class="card">
        <text class="muted">加载中…</text>
      </view>
      <view v-else-if="gems.length === 0" class="card">
        <text class="muted">宝石表还没加载出来</text>
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
        <view v-for="g in filteredGems" :key="g.id" class="rxn gem">
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
import { computed, ref } from 'vue'
import { onShow } from '@dcloudio/uni-app'
import { useGameStore } from '@/store/game'
import { fetchGems, type OwnedGem } from '@/api/client'
import { gemAttrBadge, affixName, qualityName as labelQualityName } from '@/utils/labels'
import DefIcon from '@/components/DefIcon.vue'

const store = useGameStore()

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

onShow(() => {
  if (!store.loggedIn) return
  fetchGems()
    .then((res) => {
      ownedGems.value = res.gems ?? []
    })
    .catch(() => {
      // 图鉴的拥有状态是辅助信息，加载失败不打扰主流程
    })
})
</script>

<style scoped>
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