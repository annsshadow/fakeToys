<template>
  <view class="page-root">
    <view class="pad">
      <!-- 章节选择 -->
      <scroll-view scroll-x class="chapter-scroll" :show-scrollbar="false">
        <view class="chapter-row">
          <view
            v-for="ch in chapters"
            :key="ch.id"
            class="chapter-chip"
            :class="{ active: curChapter === ch.id }"
            @click="switchChapter(ch.id)"
          >
            {{ ch.name }}
          </view>
        </view>
      </scroll-view>

      <!-- 关卡网格 -->
      <view class="grid">
        <view
          v-for="lv in chapterLevels"
          :key="lv.id"
          class="level-cell"
          :class="{
            locked: lv.id > store.unlockedLevel,
            cleared: starOf(lv.id) > 0,
            terrain: lv.terrain && lv.terrain.length > 0,
            boss: lv.is_boss,
          }"
          @click="enter(lv.id)"
        >
          <text class="level-no">{{ lv.id }}</text>
          <text v-if="lv.is_boss" class="level-badge">BOSS</text>
          <text v-else-if="lv.terrain && lv.terrain.length > 0" class="level-badge terrain">
            地形
          </text>
          <view v-if="starOf(lv.id) > 0" class="stars">
            <text v-for="i in 3" :key="i" class="star" :class="{ off: i > starOf(lv.id) }">★</text>
          </view>
        </view>
      </view>

      <!-- 当前关卡详情 -->
      <view v-if="selected" class="card">
        <text class="card-title">第 {{ selected.id }} 关 · {{ selected.name }}</text>
        <view class="info-row"><text class="muted">防线血量</text><text class="mono">{{ selected.base_hp }}</text></view>
        <view class="info-row"><text class="muted">波次</text><text class="mono">{{ selected.wave_count }}</text></view>
        <view class="info-row"><text class="muted">难度系数</text><text class="mono">{{ selected.difficulty }}‰</text></view>
        <view class="info-row"><text class="muted">体力消耗</text><text class="mono">{{ selected.energy_cost }}</text></view>
        <view class="info-row"><text class="muted">元素层数上限</text><text class="mono">{{ selected.element_cap }}</text></view>
        <view v-if="selected.terrain && selected.terrain.length" class="info-row">
          <text class="muted">地形</text>
          <text>{{ selected.terrain.map((t) => terrainName(t.kind)).join('、') }}</text>
        </view>
        <!--
          2026-10-21（P4-4）：地形**机制**。
          `TERRAIN_DESCR` 此前零消费 —— 选关页只显示地形**名字**，
          玩家无从知道油桶/潮汐闸/旋翼/崩塌掩体/蓄能塔分别做什么，
          于是「这关有地形」看起来只是个图标。
        -->
        <view v-for="(t, i) in selected.terrain" :key="i" class="terrain-note">
          <text class="terrain-note-name">{{ terrainName(t.kind) }}</text>
          <text class="terrain-note-descr">{{ terrainDescr(t.kind) }}</text>
        </view>
        <view class="star-targets">
          <text class="muted">星级门槛</text>
          <text v-for="(s, i) in selected.star_targets" :key="i" class="target">
            {{ i + 1 }}★ {{ s }}
          </text>
        </view>
      </view>

      <view
        class="btn btn-primary"
        :class="{ 'btn-disabled': !canEnter }"
        @click="startBattle"
      >
        {{ startLabel }}
      </view>

      <!-- 连续失败时展示针对性诊断（L-1） -->
      <view v-if="diagnoseInfo" class="card" style="border-color: var(--warn)">
        <text class="card-title">
          <text>卡关诊断</text>
          <text class="tag tag-neutral">置信度 {{ diagnoseInfo.confidence }}%</text>
        </text>
        <text class="diag-title">{{ diagnoseInfo.title }}</text>
        <text class="muted" style="display: block; margin-top: 8rpx">{{ diagnoseInfo.detail }}</text>
        <text v-for="(s, i) in diagnoseInfo.suggestions" :key="i" class="diag-tip">· {{ s }}</text>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { onLoad } from '@dcloudio/uni-app'
import { useGameStore } from '@/store/game'
import * as api from '@/api/client'
import { TERRAIN_NAME, TERRAIN_DESCR } from '@/game/terrain'
import type { LevelSummary, TerrainKind } from '@/game/types'

const store = useGameStore()

/**
 * 地形机制说明（2026-10-21 P4-4）。
 *
 * `TERRAIN_DESCR` 是引擎侧的地形说明表，此前**没有任何 UI 消费**。
 * 未知地形回退成「（无机制说明）」而不是空串 ——
 * 空串会让那一行看起来像没内容，而不是像「我不知道」。
 */
function terrainDescr(kind: TerrainKind): string {
  return TERRAIN_DESCR[kind] ?? '（无机制说明）'
}

const curChapter = ref(1)
const selectedId = ref(0)
const stars = ref<Record<number, number>>({})
const diagnoseInfo = ref<any>(null)

const chapters = computed(() => store.config?.chapters ?? [])

const chapterLevels = computed<LevelSummary[]>(() => {
  const all = store.config?.levels ?? []
  return all.filter((l) => l.chapter === curChapter.value)
})

// 选关页只读元数据（章节/名称/难度/体力/星级/地形），**不需要 waves** ——
// 所以这里用摘要类型就够，不必为此把完整关卡拉回来。
const selected = computed<LevelSummary | null>(() => {
  if (selectedId.value) return store.levelMap.get(selectedId.value) ?? null
  return chapterLevels.value.find((l) => l.id === store.unlockedLevel) ?? null
})

const canEnter = computed(() => !!selected.value && selected.value.id <= store.unlockedLevel)

const startLabel = computed(() => {
  if (!selected.value) return '未选择关卡'
  if (selected.value.id > store.unlockedLevel) return `第 ${selected.value.id} 关未解锁`
  if (store.wallet.energy < selected.value.energy_cost) return '体力不足'
  return `开始战斗（消耗 ${selected.value.energy_cost} 体力）`
})

function starOf(id: number): number {
  return stars.value[id] ?? 0
}

function terrainName(kind: string): string {
  return TERRAIN_NAME[kind as TerrainKind] ?? kind
}

function switchChapter(id: number) {
  curChapter.value = id
  selectedId.value = 0
  diagnoseInfo.value = null
}

function enter(id: number) {
  selectedId.value = id
  diagnoseInfo.value = null
}

function startBattle() {
  if (!canEnter.value || !selected.value) {
    if (store.wallet.energy < (selected.value?.energy_cost ?? 0)) {
      uni.showToast({ title: '体力不足', icon: 'none' })
    }
    return
  }
  uni.navigateTo({ url: `/pages/battle/battle?level=${selected.value.id}` })
}

onLoad(async (query) => {
  if (!store.loggedIn) await store.login()
  const lv = Number(query?.level ?? 0)
  if (lv > 0) {
    const target = store.levelMap.get(lv)
    if (target) curChapter.value = target.chapter
    selectedId.value = lv
  }
  // 星级以服务端的 level_stars（历史 GREATEST）为准：
  // 不拉这份数据，stars.value 永远空，starOf() 恒为 0，
  // 「已通关」与星级行在选关页永远不亮。
  try {
    const r = await api.fetchMyStars()
    stars.value = r.stars
  } catch {
    /* 星级拉取失败不阻塞选关 */
  }
  // 卡关诊断：失败次数由**服务端**算（2026-10-21 P2-7）。
  //
  // 此前这里传的是硬编码的 2，于是**每个人**第一次进关都能看到
  // 「卡关诊断」卡片 —— 一张对所有人说同一句话的卡片。
  // 现在只认服务端给的 `stuck`（连续失败 >= 2）。
  try {
    const d = await api.diagnose(selected.value?.id ?? store.unlockedLevel)
    if (d && d.stuck && d.stage !== 'unclear') diagnoseInfo.value = d
  } catch {
    /* 诊断不可用不阻塞选关 */
  }
})

onMounted(() => {
  if (!store.loggedIn) store.login()
})
</script>

<style scoped>
.chapter-scroll {
  white-space: nowrap;
  margin-bottom: 20rpx;
}

.chapter-row {
  display: flex;
  flex-direction: row;
  gap: 12rpx;
}

.chapter-chip {
  padding: 12rpx 24rpx;
  border-radius: 999rpx;
  background: var(--panel);
  border: 1rpx solid var(--border);
  font-size: 24rpx;
  color: var(--muted);
  white-space: nowrap;
}

.chapter-chip.active {
  background: rgba(255, 107, 53, 0.16);
  border-color: var(--fire);
  color: var(--fire);
}

.grid {
  display: flex;
  flex-direction: row;
  flex-wrap: wrap;
  gap: 16rpx;
  margin-bottom: 20rpx;
}

.level-cell {
  width: calc(25% - 12rpx);
  height: 150rpx;
  border-radius: 16rpx;
  background: var(--panel);
  border: 1rpx solid var(--border);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  position: relative;
}

.level-cell.locked {
  opacity: 0.35;
}

.level-cell.cleared {
  border-color: rgba(126, 231, 135, 0.4);
}

.level-cell.terrain {
  border-color: rgba(201, 167, 255, 0.4);
}

.level-cell.boss {
  border-color: var(--fire);
  background: linear-gradient(160deg, #2a1a1f, #161b22);
}

.level-no {
  font-size: 36rpx;
  font-weight: 700;
  font-variant-numeric: tabular-nums;
}

.level-badge {
  position: absolute;
  top: 6rpx;
  right: 8rpx;
  font-size: 16rpx;
  color: var(--fire);
}

.level-badge.terrain {
  color: var(--kinetic);
}

.stars {
  display: flex;
  flex-direction: row;
  margin-top: 4rpx;
}

.star {
  font-size: 18rpx;
  color: var(--warn);
}

.star.off {
  color: var(--border);
}

.info-row {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  padding: 8rpx 0;
  font-size: 26rpx;
}
/* 地形机制说明（2026-10-21 P4-4）：只显示名字不够，玩家要知道它做什么 */
.terrain-note {
  display: flex;
  flex-direction: row;
  gap: 12rpx;
  padding: 8rpx 0 8rpx 8rpx;
  border-left: 4rpx solid rgba(88, 166, 255, 0.5);
}
.terrain-note-name {
  flex-shrink: 0;
  font-size: 24rpx;
  color: #58a6ff;
}
.terrain-note-descr {
  flex: 1;
  font-size: 24rpx;
  color: var(--muted, #9aa0a6);
  line-height: 1.5;
}

.star-targets {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 20rpx;
  margin-top: 12rpx;
  padding-top: 12rpx;
  border-top: 1rpx solid var(--border);
}

.target {
  font-size: 22rpx;
  color: var(--warn);
}

.diag-title {
  font-size: 28rpx;
  font-weight: 600;
  color: var(--warn);
}

.diag-tip {
  font-size: 22rpx;
  color: var(--muted);
  line-height: 1.7;
  display: block;
  margin-top: 6rpx;
}
</style>
