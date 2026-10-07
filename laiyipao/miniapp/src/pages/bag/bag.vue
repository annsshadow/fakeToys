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
        <text class="muted" style="display: block; margin-bottom: 16rpx">
          等级只提升**伤害**，不影响热量与冷却。
          每级 +<text v-if="skillRules">{{ skillRules.coef_permille }}‰</text>伤害，满级
          <text v-if="skillRules">{{ skillRules.max_level }}</text> 级。
        </text>
        <view v-for="s in allSkills" :key="s.id" class="skill-row">
          <view class="flex-1">
            <view class="row" style="gap: 12rpx">
              <text class="tag" :class="'tag-' + s.element">{{ elementName(s.element) }}</text>
              <text class="skill-name">{{ s.name }}</text>
              <text class="dim">{{ familyName(s.family) }}</text>
              <text v-if="levelOf(s.id) > 1" class="dim">Lv.{{ levelOf(s.id) }}</text>
            </view>
            <text class="muted" style="display: block; margin-top: 6rpx">{{ s.descr }}</text>
            <text class="dim" style="display: block; margin-top: 4rpx">
              伤害 {{ damageAt(s) }} · 热量 {{ s.heat_cost }} · 冷却 {{ s.cooldown_ms }}ms
              <text v-if="s.pierce"> · 穿透 {{ s.pierce }}</text>
              <text v-if="s.aoe_radius"> · 溅射 {{ s.aoe_radius }}</text>
            </text>
          </view>
          <view class="skill-act">
            <view style="display: flex; flex-direction: column; align-items: flex-end; gap: 8rpx">
              <text v-if="isMaxLevel(s.id)" class="dim">已满级</text>
              <text
                v-else-if="!canAfford(s.id)"
                class="dim"
              >金币不足（{{ upgradeCost(s.id) }}）</text>
              <text
                v-else
                class="link"
                @click="doUpgrade(s.id)"
              >升级 {{ upgradeCost(s.id) }} 金币</text>
              <text v-if="isEquipped(s.id)" class="dim">已装备</text>
              <text v-else class="link" @click="toggleEquip(s.id)">装备</text>
            </view>
          </view>
        </view>
      </view>

      <!-- 装备 -->
      <view class="card">
        <text class="card-title">装备</text>
        <text class="muted" style="display: block; margin-bottom: 16rpx">
          装备目前只提供**护甲**（减漏怪造成的伤害）与**通用增益**。
          它的元素标签**当前不参与任何结算** ——
          按标签给加成这个机制没有实现，所以这里不再宣传它。
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
import * as api from '@/api/client'
import { ELEMENT_NAME, type Element } from '@/game/elements'
import { skillBaseDamageAtLevel } from '@/game/skill'
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

// ── 技能升级 ────────────────────────────────────────────────
//
// 端点 `POST /me/skills/:id/upgrade` 从 Round 25 就在了，
// 但**客户端一直没有入口** —— 那是个只有后端的装饰功能，玩家点不到。
// 本项目反复吃过「只写不读」的亏，这次是自己刚犯的。

const upgrading = ref<number | null>(null)

/** 升级规则来自 /config，与结算逻辑同源（不硬编码）。 */
const skillRules = computed(() => (store.config as any)?.skill_rules ?? null)

/** 玩家拥有该技能时的等级；未拥有返回 0。 */
function levelOf(skillId: number): number {
  const bag = store.build?.skills
  if (!bag) return 0
  const row = bag[String(skillId)]
  return typeof row?.level === 'number' ? row.level : 0
}

/** 升到下一级的金币花费（线性：基数 × 当前等级）。 */
function upgradeCost(skillId: number): number {
  const r = skillRules.value
  if (!r) return 0
  return r.base_cost * Math.max(levelOf(skillId), 1)
}

function isMaxLevel(skillId: number): boolean {
  const r = skillRules.value
  if (!r) return true
  return levelOf(skillId) >= r.max_level
}

function canAfford(skillId: number): boolean {
  return store.wallet.coin >= upgradeCost(skillId)
}

/**
 * 展示用的伤害（含等级加成）。
 *
 * ⚠️ 用与服务端 `SkillLevelCoef` 相同的公式现算，而不是读后端给的数 ——
 * 后端不下发「升级后的伤害」，而这里要显示的是**当前等级下**的伤害。
 * 公式在 `game/skill.ts` 的 `skillBaseDamageAtLevel` 里，两处一致。
 */
/**
 * 展示用的伤害（含等级加成）。
 *
 * 返回 `number` 而不是 string：`skillBaseDamageAtLevel` 返回 bigint，
 * 而模板 `{{ damageAt(s) }}` 把它当数值显示。这里显式转成 number，
 * **不能**用 `.toString()` —— 那会把返回类型悄悄变成 string
 * （声明写的是 number，于是 vue-tsc 报 TS2322）。
 *
 * bigint → number 在伤害这个量级（1e4~1e6）不会有精度问题；
 * 真要严格的话应该保留 bigint 并在模板里显式格式化，
 * 但那会改动模板与测试，超出本轮范围。
 */
function damageAt(s: SkillDef): number {
  const r = skillRules.value
  if (!r) return s.base_damage
  return Number(
    skillBaseDamageAtLevel(
      { maxLevel: r.max_level, coefPermille: r.coef_permille, baseCost: r.base_cost },
      BigInt(s.base_damage),
      levelOf(s.id),
    ),
  )
}

async function doUpgrade(skillId: number): Promise<void> {
  if (upgrading.value !== null) return // 防连点
  upgrading.value = skillId
  try {
    const r = await api.upgradeSkill(skillId)
    // 响应里带 wallet，直接更新，不必再拉一次
    store.wallet = { ...store.wallet, ...r.wallet }
    // 等级来自 build 快照，必须重新拉 —— 本地缓存的还是旧等级
    await store.refreshProfile()
    uni.showToast({ title: `升级成功 Lv.${r.level}`, icon: 'none' })
  } catch (e: any) {
    uni.showToast({ title: e?.message ?? '升级失败', icon: 'none' })
  } finally {
    upgrading.value = null
  }
}

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
  // 槽位以服务端为准；技能等级/升级花费同以服务端 build（owned skills 快照）为准。
  // 第 145 轮：修前只 loadLoadout 不 refreshProfile —— 已登录态直接进背包时
  // （未先经过 index/battle/me）store.build 为 null 或陈旧，levelOf 恒 0、
  // 升级花费恒按 1 级算，与玩家真实养成不符。两者并发拉取，互不阻塞。
  await Promise.all([store.loadLoadout(), store.refreshProfile()])
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
