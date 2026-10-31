<template>
  <view class="page-root">
    <view class="pad safe-bottom">
      <!-- 加载中 / 空态（与反应图鉴同款，2026-10-21 P6-1）-->
      <view v-if="store.configLoading" class="card">
        <text class="muted">加载中…</text>
      </view>
      <view v-else-if="enemies.length === 0" class="card">
        <text class="muted">敌人表还没加载出来</text>
      </view>

      <!-- 敌人图鉴（服务端下发的权威表） -->
      <view class="card">
        <text class="card-title">敌人图鉴</text>
        <text class="muted" style="display: block; margin-bottom: 12rpx">
          数值直接来自服务端 /config，与游戏内实际生效的一致。
          抗性为千分比：正值减伤、负值易伤。
        </text>
        <view v-for="en in enemies" :key="en.id" class="enemy">
          <view class="rxn-head">
            <text class="rxn-name">{{ en.name }}</text>
            <text class="dim">{{ categoryLabel(en.category) }}</text>
            <text v-if="en.is_boss" class="chip hit">Boss</text>
          </view>
          <text class="rxn-descr">{{ en.descr || '（服务端未提供说明）' }}</text>
          <view class="rxn-stats">
            <text class="stat">血量 {{ en.hp }}</text>
            <text class="stat">速度 {{ (en.speed / 1000).toFixed(1) }}</text>
            <text v-if="en.armor > 0" class="stat">护甲 {{ en.armor }}</text>
            <text v-if="en.shield_hp > 0" class="stat">护盾 {{ en.shield_hp }}</text>
            <text class="stat">攻击 {{ en.attack }}</text>
            <text v-if="en.attack_interval > 0" class="stat">
              间隔 {{ (en.attack_interval / 1000).toFixed(1) }}s
            </text>
            <text v-if="en.fly_height > 0" class="stat">飞行高度 {{ en.fly_height }}</text>
            <text v-if="en.burrow" class="stat">钻地</text>
          </view>
          <!-- 元素抗性（千分比） -->
          <view v-if="resistEntries(en).length" class="rxn-stats">
            <text v-for="[el, v] in resistEntries(en)" :key="el" class="stat">
              {{ elementName(el as Element) }} {{ v > 0 ? '+' : '' }}{{ (v / 10).toFixed(1) }}%
            </text>
          </view>
          <text v-else class="muted">无元素抗性</text>
        </view>
        <text v-if="!enemies.length" class="muted">敌人表为空（配置未加载？）</text>
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
 * 敌人图鉴（2026-10-31 新一轮第 24 轮）。
 *
 * `/config.enemies` 从第一天起就下发了完整的敌人属性（血量/护甲/护盾/
 * 攻击/速度/五维抗性），但客户端**零 UI 消费** —— 玩家在选关/战斗前
 * 无法了解每种敌人的属性与抗性，「为什么这关这么难」只能靠试错。
 * 这里补上图鉴页。
 */
const enemies = computed(() => store.config?.enemies ?? [])

function categoryLabel(c: string): string {
  const map: Record<string, string> = {
    normal: '普通',
    ranged: '远程',
    flying: '飞行',
    special: '特殊',
    elite: '精英',
    boss: 'Boss',
  }
  return map[c] ?? c
}

function elementName(e: Element): string {
  return ELEMENT_NAME[e] ?? e
}

/** 只展示非零抗性，避免列表被一堆 0 值刷屏。 */
function resistEntries(en: { resist?: Record<string, number> }): [string, number][] {
  return Object.entries(en.resist ?? {}).filter(([, v]) => v !== 0)
}
</script>

<style scoped>
.enemy {
  border-top: 1rpx solid var(--border, #eee);
  padding: 16rpx 0;
}
</style>