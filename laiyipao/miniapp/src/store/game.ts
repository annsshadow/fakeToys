/**
 * 全局状态：账号、配置、钱包、进度。
 *
 * 设计取舍：配置（100 关 + 42 技能 + 22 敌人）一次全量拉取并常驻内存。
 * 它是纯只读数据，总量约 60KB 压缩后，一次请求比七次增量更简单也更快，
 * 且关卡数据不常变。
 */
import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

import * as api from '@/api/client'
import type { GameConfig } from '@/api/client'
import { setTokenInternal } from '@/api/client'
import { defaultAttacker, type Attacker } from '@/game/damage'
import type { EnemyDef, GeneratedLevel, SkillDef } from '@/game/types'
import type { Element } from '@/game/elements'

export const useGameStore = defineStore('game', () => {
  // ---- 账号 ----
  const userId = ref<number>(0)
  const nickname = ref('')
  const isGuest = ref(true)
  const loggedIn = ref(false)
  const loginError = ref('')

  // ---- 配置 ----
  const config = ref<GameConfig | null>(null)
  const configLoading = ref(false)

  // ---- 服务端时钟基准（第 140 轮）----
  // /config 的 server_time 是服务端当时的挂钟（RFC3339）。客户端本地时钟可能
  // 偏数小时；拿本地 Date.now() 去比服务端的 shielded_until 会判错护盾状态。
  // 用「服务端响应时刻 + 自收到响应以来经过的时间」抵消本地时钟的**偏移**，
  // 残留误差只有会话期间的漂移 —— 远好于固定几小时的偏差。
  let serverTimeMs: number | null = null
  let serverTimeReceivedAtMs = 0

  // ---- 钱包与进度 ----
  const wallet = ref({ coin: 0, gem: 0, energy: 0, keys: 0 })
  const maxStage = ref(0)
  const power = ref(0)
  const buildRating = ref<any>(null)
  /** 服务端下发的构筑快照，含权威 attacker 属性 */
  const build = ref<any>(null)
  /**
   * 出战技能（本地 UI 乐观值）。
   *
   * ⚠️ 第 131 轮：初值改为**空数组**，不再硬编码 [1,2,3]。
   * 服务端对一个从未保存过槽位的玩家，`Loadout` 返回 `[0,0,0,0]`
   * （全空）—— 本地若预填 1/2/3，就会在 me 页把「玩家没装过的技能」
   * 写进防线快照（见 me.vue 的 saveDefense/myEquipped）。
   * 空值 = 「尚未从服务端加载」的诚实状态，由各消费页先 loadLoadout() 再使用。
   */
  const equippedSkillIds = ref<number[]>([])

  const enemyMap = computed(() => {
    const m = new Map<number, EnemyDef>()
    for (const e of config.value?.enemies ?? []) m.set(e.id, e)
    return m
  })

  const skillMap = computed(() => {
    const m = new Map<number, SkillDef>()
    for (const s of config.value?.skills ?? []) m.set(s.id, s)
    for (const s of config.value?.composite_skills ?? []) m.set(s.id, s)
    return m
  })

  const levelMap = computed(() => {
    const m = new Map<number, GeneratedLevel>()
    for (const l of config.value?.levels ?? []) m.set(l.id, l)
    return m
  })

  /** 已解锁关卡 = maxStage + 1（第 1 关恒可用） */
  const unlockedLevel = computed(() => Math.min(100, Math.max(1, maxStage.value + 1)))

  /**
   * 当前构筑的攻方属性。
   *
   * ⚠️ 必须来自服务端（`build.attacker`），**绝不能本地推算**。
   * 回放哈希由「关卡 + 种子 + 攻方属性 + 技能」共同决定；
   * 两端各算一套的话哈希必然不同，I-6 的验真会把所有正常对局判成伪造。
   * 本地推算只在服务端还没下发时作为占位，且会在控制台明确告警。
   */
  const attacker = computed<Attacker>(() => {
    const raw = (build.value as any)?.attacker
    if (raw && typeof raw.attack === 'number') {
      return {
        attack: BigInt(raw.attack),
        critPermille: BigInt(raw.crit_permille),
        critMultiplierPermille: BigInt(raw.crit_multiplier_permille),
        reactionMultPermille: BigInt(raw.reaction_mult_permille),
        elementCap: BigInt(raw.element_cap),
        reactionTier: BigInt(raw.reaction_tier),
        elementCoefPermille: BigInt(raw.element_coef_permille),
        // 三项本轮新增：防线护甲 / 热量上限 / 机制卡强度。
        //
        // ⚠️ 用 `?? 0` 而不是直接 BigInt(undefined)：
        // BigInt(undefined) 抛 TypeError，而 `raw` 是 `any`（服务端 JSON），
        // 老版本服务端的 build_snapshot 里没有这三个字段 ——
        // 缺省成 0 正好等价于"没有装备/宝石/专精"。
        heatCapPermille: BigInt(raw.heat_cap_permille ?? 0),
        armorPermille: BigInt(raw.armor_permille ?? 0),
        mechanicPermille: BigInt(raw.mechanic_permille ?? 0),
      }
    }
    console.warn(
      '[来一炮] build.attacker 缺失，正在使用本地占位属性。' +
        '此时战斗能跑但回放哈希会与服务端记录不符 —— I-6 验真会误判为伪造。',
    )
    return defaultAttacker()
  })

  /** 出战槽位（按位置排列，0 表示空槽）。服务端是权威来源。 */
  const loadout = ref<number[]>([0, 0, 0, 0])

  /** 已装备技能的元素（用于界面显示搭配覆盖） */
  const equippedElements = computed<Element[]>(() => {
    const out: Element[] = []
    for (const id of loadout.value) {
      if (!id) continue
      const s = skillMap.value.get(id)
      if (s) out.push(s.element)
    }
    return out
  })

  /**
   * 登录中使用的 in-flight Promise。
   *
   * ⚠️ 必须做单飞（single-flight）保护：onLoad 与 onMounted 都会调 login()，
   * 并发执行会产生两个游客账号、两个 token 互相覆盖 ——
   * 表现为"钱包显示体力 30，但开战斗报体力不足"，极难排查。
   */
  let loginInflight: Promise<boolean> | null = null

  async function login(): Promise<boolean> {
    if (loginInflight) return loginInflight
    loginInflight = doLogin()
    try {
      return await loginInflight
    } finally {
      loginInflight = null
    }
  }

  async function doLogin(): Promise<boolean> {
    loginError.value = ''
    try {
      const tp = await api.guestLogin()
      setTokenInternal(tp.access_token, tp.refresh_token)
      userId.value = tp.user.id
      nickname.value = tp.user.nickname
      isGuest.value = tp.user.is_guest
      loggedIn.value = true
      await Promise.all([loadConfig(), refreshProfile()])
      return true
    } catch (e) {
      loginError.value = (e as Error).message
      loggedIn.value = false
      return false
    }
  }

  async function loadConfig(): Promise<void> {
    if (config.value) return // 已加载
    configLoading.value = true
    try {
      const cfg = await api.fetchConfig()
      config.value = cfg
      // 记下服务端时钟基准：响应里的 server_time + 收到响应的本地时刻。
      // 本地时钟偏移由后续 estimateServerNowMs() 抵消。
      const t = Date.parse(cfg.server_time ?? '')
      if (Number.isFinite(t)) {
        serverTimeMs = t
        serverTimeReceivedAtMs = Date.now()
      }
    } finally {
      configLoading.value = false
    }
  }

  /**
   * 估算「当前服务端时间」（毫秒）。
   *
   * 有服务端时钟基准时返回 `serverTimeMs + (Date.now() - serverTimeReceivedAtMs)`
   * —— 抵消本地时钟偏移；没有（config 未加载 / server_time 缺失或不可解析）时
   * 诚实回退到本地时钟。
   *
   * ⚠️ 消费方：凡是拿服务端的绝对时间字段（如 shielded_until）做比较的地方
   * 都应用它，而不是裸 Date.now()。
   */
  function estimateServerNowMs(): number {
    if (serverTimeMs !== null) {
      return serverTimeMs + (Date.now() - serverTimeReceivedAtMs)
    }
    return Date.now()
  }

  async function refreshProfile(): Promise<void> {
    try {
      const [w, me] = await Promise.all([api.fetchWallet(), api.fetchMe()])
      wallet.value = w
      power.value = me.power
      buildRating.value = me.build_rating
      build.value = me.build
    } catch (e) {
      // 静默：页面会展示已有数据，不因刷新失败而白屏
      console.warn('[来一炮] 刷新玩家信息失败', (e as Error).message)
    }
  }

  async function refreshWallet(): Promise<void> {
    try {
      wallet.value = await api.fetchWallet()
    } catch {
      /* 忽略 */
    }
  }

  function setEquippedSkills(ids: number[]): void {
    equippedSkillIds.value = ids.slice(0, 4)
  }

  /**
   * 加载服务端权威的出战槽位。
   *
   * ⚠️ 槽位必须以服务端为准 —— 它参与回放哈希计算。
   * 前端本地那份（equippedSkillIds）只是 UI 乐观值，
   * 真正决定战斗用的是这里的 loadout。
   */
  async function loadLoadout(): Promise<void> {
    try {
      const res = await api.fetchLoadout()
      const ids = (res.skill_ids ?? []).slice(0, 4)
      while (ids.length < 4) ids.push(0)
      loadout.value = ids
      equippedSkillIds.value = ids.filter((x) => x > 0)
    } catch (e) {
      console.warn('[来一炮] 加载出战配置失败，回退到本地值', (e as Error).message)
    }
  }

  /** 保存出战槽位到服务端。返回是否成功（失败时 UI 应提示） */
  async function persistLoadout(ids: number[]): Promise<boolean> {
    const normalized = ids.slice(0, 4)
    while (normalized.length < 4) normalized.push(0)
    try {
      const res = await api.saveLoadout(normalized)
      loadout.value = (res.skill_ids ?? normalized).slice(0, 4)
      equippedSkillIds.value = loadout.value.filter((x) => x > 0)
      return true
    } catch (e) {
      console.warn('[来一炮] 保存出战配置失败', (e as Error).message)
      return false
    }
  }

  /** 开局时用服务端下发的构筑快照刷新（含权威 attacker 属性） */
  function setBuild(b: unknown): void {
    if (b && typeof b === 'object') build.value = b
  }

  function setMaxStage(n: number): void {
    if (n > maxStage.value) maxStage.value = n
  }

  return {
    userId,
    nickname,
    isGuest,
    loggedIn,
    loginError,
    config,
    configLoading,
    wallet,
    maxStage,
    power,
    buildRating,
    build,
    equippedSkillIds,
    loadout,
    enemyMap,
    skillMap,
    levelMap,
    unlockedLevel,
    attacker,
    equippedElements,
    login,
    loadConfig,
    estimateServerNowMs,
    refreshProfile,
    refreshWallet,
    setEquippedSkills,
    setBuild,
    loadLoadout,
    persistLoadout,
    setMaxStage,
  }
})
