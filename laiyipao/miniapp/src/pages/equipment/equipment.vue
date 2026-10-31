<template>
  <view class="page-root">
    <view class="pad safe-bottom">
      <!-- 加载中 / 空态（与其它图鉴同款）-->
      <view v-if="store.configLoading" class="card">
        <text class="muted">加载中…</text>
      </view>
      <view v-else-if="items.length === 0" class="card">
        <text class="muted">装备表还没加载出来</text>
      </view>

      <!-- 装备图鉴（服务端下发的权威表） -->
      <view class="card">
        <text class="card-title">装备图鉴</text>
        <text class="muted" style="display: block; margin-bottom: 12rpx">
          数值直接来自服务端 /config，与游戏内实际生效的一致。
          {{ ownedCount }} / {{ items.length }} 件已拥有。
        </text>
        <view v-for="e in items" :key="e.id" class="equip">
          <view class="rxn-head">
            <text class="rxn-name">{{ e.name }}</text>
            <text class="dim">{{ slotLabel(e.slot) }} · {{ tierLabel(e.tier) }}</text>
            <text v-if="e.element" class="tag" :class="'tag-' + e.element">{{ elementName(e.element) }}</text>
            <text v-if="isOwned(e.id)" class="chip hit">已拥有</text>
          </view>
          <text class="rxn-descr">{{ e.descr || '（服务端未提供说明）' }}</text>
          <view class="rxn-stats">
            <text v-if="e.base_armor > 0" class="stat">护甲 {{ e.base_armor }}</text>
            <text v-if="e.base_bonus_pct > 0" class="stat">加成 {{ e.base_bonus_pct }}%</text>
            <text class="stat">解锁等级 {{ e.unlock_level }}</text>
          </view>
        </view>
        <text v-if="!items.length" class="muted">装备表为空（配置未加载？）</text>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useGameStore } from '@/store/game'
import { ELEMENT_NAME, type Element } from '@/game/elements'

const store = useGameStore()

/** 装备图鉴展示所需的字段形状（服务端 /config.equipment 元素）。 */
interface EquipmentDef {
  id: number
  code: string
  name: string
  slot: string
  tier: number
  element: Element
  descr: string
  base_armor: number
  base_bonus_pct: number
  unlock_level: number
}

/**
 * 装备图鉴（2026-10-31 新一轮第 26 轮）。
 *
 * `/config.equipment` 从第一天起就下发了全部 18 件装备的护甲/加成/
 * 解锁等级，但客户端只在背包展示**已拥有**的装备 —— 玩家看不到
 * 未解锁装备的成长路径。这里补上图鉴页，并标注已拥有状态。
 */
const items = computed(() => (store.config?.equipment ?? []) as unknown as EquipmentDef[])

/** 已拥有判据：服务端 build 快照的 equipment 数组（装备 id 列表）。 */
const ownedIds = computed(() => new Set(store.build?.equipment ?? []))

const ownedCount = computed(() => items.value.filter((e) => ownedIds.value.has(e.id)).length)

function isOwned(id: number): boolean {
  return ownedIds.value.has(id)
}

function slotLabel(s: string): string {
  const map: Record<string, string> = {
    weapon: '武器',
    helmet: '头盔',
    armor: '胸甲',
    gloves: '手套',
    boots: '靴子',
    charm: '护符',
  }
  return map[s] ?? s
}

function tierLabel(t: number): string {
  return `${'★'.repeat(t)}`
}

function elementName(e: Element): string {
  return ELEMENT_NAME[e] ?? e
}
</script>

<style scoped>
.equip {
  border-top: 1rpx solid var(--border, #eee);
  padding: 16rpx 0;
}
</style>