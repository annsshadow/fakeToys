<template>
  <view class="page-root home">
    <!-- 顶部资源栏（v2）：资源从纯文本改为图标胶囊 -->
    <view class="topbar" :style="{ paddingTop: statusBarPad }">
      <view class="player">
        <view class="avatar">炮</view>
        <view class="player-info">
          <text class="nick">{{ store.nickname || '玩家' }}</text>
          <text class="dim">{{ store.isGuest ? '游客账号' : '账号登录' }}</text>
        </view>
      </view>
      <view class="res-row">
        <view class="res-pill" :class="{ 'res-bump': bump.coin }">
          <Icon name="coin" :size="26" />
          <text class="mono">{{ store.wallet.coin }}</text>
        </view>
        <view class="res-pill" :class="{ 'res-bump': bump.gem }">
          <Icon name="gemcurrency" :size="26" />
          <text class="mono">{{ store.wallet.gem }}</text>
        </view>
        <view class="res-pill" :class="{ 'res-bump': bump.energy }">
          <Icon name="bolt" :size="26" />
          <text class="mono">{{ store.wallet.energy }}</text>
        </view>
        <!-- 第六批 R56：钥匙是第四种货币（me 页有用途说明），此前首页完全不显示 -->
        <view class="res-pill">
          <text class="res-glyph">🔑</text>
          <text class="mono">{{ store.wallet.keys ?? 0 }}</text>
        </view>
      </view>
    </view>

    <view v-if="store.loginError" class="pad safe-bottom">
      <view class="card" style="border-color: var(--bad)">
        <text class="card-title">无法连接服务器</text>
        <text class="muted">{{ store.loginError }}</text>
        <text class="dim" style="display: block; margin-top: 12rpx">
          请先启动后端：cd server && go run ./cmd/api
        </text>
        <view class="btn btn-primary" style="margin-top: 20rpx" @click="retry">重试</view>
      </view>
    </view>

    <view v-else-if="store.configLoading" class="pad">
      <view class="card"><text class="muted">正在加载游戏配置…</text></view>
    </view>

    <view v-else class="pad">
      <!--
        公告（2026-10-23）。
        此前 `announcements` 表与后台的增删查全都在，唯独玩家端零入口 ——
        运营发的公告只有运营自己看得见。这是闭环缺口，不是新功能。

        位置在构筑评分之前：它是时效性内容，过期了就没价值；
        而构筑评分玩家每次进来都会看到，往后放一点没有损失。
      -->
      <view v-if="announcements.length" class="card">
        <view class="card-title">
          <text>公告</text>
          <!-- 全部公告入口（2026-11-01 新一轮第 42 轮）：首页只展示最新几条，
               历史公告在首页不可回看，这里提供公告列表页入口。 -->
          <text class="ann-all" @click="goAnnouncements">全部 {{ announcements.length }} 条 →</text>
        </view>
        <view
          v-for="(a, i) in announcements"
          :key="a.id"
          class="ann-item"
          @click="toggleAnn(a.id)"
        >
          <view class="ann-head">
            <!-- 第六批 R55：首页公告首条与公告页统一「最新」徽章 -->
            <text v-if="i === 0" class="ann-new">最新</text>
            <text class="ann-title">{{ a.title }}</text>
            <text class="dim mono">{{ formatAnnDate(a.published_at) }}</text>
            <!-- R64 展开态指示：此前只能靠正文有没有截断点来猜 -->
            <text class="ann-toggle" :class="{ open: expanded.has(a.id) }">⌄</text>
          </view>
          <text class="ann-body" :class="{ clamped: !expanded.has(a.id) }">{{ a.body }}</text>
        </view>
      </view>

      <!-- 构筑评分（I-7）：主指标不是"战力 9999" -->
      <view class="card">
        <view class="card-title">
          <text>构筑评分</text>
          <text class="score mono">{{ store.buildRating?.total ?? 0 }}</text>
        </view>

        <!-- R61 构筑五维雷达图（实质可视化）：进度条只能逐行读，雷达图把
             「偏科」形状一眼画出来 —— 哪个方向凹进去就是短板。数据直接复用 dims.pct。 -->
        <view class="radar-wrap">
          <svg class="radar-svg" viewBox="0 0 200 180" aria-label="构筑五维雷达图">
            <!-- 三圈参考网格（20%/60%/100%） -->
            <polygon
              v-for="ring in radarRings"
              :key="ring"
              class="radar-ring"
              :points="radarPolygon(ring)"
            />
            <!-- 五个顶点轴线 -->
            <line
              v-for="(pt, i) in radarAxisEnds"
              :key="'ax' + i"
              class="radar-axis"
              x1="100" y1="90" :x2="pt.x" :y2="pt.y"
            />
            <!-- 实际构筑多边形（五维 pct 决定顶点） -->
            <polygon class="radar-shape" :points="radarPolygonActual" />
            <!-- 五个顶点标签 -->
            <text
              v-for="(lb, i) in radarLabels"
              :key="'lb' + i"
              class="radar-label"
              :x="lb.x" :y="lb.y"
              :text-anchor="lb.x < 92 ? 'end' : lb.x > 108 ? 'start' : 'middle'"
            >{{ lb.text }}</text>
          </svg>
        </view>

        <!-- 五维进度条：明确告诉玩家"缺什么"。
             每行尾部的「权重」是服务端 rating_weights 下发的份额（第 150 轮），
             让玩家知道补哪一维对总分的边际收益最大。 -->
        <view v-for="d in dims" :key="d.label" class="dim-row">
          <view class="dim-row-head">
            <text class="dim-label">{{ d.label }}</text>
            <text v-if="d.weightPct > 0" class="dim-weight mono">权重 {{ d.weightPct }}%</text>
          </view>
          <view class="bar">
            <view class="bar-fill" :style="{ width: d.pct + '%', background: d.color }" />
          </view>
          <view class="dim-row-foot">
            <text class="dim-value mono">{{ d.value }}</text>
            <!-- 空维补导（100 轮打磨 R14）：0 分的维度此前是一条死灰条，玩家
                 不知道去哪补。每条弱点给出明确入口，把「缺什么」变成「点这里」。 -->
            <text
              v-if="d.pct === 0 && d.hint"
              class="dim-hint link"
              @click="go(d.hint)"
            >→ {{ d.hintLabel }}</text>
            <text v-else class="dim-empty-hint" v-if="d.pct === 0">待培养</text>
          </view>
        </view>

        <!-- 短板提示：这是 I-7 真正想做的事 -->
        <view v-if="weaknesses.length" class="weakness-box">
          <text class="weakness-title">搭配建议</text>
          <text v-for="(w, i) in weaknesses" :key="i" class="weakness-item">· {{ w }}</text>
        </view>
      </view>

      <!-- 主行动（v2 hero）：页面唯一的渐变强调块，一眼看到「该点哪里」 -->
      <view class="action-card card-hero tap" @click="goStage">
        <view class="flex-1">
          <text class="action-kicker">当前关卡</text>
          <text class="action-title">第 {{ store.unlockedLevel }} 关</text>
          <text class="action-sub">守住防线，清完全部波次</text>
          <!-- R62 已通关进度：让玩家感到「在往前走」 -->
          <text v-if="store.maxStage > 0" class="action-progress">已通关 {{ store.maxStage }} 关</text>
        </view>
        <!-- G87 闯关进度环：通关比例一眼可见 -->
        <view class="stage-ring-wrap">
          <svg class="stage-ring" viewBox="0 0 80 80" aria-label="闯关进度">
            <circle class="sr-bg" cx="40" cy="40" r="36" />
            <circle
              class="sr-fill" cx="40" cy="40" r="36" transform="rotate(-90 40 40)"
              :stroke-dasharray="`${stageRing.dash} ${stageRing.rest}`"
            />
          </svg>
          <text class="stage-ring-label">{{ stageRing.pct }}%</text>
        </view>
        <view class="action-go">
          <text>开始</text>
          <Icon name="chevron-right" :size="28" />
        </view>
      </view>

      <!-- 功能入口（2026-11-16 分组）：
           15 个入口拆成「玩法」9 项 +「图鉴」6 项两组，每组都能被 3 整除，
           网格用 CSS Grid 恒定 3 列（修真机亚像素溢出导致的两列）。 -->
      <view class="grid-group">
        <view class="grid-group-head">
          <text class="grid-group-title">玩法</text>
          <text class="grid-group-count">{{ playMenus.length }}</text>
        </view>
        <view class="grid">
          <view v-for="m in playMenus" :key="m.path" class="grid-item tap" @click="go(m.path)">
            <view class="icon-badge grid-icon"><Icon :name="m.icon" :size="44" /></view>
            <text class="grid-label">{{ m.label }}</text>
          </view>
        </view>
      </view>

      <view class="grid-group">
        <view class="grid-group-head">
          <text class="grid-group-title">图鉴</text>
          <text class="grid-group-count">{{ codexMenus.length }}</text>
        </view>
        <view class="grid">
          <view v-for="m in codexMenus" :key="m.path" class="grid-item tap" @click="go(m.path)">
            <view class="icon-badge grid-icon"><Icon :name="m.icon" :size="44" /></view>
            <text class="grid-label">{{ m.label }}</text>
          </view>
        </view>
      </view>

      <!-- 元素说明 -->
      <view class="card">
        <text class="card-title">5 种元素</text>
        <view class="elem-row">
          <!-- R61 元素 chip 可点下钻到反应图鉴：说明里提到反应链却不给入口 -->
          <view
            v-for="e in elements"
            :key="e.key"
            class="elem-chip tap"
            @click="go('/pages/reactions/reactions')"
          >
            <view class="dot" :style="{ background: e.color }" />
            <text class="elem-name">{{ e.name }}</text>
          </view>
        </view>
        <text class="muted" style="display: block; margin-top: 12rpx">
          敌人对不同元素的抗性不同。两种元素叠加会触发反应链，反应伤害与面板攻击力无关 ——
          这是低养成玩家翻盘的主要来源。
        </text>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { useStatusBarPad } from '@/utils/statusBar'
import { computed, onMounted, ref, watch } from 'vue'
import { onShow } from '@dcloudio/uni-app'
import { onShareAppMessage } from '@dcloudio/uni-app'
import { useGameStore } from '@/store/game'
import * as api from '@/api/client'
import type { Announcement } from '@/api/client'
import { indexShare } from '@/utils/share'
import { ELEMENT_ORDER, ELEMENT_NAME, ELEMENT_COLOR } from '@/game/elements'
import Icon from '@/components/Icon.vue'

const store = useGameStore()

/** R42 资源数值变化高亮：钱包变动的胶囊短促放大，玩家能确认「奖励到账了」。 */
const bump = ref<Record<string, boolean>>({ coin: false, gem: false, energy: false })
let bumpTimer: ReturnType<typeof setTimeout> | null = null
watch(
  () => [store.wallet.coin, store.wallet.gem, store.wallet.energy],
  (nv, ov) => {
    const keys = ['coin', 'gem', 'energy']
    const changed = keys.filter((_, i) => nv[i] !== ov[i])
    if (!changed.length) return
    const next: Record<string, boolean> = { coin: false, gem: false, energy: false }
    for (const k of changed) next[k] = true
    bump.value = next
    if (bumpTimer) clearTimeout(bumpTimer)
    bumpTimer = setTimeout(() => {
      bump.value = { coin: false, gem: false, energy: false }
      bumpTimer = null
    }, 700)
  },
)

/**
 * 公告（2026-10-23）。
 *
 * 拉取失败**不影响首页** —— 公告是次要内容，为它整页报错是错的取舍。
 * 但也不静默假装成功：失败时列表保持为空（不渲染空卡片），
 * 并在控制台留一条 warn。判据是「不显示假数据」，而不是「永不报错」。
 */
const announcements = ref<Announcement[]>([])
const expanded = ref<Set<number>>(new Set())

/**
 * 是否有请求在途。
 *
 * ⚠️ 光靠 `annLoadedOnce` 去重是不够的：它在**请求发出之前**就置位，
 * 于是首次请求在途时页面再 `onShow`（切后台回来、H5 路由抖动）会并发发出
 * 第二个请求 —— 首屏两次 `/announcements`。这个标志管的是「同时只跑一个」。
 */
let annInflight = false

async function loadAnnouncements(): Promise<void> {
  if (annInflight) return
  annInflight = true
  try {
    const res = await api.fetchAnnouncements(3)
    announcements.value = res.items ?? []
  } catch (e) {
    // ⚠️ 失败时**保留已有内容**，不清空。
    //
    // 清空会让一次网络抖动把玩家正在看的公告抹掉 —— 而「公告消失」
    // 看起来像运营撤了公告，不像网络问题。服务端不可达时首页本身
    // 会走 loginError 分支，这里只留一条 warn。
    console.warn('[公告] 拉取失败，保留已有内容：', (e as Error).message)
  } finally {
    annInflight = false
  }
}

function toggleAnn(id: number): void {
  const next = new Set(expanded.value)
  if (next.has(id)) next.delete(id)
  else next.add(id)
  expanded.value = next
}

/** 公告日期：只到「月-日」。玩家不关心秒，写全反而占位。 */
function formatAnnDate(iso: string | null): string {
  if (!iso) return ''
  const d = new Date(iso)
  if (Number.isNaN(d.getTime())) return ''
  return `${d.getMonth() + 1}-${String(d.getDate()).padStart(2, '0')}`
}

/** 顶部让开状态栏（2026-10-21 P6-2）。 */
const { statusBarPad } = useStatusBarPad()

const elements = ELEMENT_ORDER.map((k) => ({
  key: k,
  name: ELEMENT_NAME[k],
  color: ELEMENT_COLOR[k],
}))

/**
 * 首页入口（2026-11-16 分组）。
 *
 * 拆成「玩法」（操作型：去打、去花、去领）与「图鉴」（查阅型：看数值、看说明）
 * 两组。每组的条目数都能被 3 整除（9 / 6），3 列网格不再有残缺尾行。
 */
const playMenus = [
  { path: '/pages/stage/stage', label: '关卡', icon: 'target' },
  { path: '/pages/bag/bag', label: '背包', icon: 'bag' },
  // 商城（第 151 轮）。此前三种道具唯一的发放途径就是商城，
  // 而这里没有入口 —— 服务端扣减逻辑齐全，玩家却拿不到道具。
  { path: '/pages/shop/shop', label: '商城', icon: 'cart' },
  { path: '/pages/mastery/mastery', label: '专精', icon: 'mastery' },
  { path: '/pages/signin/signin', label: '签到', icon: 'calendar' },
  // 我的战报（2026-11-01 新一轮第 41 轮）：`/me/battles` 端点早已存在，
  // 客户端却没有任何战报历史入口 —— 打完就再也找不到那一局的记录。
  { path: '/pages/battles/battles', label: '战报', icon: 'scroll' },
  { path: '/pages/rank/rank', label: '排行', icon: 'trophy' },
  { path: '/pages/verify/verify', label: '验真', icon: 'verify' },
  { path: '/pages/me/me', label: '我的', icon: 'user' },
]

// 反应图鉴（2026-10-21 P4-2/P4-3）：`/config.reactions` 与
// `reachableReactions` 此前**零 UI 消费** —— 玩家查不到「我能打出哪些反应」。
// 敌人 / 技能 / 装备 / 宝石 / 皮肤图鉴同批（2026-10-31 新一轮第 24~35 轮），
// 把 `/config` 下发但零消费的权威表逐个接上 UI。
const codexMenus = [
  { path: '/pages/reactions/reactions', label: '反应', icon: 'flask' },
  { path: '/pages/enemies/enemies', label: '敌人', icon: 'ghost' },
  { path: '/pages/skills/skills', label: '技能', icon: 'sword' },
  { path: '/pages/equipment/equipment', label: '装备', icon: 'shield' },
  { path: '/pages/gems/gems', label: '宝石', icon: 'gem' },
  { path: '/pages/skins/skins', label: '皮肤', icon: 'palette' },
]

/**
 * 构筑评分五维进度。短板用醒目色，直接引导玩家去补。
 *
 * 分母来自服务端 `domain.BuildRating` 的各维上界：
 * 元素 0..5 / 反应 0..7 / 专精 0..8 / 装备契合 0..18 / 机制深度 0..8。
 * 第 138 轮修：① 此前只有 4 维（漏 mechanic_depth）；② equipment_synergy
 * 分母误写 6（真实上界 18），导致契合度条形永远虚高（6/18 就显示满格）。
 *
 * ⚠️ 第 150 轮：`rating_weights` 此前**下发但零消费**。
 * 它是服务端 `domain.DefaultRatingWeights()` 唯一的跨端载体
 * （total = Σ(维度 × 权重)）。之前五维权重只存在于服务端，
 * 客户端把「维度值 ÷ 硬编码上界」画成进度条 —— 上界是注释里的常量，
 * 而权重一个都没用上。于是运营在后台调权重时，
 * 玩家看到的五维条形**完全不动**：那是一个真实存在却接了半根线的字段。
 *
 * 现在权重也消费掉：每维展示它在 total 里占的**权重份额**，
 * 于是「哪一维更值钱」与「我这一维差多少」都能从同一处看出来。
 */
const dims = computed(() => {
  const r = store.buildRating
  const w = store.config?.rating_weights
  const wOf = (k: keyof NonNullable<typeof w>): number => Number(w?.[k] ?? 0)
  const weightSum =
    wOf('element_coverage') +
    wOf('reaction_coverage') +
    wOf('mastery_done') +
    wOf('equipment_synergy') +
    wOf('mechanic_depth')

  return [
    {
      label: '元素覆盖',
      value: `${r?.element_coverage ?? 0}/5`,
      pct: ((r?.element_coverage ?? 0) / 5) * 100,
      color: 'var(--ice)',
      weight: wOf('element_coverage'),
    },
    {
      label: '反应链',
      value: `${r?.reaction_coverage ?? 0}/7`,
      pct: ((r?.reaction_coverage ?? 0) / 7) * 100,
      color: 'var(--corrosion)',
      weight: wOf('reaction_coverage'),
    },
    {
      label: '专精投入',
      value: `${r?.mastery_done ?? 0}/8`,
      pct: ((r?.mastery_done ?? 0) / 8) * 100,
      color: 'var(--kinetic)',
      weight: wOf('mastery_done'),
    },
    {
      label: '装备契合',
      value: `${r?.equipment_synergy ?? 0}/18`,
      pct: Math.min(100, ((r?.equipment_synergy ?? 0) / 18) * 100),
      color: 'var(--lightning)',
      weight: wOf('equipment_synergy'),
    },
    {
      label: '机制深度',
      value: `${r?.mechanic_depth ?? 0}/8`,
      pct: ((r?.mechanic_depth ?? 0) / 8) * 100,
      color: 'var(--bad)',
      weight: wOf('mechanic_depth'),
    },
  ].map((d) => {
    const hint = DIM_HINT[d.label]
    return {
      ...d,
      /** 权重份额（%）。权重全为 0 或 config 未到时给 0，不做除零兜底成假数字。 */
      weightPct: weightSum > 0 ? Math.round((d.weight / weightSum) * 100) : 0,
      // 空维补导入口（100 轮打磨 R14）；该 label 未登记则不给入口，只显示「待培养」
      hint: hint?.path,
      hintLabel: hint?.label,
    }
  })
})

const weaknesses = computed<string[]>(() => store.buildRating?.weaknesses ?? [])

/** G87 闯关进度环：已解锁关数 / 总关数的 SVG 弧（hero 卡里的文字升级为可视化）。 */
const stageRing = computed(() => {
  const cap = store.config?.levels?.length ?? 0
  const done = Math.max(0, Math.min(cap, store.unlockedLevel ?? 0))
  const pct = cap > 0 ? done / cap : 0
  const C = 2 * Math.PI * 36
  return { done, cap, pct: Math.round(pct * 100), dash: pct * C, rest: C - pct * C }
})

/* -------------------------------------------------------------------------- */
/* R61 构筑五维雷达图                                                           */
/* 五边形：中心 (100,90)，半径 70；从正上方起顺时针五个维度。                    */
/* 纯前端几何，数据来自 dims[].pct，不改任何 api/store 契约。                    */
/* -------------------------------------------------------------------------- */
const RADAR_CX = 100
const RADAR_CY = 90
const RADAR_R = 70
/** 五个顶点方向（从正上方起，顺时针每 72°）。 */
const radarDirs = Array.from({ length: 5 }, (_, i) => {
  const a = -Math.PI / 2 + (i * Math.PI * 2) / 5
  return { dx: Math.cos(a), dy: Math.sin(a) }
})
/** 参考网格圈（比例 0.35 / 0.7 / 1）。 */
const radarRings = [0.35, 0.7, 1]
/** 轴线端点。 */
const radarAxisEnds = radarDirs.map((d) => ({
  x: RADAR_CX + d.dx * RADAR_R,
  y: RADAR_CY + d.dy * RADAR_R,
}))
/** 顶点标签（放在轴线外侧一点）。 */
const radarLabels = computed(() =>
  radarDirs.map((d, i) => ({
    x: RADAR_CX + d.dx * (RADAR_R + 12),
    y: RADAR_CY + d.dy * (RADAR_R + 12) + 4,
    text: (dims.value[i]?.label ?? '').replace('覆盖', '').replace('投入', '').replace('契合', '').replace('深度', ''),
    // text-anchor 由 x 相对中心决定
  })),
)
function radarPolygon(scale: number): string {
  return radarDirs
    .map((d) => `${RADAR_CX + d.dx * RADAR_R * scale},${RADAR_CY + d.dy * RADAR_R * scale}`)
    .join(' ')
}
const radarPolygonActual = computed(() =>
  radarDirs
    .map((d, i) => {
      const pct = Math.max(0, Math.min(100, dims.value[i]?.pct ?? 0)) / 100
      return `${RADAR_CX + d.dx * RADAR_R * pct},${RADAR_CY + d.dy * RADAR_R * pct}`
    })
    .join(' '),
)

/**
 * 五维 → 补导入口（100 轮打磨 R14）。
 *
 * 维度 label 是服务端下发的中文（评级口径，不客户端拼），
 * 但「缺了这一维该去哪补」是纯客户端导航，二者分开维护不冲突。
 * 0 分的维度给一个明确入口，把死灰条变成可点的下一步。
 */
const DIM_HINT: Record<string, { path: string; label: string }> = {
  元素覆盖: { path: '/pages/bag/bag', label: '去背包配元素' },
  反应链: { path: '/pages/reactions/reactions', label: '看反应搭配' },
  专精投入: { path: '/pages/mastery/mastery', label: '去点亮专精' },
  装备契合: { path: '/pages/equipment/equipment', label: '去配装备' },
  机制深度: { path: '/pages/bag/bag', label: '去装机制卡' },
}

function go(path: string) {
  uni.navigateTo({ url: path })
}

/** 公告列表页（2026-11-01 新一轮第 42 轮）。 */
function goAnnouncements() {
  uni.navigateTo({ url: '/pages/announcements/announcements' })
}

function goStage() {
  uni.navigateTo({ url: `/pages/stage/stage?level=${store.unlockedLevel}` })
}

async function retry() {
  await store.login()
}

/**
 * 首次进入是否已拉过公告。
 *
 * uni-app 的顺序是 onLoad → onShow → mounted，所以首次进入时 `onShow`
 * 早于 `onMounted`。用它区分「首次」与「从别的页面返回」：
 * 首次由 onMounted 拉一次，返回时才由 onShow 刷新 —— 否则首屏会发两次请求。
 */
let annLoadedOnce = false

onMounted(async () => {
  if (!store.loggedIn) await store.login()
  annLoadedOnce = true
  void loadAnnouncements()
})

onShow(async () => {
  if (store.loggedIn) await store.refreshProfile()
  if (annLoadedOnce) void loadAnnouncements()
})

/**
 * 分享（2026-10-23）。
 *
 * 此前全仓 0 处 `onShareAppMessage` —— 微信小程序**唯一免费获客通道**缺失。
 * 文案与路径由 `utils/share.ts` 生成，便于在不需要挂载页面的情况下断言。
 */
onShareAppMessage(() => indexShare())
</script>

<style scoped>
.home {
  /*
   * 顶栏现在**含状态栏**（2026-10-21 P6-2），高度 = 状态栏 + 72rpx 头像 + 内边距。
   * 而这里的偏移是**写死的 100rpx** —— 顶栏一长，构筑评分卡就被盖住了。
   *
   * 修法：顶栏改成 sticky（仍固定在顶部，但**在文档流里占位**），
   * 内容自然从它下面开始，写死的偏移就不需要了。
   */
  padding-top: 0;
}

.topbar {
  position: sticky;
  top: 0;
  left: 0;
  right: 0;
  z-index: 10;
  background: var(--panel-2);
  border-bottom: 1rpx solid var(--border);
  padding: 20rpx 24rpx;
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
}

.player {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 16rpx;
}

.avatar {
  width: 72rpx;
  height: 72rpx;
  border-radius: 16rpx;
  background: var(--accent-grad);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 34rpx;
  font-weight: 700;
  color: #fff;
}

.player-info {
  display: flex;
  flex-direction: column;
}

.nick {
  font-size: 28rpx;
  font-weight: 600;
}

.res-row {
  display: flex;
  flex-direction: row;
  gap: 12rpx;
  white-space: nowrap;
}
.res-row :deep(.res-pill svg) {
  color: var(--fire);
}

.score {
  font-size: 48rpx;
  font-weight: 700;
  color: var(--fire);
}

/* G87 闯关进度环 */
.stage-ring-wrap { position: relative; width: 96rpx; height: 96rpx; flex-shrink: 0; margin-right: 12rpx; }
.stage-ring { width: 96rpx; height: 96rpx; }
.stage-ring .sr-bg { fill: none; stroke: rgba(255,255,255,0.15); stroke-width: 8; }
.stage-ring .sr-fill { fill: none; stroke: #fff; stroke-width: 8; stroke-linecap: round; }
.stage-ring-label {
  position: absolute; top: 50%; left: 50%;
  transform: translate(-50%, -50%);
  font-size: 22rpx; font-weight: 700; color: #fff;
}

/* R61 构筑五维雷达图 */
.radar-wrap {
  display: flex;
  justify-content: center;
  margin: 8rpx 0 24rpx;
}
.radar-svg {
  width: 400rpx;
  height: 360rpx;
  overflow: visible;
}
.radar-ring {
  fill: none;
  stroke: var(--border-soft, var(--border));
  stroke-width: 1;
}
.radar-axis {
  stroke: var(--border-soft, var(--border));
  stroke-width: 1;
}
.radar-shape {
  fill: rgba(255, 107, 53, 0.22);
  stroke: var(--fire);
  stroke-width: 2;
  transition: all 0.3s ease;
}
.radar-label {
  fill: var(--muted);
  font-size: 11px;
}

.dim-row {
  margin-bottom: 18rpx;
}

/* 每维：上行标题+权重、中部进度条、数值+补导。垂直布局让空维的补导有处安放，
   不再是一条挤在 393rpx 里的死灰条（100 轮打磨 R14）。 */
.dim-row-head,
.dim-row-foot {
  display: flex;
  flex-direction: row;
  align-items: center;
}

.dim-row-head {
  justify-content: space-between;
  margin-bottom: 6rpx;
}

.dim-row-foot {
  justify-content: space-between;
  margin-top: 6rpx;
}

.dim-label {
  font-size: 24rpx;
  color: var(--muted);
}

.bar {
  width: 100%;
  height: 12rpx;
  background: var(--panel-3);
  border-radius: 6rpx;
  overflow: hidden;
}

.bar-fill {
  height: 100%;
  border-radius: 7rpx;
  transition: width 0.4s ease-out;
}

.dim-value {
  font-size: 24rpx;
  color: var(--text-2);
}

.dim-weight {
  font-size: 22rpx;
  color: var(--dim);
}

/* 空维补导链接：紧凑小链接，点进去补这一维 */
.dim-hint {
  font-size: 22rpx;
  padding: 0;
  min-height: auto;
}

/* 无补导的空维文案 */
.dim-empty-hint {
  font-size: 22rpx;
  color: var(--dim);
}

.weakness-box {
  margin-top: 20rpx;
  padding: 18rpx;
  background: rgba(255, 107, 53, 0.08);
  border-left: 4rpx solid var(--fire);
  border-radius: 8rpx;
}

.weakness-title {
  font-size: 22rpx;
  color: var(--fire);
  font-weight: 600;
  display: block;
  margin-bottom: 8rpx;
}

.weakness-item {
  font-size: 22rpx;
  color: var(--muted);
  line-height: 1.7;
  display: block;
}

.action-card {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 16rpx;
  border-radius: var(--radius-lg);
  padding: 30rpx 28rpx;
  margin-bottom: 24rpx;
  box-shadow: var(--shadow-2);
}

.action-kicker {
  font-size: 22rpx;
  color: var(--fire);
  letter-spacing: 2rpx;
  display: block;
  margin-bottom: 6rpx;
}

.action-title {
  font-size: 38rpx;
  font-weight: 700;
  display: block;
}

.action-sub {
  font-size: 24rpx;
  color: var(--muted);
  display: block;
  margin-top: 8rpx;
}

/* R62 通关进度提示 */
.action-progress {
  display: block;
  margin-top: 8rpx;
  font-size: 22rpx;
  color: var(--ok);
}

.action-go {
  flex-shrink: 0;
  display: inline-flex;
  align-items: center;
  gap: 6rpx;
  white-space: nowrap;
  height: 88rpx;
  padding: 0 30rpx;
  border-radius: 999rpx;
  background: var(--accent-grad);
  color: #fff;
  font-size: var(--fs-md);
  font-weight: 700;
  box-shadow: 0 6rpx 18rpx rgba(255, 107, 53, 0.35);
}
/* uni-text 的 span 自带 pre-line，会把「开始」按字折行 —— 显式关掉 */
.action-go :deep(span) {
  white-space: nowrap;
}

/* 分组标题（2026-11-16）：玩法 / 图鉴 两组入口 */
.grid-group {
  margin-bottom: 8rpx;
}

.grid-group-head {
  display: flex;
  align-items: baseline;
  gap: 12rpx;
  margin-bottom: 14rpx;
}

.grid-group-title {
  font-size: 24rpx;
  color: var(--dim);
  letter-spacing: 2rpx;
  position: relative;
  padding-left: 20rpx;
}
.grid-group-title::before {
  content: '';
  position: absolute;
  left: 0;
  top: 50%;
  transform: translateY(-50%);
  width: 8rpx;
  height: 24rpx;
  border-radius: 4rpx;
  background: var(--accent-grad);
}

.grid-group-count {
  font-size: 22rpx;
  color: var(--dim);
}

.grid {
  /* 2026-11-16 v2：flex+calc(33.333%-12rpx) 的组合在真机上因亚像素舍入溢出
     <0.01px，第 3 项被挤到下一行 → 整页变两列。CSS Grid 的轨道计算不产生
     这类溢出，minmax(0,1fr) 同时禁止内容撑破列宽。 */
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 16rpx;
  margin-bottom: 20rpx;
}

.grid-item {
  background: var(--panel);
  border: 1rpx solid var(--border-soft);
  border-radius: var(--radius);
  padding: 24rpx 0 20rpx;
  display: flex;
  flex-direction: column;
  align-items: center;
  box-shadow: var(--shadow-1);
}

.grid-icon {
  color: var(--text-2);
  transition: transform 0.14s, color 0.14s;
}

/* R63 入口图标按下微抬，强化「点到了」 */
.grid-item:active .grid-icon {
  transform: translateY(-4rpx) scale(1.08);
  color: var(--fire);
}

.grid-label {
  font-size: 24rpx;
  color: var(--muted);
  margin-top: 8rpx;
}

.elem-row {
  display: flex;
  flex-direction: row;
  gap: 24rpx;
  flex-wrap: wrap;
}

.elem-chip {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 8rpx;
}

.dot {
  width: 20rpx;
  height: 20rpx;
  border-radius: 50%;
}

.elem-name {
  font-size: 24rpx;
  color: var(--muted);
}

/*
 * 公告。
 *
 * 折叠用 `-webkit-line-clamp` 而不是截断字符串：
 * 截断会改内容（玩家看到的是残缺的正文），折叠只是视觉收起，展开后仍是原文。
 * 小程序 WebView 与 Skyline 都支持该属性。
 */
.ann-item {
  padding: 16rpx 0;
  border-top: 1rpx solid rgba(255, 255, 255, 0.06);
}

.ann-item:first-of-type {
  border-top: none;
}

.ann-head {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  align-items: baseline;
  gap: 16rpx;
}

/* 第六批 R55 首页公告「最新」徽章（与公告页同语言） */
.ann-new {
  font-size: 20rpx;
  color: var(--fire);
  border: 1rpx solid var(--fire);
  border-radius: 6rpx;
  padding: 0 8rpx;
  flex-shrink: 0;
}

.ann-title {
  font-size: 28rpx;
  font-weight: 600;
  color: var(--text);
  flex: 1;
}

/* R64 公告展开指示箭头 */
.ann-toggle {
  font-size: 28rpx;
  color: var(--dim);
  transition: transform 0.2s;
}

.ann-toggle.open {
  transform: rotate(180deg);
  color: var(--fire);
}

/* 全部公告入口（2026-11-01 新一轮第 42 轮） */
.ann-all {
  font-size: 22rpx;
  color: var(--fire);
  padding: 4rpx 0 4rpx 16rpx;
}

.ann-body {
  display: -webkit-box;
  -webkit-box-orient: vertical;
  overflow: hidden;
  font-size: 24rpx;
  line-height: 1.6;
  color: var(--muted);
  margin-top: 8rpx;
  white-space: pre-wrap;
}

.ann-body.clamped {
  -webkit-line-clamp: 2;
}
</style>
