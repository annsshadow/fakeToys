<template>
  <view class="page-root">
    <view class="pad safe-bottom">
      <!-- 加载中 / 空态（与敌人图鉴同款）-->
      <view v-if="store.configLoading" class="card">
        <text class="muted">加载中…</text>
      </view>
      <view v-else-if="skills.length === 0" class="card">
        <text class="muted">技能表还没加载出来</text>
      </view>

      <!-- 技能图鉴（服务端下发的权威表） -->
      <view class="card">
        <text class="card-title">技能图鉴</text>
        <text class="muted" style="display: block; margin-bottom: 12rpx">
          数值直接来自服务端 /config，与游戏内实际生效的一致。
          {{ ownedCount }} / {{ skills.length }} 个已解锁（背包可查看拥有状态）。
        </text>
        <view v-for="s in skills" :key="s.id" class="skill">
          <view class="rxn-head">
            <text class="rxn-name">{{ s.name }}</text>
            <text class="tag" :class="'tag-' + s.element">{{ elementName(s.element) }}</text>
            <text class="dim">{{ s.kind === 'active' ? '主动' : '被动' }}</text>
            <text v-if="isOwned(s.id)" class="chip hit">已拥有</text>
          </view>
          <text class="rxn-descr">{{ s.descr || '（服务端未提供说明）' }}</text>
          <view class="rxn-stats">
            <text class="stat">基础伤害 {{ s.base_damage }}</text>
            <text v-if="s.heat_cost > 0" class="stat">热量 {{ s.heat_cost }}</text>
            <text v-if="s.cooldown_ms > 0" class="stat">
              冷却 {{ (s.cooldown_ms / 1000).toFixed(1) }}s
            </text>
            <text v-if="s.pierce > 1" class="stat">穿透 {{ s.pierce }}</text>
            <text v-if="s.aoe_radius > 0" class="stat">溅射 {{ s.aoe_radius }}</text>
            <text v-if="s.chain > 1" class="stat">连锁 {{ s.chain }}</text>
            <text class="stat">解锁等级 {{ s.unlock_level }}</text>
          </view>
        </view>
        <text v-if="!skills.length" class="muted">技能表为空（配置未加载？）</text>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useGameStore } from '@/store/game'
import { ELEMENT_NAME, type Element } from '@/game/elements'

const store = useGameStore()

/**
 * 技能图鉴（2026-10-31 新一轮第 25 轮）。
 *
 * `/config.skills` 从第一天起就下发了全部技能的伤害/热量/冷却/穿透/
 * 溅射/解锁等级，但客户端只在背包展示**已拥有**的技能 —— 玩家看不到
 * 未解锁技能的长什么样、解锁等级是多少，成长路径全靠猜。
 * 这里补上图鉴页，并标注已拥有状态。
 */
const skills = computed(() => store.config?.skills ?? [])

/**
 * 已拥有判据与背包一致：服务端 `build.skills` 里有没有这一行
 * （`store.build` 来自 /me 权威快照，见 bag.vue 的 isOwned）。
 */
const ownedSkillIds = computed(() => {
  const bag = store.build?.skills
  if (!bag) return new Set<number>()
  return new Set(Object.keys(bag).map(Number))
})

const ownedCount = computed(
  () => skills.value.filter((s) => ownedSkillIds.value.has(s.id)).length,
)

function isOwned(id: number): boolean {
  return ownedSkillIds.value.has(id)
}

function elementName(e: Element): string {
  return ELEMENT_NAME[e] ?? e
}
</script>

<style scoped>
.skill {
  border-top: 1rpx solid var(--border, #eee);
  padding: 16rpx 0;
}
</style>