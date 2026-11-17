<template>
  <view class="page-root">
    <view class="pad safe-bottom">
      <!-- 加载中 / 空态（与其它图鉴同款）-->
      <view v-if="store.configLoading" class="card">
        <text class="muted">加载中…</text>
      </view>
      <view v-else-if="skins.length === 0" class="card">
        <text class="muted">皮肤表还没加载出来</text>
      </view>

      <!-- 皮肤图鉴（服务端下发的权威表） -->
      <view class="card stagger-in">
        <text class="card-title">皮肤图鉴</text>
        <!-- 搜索（2026-11-01 新一轮第 78 轮）：按名称/稀有度过滤 -->
        <input
          v-model="keyword"
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
        <!-- 第六批 R50：与背包口径一致，明说当前版本无获得途径，避免误以为能抽到 -->
        <!-- G90 被动机制分类：有哪几类皮肤机制、各几款 -->
        <view v-if="passiveGroups.length" class="passive-groups">
          <text class="dim" style="display: block; margin-bottom: 8rpx">被动机制分类</text>
          <text v-for="g in passiveGroups" :key="g.key" class="passive-chip">{{ g.name }} ×{{ g.n }}</text>
        </view>
        <text class="dim" style="display: block; margin-bottom: 8rpx">
          当前版本皮肤暂无获得途径，图鉴用于了解被动机制。
        </text>
        <text class="result-count">显示 {{ filteredSkins.length }} / 共 {{ skins.length }} 条</text>
        <view v-for="s in filteredSkins" :key="s.id" class="rxn skin">
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
        <text v-if="!skins.length" class="muted">皮肤表为空（配置未加载？）</text>
        <!-- R26 搜索无结果：明确区分「表是空的」与「关键词没匹配上」 -->
        <view v-if="keyword && skins.length && filteredSkins.length === 0" class="search-empty">
          <text class="muted">没有匹配「{{ keyword }}」的皮肤</text>
          <text class="clear-search" @click="keyword = ''">清除搜索</text>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { useGameStore } from '@/store/game'
import DefIcon from '@/components/DefIcon.vue'

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
</script>

<style scoped>
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