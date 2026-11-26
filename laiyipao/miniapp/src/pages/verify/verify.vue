<template>
  <view class="page-root">
    <view class="pad safe-bottom">
      <view class="card">
        <text class="card-title">战绩验真</text>
        <text class="muted" style="display: block">
          战斗是完全确定性的：同样的<b>关卡 + 随机种子 + 构筑</b> → 同样的事件序列 →
          同样的回放哈希。任何人都能用 battle_id 取回这三样东西并本地重放，
          <b>证伪</b>一个可疑分数。
        </text>
        <text class="muted" style="display: block; margin-top: 10rpx">
          注意第三样：<b>构筑</b>。缺了构筑就算不出正确哈希 ——
          所以服务端在结算时把当时的构筑冻结进了 build_snapshot。
        </text>
      </view>

      <!-- R5：轻量复核行，不插入步骤条，复用已有本地重放按钮 -->
      <view class="section-kicker r5-section r5-enter">
        <text class="section-kicker-cn">战斗验真</text>
        <text class="section-kicker-en">REPLAY VERIFY · RULE ENGINE</text>
      </view>
      <view class="r5-ai-note r5-enter" role="note">
        <text class="r3-ai-badge">AI</text>
        <view class="r5-note-body">
          <text class="r5-note-title">AI 复核提示</text>
          <text class="r5-note-text">{{ r5AiVerify.conclusion }}</text>
          <text class="r5-note-basis">依据：复现信息 · 本次验真 · 更新于本次进入</text>
          <text class="r5-note-cta" role="button" @click="r5FocusReplay" tabindex="0" @keydown.enter="r5FocusReplay">去本地重放 →</text>
          <text v-if="result" class="r5-note-cta" role="button" @click="r5GoBattles" tabindex="0" @keydown.enter="r5GoBattles">去战报对照 →</text>
        </view>
      </view>

      <!-- R21 流程步骤条：把「填 ID → 取信息 → 出裁定」三段进度显性化，
           与全端页签/step chip 的视觉语言一致。 -->
      <view class="steps">
        <view class="step" :class="{ on: !!battleId }">
          <text class="step-no">1</text>
          <text class="step-label">填 ID</text>
        </view>
        <view class="step" :class="{ on: !!replayInfo }">
          <text class="step-no">2</text>
          <text class="step-label">取信息</text>
        </view>
        <view class="step" :class="{ on: !!result }">
          <text class="step-no">3</text>
          <text class="step-label">出裁定</text>
        </view>
      </view>

      <view class="card">
        <text class="card-title">输入 battle_id</text>
        <view class="row" style="gap: 12rpx">
          <input
            v-model="battleId"
            v-aria-native="'battle_id'"
            aria-label="battle_id"
            class="inp"
            :class="{ 'input-error': !!errMsg }"
            type="number"
            placeholder="例如 1"
            placeholder-class="ph"
          />
          <!-- 第六批 R38：此前 fetchReplay 是 async 却无任何在途反馈，
               与同页 doReplay 的 spinner 不一致；现在显示「获取中…」并禁用重复点。 -->
          <view
            class="btn"
            :class="{ 'btn-disabled': fetching }"
            style="width: 180rpx; height: 72rpx"
            @click="fetchReplay"
          >
            {{ fetching ? '获取中…' : '取复现信息' }}
          </view>
        </view>
        <text v-if="errMsg" :key="errMsg" class="muted shake" style="display: block; margin-top: 12rpx; color: var(--bad)" role="alert">
          {{ errMsg }}
        </text>
      </view>

      <view v-if="replayInfo" class="card">
        <text class="card-title">复现信息</text>
        <view class="kv"><text class="muted">battle_id</text><text class="mono">{{ replayInfo.battle_id }}</text></view>
        <view class="kv"><text class="muted">关卡</text><text class="mono">第 {{ replayInfo.level_id }} 关</text></view>
        <view class="kv"><text class="muted">波次</text><text class="mono">{{ replayInfo.level.wave_count }}</text></view>
        <view class="kv"><text class="muted">随机种子</text><text class="mono">{{ replayInfo.seed }}</text></view>
        <view class="kv">
          <text class="muted">构筑技能</text>
          <text class="mono">{{ buildText }}</text>
        </view>
        <view class="kv">
          <text class="muted">攻方攻击力</text>
          <text class="mono">{{ attackerText }}</text>
        </view>
        <!-- G99 冻结构筑分区快照：验真的前提是看清当时用的是哪套构筑 -->
        <view class="build-snapshot">
          <text v-for="s in buildSnapshotCard" :key="s.label" class="bs-item">{{ s.label }} {{ s.value }}</text>
        </view>
        <!-- G139 攻方全属性快照 -->
        <view v-if="attackerFullRows.length" class="attacker-full">
          <text v-for="r in attackerFullRows" :key="r.label" class="af-item">{{ r.label }} {{ r.value }}</text>
        </view>
        <view class="kv"><text class="muted">记录时间</text><text class="mono">{{ fmtVerifyTime(replayInfo.created_at) }}</text></view>
        <view class="kv">
          <text class="muted">服务端记录的哈希</text>
          <text class="mono hash">{{ pretty(replayInfo.expected_hash) }}</text>
        </view>
      </view>

      <view v-if="replayInfo" class="card">
        <text class="card-title">本地重放</text>
        <text class="muted" style="display: block; margin-bottom: 16rpx">
          用上面的关卡、种子、构筑在本地重跑完整局（约 1 万 tick，毫秒级完成），
          算出哈希后提交比对。
        </text>

        <view v-if="replaying" class="row" style="gap: 12rpx; align-items: center">
          <view class="spinner" />
          <text class="muted">重放中…</text>
        </view>

        <view v-else-if="outcome?.error" class="clamp-note">
          无法重放：{{ outcome.error }}
        </view>

        <!-- R33 裁定出现时弹入：让「验真有结论」成为一个看得见的事件 -->
        <view v-else-if="outcome" class="verdict-box pop-in" :class="outcome.matched ? 'ok' : 'bad'">
          <view class="kv">
            <text class="muted">本地重放哈希</text>
            <text class="mono hash">{{ pretty(outcome.computedHash) }}</text>
          </view>
          <!-- G117 哈希逐段对照 -->
          <view v-if="hashSegments.length" class="hash-segments">
            <text v-for="s in hashSegments" :key="s.i" class="hs-seg" :class="s.match ? 'ok' : 'bad'">{{ s.part }}</text>
          </view>
          <view class="verdict">{{ outcome.matched ? '一致 — 分数可复现' : '不一致 — 该分数无法被复现' }}</view>
          <view class="stats">
            <text>击杀 {{ outcome.stats.kills }}</text>
            <text>漏怪 {{ outcome.stats.leaked }}</text>
            <text>波次 {{ outcome.stats.waves }}</text>
            <text>用时 {{ duration(outcome.stats.durationMs) }}</text>
          </view>
        </view>

        <view id="r5-replay-btn" class="btn btn-primary" style="margin-top: 16rpx" @click="doReplay">
          本地重放并提交验真
        </view>

        <!-- 第六批 R64：提交在途给即时反馈，不必等裁定卡出现 -->
        <view
          class="btn"
          :class="{ 'btn-disabled': falsifying }"
          style="margin-top: 12rpx"
          @click="doFalsify"
        >
          {{ falsifying ? '提交中…' : '提交伪造哈希（演示证伪）' }}
        </view>
        <text class="dim" style="display: block; margin-top: 12rpx">
          「伪造哈希」提交全零值，必然不一致 —— 这正是「任何人可证伪」的含义：
          分数正确性不靠信任，而靠可复现。
        </text>
      </view>

      <view v-if="replayInfo" class="card r17-reaction-diff" role="region">
  <text class="card-title">AI · 逐次反应序列对照</text>
  <block v-if="r17Diff.available">
    <view class="r17-diff-summary">{{ r17Diff.summary }}</view>
    <view v-if="r17Diff.truncated" class="r17-truncated">已截断，仅显示前 200 条</view>
    <view v-if="r17Diff.firstDiff !== null" class="r17-first-diff">首处差异：第 {{ r17Diff.firstDiff + 1 }} 条</view>
    <view v-for="row in r17Diff.rows" :key="row.index" class="r17-diff-row" :data-r17-index="row.index">
      <text class="r17-row-idx">第 {{ row.index + 1 }} 条</text>
      <text class="r17-row-line">本地：{{ row.localText }}</text>
      <text class="r17-row-line">服务端：{{ row.serverText }}</text>
    </view>
  </block>
  <view v-else class="r17-diff-empty">该战报无逐次反应记录，暂无法对照（旧版本结算）</view>
  <text class="r17-diff-basis">依据：本地重放 reaction_timeline.events 与服务端 reaction_timeline.events 逐条比对，辅助对照线索</text>
  <text class="r17-diff-note">辅助对照线索，非验真判据；权威裁定仍以 replay_hash 为准</text>
  <view class="btn r17-diff-cta" role="button" @click="r17GoAuthority" tabindex="0" @keydown.enter="r17GoAuthority">查看权威裁定 ↑</view>
</view>
<view v-if="result" id="r17-authority-card" class="card pop-in" :style="{ borderColor: result.matched ? 'var(--ok)' : 'var(--bad)' }">
        <text class="card-title">服务端裁定</text>
        <view class="kv">
          <text class="muted">你提交的哈希</text>
          <text class="mono hash">{{ pretty(result.actual_hash) }}</text>
        </view>
        <view class="kv">
          <text class="muted">服务端记录</text>
          <text class="mono hash">{{ pretty(result.expected_hash) }}</text>
        </view>
        <text class="verdict" :class="result.matched ? 'ok' : 'bad'">
          {{ result.matched ? '验真通过' : '验真失败 — 分数被判定为不可复现' }}
        </text>
        <text class="muted" style="display: block; margin-top: 12rpx">
          验真记录已落库，可在运营后台「战斗记录与验真」页查看一致率统计。
        </text>
      </view>

      <!-- G125 本次会话验真记录 -->
      <view v-if="verifyHistory.length" class="card">
        <text class="card-title">本次验真记录</text>
        <view
          v-for="h in verifyHistory"
          :key="h.id + h.at"
          class="kv vh-row tap"
          @click="refillBattleId(h.id)"
        >
          <text class="mono">#{{ h.id }}</text>
          <text :class="h.matched ? 'vh-ok' : 'vh-bad'">{{ h.matched ? '一致' : '不一致' }}</text>
          <text class="dim">{{ h.at }}</text>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { onLoad, onUnload } from '@dcloudio/uni-app'
import { ref, computed, onMounted } from 'vue'
import { useGameStore } from '@/store/game'
import * as api from '@/api/client'
// import 时改别名：函数名 replay 与页面状态概念同名会遮蔽模板解析
import {
  replay as runReplay,
  prettyHash,
  prettyDuration,
  type ReplayOutcome,
  type ReplayInfo,
} from '@/game/replay'
import { scoreRulesFromServer } from '@/game/score'
import { REACTIONS } from '@/game/elements'

const store = useGameStore()
/**
 * 延迟回调的句柄（2026-10-21 P6-7）。
 *
 * 此前那个 `setTimeout(async () => { ...写请求... })` **从不清除**：
 * 页面已经卸载了，回调照样跑、照样发 POST ——
 * 症状是「离开了这个页面，它还在提交」，
 * 而验真是一个**会改数据**的端点。
 */
let pendingTimer: ReturnType<typeof setTimeout> | null = null
/** 页面是否已卸载。卸载后不再执行任何延迟回调。 */
let pageUnloaded = false

/** 延迟执行；若页面已卸载则**不执行**（P6-7）。 */
function schedulePending(fn: () => void, ms: number): void {
  if (pendingTimer) clearTimeout(pendingTimer)
  pendingTimer = setTimeout(() => {
    pendingTimer = null
    if (pageUnloaded) return
    fn()
  }, ms)
}

const battleId = ref('')

// 2026-10-31（实质优化第 35 轮）：从「我的战报」进入时自动填充 battle_id。
// 2026-11-01（新一轮第 47 轮）：battle_id 合法时**自动拉取复现信息** ——
// 战报页「去验真此局」跳转过来后无需再点一次按钮。
onLoad((query) => {
  const bid = query?.battle_id
  if (bid) {
    battleId.value = String(bid)
    const id = Number(bid)
    if (Number.isInteger(id) && id > 0) {
      void fetchReplay()
    }
  }
})
const replayInfo = ref<ReplayInfo | null>(null)
const outcome = ref<ReplayOutcome | null>(null)
const result = ref<any>(null)

/** G125 本次会话验真记录：保留每次验真的结论，可点击回填 battle_id。 */
const verifyHistory = ref<Array<{ id: number; matched: boolean; at: string }>>([])
/** R5：验真复核提示，只读取本次复现信息与验真结论，不伪造思考过程。 */
const r5AiVerify = computed(() => {
  const info = replayInfo.value
  const missing: string[] = []
  if (!info) missing.push('关卡/种子/构筑')
  else {
    if (!info.level_id) missing.push('关卡')
    if (!info.seed) missing.push('种子')
    if (!buildSnapshotCard.value.length) missing.push('构筑')
  }
  let conclusion: string
  if (!info) conclusion = '复现信息尚未到位：先填 battle_id 取信息，再本地重放比对哈希。'
  else if (missing.length) conclusion = `复现信息缺项：${missing.join('、')}；补齐后才能作为证伪依据。`
  else if (result.value) conclusion = `本次验真${result.value.matched ? '一致' : '不一致'}；哈希一致才可作为证伪依据，可去战报对照。`
  else conclusion = '关卡/种子/构筑齐备；本地重放哈希与服务端哈希一致才可作为证伪依据。'
  return { conclusion, missing, hasInfo: !!info, historyCount: verifyHistory.value.length }
})
function r5FocusReplay(): void {
  uni.pageScrollTo({ selector: '#r5-replay-btn', duration: 240 })
}
function r5GoBattles(): void {
  uni.navigateTo({ url: '/pages/battles/battles' })
}

function refillBattleId(id: number) {
  battleId.value = String(id)
}
const errMsg = ref('')
const replaying = ref(false)

const buildText = computed(() => {
  const skills = Object.values((replayInfo.value?.build?.skills ?? {}) as Record<string, any>)
  const on = skills.filter((s) => s && s.slot >= 0)
  if (on.length === 0) return '无（全部未装备）'
  return on
    .slice()
    .sort((a, b) => a.slot - b.slot)
    .map((s) => `#${s.slot} ${s.name}(${s.element})`)
    .join(' · ')
})

const attackerText = computed(() => {
  const a = replayInfo.value?.build?.attacker
  if (!a || typeof a.attack !== 'number') return '缺失 — 无法重放'
  return `攻击 ${a.attack}‰ · 暴击 ${a.crit_permille}‰ · 元素系数 ${a.element_coef_permille}‰`
})

/** G139 冻结快照攻方全属性：验真前提是看清当时构筑，展示完整 9 项（缺字段标注老战报）。 */
const attackerFullRows = computed(() => {
  const a = replayInfo.value?.build?.attacker
  if (!a) return []
  const pm = (v: number | undefined) => v === undefined ? '—' : `${Number(v) / 10}%`
  return [
    { label: '攻击力', value: `${a.attack ?? 0}‰` },
    { label: '暴击率', value: pm(a.crit_permille) },
    { label: '暴击倍率', value: pm(a.crit_multiplier_permille) },
    { label: '反应倍率', value: pm(a.reaction_mult_permille) },
    { label: '元素系数', value: pm(a.element_coef_permille) },
    { label: '层数上限', value: `${a.element_cap ?? 0}` },
    { label: '热量上限', value: pm(a.heat_cap_permille) },
    { label: '护甲', value: pm(a.armor_permille) },
    { label: '机制强度', value: pm(a.mechanic_permille) },
  ]
})

/** G99 冻结构筑分区快照：把验真战报里冻结的各维度数量化展示（验真前提是看清当时的构筑）。 */
const buildSnapshotCard = computed(() => {
  const b = replayInfo.value?.build
  if (!b) return []
  const skills = Object.values((b.skills ?? {}) as Record<string, any>)
  const elements = b.elements ?? b.skill_ids ?? []
  const equip = Array.isArray(b.equipment) ? b.equipment : Object.keys((b.equipment ?? {}) as object)
  const mastery = Array.isArray(b.mastery_nodes) ? b.mastery_nodes : Object.keys((b.mastery_nodes ?? {}) as object)
  return [
    { label: '出战技能', value: `${skills.filter((s) => s && s.slot >= 0).length} 个` },
    { label: '元素覆盖', value: `${elements.length} 种` },
    { label: '主动槽位', value: `${b.active_slots ?? 4} 个` },
    { label: '装备', value: `${equip.length} 件` },
    { label: '专精节点', value: `${mastery.length} 个` },
  ]
})

/** G117 哈希逐段对照：本地重放哈希 vs 服务端哈希，切 4 段标出哪段不同。 */
const hashSegments = computed(() => {
  const a = outcome.value?.computedHash
  const b = replayInfo.value?.expected_hash ?? result.value?.expected_hash
  if (!a || !b) return []
  const seg = (s: string) => (s.match(/.{1,4}/g) ?? []).slice(0, 4)
  const sa = seg(String(a))
  const sb = seg(String(b))
  return sa.map((part, i) => ({ i, part, match: part === sb[i] }))
})

// R17：逐次反应序列对照（辅助线索，不参与/不覆盖 replay_hash，也不冒充验真通过/失败）。
const R17_TL_SCHEMA = 'reaction-timeline-v1'
const r17Fields = ['wave', 'tick', 'reaction', 'enemy_uid', 'damage'] as const
type R17Event = { wave: number; tick: number; reaction: string; enemy_uid: number; damage: number }
function r17Valid(e: unknown): e is R17Event {
  if (!e || typeof e !== 'object') return false
  const o = e as Record<string, unknown>
  return Number.isInteger(o.wave) && (o.wave as number) >= 1
    && Number.isInteger(o.tick) && (o.tick as number) >= 0
    && typeof o.reaction === 'string' && o.reaction.length > 0
    && Number.isInteger(o.enemy_uid)
    && typeof o.damage === 'number' && Number.isFinite(o.damage)
}
function r17Sanitize(list: unknown): R17Event[] | null {
  if (!Array.isArray(list)) return null
  return list.filter(r17Valid).map((e) => ({
    wave: e.wave, tick: e.tick, reaction: e.reaction,
    enemy_uid: e.enemy_uid, damage: e.damage,
  }))
}
function r17Same(a: R17Event, b: R17Event): boolean {
  return r17Fields.every((k) => a[k] === b[k])
}
function r17Line(e: R17Event): string {
  const name = (REACTIONS as Record<string, { name?: string }>)[e.reaction]?.name ?? e.reaction
  return `第 ${e.wave} 波 · ${e.tick} tick · ${name} · #${e.enemy_uid} · ${e.damage.toFixed(1)}伤害`
}
const r17Diff = computed(() => {
  const local = r17Sanitize(outcome.value?.reactionTimeline)
  const svc = replayInfo.value?.reaction_timeline
  const server = svc && svc.schema_version === R17_TL_SCHEMA ? r17Sanitize(svc.events) : null
  const truncated = outcome.value?.reactionTimelineTruncated === true || svc?.truncated === true
  if (!local || !server) {
    return { available: false, summary: '', rows: [] as Array<{ index: number; localText: string; serverText: string }>, firstDiff: null as number | null, truncated }
  }
  const min = Math.min(local.length, server.length)
  const max = Math.max(local.length, server.length)
  const diffIndexes: number[] = []
  let sameCount = 0
  for (let i = 0; i < min; i++) {
    if (r17Same(local[i], server[i])) sameCount++
    else diffIndexes.push(i)
  }
  for (let i = min; i < max; i++) diffIndexes.push(i)
  return {
    available: true,
    summary: `一致 ${sameCount} 条 · 差异 ${diffIndexes.length} 条`,
    rows: diffIndexes.map((i) => ({
      index: i,
      localText: i < local.length ? r17Line(local[i]) : '（无此条）',
      serverText: i < server.length ? r17Line(server[i]) : '（无此条）',
    })),
    firstDiff: diffIndexes.length ? diffIndexes[0] : null,
    truncated,
  }
})
function r17GoAuthority(): void {
  uni.pageScrollTo({ selector: '#r17-authority-card', duration: 240 })
}

function pretty(h: string): string {
  return prettyHash(h)
}

/** 验真页记录时间本地化（R71）：完整日期 + 时分，非法 ISO 回退原串。 */
function fmtVerifyTime(iso: string): string {
  const d = new Date(iso)
  if (Number.isNaN(d.getTime())) return iso
  const y = d.getFullYear()
  const mm = String(d.getMonth() + 1).padStart(2, '0')
  const dd = String(d.getDate()).padStart(2, '0')
  const hh = String(d.getHours()).padStart(2, '0')
  const mi = String(d.getMinutes()).padStart(2, '0')
  return `${y}-${mm}-${dd} ${hh}:${mi}`
}

function duration(ms: number): string {
  return prettyDuration(ms)
}

/** 第六批 R38：取复现信息的在途标记。 */
const fetching = ref(false)

async function fetchReplay() {
  if (fetching.value) return
  errMsg.value = ''
  replayInfo.value = null
  outcome.value = null
  result.value = null
  const id = Number(battleId.value)
  if (!id || id < 1) {
    errMsg.value = '请输入有效的 battle_id'
    return
  }
  fetching.value = true
  try {
    replayInfo.value = await api.getReplay(id)
  } catch (e) {
    errMsg.value = (e as Error).message
    // ⚠️ 2026-10-21（二轮审计 P2-11）：失败时**清掉上一次的结论** ——
    // 否则页面同时显示「一致 —— 分数可复现」和一行错误，
    // 玩家会以为验真通过了。
    outcome.value = null
  } finally {
    fetching.value = false
  }
}

async function submit(hash: string) {
  const id = Number(battleId.value)
  try {
    result.value = await api.verifyReplay(id, hash)
    // G125 记录本次结论（与本地裁定一致即可，不入哈希）
    const matched = result.value?.matched ?? outcome.value?.matched ?? false
    verifyHistory.value = [
      { id, matched, at: new Date().toLocaleTimeString() },
      ...verifyHistory.value.filter((h) => h.id !== id),
    ].slice(0, 8)
  } catch (e) {
    errMsg.value = (e as Error).message
  }
}

function doReplay() {
  // R285：replaying 同时覆盖 30ms 延迟窗口与 submit() 的 await verifyReplay 在途期；
  // 原仅做按钮文案标记无 return 守卫，在途期再点会启动新 timer 并发第二次验真提交。
  if (replaying.value) return
  const ri = replayInfo.value
  if (!ri) {
    errMsg.value = '请先取复现信息'
    return
  }
  replaying.value = true
  errMsg.value = ''
  // 先让 loading 态渲染出来，再做同步重放（约 1 万 tick，毫秒级）
  schedulePending(async () => {
    try {
      const r = runReplay(ri, {
        level: ri.level,
        enemies: store.enemyMap,
        skills: store.skillMap,
        // 第 133 轮：重放用服务端下发的 score_rules，与原局开局同口径，
        // 否则 I-6 会把「两端分数规则不同」误判成伪造。
        scoreRules: store.config?.score_rules
          ? scoreRulesFromServer(store.config.score_rules)
          : undefined,
      })
      outcome.value = r
      if (!r.error) await submit(r.computedHash)
    } catch (e) {
      errMsg.value = `重放异常：${(e as Error).message}`
    } finally {
      replaying.value = false
    }
  }, 30)
}

/** 第六批 R64：伪造哈希提交在途标记。 */
const falsifying = ref(false)

async function doFalsify() {
  if (falsifying.value) return
  if (!replayInfo.value) {
    errMsg.value = '请先取复现信息'
    return
  }
  outcome.value = null
  falsifying.value = true
  try {
    await submit('0000000000000000')
  } finally {
    falsifying.value = false
  }
}

onMounted(async () => {
  if (!store.loggedIn) await store.login()
})

/**
 * 页面卸载时取消待执行的延迟回调（P6-7）。
 *
 * 否则「离开这个页面之后它还在提交」——
 * 对验真这种**会改数据**的端点，那是实打实的副作用。
 */
onUnload(() => {
  pageUnloaded = true
  if (pendingTimer) {
    clearTimeout(pendingTimer)
    pendingTimer = null
  }
})
</script>

<style scoped>
/* G139 攻方全属性快照 */
.attacker-full { display: flex; flex-wrap: wrap; gap: 8rpx; margin: 8rpx 0; }
.af-item { font-size: 20rpx; color: var(--text-2); background: var(--panel-2); border-radius: 6rpx; padding: 4rpx 10rpx; }

/* G125 会话验真记录 */
.vh-row { cursor: pointer; }
.vh-ok { color: var(--ok); }
.vh-bad { color: var(--bad); }

/* G117 哈希逐段对照 */
.hash-segments { display: flex; flex-wrap: wrap; gap: 8rpx; margin: 8rpx 0; }
.hs-seg { font-family: monospace; font-size: 22rpx; padding: 4rpx 8rpx; border-radius: 6rpx; }
.hs-seg.ok { background: rgba(126,231,135,0.14); color: var(--ok); }
.hs-seg.bad { background: rgba(255,123,114,0.14); color: var(--bad); }

/* G99 冻结构筑分区快照 */
.build-snapshot { display: flex; flex-wrap: wrap; gap: 8rpx; margin: 8rpx 0; }
.bs-item {
  background: var(--panel-2);
  border: 1rpx solid var(--border-soft);
  border-radius: 8rpx;
  padding: 4rpx 12rpx;
  font-size: 22rpx;
  color: var(--text-2);
}

/* R83 校验失败输入框红框 */
.input-error {
  border-color: var(--bad);
}

/* R21 流程步骤条 */
.steps {
  display: flex;
  flex-direction: row;
  align-items: center;
  margin-bottom: 16rpx;
}

.step {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 8rpx;
  font-size: 22rpx;
  color: var(--dim);
}

.step:not(:last-child)::after {
  content: '';
  flex: 1;
  height: 1rpx;
  background: var(--border);
  margin: 0 12rpx;
}

.step-no {
  width: 40rpx;
  height: 40rpx;
  border-radius: 50%;
  border: 1rpx solid var(--border);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 22rpx;
  flex-shrink: 0;
}

.step.on {
  color: var(--fire);
}

.step.on .step-no {
  background: var(--fire-soft);
  border-color: var(--fire);
  color: var(--fire);
  font-weight: 700;
}

.verdict-box {
  border-radius: 10rpx;
  padding: 16rpx;
  background: rgba(0, 0, 0, 0.25);
}

.verdict-box.ok {
  border: 1rpx solid var(--ok);
}

.verdict-box.bad {
  border: 1rpx solid var(--bad);
}

.verdict {
  display: block;
  font-size: 28rpx;
  font-weight: 600;
  margin-top: 12rpx;
  /* R95 裁定文字加字距，结论更醒目 */
  letter-spacing: 1rpx;
}

.verdict.ok {
  color: var(--ok);
}

.verdict.bad {
  color: var(--bad);
}

.stats {
  display: flex;
  flex-direction: row;
  gap: 24rpx;
  margin-top: 12rpx;
  font-size: 22rpx;
  color: var(--muted);
  flex-wrap: wrap;
}

.clamp-note {
  font-size: 22rpx;
  color: var(--warn);
  background: rgba(255, 211, 61, 0.1);
  padding: 14rpx 16rpx;
  border-radius: 8rpx;
}
.r17-reaction-diff { border-left: 6rpx solid var(--fire); }
.r17-diff-summary { font-size: 26rpx; font-weight: 700; color: var(--text); margin: 10rpx 0; }
.r17-truncated { font-size: 22rpx; color: var(--warn); margin-bottom: 8rpx; }
.r17-first-diff { font-size: 22rpx; color: var(--fire); margin-bottom: 8rpx; }
.r17-diff-row { display: flex; flex-direction: column; gap: 6rpx; background: var(--panel-2); border: 1rpx solid var(--border-soft); border-radius: 8rpx; padding: 12rpx; margin-bottom: 8rpx; }
.r17-row-idx { font-size: 20rpx; color: var(--text-2); }
.r17-row-line { font-size: 22rpx; color: var(--text-2); word-break: break-all; }
.r17-diff-empty { font-size: 24rpx; color: var(--text-2); background: var(--panel-2); padding: 16rpx; border-radius: 8rpx; margin: 10rpx 0; }
.r17-diff-basis { display: block; font-size: 20rpx; color: var(--text-2); margin-top: 12rpx; }
.r17-diff-note { display: block; font-size: 20rpx; color: var(--warn); margin-top: 8rpx; }
.r17-diff-cta { margin-top: 14rpx; }
</style>
