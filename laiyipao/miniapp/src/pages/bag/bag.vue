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

      <!-- 熔炼：两个已拥有的材料技能 → 高阶技能。
           材料会被消耗；等级门槛由服务端判定（产物 unlock_level 30..62）。
           未集齐材料的配方**不给按钮**（约束 19：不给假按钮）。 -->
      <view v-if="recipes.length" class="card">
        <text class="card-title">熔炼</text>
        <text class="muted" style="display: block; margin-bottom: 16rpx">
          两个材料技能熔炼成高阶技能，材料会被消耗。
          产物需要等级达标（由服务端判定，不足时会提示所需等级）。
        </text>
        <view v-for="r in recipes" :key="r.output" class="skill-row">
          <view class="flex-1">
            <view class="row" style="gap: 8rpx">
              <text class="skill-name">{{ skillNameOf(r.a) }}</text>
              <text class="dim">+</text>
              <text class="skill-name">{{ skillNameOf(r.b) }}</text>
              <text class="dim">→</text>
              <text class="skill-name">{{ skillNameOf(r.output) }}</text>
              <text class="dim">{{ r.out_tier }} 阶</text>
            </view>
          </view>
          <view class="skill-act">
            <text v-if="missingFor(r).length" class="dim">
              缺 {{ missingFor(r).map(skillNameOf).join('、') }}
            </text>
            <text v-else-if="synthesizing === r.output" class="dim">熔炼中…</text>
            <text v-else class="link" @click="doSynthesize(r)">熔炼</text>
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
          基础技能按等级自动解锁；高阶技能靠熔炼（见上）。
          标「未解锁」的不能装备或升级。
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
              <!-- 未解锁：不给升级/装备入口。
                   服务端对未解锁技能一律拒（SaveLoadout → 400「技能 N 未解锁」，
                   routes_e2e_test 有断言；UpgradeSkill 同理回「未拥有」）。
                   此前这里对全表技能一律渲染「装备」，玩家点未解锁的那些
                   只会看到一句「保存失败，出战配置未变更」—— 看起来像故障。 -->
              <text v-if="!isOwned(s.id)" class="dim">未解锁</text>
              <template v-else>
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
              </template>
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
        <!-- 品质表（第 150 轮）：改读服务端下发的 gem_qualities。
             此前这里把「灰/蓝/紫/红/金」与倍率**硬编码**在文案里，
             而服务端 content.go 的 GemQualities 用的是 gray/blue/... + 真实倍率。
             两份各自自洽 —— 运营改品质表时这段文案不会跟着变，
             而它恰恰是玩家判断「这颗宝石值不值」的唯一依据。 -->
        <view v-if="gemQualities.length" style="margin-top: 16rpx">
          <text class="dim" style="display: block; margin-bottom: 8rpx">品质档位</text>
          <view class="gem-row">
            <view v-for="q in gemQualities" :key="q.name" class="gem-chip">
              <text class="gem-name">{{ q.name }} · ×{{ (q.mult / 1000).toFixed(2) }}</text>
              <text class="dim">{{ q.affix_count }} 条词条</text>
            </view>
          </view>
        </view>
        <text class="muted" style="display: block; margin-top: 12rpx">
          宝石实例可以在商城买到（随机品质与词条），洗练消耗洗练券重掷词条、品质不变。
          词条会计入你的攻击者派生属性。
        </text>

        <!-- 我持有的宝石（第 150 轮）。此前这里只有一段「拿不到宝石」的说明 ——
             `user_gems` 只有服务端内部读取，玩家在界面上看不到自己有什么。 -->
        <view style="margin-top: 16rpx">
          <text class="dim" style="display: block; margin-bottom: 8rpx">我的宝石</text>
          <text v-if="ownedGems.length === 0" class="dim">还没有宝石，去商城买一颗。</text>
          <view
            v-for="g in ownedGems"
            :key="g.id"
            class="equip-row owned-gem"
          >
            <view class="flex-1">
              <text class="skill-name">
                {{ g.name }}
                <text class="dim">· {{ qualityName(g.quality) }}</text>
              </text>
              <text class="dim" style="display: block">{{ affixText(g.affixes) }}</text>
            </view>
            <view class="skill-act">
              <text v-if="washingId === g.id" class="dim">洗练中…</text>
              <text
                v-else
                class="link"
                @click="doWash(g)"
              >洗练</text>
            </view>
          </view>
        </view>
      </view>

      <!-- 合成配方（第 150 轮）：recipes 此前从不下发到客户端，
           玩家看到「合成技能」却完全不知道它由哪两个技能合成。
           配方是内容表的一部分，缺了它就是一个悬空的词条。 -->
      <view v-if="recipes.length" class="card">
        <text class="card-title">合成配方</text>
        <text class="muted" style="display: block; margin-bottom: 12rpx">
          合成技能由两个基础技能组合而来；配方来自服务端，与实际可合成的组合逐字一致。
        </text>
        <view v-for="r in recipes" :key="r.output" class="recipe-row">
          <text class="dim">{{ skillNameOf(r.a) }} + {{ skillNameOf(r.b) }}</text>
          <text class="dim">→</text>
          <text class="skill-name">{{ skillNameOf(r.output) }}</text>
          <text class="dim">T{{ r.out_tier }}</text>
        </view>
      </view>

      <!-- 皮肤（第 150 轮） -->
      <view v-if="skins.length" class="card">
        <text class="card-title">皮肤</text>
        <text class="muted" style="display: block; margin-bottom: 12rpx">
          皮肤是**机制词条**而非数值加成。
          ⚠️ 但当前版本还没有任何途径获得它们（无掉落 / 无兑换），
          词条也**未接入战斗结算** —— 所以这里只作图鉴展示。
        </text>
        <view v-for="s in skins" :key="s.id" class="equip-row">
          <view class="flex-1">
            <text class="skill-name">{{ s.name }}</text>
            <text class="dim" style="display: block">{{ s.descr }}</text>
          </view>
          <text class="dim">{{ s.rarity }}</text>
        </view>
      </view>

      <!-- 道具（第 150 轮）：/wallet 的 tokens 首次有了消费方。 -->
      <view class="card">
        <text class="card-title">道具</text>
        <view v-for="t in tokenRows" :key="t.kind" class="equip-row">
          <view class="flex-1">
            <text class="skill-name">{{ t.label }}</text>
            <text class="dim" style="display: block">{{ t.status }}</text>
          </view>
          <text class="dim">×{{ t.count }}</text>
        </view>
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
import type { OwnedGem } from '@/api/client'
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

/**
 * 宝石品质档位（服务端 /config 的 `gem_qualities`）。
 *
 * ⚠️ 此前这段是**硬编码文案**（"灰/蓝/紫/红/金"），而服务端 content.go 的
 * `GemQualities` 用的是 gray/blue/purple/red/gold + 真实倍率。
 * 两份各自自洽：运营改品质表，玩家看到的档位说明不会变 ——
 * 而这恰恰是玩家判断「这颗宝石值不值」的唯一依据。
 */
const gemQualities = computed<{ name: string; affix_count: number; mult: number }[]>(
  () => (store.config?.gem_qualities ?? []) as any[],
)

/** 合成配方（`recipes`）：A + B → output(out_tier)。 */
const recipes = computed<{ output: number; a: number; b: number; out_tier: number }[]>(
  () => store.config?.recipes ?? [],
)

/** 技能名查表：配方两端都是技能 id，界面要显示名字。 */
function skillNameOf(id: number): string {
  return allSkills.value.find((s) => s.id === id)?.name ?? `#${id}`
}

const skins = computed<any[]>(() => store.config?.skins ?? [])

/** 品质中文名；未知品质原样返回（不要吞掉后端新增的档位）。 */
const QUALITY_CN: Record<string, string> = {
  gray: '灰',
  blue: '蓝',
  purple: '紫',
  red: '红',
  gold: '金',
}

const QUALITY_AFFIX_CN: Record<string, string> = {
  attack: '攻击',
  skill_damage: '技能伤害',
  armor: '护甲',
  crit: '暴击',
  element_coef: '元素系数',
  element_cap: '元素上限',
  reaction_mult: '反应倍率',
  heat_cap: '热量上限',
}

function qualityName(q: string): string {
  return QUALITY_CN[q] ?? q
}

/** 词条渲染成「暴击 +42 · 护甲 +18」。未知词条名原样显示。 */
function affixText(affixes: { affix: string; value: number }[]): string {
  if (!affixes || affixes.length === 0) return '（无词条）'
  return affixes.map((a) => `${QUALITY_AFFIX_CN[a.affix] ?? a.affix} +${a.value}`).join(' · ')
}

/** 我持有的宝石实例（第 150 轮新增读取面）。 */
const ownedGems = ref<OwnedGem[]>([])
/** 正在洗练的宝石 id；非 0 时该行禁用，防连点连扣券。 */
const washingId = ref(0)

async function loadGems() {
  try {
    const r = await api.fetchGems()
    ownedGems.value = r.gems ?? []
  } catch {
    // 拉不到就当没有：这一段是附加信息，不该让整页白屏。
    ownedGems.value = []
  }
}

/**
 * 洗练一颗宝石。
 *
 * 失败不静默：券是玩家花钱买的，扣了却没生效是最容易被当成 bug 报上来的状态，
 * 所以要么 toast 原因、要么明确成功。
 */
async function doWash(g: OwnedGem) {
  if (washingId.value !== 0) return
  if ((store.wallet.tokens?.gem_wash_token ?? 0) < 1) {
    uni.showToast({ title: '没有洗练券', icon: 'none' })
    return
  }
  washingId.value = g.id
  try {
    const r = await api.washGem(g.id)
    const i = ownedGems.value.findIndex((x) => x.id === g.id)
    if (i >= 0 && r.gem) ownedGems.value.splice(i, 1, r.gem)
    // 余额以服务端为准：本地那份在这笔扣减前是旧的。
    await store.refreshProfile()
    uni.showToast({ title: '洗练完成', icon: 'none' })
  } catch (e) {
    uni.showToast({ title: (e as Error).message, icon: 'none' })
  } finally {
    washingId.value = 0
  }
}

/**
 * 道具清单（`/wallet` 的 `tokens`，第 150 轮才有消费方）。
 *
 * 每条都写明**现在到底能不能用**。这不是文案洁癖：
 * `revive_token` 与 `gem_wash_token` 一度买得到却没有消费端点，
 * 玩家在商城买了之后在任何地方都找不到自己的道具，
 * 那是最容易被当成 bug 报上来的一种状态。
 *
 * 第 150 轮：复活币接上了消费端点（开战扣 1 枚 + 本关基础血量 +50%），
 * 从下架恢复；洗练石仍无消费端点（且玩家拿不到宝石实例），保持下架并在
 * 这里写明。界面文案跟着服务端状态走 —— 「已下架」与「可用」不能各说各话。
 */
const tokenRows = computed(() => {
  const tokens = store.wallet.tokens ?? {}
  const catalog: { kind: string; label: string; status: string }[] = [
    {
      kind: 'mastery_reset',
      label: '专精重铸券',
      status: '清空全部已点亮的专精节点，点数不变 → 专精页点「重铸」使用。',
    },
    {
      kind: 'revive_token',
      label: '复活币',
      status: '开战时自动消耗 1 枚，本关基础血量 +50%（扣币时机与体力一致）。',
    },
    {
      kind: 'gem_wash_token',
      label: '宝石洗练券',
      // 2026-10-21（P2-6）：这段文案此前说「尚未接入、已从商城下架」，
      // 而服务端的事实**正好相反** ——
      //   - `POST /me/gems/:id/wash` 已接通（server/internal/service/gems.go）
      //   - 商城商品 `shop_gem_wash` 是**在售**的（seed.go 的 delisted=false
      //     → enabled=true）
      //   - 宝石本体 `shop_gem` 也在售
      // 一句与事实相反的文案比没有文案更糟：玩家看着「用不了」就不买了。
      status: '在「我的宝石」里选中一颗宝石，点「洗练」消耗 1 枚，重掷词条（品质不变）。',
    },
  ]
  return catalog.map((c) => ({ ...c, count: tokens[c.kind] ?? 0 }))
})

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

/**
 * 玩家是否已解锁该技能 —— 判据是服务端 `build.skills` 里有没有这一行。
 *
 * 为什么不写成 `levelOf() > 0`：那是同一个事实的另一种说法，而这里要表达的是
 * **能不能操作**（装备与升级都要过服务端的解锁校验），读键存在性更贴题意。
 *
 * build 未就绪（/me 还没回来，或拉取失败）时**不假装拥有**：
 * 那种状态下点任何按钮都只会被服务端拒（400），先按未解锁渲染更诚实。
 */
function isOwned(skillId: number): boolean {
  const bag = store.build?.skills
  return !!bag && Object.prototype.hasOwnProperty.call(bag, String(skillId))
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
  if (!isOwned(skillId)) {
    // 未解锁的技能服务端必拒（UpgradeSkill → 「技能 N 未拥有」），
    // 本地先拦下：白跑一趟网络还只换来一句「升级失败」。
    uni.showToast({ title: '技能未解锁', icon: 'none' })
    return
  }
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

/** 熔炼中的产物 id（防连点；null = 空闲）。 */
const synthesizing = ref<number | null>(null)

/** 配方缺哪些材料（空数组 = 两个材料都拥有，可以熔炼）。 */
function missingFor(r: { a: number; b: number }): number[] {
  const out: number[] = []
  if (!isOwned(r.a)) out.push(r.a)
  if (!isOwned(r.b)) out.push(r.b)
  return out
}

/**
 * 熔炼两个材料技能。
 *
 * 本地只拦「材料不齐」这一层（约束 19：不给假按钮）——
 * 配方存在性、等级门槛、产物是否已拥有，全部由服务端判定
 * （`SynthesizeSkills`，材料会被消耗）。
 */
async function doSynthesize(r: { a: number; b: number; output: number }): Promise<void> {
  if (synthesizing.value !== null) return // 防连点
  if (missingFor(r).length) {
    uni.showToast({ title: '材料未集齐', icon: 'none' })
    return
  }
  synthesizing.value = r.output
  try {
    const res = await api.synthesizeSkills(r.a, r.b)
    // 材料已删、产物已发放、槽位可能被清空 —— build 快照必须重拉
    await store.refreshProfile()
    uni.showToast({ title: `熔炼成功：${skillNameOf(res.output)}`, icon: 'none' })
  } catch (e: any) {
    uni.showToast({ title: e?.message ?? '熔炼失败', icon: 'none' })
  } finally {
    synthesizing.value = null
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
  if (!isOwned(id)) {
    // 服务端对未解锁技能回 400「技能 N 未解锁」。这里先拦：
    // 否则 UI 提示的是「保存失败，出战配置未变更」—— 玩家会当成故障，
    // 而真实原因是他本来就没资格装备它（routes_e2e_test 有 400 断言）。
    uni.showToast({ title: '技能未解锁', icon: 'none' })
    return
  }
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
  // 第 145 轮：修前只 loadLoadout 不 refreshProfile —— 已登录态直接进背包时
  // （未先经过 index/battle/me）store.build 为 null 或陈旧，levelOf 恒 0、
  // 升级花费恒按 1 级算，与玩家真实养成不符。两者并发拉取，互不阻塞。
  // 宝石（第 150 轮）与它们并发：它只影响下面那块「我的宝石」，不参与槽位逻辑。
  await Promise.all([store.loadLoadout(), store.refreshProfile(), loadGems()])
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

.recipe-row {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 10rpx;
  padding: 8rpx 0;
}
</style>
