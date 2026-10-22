<template>
  <view class="page-root">
    <view class="pad safe-bottom">
      <!-- 加载中 / 空态（2026-10-21 P6-1）-->
      <view v-if="store.configLoading" class="card">
        <text class="muted">加载中…</text>
      </view>
      <view v-else-if="allReactions.length === 0" class="card">
        <text class="muted">反应表还没加载出来</text>
      </view>
      <!-- 我的构筑能打出哪些反应 -->
      <view class="card">
        <text class="card-title">我的构筑能触发的反应</text>
        <view v-if="myReactions.length" class="chip-row">
          <view v-for="k in myReactions" :key="k" class="chip hit">{{ reactionName(k) }}</view>
        </view>
        <text v-else class="muted">
          当前出战技能的元素凑不出任何反应链 —— 反应伤害与面板攻击力无关，
          换一种元素搭配比堆面板更有效。
        </text>
        <text v-if="myReactions.length" class="muted" style="display: block; margin-top: 12rpx">
          共 {{ myReactions.length }} / {{ allReactions.length }} 条。反应伤害里攻击力的权重被结构性限制在
          30% 以内，所以「凑齐元素」比「堆攻击力」更决定胜负。
        </text>
      </view>

      <!-- 反应图鉴（服务端下发的权威表） -->
      <view class="card">
        <text class="card-title">反应图鉴</text>
        <text class="muted" style="display: block; margin-bottom: 12rpx">
          数值直接来自服务端 /config，与游戏内实际生效的一致。
        </text>
        <view v-for="r in allReactions" :key="r.key" class="rxn">
          <view class="rxn-head">
            <text class="rxn-name">{{ r.name }}</text>
            <text v-if="reachable.has(r.key as ReactionKey)" class="rxn-hit">可达</text>
          </view>
          <text class="rxn-descr">{{ r.descr || '（服务端未提供说明）' }}</text>
          <view class="rxn-stats">
            <text class="stat">系数 {{ r.base_coef }}</text>
            <text class="stat">攻击力权重 {{ r.attack_weight_pct }}%</text>
            <text v-if="r.status_duration_ms > 0" class="stat">
              持续 {{ (r.status_duration_ms / 1000).toFixed(1) }}s
            </text>
            <text v-if="r.aoe_radius > 0" class="stat">溅射半径 {{ r.aoe_radius }}</text>
            <text v-if="r.spread_stacks > 0" class="stat">层数传播 {{ r.spread_stacks }}</text>
            <text v-if="r.dispel_shield" class="stat">驱散护盾</text>
            <text v-if="r.amplify_pct > 0" class="stat">增幅 {{ r.amplify_pct }}%</text>
            <text v-if="r.armor_shred_permille > 0" class="stat">
              削甲 {{ r.armor_shred_permille }}‰
            </text>
            <text v-if="r.knockback > 0" class="stat">击退 {{ r.knockback / 1000 }}</text>
          </view>
        </view>
        <text v-if="!allReactions.length" class="muted">反应表为空（配置未加载？）</text>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { onLoad } from '@dcloudio/uni-app'
import { useGameStore } from '@/store/game'
import { reachableReactions, type ReactionKey } from '@/game/elements'

const store = useGameStore()

/**
 * 反应表（服务端 /config 下发）。
 *
 * 2026-10-21（P4-2）：此前 `/config.reactions` **全客户端零消费** ——
 * 服务端把 7 条反应的完整参数（系数 / 攻击力权重 / 持续时间 / 溅射半径 /
 * 驱散 / 增幅 / 削甲 / 击退）都下发下来了，客户端一处都没读。
 *
 * 后果：玩家查不到「冰 + 雷 = 什么反应、伤害多少、持续多久」，
 * 而反应驱动是这套设计的**核心机制**（I-1）。
 * 数值在服务端、内容在服务端，唯独**解释**缺失。
 */
const allReactions = computed(() => store.config?.reactions ?? [])

/**
 * 我的构筑可达的反应（P4-3）。
 *
 * `reachableReactions` 此前也只有测试在调 —— 玩家看不到
 * 「我这个构筑能打出哪些反应链」，只能打完一局才知道。
 */
const reachable = computed(() => reachableReactions(new Set(store.equippedElements)))

const myReactions = computed<ReactionKey[]>(() => {
  const out: ReactionKey[] = []
  for (const r of allReactions.value) {
    if (reachable.value.has(r.key as ReactionKey)) out.push(r.key as ReactionKey)
  }
  return out
})

function reactionName(key: string): string {
  return allReactions.value.find((r) => r.key === key)?.name ?? key
}

/**
 * 加载本页需要的数据。
 *
 * ⚠️ 2026-10-21（P1-2）：此前这个页面**什么都不加载** ——
 * 不 `login()`、不 `loadConfig()`、不 `loadLoadout()`。
 *
 * 后果：「我的构筑能触发的反应」**永远是空的**，
 * 而空的时候页面会**一本正经地说**「当前出战技能的元素凑不出任何反应链」
 * —— 一句**假话**（玩家明明是火+冰构筑）。
 *
 * 而且测试是绿的：测试自己手工调了 `store.loadLoadout()`，
 * 替页面做了页面该做的事。
 */
async function loadPage(): Promise<void> {
  if (!store.loggedIn) await store.login()
  await store.loadConfig()
  await store.loadLoadout()
}

onLoad(() => {
  void loadPage()
})

onMounted(() => {
  // onLoad 在 H5 上可能早于组件挂载，这里补一次（loadPage 本身幂等）。
  void loadPage()
})
</script>

<style scoped>
.chip-row {
  display: flex;
  flex-wrap: wrap;
  gap: 10rpx;
}
.chip {
  padding: 6rpx 16rpx;
  border-radius: 999rpx;
  background: rgba(88, 166, 255, 0.16);
  color: var(--fg);
  font-size: 24rpx;
}
.chip.hit {
  background: rgba(126, 231, 135, 0.2);
}
.rxn {
  padding: 16rpx 0;
  border-top: 1rpx solid rgba(127, 127, 127, 0.25);
}
.rxn:first-of-type {
  border-top: none;
}
.rxn-head {
  display: flex;
  align-items: center;
  gap: 12rpx;
}
.rxn-name {
  font-size: 30rpx;
  font-weight: 600;
}
.rxn-hit {
  padding: 2rpx 12rpx;
  border-radius: 999rpx;
  background: rgba(126, 231, 135, 0.2);
  color: #7ee787;
  font-size: 22rpx;
}
.rxn-descr {
  display: block;
  color: var(--muted, #9aa0a6);
  font-size: 24rpx;
  margin-top: 6rpx;
}
.rxn-stats {
  display: flex;
  flex-wrap: wrap;
  gap: 10rpx;
  margin-top: 8rpx;
}
.stat {
  padding: 4rpx 12rpx;
  border-radius: 8rpx;
  background: rgba(127, 127, 127, 0.14);
  font-size: 22rpx;
}
</style>
