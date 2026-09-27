<template>
  <view class="page-root">
    <view class="pad">
      <!-- 技能槽配置 -->
      <view class="card">
        <text class="card-title">出战技能（最多 4 个主动槽）</text>
        <text class="muted" style="display: block; margin-bottom: 16rpx">
          技能携带的元素决定你能打出哪些反应链。凑齐 2 种以上元素才能触发大部分反应。
        </text>
        <view class="slots">
          <view
            v-for="i in 4"
            :key="i"
            class="slot"
            :class="{ empty: !selected[i - 1] }"
          >
            <text v-if="selected[i - 1]" class="tag" :class="'tag-' + skillOf(i - 1)?.element">
              {{ elementName(skillOf(i - 1)!.element) }}
            </text>
            <text v-if="selected[i - 1]" class="slot-name">{{ skillOf(i - 1)!.name }}</text>
            <text v-else class="dim">空槽</text>
            <text
              v-if="selected[i - 1]"
              class="dim"
              @click="clearSlot(i - 1)"
            >移除</text>
          </view>
        </view>
      </view>

      <!-- 技能列表 -->
      <view class="card">
        <text class="card-title">技能库</text>
        <view v-for="s in allSkills" :key="s.id" class="skill-row">
          <view class="flex-1">
            <view class="row" style="gap: 12rpx">
              <text class="tag" :class="'tag-' + s.element">{{ elementName(s.element) }}</text>
              <text class="skill-name">{{ s.name }}</text>
              <text class="dim">{{ familyName(s.family) }}</text>
            </view>
            <text class="muted" style="display: block; margin-top: 6rpx">{{ s.descr }}</text>
            <text class="dim" style="display: block; margin-top: 4rpx">
              伤害 {{ s.base_damage }} · 热量 {{ s.heat_cost }} · 冷却 {{ s.cooldown_ms }}ms
              <text v-if="s.pierce"> · 穿透 {{ s.pierce }}</text>
              <text v-if="s.aoe_radius"> · 溅射 {{ s.aoe_radius }}</text>
            </text>
          </view>
          <view class="skill-act">
            <text v-if="isEquipped(s.id)" class="dim">已装备</text>
            <text v-else class="link" @click="toggleEquip(s.id)">装备</text>
          </view>
        </view>
      </view>

      <!-- 装备 -->
      <view class="card">
        <text class="card-title">装备</text>
        <text class="muted" style="display: block; margin-bottom: 16rpx">
          装备带元素标签。与技能<b>同系</b>给反应伤害大幅加成，异系只给少量直接伤害 ——
          所以装备和技能要成套配。
        </text>
        <view v-for="e in equipment" :key="e.id" class="equip-row">
          <text class="tag" :class="'tag-' + e.element">{{ elementName(e.element as Element) }}</text>
          <view class="flex-1">
            <text class="skill-name">{{ e.name }}</text>
            <text class="dim" style="display: block">{{ e.descr }}</text>
          </view>
          <text class="dim">{{ slotName(e.slot) }} · T{{ e.tier }}</text>
        </view>
      </view>

      <!-- 宝石 -->
      <view class="card">
        <text class="card-title">宝石</text>
        <view class="gem-row">
          <view v-for="g in gems" :key="g.id" class="gem-chip">
            <text class="gem-name">{{ g.name }}</text>
            <text class="dim">{{ g.descr }}</text>
          </view>
        </view>
        <text class="muted" style="display: block; margin-top: 12rpx">
          品质分灰/蓝/紫/红/金五档，词条可洗练（消耗钻石）。
          注意「元素石」提高元素系数 —— 它是反应伤害的核心，比堆攻击力更划算。
        </text>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useGameStore } from '@/store/game'
import { ELEMENT_NAME, type Element } from '@/game/elements'
import type { SkillDef } from '@/game/types'

const store = useGameStore()
const selected = ref<number[]>([0, 0, 0, 0])
/** 保存中：保存期间禁用操作，避免连点产生竞态 */
const saving = ref(false)

const allSkills = computed<SkillDef[]>(() => {
  const base = store.config?.skills ?? []
  const composite = store.config?.composite_skills ?? []
  return [...base, ...composite]
})

const equipment = computed<any[]>(() => store.config?.equipment ?? [])
const gems = computed<any[]>(() => store.config?.gems ?? [])

function elementName(e: Element): string {
  return ELEMENT_NAME[e] ?? e
}

function familyName(f: string): string {
  const m: Record<string, string> = {
    flame: '焰',
    frost: '冰',
    volt: '电',
    kinetic: '物理',
    light: '光',
    blight: '毒',
    drone: '无人机',
    engineering: '工程',
  }
  return m[f] ?? f
}

function slotName(s: string): string {
  const m: Record<string, string> = {
    weapon: '武器',
    helmet: '头盔',
    armor: '护甲',
    gloves: '手套',
    boots: '靴子',
    charm: '挂件',
  }
  return m[s] ?? s
}

function skillOf(slotIdx: number): SkillDef | undefined {
  const id = selected.value[slotIdx]
  if (!id) return undefined
  return allSkills.value.find((s) => s.id === id)
}

function isEquipped(id: number): boolean {
  return selected.value.includes(id)
}

/**
 * 装备/移除一个技能。
 *
 * ⚠️ 必须落服务端。槽位参与回放哈希计算，只存本地的话
 * 验真方拿不到同一份槽位，I-6 失效。因此这里先调接口，
 * 成功才更新 UI；失败必须提示，不能让 UI 显示"已装备"但实际没生效。
 */
async function toggleEquip(id: number) {
  const s = allSkills.value.find((x) => x.id === id)
  if (!s) return
  if (s.kind !== 'active') {
    uni.showToast({ title: '被动技能不占主动槽', icon: 'none' })
    return
  }
  const next = [...selected.value]
  const existing = next.indexOf(id)
  if (existing >= 0) {
    next[existing] = 0
  } else {
    const empty = next.indexOf(0)
    if (empty < 0) {
      uni.showToast({ title: '槽位已满，先移除一个', icon: 'none' })
      return
    }
    next[empty] = id
  }
  saving.value = true
  const ok = await store.persistLoadout(next)
  saving.value = false
  if (!ok) {
    uni.showToast({ title: '保存失败，出战配置未变更', icon: 'none' })
    return
  }
  selected.value = [...store.loadout]
}

async function clearSlot(idx: number) {
  const next = [...selected.value]
  if (!next[idx]) return
  next[idx] = 0
  saving.value = true
  const ok = await store.persistLoadout(next)
  saving.value = false
  if (!ok) {
    uni.showToast({ title: '保存失败，出战配置未变更', icon: 'none' })
    return
  }
  selected.value = [...store.loadout]
}

onMounted(async () => {
  if (!store.loggedIn) await store.login()
  // 槽位以服务端为准
  await store.loadLoadout()
  selected.value = [...store.loadout]
  while (selected.value.length < 4) selected.value.push(0)
})
</script>

<style scoped>
.slots {
  display: flex;
  flex-direction: row;
  gap: 12rpx;
}

.slot {
  flex: 1;
  min-height: 120rpx;
  border-radius: 12rpx;
  border: 1rpx solid var(--border);
  background: var(--panel-2);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 12rpx 6rpx;
  gap: 6rpx;
}

.slot.empty {
  opacity: 0.4;
}

.slot-name {
  font-size: 22rpx;
}

.skill-row {
  display: flex;
  flex-direction: row;
  align-items: center;
  padding: 18rpx 0;
  border-bottom: 1rpx solid var(--border);
}

.skill-row:last-child {
  border-bottom: none;
}

.skill-name {
  font-size: 26rpx;
  font-weight: 600;
}

.skill-act {
  margin-left: 16rpx;
}

.link {
  font-size: 24rpx;
  color: var(--fire);
  padding: 8rpx 20rpx;
  border-radius: 8rpx;
  background: rgba(255, 107, 53, 0.12);
}

.equip-row {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 16rpx;
  padding: 16rpx 0;
  border-bottom: 1rpx solid var(--border);
}

.equip-row:last-child {
  border-bottom: none;
}

.gem-row {
  display: flex;
  flex-direction: row;
  flex-wrap: wrap;
  gap: 12rpx;
}

.gem-chip {
  width: calc(50% - 6rpx);
  background: var(--panel-2);
  border-radius: 10rpx;
  padding: 14rpx 16rpx;
}

.gem-name {
  font-size: 24rpx;
  font-weight: 600;
  display: block;
}
</style>
