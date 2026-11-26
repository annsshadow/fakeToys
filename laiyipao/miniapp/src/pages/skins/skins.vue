<template>
  <view class="page-root">
    <view class="pad safe-bottom">
      <!-- R4：指标带 → 流派筛选 → 数据格 → 智能空态 -->
      <view v-if="skins.length" class="r3-kpi r4-kpi r4-enter">
        <view class="r3-kpi-main">
          <text class="r3-kpi-num">{{ r4AiSkin ? r4AiSkin.element : '图鉴' }} 流派</text>
          <text class="r3-kpi-label">按当前构筑元素推荐被动</text>
          <text class="r3-kpi-sub">传说 {{ r4AiSkin ? r4AiSkin.rarityPct : 0 }}% · 共 {{ skins.length }} 款</text>
        </view>
        <view class="r3-kpi-cell"><text class="r3-kpi-num">{{ passiveElementMap.length }}</text><text class="r3-kpi-label">元素流派</text></view>
        <view class="r3-kpi-cell"><text class="r3-kpi-num">{{ filteredSkins.length }}</text><text class="r3-kpi-label">当前筛选</text></view>
      </view>
      <view v-if="r4AiSkin" class="r3-ai r4-ai r4-enter" aria-label="AI 皮肤流派提示">
        <view class="r3-ai-head">
          <text class="r3-ai-badge">AI</text>
          <text class="r3-ai-title">AI 皮肤流派提示</text>
          <text class="r3-ai-engine">规则引擎 · 基于被动映射</text>
        </view>
        <text class="r3-ai-conclusion">{{ r4AiSkin.conclusion }}</text>
        <text class="r3-ai-basis">{{ r4AiSkin.basis }}</text>
        <view class="r3-ai-cta" @click="r4FocusSkin()">查看同流派皮肤 →</view>
      </view>
      <!-- 加载中 / 空态（与其它图鉴同款）-->
      <view v-if="store.configLoading" class="card">
        <text class="muted">加载中…</text>
      </view>
      <view v-else-if="skins.length === 0" class="r3-empty r4-empty" aria-live="polite" role="status">
        <text class="r3-empty-slot">AI</text>
        <text class="r3-empty-title">皮肤表还没加载出来</text>
        <text class="r3-empty-reason">被动映射来自服务端 /config；配置未到位时不会推荐流派。</text>
        <view class="r3-empty-action" @click="loadPage()">重新加载配置</view>
      </view>

      <!-- 皮肤图鉴（服务端下发的权威表） -->
      <view class="card stagger-in">
        <text class="card-title">皮肤图鉴</text>
        <!-- 搜索（2026-11-01 新一轮第 78 轮）：按名称/稀有度过滤 -->
        <input
          v-model="keyword"
          v-aria-native="'搜索皮肤名称 / 稀有度'"
          aria-label="搜索皮肤名称 / 稀有度"
          class="search-inp"
          placeholder="搜索皮肤名称 / 稀有度"
          placeholder-class="ph"
        />
        <text class="muted" style="display: block; margin-bottom: 12rpx">
          数值与说明直接来自服务端 /config。皮肤被动是<strong>机制型词条</strong>，
          改变战斗方式而非只加数值。
        </text>
        <!-- 第六批 R49：稀有度构成一眼可见，搜索前就知道有哪些档次 -->
        <!-- G75 稀有度分布环图：四段弧按数量占比，构成一眼可见 -->
        <view class="donut-wrap">
          <svg class="donut-svg" viewBox="0 0 100 100" aria-label="稀有度分布环图">
            <circle class="donut-bg" cx="50" cy="50" r="40" />
            <circle
              v-for="seg in rarityDonut"
              :key="seg.key"
              class="donut-seg"
              cx="50" cy="50" r="40"
              :stroke="seg.color"
              :stroke-dasharray="`${seg.dash} ${251.3 - seg.dash}`"
              :stroke-dashoffset="seg.offset"
            />
          </svg>
          <view class="donut-legend">
            <text v-for="seg in rarityDonut" :key="'lg' + seg.key" class="donut-lg">
              <text class="donut-dot" :style="{ background: seg.color }" />{{ seg.label }} {{ seg.pct }}%
            </text>
          </view>
        </view>
        <!-- R4 FilterBar：按稀有度快速筛选，活动态用 soft 底 + ink 字 -->
        <view class="r3-filter r4-filter">
          <text
            class="r3-chip"
            :class="{ active: !keyword }"
            @click="keyword = ''"
          >全部</text>
          <text
            v-for="r in RARITIES"
            :key="'f' + r"
            class="r3-chip"
            :class="{ active: keyword === rarityLabel(r) }"
            @click="keyword = rarityLabel(r)"
          >{{ rarityLabel(r) }}</text>
        </view>
        <!-- 第六批 R50：与背包口径一致，明说当前版本无获得途径，避免误以为能抽到 -->
        <!-- G133 被动→元素流派映射 -->
        <view v-if="passiveElementMap.length" class="pe-map">
          <text class="dim" style="display: block; margin-bottom: 8rpx">被动对应元素流派</text>
          <text v-for="p in passiveElementMap" :key="p.el" class="pe-chip">{{ p.name }} · {{ p.count }} 款</text>
        </view>
        <!-- G121 稀有度×被动矩阵 -->
        <view v-if="passiveKinds.length" class="rp-matrix">
          <text class="dim" style="display: block; margin-bottom: 8rpx">稀有度 × 被动机制（款数）</text>
          <view v-for="r in rarityPassiveMatrix" :key="r.rarity" class="rpm-row">
            <text class="dim rpm-label">{{ r.label }}</text>
            <text v-for="(c, i) in r.cells" :key="i" class="rpm-cell" :class="{ on: c.n > 0 }">{{ c.n }}</text>
          </view>
        </view>
        <!-- G90 被动机制分类：有哪几类皮肤机制、各几款 -->
        <view v-if="passiveGroups.length" class="passive-groups">
          <text class="dim" style="display: block; margin-bottom: 8rpx">被动机制分类</text>
          <text v-for="g in passiveGroups" :key="g.key" class="passive-chip">{{ g.name }} ×{{ g.n }}</text>
        </view>
        <text class="dim" style="display: block; margin-bottom: 8rpx">
          当前版本皮肤暂无获得途径，图鉴用于了解被动机制。
        </text>
        <text class="result-count">显示 {{ filteredSkins.length }} / 共 {{ skins.length }} 条</text>
        <view class="r3-data-grid r4-grid">
        <view v-for="s in filteredSkins" :key="s.id" class="rxn skin list-row">
          <view class="rxn-head">
            <!-- 皮肤图标座：稀有度决定底座配色（传说带金色描边） -->
            <view class="icon-badge" :class="rarityBadgeClass(s.rarity)" style="width: 64rpx; height: 64rpx">
              <DefIcon name="skin" :size="40" />
            </view>
            <text class="rxn-name">{{ s.name }}</text>
            <text class="rarity-chip" :class="'rarity-' + (s.rarity in RARITY_CLASS ? s.rarity : 'common')">
              {{ rarityLabel(s.rarity) }}
            </text>
          </view>
          <text class="rxn-descr">{{ s.descr || '（服务端未提供说明）' }}</text>
          <view class="rxn-stats">
            <text class="stat">被动：{{ passiveLabel(s.passive) }}</text>
          </view>
        </view>
        </view>
        <text v-if="!skins.length" class="muted">皮肤表为空（配置未加载？）</text>
        <!-- R26 搜索无结果：明确区分「表是空的」与「关键词没匹配上」 -->
        <view v-if="keyword && skins.length && filteredSkins.length === 0" class="search-empty">
          <text class="muted search-empty-text">没有匹配「{{ keyword }}」的皮肤</text>
          <text class="clear-search" @click="keyword = ''">清除搜索</text>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { onLoad, onShow } from '@dcloudio/uni-app'
import { useGameStore } from '@/store/game'
import DefIcon from '@/components/DefIcon.vue'
import { ELEMENT_NAME, type Element } from '@/game/elements'

const store = useGameStore()

/** 皮肤图鉴展示所需的字段形状（服务端 /config.skins 元素）。 */
interface SkinDef {
  id: number
  code: string
  name: string
  rarity: string
  passive: string
  descr: string
}

/**
 * 皮肤图鉴（2026-10-31 新一轮第 35 轮）。
 *
 * `/config.skins` 从第一天起就下发了 6 款皮肤的稀有度/被动/说明，但
 * 客户端**零 UI 消费** —— 玩家不知道皮肤系统长什么样。这里补上图鉴页。
 */
const skins = computed(() => (store.config?.skins ?? []) as unknown as SkinDef[])

/** 第六批 R49：按稀有度统计，未知稀有度归入普通。 */
function countByRarity(rarity: string): number {
  return skins.value.filter((s) => (s.rarity in RARITY_CLASS ? s.rarity : 'common') === rarity).length
}

/* -------------------------------------------------------------------------- */
/* G75 稀有度分布环图（第七批）                                                 */
/* 四档稀有度按数量占比切成环形段（SVG stroke-dasharray），比纯文本更直观地看构成。 */
/* -------------------------------------------------------------------------- */
const rarityDonut = computed(() => {
  const defs = [
    { key: 'legendary', label: '传说', color: '#ffd700', count: countByRarity('legendary') },
    { key: 'epic', label: '史诗', color: '#c9a7ff', count: countByRarity('epic') },
    { key: 'rare', label: '稀有', color: '#58a6ff', count: countByRarity('rare') },
    { key: 'common', label: '普通', color: '#8b949e', count: countByRarity('common') },
  ]
  const total = defs.reduce((s, d) => s + d.count, 0)
  const C = 2 * Math.PI * 40 // 半径 40 的周长
  let acc = 0
  return defs.map((d) => {
    const frac = total > 0 ? d.count / total : 0
    const seg = { ...d, pct: Math.round(frac * 100), dash: frac * C, offset: -acc * C }
    acc += frac
    return seg
  })
})

/** 图鉴搜索（2026-11-01 新一轮第 78 轮）：名称/稀有度中文过滤。 */
const keyword = ref('')

const filteredSkins = computed(() => {
  const kw = keyword.value.trim().toLowerCase()
  if (!kw) return skins.value
  return skins.value.filter((s) => {
    return s.name.toLowerCase().includes(kw) || rarityLabel(s.rarity).toLowerCase().includes(kw)
  })
})

function rarityLabel(r: string): string {
  const map: Record<string, string> = { rare: '稀有', epic: '史诗', legendary: '传说' }
  return map[r] ?? r
}

/** 稀有度 → 图标底座配色；未知稀有度回退中性。 */
function rarityBadgeClass(r: string): string {
  const map: Record<string, string> = {
    rare: 'badge-rare',
    epic: 'badge-epic',
    legendary: 'badge-legendary',
  }
  return map[r] ?? ''
}

/** 模板里 rarity-chip 类名的白名单校验（未知稀有度走 common 中性样式）。 */
const RARITY_CLASS: Record<string, true> = { rare: true, epic: true, legendary: true, common: true }

/* -------------------------------------------------------------------------- */
/* G90 被动机制分类统计（第七批）                                               */
/* 按 passive 类型聚合皮肤数量，看「这游戏有哪几类皮肤机制」。数据来自 skins。     */
/* -------------------------------------------------------------------------- */
/** G133 被动→元素流派映射：把皮肤被动 key 对上五元素，理解该往哪方向配装。 */
const passiveElementMap = computed(() => {
  const KEY_EL: Record<string, string> = {
    fire_burn: 'fire', ice_freeze: 'ice', volt_chain: 'lightning',
    blight_spread: 'corrosion', kinetic_bonus: 'kinetic', boss_breaker: 'kinetic',
  }
  const m = new Map<string, string[]>()
  for (const s of skins.value) {
    const el = KEY_EL[s.passive]
    if (!el) continue
    const arr = m.get(el) ?? []
    arr.push(s.name)
    m.set(el, arr)
  }
  return [...m.entries()].map(([el, names]) => ({
    el, name: ELEMENT_NAME[el as Element] ?? el, count: names.length,
  }))
})

/** R4：被动流派 × 当前出战元素 → 一句推荐；配置/构筑任一缺席就保持空态。 */
const r4AiSkin = computed(() => {
  const map = passiveElementMap.value
  if (!map.length) return null
  const equipped = store.equippedElements ?? []
  const target = equipped.find((e) => map.some((m) => m.el === e)) ?? map[0]!.el
  const row = map.find((m) => m.el === target)
  if (!row) return null
  return {
    element: row.name,
    rarityPct: rarityDonut.value.find((s) => s.key === 'legendary')?.pct ?? 0,
    conclusion: `当前构筑偏${row.name}，建议关注${row.name}流派皮肤（${row.count} 款）`,
    basis: '依据：被动→元素流派映射 / 当前出战元素 · 更新于本次进入',
    keyword: row.name,
  }
})

function r4FocusSkin() {
  keyword.value = r4AiSkin.value?.keyword ?? ''
  uni.pageScrollTo({ selector: '.pe-map', duration: 200 })
}

/** G121 稀有度×被动机制矩阵：行=稀有度，列=被动类型，格内款数。 */
const RARITIES = ['legendary', 'epic', 'rare', 'common']
const passiveKinds = computed(() => [...new Set(skins.value.map((s) => s.passive))])
const rarityPassiveMatrix = computed(() =>
  RARITIES.map((r) => ({
    rarity: r,
    label: rarityLabel(r),
    cells: passiveKinds.value.map((p) => ({
      n: skins.value.filter((s) =>
        (s.rarity in RARITY_CLASS ? s.rarity : 'common') === r && s.passive === p).length,
    })),
  })),
)

const passiveGroups = computed(() => {
  const m = new Map<string, number>()
  for (const s of skins.value) {
    m.set(s.passive, (m.get(s.passive) ?? 0) + 1)
  }
  return [...m.entries()].map(([key, n]) => ({ key, name: passiveLabel(key), n }))
})

function passiveLabel(p: string): string {
  const map: Record<string, string> = {
    kinetic_bonus: '动能加成（额外破甲击退）',
    fire_burn: '燃烧（焰系命中附加一层燃烧）',
    ice_freeze: '冻结（冰系 20% 概率冻结）',
    volt_chain: '弹射 +1（电系）',
    blight_spread: '传播（毒系命中向周围传播 1 层）',
    boss_breaker: 'BOSS 特攻（反应伤害 +30%）',
  }
  return map[p] ?? p
}
/* -------------------------------------------------------------------------- */
/* R3 冷启动配置加载：深链接/刷新直入时自行拉 /config                             */
/* -------------------------------------------------------------------------- */
let pageLoaded = false
let loadInflight: Promise<void> | null = null

async function doLoadPage(): Promise<void> {
  if (!store.config) {
    // 此前本页没有任何加载调用，把「尚未加载」谎报成「皮肤表为空（配置未加载？）」。
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
onShow(() => { void loadPage() })
</script>

<style scoped>
/* G133 被动→元素映射 */
.pe-map { margin: 8rpx 0; }
.pe-chip { display: inline-block; background: var(--ice-soft); color: var(--ice); border-radius: 8rpx; padding: 4rpx 12rpx; font-size: 22rpx; margin: 0 8rpx 8rpx 0; }

/* G121 稀有度×被动矩阵 */
.rpm-row { display: flex; flex-direction: row; gap: 6rpx; margin: 6rpx 0; align-items: center; }
.rpm-label { width: 100rpx; flex-shrink: 0; font-size: 22rpx; }
.rpm-cell { flex: 1; text-align: center; font-size: 22rpx; padding: 6rpx 0; background: var(--panel-2); border-radius: 6rpx; color: var(--dim); }
.rpm-cell.on { background: var(--legendary-soft); color: var(--legendary); }

/* G90 被动机制分类 */
.passive-groups { margin: 8rpx 0; }
.passive-chip {
  display: inline-block;
  background: var(--kinetic-soft);
  color: var(--kinetic);
  border-radius: 8rpx;
  padding: 4rpx 12rpx;
  font-size: 22rpx;
  margin: 0 8rpx 8rpx 0;
}

/* G75 稀有度分布环图 */
.donut-wrap { display: flex; align-items: center; gap: 24rpx; margin: 8rpx 0 16rpx; }
.donut-svg { width: 160rpx; height: 160rpx; transform: rotate(-90deg); }
.donut-bg { fill: none; stroke: var(--panel-3); stroke-width: 12; }
.donut-seg { fill: none; stroke-width: 12; }
.donut-legend { display: flex; flex-direction: column; gap: 8rpx; flex: 1; }
.donut-lg { font-size: 22rpx; color: var(--muted); }
.donut-dot {
  display: inline-block;
  width: 16rpx; height: 16rpx;
  border-radius: 50%;
  margin-right: 8rpx;
}
/* 条目分隔线走共享的 .rxn（styles/common.css） */
/* R68 传说稀有度 chip 微光，强化「稀有」价值感 */
.rarity-chip.rarity-legendary {
  animation: rarity-shine 2.4s ease-in-out infinite;
}

@keyframes rarity-shine {
  0%, 100% { box-shadow: 0 0 0 rgba(255, 215, 0, 0); }
  50% { box-shadow: 0 0 12rpx rgba(255, 215, 0, 0.5); }
}
</style>