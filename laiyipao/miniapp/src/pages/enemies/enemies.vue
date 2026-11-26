<template>
  <view class="page-root">
    <view class="pad safe-bottom">
      <!-- R4：眉题 → 克制筛选 → 数据格 → 智能空态 -->
      <view class="section-kicker section-kicker--ai r4-section r4-enter">
        <text class="section-kicker-cn">敌人情报台</text>
        <text class="section-kicker-en">ENEMY INTEL / R4</text>
      </view>
      <view v-if="r4AiEnemy" class="r3-ai r4-ai r4-enter" aria-label="AI 克制诊断">
        <view class="r3-ai-head">
          <text class="r3-ai-badge">AI</text>
          <text class="r3-ai-title">AI 克制诊断</text>
          <text class="r3-ai-engine">规则引擎 · 基于 /config 抗性</text>
        </view>
        <text class="r3-ai-conclusion">{{ r4AiEnemy.conclusion }}</text>
        <text class="r3-ai-basis">{{ r4AiEnemy.basis }}</text>
        <view class="r3-ai-cta" @click="r4FocusEnemy()">切换对应元素 →</view>
      </view>
      <!-- 加载中 / 空态（与反应图鉴同款，2026-10-21 P6-1）-->
      <view v-if="store.configLoading" class="card">
        <text class="muted">加载中…</text>
      </view>
      <view v-else-if="enemies.length === 0" class="r3-empty r4-empty" aria-live="polite" role="status">
        <text class="r3-empty-slot">AI</text>
        <text class="r3-empty-title">敌人表还没加载出来</text>
        <text class="r3-empty-reason">抗性数据来自服务端 /config；配置未到位时不会伪造克制结论。</text>
        <view class="r3-empty-action" @click="loadPage()">重新加载配置</view>
      </view>

      <!-- 敌人图鉴（服务端下发的权威表） -->
      <view class="card stagger-in">
        <text class="card-title">敌人图鉴</text>
        <!-- 搜索（2026-11-01 新一轮第 70 轮）：图鉴条目多，名称/类别过滤快速定位 -->
        <input
          v-model="keyword"
          class="search-inp"
          placeholder="搜索敌人名称 / 类别（如 Boss、飞行）"
          placeholder-class="ph"
        />
        <text class="muted" style="display: block; margin-bottom: 12rpx">
          数值直接来自服务端 /config，与游戏内实际生效的一致。
          抗性为千分比：正值减伤、负值易伤。
        </text>
        <!-- 第六批 R52：类别快速筛选 chip，不必在搜索框里输中文 -->
        <view class="chip-row r3-filter r4-filter" style="margin-bottom: 12rpx">
          <text
            v-for="c in CATEGORY_FILTERS"
            :key="c.key"
            class="chip r3-chip tap"
            :class="{ hit: categoryFilter === c.key, active: categoryFilter === c.key }"
            @click="categoryFilter = c.key"
          >{{ c.label }}</text>
        </view>
        <!-- G104 元素克制排行：堆某元素时先打谁 -->
        <view class="counter-box">
          <text class="dim" style="display: block; margin-bottom: 8rpx">克制排行（越易伤越靠前）</text>
          <view class="chip-row" style="margin-bottom: 8rpx">
            <text
              v-for="el in ELEMENT_ORDER"
              :key="'ce' + el"
              class="chip tap"
              :class="{ hit: counterElement === el }"
              :style="counterElement === el ? { background: ELEMENT_COLOR[el], color: '#fff' } : {}"
              @click="counterElement = el"
            >{{ ELEMENT_NAME[el] }}</text>
          </view>
          <view v-for="(c, i) in counterRanking" :key="c.id" class="counter-row">
            <text class="dim">{{ i + 1 }}. {{ c.name }}</text>
            <view class="counter-track">
              <view
                class="counter-fill"
                :style="{ width: Math.min(100, Math.abs(c.resist) / 5) + '%', background: c.resist < 0 ? 'var(--ok)' : 'var(--bad)' }"
              />
            </view>
            <text class="mono">{{ c.resist > 0 ? '+' : '' }}{{ c.resist / 10 }}%</text>
          </view>
        </view>

        <!-- G119 敌人数值分布 -->
        <view class="hp-dist">
          <text class="dim" style="display: block; margin-bottom: 8rpx">血量分布（含 BOSS 计数）</text>
          <view v-for="(b, i) in hpDistribution" :key="i" class="hd-row">
            <text class="dim hd-label">{{ b.label }}+</text>
            <view class="hd-track"><view class="hd-fill" :style="{ width: b.pct + '%' }" /></view>
            <text class="mono hd-val">{{ b.count }}<text v-if="b.boss" class="hd-boss">★{{ b.boss }}</text></text>
          </view>
        </view>

        <text class="result-count">显示 {{ filteredEnemies.length }} / 共 {{ enemies.length }} 条</text>
        <view class="r3-data-grid r4-grid">
        <view v-for="en in filteredEnemies" :key="en.id" class="rxn enemy list-row">
          <view class="rxn-head">
            <!-- 敌型图标座：形状与战斗画布里的敌型一一对应（圆/菱/三角/六边/八边+冠） -->
            <view class="icon-badge" :class="en.is_boss ? 'badge-fire' : ''" style="width: 64rpx; height: 64rpx">
              <DefIcon :name="'enemy:' + en.category" :size="40" />
            </view>
            <text class="rxn-name tap" @click="toggleEnemy(en.id)">{{ en.name }}</text>
            <text class="dim">{{ categoryLabel(en.category) }}</text>
            <text v-if="en.is_boss" class="chip hit">Boss</text>
            <text class="resist-toggle" @click="toggleEnemy(en.id)">
              {{ expandedEnemy === en.id ? '收起抗性图 ▲' : '抗性图 ▼' }}
            </text>
          </view>
          <text class="rxn-descr">{{ en.descr || '（服务端未提供说明）' }}</text>
          <view class="rxn-stats">
            <text class="stat">血量 {{ en.hp }}</text>
            <text class="stat">速度 {{ (en.speed / 1000).toFixed(1) }}</text>
            <text v-if="en.armor > 0" class="stat">护甲 {{ en.armor }}</text>
            <text v-if="en.shield_hp > 0" class="stat">护盾 {{ en.shield_hp }}</text>
            <text class="stat">攻击 {{ en.attack }}</text>
            <text v-if="en.attack_interval > 0" class="stat">
              间隔 {{ (en.attack_interval / 1000).toFixed(1) }}s
            </text>
            <text v-if="en.fly_height > 0" class="stat">飞行高度 {{ en.fly_height }}</text>
            <text v-if="en.burrow" class="stat">钻地</text>
          </view>
          <!-- 元素抗性（千分比） -->
          <view v-if="resistEntries(en).length" class="rxn-stats">
            <!-- R65 抗性正负色分：正值减伤（红=对我有害），负值易伤（绿） -->
            <text
              v-for="[el, v] in resistEntries(en)"
              :key="el"
              class="stat"
              :class="v > 0 ? 'res-up' : 'res-down'"
            >
              {{ elementName(el as Element) }}抗性 {{ v > 0 ? '+' : '' }}{{ (v / 10).toFixed(1) }}%（{{ v }}‰）
            </text>
          </view>
          <text v-else class="muted">无元素抗性</text>

          <!-- G73 抗性雷达图：点名字/「抗性图」展开。五维形状把「怕什么、抗什么」一眼画出 -->
          <view v-if="expandedEnemy === en.id" class="resist-radar-wrap">
            <svg class="resist-radar" viewBox="0 0 200 180" aria-label="敌人抗性雷达图">
              <polygon class="rr-ring" points="100,88 100,20 153,50 153,126 47,126 47,50" />
              <polygon class="rr-ring" points="100,88 100,44 138,63 138,113 62,113 62,63" />
              <line
                v-for="(pt, i) in resistDirs"
                :key="'ax' + i"
                class="rr-axis"
                x1="100" y1="88"
                :x2="100 + pt.dx * 62" :y2="88 + pt.dy * 62"
              />
              <polygon class="rr-shape" :points="enemyResistPoints(en)" />
              <text
                v-for="(lb, i) in resistLabels"
                :key="'lb' + i"
                class="rr-label"
                :x="lb.x" :y="lb.y"
                text-anchor="middle"
              >{{ lb.text }}</text>
            </svg>
            <text class="dim">外圈=减伤（抗性高）· 内缩=易伤（抗性负）</text>
          </view>
        </view>
        </view>
        <text v-if="!enemies.length" class="muted">敌人表为空（配置未加载？）</text>
        <!-- R23 搜索无结果：明确区分「表是空的」与「关键词没匹配上」 -->
        <view v-if="keyword && enemies.length && filteredEnemies.length === 0" class="search-empty">
          <text class="muted">没有匹配「{{ keyword }}」的敌人</text>
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
import { ELEMENT_NAME, ELEMENT_ORDER, ELEMENT_COLOR, type Element } from '@/game/elements'
import DefIcon from '@/components/DefIcon.vue'

const store = useGameStore()

/**
 * 敌人图鉴（2026-10-31 新一轮第 24 轮）。
 *
 * `/config.enemies` 从第一天起就下发了完整的敌人属性（血量/护甲/护盾/
 * 攻击/速度/五维抗性），但客户端**零 UI 消费** —— 玩家在选关/战斗前
 * 无法了解每种敌人的属性与抗性，「为什么这关这么难」只能靠试错。
 * 这里补上图鉴页。
 */
const enemies = computed(() => store.config?.enemies ?? [])

/** 图鉴搜索（2026-11-01 新一轮第 70 轮）：按名称/类别中文过滤。 */
const keyword = ref('')

/** 第六批 R52：类别 chip 筛选，'all' 表示不过滤。 */
const categoryFilter = ref('all')
const CATEGORY_FILTERS = [
  { key: 'all', label: '全部' },
  { key: 'normal', label: '普通' },
  { key: 'ranged', label: '远程' },
  { key: 'flying', label: '飞行' },
  { key: 'special', label: '特殊' },
  { key: 'elite', label: '精英' },
  { key: 'boss', label: 'Boss' },
] as const

const filteredEnemies = computed(() => {
  const kw = keyword.value.trim().toLowerCase()
  return enemies.value.filter((en) => {
    if (categoryFilter.value !== 'all' && en.category !== categoryFilter.value) return false
    if (!kw) return true
    return (
      en.name.toLowerCase().includes(kw) ||
      categoryLabel(en.category).toLowerCase().includes(kw) ||
      en.category.toLowerCase().includes(kw)
    )
  })
})

function categoryLabel(c: string): string {
  const map: Record<string, string> = {
    normal: '普通',
    ranged: '远程',
    flying: '飞行',
    special: '特殊',
    elite: '精英',
    boss: 'Boss',
  }
  return map[c] ?? c
}

function elementName(e: Element): string {
  return ELEMENT_NAME[e] ?? e
}

/** 只展示非零抗性，避免列表被一堆 0 值刷屏。 */
function resistEntries(en: { resist?: Record<string, number> }): [string, number][] {
  return Object.entries(en.resist ?? {}).filter(([, v]) => v !== 0)
}

/* -------------------------------------------------------------------------- */
/* G73 敌人抗性雷达图（第七批）                                                 */
/* 五元素抗性 -500..500 → 半径：0 落在中圈，正抗性（减伤）外扩、负抗性（易伤）内缩。 */
/* 数据全部来自 en.resist，不改 api/store 契约。                                */
/* -------------------------------------------------------------------------- */
/** G119 敌人数值分布：血量分 5 档统计敌人数，Boss 单独计数 —— 理解后期敌人厚在哪。 */
const hpDistribution = computed(() => {
  const hps = enemies.value.map((e) => e.hp)
  const max = Math.max(1, ...hps)
  const step = Math.ceil(max / 5)
  const buckets = Array.from({ length: 5 }, (_, i) => ({ label: `${i * step}`, count: 0, boss: 0 }))
  for (const e of enemies.value) {
    const idx = Math.min(4, Math.floor(e.hp / step))
    buckets[idx]!.count++
    if (e.is_boss) buckets[idx]!.boss++
  }
  const maxCount = Math.max(1, ...buckets.map((b) => b.count))
  return buckets.map((b) => ({ ...b, pct: Math.round(b.count / maxCount * 100) }))
})

/** R4：从当前构筑元素与 /config 抗性推出一句可执行克制建议。 */
const r4AiEnemy = computed(() => {
  if (!counterRanking.value.length) return null
  const equipped = store.equippedElements ?? []
  const el = equipped.find((e) => ELEMENT_ORDER.includes(e)) ?? counterElement.value
  const top = enemies.value
    .map((en) => ({ name: en.name, resist: Number(en.resist?.[el] ?? 0) }))
    .sort((a, b) => a.resist - b.resist)[0]
  if (!top) return null
  return {
    target: el,
    conclusion: `按${ELEMENT_NAME[el]}构筑，最易伤的是「${top.name}」，抗性 ${(top.resist / 10).toFixed(1)}%`,
    basis: '依据：/config 抗性排行 / 当前出战元素 · 更新于本次进入',
  }
})

function r4FocusEnemy() {
  if (r4AiEnemy.value) counterElement.value = r4AiEnemy.value.target
  uni.pageScrollTo({ selector: '.counter-box', duration: 200 })
}

/** G104 元素克制排行：选一个元素，按该元素抗性升序排（越负越易伤），配装时快速找靶子。 */
const counterElement = ref<Element>(ELEMENT_ORDER[0]!)
const counterRanking = computed(() =>
  enemies.value
    .map((en) => ({ id: en.id, name: en.name, resist: Number(en.resist?.[counterElement.value] ?? 0) }))
    .sort((a, b) => a.resist - b.resist)
    .slice(0, 8),
)

const expandedEnemy = ref<number | null>(null)
function toggleEnemy(id: number) {
  expandedEnemy.value = expandedEnemy.value === id ? null : id
}
const RADAR_CX = 100
const RADAR_CY = 88
const RADAR_R = 62
/** 抗性千分比 → 半径比例（0→0.5，-500→0.16，+500→0.84）。 */
function resistScale(v: number): number {
  return 0.5 + Math.max(-500, Math.min(500, v)) / 1000 * 0.68
}
/** 五元素方向的单位向量（正上方起顺时针）。 */
const resistDirs = Array.from({ length: 5 }, (_, i) => {
  const a = -Math.PI / 2 + (i * Math.PI * 2) / 5
  return { dx: Math.cos(a), dy: Math.sin(a) }
})
function enemyResistPoints(en: { resist?: Record<string, number> }): string {
  return resistDirs
    .map((d, i) => {
      const el = ELEMENT_ORDER[i]
      const v = Number(en.resist?.[el] ?? 0)
      const s = resistScale(v)
      return `${(RADAR_CX + d.dx * RADAR_R * s).toFixed(1)},${(RADAR_CY + d.dy * RADAR_R * s).toFixed(1)}`
    })
    .join(' ')
}
/** 五元素标签（正上方起顺时针，与 resistDirs 同序）。 */
const resistLabels = resistDirs.map((d, i) => ({
  x: RADAR_CX + d.dx * (RADAR_R + 14),
  y: RADAR_CY + d.dy * (RADAR_R + 14) + 4,
  text: ELEMENT_NAME[ELEMENT_ORDER[i]!] ?? '',
}))
/* -------------------------------------------------------------------------- */
/* R3 冷启动配置加载：深链接/刷新直入时自行拉 /config                             */
/* -------------------------------------------------------------------------- */
let pageLoaded = false
let loadInflight: Promise<void> | null = null

async function doLoadPage(): Promise<void> {
  if (!store.config) {
    // 此前本页没有任何加载调用，把「尚未加载」谎报成「敌人表为空（配置未加载？）」。
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
/* G119 血量分布 */
.hd-row { display: flex; flex-direction: row; align-items: center; gap: 10rpx; margin: 6rpx 0; }
.hd-label { width: 120rpx; flex-shrink: 0; font-size: 22rpx; }
.hd-track { flex: 1; height: 12rpx; background: var(--panel-3); border-radius: 999rpx; overflow: hidden; }
.hd-fill { height: 100%; background: var(--bad); border-radius: 999rpx; }
.hd-val { width: 100rpx; text-align: right; font-size: 22rpx; }
.hd-boss { color: var(--warn); margin-left: 6rpx; }

/* G104 元素克制排行 */
.counter-box { margin: 12rpx 0 16rpx; padding: 12rpx; background: var(--panel-2); border-radius: 12rpx; }
.counter-row { display: flex; flex-direction: row; align-items: center; gap: 8rpx; margin: 6rpx 0; }
.counter-row .dim { width: 180rpx; flex-shrink: 0; font-size: 22rpx; }
.counter-track { flex: 1; height: 10rpx; background: var(--panel-3); border-radius: 999rpx; overflow: hidden; }
.counter-fill { height: 100%; border-radius: 999rpx; }
.counter-row .mono { width: 90rpx; text-align: right; font-size: 22rpx; }

/* G73 敌人抗性雷达图 */
.rxn-name.tap { color: var(--accent); }
.resist-toggle {
  margin-left: 12rpx;
  font-size: 20rpx;
  color: var(--ice);
}
.resist-radar-wrap {
  display: flex;
  flex-direction: column;
  align-items: center;
  margin-top: 12rpx;
}
.resist-radar {
  width: 400rpx;
  height: 360rpx;
  overflow: visible;
}
.rr-ring {
  fill: none;
  stroke: var(--border-soft, var(--border));
  stroke-width: 1;
}
.rr-axis {
  stroke: var(--border-soft, var(--border));
  stroke-width: 1;
}
.rr-shape {
  fill: rgba(88, 166, 255, 0.22);
  stroke: #58a6ff;
  stroke-width: 2;
}
.rr-label {
  fill: var(--muted);
  font-size: 11px;
}

/* 条目分隔线走共享的 .rxn（styles/common.css） */
/* R65 抗性正负色分 */
.res-up { color: var(--bad); }
.res-down { color: var(--ok); }
</style>