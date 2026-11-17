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
      <view class="card stagger-in">
        <text class="card-title">技能图鉴</text>
        <!-- 搜索（2026-11-01 新一轮第 71 轮）：按名称/元素/主动被动过滤 -->
        <input
          v-model="keyword"
          class="search-inp"
          placeholder="搜索技能名称 / 元素 / 主动被动"
          placeholder-class="ph"
        />
        <text class="muted" style="display: block; margin-bottom: 12rpx">
          数值直接来自服务端 /config，与游戏内实际生效的一致。
          {{ ownedCount }} / {{ skills.length }} 个已解锁（背包可查看拥有状态）。
        </text>
        <!-- G66 技能元素×类型覆盖矩阵：每格「已拥有/总数」，缺口一眼可见 -->
        <view class="card-title" style="font-size: 26rpx">元素 × 类型覆盖</view>
        <view class="skill-matrix">
          <view class="sm-row">
            <view class="sm-head" />
            <view v-for="k in kindCols" :key="'h' + k.key" class="sm-head">{{ k.label }}</view>
          </view>
          <view v-for="row in skillMatrix" :key="row.el" class="sm-row">
            <view class="sm-head" :style="{ color: row.color }">{{ ELEMENT_NAME[row.el] }}</view>
            <view
              v-for="cell in row.cells"
              :key="row.el + cell.key"
              class="sm-cell"
              :class="{ covered: cell.owned > 0 }"
            >
              {{ cell.owned }}/{{ cell.total }}
            </view>
          </view>
        </view>

        <text class="result-count">显示 {{ filteredSkills.length }} / 共 {{ skills.length }} 条</text>
        <view v-for="s in filteredSkills" :key="s.id" class="rxn skill">
          <view class="rxn-head">
            <view class="icon-badge" :class="'badge-' + s.element" style="width: 64rpx; height: 64rpx">
              <DefIcon :name="'elem:' + s.element" :size="40" />
            </view>
            <text class="rxn-name">{{ s.name }}</text>
            <text class="tag" :class="'tag-' + s.element">{{ elementName(s.element) }}</text>
            <!-- R67 主动/被动配色区分：扫列表时不必读字 -->
            <text class="kind-tag" :class="s.kind === 'active' ? 'kind-active' : 'kind-passive'">
              {{ s.kind === 'active' ? '主动' : '被动' }}
            </text>
            <text v-if="isOwned(s.id)" class="chip hit">已拥有</text>
            <!-- 第六批 R46：未解锁技能明示状态，避免「在图鉴看到却拿不到」的困惑 -->
            <text v-else class="chip">未解锁</text>
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
            <!-- 第六批 R45：明确「需 Lv.N」而非裸数字 -->
            <text class="stat">需 Lv.{{ s.unlock_level }}</text>
          </view>
        </view>
        <text v-if="!skills.length" class="muted">技能表为空（配置未加载？）</text>
        <!-- R25 搜索无结果：明确区分「表是空的」与「关键词没匹配上」 -->
        <view v-if="keyword && skills.length && filteredSkills.length === 0" class="search-empty">
          <text class="muted">没有匹配「{{ keyword }}」的技能</text>
          <text class="clear-search" @click="keyword = ''">清除搜索</text>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { useGameStore } from '@/store/game'
import { ELEMENT_NAME, ELEMENT_ORDER, ELEMENT_COLOR, type Element } from '@/game/elements'
import DefIcon from '@/components/DefIcon.vue'

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

/** 图鉴搜索（2026-11-01 新一轮第 71 轮）：名称/元素/主动被动过滤。 */
const keyword = ref('')

const filteredSkills = computed(() => {
  const kw = keyword.value.trim().toLowerCase()
  if (!kw) return skills.value
  return skills.value.filter((s) => {
    return (
      s.name.toLowerCase().includes(kw) ||
      elementName(s.element).toLowerCase().includes(kw) ||
      (s.kind === 'active' ? '主动' : '被动').includes(kw)
    )
  })
})

/**
 * 已拥有判据与背包一致：服务端 `build.skills` 里有没有这一行
 * （`store.build` 来自 /me 权威快照，见 bag.vue 的 isOwned）。
 */
const ownedSkillIds = computed(() => {
  const bag = store.build?.skills
  if (!bag) return new Set<number>()
  return new Set(Object.keys(bag).map(Number))
})

/* -------------------------------------------------------------------------- */
/* G66 技能元素×类型覆盖矩阵（第七批）                                          */
/* 5 元素 × 主动/被动，格子显示「已拥有/总数」，帮玩家看出哪类元素缺技能。        */
/* -------------------------------------------------------------------------- */
const kindCols = [
  { key: 'active', label: '主动' },
  { key: 'passive', label: '被动' },
] as const
const skillMatrix = computed(() =>
  ELEMENT_ORDER.map((el) => ({
    el,
    color: ELEMENT_COLOR[el],
    cells: kindCols.map((k) => {
      const all = skills.value.filter((s) => s.element === el && s.kind === k.key)
      const owned = all.filter((s) => ownedSkillIds.value.has(s.id)).length
      return { ...k, owned, total: all.length }
    }),
  })),
)

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
/* G66 技能元素×类型覆盖矩阵 */
.skill-matrix { display: flex; flex-direction: column; gap: 4rpx; margin-bottom: 16rpx; }
.sm-row { display: flex; flex-direction: row; gap: 4rpx; }
.sm-head { flex: 1; text-align: center; font-size: 22rpx; font-weight: 600; padding: 6rpx 0; }
.sm-cell {
  flex: 1;
  text-align: center;
  font-size: 22rpx;
  padding: 10rpx 0;
  border-radius: 8rpx;
  background: var(--panel-2);
  color: var(--dim);
}
.sm-cell.covered { background: rgba(126, 231, 135, 0.14); color: var(--ok); }
/* 条目分隔线走共享的 .rxn（styles/common.css） */
/* R67 主动/被动标签配色 */
.kind-tag { font-size: 22rpx; }
.kind-active { color: var(--fire); }
.kind-passive { color: var(--ice); }
</style>