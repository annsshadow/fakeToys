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
        <!-- 搜索（2026-11-01 新一轮第 77 轮）：按反应名过滤 -->
        <input
          v-model="keyword"
          class="search-inp"
          placeholder="搜索反应名称"
          placeholder-class="ph"
        />
        <text class="muted" style="display: block; margin-bottom: 12rpx">
          数值直接来自服务端 /config，与游戏内实际生效的一致。
        </text>
        <text class="result-count">显示 {{ filteredReactions.length }} / 共 {{ allReactions.length }} 条</text>
        <view v-for="r in filteredReactions" :key="r.key" class="rxn">
          <view class="rxn-head">
            <!-- 反应图标：双环交叠（两个元素圈相碰的隐喻） -->
            <view class="icon-badge" style="width: 64rpx; height: 64rpx">
              <DefIcon name="reaction" :size="40" />
            </view>
            <text class="rxn-name">{{ r.name }}</text>
            <text v-if="reachable.has(r.key as ReactionKey)" class="rxn-hit">可达</text>
          </view>
          <!-- 触发元素对（本地由 lookupReaction 反查，与服务端反应表同源） -->
          <view v-if="reactionPairs(r.key)" class="rxn-stats">
            <view v-for="(pair, i) in reactionPairs(r.key)" :key="i" class="pair-chip">
              <text class="tag" :class="'tag-' + pair[0]">{{ ELEMENT_NAME[pair[0]] }}</text>
              <text class="dim">+</text>
              <text class="tag" :class="'tag-' + pair[1]">{{ ELEMENT_NAME[pair[1]] }}</text>
            </view>
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
        <!-- R28 搜索无结果：明确区分「表是空的」与「关键词没匹配上」 -->
        <view v-if="keyword && allReactions.length && filteredReactions.length === 0" class="search-empty">
          <text class="muted">没有匹配「{{ keyword }}」的反应</text>
          <text class="clear-search" @click="keyword = ''">清除搜索</text>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { onLoad } from '@dcloudio/uni-app'
import { useGameStore } from '@/store/game'
import { ELEMENT_NAME, reachableReactions, type Element, type ReactionKey } from '@/game/elements'
import { REACTION_PAIRS } from '@/utils/reactionPairs'
import DefIcon from '@/components/DefIcon.vue'

const store = useGameStore()

/** 反应 → 触发元素对（派生表见 utils/reactionPairs，与图例共用）。 */
function reactionPairs(key: string): Array<[Element, Element]> {
  return REACTION_PAIRS[key as ReactionKey] ?? []
}

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

/** 图鉴搜索（2026-11-01 新一轮第 77 轮）：按反应名过滤。 */
const keyword = ref('')

const filteredReactions = computed(() => {
  const kw = keyword.value.trim().toLowerCase()
  if (!kw) return allReactions.value
  return allReactions.value.filter((r) => r.name.toLowerCase().includes(kw))
})

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
/* 图鉴样式已下沉到 styles/common.css（跨 6 个图鉴页复用） */
/* 触发元素对：两个元素 tag + 加号的小胶囊组 */
.pair-chip {
  display: inline-flex;
  align-items: center;
  gap: 8rpx;
  padding: 2rpx 10rpx;
  border-radius: 999rpx;
  background: rgba(127, 127, 127, 0.12);
}
</style>
