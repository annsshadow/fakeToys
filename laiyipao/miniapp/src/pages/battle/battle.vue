<template>
  <view class="page-root battle-page">
    <!-- 画布：整个战斗画面 -->
    <canvas
      id="battle-canvas"
      canvas-id="battle-canvas"
      type="2d"
      class="battle-canvas"
      :style="{ width: canvasW + 'px', height: canvasH + 'px' }"
    />

    <!-- 顶部信息层 -->
    <view class="overlay-top">
      <view class="hp-box">
        <text class="hp-label">防线</text>
        <text class="hp-value mono">{{ hpText }}</text>
        <view class="hp-bar">
          <view class="hp-fill" :style="{ width: hpPct + '%' }" />
        </view>
      </view>
      <view class="top-right">
        <text class="wave-text">波次 {{ waveText }}</text>
        <text class="kill-text">击杀 {{ engine?.kills ?? 0 }}</text>
      </view>
    </view>

    <!-- 选牌界面（I-2） -->
    <view v-if="showCards" class="card-overlay">
      <view class="card-header">
        <text class="card-title">选择一张卡</text>
        <text class="muted">弃牌 {{ deck.discardsLeft }} 次 · 弃牌返还 1 热量</text>
      </view>
      <view class="cards">
        <view
          v-for="c in hand"
          :key="c.id"
          class="hand-card"
          :class="[`rarity-${c.rarity}`, `kind-${c.kind}`]"
          @click="takeCard(c.id)"
        >
          <text class="hand-kind">{{ kindLabel(c.kind) }}</text>
          <text class="hand-name">{{ c.name }}</text>
          <text class="hand-descr">{{ c.descr }}</text>
          <view class="hand-foot">
            <text
              v-if="c.element"
              class="tag"
              :class="'tag-' + c.element"
            >{{ elementName(c.element) }}</text>
            <text class="dim" @click.stop="discardCard(c.id)">弃</text>
          </view>
        </view>
      </view>
      <view class="btn" @click="skipCards">跳过（不消耗弃牌次数）</view>
    </view>

    <!-- 结算界面 -->
    <view v-if="settleResult" class="card-overlay">
      <text class="result-title">{{ settleResult.win ? '通关' : '防线失守' }}</text>
      <view class="stars-big">
        <text
          v-for="i in 3"
          :key="i"
          class="star-big"
          :class="{ off: i > settleResult.stars }"
        >★</text>
      </view>
      <view class="result-stats">
        <view class="rstat"><text class="muted">击杀</text><text class="mono">{{ settleResult.kills }}</text></view>
        <view class="rstat"><text class="muted">漏怪</text><text class="mono">{{ settleResult.leaked }}</text></view>
        <view class="rstat"><text class="muted">反应</text><text class="mono">{{ settleResult.reactions }}</text></view>
        <view class="rstat"><text class="muted">分数</text><text class="mono">{{ settleResult.score }}</text></view>
      </view>
      <view v-if="settleResult.clamped" class="clamp-note">
        服务端已修正本次结算：{{ settleResult.clamp_note }}
      </view>
      <view class="reward-row">
        <text v-for="(v, k) in settleResult.loot" :key="k" class="reward">
          {{ currencyName(String(k)) }} +{{ v }}
        </text>
      </view>
      <text class="hash-note">回放哈希 {{ settleResult.replay_hash }}（可验真）</text>
      <view class="btn btn-primary" style="margin-top: 20rpx" @click="backToStage">返回关卡</view>
    </view>

    <!-- 渲染模式诊断：H5 下 canvas 获取方式与小程序不同，需要能看出走了哪条路径 -->
    <view class="debug-tag">
      {{ renderMode === 'raf' ? '渲染: 帧循环' : renderMode === 'fallback' ? '渲染: 降级(无canvas)' : '渲染: 初始化中' }}
    </view>

    <!-- 启动失败提示 -->
    <view v-if="startError" class="card-overlay">
      <text class="result-title" style="color: var(--bad)">无法开始战斗</text>
      <text class="muted">{{ startError }}</text>
      <view class="btn" style="margin-top: 24rpx" @click="backToStage">返回</view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, computed, getCurrentInstance } from 'vue'
import { onLoad, onUnload } from '@dcloudio/uni-app'
import { useGameStore } from '@/store/game'
import * as api from '@/api/client'
import { BattleEngine, TICK_MS } from '@/game/engine'
import { BattleRenderer } from '@/render/canvas'
import { ELEMENT_NAME, type Element } from '@/game/elements'
import type { Card, EquippedSkill } from '@/game/heatmap'
import type { GeneratedLevel } from '@/game/types'

const store = useGameStore()
const inst = getCurrentInstance()

const canvasW = ref(375)
const canvasH = ref(667)
const engine = ref<BattleEngine | null>(null)
const renderer = ref<BattleRenderer | null>(null)
/** 渲染模式：raf=帧循环已启动，fallback=无 canvas 降级，pending=尚未决定 */
const renderMode = ref<'pending' | 'raf' | 'fallback'>('pending')
const startError = ref('')
const settleResult = ref<any>(null)
const hand = ref<Card[]>([])
const tokenId = ref(0)
const levelId = ref(1)

let loopStarted = false
let timerId: ReturnType<typeof setInterval> | null = null

const hpPct = computed(() => {
  if (!engine.value) return 100
  return Math.max(0, (Number(engine.value.baseHp) / Number(engine.value.baseHpMax)) * 100)
})
const hpText = computed(() => {
  if (!engine.value) return '—'
  return `${Number(engine.value.baseHp)} / ${Number(engine.value.baseHpMax)}`
})
const waveText = computed(() => {
  if (!engine.value) return '—'
  const total = engine.value.cfg.level.wave_count
  return `${engine.value.waveIndex + 1} / ${total}`
})
const showCards = computed(() => engine.value?.phase === 'card_select' && !settleResult.value)
const deck = computed(() => engine.value?.deck ?? { discardsLeft: 0, size: 0 } as any)

function elementName(e: Element): string {
  return ELEMENT_NAME[e]
}

function kindLabel(k: string): string {
  return k === 'skill' ? '技能' : k === 'attribute' ? '属性' : '机制'
}

function currencyName(k: string): string {
  return k === 'coin' ? '金币' : k === 'gem' ? '钻石' : k === 'keys' ? '钥匙' : k === 'energy' ? '体力' : k
}

/**
 * 取可用于绘制的 canvas 节点。
 *
 * ⚠️ 关键：H5 下 `<canvas type="2d">` 会被编译成 `<uni-canvas>` 自定义元素，
 * 它**没有 getContext 方法**。必须走 uni.createSelectorQuery().fields({node:true})，
 * 它返回平台内部持有的真实 canvas 节点。
 *
 * 之前这里先试 querySelector 拿 DOM 元素 —— 那在 H5 下会拿到无用的
 * `<uni-canvas>`，导致渲染层静默失效（战斗逻辑在跑但画面全黑）。
 * 因此统一只用 createSelectorQuery，mp-weixin 与 H5 行为一致。
 */
function getCanvas(): Promise<any | null> {
  const proxy = (inst as any)?.proxy
  return new Promise((resolve) => {
    uni
      .createSelectorQuery()
      .in(proxy)
      .select('#battle-canvas')
      .fields({ node: true, size: true }, (res: any) => {
        // 不同平台 / uni 版本下 node 可能是真实 canvas，也可能是包裹它的
        // 自定义元素（H5 的 <uni-canvas>）。后者没有 getContext，
        // 必须向下找到内部真正的 <canvas>，否则会静默画到错误元素上。
        let target: any = res?.node ?? null
        if (target && typeof target.getContext !== 'function') {
          target = target.querySelector?.('canvas') ?? null
        }
        if (target && typeof target.getContext === 'function') {
          const dpr = uni.getSystemInfoSync().pixelRatio || 1
          const cssW = res?.width ?? canvasW.value
          const cssH = res?.height ?? canvasH.value
          target.width = Math.floor(cssW * dpr)
          target.height = Math.floor(cssH * dpr)
          resolve(target)
        } else {
          console.warn('[来一炮] 未取得可绘制 canvas，回退到无渲染模式')
          resolve(null)
        }
      })
      .exec()
  })
}

async function setup() {
  const info = uni.getSystemInfoSync()
  canvasW.value = info.windowWidth
  canvasH.value = info.windowHeight

  // 必须等登录完成再申请凭证：否则可能带着上一个账号的 token 去开局，
  // 服务端会按那个账号扣体力，出现"钱包体力充足但开不了局"。
  if (!store.loggedIn) {
    const ok = await store.login()
    if (!ok) {
      startError.value = store.loginError || '登录失败'
      return
    }
  }

  // 启动战斗：向服务端申请一次性凭证（扣体力 + 服务端种子）
  try {
    const bt = await api.startBattle(levelId.value)
    tokenId.value = bt.token_id
    const level = bt.level as GeneratedLevel
    const skills = store.skillMap

    /**
     * 技能槽位**必须用服务端下发的 slot**，不能用本地数组下标。
     *
     * 槽位顺序参与回放哈希：技能 id 相同但装在 0 号位还是 2 号位，
     * 引擎的事件序列就不同，哈希也不同。所以这里完全以
     * build.skills[].slot 为准 —— 服务端说什么就是什么。
     */
    const snapshotSkills = Object.values((bt.build?.skills ?? {}) as Record<string, any>)
      .filter((s) => s && typeof s.id === 'number' && s.slot >= 0)
      .sort((a: any, b: any) => a.slot - b.slot || a.id - b.id)

    const equipped: EquippedSkill[] = []
    for (const s of snapshotSkills) {
      const def = skills.get(s.id)
      if (!def) continue
      equipped.push({
        skillId: def.id,
        name: def.name,
        element: def.element,
        kind: def.kind,
        heatCost: BigInt(def.heat_cost),
        cooldownMs: def.cooldown_ms,
        pierce: def.pierce,
        aoeRadius: def.aoe_radius,
        baseDamage: BigInt(def.base_damage),
        applyElement: (def.apply_element || def.element) as Element | '',
        applyStacks: BigInt(def.apply_stacks),
        projectileSpeed: def.projectile_speed,
        chain: def.chain,
        slot: s.slot,
        cooldownRemaining: 0,
      })
    }
    if (equipped.length === 0) {
      startError.value = '未装备任何技能，请先到「背包」页配置出战技能'
      return
    }
    store.setEquippedSkills(equipped.map((e) => e.skillId))
    // 攻方属性以服务端下发为准（I-6 的前提）
    store.setBuild(bt.build)

    const eng = new BattleEngine({
      level,
      enemies: store.enemyMap,
      skills,
      equipped,
      attacker: store.attacker,
      // seed 服务端以字符串下发（避免 JSON number 精度损失），转 bigint 供 PRNG 用
      seed: BigInt(bt.seed),
      // 可用槽位数同样由服务端权威下发（基础 5 槽 + 专精「额外插槽」）。
      //
      // ⚠️ 缺省绝不能变：绝大多数战报是 4 槽的，
      // 引擎缺省取 ACTIVE_SLOTS = 4 才让历史战报照常重放。
      // 专精那 8 个「额外插槽」节点此前完全惰性 ——
      // 客户端用编译期常量，根本不读服务端下发的槽位数。
      activeSlots: bt.build?.active_slots,
    })
    engine.value = eng
    eng.start()

    // 渲染
    const canvas = await getCanvas()
    if (canvas) {
      const r = new BattleRenderer(canvas, eng)
      // 视口用 CSS 像素（逻辑缩放基准），而 canvas 内部是 dpr 放大后的像素。
      // 两者混用会把画面缩小 dpr 倍 —— 表现为"画布空白/元素挤在角落"。
      r.setViewport(canvasW.value, canvasH.value)
      renderer.value = r
      renderMode.value = 'raf'
      r.start(() => {
        const events = eng.step()
        r.handleEvents(events)
        if (eng.phase === 'card_select') {
          hand.value = [...eng.deck.hand]
        }
        if (eng.phase === 'won' || eng.phase === 'lost') {
          void settle(eng)
        }
      })
      loopStarted = true
    } else {
      renderMode.value = 'fallback'
      // 拿不到 canvas 时用定时器推进，保证逻辑仍可运行（便于自动化测试与无 canvas 环境）
      timerId = setInterval(() => {
        const events = eng.step()
        if (renderer.value) renderer.value.handleEvents(events)
        if (eng.phase === 'card_select') hand.value = [...eng.deck.hand]
        if (eng.phase === 'won' || eng.phase === 'lost') {
          if (timerId) clearInterval(timerId)
          void settle(eng)
        }
      }, TICK_MS)
    }
  } catch (e) {
    startError.value = (e as Error).message
  }
}

let settled = false

async function settle(eng: BattleEngine) {
  if (settled) return
  settled = true
  if (loopStarted && renderer.value) {
    renderer.value.stop()
    loopStarted = false
  }
  if (timerId) {
    clearInterval(timerId)
    timerId = null
  }
  try {
    const payload = eng.settleInput(tokenId.value)
    const res = await api.settleBattle(payload)
    settleResult.value = {
      win: res.win,
      stars: res.stars,
      score: res.score,
      kills: payload.kills,
      leaked: payload.leaked,
      reactions: payload.reactions,
      loot: res.loot ?? {},
      clamped: res.clamped,
      clamp_note: res.clamp_note,
      replay_hash: payload.replay_hash,
    }
    if (res.new_max_stage) store.setMaxStage(res.new_max_stage)
    await store.refreshProfile()
  } catch (e) {
    settleResult.value = {
      win: eng.phase === 'won',
      stars: 0,
      score: eng.score,
      kills: eng.kills,
      leaked: eng.leaked,
      reactions: eng.reactionsCount,
      loot: {},
      clamped: false,
      error: (e as Error).message,
      replay_hash: eng.replayHash(),
    }
  }
}

function takeCard(id: string) {
  if (!engine.value) return
  engine.value.takeCard(id)
  hand.value = [...engine.value.deck.hand]
}

function discardCard(id: string) {
  if (!engine.value) return
  engine.value.discardCard(id)
  hand.value = [...engine.value.deck.hand]
}

function skipCards() {
  if (!engine.value) return
  engine.value.skipCards()
  hand.value = []
}

function backToStage() {
  uni.navigateBack()
}

onLoad(async (query) => {
  levelId.value = Number(query?.level ?? 1)
  await setup()
})

onUnload(() => {
  if (renderer.value) renderer.value.stop()
  if (timerId) clearInterval(timerId)
})
</script>

<style scoped>
.battle-page {
  position: relative;
  width: 100vw;
  height: 100vh;
  overflow: hidden;
}

.battle-canvas {
  position: absolute;
  top: 0;
  left: 0;
}

.overlay-top {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  padding: 24rpx;
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  pointer-events: none;
}

.hp-box {
  background: rgba(0, 0, 0, 0.5);
  border-radius: 12rpx;
  padding: 12rpx 20rpx;
  min-width: 240rpx;
}

.hp-label {
  font-size: 20rpx;
  color: var(--muted);
}

.hp-value {
  font-size: 26rpx;
  font-weight: 600;
  display: block;
}

.hp-bar {
  height: 8rpx;
  background: rgba(255, 255, 255, 0.12);
  border-radius: 4rpx;
  margin-top: 8rpx;
  overflow: hidden;
}

.hp-fill {
  height: 100%;
  background: var(--ok);
}

.top-right {
  text-align: right;
}

.wave-text,
.kill-text {
  font-size: 22rpx;
  color: var(--muted);
  display: block;
}

.card-overlay {
  position: absolute;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(13, 17, 23, 0.96);
  border-top: 1rpx solid var(--border);
  padding: 28rpx 24rpx 40rpx;
}

.debug-tag {
  position: absolute;
  top: 120rpx;
  left: 24rpx;
  font-size: 18rpx;
  color: var(--dim);
  background: rgba(0, 0, 0, 0.4);
  padding: 4rpx 10rpx;
  border-radius: 6rpx;
}

.card-header {
  margin-bottom: 20rpx;
}

.card-header .card-title {
  display: block;
  margin-bottom: 6rpx;
}

.cards {
  display: flex;
  flex-direction: row;
  gap: 16rpx;
  margin-bottom: 20rpx;
}

.hand-card {
  flex: 1;
  background: var(--panel);
  border: 2rpx solid var(--border);
  border-radius: 16rpx;
  padding: 20rpx 16rpx;
  display: flex;
  flex-direction: column;
  min-height: 260rpx;
}

.hand-card.rarity-rare {
  border-color: rgba(88, 166, 255, 0.5);
}
.hand-card.rarity-epic {
  border-color: rgba(201, 167, 255, 0.6);
}

.hand-kind {
  font-size: 18rpx;
  color: var(--muted);
}

.hand-name {
  font-size: 28rpx;
  font-weight: 600;
  margin: 10rpx 0;
}

.hand-descr {
  font-size: 20rpx;
  color: var(--muted);
  line-height: 1.5;
  flex: 1;
}

.hand-foot {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
  margin-top: 12rpx;
}

.result-title {
  font-size: 44rpx;
  font-weight: 700;
  text-align: center;
  display: block;
  margin-bottom: 16rpx;
  color: var(--ok);
}

.stars-big {
  display: flex;
  flex-direction: row;
  justify-content: center;
  gap: 16rpx;
  margin-bottom: 24rpx;
}

.star-big {
  font-size: 60rpx;
  color: var(--warn);
}

.star-big.off {
  color: var(--border);
}

.result-stats {
  display: flex;
  flex-direction: row;
  justify-content: space-around;
  margin-bottom: 20rpx;
}

.rstat {
  display: flex;
  flex-direction: column;
  align-items: center;
}

.clamp-note {
  font-size: 20rpx;
  color: var(--warn);
  background: rgba(255, 211, 61, 0.1);
  padding: 12rpx 16rpx;
  border-radius: 8rpx;
  margin-bottom: 16rpx;
}

.reward-row {
  display: flex;
  flex-direction: row;
  gap: 20rpx;
  justify-content: center;
  margin-bottom: 12rpx;
}

.reward {
  font-size: 24rpx;
  color: var(--ok);
}

.hash-note {
  display: block;
  text-align: center;
  font-size: 18rpx;
  color: var(--dim);
  font-variant-numeric: tabular-nums;
}
</style>
