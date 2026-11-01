<template>
  <view class="page-root">
    <view class="pad safe-bottom">
      <!-- 加载中 / 空态（与其它图鉴同款）-->
      <view v-if="store.configLoading" class="card">
        <text class="muted">加载中…</text>
      </view>
      <view v-else-if="skins.length === 0" class="card">
        <text class="muted">皮肤表还没加载出来</text>
      </view>

      <!-- 皮肤图鉴（服务端下发的权威表） -->
      <view class="card">
        <text class="card-title">皮肤图鉴</text>
        <!-- 搜索（2026-11-01 新一轮第 78 轮）：按名称/稀有度过滤 -->
        <input
          v-model="keyword"
          class="search-inp"
          placeholder="搜索皮肤名称 / 稀有度"
          placeholder-class="ph"
        />
        <text class="muted" style="display: block; margin-bottom: 12rpx">
          数值与说明直接来自服务端 /config。皮肤被动是<strong>机制型词条</strong>，
          改变战斗方式而非只加数值。
        </text>
        <view v-for="s in filteredSkins" :key="s.id" class="skin">
          <view class="rxn-head">
            <text class="rxn-name">{{ s.name }}</text>
            <text class="dim">{{ rarityLabel(s.rarity) }}</text>
          </view>
          <text class="rxn-descr">{{ s.descr || '（服务端未提供说明）' }}</text>
          <view class="rxn-stats">
            <text class="stat">被动：{{ passiveLabel(s.passive) }}</text>
          </view>
        </view>
        <text v-if="!skins.length" class="muted">皮肤表为空（配置未加载？）</text>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { useGameStore } from '@/store/game'

const store = useGameStore()

/** 皮肤图鉴展示所需的字段形状（服务端 /config.skins 元素）。 */
interface SkinDef {
  id: number
  code: string
  name: string
  rarity: string
  passive: string
  descr: string
}

/**
 * 皮肤图鉴（2026-10-31 新一轮第 35 轮）。
 *
 * `/config.skins` 从第一天起就下发了 6 款皮肤的稀有度/被动/说明，但
 * 客户端**零 UI 消费** —— 玩家不知道皮肤系统长什么样。这里补上图鉴页。
 */
const skins = computed(() => (store.config?.skins ?? []) as unknown as SkinDef[])

/** 图鉴搜索（2026-11-01 新一轮第 78 轮）：名称/稀有度中文过滤。 */
const keyword = ref('')

const filteredSkins = computed(() => {
  const kw = keyword.value.trim().toLowerCase()
  if (!kw) return skins.value
  return skins.value.filter((s) => {
    return s.name.toLowerCase().includes(kw) || rarityLabel(s.rarity).toLowerCase().includes(kw)
  })
})

function rarityLabel(r: string): string {
  const map: Record<string, string> = { rare: '稀有', epic: '史诗', legendary: '传说' }
  return map[r] ?? r
}

function passiveLabel(p: string): string {
  const map: Record<string, string> = {
    kinetic_bonus: '动能加成（额外破甲击退）',
    fire_burn: '燃烧（焰系命中附加一层燃烧）',
    ice_freeze: '冻结（冰系 20% 概率冻结）',
    volt_chain: '弹射 +1（电系）',
    blight_spread: '传播（毒系命中向周围传播 1 层）',
    boss_breaker: 'BOSS 特攻（反应伤害 +30%）',
  }
  return map[p] ?? p
}
</script>

<style scoped>
.skin {
  border-top: 1rpx solid var(--border, #eee);
  padding: 16rpx 0;
}

/* 图鉴搜索（2026-11-01 新一轮第 78 轮） */
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