<template>
  <view class="page-root home">
    <!-- 顶部资源栏 -->
    <view class="topbar">
      <view class="player">
        <view class="avatar">炮</view>
        <view class="player-info">
          <text class="nick">{{ store.nickname || '玩家' }}</text>
          <text class="muted">{{ store.isGuest ? '游客账号' : '微信账号' }}</text>
        </view>
      </view>
      <view class="res-row">
        <text class="res">金 {{ store.wallet.coin }}</text>
        <text class="res">钻 {{ store.wallet.gem }}</text>
        <text class="res">体 {{ store.wallet.energy }}</text>
      </view>
    </view>

    <view v-if="store.loginError" class="pad">
      <view class="card" style="border-color: var(--bad)">
        <text class="card-title">无法连接服务器</text>
        <text class="muted">{{ store.loginError }}</text>
        <text class="dim" style="display: block; margin-top: 12rpx">
          请先启动后端：cd server && go run ./cmd/api
        </text>
        <view class="btn btn-primary" style="margin-top: 20rpx" @click="retry">重试</view>
      </view>
    </view>

    <view v-else-if="store.configLoading" class="pad">
      <view class="card"><text class="muted">正在加载游戏配置…</text></view>
    </view>

    <view v-else class="pad">
      <!-- 构筑评分（I-7）：主指标不是"战力 9999" -->
      <view class="card">
        <view class="card-title">
          <text>构筑评分</text>
          <text class="score mono">{{ store.buildRating?.total ?? 0 }}</text>
        </view>

        <!-- 五维进度条：明确告诉玩家"缺什么" -->
        <view v-for="d in dims" :key="d.label" class="dim-row">
          <text class="dim-label">{{ d.label }}</text>
          <view class="bar">
            <view class="bar-fill" :style="{ width: d.pct + '%', background: d.color }" />
          </view>
          <text class="dim-value mono">{{ d.value }}</text>
        </view>

        <!-- 短板提示：这是 I-7 真正想做的事 -->
        <view v-if="weaknesses.length" class="weakness-box">
          <text class="weakness-title">搭配建议</text>
          <text v-for="(w, i) in weaknesses" :key="i" class="weakness-item">· {{ w }}</text>
        </view>
      </view>

      <!-- 主行动 -->
      <view class="action-card" @click="goStage">
        <view class="flex-1">
          <text class="action-title">第 {{ store.unlockedLevel }} 关</text>
          <text class="action-sub">守住防线，清完全部波次</text>
        </view>
        <text class="action-go">开始 ›</text>
      </view>

      <!-- 功能入口 -->
      <view class="grid">
        <view v-for="m in menus" :key="m.path" class="grid-item" @click="go(m.path)">
          <text class="grid-icon">{{ m.icon }}</text>
          <text class="grid-label">{{ m.label }}</text>
        </view>
      </view>

      <!-- 元素说明 -->
      <view class="card">
        <text class="card-title">5 种元素</text>
        <view class="elem-row">
          <view v-for="e in elements" :key="e.key" class="elem-chip">
            <view class="dot" :style="{ background: e.color }" />
            <text class="elem-name">{{ e.name }}</text>
          </view>
        </view>
        <text class="muted" style="display: block; margin-top: 12rpx">
          敌人对不同元素的抗性不同。两种元素叠加会触发反应链，反应伤害与面板攻击力无关 ——
          这是低养成玩家翻盘的主要来源。
        </text>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { onShow } from '@dcloudio/uni-app'
import { useGameStore } from '@/store/game'
import { ELEMENT_ORDER, ELEMENT_NAME, ELEMENT_COLOR } from '@/game/elements'

const store = useGameStore()

const elements = ELEMENT_ORDER.map((k) => ({
  key: k,
  name: ELEMENT_NAME[k],
  color: ELEMENT_COLOR[k],
}))

const menus = [
  { path: '/pages/stage/stage', label: '关卡', icon: '🎯' },
  { path: '/pages/bag/bag', label: '背包', icon: '🎒' },
  { path: '/pages/mastery/mastery', label: '专精', icon: '🌲' },
  { path: '/pages/signin/signin', label: '签到', icon: '📅' },
  { path: '/pages/rank/rank', label: '排行', icon: '🏆' },
  { path: '/pages/verify/verify', label: '验真', icon: '🔍' },
  { path: '/pages/me/me', label: '我的', icon: '👤' },
]

/** 构筑评分五维进度。短板用醒目色，直接引导玩家去补。
 * 分母必须与服务端 BuildRating 各维上界一致（domain/power.go）：
 *   元素 0..5 / 反应 0..7 / 专精 0..8 / 装备契合 0..18 / 机制深度 0..8。
 * 第 138 轮修：① 此前只有 4 维（漏 mechanic_depth）；② equipment_synergy
 * 分母误写 6（真实上界 18），导致契合度条形永远虚高（6/18 就显示满格）。 */
const dims = computed(() => {
  const r = store.buildRating
  return [
    {
      label: '元素覆盖',
      value: `${r?.element_coverage ?? 0}/5`,
      pct: ((r?.element_coverage ?? 0) / 5) * 100,
      color: '#58a6ff',
    },
    {
      label: '反应链',
      value: `${r?.reaction_coverage ?? 0}/7`,
      pct: ((r?.reaction_coverage ?? 0) / 7) * 100,
      color: '#7ee787',
    },
    {
      label: '专精投入',
      value: `${r?.mastery_done ?? 0}/8`,
      pct: ((r?.mastery_done ?? 0) / 8) * 100,
      color: '#c9a7ff',
    },
    {
      label: '装备契合',
      value: `${r?.equipment_synergy ?? 0}/18`,
      pct: Math.min(100, ((r?.equipment_synergy ?? 0) / 18) * 100),
      color: '#ffd33d',
    },
    {
      label: '机制深度',
      value: `${r?.mechanic_depth ?? 0}/8`,
      pct: ((r?.mechanic_depth ?? 0) / 8) * 100,
      color: '#ff7b72',
    },
  ]
})

const weaknesses = computed<string[]>(() => store.buildRating?.weaknesses ?? [])

function go(path: string) {
  uni.navigateTo({ url: path })
}

function goStage() {
  uni.navigateTo({ url: `/pages/stage/stage?level=${store.unlockedLevel}` })
}

async function retry() {
  await store.login()
}

onMounted(async () => {
  if (!store.loggedIn) await store.login()
})

onShow(async () => {
  if (store.loggedIn) await store.refreshProfile()
})
</script>

<style scoped>
.home {
  padding-top: 100rpx;
}

.topbar {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 10;
  background: #10151c;
  border-bottom: 1rpx solid var(--border);
  padding: 20rpx 24rpx;
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
}

.player {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 16rpx;
}

.avatar {
  width: 72rpx;
  height: 72rpx;
  border-radius: 16rpx;
  background: linear-gradient(135deg, #ff6b35, #ff3d71);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 34rpx;
  font-weight: 700;
  color: #fff;
}

.player-info {
  display: flex;
  flex-direction: column;
}

.nick {
  font-size: 28rpx;
  font-weight: 600;
}

.res-row {
  display: flex;
  flex-direction: row;
  gap: 20rpx;
}

.res {
  font-size: 24rpx;
  color: var(--muted);
  font-variant-numeric: tabular-nums;
}

.score {
  font-size: 48rpx;
  font-weight: 700;
  color: var(--fire);
}

.dim-row {
  display: flex;
  flex-direction: row;
  align-items: center;
  margin-bottom: 14rpx;
}

.dim-label {
  width: 140rpx;
  font-size: 24rpx;
  color: var(--muted);
}

.bar {
  flex: 1;
  height: 14rpx;
  background: var(--panel-2);
  border-radius: 7rpx;
  overflow: hidden;
  margin: 0 16rpx;
}

.bar-fill {
  height: 100%;
  border-radius: 7rpx;
}

.dim-value {
  width: 80rpx;
  text-align: right;
  font-size: 22rpx;
  color: var(--muted);
}

.weakness-box {
  margin-top: 20rpx;
  padding: 18rpx;
  background: rgba(255, 107, 53, 0.08);
  border-left: 4rpx solid var(--fire);
  border-radius: 8rpx;
}

.weakness-title {
  font-size: 22rpx;
  color: var(--fire);
  font-weight: 600;
  display: block;
  margin-bottom: 8rpx;
}

.weakness-item {
  font-size: 22rpx;
  color: var(--muted);
  line-height: 1.7;
  display: block;
}

.action-card {
  display: flex;
  flex-direction: row;
  align-items: center;
  background: linear-gradient(135deg, #1f2a38, #16202b);
  border: 1rpx solid var(--border);
  border-radius: 16rpx;
  padding: 32rpx 28rpx;
  margin-bottom: 20rpx;
}

.action-title {
  font-size: 38rpx;
  font-weight: 700;
  display: block;
}

.action-sub {
  font-size: 24rpx;
  color: var(--muted);
  display: block;
  margin-top: 8rpx;
}

.action-go {
  font-size: 32rpx;
  color: var(--fire);
  font-weight: 600;
}

.grid {
  display: flex;
  flex-direction: row;
  flex-wrap: wrap;
  gap: 16rpx;
  margin-bottom: 20rpx;
}

.grid-item {
  width: calc(25% - 12rpx);
  background: var(--panel);
  border: 1rpx solid var(--border);
  border-radius: 16rpx;
  padding: 24rpx 0;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.grid-icon {
  font-size: 40rpx;
}

.grid-label {
  font-size: 22rpx;
  color: var(--muted);
  margin-top: 10rpx;
}

.elem-row {
  display: flex;
  flex-direction: row;
  gap: 24rpx;
  flex-wrap: wrap;
}

.elem-chip {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 8rpx;
}

.dot {
  width: 20rpx;
  height: 20rpx;
  border-radius: 50%;
}

.elem-name {
  font-size: 24rpx;
  color: var(--muted);
}
</style>
