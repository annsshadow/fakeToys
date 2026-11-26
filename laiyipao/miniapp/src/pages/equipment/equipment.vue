<template>
  <view class="page-root">
    <view class="pad safe-bottom">
      <view class="section-kicker section-kicker--ai r3-enter">
        <text class="section-kicker-cn">装备锻造台</text>
        <text class="section-kicker-en">EQUIPMENT FORGE</text>
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
      <view v-if="r6Loading" class="r6-skeleton r6-enter" aria-hidden="true">
        <view class="skeleton-row"><view class="skeleton-bar" style="width: 40%" /></view>
        <view class="skeleton-row"><view class="skeleton-bar" style="width: 84%" /></view>
        <view class="skeleton-row"><view class="skeleton-bar" style="width: 60%" /></view>
      </view>
      <view v-else-if="r6Rec && r6Rec.status !== 'local'" class="r6-rec r6-enter">
        <view class="r6-rec-head">
          <text class="r6-badge">{{ r6Rec.badge }}</text>
          <text class="r6-engine">{{ r6Rec.engineLabel }}</text>
        </view>
        <text class="r6-headline">{{ r6Rec.headline }}</text>
        <view v-if="r6Rec.evidence.length" class="r6-evidence">
          <view v-for="(ev, i) in r6Rec.evidence" :key="'e' + i" class="r6-ev">
            <text class="r6-ev-metric">{{ ev.metric }}</text>
            <text class="r6-ev-val">{{ fmtEv(ev.value) }}{{ ev.unit }}</text>
            <text class="r6-ev-src">{{ ev.source }} · {{ ev.window }}</text>
          </view>
        </view>
        <view class="r6-why">
          <text class="r6-why-toggle" role="button" @click="r6Open = !r6Open" tabindex="0" @keydown.enter="r6Open = !r6Open">为什么推荐 {{ r6Open ? '▴' : '▾' }}</text>
          <block v-if="r6Open">
            <text v-for="(line, i) in r6Rec.basis" :key="'b' + i" class="r6-why-line">{{ line }}</text>
            <text class="r6-why-line">目标：{{ r6Rec.cta.route }}</text>
          </block>
        </view>
        <view v-if="r6Rec.status === 'empty'" class="r6-empty r3-empty" aria-live="polite" role="status">
          <view class="r3-empty-slot">AI</view>
          <text class="r3-empty-title">{{ r6Rec.headline }}</text>
          <text class="r3-empty-reason">{{ r6Rec.basis[0] }} · {{ r6Rec.basis[1] }}</text>
          <text class="r3-empty-action" role="button" @click="r6Go()" tabindex="0" @keydown.enter="r6Go()">{{ r6Rec.cta.label }}</text>
        </view>
        <text v-else class="r6-cta" role="button" :data-route="r6Rec.cta.route" @click="r6Go()" tabindex="0" @keydown.enter="r6Go()">{{ r6Rec.cta.label }} →</text>
      </view>

      <!-- R7 §1F.4：先讲清本局靠什么拿分，再说推荐换上后的预期收益 -->
      <view class="r7-causal r7-causal-enter">
        <text class="r7-causal-title">AI 因果链 · 本局靠什么拿分 → 换上后预计提升</text>
        <text class="r7-causal-text">{{ r7Causal.text }}</text>
        <text v-if="r7Causal.hasHistory" class="r7-causal-next">
          换上推荐：{{ r7Impact ? r7Impact.candidateName + '，预期护甲 ' + fmt(r7Impact.baseline.armorPermille) + '‰ → ' + fmt(r7Impact.candidate.armorPermille) + '‰、每波伤害 ' + fmt(r7Impact.baseline.estimatedDamagePerWave) + ' → ' + fmt(r7Impact.candidate.estimatedDamagePerWave) : '收益数据不可用：没有真实收益预估，不展示预计数值' }}
        </text>
        <view v-else class="r7-causal-empty" aria-live="polite" role="status">
          <text>{{ r7Causal.text }}</text>
          <text class="r7-causal-src">规则引擎 · 依据：battle_records；收益数据不可用，不展示预计数值。</text>
        </view>
        <text class="r7-causal-src">依据：{{ r7Causal.source }} · 本地规则引擎不编造收益数字</text>
      </view>
      <!-- R7 §1F.4：收益数字只来自 impact 端点；本地规则卡不冒充真实预估 -->
      <view v-if="r6Rec" class="r7-impact r7-enter">
        <text class="r7-impact-title">真实收益预估 · 当前 vs 推荐</text>
        <view class="r7-vs-row">
          <text class="r7-vs-label">当前 {{ r7Impact ? r7Impact.replacedName : '—' }}</text>
          <text class="r7-vs-arrow">→</text>
          <text class="r7-vs-label">推荐 {{ r7Impact ? r7Impact.candidateName : '—' }}</text>
        </view>
        <view v-if="r7Impact" class="r7-bar-block">
          <view class="r7-bar-row">
            <text class="r7-bar-label">当前每波伤害</text>
            <view class="r7-bar"><view class="r7-bar-fill" :style="{ width: r7BaseWidth + '%' }" /></view>
            <text class="r7-bar-val">{{ fmt(r7Impact.baseline.estimatedDamagePerWave) }}</text>
          </view>
          <view class="r7-bar-row">
            <text class="r7-bar-label">推荐每波伤害</text>
            <view class="r7-bar r7-bar-next"><view class="r7-bar-fill" :style="{ width: r7NextWidth + '%' }" /></view>
            <text class="r7-bar-val">{{ fmt(r7Impact.candidate.estimatedDamagePerWave) }}</text>
          </view>
          <text class="r7-delta">每波 {{ fmtSigned(r7Impact.changes.damagePerWaveDelta) }}（{{ fmtSigned(r7Impact.changes.damagePerWavePct) }}%）· 战力 {{ fmtSigned(r7Impact.changes.powerDelta) }} · 评分 {{ fmtSigned(r7Impact.changes.ratingTotalDelta) }}</text>
        </view>
        <text v-if="r7Impact" class="r7-star">历史三星率 {{ r7StarText }}%（stars=3 场数占比，不是胜率）· {{ r7Impact.starRateNote }}</text>
        <text v-if="r7Impact" class="r7-why">为什么是它：{{ r7Impact.replacedName }} → {{ r7Impact.candidateName }} · {{ r7Impact.method }}</text>
        <text v-if="r7Impact" class="r7-basis">依据：{{ r7BasisText }} · 配置 {{ r7Impact.configVersion }} · 近 {{ r7Impact.windowDays }} 天 {{ r7Impact.battleCount }} 场</text>
        <text v-if="!r7Impact" class="r7-empty-note">收益数据不可用：{{ r7WhyUnavailable }}。不展示虚构数值。</text>
        <text v-if="r7Impact" class="r7-btn primary" role="button" @click="r7OpenCompare()" tabindex="0" @keydown.enter="r7OpenCompare()">应用推荐 · 先看收益对比 →</text>
      </view>
      <view v-if="r6Rec && r6Rec.status === 'local' && r3AiEquip" class="r3-ai r6-local r3-enter">
        <view class="r3-ai-head">
          <text class="r3-ai-badge">AI</text>
          <text class="r3-ai-title">AI 建议 · 搭配缺口</text>
          <text class="r3-ai-engine">本地规则引擎</text>
        </view>
        <text class="r3-ai-conclusion">{{ r3AiEquip.conclusion }}</text>
        <text class="r3-ai-basis">依据：{{ r3AiEquip.basis }}</text>
        <text class="r3-ai-cta" role="button" @click="r3GoStage" tabindex="0" @keydown.enter="r3GoStage">去关卡看掉落 →</text>
      </view>
      <view v-else-if="!store.configLoading && items.length === 0" class="r3-empty" aria-live="polite" role="status">
        <view class="r3-empty-slot">AI</view>
        <text class="r3-empty-title">还没有装备表数据</text>
        <text class="r3-empty-reason">服务端配置尚未下发，无法比较各部位天花板。</text>
        <text class="r3-empty-action" role="button" @click="r3GoStage()" tabindex="0" @keydown.enter="r3GoStage()">去关卡看掉落 →</text>
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
          v-aria-native="'搜索装备名称 / 部位 / 元素'"
          aria-label="搜索装备名称 / 部位 / 元素"
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

    <!-- R7 两步 CTA：首次点击只预览，确认后才整组替换装备 -->
    <view v-if="r7CompareOpen" class="r7-compare-mask" @click="r7CompareOpen = false">
      <view class="r7-compare r7-enter" @click.stop>
        <text class="r7-compare-title">当前 vs 推荐 · 确认替换</text>
        <view v-if="r7Impact" class="r7-vs-row">
          <text class="r7-vs-label">{{ r7Impact.replacedName }}（当前）</text>
          <text class="r7-vs-arrow">→</text>
          <text class="r7-vs-label">{{ r7Impact.candidateName }}（推荐）</text>
        </view>
        <view v-if="r7Impact" class="r7-vs-row">
          <text class="r7-vs-label">每波伤害 {{ fmt(r7Impact.baseline.estimatedDamagePerWave) }} → {{ fmt(r7Impact.candidate.estimatedDamagePerWave) }}</text>
        </view>
        <view v-if="r7Impact" class="r7-vs-row">
          <text class="r7-vs-label">护甲 {{ fmt(r7Impact.baseline.armorPermille) }}‰ → {{ fmt(r7Impact.candidate.armorPermille) }}‰ · 评分 {{ fmt(r7Impact.baseline.ratingTotal) }} → {{ fmt(r7Impact.candidate.ratingTotal) }}</text>
        </view>
        <text v-if="r7Impact" class="r7-star">历史三星率 {{ r7StarText }}%（stars=3 场数占比，不是胜率）</text>
        <text v-if="r7Impact" class="r7-basis">依据：{{ r7BasisText }} · 配置 {{ r7Impact.configVersion }}</text>
        <text v-if="r7ApplyError" class="r7-error">{{ r7ApplyError }}</text>
        <text v-if="r7ApplyDone" class="apply-done" role="status" aria-live="polite">已替换为「{{ r7ApplyDone }}」，装备配置已更新。可点「完成」关闭。</text>
        <view class="r7-actions">
          <text class="r7-btn" role="button" @click="r7CompareOpen = false" tabindex="0" @keydown.enter="r7CompareOpen = false">{{ r7ApplyDone ? '关闭' : '取消' }}</text>
          <text v-if="r7ApplyDone" class="r7-btn primary" role="button" @click="r7CompareOpen = false" tabindex="0" @keydown.enter="r7CompareOpen = false">完成</text>
          <text v-else class="r7-btn primary" :class="{ 'btn-disabled': r7Applying }" role="button" @click="r7ConfirmApply()" tabindex="0" @keydown.enter="r7ConfirmApply()">{{ r7Applying ? '写入中…' : '确认替换' }}</text>
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
import { ELEMENT_NAME, type Element } from '@/game/elements'
import DefIcon from '@/components/DefIcon.vue'
import { fetchRecommendations, localFallbackCard, type R6Card } from '@/pages/_shared/r6Recommendation'
import { fetchRecommendationImpact, applyEquipmentImpact, type R7ImpactView } from '@/pages/_shared/r7ImpactApply'
import { fetchLatestBattleCausal, normalizeBattleCausal, type BattleCausalView } from '@/pages/_shared/r7GameplayCausal'

const store = useGameStore()
// R12 跨系统成长优先级：口径与首页 aiInsight 一致，四页指向同一短板
const selfRoute = '/pages/equipment/equipment'
const r12Priority = computed(() => computeGrowthPriority(store.buildRating, store.config?.rating_weights))
const r12Cta = computed(() => resolveR12Cta(r12Priority.value, selfRoute))
function r12Go() {
  const p = r12Priority.value
  if (!p) return
  if (r12Cta.value.self) {
    r3Slot.value = ''
    return
  }
  if (p.hint) uni.navigateTo({ url: p.hint })
}

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

/* R6 §1E：先读真实数据推荐接口；接口缺席时回退并明确标注「本地规则引擎」。 */
const r6Loading = ref(true)
const r6Rec = ref<R6Card | null>(null)
const r6Open = ref(false)

/* R7：收益预估与两步确认状态，数值只来自 impact 响应。 */
const r7Impact = ref<R7ImpactView | null>(null)
const r7Error = ref('')
const r7Causal = ref<BattleCausalView>(normalizeBattleCausal(null))
const r7CompareOpen = ref(false)
const r7Applying = ref(false)
const r7ApplyError = ref('')
const r7ApplyDone = ref('')
const r7BaseWidth = computed(() => {
  const v = r7Impact.value
  if (!v) return 0
  const max = Math.max(v.baseline.estimatedDamagePerWave, v.candidate.estimatedDamagePerWave, 1)
  return Math.round(v.baseline.estimatedDamagePerWave / max * 100)
})
const r7NextWidth = computed(() => {
  const v = r7Impact.value
  if (!v) return 0
  const max = Math.max(v.baseline.estimatedDamagePerWave, v.candidate.estimatedDamagePerWave, 1)
  return Math.round(v.candidate.estimatedDamagePerWave / max * 100)
})
/** R11：收益数值统一格式化，避免 5.400000000000001 这类长浮点尾数出现在界面。 */
function fmt(n: number | null | undefined): string {
  return typeof n === 'number' && Number.isFinite(n) ? n.toFixed(1) : '—'
}
function fmtSigned(n: number | null | undefined): string {
  return typeof n === 'number' && Number.isFinite(n) ? (n >= 0 ? '+' : '') + n.toFixed(1) : '—'
}
/** R11补丁：evidence.value 为 number|string；数字走 fmt，字符串（配置版本 sha）原样保留。 */
function fmtEv(v: number | string | null | undefined): string {
  return typeof v === 'number' ? fmt(v) : (v == null ? '—' : String(v))
}
const r7StarText = computed(() => fmt(r7Impact.value?.changes.starRateObserved))
/** R10：无候选 id/replace 时，用服务端 insights[].basis 解释「为什么收益不可用」。 */
const r7WhyUnavailable = computed(() => {
  if (r7Error.value) return r7Error.value
  const b = r6Rec.value?.basis ?? []
  const why = b.filter((x) => /没有|不足|尚|未|缺口|不符合|不满足|优先|需要|缺少/.test(x)).join('；') || b.join('；')
  return why ? '服务端依据：' + why : '推荐接口未提供可执行的候选与替换参数'
})

const r7BasisText = computed(() => {
  const v = r7Impact.value
  if (!v) return '服务端未提供依据'
  const evidence = v.evidence.map((e) => e.metric + ' ' + fmtEv(e.value) + e.unit + '（' + e.source + ' · ' + e.window + '）')
  return v.basis.concat(evidence).join('；')
})

async function loadR6Recommendation() {
  r6Loading.value = true
  r7Impact.value = null
  r7Error.value = ''
  const rec = await fetchRecommendations('equipment')
  if (rec) {
    r6Rec.value = rec
    r7Error.value = ''
    const candidateId = Number(rec.cta.params?.id ?? 0)
    const replacedId = Number(rec.cta.params?.replace ?? 0)
    if (candidateId && replacedId) {
      const result = await fetchRecommendationImpact('equipment', candidateId, replacedId)
      if (result.card) r6Rec.value = result.card
      r7Impact.value = result.impact
      if (!result.impact) r7Error.value = result.error || '收益数据不可用：服务端未提供真实预估'
    } else {
      // R10 §1I.1：权威参数缺失是防作弊设计，不伪造 id/replace；直接展示服务端依据。
      const why = rec.basis.filter((x) => /没有|不足|尚|未|缺口|不符合|不满足|优先|需要|缺少/.test(x)).join('；') || rec.basis.join('；')
      r7Error.value = why
        ? '服务端依据：' + why + '；本次没有可执行的候选/替换参数'
        : '推荐接口未提供可执行的候选与替换参数'
    }
  } else {
    r6Rec.value = localFallbackCard({
      context: "equipment",
      headline: r3AiEquip.value ? r3AiEquip.value.conclusion : '本地规则引擎：装备表尚未到达，暂不写死推荐',
      basis: ['依据：装备护甲/属性搭配取自服务端配置；已拥有 ' + ownedCount.value + '/' + items.value.length + ' 件'],
      evidence: [{ metric: '装备收集', value: ownedCount.value + '/' + items.value.length, unit: '件', source: 'config/store', window: 'now' }],
      ctaLabel: '去关卡看掉落',
      ctaRoute: '/pages/stage/stage',
    })
    r7Error.value = '本地规则引擎没有真实推荐，不调用收益预估，不展示虚构数值'
  }
  r6Loading.value = false
}

function r6Go() {
  const cta = r6Rec.value?.cta
  if (!cta || !cta.route) return
  const query = cta.params
    ? '?' + Object.entries(cta.params).map(([k, v]) => k + '=' + encodeURIComponent(String(v))).join('&')
    : ''
  uni.navigateTo({ url: cta.route + query })
}

void loadR6Recommendation()
void fetchLatestBattleCausal().then((v) => { r7Causal.value = v })

function r7OpenCompare() {
  r7ApplyError.value = ''
  r7ApplyDone.value = ''
  if (!r7Impact.value) {
    r7Error.value = '收益数据不可用：请先等待真实收益预估返回'
    return
  }
  r7CompareOpen.value = true
}

async function r7ConfirmApply() {
  const impact = r7Impact.value
  if (!impact || r7Applying.value) return
  r7Applying.value = true
  r7ApplyError.value = ''
  r7ApplyDone.value = ''
  try {
    await store.loadEquipment()
    const candidate = store.equipment.find((e) => e.id === impact.candidateId)
    if (!candidate) {
      r7ApplyError.value = '推荐装备尚未拥有，未写入任何配置。请重试'
      return
    }
    const current = store.equipment.map((e) => ({ id: e.id, slot: e.slot, equipped: e.equipped }))
    await applyEquipmentImpact(current, impact.candidateId, impact.replacedId, candidate.slot)
    await store.loadEquipment()
    r7ApplyDone.value = impact.candidateName
    // R10：成功提示只在这个遮罩里；若立即关闭，玩家看不到写入结果。等玩家点「完成」。
    void loadR6Recommendation()
  } catch (e) {
    r7ApplyError.value = '写入失败：' + ((e as Error)?.message || '未知错误') + '，原装备配置未改动。请重试'
  } finally {
    r7Applying.value = false
  }
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

/* R10 写入成功反馈：与错误色区分，保持遮罩内可见直到玩家关闭 */
.apply-done { display: block; margin-top: 10rpx; font-size: 22rpx; line-height: 1.5; color: var(--ok); }
</style>