<template>
  <view class="page-root">
    <view class="pad safe-bottom">
      <!-- 加载中 / 空态（2026-10-21 P6-1）-->
      <view v-if="store.configLoading" class="card">
        <text class="muted">加载中…</text>
      </view>
      <view v-else-if="chapterLevels.length === 0" class="card empty-block">
        <text class="empty-glyph">🗺️</text>
        <text class="muted">本章节还没有关卡</text>
        <text class="dim">换一个章节看看，或从下方选择其它章节。</text>
      </view>
      <!-- 章节选择 -->
      <scroll-view scroll-x class="chapter-scroll" :show-scrollbar="false">
        <view class="chapter-row">
          <view
            v-for="ch in chapters"
            :key="ch.id"
            class="chapter-chip"
            :class="{ active: curChapter === ch.id }"
            @click="switchChapter(ch.id)"
          >
            {{ ch.name }}
            <!-- 第六批 R7 关卡数：切章前就知道这一章有多少关 -->
            <text class="chapter-count">{{ chapterLevelCount(ch.id) }}</text>
            <!-- R69 章节全清标记：切章节前就知道哪些已经打完 -->
            <text v-if="chapterAllCleared(ch.id)" class="chapter-clear-dot">★</text>
          </view>
        </view>
      </scroll-view>

      <!-- 章节星级进度（2026-11-01 新一轮第 52 轮）：
           玩家能一眼看到本章节还差多少星才全清。 -->
      <view v-if="chapterLevels.length" class="star-progress">
        <text class="muted">本章节星级</text>
        <!-- 第六批 R68：星级未到之前不要显示成 0/总，避免误读为「一星没拿」 -->
        <text v-if="!starsLoaded" class="dim">加载中…</text>
        <text v-else class="mono">{{ chapterStars }} / {{ chapterTotal }}</text>
        <view class="chapter-star-track">
          <view
            class="chapter-star-fill"
            :style="{ width: chapterTotal ? (chapterStars / chapterTotal * 100) + '%' : '0%' }"
          />
        </view>
      </view>

      <!-- G85 本章难度曲线：折线看章内难度爬升 -->
      <view v-if="difficultyCurve.pts.length >= 2" class="card diff-curve-card">
        <view class="card-title">本章难度曲线</view>
        <svg class="diff-svg" viewBox="0 0 300 92" aria-label="关卡难度曲线">
          <polyline class="diff-line" :points="difficultyCurve.line" />
          <circle
            v-for="p in difficultyCurve.pts"
            :key="p.id"
            :cx="p.x" :cy="p.y" r="3"
            class="diff-dot"
          />
        </svg>
      </view>

      <!-- G111 本章 BOSS 清单 -->
      <view v-if="bossLevels.length" class="card boss-list-card">
        <view class="card-title">本章首领关</view>
        <view v-for="b in bossLevels" :key="b.id" class="bl-row">
          <text class="bl-name">第 {{ b.id }} 关 · {{ b.name }}</text>
          <text class="dim">难度 {{ b.difficulty }}‰</text>
          <text v-if="b.stars > 0" class="bl-cleared">★{{ b.stars }}</text>
          <text v-else class="dim">未通关</text>
        </view>
      </view>

      <!-- G63 全章节星级总览：整体进度 + 每章完成度，跨章节推进度一眼可见 -->
      <view v-if="starsLoaded && chapterOverview.length" class="card overview-card">
        <view class="card-title">
          <text>全局进度</text>
          <text class="mono">{{ allStars }} / {{ allStarCap }} ★</text>
        </view>
        <view v-for="ch in chapterOverview" :key="ch.id" class="overview-row">
          <text class="dim overview-name">{{ ch.name }}</text>
          <view class="overview-track">
            <view class="overview-fill" :style="{ width: ch.pct + '%' }" />
          </view>
          <text class="mono overview-pct">{{ ch.pct }}%</text>
        </view>
      </view>

      <!-- 关卡网格 -->
      <view class="grid">
        <view
          v-for="lv in chapterLevels"
          :key="lv.id"
          class="level-cell"
          :class="{
            locked: lv.id > store.unlockedLevel,
            cleared: starOf(lv.id) > 0,
            current: lv.id === activeLevelId,
            terrain: lv.terrain && lv.terrain.length > 0,
            boss: lv.is_boss,
          }"
          @click="enter(lv.id)"
        >
          <!-- 锁定关（100 轮打磨 R11）：锁图标替代纯暗格 —— 「为什么点不了」不再靠猜 -->
          <Icon v-if="lv.id > store.unlockedLevel" name="lock" :size="36" class="lock-glyph" />
          <text class="level-no">{{ lv.id }}</text>
          <!--
            第六批 R2：把「当前」与「最新」拆成两个概念。

            改前两者都绑在 store.unlockedLevel（最新解锁关）：玩家点第 2 关，
            「当前」仍挂在第 4 关上，页面上方毫无反馈，要往下划才知道选了哪关。
            改后「当前」跟着玩家的选择走（点谁谁亮），「最新」才表示进度。
          -->
          <text
            v-if="lv.id === activeLevelId"
            class="level-badge current-badge"
            :class="{ both: lv.id === store.unlockedLevel }"
          >{{ lv.id === store.unlockedLevel ? '当前·最新' : '当前' }}</text>
          <text v-else-if="lv.id === store.unlockedLevel" class="level-badge latest-badge">最新</text>
          <!-- 第六批 R67：BOSS 与地形可同时存在，此前 v-else-if 只显示其中一个 -->
          <text v-if="lv.id !== activeLevelId && lv.id !== store.unlockedLevel && lv.is_boss" class="level-badge">BOSS</text>
          <text
            v-if="lv.id !== activeLevelId && lv.id !== store.unlockedLevel && lv.terrain && lv.terrain.length > 0"
            class="level-badge terrain"
          >地形</text>
          <!-- R41 「可以打」提示保留在最新关：那是下一个该推进的目标 -->
          <text v-if="lv.id === store.unlockedLevel" class="play-glyph">▶</text>
          <view v-if="starOf(lv.id) > 0" class="stars">
            <text v-for="i in 3" :key="i" class="star" :class="{ off: i > starOf(lv.id) }">★</text>
          </view>
        </view>
      </view>

      <!-- 当前关卡详情（第六批 R2：id 供点选后自动滚动定位） -->
      <view v-if="selected" id="level-detail" class="card">
        <text class="card-title">第 {{ selected.id }} 关 · {{ selected.name }}</text>
        <!-- R70 详情卡补历史最佳：重打前能看到自己这关的最好成绩 -->
        <view v-if="starOf(selected.id) > 0" class="info-row">
          <text class="muted">历史最佳</text>
          <text class="stars-inline">{{ '★'.repeat(starOf(selected.id)) }}</text>
        </view>
        <view class="info-row"><text class="muted">防线血量</text><text class="mono">{{ selected.base_hp }}</text></view>
        <view class="info-row"><text class="muted">波次</text><text class="mono">{{ selected.wave_count }}</text></view>
        <view class="info-row"><text class="muted">难度系数</text><text class="mono">{{ selected.difficulty }}‰</text></view>
        <view class="info-row">
          <text class="muted">体力消耗</text>
          <text class="mono">{{ energyCostText }}</text>
        </view>
        <view class="info-row"><text class="muted">元素层数上限</text><text class="mono">{{ selected.element_cap }}</text></view>
        <view v-if="selected.terrain && selected.terrain.length" class="info-row">
          <text class="muted">地形</text>
          <text>{{ selected.terrain.map((t) => terrainName(t.kind)).join('、') }}</text>
        </view>
        <!--
          2026-10-21（P4-4）：地形**机制**。
          `TERRAIN_DESCR` 此前零消费 —— 选关页只显示地形**名字**，
          玩家无从知道油桶/潮汐闸/旋翼/崩塌掩体/蓄能塔分别做什么，
          于是「这关有地形」看起来只是个图标。
        -->
        <view v-for="(t, i) in selected.terrain" :key="i" class="terrain-note">
          <text class="terrain-note-name">{{ terrainName(t.kind) }}</text>
          <text class="terrain-note-descr">{{ terrainDescr(t.kind) }}</text>
        </view>
        <!-- 敌人组成预览（2026-10-31 新一轮第 29 轮） -->
        <view v-if="detailWaves.length" class="wave-list">
          <text class="muted" style="display: block; margin-bottom: 4rpx">敌人组成</text>
          <view v-for="(w, wi) in detailWaves" :key="wi" class="wave-row">
            <text class="muted">第 {{ wi + 1 }} 波</text>
            <text v-for="(sp, si) in w.spawns" :key="si" class="wave-spawn">
              {{ enemyName(sp.enemy_id) }} ×{{ sp.count }}
            </text>
          </view>
        </view>
        <!-- 通关掉落预览（2026-11-01 新一轮第 39 轮） -->
        <view v-if="dropPreview" class="drop-preview">
          <text class="muted" style="display: block; margin-bottom: 4rpx">通关掉落</text>
          <view class="info-row"><text class="muted">基础金币</text><text class="mono">{{ dropPreview.base }}</text></view>
          <view class="info-row"><text class="muted">三星金币</text><text class="mono">+{{ dropPreview.starCoin }}</text></view>
          <view v-if="dropPreview.totalEnemies > 0" class="info-row">
            <text class="muted">击杀金币（约 {{ dropPreview.totalEnemies }} 只）</text>
            <text class="mono">+{{ dropPreview.killCoin }}</text>
          </view>
          <view class="info-row"><text class="muted">反应加成（每 10 次反应）</text><text class="mono">+{{ dropPreview.reactionBonus }}</text></view>
          <view v-if="dropPreview.keys > 0" class="info-row">
            <text class="muted">三星钥匙</text><text class="mono">×{{ dropPreview.keys }}</text>
          </view>
          <view v-if="dropPreview.energy > 0" class="info-row">
            <text class="muted">通关返还体力</text><text class="mono">+{{ dropPreview.energy }}</text>
          </view>
        </view>
        <view class="star-targets">
          <text class="muted">星级门槛</text>
          <text v-for="(s, i) in selected.star_targets" :key="i" class="target">
            {{ i + 1 }}★ {{ s }}
          </text>
        </view>
      </view>

      <!--
        开始按钮 sticky 置底（2026-11-17 100 轮打磨 R13）：
        详情卡很长（敌人组成/掉落预览/星级门槛），此前按钮压在页尾，
        选完关要滚过三屏才能点「开始」。吸底后随时可开战；
        不可进入时依旧整钮降透明（btn-disabled 语义不变）。
      -->
      <view
        class="btn btn-primary start-btn"
        :class="{ 'btn-disabled': !canEnter }"
        @click="startBattle"
      >
        {{ startLabel }}
      </view>

      <!-- 连续失败时展示针对性诊断（L-1） -->
      <view v-if="diagnoseInfo" class="card" style="border-color: var(--warn)">
        <text class="card-title">
          <text>卡关诊断</text>
          <text class="tag tag-neutral">置信度 {{ diagnoseInfo.confidence }}%</text>
        </text>
        <text class="diag-title">{{ diagnoseInfo.title }}</text>
        <text class="muted" style="display: block; margin-top: 8rpx">{{ diagnoseInfo.detail }}</text>
        <text v-for="(s, i) in diagnoseInfo.suggestions" :key="i" class="diag-tip">· {{ s }}</text>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, nextTick } from 'vue'
import {onLoad, onShow} from '@dcloudio/uni-app'
import { useGameStore } from '@/store/game'
import { unlockSfx } from '@/audio/sfx'
import * as api from '@/api/client'
import Icon from '@/components/Icon.vue'
import { TERRAIN_NAME, TERRAIN_DESCR } from '@/game/terrain'
import type { LevelSummary, TerrainKind, Wave } from '@/game/types'

const store = useGameStore()

/**
 * 地形机制说明（2026-10-21 P4-4）。
 *
 * `TERRAIN_DESCR` 是引擎侧的地形说明表，此前**没有任何 UI 消费**。
 * 未知地形回退成「（无机制说明）」而不是空串 ——
 * 空串会让那一行看起来像没内容，而不是像「我不知道」。
 */
function terrainDescr(kind: TerrainKind): string {
  return TERRAIN_DESCR[kind] ?? '（无机制说明）'
}

const curChapter = ref(1)
const selectedId = ref(0)
const stars = ref<Record<number, number>>({})
/** 星级是否已经拉过一次（避免 onLoad + onShow 各拉一次）。 */
const starsLoaded = ref(false)
const diagnoseInfo = ref<any>(null)

const chapters = computed(() => store.config?.chapters ?? [])

const chapterLevels = computed<LevelSummary[]>(() => {
  const all = store.config?.levels ?? []
  return all.filter((l) => l.chapter === curChapter.value)
})

/**
 * 章节星级进度（2026-11-01 新一轮第 52 轮）。
 * 星级以服务端 level_stars 为准（starOf 读取 stars.value），
 * 汇总当前章节已获星星与理论上限（关卡数 × 3）。
 */
const chapterStars = computed(() =>
  chapterLevels.value.reduce((sum, lv) => sum + (starOf(lv.id) ?? 0), 0),
)
const chapterTotal = computed(() => chapterLevels.value.length * 3)

/* -------------------------------------------------------------------------- */
/* G85 本章难度曲线（第七批）                                                   */
/* 各关 difficulty 折线，直观看到章内难度爬升与陡增点。数据来自 chapterLevels。     */
/* -------------------------------------------------------------------------- */
/** G111 本章 BOSS 关清单：这些是要专门配装的硬骨头，清单化后不用逐格找。 */
const bossLevels = computed(() =>
  chapterLevels.value
    .filter((l) => l.is_boss)
    .map((l) => ({ id: l.id, name: l.name, difficulty: l.difficulty, stars: starOf(l.id) })),
)

const difficultyCurve = computed(() => {
  const list = chapterLevels.value
  const max = Math.max(1, ...list.map((l) => l.difficulty ?? 0))
  const min = Math.min(...list.map((l) => l.difficulty ?? 0), max)
  const span = Math.max(1, max - min)
  const pts = list.map((l, i) => ({
    id: l.id,
    x: list.length <= 1 ? 150 : (i / (list.length - 1)) * 300,
    // 难度越高 y 越小（越靠上）
    y: 80 - ((l.difficulty ?? 0) - min) / span * 64,
    diff: l.difficulty ?? 0,
  }))
  return { pts, line: pts.map((p) => `${p.x.toFixed(1)},${p.y.toFixed(1)}`).join(' ') }
})

/* -------------------------------------------------------------------------- */
/* G63 全章节星级总览（第七批）                                                 */
/* 此前只有「本章」进度；这里汇总全部关卡的已获星/总星，并给每章一个完成度，      */
/* 玩家一眼看到整体推进度与各章差距。数据来自 stars + config.levels/chapters。   */
/* -------------------------------------------------------------------------- */
const allStars = computed(() => (store.config?.levels ?? []).reduce((s, lv) => s + (starOf(lv.id) ?? 0), 0))
const allStarCap = computed(() => (store.config?.levels ?? []).length * 3)
const chapterOverview = computed(() =>
  chapters.value.map((ch) => {
    const lv = (store.config?.levels ?? []).filter((l) => l.chapter === ch.id)
    const got = lv.reduce((s, l) => s + (starOf(l.id) ?? 0), 0)
    const cap = lv.length * 3
    return { id: ch.id, name: ch.name, got, cap, pct: cap ? Math.round(got / cap * 100) : 0 }
  }),
)

// 选关页只读元数据（章节/名称/难度/体力/星级/地形），**不需要 waves** ——
// 所以这里用摘要类型就够，不必为此把完整关卡拉回来。
const selected = computed<LevelSummary | null>(() => {
  if (selectedId.value) return store.levelMap.get(selectedId.value) ?? null
  return chapterLevels.value.find((l) => l.id === store.unlockedLevel) ?? null
})

/**
 * 第六批 R2：玩家正在查看的关卡 id。
 *
 * 未显式点选时回落到最新解锁关（保持进入页面的初始行为不变）；
 * 点选后就是被点中的那关 ——「当前」徽章与呼吸描边都跟着它走。
 * 「最新」仍然由 store.unlockedLevel 单独表达，两者不再混淆。
 */
const activeLevelId = computed(() => selected.value?.id ?? store.unlockedLevel)

const canEnter = computed(() => !!selected.value && selected.value.id <= store.unlockedLevel)

/**
 * 敌人组成预览（2026-10-31 新一轮第 29 轮）。
 *
 * 选关页此前只显示血量/波次/难度/地形/星级门槛 —— 玩家不知道这关
 * 会遇到什么敌人（飞行/精英/Boss 等），「这关为什么难」只能进战斗
 * 试错。`/levels/:id` 本就返回完整波次，这里按需拉取并展示每波敌人。
 */
const detailWaves = ref<Wave[]>([])

const enemyNameMap = computed(() => {
  const m = new Map<number, string>()
  for (const e of store.config?.enemies ?? []) m.set(e.id, e.name)
  return m
})

function enemyName(id: number): string {
  return enemyNameMap.value.get(id) ?? `#${id}`
}

watch(selected, async (lv) => {
  if (!lv) {
    detailWaves.value = []
    return
  }
  try {
    const detail = await api.fetchLevel(lv.id)
    detailWaves.value = detail.waves ?? []
  } catch {
    // 敌人预览是辅助信息，拉取失败不阻塞选关
    detailWaves.value = []
  }
}, { immediate: true })

/**
 * 体力消耗文案（2026-10-21 P4-6）。
 *
 * 此前只显示 `energy_cost`（如 6），而**通关会返还 5 点** ——
 * 于是玩家看到「消耗 6」，实际净消耗是 1。
 * 「看到的数」与「实际付出的数」不是一回事，这正是本项目反复修的那类缺陷。
 *
 * 现在两个数都给：失败全损、通关返还。
 */
const energyCostText = computed(() => {
  const cost = selected.value?.energy_cost ?? 0
  const back = store.config?.drop_rates?.energy_on_win ?? 0
  if (back <= 0) return String(cost)
  const net = cost - back
  return net > 0 ? `${cost}（通关净消耗 ${net}）` : `${cost}（通关不扣体力）`
})

/**
 * 通关掉落预览（2026-11-01 新一轮第 39 轮）。
 *
 * `/config` 的 `drop_rates` 从第一天就下发，但只有 `energy_on_win`
 * 被体力消耗文案消费 —— 玩家进关前看不到这关能拿多少金币/钥匙，
 * 「打这关值不值」只能打完看结算。
 *
 * 这里汇总成收益预览：基础金币 + 三星金币（3 颗星）+ 击杀金币预估
 * （按敌人组成求和，实际以战斗表现为准，所以标「约」）+ 反应加成
 * （每 10 次反应）+ 三星钥匙 + 通关返还体力。
 * `detailWaves` 拉取失败时击杀预估自动隐藏，其余规则照常展示。
 */
const dropPreview = computed(() => {
  const d = store.config?.drop_rates
  if (!d) return null
  const totalEnemies = detailWaves.value.reduce(
    (sum, w) => sum + w.spawns.reduce((s, sp) => s + sp.count, 0),
    0,
  )
  return {
    base: d.coin_base,
    starCoin: d.coin_per_star * 3,
    killCoin: d.coin_per_kill * totalEnemies,
    totalEnemies,
    reactionBonus: d.reaction_bonus,
    keys: d.keys_on_three_star,
    energy: d.energy_on_win,
  }
})

const startLabel = computed(() => {
  if (!selected.value) return '未选择关卡'
  if (selected.value.id > store.unlockedLevel) return `第 ${selected.value.id} 关未解锁`
  if (store.wallet.energy < selected.value.energy_cost) return '体力不足'
  return `开始战斗（消耗 ${selected.value.energy_cost} 体力）`
})

/** 第六批 R7 章节关卡数（内容元数据，只读）。 */
function chapterLevelCount(id: number): number {
  return (store.config?.levels ?? []).filter((l) => l.chapter === id).length
}

/** R69 本章所有关卡是否都已拿到星。 */
function chapterAllCleared(id: number): boolean {
  const levels = (store.config?.levels ?? []).filter((l) => l.chapter === id)
  return levels.length > 0 && levels.every((l) => starOf(l.id) > 0)
}

function starOf(id: number): number {
  return stars.value[id] ?? 0
}

function terrainName(kind: string): string {
  return TERRAIN_NAME[kind as TerrainKind] ?? kind
}

function switchChapter(id: number) {
  curChapter.value = id
  selectedId.value = 0
  diagnoseInfo.value = null
}

function enter(id: number) {
  selectedId.value = id
  diagnoseInfo.value = null
  // 第六批 R2：点关卡后立刻把详情卡滚进视野 —— 此前只有下方详情卡在变，
  // 玩家不往下划就以为「点了没反应」。等 DOM 更新（当前徽章/详情都就位）后再滚。
  void nextTick(() => {
    uni.pageScrollTo({ selector: '#level-detail', duration: 250 })
  })
}

function startBattle() {
  if (!canEnter.value || !selected.value) {
    if (store.wallet.energy < (selected.value?.energy_cost ?? 0)) {
      uni.showToast({ title: '体力不足', icon: 'none' })
    }
    return
  }
  // 解锁音频（2026-10-23）。这里是一次真实的用户手势，且是进入战斗前的
  // **最后一个手势** —— 在这里解锁，进战斗后第一发子弹就有声音，
  // 不必等玩家去碰战斗页的屏幕。
  unlockSfx()
  uni.navigateTo({ url: `/pages/battle/battle?level=${selected.value.id}` })
}

/**
 * 刷新星级（2026-10-21 P6-3）。
 *
 * 此前星级**只在 onLoad 拉一次** —— 于是打完一局返回选关页，
 * 刚拿到的星星**不亮**（要退出重进才更新）。
 * 玩家看到的是「我明明通关了，但没给星」。
 */
async function refreshStars(): Promise<void> {
  starsLoaded.value = true
  try {
    const r = await api.fetchMyStars()
    stars.value = r.stars
  } catch {
    /* 星级拉取失败不阻塞选关 */
  }
}

/** 从战斗页返回时刷新星级。 */
onShow(() => {
  // ⚠️ onLoad 也会拉一次 —— 两者都跑就是**两次请求**，
  // 而且 onShow 那次不 await，可能在令牌就绪前发出 → 401 被静默吞掉。
  // 这里用 `starsLoaded` 挡住重复拉取。
  if (starsLoaded.value) void refreshStars()
})

onLoad(async (query) => {
  if (!store.loggedIn) await store.login()
  const lv = Number(query?.level ?? 0)
  if (lv > 0) {
    const target = store.levelMap.get(lv)
    if (target) curChapter.value = target.chapter
    selectedId.value = lv
  }
  // 星级以服务端的 level_stars（历史 GREATEST）为准：
  // 不拉这份数据，stars.value 永远空，starOf() 恒为 0，
  // 「已通关」与星级行在选关页永远不亮。
  await refreshStars()
  // 卡关诊断：失败次数由**服务端**算（2026-10-21 P2-7）。
  //
  // 此前这里传的是硬编码的 2，于是**每个人**第一次进关都能看到
  // 「卡关诊断」卡片 —— 一张对所有人说同一句话的卡片。
  // 现在只认服务端给的 `stuck`（连续失败 >= 2）。
  try {
    const d = await api.diagnose(selected.value?.id ?? store.unlockedLevel)
    if (d && d.stuck && d.stage !== 'unclear') diagnoseInfo.value = d
  } catch {
    /* 诊断不可用不阻塞选关 */
  }
})

onMounted(() => {
  if (!store.loggedIn) store.login()
})
</script>

<style scoped>
.chapter-scroll {
  white-space: nowrap;
  margin-bottom: 20rpx;
  /* 章节 chips 吸顶：切章不必滚回页首 */
  position: sticky;
  top: 0;
  z-index: 5;
  padding: 12rpx 0;
  background: linear-gradient(180deg, var(--bg) 82%, rgba(7, 10, 16, 0));
}

.chapter-row {
  display: flex;
  flex-direction: row;
  gap: 12rpx;
}

.chapter-chip {
  padding: 12rpx 26rpx;
  border-radius: 999rpx;
  background: var(--panel);
  border: 1rpx solid var(--border-soft);
  font-size: 24rpx;
  color: var(--muted);
  white-space: nowrap;
}

.chapter-chip.active {
  background: rgba(255, 107, 53, 0.16);
  border-color: var(--fire);
  color: var(--fire);
}

/* 第六批 R7 章节关卡数 */
.chapter-count {
  margin-left: 8rpx;
  font-size: 20rpx;
  color: var(--muted);
}

/* R69 章节全清星标 */
.chapter-clear-dot {
  margin-left: 8rpx;
  font-size: 20rpx;
  color: var(--warn);
}

/* G111 本章 BOSS 清单 */
.bl-row { display: flex; flex-direction: row; align-items: center; gap: 12rpx; padding: 8rpx 0; border-bottom: 1rpx solid var(--border-soft); }
.bl-name { flex: 1; font-size: 24rpx; }
.bl-cleared { color: var(--warn); font-weight: 600; }

/* G85 本章难度曲线 */
.diff-svg { width: 100%; height: 184rpx; overflow: visible; }
.diff-line { fill: none; stroke: var(--fire); stroke-width: 2; stroke-linejoin: round; }
.diff-dot { fill: var(--warn); }

/* G63 全章节星级总览 */
.overview-card .card-title .mono { color: var(--warn); }
.overview-row { display: flex; flex-direction: row; align-items: center; gap: 12rpx; margin: 8rpx 0; }
.overview-name { width: 140rpx; flex-shrink: 0; }
.overview-track { flex: 1; height: 10rpx; background: var(--panel-3); border-radius: 999rpx; overflow: hidden; }
.overview-fill { height: 100%; background: var(--warn); border-radius: 999rpx; }
.overview-pct { width: 72rpx; text-align: right; font-size: 22rpx; color: var(--warn); }

/* 章节星级进度（2026-11-01 新一轮第 52 轮） */
.star-progress {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  align-items: center;
  padding: 12rpx 20rpx;
  margin-bottom: 20rpx;
  border-radius: 12rpx;
  background: var(--panel);
  border: 1rpx solid var(--border);
  font-size: 26rpx;
}

.star-progress .mono {
  color: var(--warn);
  font-weight: 600;
}

/* R54 章节星级进度条：数字之外给一条可扫读的进度 */
.star-progress {
  flex-wrap: wrap;
  row-gap: 10rpx;
}

.chapter-star-track {
  width: 100%;
  height: 10rpx;
  border-radius: 999rpx;
  background: var(--panel-3);
  overflow: hidden;
}

.chapter-star-fill {
  height: 100%;
  border-radius: 999rpx;
  background: var(--warn);
  transition: width 0.3s ease-out;
  /* R91 星级进度条淡入光泽，强化「接近全清」的正反馈 */
  box-shadow: 0 0 8rpx rgba(255, 211, 61, 0.4);
}

.grid {
  display: grid;
  grid-template-columns: repeat(4, minmax(0, 1fr));
  gap: 16rpx;
  margin-bottom: 20rpx;
}

.level-cell {
  height: 150rpx;
  border-radius: var(--radius);
  background: var(--panel);
  border: 1rpx solid var(--border-soft);
  box-shadow: var(--shadow-1);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  position: relative;
}

.level-cell.locked {
  opacity: 0.5;
  box-shadow: none;
}

/* 锁定关锁图标（100 轮打磨 R11）：盖在暗格中央，编号让位 */
.lock-glyph {
  color: var(--muted);
  margin-bottom: 2rpx;
}

/* 当前关（100 轮打磨 R12）：品牌色描边 + 呼吸光晕 —— 「下一步打哪」一眼可见 */
.level-cell.current {
  border-color: var(--fire);
  animation: current-glow 1.6s ease-in-out infinite;
}

@keyframes current-glow {
  0%, 100% { box-shadow: 0 0 0 rgba(255, 107, 53, 0); }
  50%      { box-shadow: 0 0 18rpx rgba(255, 107, 53, 0.4); }
}

.level-badge.current-badge {
  color: var(--fire);
  font-weight: 600;
}

/* 第六批 R2：最新关（进度）用亮蓝，与「当前」（玩家选择，橙）区分 */
.level-badge.latest-badge {
  color: #58a6ff;
  font-weight: 600;
}

.level-badge.current-badge.both {
  font-size: 20rpx;
}

/* R41 当前关播放键：呼吸提醒「这关可打」 */
.play-glyph {
  position: absolute;
  right: 12rpx;
  top: 12rpx;
  font-size: 20rpx;
  color: var(--fire);
  animation: play-pulse 1.6s ease-in-out infinite;
}

@keyframes play-pulse {
  0%, 100% { opacity: 0.45; transform: scale(0.9); }
  50% { opacity: 1; transform: scale(1.1); }
}

.level-cell.cleared {
  border-color: rgba(126, 231, 135, 0.4);
}

/* 开始按钮吸底（100 轮打磨 R13）：详情卡再长也随时可开战 */
.start-btn {
  position: sticky;
  bottom: calc(16rpx + env(safe-area-inset-bottom));
  z-index: 15;
}

.level-cell.terrain {
  border-color: rgba(201, 167, 255, 0.4);
}

.level-cell.boss {
  border-color: var(--fire);
  background: linear-gradient(160deg, #2a1a1f, #161b22);
}

.level-no {
  font-size: 36rpx;
  font-weight: 700;
  font-variant-numeric: tabular-nums;
}

.level-badge {
  position: absolute;
  top: 6rpx;
  right: 8rpx;
  font-size: 22rpx;
  color: var(--fire);
}

.level-badge.terrain {
  color: var(--kinetic);
}

.stars {
  display: flex;
  flex-direction: row;
  margin-top: 4rpx;
}

.star {
  font-size: 20rpx;
  color: var(--warn);
}

.star.off {
  color: var(--border);
}

.info-row {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  padding: 8rpx 0;
  font-size: 26rpx;
}

/* R70 历史最佳星数 */
.stars-inline {
  color: var(--warn);
  letter-spacing: 4rpx;
}
/* 地形机制说明（2026-10-21 P4-4）：只显示名字不够，玩家要知道它做什么 */
.terrain-note {
  display: flex;
  flex-direction: row;
  gap: 12rpx;
  padding: 8rpx 0 8rpx 8rpx;
  border-left: 4rpx solid rgba(88, 166, 255, 0.5);
}
.terrain-note-name {
  flex-shrink: 0;
  font-size: 24rpx;
  color: #58a6ff;
}
.terrain-note-descr {
  flex: 1;
  font-size: 24rpx;
  color: var(--muted, #9aa0a6);
  line-height: 1.5;
}

/* 敌人组成预览（2026-10-31 新一轮第 29 轮） */
.wave-list {
  margin-top: 8rpx;
}
.wave-row {
  display: flex;
  flex-direction: row;
  flex-wrap: wrap;
  gap: 8rpx 16rpx;
  padding: 4rpx 0;
  font-size: 24rpx;
}
.wave-spawn {
  color: var(--muted, #9aa0a6);
}

/* 通关掉落预览（2026-11-01 新一轮第 39 轮） */
.drop-preview {
  margin-top: 8rpx;
  padding-top: 8rpx;
  border-top: 1rpx dashed var(--border);
}

.star-targets {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 20rpx;
  margin-top: 12rpx;
  padding-top: 12rpx;
  border-top: 1rpx solid var(--border);
}

.target {
  font-size: 22rpx;
  color: var(--warn);
}

.diag-title {
  font-size: 28rpx;
  font-weight: 600;
  color: var(--warn);
}

.diag-tip {
  font-size: 22rpx;
  color: var(--muted);
  line-height: 1.7;
  display: block;
  margin-top: 6rpx;
}
</style>
