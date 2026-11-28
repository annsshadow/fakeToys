<template>
  <view class="page-root">
    <view class="pad safe-bottom">
      <view class="section-kicker section-kicker--ai r3-enter">
        <text class="section-kicker-cn">反应作战台</text>
        <text class="section-kicker-en">REACTION MATRIX</text>
      </view>
      <view class="r3-kpi r3-enter">
        <view class="r3-kpi-main">
          <text class="r3-kpi-num">{{ myReactions.length }}/{{ allReactions.length }}</text>
          <text class="r3-kpi-label">当前构筑可达反应</text>
          <text class="r3-kpi-sub">{{ r3AiReaction ? r3AiReaction.reason : '元素构筑数据尚未到达' }}</text>
        </view>
        <view class="r3-kpi-cell">
          <text class="r3-kpi-num">{{ store.equippedElements.length }}</text>
          <text class="r3-kpi-label">出战元素</text>
        </view>
        <view class="r3-kpi-cell">
          <text class="r3-kpi-num">{{ r3FilteredReactions.length }}</text>
          <text class="r3-kpi-label">当前筛选结果</text>
        </view>
      </view>
      <view class="r3-progress-row is-ai r3-enter">
        <text class="r3-progress-label">反应覆盖</text>
        <view class="r3-progress"><view class="r3-progress-fill" :style="{ width: r3ReactionPct + '%' }" /></view>
        <text class="r3-progress-num">已达成 {{ myReactions.length }}/{{ allReactions.length }}</text>
      </view>
      <view class="r3-filter" aria-label="按元素筛选反应">
        <text class="r3-chip" :class="{ active: r3Element === '' }" @click="r3Element = ''">全部 {{ allReactions.length }}</text>
        <text v-for="el in ELEMENT_ORDER" :key="'f' + el" class="r3-chip" :class="{ active: r3Element === el }" @click="r3Element = el">
          {{ ELEMENT_NAME[el] }}
        </text>
      </view>

      <!-- R19：玩家逐次反应伤害贡献（辅助分析，非权威结算） -->
      <view class="card r19-my-reaction-impact r3-enter" role="region" aria-label="我的反应伤害贡献">
        <view class="r19-rdi-head">
          <text class="r19-rdi-title">我的反应伤害贡献</text>
          <text class="r19-rdi-window">{{ rdiWindowText }}</text>
        </view>
        <view class="r19-view-toggle" aria-label="贡献视图切换">
          <text class="r19-toggle-btn" :class="{ active: rdiView === 'count' }" @click="rdiView = 'count'">次数</text>
          <text class="r19-toggle-btn" :class="{ active: rdiView === 'damage' }" @click="rdiView = 'damage'">伤害</text>
        </view>
        <view v-if="!rdiAvailable || rdiItems.length === 0" class="r19-rdi-empty">近 30 天无逐次反应伤害记录</view>
        <view v-else class="r19-rdi-list">
          <view v-for="row in rdiRows" :key="row.reaction" class="r19-rdi-row" :data-r19-reaction="row.reaction">
            <text class="r19-rdi-name">{{ row.name }}</text>
            <text v-if="rdiView === 'count'" class="r19-rdi-metric">权威 {{ row.count }} 次 · 事件 {{ row.events }} 条</text>
            <text v-else class="r19-rdi-metric">占比 {{ rdiPct(row.damage_share_pct) }} · 总伤害 {{ rdiDamage(row.total_damage) }}</text>
          </view>
        </view>
        <view class="r19-rdi-meta">已排除 {{ rdiExcluded }} 条无逐次记录的旧战报</view>
        <view class="r19-ai-note">
          <text class="r19-ai-title">AI · 我的反应伤害洞察</text>
          <text class="r19-ai-conclusion">{{ rdiAiConclusion }}</text>
          <text class="r19-ai-basis">依据：近 30 天 reaction_timeline.events 的 damage 聚合；触发次数以 reactions_used 为准</text>
          <text class="r19-ai-cta" @click="rdiGoAuthority">查看权威结算 →</text>
        </view>
        <view class="r19-rdi-honesty">辅助分析非权威结算依据，权威请看 replay_hash</view>
      </view>
      <view v-if="r6Loading" class="r6-skeleton r6-enter" role="status" aria-busy="true" aria-label="数据加载中">
        <view class="skeleton-row"><view class="skeleton-bar" style="width: 42%" /></view>
        <view class="skeleton-row"><view class="skeleton-bar" style="width: 86%" /></view>
        <view class="skeleton-row"><view class="skeleton-bar" style="width: 58%" /></view>
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
      <view v-if="r6Rec && r6Rec.status === 'local' && r3AiReaction" class="r3-ai r6-local r3-enter">
        <view class="r3-ai-head">
          <text class="r3-ai-badge">AI</text>
          <text class="r3-ai-title">AI 建议 · 下一条反应</text>
          <text class="r3-ai-engine">本地规则引擎</text>
        </view>
        <text class="r3-ai-conclusion">{{ r3AiReaction.conclusion }}</text>
        <text class="r3-ai-basis">依据：{{ r3AiReaction.basis }}</text>
        <text class="r3-ai-cta" role="button" @click="r3GoSkills" tabindex="0" @keydown.enter="r3GoSkills">去调整出战技能 →</text>
      </view>
      <!-- 加载中 / 空态（2026-10-21 P6-1）-->
      <view v-if="store.configLoading" class="card">
        <text class="muted">加载中…</text>
      </view>
      <view v-else-if="allReactions.length === 0" class="card">
        <text class="muted">反应表还没加载出来</text>
      </view>
      <!-- 我的构筑能打出哪些反应 -->
      <view class="card">
        <text class="card-title">我的构筑能触发的反应</text>
        <view v-if="myReactions.length" class="chip-row">
          <view v-for="k in myReactions" :key="k" class="chip hit">{{ reactionName(k) }}</view>
        </view>
        <text v-else class="muted">
          当前出战技能的元素凑不出任何反应链 —— 反应伤害中元素侧是主力，面板攻击力也有贡献但权重上限 30%，
          换一种元素搭配比堆面板更有效。
        </text>
        <text v-if="myReactions.length" class="muted" style="display: block; margin-top: 12rpx">
          共 {{ myReactions.length }} / {{ allReactions.length }} 条，还差 {{ allReactions.length - myReactions.length }} 条未解锁。反应伤害里攻击力的权重被结构性限制在
          30% 以内，所以「凑齐元素」比「堆攻击力」更决定胜负。
        </text>
      </view>

      <!-- G74 元素反应覆盖矩阵：5×5，行=元素A、列=元素B，格子=反应名；可达高亮 -->
      <view class="card">
        <text class="card-title">元素反应覆盖矩阵</text>
        <text class="dim" style="display: block; margin-bottom: 12rpx">
          行 × 列 = 两元素组合；亮色格 = 当前构筑能触发的反应，灰格 = 暂不可达。
        </text>
        <view class="matrix">
          <!-- 表头：列元素名 -->
          <view class="matrix-row">
            <view class="matrix-corner" />
            <view v-for="b in ELEMENT_ORDER" :key="'h' + b" class="matrix-head" :style="{ color: ELEMENT_COLOR[b] }">
              {{ ELEMENT_NAME[b] }}
            </view>
          </view>
          <!-- 每行：行元素名 + 5 个格子 -->
          <view v-for="row in reactionMatrix" :key="'r' + row[0]!.a" class="matrix-row">
            <view class="matrix-head" :style="{ color: ELEMENT_COLOR[row[0]!.a] }">
              {{ ELEMENT_NAME[row[0]!.a] }}
            </view>
            <view
              v-for="cell in row"
              :key="cell.a + '-' + cell.b"
              class="matrix-cell"
              :class="{ hit: cell.reachable, self: cell.self }"
              :style="{ background: cellColor(cell) }"
            >{{ cell.self ? '·' : cell.name }}</view>
          </view>
        </view>
      </view>

      <!-- G105 反应机制对照表：谁驱散/削甲/击退/增幅/持续，一眼扫完 -->
      <view class="card">
        <text class="card-title">反应机制对照</text>
        <view class="mech-row mech-head">
          <text class="dim mech-name">反应</text>
          <text v-for="h in ['驱散','削甲','击退','增幅','持续','传播','溅射']" :key="h" class="dim mech-cell">{{ h }}</text>
        </view>
        <view v-for="m in mechanismTable" :key="m.key" class="mech-row">
          <text class="mech-name">{{ m.name }}</text>
          <text v-for="(c, i) in m.cells" :key="i" class="mech-cell" :class="{ on: c.on }">{{ c.on ? '●' : '·' }}</text>
        </view>
      </view>

      <!-- G123 反应基础强度排行 -->
      <view class="card">
        <view class="card-title">反应强度<text class="dim">基础强度，非实战伤害</text></view>
        <view v-for="r in strengthRank" :key="r.key" class="sr-row">
          <text class="sr-name">{{ r.name }}</text>
          <view class="sr-track"><view class="sr-fill" :style="{ width: r.pct + '%' }" /></view>
          <text class="mono sr-val">{{ r.label }}</text>
        </view>
      </view>

      <!-- 反应图鉴（服务端下发的权威表） -->
      <view class="card stagger-in">
        <text class="card-title">反应图鉴</text>
        <!-- 搜索（2026-11-01 新一轮第 77 轮）：按反应名过滤 -->
        <input
          v-model="keyword"
          v-aria-native="'搜索反应名称'"
          aria-label="搜索反应名称"
          class="search-inp"
          placeholder="搜索反应名称"
          placeholder-class="ph"
        />
        <text class="muted" style="display: block; margin-bottom: 12rpx">
          数值直接来自服务端 /config，与游戏内实际生效的一致。
        </text>
        <text class="result-count">显示 {{ r3FilteredReactions.length }} / 共 {{ allReactions.length }} 条</text>
        <view class="r3-data-grid">
        <view v-for="r in r3FilteredReactions" :key="r.key" class="rxn list-row">
          <view class="rxn-head">
            <!-- 反应图标：双环交叠（两个元素圈相碰的隐喻） -->
            <view class="icon-badge" style="width: 64rpx; height: 64rpx">
              <DefIcon name="reaction" :size="40" />
            </view>
            <text class="rxn-name">{{ r.name }}</text>
            <!-- R66 可达反应加脉冲；第六批 R53：不可达也明示，不再只有色块需要对照 -->
            <text v-if="reachable.has(r.key as ReactionKey)" class="rxn-hit reachable-pulse">可达</text>
            <text v-else class="chip">当前不可达</text>
          </view>
          <!-- 触发元素对（本地由 lookupReaction 反查，与服务端反应表同源） -->
          <view v-if="reactionPairs(r.key)" class="rxn-stats">
            <view v-for="(pair, i) in reactionPairs(r.key)" :key="i" class="pair-chip">
              <text class="tag" :class="'tag-' + pair[0]">{{ ELEMENT_NAME[pair[0]] }}</text>
              <text class="dim">+</text>
              <text class="tag" :class="'tag-' + pair[1]">{{ ELEMENT_NAME[pair[1]] }}</text>
            </view>
          </view>
          <text class="rxn-descr">{{ r.descr || '（服务端未提供说明）' }}</text>
          <view class="rxn-stats">
            <text class="stat">系数 {{ r.base_coef }}</text>
            <text class="stat">攻击力权重 {{ r.attack_weight_pct / 10 }}%</text>
            <text v-if="r.status_duration_ms > 0" class="stat">
              持续 {{ (r.status_duration_ms / 1000).toFixed(1) }}s
            </text>
            <text v-if="r.aoe_radius > 0" class="stat">{{ r.spread_stacks > 0 ? '传播半径' : '溅射半径' }} {{ r.aoe_radius }}</text>
            <text v-if="r.spread_stacks > 0" class="stat">层数传播 {{ r.spread_stacks }}</text>
            <text v-if="r.dispel_shield" class="stat">驱散护盾</text>
            <text v-if="r.amplify_pct > 0" class="stat">增幅 {{ r.amplify_pct / 10 }}%</text>
            <text v-if="r.armor_shred_permille > 0" class="stat">
              削甲 {{ r.armor_shred_permille }}‰
            </text>
            <text v-if="r.knockback > 0" class="stat">击退 {{ r.knockback / 1000 }}</text>
          </view>
          <!-- R14 §C.2：按服务端权威元素取建议；E2E 选择器 #r14-advice-action-btn.r14-advice-cta[data-r14-source=reactions]，空态 .r14-advice-empty -->
          <view v-if="r14AdviceOf(r)" class="r14-advice r14-enter">
            <text class="r14-advice-badge">{{ r14AdviceOf(r)?.badge }} · 可执行建议</text>
            <block v-if="r14AdviceOf(r)?.status === 'ready'">
              <text class="r14-advice-text">换上「{{ r14AdviceOf(r)?.candidateName }}」{{ r14AdviceOf(r)?.expectedReaction ? '可指向 ' + r14AdviceOf(r)?.expectedReaction : '可提升当前构筑（服务端未给出目标反应）' }}</text>
              <text class="r14-advice-basis">依据：{{ r14AdviceOf(r)?.basedOn.join(' · ') }}
 · 服务端五条件裁决（不伪造 id）</text>
              <text id="r14-advice-action-btn" class="r14-advice-cta" role="button" data-r14-source="reactions" @click="r14OpenCompare(r14AdviceOf(r))" tabindex="0" @keydown.enter="r14OpenCompare(r14AdviceOf(r))">一键替换为 {{ r14AdviceOf(r)?.candidateName }} →</text>
              <text v-if="r14ApplyDone" class="apply-done r14-advice-done" role="status" aria-live="polite">已替换为「{{ r14ApplyDone }}」，出战配置已更新。</text>
            </block>
            <view v-else class="r14-advice-empty" data-r14-source="reactions" aria-live="polite" role="status">
              <text class="r14-advice-empty-title">{{ r14AdviceOf(r)?.empty?.headline }}</text>
              <text class="r14-advice-empty-msg">{{ r14AdviceOf(r)?.empty?.message }}</text>
              <text class="r14-advice-empty-cta" role="button" @click="r14GoEmpty(r14AdviceOf(r))" tabindex="0" @keydown.enter="r14GoEmpty(r14AdviceOf(r))">{{ r14AdviceOf(r)?.empty?.cta }}</text>
            </view>
          </view>
        </view>
        </view>
        <text v-if="!allReactions.length" class="muted">反应表为空（配置未加载？）</text>
        <!-- R28 搜索无结果：明确区分「表是空的」与「关键词没匹配上」 -->
        <view v-if="keyword && allReactions.length && r3FilteredReactions.length === 0" class="search-empty">
          <text class="muted search-empty-text">没有匹配「{{ keyword }}」的反应</text>
          <text class="clear-search" @click="keyword = ''">清除搜索</text>
        </view>
      </view>
    </view>
    <!-- R14 §C.3：复用 R7 两步确认浮层；首次点击只预览，确认才写入 -->
    <view v-if="r14CompareOpen" class="r7-compare-mask" @click="r14CompareOpen = false">
      <view class="r7-compare r7-enter" @click.stop>
        <text class="r7-compare-title">当前 vs 推荐 · 确认替换</text>
        <view class="r7-vs-row">
          <text class="r7-vs-label">{{ r14Current?.replacedName }}（当前）</text>
          <text class="r7-vs-arrow">→</text>
          <text class="r7-vs-label">{{ r14Current?.candidateName }}（推荐）</text>
        </view>
        <view v-if="r14Current?.impact" class="r7-vs-row">
          <text class="r7-vs-label">每波伤害 {{ fmtEv(r14Current.impact.baseline.estimatedDamagePerWave) }} → {{ fmtEv(r14Current.impact.candidate.estimatedDamagePerWave) }}</text>
        </view>
        <text v-if="r14Current" class="r7-basis">依据：{{ r14Current.basedOn.join(' · ') }}</text>
        <text v-if="r14ApplyError" class="r7-error">{{ r14ApplyError }}</text>
        <text v-if="r14ApplyDone" class="apply-done" role="status" aria-live="polite">已替换为「{{ r14ApplyDone }}」，出战配置已更新。可点「完成」关闭。</text>
        <view class="r7-actions">
          <text class="r7-btn" role="button" @click="r14CompareOpen = false" tabindex="0" @keydown.enter="r14CompareOpen = false">{{ r14ApplyDone ? '关闭' : '取消' }}</text>
          <text v-if="r14ApplyDone" class="r7-btn primary" role="button" @click="r14CompareOpen = false" tabindex="0" @keydown.enter="r14CompareOpen = false">完成</text>
          <text v-else class="r7-btn primary" :class="{ 'btn-disabled': r14Applying }" role="button" @click="r14ConfirmApply()" tabindex="0" @keydown.enter="r14ConfirmApply()">{{ r14Applying ? '写入中…' : '确认替换' }}</text>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import { onLoad } from '@dcloudio/uni-app'
import { useGameStore } from '@/store/game'
import { ELEMENT_NAME, ELEMENT_ORDER, ELEMENT_COLOR, lookupReaction, reachableReactions, type Element, type ReactionKey } from '@/game/elements'
import { REACTION_PAIRS } from '@/utils/reactionPairs'
import DefIcon from '@/components/DefIcon.vue'
import { fetchRecommendations, localFallbackCard, type R6Card } from '@/pages/_shared/r6Recommendation'
import { fetchAdviceAction, applyAdviceAction, type R14AdviceAction } from '@/pages/_shared/r14AdviceAction'
import { fetchMyReactionDamageImpact, type ReactionDamageImpact, type ReactionImpactItem } from '@/api/client'

const store = useGameStore()

/** 反应 → 触发元素对（派生表见 utils/reactionPairs，与图例共用）。 */
function reactionPairs(key: string): Array<[Element, Element]> {
  return REACTION_PAIRS[key as ReactionKey] ?? []
}

/**
 * 反应表（服务端 /config 下发）。
 *
 * 2026-10-21（P4-2）：此前 `/config.reactions` **全客户端零消费** ——
 * 服务端把 7 条反应的完整参数（系数 / 攻击力权重 / 持续时间 / 溅射半径 /
 * 驱散 / 增幅 / 削甲 / 击退）都下发下来了，客户端一处都没读。
 *
 * 后果：玩家查不到「冰 + 雷 = 什么反应、伤害多少、持续多久」，
 * 而反应驱动是这套设计的**核心机制**（I-1）。
 * 数值在服务端、内容在服务端，唯独**解释**缺失。
 */
const allReactions = computed(() => store.config?.reactions ?? [])

/** 图鉴搜索（2026-11-01 新一轮第 77 轮）：按反应名过滤。 */
const keyword = ref('')

const filteredReactions = computed(() => {
  const kw = keyword.value.trim().toLowerCase()
  if (!kw) return allReactions.value
  return allReactions.value.filter((r) => r.name.toLowerCase().includes(kw))
})

/**
 * 我的构筑可达的反应（P4-3）。
 *
 * `reachableReactions` 此前也只有测试在调 —— 玩家看不到
 * 「我这个构筑能打出哪些反应链」，只能打完一局才知道。
 */
const reachable = computed(() => reachableReactions(new Set(store.equippedElements)))

/* -------------------------------------------------------------------------- */
/* G74 元素反应覆盖矩阵（第七批）                                               */
/* 5×5 穷举元素对 → 反应；可达的（当前构筑能触发）高亮，不可达的灰显。            */
/* 数据来自引擎同款 lookupReaction + 服务端反应表，纯客户端派生，零契约改动。     */
/* -------------------------------------------------------------------------- */
interface MatrixCell {
  a: Element
  b: Element
  key: ReactionKey | null
  name: string
  reachable: boolean
  self: boolean
}
/** G123 反应基础强度排行：按 base_coef × 增幅 排（口径是基础强度，非实战伤害）。 */
const strengthRank = computed(() => {
  const arr = allReactions.value.map((r) => ({
    key: r.key,
    name: r.name,
    score: Number(r.base_coef ?? 0) * (1 + Number(r.amplify_pct ?? 0) / 1000),
  })).sort((a, b) => b.score - a.score)
  const max = Math.max(1, ...arr.map((a) => a.score))
  return arr.map((a) => ({ ...a, pct: Math.round(a.score / max * 100), label: Math.round(a.score * 100) / 100 }))
})

/** G105 反应机制对照表：7 种反应 × 5 种效果，有则点亮 —— 搭配前的速查表。 */
const mechanismTable = computed(() =>
  allReactions.value.map((r) => ({
    key: r.key,
    name: r.name,
    cells: [
      { on: !!r.dispel_shield, label: '驱散' },
      { on: Number(r.armor_shred_permille ?? 0) > 0, label: '削甲' },
      { on: Number(r.knockback ?? 0) > 0, label: '击退' },
      { on: Number(r.amplify_pct ?? 0) > 0, label: '增幅' },
      { on: Number(r.status_duration_ms ?? 0) > 0, label: '持续' },
      { on: Number(r.spread_stacks ?? 0) > 0, label: '传播' },
      { on: Number(r.aoe_radius ?? 0) > 0 && Number(r.spread_stacks ?? 0) === 0, label: '溅射' },
    ],
  })),
)

const reactionMatrix = computed<MatrixCell[][]>(() =>
  ELEMENT_ORDER.map((a) =>
    ELEMENT_ORDER.map((b) => {
      const key = a === b ? null : lookupReaction(a, b)
      const def = key ? allReactions.value.find((r) => r.key === key) : null
      return {
        a, b, key,
        name: def?.name ?? (a === b ? '—' : '无反应'),
        reachable: !!key && reachable.value.has(key),
        self: a === b,
      }
    }),
  ),
)
function cellColor(cell: MatrixCell): string {
  if (cell.self) return 'var(--panel-3)'
  if (!cell.key) return 'var(--panel-2)'
  return cell.reachable ? ELEMENT_COLOR[cell.a] : 'var(--panel-2)'
}

const myReactions = computed<ReactionKey[]>(() => {
  const out: ReactionKey[] = []
  for (const r of allReactions.value) {
    if (reachable.value.has(r.key as ReactionKey)) out.push(r.key as ReactionKey)
  }
  return out
})

function reactionName(key: string): string {
  return allReactions.value.find((r) => r.key === key)?.name ?? key
}

/**
 * 加载本页需要的数据。
 *
 * ⚠️ 2026-10-21（P1-2）：此前这个页面**什么都不加载** ——
 * 不 `login()`、不 `loadConfig()`、不 `loadLoadout()`。
 *
 * 后果：「我的构筑能触发的反应」**永远是空的**，
 * 而空的时候页面会**一本正经地说**「当前出战技能的元素凑不出任何反应链」
 * —— 一句**假话**（玩家明明是火+冰构筑）。
 *
 * 而且测试是绿的：测试自己手工调了 `store.loadLoadout()`，
 * 替页面做了页面该做的事。
 */
async function loadPage(): Promise<void> {
  if (!store.loggedIn) await store.login()
  await store.loadConfig()
  await store.loadLoadout()
}

onLoad(() => {
  void loadPage()
  void loadRdiImpact()
})

onMounted(() => {
  // onLoad 在 H5 上可能早于组件挂载，这里补一次（loadPage/loadRdiImpact 本身幂等）。
  void loadPage()
  void loadRdiImpact()
})

/** R3 FilterBar：在既有搜索结果上按元素二次筛选，不改动原搜索 computed。 */
const r3Element = ref<Element | ''>('')
const r3FilteredReactions = computed(() => {
  const base = filteredReactions.value
  if (!r3Element.value) return base
  return base.filter((r) => reactionPairs(r.key).some((pair) => pair.includes(r3Element.value as Element)))
})
const r3ReactionPct = computed(() => {
  const total = allReactions.value.length
  return total ? Math.round(myReactions.value.length / total * 100) : 0
})

/** R3 AI 线索：取第一条不可达反应 + 其元素对，依据来自 reachable / 反应表。 */
const r3AiReaction = computed(() => {
  const list = allReactions.value
  if (!list.length) return null
  const miss = list.filter((r) => !reachable.value.has(r.key as ReactionKey))
  const pick = miss[0] ?? list[0]
  const pair = reactionPairs(pick.key)[0]
  const pairText = pair ? ELEMENT_NAME[pair[0]] + ' + ' + ELEMENT_NAME[pair[1]] : '对应元素对'
  return {
    reason: miss.length ? '还差 ' + miss.length + ' 条反应未可达' : '全部反应已可达',
    conclusion: miss.length
      ? '还差 ' + miss.length + ' 条反应未可达：先凑「' + pick.name + '」的 ' + pairText
      : '已覆盖全部 ' + list.length + ' 条反应：保持当前元素搭配',
    basis: '当前出战 ' + store.equippedElements.length + ' 种元素 + ' + allReactions.value.length + ' 条反应表',
  }
})
function r3GoSkills() {
  uni.navigateTo({ url: '/pages/skills/skills' })
}

/** R11补丁：evidence.value 为 number|string；数字一位小数，字符串（配置版本 sha）原样。 */
function fmtEv(v: number | string | null | undefined): string {
  return typeof v === 'number' ? (Number.isFinite(v) ? v.toFixed(1) : '—') : (v == null ? '—' : String(v))
}

/* R19：玩家反应伤害贡献卡。辅助分析，绝不参与/冒充权威结算。 */
const rdiImpact = ref<ReactionDamageImpact | null>(null)
const rdiAvailable = ref(false)
const rdiView = ref<'count' | 'damage'>('count')

function rdiNum(v: unknown): number {
  return typeof v === 'number' && Number.isFinite(v) ? v : 0
}
function rdiPct(v: number): string {
  return rdiNum(v).toFixed(1) + '%'
}
function rdiDamage(v: number): string {
  return rdiNum(v).toFixed(1)
}

async function loadRdiImpact() {
  try {
    const data = await fetchMyReactionDamageImpact()
    const items = Array.isArray(data?.items) ? data.items.filter((it): it is ReactionImpactItem =>
      !!it && typeof it.reaction === 'string' && Number.isFinite(Number(it.count))) : []
    rdiImpact.value = {
      window_days: rdiNum(data?.window_days),
      window_from: String(data?.window_from ?? ''),
      window_to: String(data?.window_to ?? ''),
      excluded_old_battles: rdiNum(data?.excluded_old_battles),
      items,
    }
    rdiAvailable.value = true
  } catch {
    // 端点尚未全量铺开或网络失败：走诚实空态，不编造数据。
    rdiImpact.value = null
    rdiAvailable.value = false
  }
}

const rdiItems = computed<ReactionImpactItem[]>(() => rdiImpact.value?.items ?? [])
const rdiExcluded = computed(() => rdiNum(rdiImpact.value?.excluded_old_battles))
const rdiWindowText = computed(() => {
  const d = rdiImpact.value
  return d && d.window_from && d.window_to ? d.window_from + ' ~ ' + d.window_to + '（' + rdiNum(d.window_days) + ' 天）' : '近 30 天'
})
const rdiRows = computed(() => {
  const list = [...rdiItems.value].map((it) => ({ ...it, name: reactionName(it.reaction) }))
  if (rdiView.value === 'damage') list.sort((a, b) => rdiNum(b.damage_share_pct) - rdiNum(a.damage_share_pct))
  return list
})
const rdiAiConclusion = computed(() => {
  const sorted = [...rdiItems.value].sort((a, b) => rdiNum(b.count) - rdiNum(a.count))
  const top = sorted[0]
  if (!top) return '暂无可用数据'
  const damageTop = [...rdiItems.value].sort((a, b) => rdiNum(b.damage_share_pct) - rdiNum(a.damage_share_pct))[0]
  if (damageTop && damageTop.reaction !== top.reaction) {
    return reactionName(top.reaction) + ' 触发 ' + rdiNum(top.count) + ' 次但伤害贡献仅 ' + rdiPct(top.damage_share_pct) + '，伤害主力是 ' + reactionName(damageTop.reaction) + '（' + rdiPct(damageTop.damage_share_pct) + '），疑似高频低伤'
  }
  return reactionName(top.reaction) + ' 触发 ' + rdiNum(top.count) + ' 次，贡献 ' + rdiPct(top.damage_share_pct) + ' 的伤害，是当前主力反应'
})
function rdiGoAuthority() {
  uni.navigateTo({ url: '/pages/battles/battles' })
}

/* R6 §1E：反应链结论优先读真实战斗/配置推荐；接口缺席才用本地规则并标注。 */
const r6Loading = ref(true)
const r6Rec = ref<R6Card | null>(null)
const r6Open = ref(false)

async function loadR6Recommendation() {
  r6Loading.value = true
  const rec = await fetchRecommendations('reactions')
  if (rec) {
    r6Rec.value = rec
  } else {
    r6Rec.value = localFallbackCard({
      context: "reactions",
      headline: r3AiReaction.value ? r3AiReaction.value.conclusion : '本地规则引擎：反应表尚未到达，暂不写死推荐',
      basis: ['依据：当前出战元素 + 反应表（' + allReactions.value.length + ' 条）；触发率数据待接口下发'],
      evidence: [{ metric: '可达反应', value: myReactions.value.length + '/' + allReactions.value.length, unit: '条', source: 'config/store', window: 'now' }],
      ctaLabel: '去调整出战技能',
      ctaRoute: '/pages/skills/skills',
    })
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

/* R14 §C.2：图鉴条目按服务端权威元素 key 取建议，绝不拼 id/replace。 */
const r14AdviceMap = ref<Record<string, R14AdviceAction | null>>({})
const r14Current = ref<R14AdviceAction | null>(null)
const r14CompareOpen = ref(false)
const r14Applying = ref(false)
const r14ApplyError = ref('')
const r14ApplyDone = ref('')

function r14ElementOf(r: { key: string }): string {
  return reactionPairs(r.key)[0]?.[0] ?? ''
}
function r14AdviceOf(r: { key: string }): R14AdviceAction | null {
  return r14AdviceMap.value[r14ElementOf(r)] ?? null
}
async function loadR14Advice(element: string) {
  if (!element || Object.prototype.hasOwnProperty.call(r14AdviceMap.value, element)) return
  const a = await fetchAdviceAction({ element, source: 'reactions' })
  r14AdviceMap.value = { ...r14AdviceMap.value, [element]: a }
}
watch(r3FilteredReactions, (list) => {
  for (const r of list) void loadR14Advice(r14ElementOf(r))
}, { immediate: true })

function r14OpenCompare(a: R14AdviceAction | null) {
  if (!a || a.status !== 'ready') return
  r14Current.value = a
  r14ApplyError.value = ''
  r14ApplyDone.value = ''
  r14CompareOpen.value = true
}

function r14GoEmpty(a: R14AdviceAction | null) {
  const route = a?.empty?.route || '/pages/stage/stage'
  uni.navigateTo({ url: route.startsWith('/pages/') ? route : '/pages/stage/stage' })
}

async function r14ConfirmApply() {
  const a = r14Current.value
  if (!a || a.status !== 'ready' || r14Applying.value) return
  r14Applying.value = true
  r14ApplyError.value = ''
  r14ApplyDone.value = ''
  try {
    const res = await applyAdviceAction(a, store)
    if (!res.ok) { r14ApplyError.value = res.error; return }
    r14ApplyDone.value = res.doneName
    const fresh = await fetchAdviceAction({ element: a.element, source: 'reactions' })
    r14AdviceMap.value = { ...r14AdviceMap.value, [a.element]: fresh }
  } catch (e) {
    r14ApplyError.value = '写入失败：' + ((e as Error)?.message || '未知错误') + '，原配置未改动。请重试'
  } finally {
    r14Applying.value = false
  }
}

void loadR6Recommendation()
</script>

<style scoped>
/* G123 反应强度排行 */
.sr-row { display: flex; flex-direction: row; align-items: center; gap: 10rpx; margin: 8rpx 0; }
.sr-name { width: 160rpx; flex-shrink: 0; font-size: 22rpx; }
.sr-track { flex: 1; height: 12rpx; background: var(--panel-3); border-radius: 999rpx; overflow: hidden; }
.sr-fill { height: 100%; background: var(--corrosion); border-radius: 999rpx; }
.sr-val { width: 80rpx; text-align: right; font-size: 22rpx; color: var(--corrosion); }

/* G105 反应机制对照表 */
.mech-row { display: flex; flex-direction: row; align-items: center; padding: 6rpx 0; }
.mech-name { flex: 1.4; font-size: 22rpx; }
.mech-cell { flex: 1; text-align: center; font-size: 24rpx; }
.mech-cell.on { color: var(--ok); }
.mech-head { border-bottom: 1rpx solid var(--border); }

/* G74 元素反应覆盖矩阵 */
.matrix { display: flex; flex-direction: column; gap: 4rpx; }
.matrix-row { display: flex; flex-direction: row; gap: 4rpx; }
.matrix-corner { width: 96rpx; flex-shrink: 0; }
.matrix-head {
  width: 96rpx;
  flex: 1;
  text-align: center;
  font-size: 22rpx;
  font-weight: 600;
  padding: 8rpx 0;
}
.matrix-cell {
  flex: 1;
  min-height: 56rpx;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 18rpx;
  text-align: center;
  border-radius: 8rpx;
  color: var(--muted);
  opacity: 0.7;
}
.matrix-cell.hit {
  color: #fff;
  opacity: 1;
  font-weight: 600;
}
.matrix-cell.self { opacity: 0.35; }

/* 图鉴样式已下沉到 styles/common.css（跨 6 个图鉴页复用） */
/* R66 可达反应脉冲 */
.reachable-pulse { animation: reachable-pulse 2s ease-in-out infinite; }
@keyframes reachable-pulse {
  0%, 100% { opacity: 0.75; }
  50% { opacity: 1; }
}
/* 触发元素对：两个元素 tag + 加号的小胶囊组 */
.pair-chip {
  display: inline-flex;
  align-items: center;
  gap: 8rpx;
  padding: 2rpx 10rpx;
  border-radius: 999rpx;
  background: rgba(127, 127, 127, 0.12);
}
/* R19：玩家反应伤害贡献卡（沿用既有 token，零新增视觉令牌） */
.r19-my-reaction-impact { margin: 24rpx 0; padding: 24rpx; background: var(--panel-2); border: 1px solid var(--border-soft); border-radius: 18rpx; }
.r19-rdi-head { display: flex; flex-direction: row; justify-content: space-between; align-items: center; }
.r19-rdi-title { font-size: 30rpx; font-weight: 700; color: var(--text); }
.r19-rdi-window { font-size: 20rpx; color: var(--text-2); }
.r19-view-toggle { display: inline-flex; gap: 8rpx; margin: 18rpx 0 12rpx; }
.r19-toggle-btn { padding: 8rpx 22rpx; border-radius: 999rpx; font-size: 22rpx; color: var(--text-2); background: rgba(127,127,127,.12); }
.r19-toggle-btn.active { color: var(--text); background: var(--accent); }
.r19-rdi-list { display: flex; flex-direction: column; gap: 8rpx; }
.r19-rdi-row { display: flex; flex-direction: row; justify-content: space-between; gap: 12rpx; padding: 12rpx 16rpx; border-radius: 10rpx; background: rgba(127,127,127,.08); }
.r19-rdi-name { font-size: 24rpx; color: var(--text); }
.r19-rdi-metric { font-size: 22rpx; color: var(--text-2); text-align: right; }
.r19-rdi-empty { padding: 24rpx 0; font-size: 24rpx; color: var(--text-2); text-align: center; }
.r19-rdi-meta { margin-top: 12rpx; font-size: 20rpx; color: var(--warn); }
.r19-ai-note { margin-top: 18rpx; padding: 16rpx; border-left: 4rpx solid var(--accent); background: rgba(88,166,255,.08); border-radius: 0 10rpx 10rpx 0; display: flex; flex-direction: column; gap: 8rpx; }
.r19-ai-title { font-size: 24rpx; font-weight: 700; color: var(--text); }
.r19-ai-conclusion { font-size: 22rpx; color: var(--text); }
.r19-ai-basis { font-size: 19rpx; color: var(--text-2); }
.r19-ai-cta { margin-top: 4rpx; font-size: 22rpx; color: var(--accent); text-decoration: underline; }
.r19-rdi-honesty { margin-top: 10rpx; font-size: 19rpx; color: var(--text-2); }
</style>
