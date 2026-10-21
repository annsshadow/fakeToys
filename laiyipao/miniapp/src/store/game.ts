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
import type { EnemyDef, LevelSummary, SkillDef } from '@/game/types'
import type { Element } from '@/game/elements'

// ---- 配置持久化（第 150 轮）----
//
// 配置包实测未压缩约 168KB，`levels` 一个字段就占 123KB（74%）。
// 加上 storage 后，冷启动先用本地副本点亮界面，再用条件请求确认版本 ——
// 内容没变时服务端只回 304，**零字节载荷**。
//
// ⚠️ 存储键带 `_v1` 后缀：将来若缓存结构变了（比如拆成多个键），
// 改后缀就能让旧缓存自然失效，不需要写迁移代码。
const CONFIG_CACHE_KEY = 'lyp_config_cache_v1'

interface ConfigCache {
  version: string
  config: GameConfig
}

/**
 * 读持久化配置。
 *
 * 任何不完整/不可解析的缓存都返回 null（宁可重拉）：
 * 半份配置会让界面显示**部分**内容，而那看起来像是服务端数据不全，
 * 会把人引向完全错误的排查方向。
 */
function readConfigCache(): ConfigCache | null {
  try {
    const raw = uni.getStorageSync(CONFIG_CACHE_KEY)
    if (typeof raw !== 'string' || !raw) return null
    const parsed = JSON.parse(raw) as ConfigCache | null
    if (!parsed || typeof parsed.version !== 'string' || !parsed.config) return null
    return parsed
  } catch {
    return null
  }
}

function writeConfigCache(version: string, cfg: GameConfig): void {
  try {
    uni.setStorageSync(
      CONFIG_CACHE_KEY,
      JSON.stringify({ version, config: cfg } satisfies ConfigCache),
    )
  } catch {
    /* 存储不可用时退化为只缓存内存（本次会话有效）—— 与令牌存取同一策略 */
  }
}

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
  // tokens 是道具余额（第 150 轮起由 /wallet 下发）。
  // 初值为空对象而不是不发：消费方用 `tokens['mastery_reset'] ?? 0`，
  // 两种缺省都指向 0，语义一致。
  const wallet = ref<{
    coin: number
    gem: number
    energy: number
    keys: number
    tokens: Record<string, number>
  }>({ coin: 0, gem: 0, energy: 0, keys: 0, tokens: {} })
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
    // 值是**摘要**（无 waves）：/config 不再下发波次 —— 它占整个配置包 56%，
    // 而客户端只有防线挑战一处需要。那一处走 `api.fetchLevel()`。
    const m = new Map<number, LevelSummary>()
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

  /**
   * 本环境是否要求账号登录。
   *
   * 来自服务端 `/config` 的 `require_login`（`PLAYER_LOGIN_REQUIRED`）。
   * 开启后：不再自动游客建号，未登录时跳登录页。
   *
   * ⚠️ 配置副本可能过期（这个字段不在内容哈希里），
   * 所以它只是「提前知道」，权威判据是服务端对 `/auth/guest` 的
   * 403（code=login_required）—— 见 doLogin 的 catch 分支。
   */
  const loginRequired = computed(() => config.value?.require_login === true)

  /** 需要跳登录页（页面据此渲染「请先登录」而不是空白/报错）。 */
  const needLogin = ref(false)

  /**
   * 跳登录页。
   *
   * 用 `reLaunch` 而不是 `navigateTo`：登录页是**闸门** ——
   * 栈里若留着需要鉴权的页面，返回键会把用户带回一个
   * 「没有数据 / 401」的页面，而那正是最让人困惑的形态。
   *
   * 已经站在登录页上就不再跳（否则 12 个调用点会互相踩）。
   */
  function gotoLoginPage(): void {
    // ⚠️ `getCurrentPages` 在真机上是全局函数，但测试环境里可能不存在
    // （jsdom 里没有 uni 运行时）—— 用 typeof 守卫，
    // 缺失时**跳过**「已经在登录页」的判据，而不是崩掉。
    if (typeof getCurrentPages === 'function') {
      const pages = getCurrentPages()
      const cur = pages[pages.length - 1]
      if (cur && cur.route === 'pages/login/login') return
    }
    uni.reLaunch({ url: '/pages/login/login' })
  }

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
      // ① 已有可用令牌（口令登录后的冷启动）→ 直接用，**别再打 /auth/guest**。
      //    这一步是「有账号」与「每设备一个游客号」的分水岭：
      //    调 /auth/guest 会建出一个**新的游客号**，
      //    于是「我刚打的关卡呢」又回来了。
      //
      //    ⚠️ 昵称取自上次登录时存下的身份摘要（api.loadProfile）——
      //    口令登录不会像游客登录那样每次回一个 user 对象。
      if (api.hasUsableToken()) {
        const prof = api.loadProfile()
        nickname.value = prof?.nickname ?? ''
        isGuest.value = prof?.isGuest === true
        loggedIn.value = true
        await Promise.all([loadConfig(), refreshProfile()])
        return true
      }
      // ② 游客通道。
      const tp = await api.guestLogin()
      setTokenInternal(tp.access_token, tp.refresh_token, tp.expires_at)
      api.saveProfile({ nickname: tp.user.nickname, isGuest: tp.user.is_guest })
      userId.value = tp.user.id
      nickname.value = tp.user.nickname
      isGuest.value = tp.user.is_guest
      loggedIn.value = true
      await Promise.all([loadConfig(), refreshProfile()])
      return true
    } catch (e) {
      // 服务端拒绝游客建号（开关打开）→ 去登录页，而不是显示一句
      // 「游客登录失败」让人以为是网络问题。
      //
      // ⚠️ 这是**权威**判据：`/config` 的 require_login 不在内容哈希里，
      // 客户端本地缓存的配置副本可能带着旧值；
      // 服务端这道 403 则一定是当下的真实状态。
      if ((e as { code?: string })?.code === 'login_required') {
        needLogin.value = true
        gotoLoginPage()
        return false
      }
      loginError.value = (e as Error).message
      loggedIn.value = false
      return false
    }
  }

  /**
   * 账号口令登录。
   *
   * 成功后**不**再调 /auth/guest：账号登录拿到的是同一个账号，
   * 再打一次游客建号只会多造一个游客号。
   */
  async function loginWithPassword(username: string, password: string): Promise<boolean> {
    loginError.value = ''
    try {
      const tp = await api.login(username, password)
      setTokenInternal(tp.access_token, tp.refresh_token, tp.expires_at)
      api.saveProfile({ nickname: tp.user.nickname, isGuest: tp.user.is_guest })
      api.saveLastUsername(username)
      userId.value = tp.user.id
      nickname.value = tp.user.nickname
      isGuest.value = tp.user.is_guest
      loggedIn.value = true
      needLogin.value = false
      await Promise.all([loadConfig(), refreshProfile()])
      return true
    } catch (e) {
      loginError.value = (e as Error).message
      loggedIn.value = false
      return false
    }
  }

  /**
   * 加载配置。
   *
   * 原来这里只有 `if (config.value) return` 一个**内存内**缓存，
   * 于是每次冷启动都实打实拉一次 168KB（gzip 后 24KB），
   * 且 `config.version` 在整个客户端零引用 ——
   * 「版本没变就不用刷新」这个判断从来没被实现过。
   *
   * 第 150 轮接上完整闭环（服务端注释里早已写明需要这三件事一起做）：
   *   a. 服务端 `version` 改成内容哈希；
   *   b. 客户端把配置持久化到 storage；
   *   c. 拿持久化的 version 与服务端比对，相同就不替换、不重新解析。
   *
   * 于是冷启动变成：先用本地副本点亮界面（零等待、可按需离线），
   * 再带 `?if_version=` 问服务端 —— 内容没变时只回 304，**零载荷**。
   *
   * ⚠️ 这条路径能成立的前提是「配置真的是静态的」：关卡内容由生成器决定
   * （`domain.GenerateLevel` 纯函数，不查库），由服务端
   * `level_authority_test.go` 钉住玩家路径与 `levels` 表无关。
   */
  async function loadConfig(): Promise<void> {
    if (config.value) return // 已加载
    configLoading.value = true
    try {
      // ① 先用持久化副本点亮界面 —— 冷启动不再白屏等 168KB。
      const cached = readConfigCache()
      if (cached) {
        config.value = cached.config
        calibrateServerClock(Date.parse(cached.config.server_time ?? ''))
      }
      // ② 带 version 问服务端：没变 → 304，变了 → 200 全量。
      const res = await api.fetchConfigIfVersion(cached?.version ?? '')
      if (res.status === 'unmodified') {
        // 内容没变：本地副本就是真相。不替换、不重新 JSON.parse。
        // 时钟仍要校准 —— 304 没有响应体，只能靠 Date 头。
        calibrateServerClock(res.serverTimeMs)
        return
      }
      config.value = res.config
      writeConfigCache(res.config.version, res.config)
      // 200 路径优先用响应体 server_time（纳秒精度），
      // serverTimeMs 就是由它派生的；取不到时再回退到解析 config.server_time ——
      // 两层都失败就保持现有基准，不猜。
      calibrateServerClock(res.serverTimeMs || Date.parse(res.config.server_time ?? ''))
    } finally {
      configLoading.value = false
    }
  }

  /**
   * 用服务端时钟校准本地偏移。
   *
   * 接受**毫秒时间戳**（不是字符串），因为 304 路径拿不到响应体 ——
   * 那时服务端时间来自 HTTP 标准 `Date` 头（秒粒度）。
   *
   * 解析失败时**不覆盖**已有基准：宁可保留上一次的，也不要退化成
   * 本地时钟 —— 后者会让「护盾是否生效」的判断静默改变答案。
   */
  function calibrateServerClock(serverNowMs: number): void {
    if (!Number.isFinite(serverNowMs) || serverNowMs <= 0) return
    serverTimeMs = serverNowMs
    serverTimeReceivedAtMs = Date.now()
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
      wallet.value = { ...w, tokens: w.tokens ?? {} }
      // 第 150 轮：以 /me 的 user_id 为身份权威。
      // 此前 userId 只在 login() 里从 TokenPair.user.id 赋值，于是
      // **只要没走过 login() 就是 0** —— 排行榜页「高亮我自己那一行」
      // （rank.vue 比的是 it.user_id === store.userId）会静默失效。
      // /me 是当前登录态的权威快照，每次 refreshProfile 都会打，
      // 拿它回填比依赖「登录过一次」这个隐含前提可靠。
      if (typeof me.user_id === 'number' && me.user_id > 0) userId.value = me.user_id
      power.value = me.power
      buildRating.value = me.build_rating
      build.value = me.build
      // 第 142 轮：恢复服务端权威进度。修前 maxStage 只在**本会话结算后**
      // 才有值，老玩家重进 App 恒从 0 开始（解锁关卡 / 最高关卡显示全错位）。
      // setMaxStage 只增不减，不会压掉本会话刚结算出的更高值。
      if (typeof me.max_stage === 'number') setMaxStage(me.max_stage)
    } catch (e) {
      // 静默：页面会展示已有数据，不因刷新失败而白屏
      console.warn('[来一炮] 刷新玩家信息失败', (e as Error).message)
    }
  }

  async function refreshWallet(): Promise<void> {
    try {
      // tokens ?? {} 是对**旧版服务端**的兜底（没这个字段时不该让页面崩），
      // 不是对当前契约的容忍 —— 契约由 /wallet 的类型定义表达。
      const fresh = await api.fetchWallet()
      wallet.value = { ...fresh, tokens: fresh.tokens ?? {} }
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
    loginRequired,
    needLogin,
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
    loginWithPassword,
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
