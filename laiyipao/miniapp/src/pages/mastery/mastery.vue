<template>
  <view class="page-root">
    <view class="pad safe-bottom">
      <!-- 加载中 / 空态（2026-10-21 P6-1）-->
      <view v-if="loading" class="card">
        <text class="muted">加载中…</text>
      </view>
      <view class="card">
        <view class="card-title">
          <text>专精点</text>
          <text class="points mono">{{ points }}</text>
        </view>
        <text class="muted" style="display: block">
          每层只能选 2 个节点（4 选 2），且第 2、3 层必须先点亮前一层 ——
          专精点不够点满任何一系，<b>必须做方向选择</b>。这是"专精点有限"之外的第二重约束。
        </text>
        <!-- R57 点数从哪来：此前界面只显示一个数字，玩家不知道怎么变多 -->
        <text class="dim" style="display: block; margin-top: 8rpx">
          初始 3 点，每提升 10 级 +1 点（随战斗经验成长）。
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
            <view v-if="resetting" class="spinner" style="display: inline-block" />
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
            <!-- 第六批 R61：已投点数直接写在页签上，此前只有一个小圆点看不出多少 -->
            <text v-if="invested(f.family) > 0" class="tab-invest">+{{ invested(f.family) }}</text>
          </view>
        </view>
      </scroll-view>

      <!-- G71 专精投入总览：跨系柱状图，看清点数倾斜与各系进度 -->
      <view v-if="familyOverview.length" class="card overview-card">
        <view class="card-title">
          <text>专精投入总览</text>
          <text class="mono">共 {{ totalInvested }} 点</text>
        </view>
        <view v-for="f in familyOverview" :key="f.family" class="fo-row" :class="{ active: f.active }">
          <text class="dim fo-name">{{ f.name }}</text>
          <view class="fo-track">
            <view class="fo-fill" :style="{ width: f.pct + '%' }" />
          </view>
          <text class="mono fo-val">{{ f.got }}/{{ f.cap }}</text>
        </view>
        <!-- G103 收益汇总：专精点换来了什么 -->
        <view v-if="investedValue.length" class="invested-value">
          <text v-for="v in investedValue" :key="v.kind" class="iv-chip">{{ v.label }} +{{ v.value }}</text>
        </view>
      </view>

      <!-- G128 全系 4选2 结构矩阵 -->
      <view v-if="familyMatrix.length" class="card">
        <view class="card-title">全系结构</view>
        <view class="fm-row fm-head">
          <text class="dim fm-name">系</text>
          <text v-for="l in [1,2,3]" :key="l" class="dim fm-cell">第{{ l }}层</text>
        </view>
        <view v-for="f in familyMatrix" :key="f.family" class="fm-row">
          <text class="fm-name">{{ f.name }}</text>
          <text v-for="l in f.layers" :key="l.layer" class="fm-cell" :class="{ on: l.got > 0 }">{{ l.got }}/{{ l.cap }}</text>
        </view>
      </view>

      <view v-if="curNodes.length" class="card">
        <text class="card-title">{{ curName }}专精树</text>
        <!-- 第六批 R62：说明锁定原因，玩家不必靠点一下才收到 toast 才知道 -->
        <text class="dim" style="display: block; margin-bottom: 12rpx">
          锁定节点需先在上一层点亮 2 项；每层最多选 2 项。
        </text>
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
                selectable: !selected.includes(n.id) && !isLocked(n),
              }"
              @click="toggle(n)"
            >
              <!-- 已点亮勾选标（100 轮打磨 R15）：三态此前只有底色差别，
                   已选与可选在深色背景下几乎不可分。加角标一眼可辨。 -->
              <view v-if="selected.includes(n.id)" class="node-check">✓</view>
              <!-- R43 可选未点亮节点给「+」脉冲，强调这里还能点 -->
              <text v-else-if="!isLocked(n)" class="node-plus">+</text>
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

/** 加载中（2026-10-21 P6-1）：数据没到之前页面不能是空白 ——
 * 空白与「你什么都没有」长得一模一样，玩家不知道该等还是该重试。 */
const loading = ref(false)
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

/**
 * 按层分组的节点（2026-10-21 P6-6）。
 *
 * 此前 `nodesInLayer` 每次调用都**重新 filter 一遍** `curNodes`，
 * 而它在模板的 `v-for` 里被逐节点调用（每层 4 个节点 × 3 层），
 * `pickedInLayer` 又在里面再 filter 一次 + 对 `selected` 做 `includes`
 * —— 每帧 O(n²)。
 *
 * 改成 computed 之后只算一次。
 */
const nodesByLayer = computed(() => {
  const m = new Map<number, MasteryNode[]>()
  for (const n of curNodes.value) {
    const list = m.get(n.layer)
    if (list) list.push(n)
    else m.set(n.layer, [n])
  }
  return m
})

/** 已点亮的节点 id 集合（`includes` 在一个 Set 上是 O(1)）。 */
const selectedSet = computed(() => new Set(selected.value))

function nodesInLayer(layer: number): MasteryNode[] {
  return nodesByLayer.value.get(layer) ?? []
}

function pickedInLayer(layer: number): number {
  let n = 0
  for (const node of nodesInLayer(layer)) {
    if (selectedSet.value.has(node.id)) n++
  }
  return n
}

function isLocked(n: MasteryNode): boolean {
  if (n.layer > 1 && pickedInLayer(n.layer - 1) === 0) return true
  if (pickedInLayer(n.layer) >= 2) return true
  return false
}

function invested(family: string): number {
  const f = families.value.find((x) => x.family === family)
  if (!f) return 0
  let n = 0
  for (const node of f.nodes) {
    if (selectedSet.value.has(node.id)) n++
  }
  return n
}

/* -------------------------------------------------------------------------- */
/* G71 专精投入总览（第七批）                                                   */
/* 跨所有系的点数分布柱状图 + 剩余可点节点数，帮玩家看「专精往哪倾斜、还能点哪」。  */
/* 数据来自 families + selectedSet，纯客户端派生。                               */
/* -------------------------------------------------------------------------- */
const familyOverview = computed(() => {
  const max = Math.max(1, ...families.value.map((f) => f.nodes.length))
  return families.value.map((f) => {
    const got = f.nodes.filter((n) => selectedSet.value.has(n.id)).length
    return {
      family: f.family,
      name: f.name,
      got,
      cap: f.nodes.length,
      pct: Math.round(got / max * 100),
      active: f.family === curFamily.value,
    }
  })
})
const totalInvested = computed(() => familyOverview.value.reduce((s, f) => s + f.got, 0))

/** G128 全系 4选2 结构矩阵：每系一行、三层，格内显示该层已选数/总数。 */
const familyMatrix = computed(() =>
  families.value.map((f) => ({
    family: f.family,
    name: f.name,
    layers: [1, 2, 3].map((layer) => {
      const nodes = f.nodes.filter((n) => n.layer === layer)
      const got = nodes.filter((n) => selectedSet.value.has(n.id)).length
      return { layer, got, cap: nodes.length }
    }),
  })),
)

/** G103 已点亮节点属性收益汇总：按 kind 分组累加 value，看清专精点换来了什么。 */
const investedValue = computed(() => {
  const m = new Map<string, number>()
  for (const f of families.value) {
    for (const n of f.nodes) {
      if (selectedSet.value.has(n.id)) {
        m.set(n.kind, (m.get(n.kind) ?? 0) + n.value)
      }
    }
  }
  const KIND_CN: Record<string, string> = {
    attack: '攻击', heat: '热量', element: '元素', reaction: '反应',
    armor: '护甲', crit: '暴击', slot: '额外槽位', mechanic: '机制',
  }
  return [...m.entries()]
    .map(([kind, value]) => ({ kind, label: KIND_CN[kind] ?? kind, value }))
    .sort((a, b) => b.value - a.value)
})

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
  loading.value = true
  try {
    const res = await api.fetchMastery()
    points.value = res.points
    selected.value = res.nodes ?? []
  } catch (e) {
    uni.showToast({ title: (e as Error).message, icon: 'none' })
  } finally {
    loading.value = false
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
/* G128 全系结构矩阵 */
.fm-row { display: flex; flex-direction: row; align-items: center; padding: 6rpx 0; }
.fm-head { border-bottom: 1rpx solid var(--border); }
.fm-name { flex: 1.4; font-size: 22rpx; }
.fm-cell { flex: 1; text-align: center; font-size: 22rpx; color: var(--dim); }
.fm-cell.on { color: var(--kinetic); font-weight: 600; }

/* G103 专精收益汇总 */
.invested-value { display: flex; flex-wrap: wrap; gap: 8rpx; margin-top: 12rpx; }
.iv-chip {
  background: var(--kinetic-soft);
  color: var(--kinetic);
  border-radius: 8rpx;
  padding: 4rpx 12rpx;
  font-size: 22rpx;
}

/* G71 专精投入总览 */
.overview-card .card-title .mono { color: var(--kinetic); }
.fo-row { display: flex; flex-direction: row; align-items: center; gap: 12rpx; margin: 8rpx 0; }
.fo-row.active .fo-name { color: var(--kinetic); font-weight: 600; }
.fo-name { width: 140rpx; flex-shrink: 0; }
.fo-track { flex: 1; height: 10rpx; background: var(--panel-3); border-radius: 999rpx; overflow: hidden; }
.fo-fill { height: 100%; background: var(--kinetic); border-radius: 999rpx; }
.fo-val { width: 80rpx; text-align: right; font-size: 22rpx; }

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

/* 第六批 R61 页签已投点数徽章 */
.tab-invest {
  margin-left: 8rpx;
  color: var(--ok);
  font-size: 20rpx;
}

.reset-row {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
  margin-top: 16rpx;
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
  position: relative;
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

/* 可选未选（100 轮打磨 R15）：比锁定态更亮的可交互提示（描边 + 轻抬） */
.node.selectable {
  border-color: rgba(201, 167, 255, 0.45);
  background: var(--panel-2);
}

.node.selectable:active {
  transform: scale(0.97);
}

/* R43 可选节点「+」脉冲 */
.node-plus {
  position: absolute;
  top: 3rpx;
  right: 8rpx;
  font-size: 22rpx;
  line-height: 1;
  color: var(--epic);
  animation: node-plus-pulse 1.8s ease-in-out infinite;
}

@keyframes node-plus-pulse {
  0%, 100% { opacity: 0.5; }
  50% { opacity: 1; }
}

/* 已点亮勾选角标 */
.node-check {
  position: absolute;
  top: 4rpx;
  right: 6rpx;
  font-size: 20rpx;
  line-height: 1;
  color: var(--kinetic);
  font-weight: 700;
}

.node.locked {
  opacity: 0.35;
}

.node-name {
  font-size: 22rpx;
  text-align: center;
}

.node-value {
  font-size: 22rpx;
  color: var(--kinetic);
  font-weight: 600;
  margin-top: 8rpx;
}
</style>
