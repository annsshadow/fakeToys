<template>
  <view class="page-root">
    <view class="pad safe-bottom">
      <!-- 加载中 / 空态（与其它图鉴同款）-->
      <view v-if="store.configLoading" class="card">
        <text class="muted">加载中…</text>
      </view>
      <view v-else-if="gems.length === 0" class="card">
        <text class="muted">宝石表还没加载出来</text>
      </view>

      <!-- 宝石图鉴（服务端下发的权威表） -->
      <view class="card">
        <text class="card-title">宝石图鉴</text>
        <!-- 搜索（2026-11-01 新一轮第 75 轮）：按名称/属性过滤 -->
        <input
          v-model="keyword"
          class="search-inp"
          placeholder="搜索宝石名称 / 属性"
          placeholder-class="ph"
        />
        <text class="muted" style="display: block; margin-bottom: 12rpx">
          数值与描述直接来自服务端 /config。共 {{ ownedCount }} 颗已拥有。
        </text>
        <view v-for="g in filteredGems" :key="g.id" class="gem">
          <view class="rxn-head">
            <text class="rxn-name">{{ g.name }}</text>
            <text class="dim">{{ attrLabel(g.attr) }}</text>
            <text v-if="ownedCountOf(g.id) > 0" class="chip hit">已拥有 ×{{ ownedCountOf(g.id) }}</text>
          </view>
          <text class="rxn-descr">{{ g.descr || '（服务端未提供说明）' }}</text>
        </view>
        <text v-if="!gems.length" class="muted">宝石表为空（配置未加载？）</text>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { onShow } from '@dcloudio/uni-app'
import { useGameStore } from '@/store/game'
import { fetchGems, type OwnedGem } from '@/api/client'

const store = useGameStore()

/** 宝石图鉴展示所需的字段形状（服务端 /config.gems 元素）。 */
interface GemDef {
  id: number
  code: string
  name: string
  attr: string
  descr: string
}

/**
 * 宝石图鉴（2026-10-31 新一轮第 27 轮）。
 *
 * `/config.gems` 从第一天起就下发了 8 种宝石的属性与说明，但客户端只在
 * 背包展示**已拥有**的宝石实例 —— 玩家看不到全部宝石的成长路径。
 * 这里补上图鉴页，并标注每种宝石的拥有数量。
 */
const gems = computed(() => (store.config?.gems ?? []) as unknown as GemDef[])

/** 图鉴搜索（2026-11-01 新一轮第 75 轮）：名称/属性中文过滤。 */
const keyword = ref('')

const filteredGems = computed(() => {
  const kw = keyword.value.trim().toLowerCase()
  if (!kw) return gems.value
  return gems.value.filter((g) => {
    return g.name.toLowerCase().includes(kw) || attrLabel(g.attr).toLowerCase().includes(kw)
  })
})

/** 已拥有的宝石实例列表（/me/gems）。 */
const ownedGems = ref<OwnedGem[]>([])

const ownedCount = computed(() => ownedGems.value.length)

function ownedCountOf(gemId: number): number {
  return ownedGems.value.filter((g) => g.gem_id === gemId).length
}

function attrLabel(a: string): string {
  const map: Record<string, string> = {
    attack: '攻击',
    element_coef: '元素系数',
    crit: '暴击率',
    reaction_mult: '反应倍率',
    element_cap: '元素层数上限',
    heat_cap: '热量上限',
    armor: '护甲',
    skill_damage: '技能伤害',
  }
  return map[a] ?? a
}

onShow(() => {
  if (!store.loggedIn) return
  fetchGems()
    .then((res) => {
      ownedGems.value = res.gems ?? []
    })
    .catch(() => {
      // 图鉴的拥有状态是辅助信息，加载失败不打扰主流程
    })
})
</script>

<style scoped>
.gem {
  border-top: 1rpx solid var(--border, #eee);
  padding: 16rpx 0;
}

/* 图鉴搜索（2026-11-01 新一轮第 75 轮） */
.search-inp {
  display: block;
  height: 72rpx;
  padding: 0 20rpx;
  margin-bottom: 12rpx;
  background: var(--panel-2, #f5f6f7);
  border: 1rpx solid var(--border, #eee);
  border-radius: 12rpx;
  font-size: 26rpx;
  color: var(--text, #222);
}

.ph {
  color: var(--dim, #9aa0a6);
}
</style>