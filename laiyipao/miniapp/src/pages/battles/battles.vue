<template>
  <view class="page-root">
    <view class="pad safe-bottom">
      <view v-if="!loading && items.length" class="r3-kpi r4-kpi r4-enter">
        <view class="r3-kpi-main">
          <text class="r3-kpi-num">{{ r4AiBattle ? r4AiBattle.leakPct + '%' : '—' }}</text>
          <text class="r3-kpi-label">漏杀比 · {{ r4AiBattle ? r4AiBattle.grade : '待计算' }}</text>
          <text class="r3-kpi-sub">{{ battleTotals.count }} 场 · 均分 {{ battleTotals.avgScore }}</text>
        </view>
        <view class="r3-kpi-cell"><text class="r3-kpi-num">{{ battleTotals.kills }}</text><text class="r3-kpi-label">总击杀</text></view>
        <view class="r3-kpi-cell"><text class="r3-kpi-num">{{ battleTotals.leaked }}</text><text class="r3-kpi-label">总漏怪</text></view>
      </view>
      <view v-if="!loading && r4AiBattle" class="r3-ai r4-ai r4-enter" aria-label="AI 防守诊断">
        <view class="r3-ai-head">
          <text class="r3-ai-badge">AI</text>
          <text class="r3-ai-title">AI 防守诊断</text>
          <text class="r3-ai-engine">规则引擎 · 基于已加载战报</text>
        </view>
        <text class="r3-ai-conclusion">{{ r4AiBattle.conclusion }}</text>
        <text class="r3-ai-basis">{{ r4AiBattle.basis }}</text>
        <view class="r3-ai-cta" @click="r4GoBattle()">看战报详情 →</view>
      </view>
      <text class="muted" style="display: block; margin-bottom: 16rpx">
        最近战斗的记录。点开一行可以回顾单局掉落与元素/反应/地形使用。
      </text>

      <!-- 结果筛选（2026-11-01 新一轮第 74 轮） -->
      <view class="filter-row r3-filter r4-filter">
        <view
          v-for="f in filters"
          :key="f.key"
          class="filter-chip r3-chip"
          :class="{ active: filter === f.key }"
          @click="filter = f.key"
        >
          {{ f.label }}
        </view>
        <!-- R78 筛选结果计数：筛选后还剩几场一眼可见 -->
        <!-- 第六批 R60：分页未拉完时「共 N 场」是错的，改为「已加载 N 场」 -->
        <text class="filter-count">{{ hasMore ? '已加载' : '共' }} {{ items.length }} 场</text>
      </view>

      <!-- G96 战绩总账：击杀/漏怪/分数累计与均分 -->
      <view v-if="!loading && items.length" class="card total-card">
        <view class="card-title">战绩总账</view>
        <view class="total-grid">
          <view class="total-item"><text class="mono total-val">{{ battleTotals.count }}</text><text class="dim">场数</text></view>
          <view class="total-item"><text class="mono total-val">{{ battleTotals.kills }}</text><text class="dim">总击杀</text></view>
          <view class="total-item"><text class="mono total-val">{{ battleTotals.leaked }}</text><text class="dim">总漏怪</text></view>
          <view class="total-item"><text class="mono total-val">{{ battleTotals.avgScore }}</text><text class="dim">均分</text></view>
        </view>
        <!-- G140 连胜/连败 -->
        <text v-if="streak.count > 0" class="streak-note">
          当前{{ streak.type === 'win' ? '连胜' : '连败' }} {{ streak.count }} 场 · 最长连胜 {{ streak.maxWin }}
        </text>
      </view>

      <!-- G70 战绩趋势折线：分数走势 + 胜负色点，趋势一眼可见 -->
      <view v-if="trend.count >= 2" class="card trend-card">
        <view class="card-title">
          <text>战绩趋势</text>
          <text class="dim">近 {{ trend.count }} 局 · 均分 {{ trend.avg }}</text>
        </view>
        <svg class="trend-svg" viewBox="0 0 300 96" aria-label="战绩分数趋势">
          <polyline class="trend-line" :points="trend.line" />
          <polyline v-if="leakRateTrend.pts.length >= 2" class="leak-line" :points="leakRateTrend.line" />
          <circle
            v-for="(p, i) in trend.pts"
            :key="'pt' + i"
            :cx="p.x" :cy="p.y" r="3"
            :class="p.win ? 'trend-win' : 'trend-lose'"
          />
        </svg>
        <view class="trend-legend">
          <text class="dim">━ 分数（蓝·越高越好）　┄ 漏怪率（红虚线·越低越好）　● 绿胜 ● 红负</text>
        </view>
      </view>

      <!-- G83 关卡胜率分布：每关打赢比例，找出还卡着的关 -->
      <view v-if="!loading && winRateByLevel.length" class="card winrate-card">
        <view class="card-title">关卡胜率</view>
        <view v-for="w in winRateByLevel" :key="w.level" class="wr-row">
          <text class="dim wr-label">第 {{ w.level }} 关</text>
          <view class="wr-track">
            <view class="wr-fill" :style="{ width: w.rate + '%' }" />
          </view>
          <text class="mono wr-val">{{ w.rate }}%</text>
        </view>
      </view>

      <!-- G88 星级分布：0-3 星各占比，看通关质量 -->
      <view v-if="!loading && items.length" class="card stardist-card">
        <view class="card-title">星级分布</view>
        <view v-for="s in starDistribution" :key="s.stars" class="sd-row">
          <text class="dim sd-label">{{ s.stars }} 星</text>
          <view class="sd-track">
            <view class="sd-fill" :style="{ width: s.pct + '%' }" />
          </view>
          <text class="mono sd-val">{{ s.n }}</text>
        </view>
      </view>

      <!-- G118 关卡交战热度 -->
      <view v-if="!loading && battleHeat.length" class="card heat-card">
        <view class="card-title">交战热度<text class="dim">反复挑战的关卡</text></view>
        <view v-for="h in battleHeat" :key="h.level" class="heat-row">
          <text class="dim heat-label">第 {{ h.level }} 关</text>
          <view class="heat-track"><view class="heat-fill" :style="{ width: h.pct + '%' }" /></view>
          <text class="mono heat-val">{{ h.n }}</text>
        </view>
      </view>

      <!-- G132 命中率效率卡 -->
      <view v-if="efficiencyStats" class="card eff-card">
        <view class="card-title">战斗效率<text class="dim">已展开 {{ efficiencyStats.count }} 局</text></view>
        <text class="dim">命中率 {{ efficiencyStats.accuracy }}% · 平均最高热量 {{ efficiencyStats.avgHeat }}</text>
      </view>
      <view v-else-if="!loading && items.length" class="card">
        <text class="dim">点开任意战报后，这里会统计命中率与热量效率。</text>
      </view>

      <!-- R20 服务端全量效率趋势卡（G132 本地即时卡保留，两者口径不同） -->
      <view v-if="r20Efficiency" class="r20-efficiency-trend" role="region">
        <view class="r20-eff-head">
          <text class="r20-eff-title">战斗效率趋势 · 全量战报</text>
          <text class="r20-eff-window">窗口 {{ r20Efficiency.window_from }} ~ {{ r20Efficiency.window_to }}（近 {{ r20Efficiency.window_days }} 天）</text>
        </view>
        <template v-if="r20Efficiency.battles > 0">
          <view class="r20-efficiency-row">
            <text>命中率 {{ r20Accuracy }}%</text>
            <text>平均峰值热量 {{ r20Heat }}</text>
            <text>场均漏怪 {{ r20Leak }}</text>
            <text>场均时长 {{ r20Duration }}ms</text>
          </view>
          <view class="r20-trend-list">
            <view
              v-for="b in r20Efficiency.buckets"
              :key="b.bucket"
              class="r20-trend-bar"
              :data-r20-bucket="b.bucket"
            >
              <text class="r20-trend-bucket">{{ b.bucket }}</text>
              <view class="r20-trend-track"><view class="r20-trend-fill" :style="{ width: b.accuracy_pct + '%' }" /></view>
              <text class="r20-trend-value">{{ b.accuracy_pct.toFixed(1) }}% · {{ b.battles }} 场</text>
            </view>
          </view>
          <view v-if="r20Ai" class="r20-ai-note">
            <text class="r20-ai-title">AI · 战斗效率诊断</text>
            <text class="r20-ai-conclusion">{{ r20Ai.conclusion }}</text>
            <text class="r20-ai-basis">依据：数据来自全量battle_records近30天；近 30 天 battle_records 的 shots/hits/heat_max 聚合；场次以 battle_records 行数为准</text>
            <text class="r20-ai-cta" data-r20-cta role="button" @click="goStage()" tabindex="0" @keydown.enter="goStage()">去战斗→</text>
          </view>
          <text class="r20-honest-note">辅助诊断非权威结算依据，权威请看 replay_hash</text>
        </template>
        <view v-else class="r20-efficiency-empty" role="status" aria-live="polite">
          <text>近 30 天无战斗记录</text>
          <text class="r20-eff-window">窗口 {{ r20Efficiency.window_from }} ~ {{ r20Efficiency.window_to }}（近 {{ r20Efficiency.window_days }} 天）</text>
        </view>
      </view>
      <view v-else-if="r20Error" class="r20-efficiency-empty" role="status" aria-live="polite">
        <text>效率数据加载失败，稍后重试</text>
      </view>

      <view v-if="loading" class="card">
        <text class="muted">加载中…</text>
        <view v-for="n in 3" :key="n" class="skeleton-row">
          <view class="skeleton-bar" style="width: 44rpx; height: 44rpx; border-radius: 8rpx" />
          <view class="skeleton-bar" style="width: 50%" />
          <view class="skeleton-bar" style="width: 14%; margin-left: auto" />
        </view>
      </view>

      <view v-else-if="items.length === 0" class="r3-empty r4-empty empty-state" aria-live="polite" role="status">
        <text class="r3-empty-slot empty-glyph">⚔</text>
        <text class="r3-empty-title">还没有战报</text>
        <text class="r3-empty-reason">打一局战斗后，胜负、星级、分数和击杀都会出现在这里。</text>
        <view class="r3-empty-action" role="button" @click="r4GoBattle()" tabindex="0" @keydown.enter="r4GoBattle()">去关卡页挑一关开打 →</view>
      </view>

      <view v-else class="card r3-data-grid r4-grid" style="padding: 8rpx 24rpx">
        <view
          v-for="it in items"
          :key="it.id"
          class="battle-row tap"
          :class="[it.result === 'win' ? 'win' : 'lose', { expanded: expandedId === it.id }]"
          @click="toggleDetail(it)"
          role="button"
          tabindex="0"
          @keydown.enter="toggleDetail(it)"
          :aria-expanded="String(expandedId === it.id)"
        >
          <view class="battle-head">
            <!-- R19 胜负做成实心徽章，整行再给淡色底，扫一眼就能分胜败 -->
            <text class="result-badge" :class="it.result === 'win' ? 'win' : 'lose'">
              {{ it.result === 'win' ? '胜' : '负' }}
            </text>
            <text class="battle-stars" :class="{ dim: it.stars === 0 }">
              {{ '★'.repeat(it.stars) || '☆☆☆' }}
            </text>
            <text class="flex-1 ellipsis">第 {{ it.level_id }} 关</text>
            <text class="mono battle-score">{{ it.score }}</text>
          </view>
          <view class="battle-meta">
            <text class="dim">击杀 {{ it.kills }} · 漏怪 {{ it.leaked }}</text>
            <text class="dim">{{ fmtTime(it.created_at) }}</text>
          </view>

          <!-- 详情（2026-11-01 新一轮第 41 轮）：展开时按需拉取单局完整回顾 -->
          <view v-if="expandedId === it.id" class="battle-detail">
            <view v-if="detailLoadingIds.has(it.id)" class="dim" style="padding: 8rpx 0">加载中…</view>
            <template v-else-if="detail">
              <view class="kv">
                <text class="muted">发射 / 命中</text>
                <text class="mono">{{ detail.shots }} / {{ detail.hits }}</text>
              </view>
              <view class="kv"><text class="muted">波次</text><text class="mono">{{ detail.wave_reached }}</text></view>
              <view v-if="detailDefeatReview" class="defeat-review" role="status" aria-live="polite">
                <text class="defeat-review-label">AI · 失败复盘</text>
                <text class="defeat-review-text">{{ detailDefeatReview }}</text>
              </view>
              <view class="kv"><text class="muted">峰值热量</text><text class="mono">{{ detail.heat_max }}</text></view>
              <view class="kv"><text class="muted">剩余血量</text><text class="mono">{{ detail.hp_left }}</text></view>
              <view class="kv">
                <text class="muted">掉落</text>
                <text class="mono" style="color: var(--ok)">{{ lootText(detail.loot) || '—' }}</text>
              </view>
              <view v-if="elementText" class="kv"><text class="muted">元素</text><text class="flex-1">{{ elementText }}</text></view>
              <!-- G78 元素使用频次构成条（不是伤害量，服务端无按元素伤害拆分） -->
              <view v-if="elementMix.length" class="mix-bar">
                <view v-for="(m, i) in elementMix" :key="i" class="mix-seg" :style="{ width: m.pct + '%' }" :title="m.name" />
              </view>
              <view v-if="reactionText" class="kv"><text class="muted">反应</text><text class="flex-1">{{ reactionText }}</text></view>
              <view v-if="reactionLearning" class="reaction-learning" role="status" aria-live="polite">
                <text class="reaction-learning-label">AI · 反应搭配建议</text>
                <text class="reaction-learning-text">{{ reactionLearning }}</text>
              </view>
              <!-- R15：逐次反应归因（服务端展示性字段，不是结算输入）；旧战报显示诚实空态 -->
              <view class="r15-reaction-timeline" role="status" aria-live="polite">
                <text class="r15-timeline-label">AI · 反应时间线</text>
                <template v-if="reactionTimeline.length">
                  <view
                    v-for="(ev, i) in reactionTimeline"
                    :key="i"
                    class="r15-timeline-row"
                    :data-r15-wave="ev.wave"
                  >
                    <text class="r15-timeline-text">第 {{ ev.wave }} 波 · {{ ev.reactionName }} · #{{ ev.enemy_uid }} · {{ fmt(ev.damage) }} 伤害</text>
                  </view>
                  <text v-if="reactionTimelineTruncated" class="r15-truncated-note">已截断，仅显示前 200 条</text>
                </template>
                <text v-else class="r15-timeline-empty">该战报无逐次反应记录（旧版本结算）</text>
              </view>
              <!-- R25：单局反应伤害主力聚合（纯前端派生，count 取权威 reactions_used，events 不冒充次数） -->
              <view class="r25-reaction-impact" role="status" aria-live="polite">
                <text class="r25-impact-label">AI · 反应伤害主力</text>
                <view v-if="reactionImpact.length" class="r25-impact-rows">
                  <view
                    v-for="row in reactionImpact"
                    :key="row.reaction"
                    class="r25-impact-row"
                    :data-r25-reaction="row.reaction"
                  >
                    <text class="r25-impact-name">{{ row.reactionName }}</text>
                    <text class="r25-impact-damage">{{ fmt(row.total_damage) }} 伤害</text>
                    <text class="r25-impact-share">{{ row.damage_share_pct.toFixed(1) }}%</text>
                    <text class="r25-impact-count">权威{{ row.count }}次</text>
                    <text class="r25-impact-events">事件{{ row.events }}条</text>
                  </view>
                  <text v-if="reactionTimelineTruncated" class="r25-truncated-note">已截断，仅统计前 200 条</text>
                </view>
                <text v-else class="r25-impact-empty">本局无逐次反应伤害记录</text>
                <view class="r25-ai-note">
                  <text class="r25-ai-conclusion">{{ r25Conclusion }}</text>
                  <text class="r25-ai-basis">依据：本局 reaction_timeline.events 的 damage 聚合；触发次数以 reactions_used 为准</text>
                  <text class="r25-ai-cta">下一局优先凑齐该元素组合 →</text>
                  <text class="r25-ai-honesty">辅助展示非伤害权威,权威以 replay_hash 为准</text>
                </view>
              </view>
              <!-- R14 §C.1：建议下方给服务端权威 CTA；E2E 选择器 #r14-advice-action-btn.r14-advice-cta[data-r14-source=battle]，空态 .r14-advice-empty -->
              <view v-if="r14Advice" class="r14-advice r14-enter">
                <text class="r14-advice-badge">{{ r14Advice.badge }} · 可执行建议</text>
                <block v-if="r14Advice.status === 'ready'">
                  <text class="r14-advice-text">依据 {{ r14Advice.basedOn[0] }}：一键换上「{{ r14Advice.candidateName }}」</text>
                  <text v-if="r14Advice.impact" class="r14-advice-text">预计每波伤害 {{ fmt(r14Advice.impact.baseline.estimatedDamagePerWave) }} → {{ fmt(r14Advice.impact.candidate.estimatedDamagePerWave) }}（服务端收益预估）</text>
                  <text class="r14-advice-basis">依据：{{ r14Advice.basedOn.join(' · ') }} · 服务端五条件裁决（不伪造 id）</text>
                  <text id="r14-advice-action-btn" class="r14-advice-cta" role="button" data-r14-source="battle" @click="r14OpenCompare()" tabindex="0" @keydown.enter="r14OpenCompare()">一键替换为 {{ r14Advice.candidateName }} →</text>
                  <text v-if="r14ApplyDone" class="apply-done r14-advice-done" role="status" aria-live="polite">已替换为「{{ r14ApplyDone }}」，出战配置已更新。</text>
                </block>
                <view v-else class="r14-advice-empty" data-r14-source="battle" aria-live="polite" role="status">
                  <text class="r14-advice-empty-title">{{ r14Advice.empty?.headline }}</text>
                  <text class="r14-advice-empty-msg">{{ r14Advice.empty?.message }}</text>
                  <text class="r14-advice-empty-cta" role="button" @click="r14GoEmpty()" tabindex="0" @keydown.enter="r14GoEmpty()">{{ r14Advice.empty?.cta }}</text>
                </view>
              </view>
              <view v-if="terrainText" class="kv"><text class="muted">地形</text><text class="flex-1">{{ terrainText }}</text></view>
              <view class="kv">
                <text class="muted">回放哈希</text>
                <text class="mono hash">{{ detail.replay_hash }}</text>
              </view>
              <view class="r10-growth r10-enter">
                <text class="r10-growth-title">AI · 养成前后（同一关可信差值）</text>
                <template v-if="growthCompare.available">
                  <text class="r10-growth-text">{{ growthCompare.text }}</text>
                  <view class="r10-growth-grid">
                    <view v-for="row in growthCompare.rows" :key="row.label" class="r10-growth-row">
                      <text class="r10-growth-label">{{ row.label }}</text>
                      <text class="r10-growth-num">{{ fmt(row.before) }} → {{ fmt(row.after) }}</text>
                      <text class="r10-growth-delta" :class="{ negative: row.delta < 0 }">{{ fmtSigned(row.delta) }}</text>
                    </view>
                  </view>
                  <text class="r10-hashes">hash：{{ growthHashes }}</text>
                  <text class="r10-growth-basis">依据：{{ growthCompare.basis }}</text>
                </template>
                <text v-else class="r10-growth-empty" aria-live="polite" role="status">还缺养成前那局战报，先打一局同关 · 构筑变更点未上报，不做估算。</text>
              </view>
              <!-- 复制哈希（2026-11-01 新一轮第 87 轮）：用外部工具验真时无需手选 -->
              <view class="btn copy-hash-btn" @click="copyHash(detail.replay_hash)">复制回放哈希</view>
              <!-- 直达验真（2026-11-01 新一轮第 47 轮）：验真页已支持 battle_id 深链，
                   这里一键跳转，玩家无需手动复制/输入 battle_id。 -->
              <view class="btn verify-btn" @click="goVerify(it.id)">去验真此局</view>
            </template>
          </view>
        </view>
      </view>

      <!-- 加载更早战报（2026-11-01 新一轮第 55 轮） -->
      <view v-if="hasMore" class="load-more" @click="loadMore">
        {{ loadingMore ? '加载中…' : '加载更多' }}
      </view>
    <!-- R14 §C.3：复用 R7 两步确认浮层；首次点击只预览，确认才写入 -->
    <view v-if="r14CompareOpen" class="r7-compare-mask" @click="r14CompareOpen = false">
      <view class="r7-compare r7-enter" @click.stop>
        <text class="r7-compare-title">当前 vs 推荐 · 确认替换</text>
        <view class="r7-vs-row">
          <text class="r7-vs-label">{{ r14Advice?.replacedName }}（当前）</text>
          <text class="r7-vs-arrow">→</text>
          <text class="r7-vs-label">{{ r14Advice?.candidateName }}（推荐）</text>
        </view>
        <view v-if="r14Advice?.impact" class="r7-vs-row">
          <text class="r7-vs-label">每波伤害 {{ fmt(r14Advice.impact.baseline.estimatedDamagePerWave) }} → {{ fmt(r14Advice.impact.candidate.estimatedDamagePerWave) }}</text>
        </view>
        <text v-if="r14Advice" class="r7-basis">依据：{{ r14Advice.basedOn.join(' · ') }}</text>
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
  </view>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { onLoad } from '@dcloudio/uni-app'
import { useGameStore } from '@/store/game'
import * as api from '@/api/client'
import type { BattleHistoryItem, BattleDetail, BattleEfficiency, BattleEfficiencyBucket } from '@/api/client'
import { normalizeGrowthCompare, type GrowthCompareView } from '@/pages/_shared/r10GrowthCompare'
import { ELEMENT_NAME } from '@/game/elements'
import { TERRAIN_NAME } from '@/game/terrain'
import { defeatReview, reactionLearningAdvice } from '@/game/feedback'
import { fetchAdviceAction, applyAdviceAction, type R14AdviceAction } from '@/pages/_shared/r14AdviceAction'

const store = useGameStore()

const items = ref<BattleHistoryItem[]>([])
const loading = ref(false)
const loadingMore = ref(false)
/** 是否还有更早的战报（上次返回满一页 20 条即视为还有更多）。 */
const hasMore = ref(false)
/** 服务端提供的 keyset 游标（上一页末行 id）；0 表示没有更多页。 */
const nextCursor = ref(0)
/**
 * 列表请求序号（第 52 轮）：每次 reload / loadMore 各占一个序号。
 * 切筛选触发 reload 后，仍在飞的旧 loadMore 响应序号已过期，必须丢弃，
 * 否则旧游标的页会 append 进新筛选的列表（服务端 result 是全量过滤的）。
 */
let listReqSeq = 0
/** R64：辅助面板（growthCompare / r14Advice）绑定当前展开战报的序号，旧响应直接丢弃。 */
let detailPanelSeq = 0
const expandedId = ref(0)

/**
 * 结果筛选（2026-11-01 新一轮第 74 轮）：全部 / 胜 / 负。
 * 筛选作用于已加载的列表（最近 N 局内过滤）。
 */
const filters = [
  { key: 'all', label: '全部' },
  { key: 'win', label: '胜利' },
  { key: 'lose', label: '失败' },
] as const
type BattleFilter = (typeof filters)[number]['key']
const filter = ref<BattleFilter>('all')

/* -------------------------------------------------------------------------- */
/* G70 战绩趋势折线（第七批）                                                   */
/* 已加载战报按时间正序（列表最新在前，这里反转）画分数折线 + 胜负色点。         */
/* 数据来自 items[].score/result，纯客户端派生，零契约改动。                     */
/* -------------------------------------------------------------------------- */
const trend = computed(() => {
  const seq = [...items.value].reverse().slice(-20) // 最近 20 局，时间正序
  const n = seq.length
  const maxScore = Math.max(1, ...seq.map((it) => it.score))
  const W = 300
  const H = 90
  const pts = seq.map((it, i) => ({
    x: n <= 1 ? W / 2 : (i / (n - 1)) * W,
    y: H - (it.score / maxScore) * (H - 12) - 6,
    // 胜负只认 result（服务端 resultStr 只有 win/lose）；stars 由分数门槛
    // 独立算出、与胜负无关（lose+多星是合法数据），不能拿 stars>0 判胜。
    win: it.result === 'win',
    score: it.score,
  }))
  return {
    pts,
    line: pts.map((p) => `${p.x.toFixed(1)},${p.y.toFixed(1)}`).join(' '),
    count: n,
    maxScore,
    avg: n ? Math.round(seq.reduce((s, it) => s + it.score, 0) / n) : 0,
  }
})

/**
 * 结果筛选（2026-11-01 新一轮第 80 轮）：切换时**重新从第一页加载**，
 * 服务端按 result 过滤（配合分页，筛选对全部历史生效，而非仅已加载部分）。
 */
watch(filter, () => {
  void reload()
})

async function reload(): Promise<void> {
  const seq = ++listReqSeq
  // 第106轮：reload 重建整个列表（重新拉第一页），任何在飞的旧 loadMore 都随之作废。
  // 这里必须复位 loadingMore —— 否则旧 loadMore 被 seq 守卫丢弃时，它的 finally
  // 因 seq 已过期不会复位忙标志，按钮会永久停在「加载中…」且后续无法再翻页。
  loadingMore.value = false
  loading.value = true
  try {
    const res = await api.myBattleHistory(20, 0, filter.value === 'all' ? '' : filter.value)
    if (seq !== listReqSeq) return // 已有更新的 reload/loadMore，丢弃陈旧响应
    items.value = res.items ?? []
    // 服务端 MyBattleHistoryPage 先多取 1 行判断 hasMore：总数恰好满一页时
    // hasMore=false → next_cursor=0。以 next_cursor 为准，避免满页时按钮无效常驻。
    hasMore.value = (res.next_cursor ?? 0) > 0
    nextCursor.value = res.next_cursor ?? 0
    expandedId.value = 0
    // 深链（2026-11-01 新一轮第 43 轮）：结算页跳转过来时自动展开刚打的那一局
    if (focusBattleId.value > 0) {
      const target = items.value.find((it) => it.id === focusBattleId.value)
      if (target) {
        expandedId.value = target.id
        void loadDetail(target)
      }
    }
  } catch {
    // 战报加载失败：展示空态，不阻塞页面
  } finally {
    if (seq === listReqSeq) loading.value = false
  }
}
const details = ref<Record<number, BattleDetail>>({})
// 第 140 轮：按战报 id 归属「加载中」，避免并行拉取时先回的响应误清后者的提示
const detailLoadingIds = ref<Set<number>>(new Set())

/** 深链目标战报 id（2026-11-01 新一轮第 43 轮）：结算页跳转时携带。 */
const focusBattleId = ref(0)

onLoad((query) => {
  const id = Number(query?.battle_id ?? 0)
  if (Number.isInteger(id) && id > 0) focusBattleId.value = id
})

/** 当前展开行的详情（未拉取时为 undefined）。 */
const detail = computed<BattleDetail | undefined>(() =>
  expandedId.value ? details.value[expandedId.value] : undefined,
)

/** R12 失败复盘：战报详情用服务端权威字段给出根因与下一动作；胜利不误报。 */
const detailDefeatReview = computed(() => {
  const d = detail.value
  if (!d || d.result !== 'lose') return null
  return defeatReview({
    wave: Number(d.wave_reached ?? 0) || null,
    kills: Number(d.kills ?? 0),
    leaked: Number(d.leaked ?? 0),
    reactions: Number(d.reactions ?? 0),
    heatMax: Number(d.heat_max ?? 0) || null,
    elementsUsed: d.elements_used,
  })
})

/** R10 §1I.2：同关相邻两次真实结算的确定性差值；详情加载后按 level_id 从历史里配对。 */
const growthCompare = ref<GrowthCompareView>(normalizeGrowthCompare([]))
const growthHashes = computed(() => growthCompare.value.hashes.join(' / ') || '—')

async function loadGrowthCompare(d: BattleDetail, seq?: number) {
  if (seq !== undefined && seq !== detailPanelSeq) return
  let baseline: BattleDetail | null = null
  try {
    const history = await api.myBattleHistory(20)
    const sameLevel = (history.items ?? []).filter((x) => x.level_id === d.level_id && x.replay_hash && x.id !== d.id)
    const priorId = sameLevel[0]?.id
    if (priorId) baseline = await api.myBattleDetail(priorId)
  } catch {
    baseline = null
  }
  if (seq !== undefined && seq !== detailPanelSeq) return // 异步恢复后复核：过期响应不写 ref
  growthCompare.value = baseline ? normalizeGrowthCompare([baseline, d]) : normalizeGrowthCompare([])
}

const CURRENCY_NAME: Record<string, string> = {
  coin: '金币',
  gem: '钻石',
  energy: '体力',
  keys: '钥匙',
}

function currencyName(k: string): string {
  return CURRENCY_NAME[k] ?? k
}

function lootText(loot: Record<string, number> | undefined): string {
  if (!loot) return ''
  const entries = Object.entries(loot)
  if (!entries.length) return ''
  return entries.map(([k, v]) => `${v} ${currencyName(k)}`).join(' + ')
}

/** 反应名以服务端 /config.reactions 为准（与图鉴页同源）。 */
const reactionNameMap = computed(() => {
  const m = new Map<string, string>()
  for (const r of store.config?.reactions ?? []) m.set(r.key, r.name)
  return m
})

const elementText = computed(() => {
  const d = detail.value
  if (!d) return ''
  return Object.entries(d.elements_used)
    .map(([k, v]) => `${ELEMENT_NAME[k as keyof typeof ELEMENT_NAME] ?? k} ×${v}`)
    .join('、')
})

/* -------------------------------------------------------------------------- */
/* G140 连胜/连败：当前连续胜负状态 + 最长连胜，给连续作战的节奏反馈。             */
/* -------------------------------------------------------------------------- */
const streak = computed(() => {
  const seq = [...items.value].reverse() // 时间正序
  let cur = 0
  let curType = ''
  let maxWin = 0
  let tmp = 0
  for (const it of seq) {
    const win = it.result === 'win'
    if (curType === (win ? 'win' : 'lose')) cur++
    else { curType = win ? 'win' : 'lose'; cur = 1 }
    if (win) { tmp++; maxWin = Math.max(maxWin, tmp) } else tmp = 0
  }
  return { type: curType, count: seq.length ? cur : 0, maxWin }
})

/* -------------------------------------------------------------------------- */
/* G132 命中率效率卡：只聚合已展开过的局（details 缓存），不额外发请求。           */
/* -------------------------------------------------------------------------- */
const efficiencyStats = computed(() => {
  const ds = Object.values(details.value)
  if (!ds.length) return null
  const shots = ds.reduce((s, d) => s + (d.shots ?? 0), 0)
  const hits = ds.reduce((s, d) => s + (d.hits ?? 0), 0)
  return {
    count: ds.length,
    accuracy: shots > 0 ? Math.round(hits / shots * 100) : 0,
    avgHeat: Math.round(ds.reduce((s, d) => s + (d.heat_max ?? 0), 0) / ds.length),
  }
})

/* -------------------------------------------------------------------------- */
/* G118 关卡交战热度：按 level_id 聚合次数 Top8 —— 看出反复卡在哪关。             */
/* -------------------------------------------------------------------------- */
const battleHeat = computed(() => {
  const m = new Map<number, number>()
  for (const it of items.value) m.set(it.level_id, (m.get(it.level_id) ?? 0) + 1)
  const arr = [...m.entries()].map(([level, n]) => ({ level, n })).sort((a, b) => b.n - a.n).slice(0, 8)
  const max = Math.max(1, ...arr.map((a) => a.n))
  return arr.map((a) => ({ ...a, pct: Math.round(a.n / max * 100) }))
})

/* -------------------------------------------------------------------------- */
/* G102 漏怪率走势（第七批）                                                    */
/* 每局 leaked/(kills+leaked) 的防守质量百分比，序列折线 —— 看出是靠输出硬堆还是守得稳。 */
/* -------------------------------------------------------------------------- */
const leakRateTrend = computed(() => {
  const seq = [...items.value].reverse().slice(-20)
  const pts = seq.map((it, i) => {
    const k = it.kills ?? 0
    const l = it.leaked ?? 0
    const rate = k + l > 0 ? Math.round(l / (k + l) * 100) : 0
    return {
      x: seq.length <= 1 ? 150 : (i / (seq.length - 1)) * 300,
      y: 80 - rate * 0.7,
      rate,
    }
  })
  return { pts, line: pts.map((p) => `${p.x.toFixed(1)},${p.y.toFixed(1)}`).join(' ') }
})

/* -------------------------------------------------------------------------- */
/* G96 战绩总量统计（第七批）                                                   */
/* 已加载战报的击杀/漏怪/反应/分数总和与均值 —— 一个「我的战斗生涯」总账。        */
/* -------------------------------------------------------------------------- */
const battleTotals = computed(() => {
  const n = items.value.length || 1
  const sum = (k: string) => items.value.reduce((s, it) => s + (Number((it as any)[k]) || 0), 0)
  return {
    count: items.value.length,
    kills: sum('kills'),
    leaked: sum('leaked'),
    score: sum('score'),
    avgScore: Math.round(sum('score') / n),
  }
})

/* -------------------------------------------------------------------------- */
/* G88 星级分布（第七批）                                                       */
/* 已加载战报按 0/1/2/3 星聚合，看「我的通关质量」。数据来自 items[].stars。      */
/* -------------------------------------------------------------------------- */
const starDistribution = computed(() => {
  const counts = [0, 0, 0, 0]
  for (const it of items.value) {
    const s = Math.max(0, Math.min(3, it.stars ?? 0))
    counts[s]++
  }
  const total = counts.reduce((a, b) => a + b, 0) || 1
  return counts.map((n, stars) => ({ stars, n, pct: Math.round(n / total * 100) }))
})

/* -------------------------------------------------------------------------- */
/* G83 关卡胜率分布（第七批）                                                   */
/* 已加载战报按 level_id 聚合胜/负，展示每关打赢比例 —— 看「哪些关还卡着」。       */
/* 数据来自 items[].level_id/result，纯客户端派生。                               */
/* -------------------------------------------------------------------------- */
const winRateByLevel = computed(() => {
  const m = new Map<number, { level: number; win: number; lose: number }>()
  for (const it of items.value) {
    const win = it.result === 'win'
    const cur = m.get(it.level_id) ?? { level: it.level_id, win: 0, lose: 0 }
    if (win) cur.win++
    else cur.lose++
    m.set(it.level_id, cur)
  }
  return [...m.values()]
    .map((v) => ({ ...v, rate: Math.round(v.win / (v.win + v.lose) * 100) }))
    .sort((a, b) => a.level - b.level)
    .slice(-8) // 最近 8 个关卡，避免长榜
})

/* G78 元素使用构成（第七批）：注意 BattleDetail 只有「使用次数」，无按元素伤害数值，
   所以这里诚实标注是「使用频次构成」，绝不声称伤害量。 */
const elementMix = computed(() => {
  const d = detail.value
  if (!d) return []
  const entries = Object.entries(d.elements_used).filter(([, v]) => v > 0)
  const total = entries.reduce((s, [, v]) => s + v, 0) || 1
  return entries.map(([k, v]) => ({
    name: ELEMENT_NAME[k as keyof typeof ELEMENT_NAME] ?? k,
    pct: Math.round(v / total * 100),
  }))
})

const reactionText = computed(() => {
  const d = detail.value
  if (!d) return ''
  return Object.entries(d.reactions_used)
    .map(([k, v]) => `${reactionNameMap.value.get(k) ?? k} ×${v}`)
    .join('、')
})

/** R13：反应次数不是教学，必须把权威使用记录翻译成下一局补哪种元素。 */
const reactionLearning = computed(() => {
  const d = detail.value
  if (!d) return ''
  return reactionLearningAdvice({ reactionsUsed: d.reactions_used, elementsUsed: d.elements_used })
})

/**
 * R15：服务端可选逐次反应归因；旧战报无字段/结构异常时一律空态，
 * 不用聚合 reactions_used 反推波次与敌人（那是编造）。只做展示，不参与任何结算。
 */
const reactionTimeline = computed(() => {
  const raw = detail.value?.reaction_timeline
  const events = raw && Array.isArray(raw.events) ? raw.events : []
  const valid = events.filter((ev) =>
    ev && Number.isInteger(ev.wave) && ev.wave >= 1 &&
    Number.isInteger(ev.tick) && ev.tick >= 0 &&
    typeof ev.reaction === 'string' && ev.reaction.length > 0 &&
    Number.isInteger(ev.enemy_uid) &&
    typeof ev.damage === 'number' && Number.isFinite(ev.damage),
  )
  return valid
    .sort((a, b) => a.wave - b.wave || a.tick - b.tick)
    .map((ev) => ({
      reaction: ev.reaction,
      wave: ev.wave,
      enemy_uid: ev.enemy_uid,
      damage: ev.damage,
      reactionName: reactionNameMap.value.get(ev.reaction) ?? ev.reaction,
    }))
})

const reactionTimelineTruncated = computed(() => detail.value?.reaction_timeline?.truncated === true)

/** R25：单局反应伤害主力聚合。只读派生，count 取权威 reactions_used，绝不用 events 条数冒充。 */
const reactionImpact = computed(() => {
  const events = reactionTimeline.value
  if (!events.length) return []
  const groups = new Map<string, { total: number; events: number }>()
  for (const ev of events) {
    if (typeof ev.reaction !== 'string' || !ev.reaction || typeof ev.damage !== 'number' || !Number.isFinite(ev.damage)) continue
    const g = groups.get(ev.reaction) ?? { total: 0, events: 0 }
    g.total += ev.damage
    g.events += 1
    groups.set(ev.reaction, g)
  }
  if (!groups.size) return []
  let sum = 0
  for (const g of groups.values()) sum += g.total
  const used = detail.value?.reactions_used ?? {}
  const rows = [...groups.entries()].map(([reaction, g]) => ({
    reaction,
    reactionName: reactionNameMap.value.get(reaction) ?? reaction,
    total_damage: g.total,
    damage_share_pct: Number((sum > 0 ? (g.total / sum) * 100 : 0).toFixed(1)),
    events: g.events,
    count: typeof used[reaction] === 'number' ? used[reaction] : 0,
  }))
  rows.sort((a, b) => b.total_damage - a.total_damage || a.reaction.localeCompare(b.reaction))
  return rows
})
const r25Conclusion = computed(() => {
  const rows = reactionImpact.value
  if (!rows.length) return '本局无逐次反应伤害记录，不编造主力'
  const top = rows[0]!
  if (top.total_damage <= 0) return 'AI：本局反应伤害为 0，无可归因主力'
  const leaders = rows.filter(r => r.damage_share_pct === top.damage_share_pct)
  return 'AI：本局' + leaders.map(r => r.reactionName).join('、') + '造成 ' + top.damage_share_pct.toFixed(1) + '% 反应伤害，是伤害主力'
})

const terrainText = computed(() => {
  const d = detail.value
  if (!d) return ''
  return (d.terrain_used ?? [])
    .map((k) => TERRAIN_NAME[k as keyof typeof TERRAIN_NAME] ?? k)
    .join('、')
})

/** R11：战报差值数值统一格式化，避免长浮点尾数；不改差值本身。 */
function fmt(n: number | null | undefined): string {
  return typeof n === 'number' && Number.isFinite(n) ? n.toFixed(1) : '—'
}
function fmtSigned(n: number | null | undefined): string {
  return typeof n === 'number' && Number.isFinite(n) ? (n >= 0 ? '+' : '') + n.toFixed(1) : '—'
}

/* R14 §C.1：战报建议 → 服务端权威可执行 CTA；元素取自战报权威 elements_used。 */
const r14Advice = ref<R14AdviceAction | null>(null)
const r14CompareOpen = ref(false)
const r14Applying = ref(false)
const r14ApplyError = ref('')
const r14ApplyDone = ref('')

async function loadR14Advice(d: BattleDetail, seq?: number) {
  if (seq !== undefined && seq !== detailPanelSeq) return
  const entries = Object.entries(d.elements_used || {}).filter(([, v]) => v > 0).sort((a, b) => b[1] - a[1])
  if (!entries.length) { r14Advice.value = null; return }
  const reactions = Object.entries(d.reactions_used || {}).filter(([, v]) => v > 0).sort((a, b) => b[1] - a[1])
  const advice = await fetchAdviceAction({ element: entries[0]![0], reaction: reactions[0]?.[0], source: 'battle' })
  if (seq !== undefined && seq !== detailPanelSeq) return // 异步恢复后复核：过期建议不写 ref
  r14Advice.value = advice
}

function r14OpenCompare() {
  if (!r14Advice.value || r14Advice.value.status !== 'ready') return
  r14ApplyError.value = ''
  r14ApplyDone.value = ''
  r14CompareOpen.value = true
}

function r14GoEmpty() {
  const route = r14Advice.value?.empty?.route || '/pages/stage/stage'
  uni.navigateTo({ url: route.startsWith('/pages/') ? route : '/pages/stage/stage' })
}

async function r14ConfirmApply() {
  const a = r14Advice.value
  if (!a || a.status !== 'ready' || r14Applying.value) return
  r14Applying.value = true
  r14ApplyError.value = ''
  r14ApplyDone.value = ''
  try {
    const res = await applyAdviceAction(a, store)
    if (!res.ok) { r14ApplyError.value = res.error; return }
    r14ApplyDone.value = res.doneName
    if (detail.value) await loadR14Advice(detail.value)
  } catch (e) {
    r14ApplyError.value = '写入失败：' + ((e as Error)?.message || '未知错误') + '，原配置未改动。请重试'
  } finally {
    r14Applying.value = false
  }
}
function fmtTime(iso: string): string {
  const d = new Date(iso)
  if (Number.isNaN(d.getTime())) return iso
  const mm = String(d.getMonth() + 1).padStart(2, '0')
  const dd = String(d.getDate()).padStart(2, '0')
  const hh = String(d.getHours()).padStart(2, '0')
  const mi = String(d.getMinutes()).padStart(2, '0')
  return `${mm}/${dd} ${hh}:${mi}`
}

/**
 * 展开/收起战报详情。详情按需拉取并缓存：同一场只请求一次。
 */
async function toggleDetail(it: BattleHistoryItem) {
  if (expandedId.value === it.id) {
    expandedId.value = 0
    return
  }
  expandedId.value = it.id
  await loadDetail(it)
}

/** 拉取单局详情（已缓存则直接返回）。 */
async function loadDetail(it: BattleHistoryItem): Promise<void> {
  const seq = ++detailPanelSeq
  if (details.value[it.id]) return
  detailLoadingIds.value.add(it.id)
  try {
    const d = await api.myBattleDetail(it.id)
    details.value[it.id] = d
    await loadGrowthCompare(d, seq)
    void loadR14Advice(d, seq)
  } catch {
    // 详情加载失败：保留展开态，可再次点击重试
  } finally {
    // 只清当前战报的加载标记，不能触碰其它仍在飞的战报（第 140 轮竞态修复）
    detailLoadingIds.value.delete(it.id)
  }
}

/**
 * 直达验真（2026-11-01 新一轮第 47 轮）。
 * 验真页支持 `?battle_id=` 深链并自动拉取复现信息，
 * 玩家从战报详情一键跳转即可验真，无需手动复制哈希。
 */
function goVerify(battleId: number) {
  uni.navigateTo({ url: `/pages/verify/verify?battle_id=${battleId}` })
}

/** R4：把总账与连胜状态翻译成一条防守质量结论；无战报时返回 null。 */
const r4AiBattle = computed(() => {
  const t = battleTotals.value
  if (!t.count) return null
  const denom = t.kills + t.leaked
  const leakPct = denom > 0 ? Math.round(t.leaked / denom * 100) : 0
  const grade = leakPct >= 25 ? '漏怪偏高' : leakPct >= 10 ? '防守尚可' : '防守扎实'
  const streakText = streak.value.count
    ? `当前${streak.value.type === 'win' ? '连胜' : '连败'} ${streak.value.count} 场`
    : '暂无连胜连败'
  return {
    leakPct,
    grade,
    conclusion: `${grade}：${t.count} 场共漏 ${t.leaked}/${denom} 只（${leakPct}%），均分 ${t.avgScore}`,
    basis: `依据：战绩总账 ${t.kills} 杀 / ${t.leaked} 漏 · ${streakText}`,
  }
})

function r4GoBattle() {
  const first = items.value[0]
  if (first) {
    toggleDetail(first)
    uni.pageScrollTo({ selector: '.battle-row', duration: 200 })
    return
  }
  goStage()
}

/** 第六批 R36：空态引导接上真实跳转（此前是点不动的假链接）。 */
function goStage() {
  uni.navigateTo({ url: '/pages/stage/stage' })
}

/**
 * 复制回放哈希（2026-11-01 新一轮第 87 轮）。
 * 玩家用外部工具重放验真时，无需手动选中一长串字符。
 */
function copyHash(hash: string) {
  if (!hash) return
  uni.setClipboardData({
    data: hash,
    success: () => {
      uni.showToast({ title: '已复制回放哈希', icon: 'success' })
    },
    fail: () => { uni.showToast({ title: '复制失败，请手动选择复制', icon: 'none' }) },
  })
}

/**
 * 加载更早的战报（2026-11-01 新一轮第 55 轮）。
 * 服务端按时间倒序分页，响应给出 next_cursor（上一页末行 id）作 keyset 游标：
 * 翻页途中新插入的战报不会把旧行挤到下一页造成重复/漏行。
 */
async function loadMore() {
  if (loadingMore.value || !hasMore.value) return
  const seq = ++listReqSeq
  loadingMore.value = true
  try {
    const res = await api.myBattleHistory(20, 0, filter.value === 'all' ? '' : filter.value, nextCursor.value)
    if (seq !== listReqSeq) return // 筛选已切换/已有更新的 reload，丢弃陈旧响应
    const more = res.items ?? []
    const existing = new Set(items.value.map((it) => it.id))
    items.value = items.value.concat(more.filter((it) => !existing.has(it.id)))
    // 同上：末页恰好满 20 条时 next_cursor=0，不应继续显示加载更多。
    hasMore.value = (res.next_cursor ?? 0) > 0
    nextCursor.value = res.next_cursor ?? 0
  } catch {
    // 加载更多失败：保留按钮可重试
  } finally {
    if (seq === listReqSeq) loadingMore.value = false
  }
}

/* -------------------------------------------------------------------------- */
/* R20 全量效率趋势：服务端 battle_records 聚合，G132 本地即时卡继续保留。       */
/* -------------------------------------------------------------------------- */
const r20Efficiency = ref<BattleEfficiency | null>(null)
const r20Error = ref(false)

function r20Num(v: unknown): number {
  const n = typeof v === 'number' ? v : Number(v)
  return Number.isFinite(n) ? n : 0
}

async function loadR20Efficiency(): Promise<void> {
  try {
    // 老测试可能只 mock 了部分 api 导出；缺函数时诚实不渲染，绝不本地造数。
    if (typeof api.fetchBattleEfficiency !== 'function') return
    const raw = await api.fetchBattleEfficiency(30)
    if (!raw || typeof raw !== 'object') return
    const rawBuckets = Array.isArray(raw.buckets) ? raw.buckets : []
    const buckets: BattleEfficiencyBucket[] = rawBuckets
      .filter((b) => b && typeof b.bucket === 'string' && b.bucket.length > 0)
      .map((b) => ({
        bucket: b.bucket,
        battles: r20Num(b.battles),
        shots: r20Num(b.shots),
        hits: r20Num(b.hits),
        accuracy_pct: r20Num(b.accuracy_pct),
      }))
      .sort((a, b) => a.bucket.localeCompare(b.bucket))
    r20Efficiency.value = {
      window_days: 30,
      window_from: typeof raw.window_from === 'string' ? raw.window_from : '',
      window_to: typeof raw.window_to === 'string' ? raw.window_to : '',
      battles: r20Num(raw.battles),
      shots: r20Num(raw.shots),
      hits: r20Num(raw.hits),
      accuracy_pct: r20Num(raw.accuracy_pct),
      avg_heat_max: r20Num(raw.avg_heat_max),
      leaked_per_battle: r20Num(raw.leaked_per_battle),
      avg_duration_ms: r20Num(raw.avg_duration_ms),
      buckets,
      ai_insight: raw.ai_insight && typeof raw.ai_insight === 'object' ? raw.ai_insight : {},
    }
  } catch {
    r20Error.value = true
  }
}

const r20Accuracy = computed(() => r20Num(r20Efficiency.value?.accuracy_pct).toFixed(1))
const r20Heat = computed(() => r20Num(r20Efficiency.value?.avg_heat_max).toFixed(1))
const r20Leak = computed(() => r20Num(r20Efficiency.value?.leaked_per_battle).toFixed(1))
const r20Duration = computed(() => r20Num(r20Efficiency.value?.avg_duration_ms).toFixed(1))

/** 结论只由服务端真实字段与趋势桶推导；空数据不产出结论。 */
const r20Ai = computed(() => {
  const d = r20Efficiency.value
  if (!d || d.battles <= 0) return null
  const acc = r20Num(d.accuracy_pct)
  const heat = r20Num(d.avg_heat_max)
  const leak = r20Num(d.leaked_per_battle)
  const first = d.buckets.length ? d.buckets[0]!.accuracy_pct : acc
  const last = d.buckets.length ? d.buckets[d.buckets.length - 1]!.accuracy_pct : acc
  const trend = last > first ? '命中率呈上升趋势' : last < first ? '命中率呈下降趋势' : '命中率保持稳定'
  const advice = acc < 70 ? '建议优先提升命中与清漏效率' : '保持当前节奏，继续关注热量管理'
  return { conclusion: `近 30 天命中率 ${acc.toFixed(1)}%，场均漏怪 ${leak.toFixed(1)}，平均峰值热量 ${heat.toFixed(1)}；${trend}，${advice}。` }
})

onMounted(async () => {
  if (!store.loggedIn) await store.login()
  await reload()
  void loadR20Efficiency()
})
</script>

<style scoped>
/* G140 连胜连败 */
.streak-note { display: block; text-align: center; font-size: 22rpx; color: var(--warn); margin-top: 8rpx; }

/* G118 交战热度 */
.heat-row { display: flex; flex-direction: row; align-items: center; gap: 12rpx; margin: 8rpx 0; }
.heat-label { width: 140rpx; flex-shrink: 0; }
.heat-track { flex: 1; height: 12rpx; background: var(--panel-3); border-radius: 999rpx; overflow: hidden; }
.heat-fill { height: 100%; background: var(--accent); border-radius: 999rpx; }
.heat-val { width: 60rpx; text-align: right; font-size: 22rpx; color: var(--accent); }

/* G96 战绩总账 */
.total-grid { display: flex; flex-direction: row; }
.total-item { flex: 1; display: flex; flex-direction: column; align-items: center; gap: 4rpx; }
.total-val { font-size: 36rpx; font-weight: 700; color: var(--fire); }

/* G88 星级分布 */
.sd-row { display: flex; flex-direction: row; align-items: center; gap: 12rpx; margin: 8rpx 0; }
.sd-label { width: 80rpx; flex-shrink: 0; }
.sd-track { flex: 1; height: 12rpx; background: var(--panel-3); border-radius: 999rpx; overflow: hidden; }
.sd-fill { height: 100%; background: var(--warn); border-radius: 999rpx; }
.sd-val { width: 60rpx; text-align: right; font-size: 22rpx; color: var(--warn); }

/* G83 关卡胜率分布 */
.wr-row { display: flex; flex-direction: row; align-items: center; gap: 12rpx; margin: 8rpx 0; }
.wr-label { width: 140rpx; flex-shrink: 0; }
.wr-track { flex: 1; height: 12rpx; background: var(--panel-3); border-radius: 999rpx; overflow: hidden; }
.wr-fill { height: 100%; background: var(--ok); border-radius: 999rpx; }
.wr-val { width: 72rpx; text-align: right; font-size: 22rpx; color: var(--ok); }

/* G78 元素使用频次构成条 */
.mix-bar { display: flex; flex-direction: row; height: 12rpx; border-radius: 6rpx; overflow: hidden; margin: 6rpx 0 10rpx; }
.mix-seg { height: 100%; background: var(--ice); }
.mix-seg:nth-child(5n+2) { background: var(--fire); }
.mix-seg:nth-child(5n+3) { background: var(--lightning); }
.mix-seg:nth-child(5n+4) { background: var(--corrosion); }
.mix-seg:nth-child(5n+5) { background: var(--kinetic); }

/* G70 战绩趋势折线 */
.trend-card .card-title { margin-bottom: 8rpx; }
.trend-svg { width: 100%; height: 192rpx; overflow: visible; }
.trend-line {
  fill: none;
  stroke: var(--ice);
  stroke-width: 2;
  stroke-linejoin: round;
}
.leak-line { fill: none; stroke: var(--bad); stroke-width: 1.5; stroke-dasharray: 4 3; }
.trend-legend { text-align: center; font-size: 20rpx; }
.trend-win { fill: var(--ok); }
.trend-lose { fill: var(--bad); }

.battle-row {
  padding: 12rpx 0;
  border-bottom: 1rpx solid var(--border);
}

/* R19 胜负色差：整行淡底 + 左侧色条，不必逐字读「胜/负」 */
.battle-row.win {
  background: linear-gradient(90deg, rgba(126, 231, 135, 0.08), transparent 60%);
  box-shadow: inset 4rpx 0 0 rgba(126, 231, 135, 0.55);
}

.battle-row.lose {
  background: linear-gradient(90deg, rgba(255, 123, 114, 0.08), transparent 60%);
  box-shadow: inset 4rpx 0 0 rgba(255, 123, 114, 0.45);
}

/* R93 胜负徽章弹入，扫列表时胜负结论更醒目 */
.result-badge {
  animation: badge-pop 0.3s cubic-bezier(0.2, 0.9, 0.3, 1.3);
}

@keyframes badge-pop {
  from { transform: scale(0.7); opacity: 0; }
  to { transform: scale(1); opacity: 1; }
}

.result-badge {
  min-width: 44rpx;
  text-align: center;
  padding: 6rpx 0;
  border-radius: 8rpx;
  font-size: 24rpx;
  font-weight: 700;
}

.result-badge.win {
  color: var(--ok);
  background: rgba(126, 231, 135, 0.14);
}

.result-badge.lose {
  color: var(--bad);
  background: rgba(255, 123, 114, 0.14);
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
}

.empty-glyph {
  font-size: 64rpx;
  opacity: 0.6;
  margin-bottom: 8rpx;
}

.empty-hint {
  margin-top: 16rpx;
  font-size: 24rpx;
  color: var(--fire);
}

.battle-row:last-child {
  border-bottom: none;
}

.battle-row.expanded {
  background: rgba(88, 166, 255, 0.06);
  margin: 0 -12rpx;
  padding: 12rpx;
  border-radius: 12rpx;
}

.battle-head {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 12rpx;
  font-size: 26rpx;
}

.battle-stars {
  color: var(--warn);
  font-size: 24rpx;
}

.battle-score {
  font-size: 26rpx;
}

.battle-meta {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  margin-top: 4rpx;
  font-size: 24rpx;
}

.battle-detail {
  margin-top: 10rpx;
  padding-top: 8rpx;
  border-top: 1rpx dashed var(--border);
}

.defeat-review {
  margin: 10rpx 0;
  padding: 12rpx 14rpx;
  background: rgba(255, 123, 114, 0.08);
  border: 1rpx solid rgba(255, 123, 114, 0.3);
  border-radius: 10rpx;
}

.defeat-review-label {
  display: block;
  font-size: 22rpx;
  color: var(--bad);
  font-weight: 700;
}

.defeat-review-text {
  display: block;
  margin-top: 6rpx;
  font-size: 21rpx;
  line-height: 1.55;
  color: var(--text);
}

.reaction-learning {
  margin: 10rpx 0;
  padding: 12rpx 14rpx;
  background: rgba(88, 166, 255, 0.08);
  border: 1rpx solid rgba(88, 166, 255, 0.28);
  border-radius: 10rpx;
}

.reaction-learning-label {
  display: block;
  font-size: 22rpx;
  color: var(--accent);
  font-weight: 700;
}

.reaction-learning-text {
  display: block;
  margin-top: 6rpx;
  font-size: 21rpx;
  line-height: 1.55;
  color: var(--text);
}

/* R15 · 逐次反应时间线：沿用 AI 卡令牌，不新增视觉变量 */
.r15-reaction-timeline {
  margin: 10rpx 0;
  padding: 12rpx 14rpx;
  background: rgba(88, 166, 255, 0.08);
  border: 1rpx solid rgba(88, 166, 255, 0.28);
  border-radius: 10rpx;
}

.r15-timeline-label {
  display: block;
  font-size: 22rpx;
  color: var(--accent);
  font-weight: 700;
}

.r15-timeline-row {
  display: block;
  margin-top: 6rpx;
  padding: 6rpx 8rpx;
  border-left: 3rpx solid var(--accent);
  background: rgba(88, 166, 255, 0.05);
  border-radius: 6rpx;
}

.r15-timeline-text {
  display: block;
  font-size: 21rpx;
  line-height: 1.55;
  color: var(--text);
}

.r15-timeline-empty,
.r15-truncated-note {
  display: block;
  margin-top: 6rpx;
  font-size: 21rpx;
  line-height: 1.55;
  color: var(--text);
}

.r15-truncated-note {
  color: var(--bad);
}



/* R20 服务端全量效率趋势卡：沿用现有令牌，不新增视觉变量 */
.r20-efficiency-trend { margin: 16rpx 0; padding: 16rpx; background: var(--panel); border: 1rpx solid var(--border); border-radius: 14rpx; }
.r20-eff-head { display: flex; flex-direction: column; gap: 4rpx; }
.r20-eff-title { font-size: 26rpx; font-weight: 700; color: var(--fire); }
.r20-eff-window { font-size: 20rpx; color: var(--dim); }
.r20-efficiency-row { display: flex; flex-wrap: wrap; gap: 8rpx 16rpx; margin: 12rpx 0; font-size: 22rpx; color: var(--text); }
.r20-trend-list { display: flex; flex-direction: column; gap: 8rpx; }
.r20-trend-bar { display: flex; flex-direction: row; align-items: center; gap: 10rpx; font-size: 20rpx; }
.r20-trend-bucket { width: 150rpx; color: var(--dim); }
.r20-trend-track { flex: 1; height: 10rpx; background: var(--panel-3); border-radius: 999rpx; overflow: hidden; }
.r20-trend-fill { height: 100%; background: var(--ice); border-radius: 999rpx; }
.r20-trend-value { width: 170rpx; text-align: right; color: var(--text); }
.r20-ai-note { margin-top: 14rpx; padding: 12rpx 14rpx; background: rgba(88, 166, 255, 0.08); border: 1rpx solid rgba(88, 166, 255, 0.28); border-radius: 10rpx; display: flex; flex-direction: column; gap: 6rpx; }
.r20-ai-title { font-size: 22rpx; font-weight: 700; color: var(--accent); }
.r20-ai-conclusion, .r20-ai-basis { font-size: 21rpx; line-height: 1.5; color: var(--text); }
.r20-ai-cta { font-size: 22rpx; color: var(--fire); }
.r20-honest-note { display: block; margin-top: 10rpx; font-size: 20rpx; color: var(--dim); }
.r20-efficiency-empty { display: flex; flex-direction: column; gap: 6rpx; margin: 16rpx 0; padding: 16rpx; font-size: 22rpx; color: var(--dim); background: var(--panel); border: 1rpx dashed var(--border); border-radius: 14rpx; }

/* 复制回放哈希（2026-11-01 新一轮第 87 轮） */
.copy-hash-btn {
  margin-top: 10rpx;
  padding: 12rpx 0;
  text-align: center;
  font-size: 24rpx;
  color: var(--fire);
  background: rgba(255, 107, 53, 0.08);
  border: 1rpx solid rgba(255, 107, 53, 0.3);
  border-radius: 10rpx;
}

/* 加载更多（2026-11-01 新一轮第 55 轮） */
.load-more {
  margin-top: 16rpx;
  padding: 16rpx 0;
  text-align: center;
  font-size: 26rpx;
  color: var(--fire);
  background: var(--panel);
  border: 1rpx solid var(--border);
  border-radius: 12rpx;
}

/* 结果筛选（2026-11-01 新一轮第 74 轮） */
.filter-row {
  display: flex;
  flex-direction: row;
  gap: 12rpx;
  margin-bottom: 16rpx;
}

/* R78 筛选结果计数 */
.filter-count {
  margin-left: auto;
  align-self: center;
  font-size: 22rpx;
  color: var(--dim);
}

.filter-chip {
  padding: 10rpx 28rpx;
  border-radius: 999rpx;
  background: var(--panel);
  border: 1rpx solid var(--border);
  font-size: 24rpx;
  color: var(--muted);
}

.filter-chip.active {
  background: rgba(255, 107, 53, 0.16);
  border-color: var(--fire);
  color: var(--fire);
}

/* R25 单局反应伤害主力聚合：沿用既有 token，零新增视觉令牌 */
.r25-reaction-impact {
  margin-top: 12rpx;
  padding: 10rpx 12rpx;
  background: var(--panel);
  border: 1rpx solid var(--border);
  border-radius: 12rpx;
}
.r25-impact-label { color: var(--accent); font-size: 22rpx; font-weight: 700; }
.r25-impact-rows { display: flex; flex-direction: column; gap: 6rpx; margin-top: 8rpx; }
.r25-impact-row { display: flex; flex-wrap: wrap; gap: 10rpx; font-size: 21rpx; }
.r25-impact-name { color: var(--text); font-weight: 700; }
.r25-impact-damage, .r25-impact-events { color: var(--muted); }
.r25-impact-share { color: var(--fire); font-weight: 700; }
.r25-impact-count { color: var(--accent); }
.r25-truncated-note, .r25-impact-empty { color: var(--muted); font-size: 20rpx; }
.r25-ai-note { display: flex; flex-direction: column; gap: 4rpx; margin-top: 10rpx; padding-top: 8rpx; border-top: 1rpx solid var(--border); }
.r25-ai-conclusion { color: var(--text); font-size: 21rpx; }
.r25-ai-basis, .r25-ai-honesty { color: var(--muted); font-size: 19rpx; }
.r25-ai-cta { color: var(--accent); font-size: 20rpx; font-weight: 700; }
</style>