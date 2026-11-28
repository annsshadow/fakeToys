<template>
  <view class="page-root">
    <view class="pad safe-bottom">
      <view class="section-kicker section-kicker--ai r3-enter">
        <text class="section-kicker-cn">技能构筑台</text>
        <text class="section-kicker-en">SKILL LOADOUT</text>
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
      <!-- R23 备战态构筑反应覆盖诊断：纯客户端派生，只读不写构筑 -->
      <view class="r23-coverage-card">
        <view v-if="r23Coverage.status === 'ok'">
          <text class="r23-coverage-head">当前构筑可触发反应</text>
          <view class="r23-coverage-tags">
            <text v-for="k in r23Coverage.covered" :key="k" class="r23-coverage-tag" :data-r23-reaction="k">{{ r23ReactionName(k) }}</text>
          </view>
          <text class="r23-gap-suggest" v-if="r23Coverage.gap">{{ r23Coverage.gap }}</text>
        </view>
        <text v-else class="r23-coverage-empty" role="status" aria-live="polite">{{ r23Coverage.emptyText }}</text>
        <view class="r23-ai-note">
          <text class="r23-ai-badge">AI · 构筑反应诊断</text>
          <text class="r23-ai-conclusion">{{ r23Coverage.aiConclusion }}</text>
          <text class="r23-ai-basis">依据：构筑槽元素 × REACTION_PAIRS(源自引擎 lookupReaction)派生</text>
          <text class="r23-ai-cta" @click="r23GoLoadout">去调整出战配置 →</text>
          <text class="r23-ai-honesty">辅助诊断非伤害承诺，实际触发取决于释放顺序/命中/层数，权威以 replay_hash 为准</text>
        </view>
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
      <view class="r3-progress-row r3-enter">
        <text class="r3-progress-label">收集进度</text>
        <view class="r3-progress"><view class="r3-progress-fill" :style="{ width: r3SkillPct + '%' }" /></view>
        <text class="r3-progress-num">已解锁 {{ ownedCount }}/{{ skills.length }}</text>
      </view>
      <view class="r3-filter" aria-label="按元素筛选技能">
        <text class="r3-chip" :class="{ active: r3Element === '' }" @click="r3Element = ''">全部 {{ skills.length }}</text>
        <text v-for="el in ELEMENT_ORDER" :key="'f' + el" class="r3-chip" :class="{ active: r3Element === el }" @click="r3Element = el">
          {{ ELEMENT_NAME[el] }}
        </text>
      </view>
      <view v-if="r6Loading" class="r6-skeleton r6-enter" role="status" aria-busy="true" aria-label="数据加载中">
        <view class="skeleton-row"><view class="skeleton-bar" style="width: 40%" /></view>
        <view class="skeleton-row"><view class="skeleton-bar" style="width: 88%" /></view>
        <view class="skeleton-row"><view class="skeleton-bar" style="width: 62%" /></view>
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
          换上推荐：{{ r7Impact ? r7Impact.candidateName + '，预期每波伤害 ' + fmt(r7Impact.baseline.estimatedDamagePerWave) + ' → ' + fmt(r7Impact.candidate.estimatedDamagePerWave) + '（' + fmt(r7Impact.changes.damagePerWavePct) + '%）' : '收益数据不可用：没有真实收益预估，不展示预计数值' }}
        </text>
        <view v-else class="r7-causal-empty" aria-live="polite" role="status">
          <text>{{ r7Causal.text }}</text>
          <text class="r7-causal-src">规则引擎 · 依据：battle_records；收益数据不可用，不展示预计数值。</text>
        </view>
        <text class="r7-causal-src">依据：{{ r7Causal.source }} · 本地规则引擎不编造收益数字</text>
      </view>
      <!-- R7 §1F.4：真实收益只来自 impact 端点；接口缺席时只写诚实文案，不编数字 -->
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
      <view v-if="r6Rec && r6Rec.status === 'local' && r3AiSkill" class="r3-ai r6-local r3-enter">
        <view class="r3-ai-head">
          <text class="r3-ai-badge">AI</text>
          <text class="r3-ai-title">AI 建议 · 取舍与热量</text>
          <text class="r3-ai-engine">本地规则引擎</text>
        </view>
        <text class="r3-ai-conclusion">{{ r3AiSkill.conclusion }}</text>
        <text class="r3-ai-basis">依据：{{ r3AiSkill.basis }}</text>
        <text class="r3-ai-cta" role="button" @click="r3GoBag" tabindex="0" @keydown.enter="r3GoBag">去背包查看解锁 →</text>
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
          v-aria-native="'搜索技能名称 / 元素 / 主动被动'"
          aria-label="搜索技能名称 / 元素 / 主动被动"
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
          <text class="muted search-empty-text">没有匹配「{{ keyword }}」的技能</text>
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

    <!-- R7 两步 CTA 的第二步：只有这里能确认写入；失败保留原配置 -->
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
          <text class="r7-vs-label">攻击 {{ fmt(r7Impact.baseline.attackPermille) }}‰ → {{ fmt(r7Impact.candidate.attackPermille) }}‰ · 评分 {{ fmt(r7Impact.baseline.ratingTotal) }} → {{ fmt(r7Impact.candidate.ratingTotal) }}</text>
        </view>
        <text v-if="r7Impact" class="r7-star">历史三星率 {{ r7StarText }}%（stars=3 场数占比，不是胜率）</text>
        <text v-if="r7Impact" class="r7-basis">依据：{{ r7BasisText }} · 配置 {{ r7Impact.configVersion }}</text>
        <text v-if="r7ApplyError" class="r7-error">{{ r7ApplyError }}</text>
        <text v-if="r7ApplyDone" class="apply-done" role="status" aria-live="polite">已替换为「{{ r7ApplyDone }}」，出战配置已更新。可点「完成」关闭。</text>
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
import { ELEMENT_NAME, ELEMENT_ORDER, ELEMENT_COLOR, REACTIONS, REACTION_ORDER, type Element, type ReactionKey } from '@/game/elements'
import { REACTION_PAIRS } from '@/utils/reactionPairs'
import DefIcon from '@/components/DefIcon.vue'
import { fetchRecommendations, localFallbackCard, type R6Card } from '@/pages/_shared/r6Recommendation'
import { fetchRecommendationImpact, type R7ImpactView } from '@/pages/_shared/r7ImpactApply'
import { fetchLatestBattleCausal, normalizeBattleCausal, type BattleCausalView } from '@/pages/_shared/r7GameplayCausal'

const store = useGameStore()
// R12 跨系统成长优先级：口径与首页 aiInsight 一致，四页指向同一短板
const selfRoute = '/pages/skills/skills'
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

/* -------------------------------------------------------------------------- */
/* R23 备战态构筑反应覆盖诊断（纯客户端派生，只读不写 loadout）                  */
/* -------------------------------------------------------------------------- */
function r23ReactionName(k: ReactionKey): string {
  return REACTIONS[k].name
}

function r23Covered(elements: Element[]): ReactionKey[] {
  const set = new Set<ReactionKey>()
  for (const a of elements) {
    for (const b of elements) {
      if (a === b) continue
      for (const k of REACTION_ORDER) {
        if (REACTION_PAIRS[k].some(([x, y]) => x === a && y === b)) set.add(k)
      }
    }
  }
  return REACTION_ORDER.filter((k) => set.has(k))
}

function r23BestGap(elements: Element[]): { element: Element; added: ReactionKey[] } | null {
  if (elements.length === 0) return null
  const before = r23Covered(elements)
  let best: { element: Element; added: ReactionKey[] } | null = null
  for (const e of ELEMENT_ORDER) {
    const added = r23Covered([...elements, e]).filter((k) => !before.includes(k))
    if (added.length === 0) continue
    if (!best || added.length > best.added.length) best = { element: e, added }
  }
  return best
}

const r23Coverage = computed(() => {
  const byId = new Map(allSkillDefs.value.map((s) => [s.id, s]))
  // captain 口径：apply_element 是引擎 applyElement 实际施加元素，与 R22 battle.vue 同源；
  // element 与之同源但 apply_element 才是引擎真值，空串过滤。
  const elements = Array.from(new Set(
    store.loadout.slice(0, 4)
      .map((id) => byId.get(id)?.apply_element)
      .filter((e): e is Element => !!e && ELEMENT_ORDER.includes(e as Element)),
  ))
  const covered = r23Covered(elements)
  const gap = r23BestGap(elements)
  const base = { covered, gap: null as string | null, status: 'ok' as 'ok' | 'empty', emptyText: '' }
  if (elements.length === 0) {
    return { ...base, status: 'empty' as const, emptyText: '当前构筑没有元素技能，无法诊断反应覆盖', aiConclusion: '当前构筑还没有元素技能，无法诊断反应覆盖' }
  }
  if (!gap && covered.length && covered.length >= 5) {
    return { ...base, status: 'empty' as const, emptyText: '当前构筑已覆盖全部可触发反应，无需补齐', aiConclusion: '本套构筑已覆盖全部可触发反应，无需补齐' }
  }
  if (!gap) {
    return { ...base, status: 'empty' as const, emptyText: '当前构筑已覆盖全部可触发反应，无需补齐', aiConclusion: '本套构筑已覆盖全部可触发反应，无需补齐' }
  }
  const gapText = `补【${ELEMENT_NAME[gap.element]}】可新增【${r23ReactionName(gap.added[0]!)}】等${gap.added.length}种反应`
  return {
    ...base,
    gap: gapText,
    aiConclusion: `本套构筑覆盖 ${covered.length} 种反应（${covered.map(r23ReactionName).join('、')}）；${gapText}`,
  }
})

function r23GoLoadout() {
  // 本页即出战构筑页，CTA 只做聚焦提示，不写入任何 store 状态。
  uni.showToast?.({ title: '调整下方出战槽位即可切换元素', icon: 'none' })
}
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
const r3SkillPct = computed(() => skills.value.length ? Math.round(ownedCount.value / skills.value.length * 100) : 0)

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

/* R6 §1E：技能热度优先读战斗历史推荐；接口缺席才走本地规则并明确标注。 */
const r6Loading = ref(true)
const r6Rec = ref<R6Card | null>(null)
const r6Open = ref(false)

/* R7：真实收益对比状态。所有数值来自 impact 响应，接口缺席保持空。 */
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
  const rec = await fetchRecommendations('skills')
  if (rec) {
    r6Rec.value = rec
    r7Error.value = ''
    const candidateId = Number(rec.cta.params?.id ?? 0)
    const replacedId = Number(rec.cta.params?.replace ?? 0)
    if (candidateId && replacedId) {
      const result = await fetchRecommendationImpact('skill', candidateId, replacedId)
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
      context: "skills",
      headline: r3AiSkill.value ? r3AiSkill.value.conclusion : '本地规则引擎：技能表尚未到达，暂不写死推荐',
      basis: ['依据：技能效率榜 + 热量硬约束；热度数据待接口下发'],
      evidence: [{ metric: '技能解锁', value: ownedCount.value + '/' + skills.value.length, unit: '个', source: 'config/store', window: 'now' }],
      ctaLabel: '去背包查看解锁',
      ctaRoute: '/pages/bag/bag',
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
    await store.loadLoadout()
    const current = store.loadout.slice(0, 4)
    while (current.length < 4) current.push(0)
    const slot = current.indexOf(impact.replacedId)
    if (slot < 0) {
      r7ApplyError.value = '当前出战槽位已变化，未写入任何配置。请重试'
      return
    }
    if (current.includes(impact.candidateId)) {
      r7ApplyError.value = '推荐技能已在出战配置中，未重复写入。请重试'
      return
    }
    current[slot] = impact.candidateId
    const ok = await store.persistLoadout(current)
    if (!ok) {
      r7ApplyError.value = '写入失败，原出战配置未改动。请重试'
      return
    }
    r7ApplyDone.value = impact.candidateName
    // R10：成功提示只在这个遮罩里；若立即关闭，玩家看不到写入结果。等玩家点「完成」。
    void loadR6Recommendation()
  } catch (e) {
    r7ApplyError.value = '写入失败：' + ((e as Error)?.message || '未知错误') + '，原出战配置未改动。请重试'
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
  // R23-BUG：深链接/刷新直入时 loadout 仍是默认 [0,0,0,0]，
  // 诊断卡会把「尚未加载权威构筑」谎报成空构筑。必须同幂等路径拉一次权威槽位。
  // loadLoadout 自带序号守卫且失败回退本地值，幂等安全。
  await store.loadLoadout()
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

/* R10 写入成功反馈：与错误色区分，保持遮罩内可见直到玩家关闭 */
.apply-done { display: block; margin-top: 10rpx; font-size: 22rpx; line-height: 1.5; color: var(--ok); }

/* R23 构筑反应覆盖诊断：沿用既有 token，零新增视觉令牌 */
.r23-coverage-card {
  margin: 12rpx 0;
  padding: 12rpx;
  border-left: 4rpx solid var(--accent);
  border-radius: 10rpx;
  background: var(--panel-2);
  display: flex;
  flex-direction: column;
  gap: 6rpx;
}
.r23-coverage-head { font-size: 22rpx; font-weight: 700; color: var(--accent); }
.r23-coverage-tags { display: flex; flex-wrap: wrap; gap: 6rpx; }
.r23-coverage-tag {
  padding: 3rpx 8rpx;
  border: 1rpx solid var(--fire);
  border-radius: 8rpx;
  color: var(--fire);
  font-size: 20rpx;
}
.r23-gap-suggest { color: var(--warn); font-size: 21rpx; }
.r23-coverage-empty { color: var(--text-2); font-size: 21rpx; }
.r23-ai-note { display: flex; flex-direction: column; gap: 3rpx; margin-top: 6rpx; }
.r23-ai-badge { color: var(--accent); font-weight: 700; font-size: 21rpx; }
.r23-ai-conclusion { color: var(--text); font-size: 21rpx; }
.r23-ai-basis, .r23-ai-honesty { color: var(--text-2); font-size: 19rpx; }
.r23-ai-cta { color: var(--accent); font-size: 20rpx; font-weight: 700; }
</style>