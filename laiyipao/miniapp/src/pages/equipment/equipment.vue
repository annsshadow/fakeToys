<template>
  <view class="page-root">
    <view class="pad safe-bottom">
      <view class="section-kicker section-kicker--ai r3-enter">
        <text class="section-kicker-cn">装备锻造台</text>
        <text class="section-kicker-en">EQUIPMENT FORGE</text>
      </view>
      <view class="r3-kpi r3-enter">
        <view class="r3-kpi-main">
          <text class="r3-kpi-num">{{ r3AiEquip ? r3AiEquip.name : '待配置' }}</text>
          <text class="r3-kpi-label">AI 推荐 · 优先刷取部位</text>
          <text class="r3-kpi-sub">{{ r3AiEquip ? r3AiEquip.reason : '装备表尚未到达' }}</text>
        </view>
        <view class="r3-kpi-cell">
          <text class="r3-kpi-num">{{ ownedCount }}/{{ items.length }}</text>
          <text class="r3-kpi-label">已拥有 / 装备总数</text>
        </view>
        <view class="r3-kpi-cell">
          <text class="r3-kpi-num">{{ bestPerSlot.length }}</text>
          <text class="r3-kpi-label">部位天花板</text>
        </view>
      </view>
      <view class="r3-progress-row is-ai r3-enter">
        <text class="r3-progress-label">收集进度</text>
        <view class="r3-progress"><view class="r3-progress-fill" :style="{ width: r3EquipPct + '%' }" /></view>
        <text class="r3-progress-num">已拥有 {{ ownedCount }}/{{ items.length }}</text>
      </view>
      <view class="r3-filter" aria-label="按部位筛选装备">
        <text class="r3-chip" :class="{ active: r3Slot === '' }" @click="r3Slot = ''">全部 {{ items.length }}</text>
        <text v-for="b in bestPerSlot" :key="'f' + b.slot" class="r3-chip" :class="{ active: r3Slot === b.slot }" @click="r3Slot = b.slot">
          {{ b.label }} {{ b.count }}
        </text>
      </view>
      <view v-if="r3AiEquip" class="r3-ai r3-enter">
        <view class="r3-ai-head">
          <text class="r3-ai-badge">AI</text>
          <text class="r3-ai-title">AI 建议 · 搭配缺口</text>
          <text class="r3-ai-engine">规则引擎</text>
        </view>
        <text class="r3-ai-conclusion">{{ r3AiEquip.conclusion }}</text>
        <text class="r3-ai-basis">依据：{{ r3AiEquip.basis }}</text>
        <text class="r3-ai-cta" role="button" @click="r3GoStage">去关卡看掉落 →</text>
      </view>
      <view v-else-if="!store.configLoading && items.length === 0" class="r3-empty" aria-live="polite" role="status">
        <view class="r3-empty-slot">AI</view>
        <text class="r3-empty-title">还没有装备表数据</text>
        <text class="r3-empty-reason">服务端配置尚未下发，无法比较各部位天花板。</text>
        <text class="r3-empty-action" role="button" @click="r3GoStage()">去关卡看掉落 →</text>
      </view>
      <!-- 加载中 / 空态（与其它图鉴同款）-->
      <view v-if="store.configLoading" class="card">
        <text class="muted">加载中…</text>
      </view>
      <view v-else-if="items.length === 0" class="card">
        <text class="muted">装备表还没加载出来</text>
      </view>

      <!-- 装备图鉴（服务端下发的权威表） -->
      <view class="card stagger-in">
        <text class="card-title">装备图鉴</text>
        <!-- 搜索（2026-11-01 新一轮第 73 轮）：按名称/部位/元素过滤 -->
        <input
          v-model="keyword"
          class="search-inp"
          placeholder="搜索装备名称 / 部位 / 元素"
          placeholder-class="ph"
        />
        <text class="muted" style="display: block; margin-bottom: 12rpx">
          数值直接来自服务端 /config，与游戏内实际生效的一致。
          {{ ownedCount }} / {{ items.length }} 件已拥有。
        </text>
        <!-- G81 同部位天花板：每部位最强装备，指引刷装方向 -->
        <view v-if="bestPerSlot.length" class="best-slots">
          <text class="card-title" style="font-size: 26rpx">各部位最强</text>
          <view v-for="b in bestPerSlot" :key="b.slot" class="bs-row">
            <text class="dim bs-label">{{ b.label }}</text>
            <text class="bs-name">{{ b.name }}</text>
            <text v-if="b.owned" class="chip hit">已拥有</text>
            <text class="dim">护甲 {{ b.armor }} · 加成 {{ b.bonus }}‰</text>
          </view>
        </view>

        <!-- G108 部位×阶成长表 -->
        <view v-if="tierMatrix.tiers.length" class="tier-matrix">
          <text class="card-title" style="font-size: 26rpx">部位 × 阶（护甲）</text>
          <view class="tm-row">
            <view class="tm-head" />
            <view v-for="t in tierMatrix.tiers" :key="t" class="tm-head">T{{ t }}</view>
          </view>
          <view v-for="r in tierMatrix.rows" :key="r.slot" class="tm-row">
            <view class="tm-head tm-label">{{ r.label }}</view>
            <view v-for="(v, i) in r.cells" :key="i" class="tm-cell">{{ v === null ? '—' : v }}</view>
          </view>
        </view>

        <!-- G120 解锁等级分布 -->
        <view class="unlock-dist">
          <text class="card-title" style="font-size: 26rpx">解锁等级分布</text>
          <view v-for="b in unlockDist" :key="b.lo" class="ud-row">
            <text class="dim ud-label">{{ b.label }}</text>
            <view class="ud-track"><view class="ud-fill" :style="{ width: b.pct + '%' }" /></view>
            <text class="mono ud-val">{{ b.total }}<text v-if="b.owned" class="ud-owned">({{ b.owned }}有)</text></text>
          </view>
        </view>

        <text class="result-count">显示 {{ r3FilteredItems.length }} / 共 {{ items.length }} 条</text>
        <view class="r3-data-grid">
        <view v-for="e in r3FilteredItems" :key="e.id" class="rxn equip list-row">
          <view class="rxn-head">
            <!-- 部位图标座：装备的类别锚点（武器/头盔/胸甲…各有专属线性图标） -->
            <view class="icon-badge" style="width: 64rpx; height: 64rpx">
              <DefIcon :name="'slot:' + e.slot" :size="40" />
            </view>
            <text class="rxn-name">{{ e.name }}</text>
            <!-- 第六批 R47：★ 与 T 阶并列，玩家不必猜星星代表什么 -->
            <text class="dim">{{ slotLabel(e.slot) }} · {{ tierLabel(e.tier) }}（T{{ e.tier }}）</text>
            <text v-if="e.element" class="tag" :class="'tag-' + e.element">{{ elementName(e.element) }}</text>
            <text v-if="isOwned(e.id)" class="chip hit">已拥有</text>
          </view>
          <text class="rxn-descr">{{ e.descr || '（服务端未提供说明）' }}</text>
          <view class="rxn-stats">
            <text v-if="e.base_armor > 0" class="stat">护甲 {{ e.base_armor }}</text>
            <text v-if="e.base_bonus_pct > 0" class="stat">加成 {{ e.base_bonus_pct }}‰</text>
            <!-- 第六批 R48：与技能图鉴口径统一，「需 Lv.N」 -->
            <text class="stat">需 Lv.{{ e.unlock_level }}</text>
          </view>
        </view>
        </view>
        <text v-if="!items.length" class="muted">装备表为空（配置未加载？）</text>
        <!-- R24 搜索无结果：明确区分「表是空的」与「关键词没匹配上」 -->
        <view v-if="keyword && items.length && r3FilteredItems.length === 0" class="search-empty">
          <text class="muted">没有匹配「{{ keyword }}」的装备</text>
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
import { ELEMENT_NAME, type Element } from '@/game/elements'
import DefIcon from '@/components/DefIcon.vue'

const store = useGameStore()

/** 装备图鉴展示所需的字段形状（服务端 /config.equipment 元素）。 */
interface EquipmentDef {
  id: number
  code: string
  name: string
  slot: string
  tier: number
  element: Element
  descr: string
  base_armor: number
  base_bonus_pct: number
  unlock_level: number
}

/**
 * 装备图鉴（2026-10-31 新一轮第 26 轮）。
 *
 * `/config.equipment` 从第一天起就下发了全部 18 件装备的护甲/加成/
 * 解锁等级，但客户端只在背包展示**已拥有**的装备 —— 玩家看不到
 * 未解锁装备的成长路径。这里补上图鉴页，并标注已拥有状态。
 */
const items = computed(() => (store.config?.equipment ?? []) as unknown as EquipmentDef[])

/* -------------------------------------------------------------------------- */
/* G81 同部位最强装备（第七批）                                                 */
/* 按 slot 分组，每组挑护甲+加成最高的一件 —— 看「每个部位的天花板」，           */
/* 配合背包已拥有状态决定刷装方向。数据来自 items + ownedIds。                   */
/* -------------------------------------------------------------------------- */
/** G120 解锁等级分布：按 Lv 区间分桶，叠加已拥有件数 —— 看再升 10 级能解锁几件。 */
const unlockDist = computed(() => {
  const buckets = [0, 10, 20, 30, 40, 50].map((lo) => ({
    lo, label: `Lv${lo}`, total: 0, owned: 0,
  }))
  for (const e of items.value) {
    const lv = e.unlock_level ?? 0
    const idx = Math.min(5, Math.max(0, Math.floor(lv / 10)))
    buckets[idx]!.total++
    if (ownedIds.value.has(e.id)) buckets[idx]!.owned++
  }
  const max = Math.max(1, ...buckets.map((b) => b.total))
  return buckets.map((b) => ({ ...b, pct: Math.round(b.total / max * 100) }))
})

/** G108 部位×阶成长表：CSS 矩阵，每部位各阶护甲值 —— 看数值成长曲线。 */
const tierMatrix = computed(() => {
  const tiers = [...new Set(items.value.map((e) => e.tier))].sort((a, b) => a - b)
  const slots = [...new Set(items.value.map((e) => e.slot))]
  return {
    tiers,
    rows: slots.map((slot) => ({
      slot,
      label: slotLabel(slot),
      cells: tiers.map((tier) => {
        const eq = items.value.filter((e) => e.slot === slot && e.tier === tier)
        return eq.length ? Math.max(...eq.map((e) => e.base_armor)) : null
      }),
    })),
  }
})

const bestPerSlot = computed(() => {
  const groups = new Map<string, EquipmentDef[]>()
  for (const e of items.value) {
    const arr = groups.get(e.slot) ?? []
    arr.push(e)
    groups.set(e.slot, arr)
  }
  return [...groups.entries()].map(([slot, arr]) => {
    const best = [...arr].sort((a, b) => (b.base_armor + b.base_bonus_pct) - (a.base_armor + a.base_bonus_pct))[0]!
    return {
      slot,
      label: slotLabel(slot),
      name: best.name,
      armor: best.base_armor,
      bonus: best.base_bonus_pct,
      owned: ownedIds.value.has(best.id),
      count: arr.length,
    }
  }).sort((a, b) => a.label.localeCompare(b.label))
})

/** 图鉴搜索（2026-11-01 新一轮第 73 轮）：名称/部位中文/元素过滤。 */
const keyword = ref('')

const filteredItems = computed(() => {
  const kw = keyword.value.trim().toLowerCase()
  if (!kw) return items.value
  return items.value.filter((e) => {
    return (
      e.name.toLowerCase().includes(kw) ||
      slotLabel(e.slot).toLowerCase().includes(kw) ||
      (e.element && elementName(e.element).toLowerCase().includes(kw))
    )
  })
})

/** 已拥有判据：服务端 build 快照的 equipment 数组（装备 id 列表）。 */
const ownedIds = computed(() => new Set(store.build?.equipment ?? []))

const ownedCount = computed(() => items.value.filter((e) => ownedIds.value.has(e.id)).length)

function isOwned(id: number): boolean {
  return ownedIds.value.has(id)
}

function slotLabel(s: string): string {
  const map: Record<string, string> = {
    weapon: '武器',
    helmet: '头盔',
    armor: '胸甲',
    gloves: '手套',
    boots: '靴子',
    charm: '护符',
  }
  return map[s] ?? s
}

function tierLabel(t: number): string {
  return `${'★'.repeat(t)}`
}

function elementName(e: Element): string {
  return ELEMENT_NAME[e] ?? e
}

/** R3 FilterBar：按部位筛选，保留原搜索 computed 与配置数据源不动。 */
const r3Slot = ref('')
const r3FilteredItems = computed(() => {
  const base = filteredItems.value
  if (!r3Slot.value) return base
  return base.filter((e) => e.slot === r3Slot.value)
})
const r3EquipPct = computed(() => items.value.length ? Math.round(ownedCount.value / items.value.length * 100) : 0)

/** R3 AI 线索：从各部位天花板里挑尚未拥有的一件，依据来自 bestPerSlot + 持有数。 */
const r3AiEquip = computed(() => {
  const list = bestPerSlot.value
  if (!list.length) return null
  const pick = list.find((b) => !b.owned) ?? list[0]
  return {
    name: pick.name,
    reason: pick.label + '天花板 · 护甲 ' + pick.armor + (pick.owned ? ' · 已拥有' : ' · 未拥有'),
    conclusion: pick.owned
      ? '「' + pick.name + '」已是' + pick.label + '天花板：继续补其他部位'
      : '「' + pick.name + '」是' + pick.label + '天花板（护甲 ' + pick.armor + '）且未拥有：优先刷它',
    basis: '部位天花板 ' + list.length + ' 项 + 已拥有 ' + ownedCount.value + '/' + items.value.length + ' 件',
  }
})
function r3GoStage() {
  uni.navigateTo({ url: '/pages/stage/stage' })
}
/* -------------------------------------------------------------------------- */
/* R3 冷启动配置加载：深链接/刷新直入时自行拉 /config                             */
/* -------------------------------------------------------------------------- */
let pageLoaded = false
let loadInflight: Promise<void> | null = null

async function doLoadPage(): Promise<void> {
  if (!store.config) {
    // 此前本页没有任何加载调用，把「尚未加载」谎报成「装备表为空（配置未加载？）」。
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
/* G120 解锁等级分布 */
.ud-row { display: flex; flex-direction: row; align-items: center; gap: 10rpx; margin: 6rpx 0; }
.ud-label { width: 100rpx; flex-shrink: 0; font-size: 22rpx; }
.ud-track { flex: 1; height: 12rpx; background: var(--panel-3); border-radius: 999rpx; overflow: hidden; }
.ud-fill { height: 100%; background: var(--lightning); border-radius: 999rpx; }
.ud-val { width: 130rpx; text-align: right; font-size: 22rpx; }
.ud-owned { color: var(--ok); margin-left: 6rpx; }

/* G108 部位×阶成长表 */
.tier-matrix { margin: 8rpx 0 16rpx; padding: 12rpx; background: var(--panel-2); border-radius: 12rpx; }
.tm-row { display: flex; flex-direction: row; gap: 4rpx; margin: 4rpx 0; }
.tm-head { flex: 1; text-align: center; font-size: 20rpx; color: var(--muted); font-weight: 600; }
.tm-label { flex: 1.4; text-align: left; }
.tm-cell { flex: 1; text-align: center; font-size: 22rpx; padding: 6rpx 0; background: var(--panel-3); border-radius: 6rpx; }

/* G81 各部位最强装备 */
.best-slots { margin: 8rpx 0 16rpx; padding: 12rpx; background: var(--panel-2); border-radius: 12rpx; }
.bs-row { display: flex; flex-direction: row; align-items: center; flex-wrap: wrap; gap: 8rpx; padding: 6rpx 0; }
.bs-label { width: 100rpx; flex-shrink: 0; }
.bs-name { color: var(--warn); font-weight: 600; }

/* 条目分隔线走共享的 .rxn（styles/common.css），本页无需额外样式 */
</style>