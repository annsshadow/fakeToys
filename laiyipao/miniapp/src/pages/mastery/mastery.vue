<template>
  <view class="page-root">
    <view class="pad">
      <view class="card">
        <view class="card-title">
          <text>专精点</text>
          <text class="points mono">{{ points }}</text>
        </view>
        <text class="muted" style="display: block">
          每层只能选 2 个节点（4 选 2），且第 2、3 层必须先点亮前一层 ——
          专精点不够点满任何一系，<b>必须做方向选择</b>。这是"专精点有限"之外的第二重约束。
        </text>
        <!-- 重铸入口（第 150 轮）。
             没有它，`mastery_reset` 就是商店里一件「买了不知道干嘛」的商品 ——
             服务端这个端点从第 74 轮起就存在，客户端从来没接过。 -->
        <view class="reset-row">
          <text class="dim">重铸道具 {{ resetTokens }}</text>
          <text
            class="link"
            :class="{ 'link-off': resetting || selected.length === 0 || resetTokens <= 0 }"
            @click="resetMastery"
          >
            {{ resetting ? '重铸中…' : '重铸' }}
          </text>
        </view>
        <text class="muted" style="display: block; margin-top: 8rpx">
          重铸清空全部已点亮节点，<b>点数不变</b> —— 点错方向后唯一的改向途径。
        </text>
      </view>

      <scroll-view scroll-x class="tab-scroll" :show-scrollbar="false">
        <view class="tab-row">
          <view
            v-for="f in families"
            :key="f.family"
            class="tab-chip"
            :class="{ active: curFamily === f.family }"
            @click="curFamily = f.family"
          >
            {{ f.name }}
            <text v-if="invested(f.family) > 0" class="tab-dot" />
          </view>
        </view>
      </scroll-view>

      <view v-if="curNodes.length" class="card">
        <text class="card-title">{{ curName }}专精树</text>
        <view v-for="layer in 3" :key="layer" class="layer">
          <view class="layer-head">
            <text class="layer-title">第 {{ layer }} 层</text>
            <text class="dim">已选 {{ pickedInLayer(layer) }}/2</text>
          </view>
          <view class="node-row">
            <view
              v-for="n in nodesInLayer(layer)"
              :key="n.id"
              class="node"
              :class="{
                on: selected.includes(n.id),
                locked: isLocked(n),
              }"
              @click="toggle(n)"
            >
              <text class="node-name">{{ n.name }}</text>
              <text class="node-value">+{{ n.value }}</text>
            </view>
          </view>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useGameStore } from '@/store/game'
import * as api from '@/api/client'
import type { MasteryNode } from '@/game/types'

const store = useGameStore()
const points = ref(0)
const selected = ref<number[]>([])
const curFamily = ref('flame')
/** mastery_reset 道具余额（服务端 /wallet 的 tokens 下发） */
const resetTokens = computed(() => store.wallet.tokens?.['mastery_reset'] ?? 0)
const resetting = ref(false)

const families = computed(() => store.config?.mastery_families ?? [])

const curNodes = computed<MasteryNode[]>(() => {
  const f = families.value.find((x) => x.family === curFamily.value)
  return f?.nodes ?? []
})

const curName = computed(
  () => families.value.find((x) => x.family === curFamily.value)?.name ?? curFamily.value,
)

function nodesInLayer(layer: number): MasteryNode[] {
  return curNodes.value.filter((n) => n.layer === layer)
}

function pickedInLayer(layer: number): number {
  return nodesInLayer(layer).filter((n) => selected.value.includes(n.id)).length
}

function isLocked(n: MasteryNode): boolean {
  if (n.layer > 1 && pickedInLayer(n.layer - 1) === 0) return true
  if (pickedInLayer(n.layer) >= 2) return true
  return false
}

function invested(family: string): number {
  const f = families.value.find((x) => x.family === family)
  if (!f) return 0
  return f.nodes.filter((n) => selected.value.includes(n.id)).length
}

async function toggle(n: MasteryNode) {
  if (selected.value.includes(n.id)) {
    uni.showToast({ title: '已点亮的节点需通过重置取消', icon: 'none' })
    return
  }
  if (isLocked(n)) {
    uni.showToast({ title: n.layer > 1 && pickedInLayer(n.layer - 1) === 0 ? '请先点亮上一层' : '本层已选满 2 个', icon: 'none' })
    return
  }
  if (points.value <= 0) {
    uni.showToast({ title: '专精点不足（升级可获得更多）', icon: 'none' })
    return
  }
  try {
    const res = await api.allocateMastery(n.id)
    points.value = res.points
    selected.value = res.nodes ?? []
    await store.refreshProfile()
  } catch (e) {
    const msg = (e as any)?.error?.message ?? (e as Error).message
    uni.showToast({ title: msg, icon: 'none', duration: 2500 })
  }
}

async function load() {
  try {
    const res = await api.fetchMastery()
    points.value = res.points
    selected.value = res.nodes ?? []
  } catch (e) {
    uni.showToast({ title: (e as Error).message, icon: 'none' })
  }
}

/**
 * 重铸专精。
 *
 * 三条前置判断都在本地先挡掉，但**真正的裁定始终在服务端**：
 * 道具够不够、节点有没有，服务端说了算。本地挡只是为了
 * 「没有道具时不要把按钮做成可点的样子」。
 *
 * 成功之后用回执里的状态直接刷 UI —— 回执回显的就是清完之后的真实状态，
 * 再拉一次 `GET /mastery` 是白跑一趟，而且会有一个中间闪烁。
 */
async function resetMastery() {
  if (resetting.value) return // 防连点：道具扣完第二次必失败，但失败提示很吵
  if (resetTokens.value <= 0) {
    uni.showToast({ title: '没有重铸道具（商城可购）', icon: 'none' })
    return
  }
  if (selected.value.length === 0) {
    uni.showToast({ title: '当前没有已点亮的节点', icon: 'none' })
    return
  }
  resetting.value = true
  try {
    const res = await api.resetMastery()
    points.value = res.points
    selected.value = res.nodes ?? []
    // 道具余额以服务端为准：本地减 1 在并发购买时会让余额短暂不准。
    await store.refreshWallet()
    await store.refreshProfile()
    uni.showToast({ title: `已重铸，清掉 ${res.reset_cleared} 个节点`, icon: 'none' })
  } catch (e) {
    const msg = (e as any)?.error?.message ?? (e as Error).message
    uni.showToast({ title: msg, icon: 'none', duration: 2500 })
  } finally {
    resetting.value = false
  }
}

onMounted(async () => {
  if (!store.loggedIn) await store.login()
  await Promise.all([load(), store.refreshWallet()])
})
</script>

<style scoped>
.points {
  font-size: 44rpx;
  font-weight: 700;
  color: var(--kinetic);
}

.tab-scroll {
  white-space: nowrap;
  margin-bottom: 20rpx;
}

.tab-row {
  display: flex;
  flex-direction: row;
  gap: 12rpx;
}

.tab-chip {
  position: relative;
  padding: 12rpx 28rpx;
  border-radius: 999rpx;
  background: var(--panel);
  border: 1rpx solid var(--border);
  font-size: 24rpx;
  color: var(--muted);
  white-space: nowrap;
}

.tab-chip.active {
  background: rgba(201, 167, 255, 0.16);
  border-color: var(--kinetic);
  color: var(--kinetic);
}

.tab-dot {
  position: absolute;
  top: 6rpx;
  right: 8rpx;
  width: 10rpx;
  height: 10rpx;
  border-radius: 50%;
  background: var(--ok);
}

.reset-row {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
  margin-top: 16rpx;
}

.link-off {
  color: var(--muted) !important;
  pointer-events: none;
}

.layer {
  margin-bottom: 28rpx;
}

.layer-head {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 12rpx;
}

.layer-title {
  font-size: 24rpx;
  color: var(--muted);
}

.node-row {
  display: flex;
  flex-direction: row;
  gap: 12rpx;
}

.node {
  flex: 1;
  background: var(--panel-2);
  border: 1rpx solid var(--border);
  border-radius: 12rpx;
  padding: 18rpx 8rpx;
  display: flex;
  flex-direction: column;
  align-items: center;
  min-height: 110rpx;
}

.node.on {
  background: rgba(201, 167, 255, 0.16);
  border-color: var(--kinetic);
}

.node.locked {
  opacity: 0.35;
}

.node-name {
  font-size: 20rpx;
  text-align: center;
}

.node-value {
  font-size: 22rpx;
  color: var(--kinetic);
  font-weight: 600;
  margin-top: 8rpx;
}
</style>
