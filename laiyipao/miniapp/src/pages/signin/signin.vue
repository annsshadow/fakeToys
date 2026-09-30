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
          {{ canSign ? '领取今日奖励' : '今日已签到' }}
        </view>
        <text v-if="lastResult" class="muted" style="display: block; margin-top: 16rpx">
          {{ lastResult }}
        </text>
      </view>

      <view class="card">
        <text class="card-title">每日任务</text>
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
const lastResult = ref('')
const code = ref('')
const redeemMsg = ref('')

function rewardFor(day: number): Record<string, number> {
  const r: Record<string, number> = { coin: 1000 * day }
  if (day === 3 || day === 7) r.gem = 20 * day
  if (day === 7) r.energy = 50
  return r
}

function rewardText(reward: Record<string, number>): string {
  return Object.entries(reward ?? {})
    .map(([k, v]) => `${currencyName(k)} ${v}`)
    .join(' · ')
}

function currencyName(k: string): string {
  return k === 'coin' ? '金' : k === 'gem' ? '钻' : k === 'energy' ? '体' : k === 'keys' ? '钥匙' : k
}

async function load() {
  try {
    const res = await api.fetchTasks('daily')
    tasks.value = res.items ?? []
  } catch (e) {
    uni.showToast({ title: (e as Error).message, icon: 'none' })
  }
}

async function doSignIn() {
  if (!canSign.value) return
  try {
    const res = await api.signIn()
    if (res.already) {
      canSign.value = false
      lastResult.value = '今日已签到'
    } else {
      claimedCount.value = res.day_index
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
  await load()
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
