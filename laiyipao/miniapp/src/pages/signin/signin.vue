<template>
  <view class="page-root">
    <view class="pad">
      <view class="card">
        <text class="card-title">每日签到</text>
        <text class="muted">连续 7 天，第 7 天额外获得体力。</text>
        <view class="days">
          <view
            v-for="d in 7"
            :key="d"
            class="day"
            :class="{ claimed: d <= claimedCount, today: d === claimedCount + 1 }"
          >
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
          {{ canSign ? '领取今日奖励' : signedToday ? '今日已签到' : '七日签到已完成' }}
        </view>
        <text v-if="lastResult" class="muted" style="display: block; margin-top: 16rpx">
          {{ lastResult }}
        </text>
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
          </text>
        </view>
        <view v-for="t in tasks" :key="t.id" class="task">
          <view class="flex-1">
            <text class="task-name">{{ t.name }}</text>
            <text class="dim" style="display: block; margin-top: 4rpx">
              {{ t.progress }} / {{ t.target }}
            </text>
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
            placeholder="输入兑换码"
            placeholder-class="ph"
          />
          <view class="btn" style="width: 160rpx; height: 72rpx" @click="doRedeem">兑换</view>
        </view>
        <text v-if="redeemMsg" class="muted" style="display: block; margin-top: 12rpx">
          {{ redeemMsg }}
        </text>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useGameStore } from '@/store/game'
import * as api from '@/api/client'

const store = useGameStore()
const tasks = ref<any[]>([])
const claimedCount = ref(0)
const canSign = ref(true)
const signedToday = ref(false)
const lastResult = ref('')
const code = ref('')
const redeemMsg = ref('')
// 第 135 轮：七日签到奖励表改由服务端下发（权威），不再本地硬编码。
// key = day_index。未加载成功时为空对象 → rewardFor 回落本地缺省公式。
const calendar = ref<Record<number, Record<string, number>>>({})

async function loadCalendar() {
  try {
    const res = await api.fetchSignInCalendar()
    const m: Record<number, Record<string, number>> = {}
    for (const d of res.days ?? []) m[d.day_index] = d.reward
    calendar.value = m
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

function currencyName(k: string): string {
  return k === 'coin' ? '金' : k === 'gem' ? '钻' : k === 'energy' ? '体' : k === 'keys' ? '钥匙' : k
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

async function load() {
  try {
    // ⚠️ 传当前 scope，而不是写死 'daily'。
    const res = await api.fetchTasks(scope.value)
    tasks.value = res.items ?? []
  } catch (e) {
    uni.showToast({ title: (e as Error).message, icon: 'none' })
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
      await store.refreshWallet()
    }
  } catch (e) {
    canSign.value = false
    lastResult.value = (e as Error).message
  }
}

async function claim(id: number) {
  try {
    const res = await api.claimTask(id)
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
  await Promise.all([loadCalendar(), loadStatus(), load()])
})
</script>

<style scoped>
.days {
  display: flex;
  flex-direction: row;
  flex-wrap: wrap;
  gap: 12rpx;
  margin-top: 16rpx;
}

.day {
  width: calc(25% - 9rpx);
  background: var(--panel-2);
  border: 1rpx solid var(--border);
  border-radius: 10rpx;
  padding: 14rpx 8rpx;
  display: flex;
  flex-direction: column;
  align-items: center;
  opacity: 0.55;
}

.day.claimed {
  opacity: 1;
  border-color: rgba(126, 231, 135, 0.4);
}

.day.today {
  opacity: 1;
  border-color: var(--fire);
  background: rgba(255, 107, 53, 0.1);
}

.day-no {
  font-size: 20rpx;
  color: var(--muted);
}

.day-coin {
  font-size: 22rpx;
  font-weight: 600;
  margin-top: 4rpx;
}

.day-gem,
.day-energy {
  font-size: 18rpx;
  color: var(--ok);
}

.task {
  display: flex;
  flex-direction: row;
  align-items: center;
  padding: 18rpx 0;
  border-bottom: 1rpx solid var(--border);
}

.task:last-child {
  border-bottom: none;
}

.task-name {
  font-size: 26rpx;
}

.code-input {
  flex: 1;
  height: 72rpx;
  background: var(--panel-2);
  border: 1rpx solid var(--border);
  border-radius: 10rpx;
  padding: 0 20rpx;
  font-size: 26rpx;
  color: var(--text);
}

.ph {
  color: var(--dim);
}
</style>
