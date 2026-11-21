<template>
  <view class="page-root">
    <view class="pad safe-bottom">
      <view class="section-kicker section-kicker--ai r3-enter">
        <text class="section-kicker-cn">技能构筑台</text>
        <text class="section-kicker-en">SKILL LOADOUT</text>
      </view>
      <view class="r3-kpi r3-enter">
        <view class="r3-kpi-main">
          <text class="r3-kpi-num">{{ r3AiSkill ? r3AiSkill.name : '待配置' }}</text>
          <text class="r3-kpi-label">AI 推荐 · 效率榜首选</text>
          <text class="r3-kpi-sub">{{ r3AiSkill ? r3AiSkill.reason : '技能表尚未到达' }}</text>
        </view>
        <view class="r3-kpi-cell">
          <text class="r3-kpi-num">{{ ownedCount }}/{{ skills.length }}</text>
          <text class="r3-kpi-label">已解锁 / 技能总数</text>
        </view>
        <view class="r3-kpi-cell">
          <text class="r3-kpi-num">{{ compositeSkills.length }}</text>
          <text class="r3-kpi-label">合成技能</text>
        </view>
      </view>
      <view class="r3-filter" aria-label="按元素筛选技能">
        <text class="r3-chip" :class="{ active: r3Element === '' }" @click="r3Element = ''">全部 {{ skills.length }}</text>
        <text v-for="el in ELEMENT_ORDER" :key="'f' + el" class="r3-chip" :class="{ active: r3Element === el }" @click="r3Element = el">
          {{ ELEMENT_NAME[el] }}
        </text>
      </view>
      <view v-if="r3AiSkill" class="r3-ai r3-enter">
        <view class="r3-ai-head">
          <text class="r3-ai-badge">AI</text>
          <text class="r3-ai-title">AI 建议 · 取舍与热量</text>
          <text class="r3-ai-engine">规则引擎</text>
        </view>
        <text class="r3-ai-conclusion">{{ r3AiSkill.conclusion }}</text>
        <text class="r3-ai-basis">依据：{{ r3AiSkill.basis }}</text>
        <text class="r3-ai-cta" role="button" @click="r3GoBag">去背包查看解锁 →</text>
      </view>
      <!-- 加载中 / 空态（与敌人图鉴同款）-->
      <view v-if="store.configLoading" class="card">
        <text class="muted">加载中…</text>
      </view>
      <view v-else-if="skills.length === 0" class="card">
        <text class="muted">技能表还没加载出来</text>
      </view>

      <!-- 技能图鉴（服务端下发的权威表） -->
      <view class="card stagger-in">
        <text class="card-title">技能图鉴</text>
        <!-- 搜索（2026-11-01 新一轮第 71 轮）：按名称/元素/主动被动过滤 -->
        <input
          v-model="keyword"
          class="search-inp"
          placeholder="搜索技能名称 / 元素 / 主动被动"
          placeholder-class="ph"
        />
        <text class="muted" style="display: block; margin-bottom: 12rpx">
          数值直接来自服务端 /config，与游戏内实际生效的一致。
          {{ ownedCount }} / {{ skills.length }} 个已解锁（背包可查看拥有状态）。
        </text>
        <!-- G66 技能元素×类型覆盖矩阵：每格「已拥有/总数」，缺口一眼可见 -->
        <view class="card-title" style="font-size: 26rpx">元素 × 类型覆盖</view>
        <view class="skill-matrix">
          <view class="sm-row">
            <view class="sm-head" />
            <view v-for="k in kindCols" :key="'h' + k.key" class="sm-head">{{ k.label }}</view>
          </view>
          <view v-for="row in skillMatrix" :key="row.el" class="sm-row">
            <view class="sm-head" :style="{ color: row.color }">{{ ELEMENT_NAME[row.el] }}</view>
            <view
              v-for="cell in row.cells"
              :key="row.el + cell.key"
              class="sm-cell"
              :class="{ covered: cell.owned > 0 }"
            >
              {{ cell.owned }}/{{ cell.total }}
            </view>
          </view>
        </view>

        <text class="result-count">显示 {{ r3FilteredSkills.length }} / 共 {{ skills.length }} 条</text>
        <view class="r3-data-grid">
        <view v-for="s in r3FilteredSkills" :key="s.id" class="rxn skill list-row">
          <view class="rxn-head">
            <view class="icon-badge" :class="'badge-' + s.element" style="width: 64rpx; height: 64rpx">
              <DefIcon :name="'elem:' + s.element" :size="40" />
            </view>
            <text class="rxn-name">{{ s.name }}</text>
            <text class="tag" :class="'tag-' + s.element">{{ elementName(s.element) }}</text>
            <!-- R67 主动/被动配色区分：扫列表时不必读字 -->
            <text class="kind-tag" :class="s.kind === 'active' ? 'kind-active' : 'kind-passive'">
              {{ s.kind === 'active' ? '主动' : '被动' }}
            </text>
            <text v-if="isOwned(s.id)" class="chip hit">已拥有</text>
            <!-- 第六批 R46：未解锁技能明示状态，避免「在图鉴看到却拿不到」的困惑 -->
            <text v-else class="chip">未解锁</text>
          </view>
          <text class="rxn-descr">{{ s.descr || '（服务端未提供说明）' }}</text>
          <view class="rxn-stats">
            <text class="stat">基础伤害 {{ s.base_damage }}</text>
            <text v-if="s.heat_cost > 0" class="stat">热量 {{ s.heat_cost }}</text>
            <text v-if="s.cooldown_ms > 0" class="stat">
              冷却 {{ (s.cooldown_ms / 1000).toFixed(1) }}s
            </text>
            <text v-if="s.pierce > 1" class="stat">穿透 {{ s.pierce }}</text>
            <text v-if="s.aoe_radius > 0" class="stat">溅射 {{ s.aoe_radius }}</text>
            <text v-if="s.chain > 1" class="stat">连锁 {{ s.chain }}</text>
            <!-- 第六批 R45：明确「需 Lv.N」而非裸数字 -->
            <text class="stat">需 Lv.{{ s.unlock_level }}</text>
          </view>
        </view>
        </view>
        <text v-if="!skills.length" class="muted">技能表为空（配置未加载？）</text>
        <!-- R25 搜索无结果：明确区分「表是空的」与「关键词没匹配上」 -->
        <view v-if="keyword && skills.length && r3FilteredSkills.length === 0" class="search-empty">
          <text class="muted">没有匹配「{{ keyword }}」的技能</text>
          <text class="clear-search" @click="keyword = ''">清除搜索</text>
        </view>
      </view>

      <!-- G98 合成技能专区：此前图鉴完全缺失的熔炼产物 -->
      <view v-if="compositeSkills.length" class="card">
        <text class="card-title">合成技能<text class="dim">（熔炼产物）</text></text>
        <text class="muted" style="display: block; margin-bottom: 12rpx">
          这些高阶技能不在常规解锁列表里，由背包熔炼两个材料技能获得。
        </text>
        <view v-for="s in compositeSkills" :key="s.id" class="rxn composite">
          <view class="rxn-head">
            <text class="rxn-name">{{ s.name }}</text>
            <text class="tag" :class="'tag-' + s.element">{{ elementName(s.element) }}</text>
            <text v-if="s.tier" class="chip">{{ s.tier }} 阶</text>
          </view>
          <text class="dim">合成：{{ s.materialA }} + {{ s.materialB }}</text>
        </view>
      </view>

      <!-- G109 技能效率榜 -->
      <view v-if="efficiencyRank.byHeat.length" class="card">
        <text class="card-title">技能效率榜</text>
        <text class="dim" style="display: block; margin-bottom: 8rpx">每点热量伤害（热量是硬约束）</text>
        <view v-for="s in efficiencyRank.byHeat" :key="s.id" class="eff-row">
          <text class="eff-name">{{ s.name }}</text>
          <view class="eff-track"><view class="eff-fill" :style="{ width: s.pct + '%' }" /></view>
          <text class="mono eff-val">{{ s.v }}</text>
        </view>
        <text class="dim" style="display: block; margin: 12rpx 0 8rpx">每秒伤害 Top5</text>
        <text class="eff-time" v-for="(s, i) in efficiencyRank.byTime" :key="'t' + s.id">
          {{ i + 1 }}. {{ s.name }} · {{ s.v }}/s
        </text>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { onLoad, onShow } from '@dcloudio/uni-app'
import { useGameStore } from '@/store/game'
import { ELEMENT_NAME, ELEMENT_ORDER, ELEMENT_COLOR, type Element } from '@/game/elements'
import DefIcon from '@/components/DefIcon.vue'

const store = useGameStore()

/**
 * 技能图鉴（2026-10-31 新一轮第 25 轮）。
 *
 * `/config.skills` 从第一天起就下发了全部技能的伤害/热量/冷却/穿透/
 * 溅射/解锁等级，但客户端只在背包展示**已拥有**的技能 —— 玩家看不到
 * 未解锁技能的长什么样、解锁等级是多少，成长路径全靠猜。
 * 这里补上图鉴页，并标注已拥有状态。
 */
const skills = computed(() => store.config?.skills ?? [])

/* -------------------------------------------------------------------------- */
/* G98 合成技能专区（第七批）                                                   */
/* 图鉴此前只取 config.skills，18 个合成技能（composite_skills）根本看不到 ——     */
/* 这是图鉴最大的内容缺口。这里补上：产物 + 由哪两个材料熔炼而来。                 */
/* 数据来自 config.composite_skills / recipes，纯客户端派生。                     */
/* -------------------------------------------------------------------------- */
/** G109 技能效率榜：伤害/热量 与 伤害/秒，帮玩家在热量硬约束下选技能。 */
const efficiencyRank = computed(() => {
  const rank = (fn: (s: any) => number) =>
    [...skills.value]
      .filter((s: any) => fn(s) > 0)
      .sort((a: any, b: any) => fn(b) - fn(a))
      .slice(0, 5)
  const byHeat = rank((s) => s.base_damage / Math.max(1, s.heat_cost))
  const byTime = rank((s) => s.base_damage / Math.max(0.1, s.cooldown_ms / 1000))
  const maxHeat = Math.max(1, ...byHeat.map((s: any) => s.base_damage / Math.max(1, s.heat_cost)))
  return {
    byHeat: byHeat.map((s: any) => ({
      id: s.id, name: s.name,
      v: (s.base_damage / Math.max(1, s.heat_cost)).toFixed(0),
      pct: Math.round((s.base_damage / Math.max(1, s.heat_cost)) / maxHeat * 100),
    })),
    byTime: byTime.map((s: any) => ({
      id: s.id, name: s.name,
      v: (s.base_damage / Math.max(0.1, s.cooldown_ms / 1000)).toFixed(0),
    })),
  }
})

const allSkillDefs = computed(() => [
  ...(store.config?.skills ?? []),
  ...(store.config?.composite_skills ?? []),
])
const compositeSkills = computed(() => {
  const byId = new Map(allSkillDefs.value.map((s) => [s.id, s]))
  const recipes = store.config?.recipes ?? []
  const composites = store.config?.composite_skills ?? []
  return composites.map((s) => {
    const recipe = recipes.find((r) => r.output === s.id)
    return {
      id: s.id,
      name: s.name,
      element: s.element,
      kind: s.kind,
      tier: recipe?.out_tier ?? 0,
      materialA: recipe ? byId.get(recipe.a)?.name ?? `#${recipe.a}` : '—',
      materialB: recipe ? byId.get(recipe.b)?.name ?? `#${recipe.b}` : '—',
    }
  })
})

/** 图鉴搜索（2026-11-01 新一轮第 71 轮）：名称/元素/主动被动过滤。 */
const keyword = ref('')

const filteredSkills = computed(() => {
  const kw = keyword.value.trim().toLowerCase()
  if (!kw) return skills.value
  return skills.value.filter((s) => {
    return (
      s.name.toLowerCase().includes(kw) ||
      elementName(s.element).toLowerCase().includes(kw) ||
      (s.kind === 'active' ? '主动' : '被动').includes(kw)
    )
  })
})

/**
 * 已拥有判据与背包一致：服务端 `build.skills` 里有没有这一行
 * （`store.build` 来自 /me 权威快照，见 bag.vue 的 isOwned）。
 */
const ownedSkillIds = computed(() => {
  const bag = store.build?.skills
  if (!bag) return new Set<number>()
  return new Set(Object.keys(bag).map(Number))
})

/* -------------------------------------------------------------------------- */
/* G66 技能元素×类型覆盖矩阵（第七批）                                          */
/* 5 元素 × 主动/被动，格子显示「已拥有/总数」，帮玩家看出哪类元素缺技能。        */
/* -------------------------------------------------------------------------- */
const kindCols = [
  { key: 'active', label: '主动' },
  { key: 'passive', label: '被动' },
] as const
const skillMatrix = computed(() =>
  ELEMENT_ORDER.map((el) => ({
    el,
    color: ELEMENT_COLOR[el],
    cells: kindCols.map((k) => {
      const all = skills.value.filter((s) => s.element === el && s.kind === k.key)
      const owned = all.filter((s) => ownedSkillIds.value.has(s.id)).length
      return { ...k, owned, total: all.length }
    }),
  })),
)

const ownedCount = computed(
  () => skills.value.filter((s) => ownedSkillIds.value.has(s.id)).length,
)

function isOwned(id: number): boolean {
  return ownedSkillIds.value.has(id)
}

function elementName(e: Element): string {
  return ELEMENT_NAME[e] ?? e
}

/** R3 FilterBar：在既有搜索结果上按元素筛选，保留原搜索 computed 与加载入口。 */
const r3Element = ref<Element | ''>('')
const r3FilteredSkills = computed(() => {
  const base = filteredSkills.value
  if (!r3Element.value) return base
  return base.filter((s) => s.element === r3Element.value)
})

/** R3 AI 线索：效率榜第 1 名 + 是否已解锁，结论不写死、依据取现有派生数据。 */
const r3AiSkill = computed(() => {
  const rank = efficiencyRank.value.byHeat
  if (!rank.length) return null
  const pick = rank.find((r) => !ownedSkillIds.value.has(r.id)) ?? rank[0]
  const owned = ownedSkillIds.value.has(pick.id)
  return {
    id: pick.id,
    name: pick.name,
    reason: '每热量 ' + pick.v + ' 伤害，' + (owned ? '已解锁' : '尚未解锁'),
    conclusion: pick.name + '每热量 ' + pick.v + ' 伤害（效率榜第 1），' + (owned ? '已解锁，优先带上' : '尚未解锁，优先攒它'),
    basis: '技能效率榜 ' + rank.length + ' 条 + 已解锁 ' + ownedCount.value + '/' + skills.value.length + ' · 热量是硬约束',
  }
})
function r3GoBag() {
  uni.navigateTo({ url: '/pages/bag/bag' })
}
/* -------------------------------------------------------------------------- */
/* R3 冷启动配置加载：深链接/刷新直入时自行拉 /config                             */
/* -------------------------------------------------------------------------- */
let pageLoaded = false
let loadInflight: Promise<void> | null = null

async function doLoadPage(): Promise<void> {
  if (!store.config) {
    // 此前本页没有任何加载调用，把「尚未加载」谎报成「技能表为空（配置未加载？）」。
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
/* G109 技能效率榜 */
.eff-row { display: flex; flex-direction: row; align-items: center; gap: 10rpx; margin: 8rpx 0; }
.eff-name { width: 180rpx; flex-shrink: 0; font-size: 22rpx; }
.eff-track { flex: 1; height: 10rpx; background: var(--panel-3); border-radius: 999rpx; overflow: hidden; }
.eff-fill { height: 100%; background: var(--fire); border-radius: 999rpx; }
.eff-val { width: 70rpx; text-align: right; font-size: 22rpx; color: var(--warn); }
.eff-time { display: block; font-size: 22rpx; color: var(--text-2); margin: 4rpx 0; }

/* G66 技能元素×类型覆盖矩阵 */
.skill-matrix { display: flex; flex-direction: column; gap: 4rpx; margin-bottom: 16rpx; }
.sm-row { display: flex; flex-direction: row; gap: 4rpx; }
.sm-head { flex: 1; text-align: center; font-size: 22rpx; font-weight: 600; padding: 6rpx 0; }
.sm-cell {
  flex: 1;
  text-align: center;
  font-size: 22rpx;
  padding: 10rpx 0;
  border-radius: 8rpx;
  background: var(--panel-2);
  color: var(--dim);
}
.sm-cell.covered { background: rgba(126, 231, 135, 0.14); color: var(--ok); }
/* 条目分隔线走共享的 .rxn（styles/common.css） */
/* R67 主动/被动标签配色 */
.kind-tag { font-size: 22rpx; }
.kind-active { color: var(--fire); }
.kind-passive { color: var(--ice); }
</style>