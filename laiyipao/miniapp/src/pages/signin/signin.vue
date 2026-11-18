<template>
  <view class="page-root">
    <view class="pad safe-bottom">
      <!-- 加载中 / 空态（2026-10-21 P6-1）-->
      <view v-if="loading" class="card">
        <text class="muted">加载中…</text>
      </view>
      <view v-else-if="tasks.length === 0" class="card">
        <text class="muted">暂无任务</text>
      </view>
      <view class="card">
        <text class="card-title">每日签到</text>
        <text class="muted">{{ cycleHint }}</text>
        <!-- R40 连续进度条：已签天数一眼可见，不必数格子 -->
        <view class="streak-track">
          <view class="streak-fill" :style="{ width: streakPct + '%' }" />
        </view>
        <view class="days">
          <view
            v-for="d in cycleDays"
            :key="d"
            class="day"
            :class="{
              claimed: d <= claimedCount,
              today: d === claimedCount + 1,
              future: d > claimedCount + 1,
              'claim-pop': justSigned && d === claimedCount,
            }"
          >
            <!-- R20 已签天给对勾角标，未来天整体降权：三态一眼可分 -->
            <text v-if="d <= claimedCount" class="day-check">✓</text>
            <text class="day-no">第 {{ d }} 天</text>
            <text class="day-coin">{{ rewardFor(d).coin ?? 0 }} 金</text>
            <text v-if="rewardFor(d).gem" class="day-gem">{{ rewardFor(d).gem }} 钻</text>
            <text v-if="rewardFor(d).energy" class="day-energy">{{ rewardFor(d).energy }} 体</text>
          </view>
        </view>
        <view
          class="btn btn-primary"
          :class="{ 'btn-disabled': canSign === false }"
          style="margin-top: 20rpx"
          @click="doSignIn"
        >
          {{ canSign ? '领取今日奖励' : signedToday ? '✓ 今日已签到' : `✓ ${cycleCompleteText}` }}
        </view>
        <text
          v-if="lastResult"
          :key="lastResult"
          class="muted"
          :class="{ shake: lastResult.startsWith('签到失败') }"
          style="display: block; margin-top: 16rpx"
        >
          {{ lastResult }}
        </text>
      </view>

      <!--
        离线收益（第六批 R1）：与签到同页。

        此前入口埋在「我的」页深处，而「每日签到」和「离线收益」都是
        每天回来第一件事要点的领取动作 —— 两个领取分在两页，玩家
        领完签到还要跨页去找挂机收益。这里把它们并到同一页，
        领完今日签到顺手就能领挂机金币。
      -->
      <view v-if="offlineReward" class="card offline-card">
        <text class="card-title">离线收益</text>
        <view class="row" style="justify-content: space-between; align-items: center">
          <view class="flex-1">
            <view class="row" style="gap: 12rpx; align-items: baseline">
              <text class="res-v mono">{{ offlineReward.coin }}</text>
              <text class="res-k">金币可领</text>
            </view>
            <text class="dim" style="display: block">
              已挂机 {{ offlineElapsed }}（上限 {{ offlineReward.cap_hours }} 小时）
            </text>
            <text
              v-if="offlineMsg"
              :key="offlineMsg"
              class="offline-msg"
              :class="{ shake: offlineMsg.startsWith('领取失败') }"
            >
              {{ offlineMsg }}
            </text>
          </view>
          <view
            class="btn btn-mini btn-primary offline-claim"
            :class="{ disabled: offlineReward.coin <= 0 || offlineClaiming }"
            @click="claimOffline"
          >
            {{ offlineClaiming ? '领取中' : offlineReward.coin > 0 ? '领取' : '已领完' }}
          </view>
        </view>
      </view>

      <view class="card">
        <!--
          ⚠️ 第 94 轮：加周期切换。

          此前这个页面**只**拉 `fetchTasks('daily')`，于是：

            3 条周任务（weekly_kills_200 / weekly_clears_20 / weekly_reactions_150）
            5 条成就（ach_first_clear / ach_reach_20 / ach_reach_60 /
                       ach_reach_100 / ach_reactions_100）

          **在客户端没有任何入口** —— 看不到、也领不到。
          `/tasks?scope=weekly` 与 `?scope=achievement` 有服务端实现、
          `client.ts` 里有 `fetchTasks(scope)`，只是**没有人调用**。

          第 76 轮修的「周任务永远领不到」在服务端是对的，
          但在客户端仍然够不着 —— 那是本轮要补的最后一环。
        -->
        <view class="flex-1" />
        <text class="card-title">任务</text>
        <view style="display: flex; gap: 12rpx">
          <text
            v-for="s in SCOPES"
            :key="s.key"
            class="dim"
            :style="scope === s.key ? 'color: #58a6ff' : ''"
            @click="switchScope(s.key)"
          >
            {{ s.label }}
            <!-- 第六批 R65：当前周期有几项可领，一眼可见，不必逐行找「领取」 -->
            <text v-if="scope === s.key && claimableCount > 0" class="task-claimable">{{ claimableCount }}</text>
          </text>
        </view>
        <view
          v-for="t in tasks"
          :key="t.id"
          class="task"
          :class="{ 'claim-pop': justClaimedTask === t.id }"
        >
          <view class="flex-1">
            <text class="task-name">{{ t.name }}</text>
            <text class="dim" style="display: block; margin-top: 4rpx">
              {{ t.progress }} / {{ t.target }}
            </text>
            <!-- 任务进度条（2026-11-01 新一轮第 36 轮） -->
            <view class="task-progress" :class="{ done: t.done }">
              <view
                class="task-progress-fill"
                :class="{ 'ok-pulse': t.done }"
                :style="{ width: taskPercent(t) + '%' }"
              />
            </view>
            <text class="task-percent" :class="{ done: t.done }">{{ taskPercent(t) }}%</text>
            <text class="dim" style="display: block">
              奖励：{{ rewardText(t.reward) }}
            </text>
          </view>
          <view
            class="btn"
            style="width: 140rpx; height: 64rpx; font-size: 24rpx"
            :class="{ 'btn-disabled': !(t.done && !t.claimed) }"
            @click="claim(t.id)"
          >
            {{ t.claimed ? '已领' : t.done ? '领取' : '未完成' }}
          </view>
        </view>
        <view v-if="tasks.length === 0" class="muted">暂无任务</view>
      </view>

      <view class="card">
        <text class="card-title">兑换码</text>
        <view class="row" style="gap: 12rpx">
          <input
            v-model="code"
            class="code-input"
            :class="{ 'input-error': !!redeemMsg && !redeemMsg.startsWith('兑换成功') }"
            placeholder="输入兑换码"
            placeholder-class="ph"
          />
          <view class="btn" style="width: 160rpx; height: 72rpx" @click="doRedeem">兑换</view>
        </view>
        <text v-if="redeemMsg" :key="redeemMsg" class="muted shake" style="display: block; margin-top: 12rpx">
          {{ redeemMsg }}
        </text>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { currencyName } from '@/utils/labels'
import { ref, computed, onMounted } from 'vue'
import { useGameStore } from '@/store/game'
import * as api from '@/api/client'

/** 加载中（2026-10-21 P6-1）：数据没到之前页面不能是空白 ——
 * 空白与「你什么都没有」长得一模一样，玩家不知道该等还是该重试。 */
const loading = ref(false)
const store = useGameStore()
const tasks = ref<any[]>([])
const claimedCount = ref(0)
const canSign = ref(true)
const signedToday = ref(false)
const lastResult = ref('')
const code = ref('')
const redeemMsg = ref('')
/** R34 成功反馈：刚签到/刚领奖的目标短暂高亮，随后自动熄灭。 */
const justSigned = ref(false)
const justClaimedTask = ref(0)
let feedbackTimer: ReturnType<typeof setTimeout> | null = null
function flashFeedback(kind: 'sign' | 'task', id = 0) {
  if (feedbackTimer) clearTimeout(feedbackTimer)
  justSigned.value = kind === 'sign'
  justClaimedTask.value = kind === 'task' ? id : 0
  feedbackTimer = setTimeout(() => {
    justSigned.value = false
    justClaimedTask.value = 0
    feedbackTimer = null
  }, 1200)
}
// 第 135 轮：七日签到奖励表改由服务端下发（权威），不再本地硬编码。
// key = day_index。未加载成功时为空对象 → rewardFor 回落本地缺省公式。
const calendar = ref<Record<number, Record<string, number>>>({})
/** 服务端日历天数：周期长度的唯一权威（缺省 7 与历史行为一致）。 */
const calendarDayCount = ref(7)
/** 天数格子序号（1..cycleDays），由服务端日历长度驱动。 */
const cycleDays = computed(() => Array.from({ length: calendarDayCount.value }, (_, i) => i + 1))
/** 顶部说明跟随真实周期，避免日历拉长后仍写「连续 7 天」。 */
const cycleHint = computed(() => `连续 ${calendarDayCount.value} 天，第 ${calendarDayCount.value} 天额外获得体力。`)
/** 周期签满文案：默认 7 天沿用历史「七日签到已完成」，其余长度用数字。 */
const cycleCompleteText = computed(() =>
  calendarDayCount.value === 7 ? '七日签到已完成' : `${calendarDayCount.value} 天签到已完成`,
)

async function loadCalendar() {
  try {
    const res = await api.fetchSignInCalendar()
    const m: Record<number, Record<string, number>> = {}
    for (const d of res.days ?? []) m[d.day_index] = d.reward
    calendar.value = m
    // 日历加载成功才采用服务端周期长度；失败/空表保留缺省 7，不阻断页面。
    if (res.days?.length) calendarDayCount.value = res.days.length
  } catch {
    /* 离线/失败：回落本地缺省公式（preview 可能与服务端不一致，但不阻断页面） */
  }
}

// 第 141 轮：签到初始态以服务端为准。修前 claimedCount=0 / canSign=true 写死，
// 当天已签到（或周期签完）后重进页面仍显示「可领取」，点了才报错。
async function loadStatus() {
  try {
    const res = await api.fetchSignInStatus()
    claimedCount.value = res.claimed_count
    signedToday.value = res.signed_today
    canSign.value = res.can_sign
  } catch {
    /* 拉取失败保持缺省（可点，点了由 POST 的权威结果兜底） */
  }
}

// 预览奖励优先用服务端日历；缺该天（或日历未加载）时回落本地缺省公式。
function localRewardFor(day: number): Record<string, number> {
  const r: Record<string, number> = { coin: 1000 * day }
  if (day === 3 || day === 7) r.gem = 20 * day
  if (day === 7) r.energy = 50
  return r
}

function rewardFor(day: number): Record<string, number> {
  return calendar.value[day] ?? localRewardFor(day)
}

function rewardText(reward: Record<string, number>): string {
  return Object.entries(reward ?? {})
    .map(([k, v]) => `${currencyName(k)} ${v}`)
    .join(' · ')
}

/** R40 签到连续进度（0–100%）：分母跟随服务端日历天数，不再写死 7。 */
const streakPct = computed(() => Math.min(100, Math.round((claimedCount.value / calendarDayCount.value) * 100)))

/* -------------------------------------------------------------------------- */
/* 离线收益（第六批 R1）                                                        */
/* 从「我的」页搬来，与签到同页 —— 两个每日领取动作放在同一处。                  */
/* -------------------------------------------------------------------------- */
const offlineReward = ref<api.OfflineRewardStatus | null>(null)
const offlineClaiming = ref(false)
const offlineMsg = ref('')

/** 挂机时长转可读文本（与旧「我的」页实现一致）。 */
function fmtElapsed(hours: number): string {
  if (!isFinite(hours) || hours <= 0) return '不足 1 小时'
  if (hours < 1) return `${Math.round(hours * 60)} 分钟`
  return `${hours.toFixed(1)} 小时`
}

const offlineElapsed = computed(() => fmtElapsed(offlineReward.value?.elapsed_hours ?? 0))

async function loadOfflineReward() {
  try {
    offlineReward.value = await api.offlineRewardStatus()
  } catch {
    /* 离线收益是辅助信息，加载失败不打扰签到主流程 */
  }
}

/** 领取离线收益：成功则刷新收益状态与钱包；失败保留按钮可重试。 */
async function claimOffline() {
  if (offlineClaiming.value || !offlineReward.value || offlineReward.value.coin <= 0) return
  offlineClaiming.value = true
  try {
    const res = await api.claimOfflineReward()
    offlineMsg.value = `领取成功：${res.coin} 金币`
    await Promise.all([loadOfflineReward(), store.refreshWallet()])
  } catch (e) {
    offlineMsg.value = `领取失败：${(e as Error).message}`
  } finally {
    offlineClaiming.value = false
  }
}

/** 第六批 R65：当前周期可领取的任务数。 */
const claimableCount = computed(() => tasks.value.filter((t) => t.done && !t.claimed).length)

/** 任务完成度百分比（2026-11-01 新一轮第 36 轮）。 */
function taskPercent(t: { progress: number; target: number }): number {
  if (!t.target || t.target <= 0) return 0
  return Math.min(100, Math.round((t.progress / t.target) * 100))
}


/**
 * SCOPES 是任务面板可切换的三种周期。
 *
 * ⚠️ 三者都必须有：服务端 `tasks.scope` 只有这三个取值
 * （`seedTasks` 里种的就是 daily / weekly / achievement），
 * 少一个就有一批任务在界面上消失。
 */
const SCOPES = [
  { key: 'daily', label: '每日' },
  { key: 'weekly', label: '每周' },
  { key: 'achievement', label: '成就' },
] as const

type ScopeKey = (typeof SCOPES)[number]['key']

const scope = ref<ScopeKey>('daily')

// 任务列表按周期切换存在「陈旧响应覆盖」竞态：快速 daily→weekly 时两个
// fetchTasks 并发，先发慢到的旧响应会用旧周期数据覆盖当前选中 tab 的列表。
// 用序号只认最后一次请求的结果（与 rank.vue 的 loadSeq 守卫同类）。
let loadSeq = 0

async function load() {
  const seq = ++loadSeq
  loading.value = true
  try {
    // ⚠️ 传当前 scope，而不是写死 'daily'。
    const res = await api.fetchTasks(scope.value)
    if (seq !== loadSeq) return // 已被更新的请求取代，丢弃陈旧响应
    tasks.value = res.items ?? []
  } catch (e) {
    if (seq !== loadSeq) return
    uni.showToast({ title: (e as Error).message, icon: 'none' })
  } finally {
    if (seq === loadSeq) loading.value = false
  }
}

/** 切换周期。scope 不是 ref 级别的响应式来源时这里会拉到同一份数据。 */
function switchScope(k: ScopeKey) {
  if (scope.value === k) return
  scope.value = k
  load()
}

async function doSignIn() {
  if (!canSign.value) return
  try {
    const res = await api.signIn()
    if (res.already) {
      canSign.value = false
      signedToday.value = true
      lastResult.value = '今日已签到'
    } else {
      claimedCount.value = res.day_index
      canSign.value = false
      signedToday.value = true
      lastResult.value = `签到成功，获得 ${rewardText(res.reward)}`
      flashFeedback('sign') // R34
      // 签到会 bumpTasks({signin:1}) 推进「今日完成签到」任务；不重拉则本会话一直停在 0/1
      await load()
      await store.refreshWallet()
    }
  } catch (e) {
    // 2026-10-21（P2-8）：失败时**不能**把按钮禁掉。
    //
    // 此前 catch 里 `canSign.value = false`，而按钮文案是
    // `canSign ? '领取今日奖励' : signedToday ? '今日已签到' : '七日签到已完成'`
    // —— 于是失败后按钮变成「**七日签到已完成**」：
    // 一句**假话**（今天既没签到、周期也没完成），而且本会话内再也点不了。
    //
    // 正确做法：保持可点（网络抖一下重试就好），把错误如实显示出来。
    lastResult.value = `签到失败：${(e as Error).message}`
  }
}

async function claim(id: number) {
  try {
    const res = await api.claimTask(id)
    flashFeedback('task', id) // R34
    uni.showToast({ title: `已领取 ${rewardText(res.reward)}`, icon: 'none' })
    await load()
    await store.refreshWallet()
  } catch (e) {
    uni.showToast({ title: (e as Error).message, icon: 'none' })
  }
}

async function doRedeem() {
  if (!code.value.trim()) return
  try {
    const res = await api.redeem(code.value.trim())
    redeemMsg.value = `兑换成功：${rewardText(res.reward)}`
    code.value = ''
    await store.refreshWallet()
  } catch (e) {
    redeemMsg.value = (e as Error).message
  }
}

onMounted(async () => {
  if (!store.loggedIn) await store.login()
  // 第 135 轮：拉服务端权威签到日历（与 load 并发，互不阻塞）
  // 第 141 轮：并发回读签到状态（点亮已签天 / 决定按钮可否点）
  // 第六批 R1：并发拉离线收益（与签到同页展示）
  await Promise.all([loadCalendar(), loadStatus(), load(), loadOfflineReward()])
})
</script>

<style scoped>
/* R40 连续签到进度条 */
.streak-track {
  height: 10rpx;
  background: var(--panel-3);
  border-radius: 999rpx;
  overflow: hidden;
  margin: 12rpx 0;
}

.streak-fill {
  height: 100%;
  border-radius: 999rpx;
  background: var(--accent-grad);
  transition: width 0.35s ease-out;
}

.days {
  display: grid;
  grid-template-columns: repeat(4, minmax(0, 1fr));
  gap: 12rpx;
  margin-top: 16rpx;
}

.day {
  background: var(--panel-2);
  border: 1rpx solid var(--border-soft, var(--border));
  border-radius: 14rpx;
  padding: 14rpx 8rpx;
  display: flex;
  flex-direction: column;
  align-items: center;
  position: relative;
  opacity: 0.55;
  transition: opacity 0.2s, border-color 0.2s, background 0.2s;
}

/* R20 三态：已签（对勾+绿框）/ 今日（橙光脉冲）/ 未来（进一步降权） */
.day.claimed {
  opacity: 1;
  border-color: rgba(126, 231, 135, 0.4);
  background: rgba(126, 231, 135, 0.06);
}

.day.today {
  opacity: 1;
  border-color: var(--fire);
  background: var(--fire-soft, rgba(255, 107, 53, 0.18));
  animation: glow-pulse 2s ease-in-out infinite;
}

.day.future {
  opacity: 0.3;
}

.day-check {
  position: absolute;
  top: 2rpx;
  right: 8rpx;
  font-size: 22rpx;
  color: var(--ok);
}

.day-no {
  font-size: 22rpx;
  color: var(--muted);
}

.day-coin {
  font-size: 22rpx;
  font-weight: 600;
  margin-top: 4rpx;
}

.day-gem,
.day-energy {
  font-size: 22rpx;
  color: var(--ok);
}

.task {
  display: flex;
  flex-direction: row;
  align-items: center;
  padding: 18rpx 0;
  border-bottom: 1rpx solid var(--border-soft, var(--border));
}

.task:last-child {
  border-bottom: none;
}

.task-name {
  font-size: 26rpx;
}

/* 任务进度条（2026-11-01 新一轮第 36 轮） */
.task-progress {
  height: 12rpx;
  background: rgba(127, 127, 127, 0.2);
  border-radius: 6rpx;
  overflow: hidden;
  margin-top: 8rpx;
}
.task-progress-fill {
  height: 100%;
  background: var(--ok, #3fb950);
  border-radius: 6rpx;
  transition: width 0.3s ease;
}

/* R20 完成态：渐变金 + 百分比变绿，未完成保持单一绿条 */
.task-progress.done .task-progress-fill {
  background: linear-gradient(90deg, var(--ok), var(--warn));
}

.task-percent {
  display: block;
  margin-top: 4rpx;
  font-size: 20rpx;
  color: var(--muted);
}

.task-percent.done {
  color: var(--ok);
  font-weight: 600;
}

/* 第六批 R65 可领任务数徽章 */
.task-claimable {
  display: inline-block;
  margin-left: 6rpx;
  min-width: 28rpx;
  text-align: center;
  border-radius: 999rpx;
  background: var(--fire);
  color: #fff;
  font-size: 18rpx;
}

/* 第六批 R1 离线收益反馈 */
.offline-msg {
  display: block;
  margin-top: 8rpx;
  font-size: 24rpx;
  color: var(--ok);
}

.offline-msg.shake {
  color: var(--bad);
}

/* R82 兑换失败输入框红框 */
.code-input.input-error {
  border-color: var(--bad);
}

.code-input {
  flex: 1;
  height: 72rpx;
  background: var(--panel-2);
  border: 1rpx solid var(--border-soft, var(--border));
  border-radius: 12rpx;
  padding: 0 20rpx;
  font-size: 26rpx;
  color: var(--text);
}

</style>
