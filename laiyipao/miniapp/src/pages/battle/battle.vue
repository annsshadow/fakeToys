<template>
  <view class="page-root battle-page" @click="unlockSfx()">
    <!-- 画布：整个战斗画面 -->
    <canvas
      id="battle-canvas"
      canvas-id="battle-canvas"
      type="2d"
      class="battle-canvas"
      :style="{ width: canvasW + 'px', height: canvasH + 'px' }"
    />

    <!-- 顶部信息层 -->
    <view class="overlay-top" :style="{ paddingTop: statusBarPad }">
      <view class="hud-row">
        <view class="hp-box">
          <view class="hp-line">
            <text class="hp-label">防线</text>
            <text class="hp-value mono">{{ hpText }}</text>
          </view>
          <view class="hp-bar">
            <view class="hp-fill" :style="{ width: hpPct + '%' }" />
          </view>
          <!-- 第六批 R5：HUD 常驻当前关卡名，切波/结算前都知道自己在打哪一关 -->
          <text v-if="levelTitle" class="hp-level">{{ levelTitle }}</text>
        </view>
        <view class="hud-spacer" />
        <view class="hud-pill"><text class="wave-text">波次 {{ waveText }}</text></view>
        <view class="hud-pill"><text class="kill-text">击杀 {{ engine?.kills ?? 0 }}</text></view>
      </view>
      <view class="hud-row">
        <!-- 音效开关（2026-10-23）：放在战斗 HUD 而不是设置页 —— 玩家想静音
             的时刻几乎总是"正在打"，让他退出战斗去找设置等于把开关藏起来。 -->
        <view class="mute-chip" :aria-label="muted ? '取消静音' : '静音'" @click="toggleMute">
          <Icon :name="muted ? 'volume-off' : 'volume-on'" :size="30" />
        </view>
        <!--
          战场图例（2026-11-17）：画布底部的热量条与技能槽此前只有颜色块，
          没有任何语义说明（用户反馈「红蓝黄块看不出是干嘛的」）。
          图例遮罩逐项解释 HUD 元素、热量档位、技能槽读法与全部反应链。
          打开图例不改变战斗状态 —— 想边看边停可以自己点暂停。
        -->
        <!-- 第六批 R70：首次进战斗给图例按钮一个呼吸提示，读过一次后记住，不再打扰 -->
        <view
          class="help-chip"
          :class="{ on: helpVisible, 'help-first': !helpSeen }"
          aria-label="战场图例"
          @click="openHelp"
        >
          <text>?</text>
        </view>
        <!--
          倍速档位（2026-10-22）。
          加它是因为「等一波打完」是测试时最耗时间的一件事：1× 跑完一局
          要几分钟，改一行数值就得重跑一次。
          倍速只压缩等待，不改模拟 —— 理由见 render/canvas.ts 的
          `BATTLE_SPEED_STEPS`（动 TICK_MS 会毁掉 I-6 的可重放性）。
        -->
        <view v-if="!settled" class="speed-row">
          <view
            v-for="s in SPEED_STEPS"
            :key="s"
            class="speed-chip"
            :class="{ on: speed === s }"
            @click="setSpeed(s)"
          >{{ s }}×</view>
        </view>
        <view class="hud-spacer" />
        <!-- 2026-10-21（P6-4）：拿不到 canvas 时明确告诉玩家（此前静默降级）。 -->
        <view v-if="renderMode === 'fallback'" class="mode-note">兼容模式</view>
        <!--
          2026-10-21（P4-5）：放弃入口。
          此前体力在开战时就扣了，而页面**没有**任何放弃路径 ——
          中途退出等于「什么都没发生，但资源没了」。
        -->
        <view v-if="!settled" class="abandon-btn" @click="abandon">
          <Icon name="flag" :size="26" />
          <text>{{ abandoning ? '处理中…' : '放弃' }}</text>
        </view>
        <!-- 暂停（2026-10-23）：切后台会自动暂停，回来由玩家自己点继续。 -->
        <view v-if="!settled && !paused" class="pause-btn" @click="doPause()">
          <Icon name="pause" :size="26" />
        </view>
      </view>
    </view>

    <!-- 暂停遮罩：挡住画布点击，避免「看起来暂停了其实还能点」 -->
    <view v-if="paused" class="pause-overlay">
      <view class="pause-box">
        <text class="pause-title">已暂停</text>
        <text class="muted pause-hint">
          {{ pausedByHide ? '切到后台时自动暂停了。' : '' }}战斗时间不会流逝，回放哈希不受影响。
        </text>
        <view class="btn btn-primary pause-go" @click="doResume">继续战斗</view>
      </view>
    </view>

    <!-- 战场图例（2026-11-17）：解释画布 HUD 的全部视觉语言 -->
    <view v-if="helpVisible" class="legend-overlay" @click="helpVisible = false">
      <view class="legend-box" @click.stop>
        <view class="legend-head">
          <text class="legend-title">战场图例</text>
          <view class="legend-close" @click="helpVisible = false"><text>×</text></view>
        </view>
        <scroll-view scroll-y class="legend-scroll">
          <!-- 画布底部：热量条 -->
          <view class="legend-sec">
            <text class="legend-sec-title">热量条（画布底部蓝/黄/橙长条）</text>
            <text class="legend-p">释放技能会积累<b>热量</b>；热量随时间自动下降。</text>
            <view class="legend-rows">
              <view class="legend-row"><view class="dot" style="background: #58a6ff" /><text>蓝色 = 常态，随便放</text></view>
              <view class="legend-row"><view class="dot" style="background: #ffd33d" /><text>黄色 = 已过 80%，快到上限</text></view>
              <view class="legend-row"><view class="dot" style="background: #ff6b35" /><text>橙色 = <b>过热</b>：技能全部锁定 2 秒（元素层数保留，高热量爆发是可用的战术）</text></view>
            </view>
          </view>
          <!-- 画布底部：技能槽 -->
          <view class="legend-sec">
            <text class="legend-sec-title">技能槽（热量条下方的小方格）</text>
            <view class="legend-slot-demo">
              <view class="slot-demo">
                <DefIcon name="elem:fire" :size="30" style="color: #ff6b35" />
                <view class="slot-cost"><text>20</text></view>
              </view>
              <text class="legend-p">每个格子 = 一个出战技能。中央图形是它的元素（和图鉴一致），</text>
            </view>
            <view class="legend-rows">
              <view class="legend-row"><view class="dot dot-ring" style="border-color: #ffd33d" /><text>右上角数字 = 热量费用（每次释放消耗多少热量）</text></view>
              <view class="legend-row"><view class="dot dot-ring" style="border-color: #8b949e" /><text>格内变暗 + 剩余秒数 = 冷却中；描边亮起 = 就绪</text></view>
              <view class="legend-row"><view class="dot dot-ring" style="border-color: #58a6ff" /><text>格子数量 = 你携带的元素种类，「元素层数 N 技能」即来源于此</text></view>
            </view>
          </view>
          <!-- 元素与反应 -->
          <view class="legend-sec">
            <text class="legend-sec-title">元素反应（核心玩法）</text>
            <text class="legend-p">敌人身上先挂一种元素，再用另一种元素命中它就会触发<b>反应</b> —— 反应伤害与面板攻击力关系不大，凑齐元素搭配更重要。</text>
            <view v-for="k in REACTION_ORDER" :key="k" class="legend-rxn">
              <view class="legend-rxn-head">
                <text class="legend-rxn-name">{{ REACTIONS[k].name }}</text>
                <view class="pair-chip">
                  <template v-for="(pair, i) in REACTION_PAIRS[k]" :key="i">
                    <text v-if="i > 0" class="dim">/</text>
                    <text class="tag" :class="'tag-' + pair[0]">{{ ELEMENT_NAME[pair[0]] }}</text>
                    <text class="tag" :class="'tag-' + pair[1]">{{ ELEMENT_NAME[pair[1]] }}</text>
                  </template>
                </view>
              </view>
              <text class="legend-rxn-descr">{{ REACTIONS[k].descr }}</text>
            </view>
          </view>
          <!-- 第六批 R4：敌人视觉语言说明 —— R3 给每种敌人加了独立配色/徽记/形状，
               若图例不讲，玩家只看到「花花绿绿」而不懂这意味着什么。 -->
          <view class="legend-sec">
            <text class="legend-sec-title">敌人外观（怎么分辨敌种）</text>
            <text class="legend-p">每种敌人有独立的<b>本体色 + 五官徽记</b>，形状标记它的行动方式：</text>
            <view class="legend-rows">
              <view class="legend-row"><view class="dot" style="background: #6f9e5c" /><text>绿=游荡者、黄=疾行者、灰=石肤兽…同屏扫一眼就能分种</text></view>
              <view class="legend-row"><view class="dot" style="background: #4aa3b8" /><text>圆=普通，三角=飞行，菱形=远程，六边形=特殊，八边形=精英/BOSS</text></view>
              <view class="legend-row"><view class="dot" style="background: #f0f6fc" /><text>白点五官=双眼/护目镜/核心环等徽记；身上的小方块=元素层数</text></view>
              <view class="legend-row"><view class="dot dot-ring" style="border-color: #58a6ff" /><text>蓝环=护盾；受伤才出红色血条；BOSS 带橙色警示描边</text></view>
            </view>
          </view>
          <!-- 顶部与控制 -->
          <view class="legend-sec">
            <text class="legend-sec-title">顶部信息与控制</text>
            <view class="legend-rows">
              <view class="legend-row"><Icon name="shield" :size="26" style="color: #58a6ff" /><text>防线 = 你的血量，漏怪会扣，扣光即失败</text></view>
              <view class="legend-row"><Icon name="scroll" :size="26" style="color: #8b949e" /><text>波次 = 本关共几波敌人；击杀 = 已消灭数</text></view>
              <view class="legend-row"><Icon name="volume-on" :size="26" style="color: #8b949e" /><text>音效开关；倍速档只压缩等待，不影响战斗结果与回放</text></view>
              <view class="legend-row"><Icon name="flag" :size="26" style="color: #ff7b72" /><text>放弃 = 作废本局并退还体力（无战报无奖励）</text></view>
              <view class="legend-row"><Icon name="pause" :size="26" style="color: #58a6ff" /><text>暂停 = 战斗时间冻结，切后台会自动暂停</text></view>
            </view>
          </view>
          <text class="legend-foot">元素 / 反应 / 技能 / 敌人 / 装备 / 宝石 / 皮肤的完整数值见首页「图鉴」入口。</text>
        </scroll-view>
      </view>
    </view>

    <!-- 选牌界面（I-2） -->
    <view v-if="showCards" class="sheet">
      <view class="card-header">
        <text class="card-title">这一波给你 3 张卡，点一张就装备上</text>
        <text class="card-hint">
          <text>不想？点卡上的「弃」丢掉，还能回 1 点热量（热量攒满就会过热卡 2 秒）。</text>
          <!-- 第六批 R69：牌堆剩余此前完全不展示，玩家不知道这波卡是从多少张里抽的 -->
          <text> 本波还能弃 {{ deck.discardsLeft }} 张，牌堆剩 {{ deck.size }} 张。</text>
        </text>
      </view>
      <view class="cards">
        <view
          v-for="c in hand"
          :key="c.id"
          class="hand-card"
          :class="[`rarity-${c.rarity}`, `kind-${c.kind}`]"
          @click="takeCard(c.id)"
        >
          <view class="hand-top">
            <text class="hand-kind">{{ kindLabel(c.kind) }}</text>
            <text class="rarity-chip" :class="'rarity-' + c.rarity">{{ rarityLabel(c.rarity) }}</text>
          </view>
          <!-- 卡面图标：技能卡 = 元素图形，属性/机制卡 = 对应类别图形（与图鉴同语言） -->
          <view class="hand-icon" :class="handBadgeClass(c)">
            <DefIcon :name="handIconName(c)" :size="44" />
          </view>
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
      <view class="btn" @click="skipCards">全部不要了，直接下一波</view>
    </view>

    <!-- 结算界面 -->
    <view v-if="settleResult" class="sheet">
      <text class="result-title">{{ settleResult.win ? '通关' : '防线失守' }}</text>
      <!-- G91 战斗评级徽章：一个可追求的综合目标 -->
      <view v-if="battleGrade" class="grade-badge" :style="{ borderColor: battleGrade.color, color: battleGrade.color }">
        <text class="grade-letter">{{ battleGrade.grade }}</text>
        <text class="grade-label">{{ battleGrade.label }}</text>
      </view>
      <!-- G92 下次目标：一句可执行的改进方向 -->
      <text v-if="nextGoal" class="next-goal">下一步：{{ nextGoal }}</text>
      <view class="stars-big">
        <text
          v-for="i in 3"
          :key="i"
          class="star-big"
          :class="{ off: i > settleResult.stars }"
        >★</text>
      </view>
      <view class="result-stats">
        <view class="rstat">
          <Icon name="sword" :size="30" style="color: #8b949e" />
          <text class="muted">击杀</text><text class="mono">{{ settleResult.kills }}</text>
        </view>
        <view class="rstat">
          <Icon name="flag" :size="30" style="color: #ff7b72" />
          <text class="muted">漏怪</text><text class="mono">{{ settleResult.leaked }}</text>
        </view>
        <view class="rstat">
          <DefIcon name="reaction" :size="30" style="color: #7ee787" />
          <text class="muted">反应</text><text class="mono">{{ settleResult.reactions }}</text>
        </view>
        <view class="rstat">
          <Icon name="trophy" :size="30" style="color: #ffd33d" />
          <text class="muted">分数</text><text class="mono">{{ settleResult.score }}</text>
        </view>
      </view>
      <view v-if="settleResult.clamped" class="clamp-note">
        服务端已修正本次结算：{{ settleResult.clamp_note }}
      </view>
      <!-- 结算接口失败时的兜底提示：否则用户只能看到本地统计，无从得知上报失败 -->
      <text v-if="settleResult.error" class="clamp-note">{{ settleResult.error }}</text>
      <view class="reward-row">
        <view v-for="(v, k) in settleResult.loot" :key="k" class="reward">
          <Icon :name="currencyIcon(String(k))" :size="28" />
          <text>{{ currencyName(String(k)) }} +{{ v }}</text>
        </view>
      </view>
      <text class="hash-note">回放哈希 {{ settleResult.replay_hash }}（可验真）</text>
      <!-- 查看本局战报（2026-11-01 新一轮第 43 轮）：结算响应带 battle_id，
           跳转战报页并自动展开这一局，玩家可以立即回顾掉落/元素/反应/地形。 -->
      <view
        v-if="settleResult.battle_id"
        class="btn"
        style="margin-top: 20rpx"
        @click="goBattleReport"
      >
        查看本局战报
      </view>
      <!-- 再来一局（2026-11-01 新一轮第 53 轮）：redirectTo 替换当前页，
           重新进入同一关（重新扣体力/发 token），省去返回选关再点一次的路径。 -->
      <view class="btn" style="margin-top: 20rpx" @click="retryLevel">再来一局</view>
      <view class="btn btn-primary" style="margin-top: 20rpx" @click="backToStage">返回关卡</view>
    </view>

    <!-- 启动失败提示 -->
    <view v-if="startError" class="sheet">
      <text class="result-title" style="color: var(--bad)">无法开始战斗</text>
      <text class="muted">{{ startError }}</text>
      <view class="btn" style="margin-top: 24rpx" @click="backToStage">返回</view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { currencyName } from '@/utils/labels'
import Icon from '@/components/Icon.vue'
import { useStatusBarPad } from '@/utils/statusBar'
import { ref, computed, getCurrentInstance, triggerRef } from 'vue'
import { onLoad, onUnload, onHide, onShow, onShareAppMessage } from '@dcloudio/uni-app'
import { useGameStore } from '@/store/game'
import * as api from '@/api/client'
import { BattleEngine } from '@/game/engine'
import { equippedFromSnapshot } from '@/game/replay'
import { battleShare } from '@/utils/share'
import { playBattleEvents } from '@/audio/battle-sfx'
// 音效解锁（2026-10-23）：模板根节点的 `@touchstart="unlockSfx()"` 是**第二道保险**。
//
// 主入口在 `stage.vue` 的「开始战斗」——那是一次真实的用户手势，且发生在
// 进入战斗之前，所以第一发子弹就有声音。这里再兜一次：玩家在战斗页碰一下
// 屏幕也能解锁，不必等到某个特定按钮。
//
// 微信与 iOS 都要求音频在用户手势之后才能播；`unlockSfx()` 会在同一帧里
// 播一次 0 音量的 tap 把手势用掉（理由见 audio/sfx.ts）。
import { initSfx, unlockSfx, isMuted, setMuted, playSfx } from '@/audio/sfx'
import {
  BattleRenderer,
  BATTLE_SPEED_STEPS,
  logicIntervalMs,
  normalizeSpeed,
} from '@/render/canvas'
import { ELEMENT_NAME, REACTIONS, REACTION_ORDER, type Element } from '@/game/elements'
import { REACTION_PAIRS } from '@/utils/reactionPairs'
import { scoreRulesFromServer } from '@/game/score'
import type { Card, EquippedSkill } from '@/game/heatmap'
import type { GeneratedLevel } from '@/game/types'
import DefIcon from '@/components/DefIcon.vue'

const store = useGameStore()

/** 顶部让开状态栏（2026-10-21 P6-2）。 */
const { statusBarPad } = useStatusBarPad()
const inst = getCurrentInstance()

const canvasW = ref(375)
const canvasH = ref(667)
const engine = ref<BattleEngine | null>(null)
const renderer = ref<BattleRenderer | null>(null)
const startError = ref('')
const settleResult = ref<any>(null)

/** G91 战斗评级：基于星级/漏怪/反应的综合评价，给玩家一个可追求的目标（纯客户端派生）。 */
const battleGrade = computed(() => {
  const r = settleResult.value
  if (!r) return null
  if (!r.win) return { grade: 'D', label: '防线失守', color: '#ff7b72' }
  const stars = Number(r.stars ?? 0)
  const leaked = Number(r.leaked ?? 0)
  const reactions = Number(r.reactions ?? 0)
  let score = stars * 30 + Math.min(20, reactions) - leaked * 10
  if (score >= 95) return { grade: 'S', label: '完美防线', color: '#ffd700' }
  if (score >= 70) return { grade: 'A', label: '出色', color: '#7ee787' }
  if (score >= 45) return { grade: 'B', label: '良好', color: '#58a6ff' }
  return { grade: 'C', label: '过关', color: '#8b949e' }
})

/** G92 下次目标：基于本局短板给一句可执行的改进方向（纯客户端派生）。 */
const nextGoal = computed(() => {
  const r = settleResult.value
  if (!r || !r.win) return null
  const leaked = Number(r.leaked ?? 0)
  const reactions = Number(r.reactions ?? 0)
  const stars = Number(r.stars ?? 0)
  if (stars >= 3) return '完美三星！试试更少反应或更高难度关卡。'
  if (leaked > 0) return `漏了 ${leaked} 只怪 —— 补点溅射/控场技能守住防线即可拿更高星。`
  if (reactions < 3) return '多凑一种元素触发反应链，反应伤害不吃攻击力加成。'
  return '再稳一点，冲 3 星拿满钥匙。'
})
const hand = ref<Card[]>([])
const tokenId = ref(0)

/**
 * 渲染模式（2026-10-21 P6-4）。
 *
 * 'pending'  —— 还没起
 * 'raf'     —— 正常，用 requestAnimationFrame 驱动
 * 'fallback'—— **拿不到 canvas**，改用定时器推进
 *
 * ⚠️ 此前这个 ref 只写不读：拿不到 canvas 时页面**静默**降级，
 * 玩家只看到一片黑屏，还以为是游戏卡了。
 * 现在 fallback 会明确提示（「兼容模式」）。
 */
const renderMode = ref<'pending' | 'raf' | 'fallback'>('pending')
const levelId = ref(1)

/** 可选档位（与渲染层逻辑时钟共用一份定义，避免两处各写一套数字）。 */
const SPEED_STEPS = BATTLE_SPEED_STEPS

/** 倍速的持久化键。 */
const SPEED_KEY = 'lyp_battle_speed'

/**
 * 倍速（2026-10-22，测试便利）。
 *
 * 为什么持久化：调数值时同一关要反复重开，每次进战斗都重新点一遍
 * 档位纯属浪费；而档位在 HUD 上一直可见，误留 8× 也一眼能发现。
 */
const speed = ref(loadSpeed())

function loadSpeed(): number {
  try {
    const n = Number(uni.getStorageSync(SPEED_KEY))
    return Number.isFinite(n) && n > 1 ? normalizeSpeed(n) : 1
  } catch {
    // 存储不可用（隐私模式 / 老基础库）不该让战斗开不起来 —— 退回 1×
    return 1
  }
}

/**
 * 换挡。
 *
 * 三条路径都要跟着换：
 *   - 有渲染器 → 交给它重建逻辑时钟
 *   - 无 canvas 的降级路径 → 本页自己的定时器，重建
 *   - 还没开局 → 只记下档位，setup() 建时钟时读 speed
 */
function setSpeed(mult: number): void {
  const next = normalizeSpeed(mult)
  if (next === speed.value) return
  speed.value = next
  try {
    uni.setStorageSync(SPEED_KEY, String(next))
  } catch {
    // 写失败只是下次进来回到 1×，不影响本局
  }
  if (renderer.value) {
    renderer.value.setSpeed(next)
  } else if (timerId) {
    startFallbackLoop()
  }
}

let loopStarted = false
let timerId: ReturnType<typeof setInterval> | null = null
// 第 137 轮：页面是否已卸载。setup() 里有多个 await（login/startBattle/getCanvas），
// 若用户在任一 await 期间退出页面，onUnload 先跑（此时 renderer 还是 null、没东西可停），
// 随后 await 完成会继续创建并 start() 渲染器 —— 在一个已卸载的页面上留下
// 永久运行的 rAF/定时器 + 引擎步进（泄漏）。用本标志在每次 await 后拦截。
let pageUnloaded = false

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
/** 第六批 R5：当前关卡名（引擎公开字段，只读）。 */
const levelTitle = computed(() => {
  if (!engine.value) return ''
  const lv = engine.value.cfg.level
  return `第 ${lv.id} 关 · ${lv.name}`
})
const showCards = computed(() => engine.value?.phase === 'card_select' && !settleResult.value)
const deck = computed(() => engine.value?.deck ?? { discardsLeft: 0, size: 0 } as any)

function elementName(e: Element): string {
  return ELEMENT_NAME[e]
}

function kindLabel(k: string): string {
  return k === 'skill' ? '技能' : k === 'attribute' ? '属性' : '机制'
}

// ── 战场图例（2026-11-17，见模板内说明）────────────────────────────
const helpVisible = ref(false)
/** 第六批 R70：是否读过图例（本机记忆）。 */
const helpSeen = ref(uni.getStorageSync('battle_help_seen') === '1')

function openHelp() {
  helpVisible.value = !helpVisible.value
  if (helpVisible.value && !helpSeen.value) {
    helpSeen.value = true
    uni.setStorageSync('battle_help_seen', '1')
  }
}

/** 手牌稀有度中文名。 */
function rarityLabel(r: string): string {
  return r === 'rare' ? '稀有' : r === 'epic' ? '史诗' : '普通'
}

/** 手卡图标：技能卡用元素图形，属性卡用冲击星，机制卡用双环（反应）。 */
function handIconName(c: Card): string {
  if (c.kind === 'skill' && c.element) return `elem:${c.element}`
  return c.kind === 'mechanic' ? 'reaction' : 'elem:kinetic'
}

/** 手卡图标底座配色：跟随元素；无元素的类别卡走稀有度色。 */
function handBadgeClass(c: Card): string {
  if (c.element) return `badge-${c.element}`
  if (c.rarity === 'rare') return 'badge-rare'
  if (c.rarity === 'epic') return 'badge-epic'
  return ''
}

/** 结算奖励的货币图标（与顶栏资源胶囊同语言）。 */
function currencyIcon(k: string): string {
  if (k === 'gem') return 'gemcurrency'
  if (k === 'energy') return 'bolt'
  return 'coin'
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

/**
 * 无 canvas 时的逻辑推进（与渲染器的逻辑时钟等价，同样受倍速影响）。
 *
 * 抽成具名函数是为了换挡时能重建定时器 —— 此前它是 setup() 里的一个
 * 内联闭包，倍速改不到它，于是「兼容模式」下档位是个摆设。
 */
function fallbackTick(): void {
  // ⚠️ 必须显式还原类型：`engine` 是 ref，Vue 的 `UnwrapRef` 会把类实例的
  // **类型**摊成结构映射（私有成员不在 keyof 里 → 方法看起来全丢了），
  // 而运行时的值仍是那个原始实例（reactive 不代理类实例）。
  // 不还原的话，`settle(eng)` 会报「缺 49 个属性」——一个纯类型假象。
  const eng = engine.value as BattleEngine | null
  if (!eng) return
  // 暂停守卫：与 rAF 路径同一条理由（定时器可能在暂停后被平台补发一次）
  if (paused.value) return
  const events = eng.step()
  playBattleEvents(events)
  // 同上：step() 的内部写不走响应式代理，需手动触发
  triggerRef(engine)
  if (renderer.value) renderer.value.handleEvents(events)
  if (eng.phase === 'card_select') hand.value = [...eng.deck.hand]
  if (eng.phase === 'won' || eng.phase === 'lost') {
    if (timerId) {
      clearInterval(timerId)
      timerId = null
    }
    void settle(eng)
  }
}

/** 按当前倍速（重新）启动降级定时器。重复调用是安全的。 */
function startFallbackLoop(): void {
  if (timerId) {
    clearInterval(timerId)
    timerId = null
  }
  timerId = setInterval(fallbackTick, logicIntervalMs(speed.value))
}

async function setup() {
  const info = uni.getSystemInfoSync()
  // 桌面 H5 内容列限宽 560：画布跟随容器宽，否则画布会横向溢出内容列
  canvasW.value = Math.min(Number(info.windowWidth) || 375, 560)
  canvasH.value = Number(info.windowHeight) || 667

  // 必须等登录完成再申请凭证：否则可能带着上一个账号的 token 去开局，
  // 服务端会按那个账号扣体力，出现"钱包体力充足但开不了局"。
  if (!store.loggedIn) {
    const ok = await store.login()
    if (!ok) {
      startError.value = store.loginError || '登录失败'
      return
    }
  }
  // 第 137 轮：login 的 await 期间可能已退出页面
  if (pageUnloaded) return

  // 启动战斗：向服务端申请一次性凭证（扣体力 + 服务端种子）
  try {
    const bt = await api.startBattle(levelId.value)
    // startBattle 的 await 期间可能已退出页面
    if (pageUnloaded) return
    tokenId.value = bt.token_id
    const level = bt.level as GeneratedLevel
    const skills = store.skillMap

    /**
     * 技能槽位**必须用服务端下发的 slot**，不能用本地数组下标。
     *
     * 槽位顺序参与回放哈希：技能 id 相同但装在 0 号位还是 2 号位，
     * 引擎的事件序列就不同，哈希也不同。所以完全以
     * build.skills[].slot 为准 —— 服务端说什么就是什么。
     *
     * ⚠️ 这里**必须**调 `equippedFromSnapshot`，不能自己再拼一遍。
     *
     * 本文件此前持有该函数的**逐行复制**版本（实战开局拼一次、
     * 重放时 replay.ts 再拼一次）。两份都自洽，但：
     *  - 改一处忘另一处 → 实战伤害与重放伤害不同 → 哈希永远失配，
     *    而症状会把排查方向引向「I-6 验真坏了」，
     *    真因却是「构造逻辑有两份」
     *  - 技能等级系数只在一个地方烘进 baseDamage（见 skill.ts），
     *    两份拼法必然有一份漏掉等级
     */
    const equipped: EquippedSkill[] = equippedFromSnapshot(bt.build, skills)
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
      // 复活币的额外血量（第 153 轮）。**只加当前血量、不动 baseHpMax** ——
      // 漏怪伤害以 baseHpMax 为分母，改 level.base_hp 会把加成整个抵掉
      // （第 148–152 轮踩过的坑，见 domain/revive.go）。
      extraBaseHp: BigInt(bt.extra_base_hp ?? 0),
      // 可用槽位数同样由服务端权威下发（基础 5 槽 + 专精「额外插槽」）。
      //
      // ⚠️ 缺省绝不能变：绝大多数战报是 4 槽的，
      // 引擎缺省取 ACTIVE_SLOTS = 4 才让历史战报照常重放。
      // 专精那 8 个「额外插槽」节点此前完全惰性 ——
      // 客户端用编译期常量，根本不读服务端下发的槽位数。
      activeSlots: bt.build?.active_slots,
      // 第 133 轮：分数规则同样必须来自服务端 /config 的 score_rules。
      // 修前引擎恒用编译期 DEFAULT_SCORE_RULES —— 一旦服务端重标定
      // DefaultScoreRules，客户端星级/得分与结算、I-6 验真全部漂移。
      // config 未到达时 undefined → 引擎回落到 DEFAULT（离线兜底）。
      scoreRules: store.config?.score_rules
        ? scoreRulesFromServer(store.config.score_rules)
        : undefined,
    })
    engine.value = eng
    eng.start()

    // 渲染
    const canvas = await getCanvas()
    // 第 137 轮：getCanvas 的 await 是关键窗口 —— 若期间已退出页面，
    // 不再创建/启动渲染器（否则会留下无人 stop 的 rAF/定时器 + 引擎步进）。
    if (pageUnloaded) return
    if (canvas) {
      const r = new BattleRenderer(canvas, eng)
      // 视口用 CSS 像素（逻辑缩放基准），而 canvas 内部是 dpr 放大后的像素。
      // 两者混用会把画面缩小 dpr 倍 —— 表现为"画布空白/元素挤在角落"。
      r.setViewport(canvasW.value, canvasH.value)
      // 持久化的档位要在建时钟**之前**交进去：start() 会按当前档位建
      // 逻辑时钟，之后再 setSpeed 也能生效，但会白建一次再拆掉。
      r.setSpeed(speed.value)
      renderer.value = r
      renderMode.value = 'raf'
      // 若在 `await getCanvas()` 期间已经 onHide 过，渲染器此刻才刚建好 ——
      // 把「暂停意图」补交给它，否则 UI 显示暂停而时钟在跑。
      if (paused.value) r.pause()
      r.start(() => {
        // 暂停守卫（2026-10-23）。
        //
        // 正常路径下 `renderer.pause()` 已经停掉了逻辑时钟，这里不会进。
        // 但**多一道守卫是有意的**：暂停的意义是「战斗时间不流逝」，
        // 而它依赖「没有别的路径在调 loop」。真机上定时器可能因平台行为
        // 在暂停后仍被触发一次（例如 JS 线程恢复时补发），
        // 那时若没有这行，玩家会看到「已暂停」却仍在掉血。
        if (paused.value) return
        const events = eng.step()
        // 音效与视觉同源：都从同一批事件来，不另算一份状态
        playBattleEvents(events)
        // ⚠️ 引擎在 step() 内部以原始对象身份改写 phase/baseHp 等字段，
        // 这些写入不经过 engine ref 的响应式代理，模板里的 computed
        // （HP/波次/选牌界面）感知不到 —— 必须手动触发一次 ref。
        triggerRef(engine)
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
      startFallbackLoop()
    }
  } catch (e) {
    startError.value = (e as Error).message
  }
}

// 2026-10-21（P4-5）：改成 ref —— 「放弃」按钮要在模板里读它
//（`v-if="!settled"`），普通 let 不是响应式的，模板不会跟着更新。
const settled = ref(false)

/**
 * 结算返回 404 `not_found` 表示这一局的 token 行不存在。
 *
 * 候选⑭曾计划在 token 签发语句上关掉 `synchronous_commit`。R151 复核后未启用：
 * 该 INSERT 与体力/复活币扣减在同一个事务，关掉会连带钱包一起失去崩溃持久性。
 * 这条路先按「万一令牌行丢了」的场景补齐：PG 自身是崩溃安全的，只有 OS 崩溃/
 * 断电才可能丢掉已下发的 token；这时不能把玩家这一局判掉 ——
 * 重取一次同关凭证，用新 token 重报。
 */
function isInvalidBattleToken(e: unknown): boolean {
  const err = e as { status?: number; code?: string }
  return err?.status === 404 && err?.code === 'not_found'
}

async function settle(eng: BattleEngine) {
  if (settled.value) return
  settled.value = true
  if (loopStarted && renderer.value) {
    renderer.value.stop()
    loopStarted = false
  }
  if (timerId) {
    clearInterval(timerId)
    timerId = null
  }
  try {
    let payload = eng.settleInput(tokenId.value)
    let res: any
    try {
      res = await api.settleBattle(payload)
    } catch (e) {
      if (!isInvalidBattleToken(e)) throw e
      // 只重试一次：网络故障也可能表现为 404 之外的错误，
      // 但 404 若持续存在，再开局只会多扣一次体力，不会多拿奖励。
      const bt = await api.startBattle(levelId.value)
      tokenId.value = bt.token_id
      payload = { ...payload, token_id: bt.token_id }
      res = await api.settleBattle(payload)
    }
    settleResult.value = {
      battle_id: res.battle_id,
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

/**
 * 查看本局战报（2026-11-01 新一轮第 43 轮）。
 *
 * 结算响应带 `battle_id`，但此前没有任何消费 —— 玩家打完只能看到
 * 结算面板上的数字，想回顾掉落/元素/反应/地形没有入口。
 * 这里跳转战报页并携带 battle_id，战报页会自动展开这一局。
 */
function goBattleReport() {
  const id = Number(settleResult.value?.battle_id)
  if (Number.isInteger(id) && id > 0) {
    uni.navigateTo({ url: `/pages/battles/battles?battle_id=${id}` })
  }
}

/**
 * 再来一局（2026-11-01 新一轮第 53 轮）。
 * 用 redirectTo 替换当前战斗页，重新进入同一关。
 * 不用 navigateTo：否则每次重开都会叠加一个战斗页，页面栈无限增长。
 */
function retryLevel() {
  uni.redirectTo({ url: `/pages/battle/battle?level=${levelId.value}` })
}

/**
 * 放弃这一局，退回体力（2026-10-21 P4-5）。
 *
 * 此前**没有放弃路径** —— 体力在开战时就扣了，
 * 玩家中途退出就等于「什么都没发生，但资源没了」。
 *
 * 放弃会**作废**这枚开局凭证（服务端标记为已用），
 * 所以放弃之后不能再结算这一局 —— 那是刻意的：
 * 放弃与结算必须是**互斥**的两条路，否则能「打完再退」。
 */
const abandoning = ref(false)

/** 放弃成功后的跳转定时器（页面卸载时必须清掉 —— 否则会弹掉上一页）。 */
let abandonNavTimer: ReturnType<typeof setTimeout> | null = null

async function abandon() {
  // ⚠️ 守卫必须在 showModal **之前**：
  // 放在之后的话，双击会弹出两个确认框，两个都 resolve → 发两次请求，
  // 第二次必然被服务端拒（这局已经结束了）→ 弹一个莫名其妙的错误。
  if (abandoning.value || !tokenId.value) return
  abandoning.value = true
  const ok = await new Promise<boolean>((resolve) => {
    uni.showModal({
      title: '放弃这一局？',
      content: '体力会退还，但这一局没有任何战报与奖励。',
      confirmText: '放弃',
      cancelText: '继续打',
      success: (res) => resolve(!!res.confirm),
      fail: () => resolve(false),
    })
  })
  if (!ok) {
    abandoning.value = false
    return
  }
  try {
    const res = await api.abandonBattle(tokenId.value)
    await store.refreshWallet()
    uni.showToast({
      title: res.refunded_energy > 0 ? `已放弃，退回 ${res.refunded_energy} 体力` : '已放弃',
      icon: 'none',
    })
    settled.value = true
    // 停掉引擎：画面继续跑 800ms 与「已放弃」矛盾，
    // 而且引擎终局会触发 settle() —— 与 abandon() 并发。
    stopLoop()
    abandonNavTimer = setTimeout(() => {
      abandonNavTimer = null
      uni.navigateBack()
    }, 800)
  } catch (e) {
    uni.showToast({ title: (e as Error).message, icon: 'none' })
  } finally {
    abandoning.value = false
  }
}

/** 停掉渲染器与逻辑定时器（结算与放弃都要用）。 */
function stopLoop(): void {
  if (renderer.value) renderer.value.stop()
  renderer.value = null
  if (timerId) {
    clearInterval(timerId)
    timerId = null
  }
  loopStarted = false
}

onLoad(async (query) => {
  levelId.value = Number(query?.level ?? 1)
  await setup()
})

onUnload(() => {
  // 放弃后的跳转定时器必须清掉：
  // 玩家在这 800ms 内按系统返回键离开 → 定时器在已卸载页面上触发
  // `uni.navigateBack()` → 把**上一页**（选关页）一起弹掉。
  if (abandonNavTimer) {
    clearTimeout(abandonNavTimer)
    abandonNavTimer = null
  }
  // 第 137 轮：先置标志，让仍在飞行中的 setup()（各 await 之后）能感知到
  // 「页面已卸载」并停止创建渲染器/定时器。
  pageUnloaded = true
  if (renderer.value) renderer.value.stop()
  if (timerId) clearInterval(timerId)
})

// ── 暂停（2026-10-23）─────────────────────────────────────────────
//
// ## 为什么必须有
//
// 逻辑时钟是 `setInterval`，**与页面可见性无关** —— 这是 `canvas.ts`
// 当初为「后台也能跑逻辑」刻意做的设计（无头环境 / 真机切后台）。
// 副作用是：玩家切到微信聊天再回来，「战斗推进了多少」取决于
// 平台对后台定时器的处置（小程序挂起 JS、H5 标签页节流到 1Hz）。
// 也就是说这个行为**平台相关且未定义**。
//
// ## 为什么不自动恢复
//
// 回来就自动继续，玩家会发现自己离开的这几秒里防线被打掉一半 ——
// 那是比「暂停着等你点继续」糟糕得多的体感。所以 `onShow` 只**保持暂停**，
// 由玩家自己点「继续」。
//
// ## 为什么暂停不污染 replay_hash
//
// 暂停只是「不再 step」。`tick` 与 `elapsedMs` 都只在 `step()` 内自增，
// 于是 `duration_ms` 与不暂停时完全一致。判据见 battle.test.ts
// 「暂停后结算的 duration_ms 与不暂停相等」—— 那条一旦红，
// 意味着所有暂停过的战报都无法被重放验证。

const paused = ref(false)

/**
 * 音效开关（2026-10-23）。
 *
 * `initSfx()` 必须在读 `isMuted()` 之前调 —— 它负责把持久化偏好读进模块。
 * 这里用 ref 镜像一份，是为了让按钮的图标随点击立刻变（模块内的是普通变量，
 * 不参与响应式）。
 */
initSfx()
const muted = ref(isMuted())

function toggleMute(): void {
  const next = !muted.value
  muted.value = next
  setMuted(next)
  // 开启音效时给一声确认音：否则点了没反应，玩家不知道是开了还是没开
  if (!next) {
    unlockSfx()
    playSfxTap()
  }
}

/** 开关确认音。 */
function playSfxTap(): void {
  playSfx('tap')
}
/**
 * 暂停是由「切后台」触发的（模板里据此显示不同提示）。
 *
 * ⚠️ 必须是 `ref`，不能是普通 `let`：模板在 `v-if="paused"` 块里读它。
 * 第一版是 `let`，于是「玩家先手动点暂停（pausedByHide=false，遮罩已渲染），
 * 随后切后台（改成 true）」这个组合下，遮罩**不会重渲染** ——
 * 提示永远停在「战斗时间不会流逝」而少了「切到后台时自动暂停了」。
 */
const pausedByHide = ref(false)

/**
 * 暂停。
 *
 * `reason` 决定遮罩上的说明文案，也决定 `onShow` 时的行为（两者都不自动恢复）。
 */
function doPause(reason: 'user' | 'hide' = 'user'): void {
  if (paused.value || settled.value) return
  paused.value = true
  pausedByHide.value = reason === 'hide'
  if (renderer.value) renderer.value.pause()
  if (timerId) {
    // 降级路径（无 canvas）：本页自己的定时器也要停
    clearInterval(timerId)
    timerId = null
  }
}

function doResume(): void {
  if (!paused.value) return
  paused.value = false
  pausedByHide.value = false
  if (renderer.value) renderer.value.resume()
  else if (loopStarted || engine.value) startFallbackLoop()
}

onHide(() => {
  // 判据不是 `engine.value` —— 而是「战斗已开始、尚未结算」。
  //
  // 用 `engine.value` 会漏掉一个真实窗口：`eng.start()` 之后、`r.start()` 之前
  // 有一次 `await getCanvas()`。那期间 engine 已存在、renderer 还是 null、
  // timerId 也是 null —— 走 `engine.value` 分支会 `paused=true` 但两条时钟都没起，
  // 随后 setup 继续 `r.start()`，只能靠 loop 里的 `if (paused) return` 兜住。
  // 那行守卫是**第二道防线**，不该成为唯一防线。
  if (settled.value || startError.value) return
  doPause('hide')
})

onShow(() => {
  // 不自动恢复：回来就继续，玩家会发现自己离开的几秒里防线被打掉一半。
  //
  // 这里做一次**对账**：`paused` 是 UI 状态，`renderer.pause()` 是时钟状态，
  // 两者若不一致（例如渲染器尚未 start 时先暂停过），以渲染器为准把 UI 纠正过来。
  // 没有这次对账，`isPaused()` 就是个没人读的字段。
  if (!paused.value && renderer.value?.isPaused()) {
    paused.value = true
  }
})

/**
 * 分享（2026-10-23）。
 *
 * 未结算时**不得**出现分数/星级：那两个值此时还没有权威结果
 * （服务端结算才算数），拼进标题就是假数据。
 */
onShareAppMessage(() =>
  battleShare({
    // ⚠️ 判据是「**结果对象已就位**且没有错误」，不是「settled 已置位」。
    //
    // `settled.value = true` 在 settle() 的第一行，而 settleResult 要等
    // `await api.settleBattle` 返回后才赋值。这段窗口（弱网可达数秒）里
    // settleResult 是 null，`?.error` 得到 undefined，`!undefined === true`
    // → 走「已结算」分支，星级/分数取 `?? 0` —— 分享出去是
    // 「第 3 关 0 星 · 0 分」。share.ts 的注释专门写了要防这个。
    settled: !!settleResult.value && !settleResult.value.error,
    levelId: levelId.value,
    stars: Number(settleResult.value?.stars ?? 0),
    score: Number(settleResult.value?.score ?? 0),
  }),
)
</script>

<style scoped>
.battle-page {
  position: relative;
  /* 100vw 在 uni-page-body 限宽 560 的桌面 H5 下会比内容列更宽（含滚动条宽），
     造成横向溢出；100% 跟随容器即可 */
  width: 100%;
  height: 100vh;
  overflow: hidden;
}

.battle-canvas {
  position: absolute;
  top: 0;
  left: 0;
}

.abandon-btn {
  /*!
   * ⚠️ 必须显式写 `pointer-events: auto`。
   *
   * `.overlay-top` 有 `pointer-events: none`（为了让顶栏不挡住画布点击），
   * 而 `pointer-events` 是**可继承**属性 —— 不写这一行，
   * 按钮就继承到 `none`，**永远点不动**（而且画面上看起来完全正常）。
   *
   * 这条是 P4-5「放弃战斗」的整个功能：按钮可见、点不动，
   * 玩家只会以为游戏卡了。
   */
  pointer-events: auto;
  display: flex;
  align-items: center;
  gap: 6rpx;
  height: 60rpx;
  padding: 0 22rpx;
  border-radius: 14rpx;
  background: rgba(255, 123, 114, 0.18);
  border: 1rpx solid rgba(255, 123, 114, 0.4);
  color: #ff7b72;
  font-size: 24rpx;
}

/*
 * 暂停按钮。
 *
 * ⚠️ 与 `.abandon-btn` / `.speed-row` 同一条坑：`.overlay-top` 的
 * `pointer-events: none` 是**可继承**属性，不写 `auto` 就永远点不动。
 * 这已经是本项目第三次踩它了 —— 所以这里连注释一起复制，
 * 而不是指望下一个加按钮的人记得。
 */
.mute-chip {
  /* 与 .abandon-btn / .speed-row / .pause-btn 同一条坑：父级 pointer-events: none */
  pointer-events: auto;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 60rpx;
  height: 60rpx;
  border-radius: 14rpx;
  background: rgba(0, 0, 0, 0.5);
  border: 1rpx solid var(--border-soft);
  color: var(--text-2);
}

.pause-btn {
  pointer-events: auto;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 60rpx;
  height: 60rpx;
  border-radius: 14rpx;
  background: rgba(88, 166, 255, 0.18);
  border: 1rpx solid rgba(88, 166, 255, 0.4);
  color: #58a6ff;
}

/*
 * 战场图例按钮（2026-11-17）。
 * ⚠️ 与 .abandon-btn / .speed-row 同一条坑：`.overlay-top` 的
 * pointer-events: none 可继承，不写 auto 就点不动（见同文件其他 chip 注释）。
 */
.help-chip {
  pointer-events: auto;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 60rpx;
  height: 60rpx;
  border-radius: 14rpx;
  background: rgba(0, 0, 0, 0.5);
  border: 1rpx solid var(--border-soft);
  color: var(--muted);
  font-size: 30rpx;
  font-weight: 700;
}

.help-chip.on {
  background: rgba(88, 166, 255, 0.18);
  border-color: rgba(88, 166, 255, 0.4);
  color: #58a6ff;
}

/* 第六批 R70 首次未读图例的呼吸提示 */
.help-chip.help-first {
  border-color: var(--fire);
  color: var(--fire);
  animation: glow-pulse 2s ease-in-out infinite;
}

/* 图例遮罩：结构同暂停遮罩，内容可滚动 */
.legend-overlay {
  position: absolute;
  left: 0;
  top: 0;
  right: 0;
  bottom: 0;
  z-index: 30;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(13, 17, 23, 0.86);
}

.legend-box {
  width: 86%;
  max-height: 82vh;
  padding: 28rpx 28rpx 20rpx;
  border-radius: 20rpx;
  background: var(--panel);
  border: 1rpx solid var(--border);
  display: flex;
  flex-direction: column;
  box-shadow: var(--shadow-2);
}

.legend-head {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 14rpx;
}

.legend-title {
  font-size: 32rpx;
  font-weight: 700;
  color: var(--text);
}

.legend-close {
  width: 56rpx;
  height: 56rpx;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  background: var(--panel-2);
  color: var(--muted);
  font-size: 34rpx;
}

.legend-scroll {
  max-height: 62vh;
}

.legend-sec {
  padding: 14rpx 0;
  border-top: 1rpx solid var(--border-soft);
}

.legend-sec:first-child {
  border-top: none;
}

.legend-sec-title {
  display: block;
  font-size: 28rpx;
  font-weight: 600;
  color: var(--text);
  margin-bottom: 10rpx;
}

.legend-p {
  display: block;
  font-size: 24rpx;
  line-height: 1.6;
  color: var(--muted);
  margin-bottom: 8rpx;
}

.legend-rows {
  display: flex;
  flex-direction: column;
  gap: 10rpx;
}

.legend-row {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 14rpx;
  font-size: 24rpx;
  color: var(--text-2);
  line-height: 1.5;
}

.legend-row .dot {
  width: 20rpx;
  height: 20rpx;
  border-radius: 50%;
  flex-shrink: 0;
}

.legend-row .dot-ring {
  background: transparent;
  border: 3rpx solid var(--border);
}

.legend-slot-demo {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 16rpx;
  margin-bottom: 10rpx;
}

.slot-demo {
  position: relative;
  width: 64rpx;
  height: 64rpx;
  border-radius: 12rpx;
  background: rgba(22, 27, 34, 0.9);
  border: 2rpx solid #ff6b35;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.slot-cost {
  position: absolute;
  top: -10rpx;
  right: -10rpx;
  width: 30rpx;
  height: 30rpx;
  border-radius: 50%;
  background: rgba(0, 0, 0, 0.75);
  display: flex;
  align-items: center;
  justify-content: center;
}

.slot-cost text {
  font-size: 16rpx;
  color: #ffd33d;
  font-weight: 700;
}

.legend-rxn {
  padding: 10rpx 0;
}

.legend-rxn-head {
  display: flex;
  flex-direction: row;
  align-items: center;
  flex-wrap: wrap;
  gap: 12rpx;
}

.legend-rxn-name {
  font-size: 26rpx;
  font-weight: 600;
  color: var(--text);
}

.pair-chip {
  display: inline-flex;
  flex-direction: row;
  align-items: center;
  flex-wrap: wrap;
  gap: 6rpx;
}

.legend-rxn-descr {
  display: block;
  font-size: 22rpx;
  line-height: 1.5;
  color: var(--muted);
  margin-top: 4rpx;
}

.legend-foot {
  display: block;
  padding: 12rpx 0 4rpx;
  font-size: 22rpx;
  color: var(--dim);
}

/*
 * 暂停遮罩。
 *
 * 除了展示，它还承担一个功能：**挡住画布**。
 * 没有它的话暂停期间玩家仍能点画布，看起来「暂停了但还能操作」——
 * 而实际上那些点击会落到画布上并被忽略，是纯粹的困惑来源。
 */
.pause-overlay {
  position: absolute;
  left: 0;
  top: 0;
  right: 0;
  bottom: 0;
  z-index: 30;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(13, 17, 23, 0.78);
}

.pause-box {
  width: 70%;
  padding: 40rpx 32rpx;
  border-radius: 20rpx;
  background: var(--panel);
  border: 1rpx solid var(--border);
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16rpx;
}

.pause-title {
  font-size: 34rpx;
  font-weight: 700;
  color: var(--text);
}

.pause-hint {
  font-size: 22rpx;
  line-height: 1.6;
  text-align: center;
}

.pause-go {
  margin-top: 12rpx;
  width: 100%;
}

.mode-note {
  pointer-events: auto;
  padding: 4rpx 12rpx;
  border-radius: 8rpx;
  background: rgba(255, 209, 61, 0.2);
  color: #ffd33d;
  font-size: 22rpx;
}

/**
 * 倍速档位。
 *
 * ⚠️ 与 `.abandon-btn` 同一条坑：`.overlay-top` 有 `pointer-events: none`，
 * 而它是**可继承**属性 —— 不写 `auto`，四个档位看起来正常但一个都点不动。
 */
.speed-row {
  pointer-events: auto;
  display: flex;
  flex-direction: row;
  gap: 8rpx;
}

.speed-chip {
  /* 24rpx 字号 + 上下 8rpx 内边距 → 触控高度约 40rpx；
     仍比「放弃」按钮小，避免遮挡画面中央的战场 */
  min-width: 64rpx;
  height: 60rpx;
  box-sizing: border-box;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 14rpx;
  border: 1rpx solid var(--border-soft);
  background: rgba(0, 0, 0, 0.5);
  color: var(--muted);
  font-size: 24rpx;
}

.speed-chip.on {
  border-color: rgba(126, 231, 135, 0.6);
  background: rgba(126, 231, 135, 0.16);
  color: #7ee787;
  font-weight: 600;
}

.overlay-top {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  padding: 20rpx 20rpx 0;
  display: flex;
  flex-direction: column;
  gap: 14rpx;
  pointer-events: none;
}

/* HUD 行：胶囊各自排布，行内不再互相拉伸 */
.hud-row {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 12rpx;
}

.hud-spacer {
  flex: 1;
}

/* 波次 / 击杀胶囊 */
.hud-pill {
  display: inline-flex;
  align-items: center;
  height: 56rpx;
  padding: 0 20rpx;
  border-radius: 999rpx;
  background: rgba(0, 0, 0, 0.5);
  border: 1rpx solid var(--border-soft);
}

.hp-box {
  background: rgba(0, 0, 0, 0.5);
  border: 1rpx solid var(--border-soft);
  border-radius: 16rpx;
  padding: 12rpx 20rpx;
  min-width: 240rpx;
}

.hp-line {
  display: flex;
  flex-direction: row;
  align-items: baseline;
  justify-content: space-between;
  gap: 16rpx;
}

.hp-label {
  font-size: 24rpx;
  color: var(--muted);
}

.hp-value {
  font-size: 28rpx;
  font-weight: 700;
}

.hp-bar {
  height: 10rpx;
  background: rgba(255, 255, 255, 0.12);
  border-radius: 5rpx;
  margin-top: 10rpx;
  overflow: hidden;
}

.hp-fill {
  height: 100%;
  background: var(--ok);
}

/* 第六批 R5 当前关卡名 */
.hp-level {
  display: block;
  margin-top: 6rpx;
  font-size: 20rpx;
  color: var(--muted);
  max-width: 320rpx;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.wave-text,
.kill-text {
  font-size: 24rpx;
  color: var(--text-2);
}

.sheet {
  position: absolute;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(13, 17, 23, 0.96);
  border-top: 1rpx solid var(--border);
  padding: 28rpx 24rpx 40rpx;
  animation: fade-in-up 0.25s ease-out;
}

.card-header {
  margin-bottom: 20rpx;
}

.card-header .card-title {
  display: block;
  margin-bottom: 6rpx;
}

/* 选牌引导（口语化说明，见模板注释） */
.card-header .card-hint {
  display: block;
  font-size: 22rpx;
  line-height: 1.6;
  color: var(--dim);
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
  border: 2rpx solid var(--border-soft, var(--border));
  border-radius: 16rpx;
  padding: 20rpx 16rpx;
  display: flex;
  flex-direction: column;
  min-height: 260rpx;
  transition: transform 0.15s, border-color 0.2s, box-shadow 0.2s;
}

.hand-card:active {
  transform: scale(0.97);
}

.hand-card.rarity-rare {
  border-color: var(--ice-soft, rgba(88, 166, 255, 0.5));
  box-shadow: 0 0 0 1rpx rgba(88, 166, 255, 0.25), 0 4rpx 12rpx rgba(88, 166, 255, 0.15);
}
.hand-card.rarity-epic {
  border-color: var(--kinetic-soft, rgba(201, 167, 255, 0.6));
  box-shadow: 0 0 0 1rpx rgba(201, 167, 255, 0.25), 0 4rpx 12rpx rgba(201, 167, 255, 0.15);
}

.hand-kind {
  font-size: 22rpx;
  color: var(--muted);
}

/* 卡面顶部行：类别 + 稀有度徽章 */
.hand-top {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
  gap: 8rpx;
}

/* 卡面图标座：元素 / 类别的视觉锚点（与图鉴同一套语言） */
.hand-icon {
  width: 84rpx;
  height: 84rpx;
  border-radius: 16rpx;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--panel-2);
  color: var(--text-2);
  margin: 4rpx 0 10rpx;
  align-self: flex-start;
}

.hand-name {
  font-size: 28rpx;
  font-weight: 600;
  margin: 10rpx 0;
}

.hand-descr {
  font-size: 22rpx;
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

/* G91 战斗评级徽章 */
.grade-badge {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: center;
  gap: 12rpx;
  width: 240rpx;
  margin: 0 auto 16rpx;
  padding: 8rpx 0;
  border: 2rpx solid;
  border-radius: 999rpx;
}
.grade-letter { font-size: 40rpx; font-weight: 800; }
.grade-label { font-size: 22rpx; }
.next-goal {
  display: block;
  text-align: center;
  font-size: 22rpx;
  color: var(--muted);
  margin: -6rpx 0 16rpx;
  padding: 0 24rpx;
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
  gap: 4rpx;
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
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 8rpx;
  font-size: 24rpx;
  color: var(--ok);
}

.hash-note {
  display: block;
  text-align: center;
  font-size: 22rpx;
  color: var(--dim);
  font-variant-numeric: tabular-nums;
}
</style>
