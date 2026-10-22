<script setup lang="ts">
/**
 * 玩家玩法文档站。
 *
 * 为什么要放在运营后台里：它读的是同一份 /api/v1/config，
 * 因此技能数值、抗性、反应链说明永远与线上版本一致 ——
 * 不会出现"文档写的和游戏里不一样"这种最伤玩家信任的问题。
 *
 * ## 2026-10-22：目录 / 搜索 / 深链
 *
 * 三件事的共同前提是**同一份结构化索引**：目录项、搜索命中、深链目标
 * 说的都是「哪个 section + 哪个锚点 / 哪一行」。
 * 各写一套必然漂移 —— 改一个锚点 id，目录跳空、搜索跳空、旧分享链接也跳空，
 * 而且**三处都不会报错**（滚动到不存在的位置就是什么都不发生）。
 *
 * 所以：目录从 `SECTIONS` 派生，搜索从 `searchIndex` 派生，
 * 深链的 URL 契约（`?tab=&anchor=&focus=`）在 `targetFromQuery` 一处解析、
 * 在 `writeUrl` 一处写出。锚点 id 与模板里的 `id="..."` 是否对得上，
 * 由 `tests/WikiView.test.ts` 里那条「目录锚点都能在 DOM 里找到」的用例守。
 */
import { onMounted, onUnmounted, ref, computed, nextTick, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import { fetchDashboardSoft } from '@/api'

interface ConfigResp {
  levels: Array<{ id: number; chapter: number; name: string; is_boss: boolean; terrain: unknown[]; difficulty: number; base_hp: number; wave_count: number; energy_cost: number; element_cap: number; max_reaction_tier: number; star_targets: number[] }>
  enemies: Array<{ id: number; code: string; name: string; category: string; hp: number; resist: Record<string, number>; descr: string }>
  skills: Array<{ id: number; name: string; element: string; family: string; kind: string; base_damage: number; heat_cost: number; cooldown_ms: number; pierce: number; aoe_radius: number; apply_stacks: number; descr: string }>
  composite_skills: Array<{ id: number; name: string; element: string; descr: string }>
  // 装备目录（2026-10-21 P6-12）。
  //
  // ⚠️ 此前这个字段**全仓库零消费** —— 小程序端改从 `/me/equipment` 读
  // （目录 + 拥有状态），而文档站也一直没展示它。
  // 于是「玩家该穿什么、每件加多少」这个问题在**玩家面向的文档站上**
  // 没有答案，而契约守卫（正确地）报告它是死字段。
  //
  // 修法不是删字段，而是**把它用起来**：装备目录正是玩家要查的东西。
  equipment: Array<{
    id: number
    code: string
    name: string
    slot: string
    tier: number
    element: string
    descr: string
    base_armor: number
    base_bonus_pct: number
    unlock_level: number
  }>
  // ⚠️ 字段名必须与 Go 侧 json tag 逐字一致（snake_case）。
  // 这两行曾经写成 PascalCase，而服务端一直下发 snake_case，
  // 于是反应表整列为空、章节下拉框没有选项 —— 且不报任何错。
  reactions: Array<{ key: string; name: string; base_coef: number; attack_weight_pct: number; status_duration_ms: number; aoe_radius: number; dispel_shield: boolean; amplify_pct: number; descr: string }>
  chapters: Array<{ id: number; name: string; start_level: number; end_level: number; terrain_kind: string; boss_enemy_id: number }>
  rating_weights: Record<string, number>
  // ⚠️ 字段名与 Go 的 json tag 一律 snake_case（PascalCase 那些是笔误，
  // 全链路一律走 snake_case）。
  score_rules: {
    per_damage_unit: number
    on_kill_normal: number
    on_kill_boss: number
    star_target_ratio: number[]
    score_full_at_sec: number
  }
  skill_rules: {
    max_level: number
    coef_permille: number
    base_cost: number
  }
  mastery_families: Array<{ family: string; name: string; nodes: Array<{ name: string; layer: number }> }>
}

// ---------- 章节与目录 ----------

type SectionKey =
  | 'basics'
  | 'enemies'
  | 'skills'
  | 'equipment'
  | 'rules'
  | 'mastery'
  | 'levels'
  | 'live'

interface TocSection {
  key: SectionKey
  label: string
  /**
   * 页内小节锚点。
   *
   * ⚠️ `id` 必须与模板里的 `id="..."` **逐字一致**。
   * 对不上时页面不报错，只是点了没反应 —— 所以有测试守这一条。
   */
  anchors?: Array<{ id: string; label: string }>
}

/**
 * 目录。顺序即阅读顺序，也是 `?tab=` 的合法取值来源。
 *
 * 8 个 section 全部列在这里；模板里对应 8 个 `<section v-show>`。
 * 新增一节时两边都要加，漏了模板那边就是「目录点了没反应」，
 * 漏了这里就是「内容永远显示不出来」（activeTab 取不到那个 key）。
 */
const SECTIONS: TocSection[] = [
  {
    key: 'basics',
    label: '核心规则',
    anchors: [
      { id: 'reactions', label: '反应链一览' },
      { id: 'rating', label: '构筑评分' },
    ],
  },
  { key: 'enemies', label: '敌人抗性图鉴' },
  {
    key: 'skills',
    label: '技能图鉴',
    anchors: [
      { id: 'skill-table', label: '技能表' },
      { id: 'composite', label: '合成技能' },
    ],
  },
  { key: 'equipment', label: '装备图鉴' },
  {
    key: 'rules',
    label: '数值规则',
    anchors: [
      { id: 'score', label: '分数与星级' },
      { id: 'upgrade', label: '技能升级' },
    ],
  },
  { key: 'mastery', label: '专精树' },
  { key: 'levels', label: '关卡一览' },
  { key: 'live', label: '当前版本实战分布' },
]

const SECTION_KEYS = SECTIONS.map((s) => s.key)

function isSectionKey(v: string): v is SectionKey {
  return (SECTION_KEYS as string[]).includes(v)
}

// ---------- 深链 ----------

/** 搜索命中的实体类型。`focus` 参数的前缀就是它。 */
type HitKind = 'level' | 'enemy' | 'skill' | 'composite' | 'equip' | 'reaction' | 'mastery'

const HIT_KINDS: HitKind[] = ['level', 'enemy', 'skill', 'composite', 'equip', 'reaction', 'mastery']

const KIND_LABEL: Record<HitKind, string> = {
  level: '关卡',
  enemy: '敌人',
  skill: '技能',
  composite: '合成',
  equip: '装备',
  reaction: '反应',
  mastery: '专精',
}

/** 导航目标：一个 section，可选页内锚点 / 可选高亮某一行。 */
interface NavTarget {
  tab: SectionKey
  anchor?: string | null
  focus?: { kind: HitKind; id: string } | null
}

/**
 * 解析 `?focus=<kind>:<id>`。
 *
 * 用冒号而不是再加一个 `focus_kind` 参数：一行就能读完整个目标，
 * 分享出去的链接也更短（`#/wiki?tab=enemies&focus=enemy:12`）。
 * 非法值一律当「没有焦点」—— 深链是外部输入，不能假设它守规矩。
 */
function parseFocus(raw: unknown): { kind: HitKind; id: string } | null {
  if (typeof raw !== 'string') return null
  const i = raw.indexOf(':')
  if (i <= 0 || i === raw.length - 1) return null
  const kind = raw.slice(0, i)
  if (!(HIT_KINDS as string[]).includes(kind)) return null
  return { kind: kind as HitKind, id: raw.slice(i + 1) }
}

function targetFromQuery(q: Record<string, unknown>): NavTarget {
  const tab = String(q.tab ?? '')
  return {
    tab: isSectionKey(tab) ? tab : 'basics',
    anchor: typeof q.anchor === 'string' && q.anchor ? q.anchor : null,
    focus: parseFocus(q.focus),
  }
}

const activeTab = ref<SectionKey>('basics')
/** 当前锚点（只用于 URL 还原，不做高亮 —— 手动滚动后它会过期）。 */
const activeAnchor = ref<string | null>(null)
const focus = ref<{ kind: HitKind; id: string } | null>(null)

/** 焦点高亮标识。模板里 `data-focus` 读它，见下方 `rowFocusClass` 的注释。 */
const focusKey = computed(() => (focus.value ? `${focus.value.kind}:${focus.value.id}` : ''))

function currentTarget(): NavTarget {
  return { tab: activeTab.value, anchor: activeAnchor.value, focus: focus.value }
}

function sameTarget(a: NavTarget, b: NavTarget): boolean {
  return (
    a.tab === b.tab &&
    (a.anchor ?? null) === (b.anchor ?? null) &&
    (a.focus?.kind ?? '') === (b.focus?.kind ?? '') &&
    (a.focus?.id ?? '') === (b.focus?.id ?? '')
  )
}

const route = useRoute()
const router = useRouter()

/** 目录项的 `href`。用真链接（不是纯 click 事件）才能「复制链接 / 新标签打开」。 */
function urlFor(tab: SectionKey, anchor?: string): string {
  const query: Record<string, string> = {}
  if (tab !== 'basics') query.tab = tab
  if (anchor) query.anchor = anchor
  return router.resolve({ path: '/wiki', query }).href
}

/**
 * 把当前目标写回 URL。
 *
 * 用 `replace` 而不是 `push`：点目录不该在浏览器历史里堆一串条目
 * （按十几次返回才能离开文档站）。深链本身仍然有效 ——
 * 复制地址栏就是当前看到的这一节。
 */
function writeUrl(t: NavTarget): void {
  const query: Record<string, string> = {}
  if (t.tab !== 'basics') query.tab = t.tab
  if (t.anchor) query.anchor = t.anchor
  if (t.focus) query.focus = `${t.focus.kind}:${t.focus.id}`

  const cur = route.query
  const same =
    String(cur.tab ?? '') === (query.tab ?? '') &&
    String(cur.anchor ?? '') === (query.anchor ?? '') &&
    String(cur.focus ?? '') === (query.focus ?? '')
  if (same) return
  void router.replace({ path: route.path, query }).catch(() => {
    // 导航被取消（同一目标重复 replace）不是错误，也不该冒泡到控制台
  })
}

// ---------- 滚动 ----------

function scrollToAnchor(id: string): void {
  if (typeof document === 'undefined') return
  const el = document.getElementById(id)
  // jsdom 没有 scrollIntoView —— 可选调用，测试环境里安静地什么都不做
  el?.scrollIntoView?.({ block: 'start' })
}

/** 滚到被高亮的那一行。返回是否真的找到了（找不到时调用方回退到锚点）。 */
function scrollToFocus(): boolean {
  if (typeof document === 'undefined') return false
  const el = document.querySelector('.row-focus, .node-focus')
  if (!el) return false
  el.scrollIntoView?.({ block: 'center' })
  return true
}

/**
 * 回到页首。
 *
 * 不用 `window.scrollTo({behavior:'smooth'})`：jsdom 里它只会往 stderr
 * 打一行「Not implemented」，测试输出会被噪声淹没。
 * 直接改 scrollTop 在两个环境里都是安静且有效的。
 */
function scrollToTop(): void {
  if (typeof document === 'undefined') return
  document.documentElement.scrollTop = 0
  document.body.scrollTop = 0
}

// ---------- 导航 ----------

/**
 * 唯一的导航入口：目录点击、搜索命中、URL 变化都走这里。
 *
 * 顺序不能换：先改 state（v-show 立刻切 section）→ 等 DOM 更新 →
 * 再滚动。反过来滚的是**上一节**的位置，而且看起来像「点了没反应」。
 */
async function goTo(target: NavTarget, opts: { url?: boolean } = {}): Promise<void> {
  activeTab.value = target.tab
  activeAnchor.value = target.anchor ?? null
  focus.value = target.focus ?? null
  // 关卡焦点要先切章节 —— 否则那一行根本不在表里，滚过去是空白
  if (target.focus?.kind === 'level') ensureChapter(target.focus.id)

  await nextTick()

  const focused = target.focus ? scrollToFocus() : false
  if (!focused) {
    if (target.anchor) scrollToAnchor(target.anchor)
    else scrollToTop()
  }

  if (opts.url !== false) writeUrl(target)
  if (target.focus) scheduleFocusClear()
}

/** URL 变化 → 应用。若与当前状态一致就什么都不做（避免自己写的 URL 反过来打自己）。 */
async function applyRoute(): Promise<void> {
  const t = targetFromQuery(route.query as Record<string, unknown>)
  if (sameTarget(t, currentTarget())) return
  await goTo(t, { url: false })
}

/** 焦点高亮的存活时间。高亮是「你跳到这里」的指示，不是常驻标记。 */
const FOCUS_HIGHLIGHT_MS = 3200
let focusTimer: ReturnType<typeof setTimeout> | null = null

function scheduleFocusClear(): void {
  if (focusTimer) clearTimeout(focusTimer)
  focusTimer = setTimeout(() => {
    focusTimer = null
    focus.value = null
  }, FOCUS_HIGHLIGHT_MS)
}

/**
 * 行高亮类名工厂。
 *
 * ⚠️ 这里有一个不显眼但会让功能整体失效的坑：
 * 返回的闭包只在 el-table **自己**渲染时才读 `focus`，
 * 而 WikiView 的渲染副作用并不会追踪它 —— 于是 `focus` 变了、
 * 父组件不重渲染、el-table 也不重渲染，行上永远加不上类。
 * 表现是「搜索能跳，但看不出跳到了哪一行」。
 *
 * 解法：模板里 `<div class="wiki-body" :data-focus="focusKey">` 让父渲染
 * 显式依赖 `focusKey`；父组件重渲染 → 本函数被重新调用（新函数身份）
 * → el-table 收到新的 `row-class-name` prop → 重算行类名。
 */
function rowFocusClass(kind: HitKind) {
  return (scope: { row: Record<string, unknown> }): string => {
    const f = focus.value
    if (!f || f.kind !== kind) return ''
    // 反应表用 key（字符串），其余表用 id（数字）
    const id = scope.row.id ?? scope.row.key
    return String(id) === f.id ? 'row-focus' : ''
  }
}

// ---------- 搜索 ----------

interface SearchHit {
  kind: HitKind
  id: string
  title: string
  meta: string
  /** 参与匹配的正文（说明文案等） */
  body: string
  section: SectionKey
  anchor?: string
}

/** 搜索框输入（与 URL 无关：搜索是一次性的跳转动作，不该进历史） */
const searchText = ref('')
const searchOpen = ref(false)

/** 部位中文名（与服务端 `SeedEquipment.Slot` 同名）。 */
const SLOT_LABEL: Record<string, string> = {
  weapon: '武器',
  helmet: '头盔',
  armor: '护甲',
  gloves: '手套',
  boots: '靴子',
  charm: '挂件',
}

const ELEMENT_LABEL: Record<string, string> = {
  fire: '焰',
  ice: '冰',
  lightning: '电',
  corrosion: '毒',
  kinetic: '动能',
}
const ELEMENT_COLOR: Record<string, string> = {
  fire: '#ff6b35',
  ice: '#58a6ff',
  lightning: '#ffd33d',
  corrosion: '#7ee787',
  kinetic: '#c9a7ff',
}
const CATEGORY_LABEL: Record<string, string> = {
  normal: '普通',
  ranged: '远程',
  flying: '飞行',
  special: '特殊',
  elite: '精英',
  boss: 'BOSS',
}

/**
 * 全站索引：所有「玩家会想查的东西」各占一条。
 *
 * 为什么在客户端建索引而不是加后端搜索接口：`/config` 本来就是
 * 一次性下发的全量内容（文档站整页都靠它渲染），索引只是同一份数据
 * 换个形状 —— 再做一个搜索端点等于让服务端和文档站各维护一套匹配规则。
 */
const searchIndex = computed<SearchHit[]>(() => {
  const c = cfg.value
  if (!c) return []
  const hits: SearchHit[] = []

  for (const l of c.levels ?? []) {
    hits.push({
      kind: 'level',
      id: String(l.id),
      title: l.name,
      meta: `第 ${l.chapter} 章 · 关卡 ${l.id}`,
      body: `${l.name} 第${l.chapter}章`,
      section: 'levels',
    })
  }
  for (const e of c.enemies ?? []) {
    hits.push({
      kind: 'enemy',
      id: String(e.id),
      title: e.name,
      meta: `${CATEGORY_LABEL[e.category] ?? e.category} · ${e.code}`,
      body: e.descr ?? '',
      section: 'enemies',
    })
  }
  for (const s of c.skills ?? []) {
    hits.push({
      kind: 'skill',
      id: String(s.id),
      title: s.name,
      meta: `${ELEMENT_LABEL[s.element] ?? s.element} · ${s.family}`,
      body: s.descr ?? '',
      section: 'skills',
      anchor: 'skill-table',
    })
  }
  for (const s of c.composite_skills ?? []) {
    hits.push({
      kind: 'composite',
      id: String(s.id),
      title: s.name,
      meta: '合成技能',
      body: s.descr ?? '',
      section: 'skills',
      anchor: 'composite',
    })
  }
  for (const eq of c.equipment ?? []) {
    hits.push({
      kind: 'equip',
      id: String(eq.id),
      title: eq.name,
      meta: `${SLOT_LABEL[eq.slot] ?? eq.slot} · ${eq.tier} 阶`,
      body: eq.descr ?? '',
      section: 'equipment',
    })
  }
  for (const r of c.reactions ?? []) {
    hits.push({
      kind: 'reaction',
      id: r.key,
      title: r.name,
      meta: '元素反应',
      body: r.descr ?? '',
      section: 'basics',
      anchor: 'reactions',
    })
  }
  for (const f of c.mastery_families ?? []) {
    for (const n of f.nodes ?? []) {
      hits.push({
        kind: 'mastery',
        id: n.name,
        title: n.name,
        meta: `${f.name} · 第 ${n.layer} 层`,
        body: f.name,
        section: 'mastery',
      })
    }
  }
  return hits
})

/** 一次最多列多少条（再多也看不过来，不如让玩家把词打准） */
const SEARCH_LIMIT = 20

const searchMatches = computed<{ total: number; list: SearchHit[] }>(() => {
  const q = searchText.value.trim().toLowerCase()
  if (!q) return { total: 0, list: [] }
  // 子串匹配即可：内容是中文名词（技能名/敌人名），分词反而会把
  // 「熔岩龙」这种整体名词切碎。规模（百级 + 几十条）也不值得上索引结构。
  const all = searchIndex.value.filter((h) =>
    `${h.title} ${h.meta} ${h.body}`.toLowerCase().includes(q),
  )
  return { total: all.length, list: all.slice(0, SEARCH_LIMIT) }
})

/** 失焦后延迟关闭：否则点结果的 mousedown 会先把面板关掉，click 落空。 */
let searchBlurTimer: ReturnType<typeof setTimeout> | null = null

function closeSearchSoon(): void {
  if (searchBlurTimer) clearTimeout(searchBlurTimer)
  searchBlurTimer = setTimeout(() => {
    searchBlurTimer = null
    searchOpen.value = false
  }, 120)
}

async function pick(hit: SearchHit): Promise<void> {
  searchOpen.value = false
  await goTo({
    tab: hit.section,
    anchor: hit.anchor ?? null,
    focus: { kind: hit.kind, id: hit.id },
  })
}

function pickFirst(): void {
  const first = searchMatches.value.list[0]
  if (first) void pick(first)
}

// ---------- 展示辅助 ----------

/** 把控制时长渲染成人话（秒，保留 1 位小数）。 */
function statusText(key: string, ms: number): string {
  const kind =
    key === 'flash_freeze' ? '冻结'
    : key === 'superconduct' ? '减速'
    : key === 'overheat' ? '眩晕'
    : '控制'
  return `${kind} ${(ms / 1000).toFixed(1)}s`
}

/** 空态文案（2026-10-21 P5-8）：空表要说清「为什么空」。 */
const emptyText = '暂无数据'
const loading = ref(false)
const cfg = ref<ConfigResp | null>(null)
const reactUsage = ref<Array<{ reaction: string; count: number }>>([])

/** 星级比例行（三档），由 `score_rules.star_target_ratio` 驱动而非硬编码。 */
const starRatioRows = computed(() => {
  const r = cfg.value?.score_rules?.star_target_ratio ?? []
  const labels = ['一星', '二星', '三星']
  return r.map((v, i) => ({ label: labels[i] ?? `${i + 1} 星`, v: `${v}‰` }))
})

/**
 * 满级一个技能的总金币。
 *
 * 费用是线性的（`base_cost × 当前等级`），所以
 * `1 + 2 + … + (max_level-1)` = `max_level × (max_level-1) / 2`。
 *
 * ⚠️ 这个求和公式必须与服务端 `SkillRules.CostFrom` 一致。
 * 漂移的表现是「页面显示 4500、实际扣 4600」——
 * 而玩家只会在扣款那一刻发现，所以这里用服务端下发的
 * `base_cost` / `max_level` 现算，而不是写死 4500。
 */
const fullUpgradeCost = computed(() => {
  const r = cfg.value?.skill_rules
  if (!r) return '—'
  const n = r.max_level
  return (r.base_cost * (n * (n - 1))) / 2
})

/** 抗性条：正数为抗（减伤），负数为弱（增伤），范围 ±50% */
function resistPct(v: number): number {
  return Math.max(-50, Math.min(50, v / 10))
}

function resistColor(v: number): string {
  if (v >= 0) return '#58a6ff'
  return '#7ee787'
}

const activeChapter = ref(1)
const chapterLevels = computed(() =>
  (cfg.value?.levels ?? []).filter((l) => l.chapter === activeChapter.value),
)

/** 关卡焦点要先把章节切到该关所在章，否则那一行不在表里。 */
function ensureChapter(levelId: string): void {
  const lv = (cfg.value?.levels ?? []).find((l) => String(l.id) === levelId)
  if (lv) activeChapter.value = lv.chapter
}

/** 玩家实际最常用哪条反应 —— 文档站给出"主流打法"提示 */
const topReaction = computed(() => {
  const sorted = [...reactUsage.value].sort((a, b) => b.count - a.count)
  return sorted[0]
})

async function load() {
  loading.value = true
  try {
    // config 端点无需管理员令牌，因此直接走底层 api 客户端而不经 admin 封装
    const { api } = await import('@/api/client')
    const data = await api.get<ConfigResp>('/config', { auth: false })
    cfg.value = data

    // 2026-10-21（P2-5）：「实战分布」用的是**管理员**数据
    // （GET /admin/dashboard），所以它用 **soft** 版本调用：
    // 401 是**预期答案**（匿名访客本来就没这个权限），不该触发
    // `handleUnauthorized()` 把 location.hash 改成 '#/admin/login'
    // —— 一个**玩家向的文档站**把访客踢去了运营后台登录页。
    //
    // 此前那个 `.catch(() => null)` 只吞掉了**异常**，
    // 拦不住**已经发生的跳转** —— 症状是「打开文档站 → 被弹去登录页」。
    try {
      const dash = await fetchDashboardSoft()
      reactUsage.value = dash?.reaction_usage ?? []
    } catch {
      // 锦上添花的东西拿不到，不该影响文档站本身
      reactUsage.value = []
    }
  } catch (e) {
    ElMessage.error(`配置加载失败：${(e as Error).message}`)
  } finally {
    loading.value = false
  }
}

onMounted(async () => {
  // 先取数据再应用深链：`?focus=level:96` 需要 levels 才能切到对应章节
  await load()
  await applyRoute()
})

// 同一页面内的深链跳转（例如别处 push 了新的 query）
watch(() => route.fullPath, () => { void applyRoute() })

onUnmounted(() => {
  if (focusTimer) clearTimeout(focusTimer)
  if (searchBlurTimer) clearTimeout(searchBlurTimer)
})
</script>

<template>
  <div v-loading="loading" class="wiki">
    <div class="page-title">玩家玩法文档站</div>
    <p class="page-subtitle">
      本页数据直接来自线上 <code>/api/v1/config</code>，与游戏内实际数值同源 ——
      改配置即改文档，不存在「文档说的和游戏里不一样」。
    </p>

    <!-- 搜索：跨全部章节，命中后直接跳到对应那一行并高亮 -->
    <div class="wiki-search">
      <input
        v-model="searchText"
        class="search-input"
        type="search"
        placeholder="搜索关卡 / 敌人 / 技能 / 装备 / 反应 / 专精（回车跳第一条）"
        @focus="searchOpen = true"
        @input="searchOpen = true"
        @keydown.enter.prevent="pickFirst"
        @keydown.esc="searchOpen = false"
        @blur="closeSearchSoon"
      />
      <div v-if="searchOpen && searchText.trim()" class="search-panel" @mousedown.prevent>
        <div v-if="searchMatches.total === 0" class="search-empty">
          没有匹配「{{ searchText.trim() }}」的内容
        </div>
        <template v-else>
          <button
            v-for="h in searchMatches.list"
            :key="h.kind + ':' + h.id"
            type="button"
            class="search-hit"
            @click="pick(h)"
          >
            <span class="hit-kind">{{ KIND_LABEL[h.kind] }}</span>
            <span class="hit-title">{{ h.title }}</span>
            <span class="hit-meta">{{ h.meta }}</span>
          </button>
          <div v-if="searchMatches.total > searchMatches.list.length" class="search-more">
            共 {{ searchMatches.total }} 条，只列前 {{ searchMatches.list.length }} 条 —— 把关键词打准一点
          </div>
        </template>
      </div>
    </div>

    <!--
      ⚠️ `:data-focus` 不是装饰：它让本组件的渲染副作用**依赖** focusKey，
      从而在焦点变化时重渲染，把新的 row-class-name 传给 el-table。
      少了它，搜索能跳但行不会高亮 —— 详见 `rowFocusClass` 的注释。
    -->
    <div class="wiki-body" :data-focus="focusKey">
      <aside class="wiki-nav" aria-label="目录">
        <div class="nav-head">目录</div>
        <nav>
          <div v-for="s in SECTIONS" :key="s.key" class="nav-group">
            <a
              class="nav-item"
              :class="{ on: activeTab === s.key }"
              :href="urlFor(s.key)"
              @click.prevent="goTo({ tab: s.key })"
            >{{ s.label }}</a>
            <div v-if="s.anchors" class="nav-sub">
              <a
                v-for="a in s.anchors"
                :key="a.id"
                class="nav-sub-item"
                :href="urlFor(s.key, a.id)"
                @click.prevent="goTo({ tab: s.key, anchor: a.id })"
              >{{ a.label }}</a>
            </div>
          </div>
        </nav>
      </aside>

      <main class="wiki-main">
        <!-- 基础规则 -->
        <section v-show="activeTab === 'basics'" class="wiki-section">
          <el-alert
            type="warning"
            show-icon
            :closable="false"
            title="为什么「战力」不是最重要的数字"
            description="本作的伤害分三段：直接伤害吃养成，元素层数伤害与反应伤害只吃元素投入，且反应伤害中攻击力的权重被结构性限制在 30% 以内。堆面板能变强，但打不过抗性矩阵 —— 搭配才是主要胜负手。"
            style="margin-bottom: 14px"
          />

          <h4>5 元素</h4>
          <p>
            敌人对每种元素有独立抗性（±50%）。两种元素叠加会触发<strong>反应链</strong>，共 7 条。
            动能不能作为「已附着元素」触发反应，它只能后手打出破甲。
          </p>

          <h4 id="reactions">反应链一览</h4>
          <!--
            ⚠️ 列宽总和要留在正文可用宽度之内（桌面 ~914px）：
            这张表 8 列，原来 120+90+130+150+110+100+100+150 = 950，
            加上左侧目录后就溢出了 —— 表现是「说明」列被切掉半截，
            而 el-table 的横向滚动条藏在表底，没人会去拖它。
          -->
          <el-table :data="cfg?.reactions ?? []" size="small" border>
            <el-table-column prop="name" label="反应" width="110" />
            <el-table-column label="系数" width="80">
              <template #default="{ row }">{{ row.base_coef }}‰</template>
            </el-table-column>
            <el-table-column label="攻击力权重" width="110">
              <template #default="{ row }">
                <el-tag :type="row.attack_weight_pct >= 300 ? 'warning' : 'info'" size="small">
                  {{ row.attack_weight_pct }}‰
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column label="控制效果" width="120">
              <template #default="{ row }">
                <el-tag v-if="row.status_duration_ms > 0" size="small" type="success">
                  {{ statusText(row.key, row.status_duration_ms) }}
                </el-tag>
                <span v-else class="muted">—</span>
              </template>
            </el-table-column>
            <el-table-column label="受击增伤" width="90">
              <template #default="{ row }">
                <el-tag v-if="row.amplify_pct > 0" size="small" type="warning">
                  +{{ row.amplify_pct }}%
                </el-tag>
                <span v-else class="muted">—</span>
              </template>
            </el-table-column>
            <el-table-column label="驱散护盾" width="90">
              <template #default="{ row }">
                <el-tag v-if="row.dispel_shield" size="small" type="danger">可驱散</el-tag>
                <span v-else class="muted">—</span>
              </template>
            </el-table-column>
            <!--
              ⚠️ 第 87 轮写这段注释时：`aoe_radius` 这一列**恒为 0** ——
              第 83 轮把溅射值全部置 0（那之前它唯一的消费者是
              `render/canvas.ts` 的屏幕震动，已在第 83 轮删掉，
              因为那是在视觉上谎称发生了爆炸）。

              此后 `steam_burst`（2026-10-10，120）与 `overheat`（第 160 轮，90）
              相继恢复历史半径并接上真实范围伤害（引擎 `triggerReactionAoe`），
              这一列不再恒 0 —— 它显示的就是实际生效的溅射半径。

              保留这一列而不是删掉：它是 `/config` 公开契约字段，
              「单体」与具体半径的对比正是玩家关心的信息。
              宽度之外的说明文字放在下面的 note 里。
            -->
            <el-table-column label="溅射半径" width="90">
              <template #default="{ row }">{{ row.aoe_radius || '单体' }}</template>
            </el-table-column>
            <!-- 第 87 轮新增：说明列。
                 敌人 / 技能 / 复合技能三张表都有这一列，只有反应表没有 ——
                 于是 `descr` 字段（服务端 `ReactionSpec.Descr` 的注释写着
                 「运营后台的玩法文档站直接展示它」）在这张表上被丢掉了。
                 第 83 轮修正的 5 条错误文案因此**一个都看不到**。 -->
            <el-table-column prop="descr" label="说明" min-width="150" />
            <template #empty>
              <el-empty :description="emptyText" :image-size="60" />
            </template>
          </el-table>
          <p class="note">
            「溅射半径」一栏显示各反应**实际生效**的溅射半径
            （蒸汽爆发 120 / 过热 90 已接上真实范围伤害）；显示「单体」
            表示该反应没有溅射。仍**没有特殊效果**的只剩两条
            （燃烧云的火区 / 腐蚀扩散的层数传播）—— 具体缺什么见
            README 已知边界第 19 条。
            <br />
            「攻击力权重」就是这一条反应里由面板攻击力贡献的比例上限。全部 ≤30% ——
            这是保证低养成玩家能靠技巧翻盘的结构性约束，不是可以随数值膨胀绕过的软性限制。
          </p>

          <h4 id="rating">构筑评分（I-7）</h4>
          <p>主指标不是总战力，而是这五个维度：</p>
          <el-table :data="Object.entries(cfg?.rating_weights ?? {}).map(([k, v]) => ({ k, v }))" size="small" border>
            <el-table-column prop="k" label="维度" />
            <el-table-column prop="v" label="权重" />
            <template #empty>
              <el-empty :description="emptyText" :image-size="60" />
            </template>
          </el-table>
        </section>

        <!-- 元素矩阵 -->
        <section v-show="activeTab === 'enemies'" class="wiki-section">
          <p class="note">
            绿色为<strong>弱点</strong>（负抗性，伤害放大），蓝色为<strong>抗性</strong>（减伤）。
            全部 BOSS 都被设计成「必有解」：任意一只都至少有两到三种元素是弱点。
          </p>
          <el-table
            :data="cfg?.enemies ?? []"
            size="small"
            border
            stripe
            :row-class-name="rowFocusClass('enemy')"
          >
            <el-table-column prop="id" label="ID" width="55" />
            <el-table-column prop="name" label="名称" min-width="120" />
            <el-table-column label="类型" width="80">
              <template #default="{ row }">
                <el-tag :type="row.category === 'boss' ? 'danger' : 'info'" size="small">
                  {{ CATEGORY_LABEL[row.category] ?? row.category }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column prop="hp" label="血量" width="90" />
            <el-table-column label="五维抗性" min-width="260">
              <template #default="{ row }">
                <div v-for="e in ['fire', 'ice', 'lightning', 'corrosion', 'kinetic']" :key="e" class="resist-row">
                  <span class="resist-name" :style="{ color: ELEMENT_COLOR[e] }">{{ ELEMENT_LABEL[e] }}</span>
                  <div class="resist-bar">
                    <div class="resist-zero" />
                    <div
                      class="resist-fill"
                      :style="{
                        width: Math.abs(resistPct(row.resist?.[e] ?? 0)) * 2 + '%',
                        background: resistColor(row.resist?.[e] ?? 0),
                        left: (row.resist?.[e] ?? 0) >= 0 ? '50%' : `${50 - Math.abs(resistPct(row.resist?.[e] ?? 0)) * 2}%`,
                      }"
                    />
                  </div>
                  <span class="resist-num">{{ (row.resist?.[e] ?? 0) / 10 }}%</span>
                </div>
              </template>
            </el-table-column>
            <el-table-column prop="descr" label="说明" min-width="150" />
            <template #empty>
              <el-empty :description="emptyText" :image-size="60" />
            </template>
          </el-table>
        </section>

        <!-- 技能 -->
        <section v-show="activeTab === 'skills'" class="wiki-section">
          <div id="skill-table">
            <el-table
              :data="cfg?.skills ?? []"
              size="small"
              border
              stripe
              :row-class-name="rowFocusClass('skill')"
            >
              <el-table-column prop="id" label="ID" width="55" />
              <el-table-column prop="name" label="名称" min-width="120" />
              <el-table-column label="元素" width="80">
                <template #default="{ row }">
                  <span :style="{ color: ELEMENT_COLOR[row.element], fontWeight: 600 }">
                    {{ ELEMENT_LABEL[row.element] ?? row.element }}
                  </span>
                </template>
              </el-table-column>
              <el-table-column prop="family" label="系别" width="90" />
              <el-table-column prop="base_damage" label="伤害" width="76" />
              <el-table-column prop="heat_cost" label="热量" width="70" />
              <el-table-column prop="cooldown_ms" label="冷却" width="86" />
              <el-table-column prop="apply_stacks" label="层数" width="66" />
              <el-table-column prop="descr" label="说明" min-width="180" />
              <template #empty>
                <el-empty :description="emptyText" :image-size="60" />
              </template>
            </el-table>
          </div>

          <h4 id="composite" style="margin-top: 16px">合成技能</h4>
          <el-table
            :data="cfg?.composite_skills ?? []"
            size="small"
            border
            stripe
            :row-class-name="rowFocusClass('composite')"
          >
            <el-table-column prop="id" label="ID" width="55" />
            <el-table-column prop="name" label="名称" min-width="140" />
            <el-table-column label="元素" width="80">
              <template #default="{ row }">
                <span :style="{ color: ELEMENT_COLOR[row.element] }">{{ ELEMENT_LABEL[row.element] }}</span>
              </template>
            </el-table-column>
            <el-table-column prop="descr" label="说明" min-width="220" />
            <template #empty>
              <el-empty :description="emptyText" :image-size="60" />
            </template>
          </el-table>
        </section>

        <!-- 装备（P6-12）-->
        <section v-show="activeTab === 'equipment'" class="wiki-section">
          <el-alert
            type="info"
            :closable="false"
            show-icon
            title="装备怎么生效"
            description="装备提供护甲（减少漏怪造成的伤害）与通用攻击加成。**元素标签当前不参与任何结算** —— 按标签给加成的机制没有实现，所以这里不宣传它。一个部位同时只能装一件。"
            style="margin-bottom: 12px"
          />
          <el-table
            :data="cfg?.equipment ?? []"
            size="small"
            border
            stripe
            :row-class-name="rowFocusClass('equip')"
          >
            <el-table-column prop="id" label="ID" width="55" />
            <el-table-column prop="name" label="名称" min-width="130" />
            <el-table-column label="部位" width="80">
              <template #default="{ row }">{{ SLOT_LABEL[row.slot] ?? row.slot }}</template>
            </el-table-column>
            <el-table-column prop="tier" label="阶级" width="66" />
            <el-table-column label="元素" width="80">
              <template #default="{ row }">
                <span :style="{ color: ELEMENT_COLOR[row.element] }">
                  {{ ELEMENT_LABEL[row.element] ?? row.element }}
                </span>
              </template>
            </el-table-column>
            <el-table-column label="护甲‰" width="76">
              <template #default="{ row }">{{ row.base_armor }}</template>
            </el-table-column>
            <el-table-column label="攻击‰" width="76">
              <template #default="{ row }">{{ row.base_bonus_pct }}</template>
            </el-table-column>
            <el-table-column label="解锁等级" width="90">
              <template #default="{ row }">{{ row.unlock_level }}</template>
            </el-table-column>
            <el-table-column prop="descr" label="说明" min-width="200" />
            <template #empty>
              <el-empty :description="emptyText" :image-size="60" />
            </template>
          </el-table>
        </section>

        <!--
          数值规则这一节展示的是**分数与升级的规则本身**，不是内容数据。

          为什么单独一节：运营最常问的两个问题是
          「为什么我三星变两星」「升级为什么涨这么少」，
          而答案（星级门槛、每级系数、费用曲线）都在这两个规则对象里。
          之前它们没被展示，运营只能去翻代码。

          ⚠️ 全部读 `/api/v1/config` 的 `score_rules` / `skill_rules`，
          **不硬编码任何数字** —— 这样后台改规则，这一页自动跟着变，
          不会成为第二份需要同步的副本。
        -->
        <section v-show="activeTab === 'rules'" class="wiki-section">
          <el-alert
            type="info"
            show-icon
            :closable="false"
            title="这一页的数字全部来自 /api/v1/config，与结算逻辑同源"
            style="margin-bottom: 14px"
          />

          <h4 id="score">分数与星级</h4>
          <el-descriptions :column="2" border size="small">
            <el-descriptions-item label="每多少伤害算 1 分">
              {{ cfg?.score_rules?.per_damage_unit ?? '—' }}
            </el-descriptions-item>
            <el-descriptions-item label="击杀普通敌人">
              {{ cfg?.score_rules?.on_kill_normal ?? '—' }}
            </el-descriptions-item>
            <el-descriptions-item label="击杀 BOSS">
              {{ cfg?.score_rules?.on_kill_boss ?? '—' }}
            </el-descriptions-item>
            <el-descriptions-item label="打满理论满分所需最短秒数">
              {{ cfg?.score_rules?.score_full_at_sec ?? '—' }}
            </el-descriptions-item>
          </el-descriptions>
          <p class="note">
            星级门槛 = 该关理论满分 × 下列比例。理论满分由「所有敌人的
            血量＋护盾换算成分」加「全部击杀分」得出，
            所以<strong>打法不同不影响门槛，只有实际拿到的分影响</strong>。
          </p>
          <el-table :data="starRatioRows" size="small" border style="margin-bottom: 8px">
            <el-table-column prop="label" label="星级" width="100" />
            <el-table-column prop="v" label="占理论满分" width="140" />
            <template #empty>
              <el-empty :description="emptyText" :image-size="60" />
            </template>
          </el-table>
          <p class="note">
            ⚠️ 结算时的得分速率裁剪锚点（<code>score_full_at_sec</code>）
            远低于实际通关时长，因此<strong>它从不生效</strong> ——
            「打得快」目前不进分数。
          </p>

          <h4 id="upgrade">技能升级</h4>
          <el-descriptions :column="3" border size="small">
            <el-descriptions-item label="等级上限">
              {{ cfg?.skill_rules?.max_level ?? '—' }}
            </el-descriptions-item>
            <el-descriptions-item label="每级伤害加成">
              {{ cfg?.skill_rules?.coef_permille ?? '—' }}‰
            </el-descriptions-item>
            <el-descriptions-item label="费用基数">
              {{ cfg?.skill_rules?.base_cost ?? '—' }} 金币 × 当前等级
            </el-descriptions-item>
          </el-descriptions>
          <p class="note">
            升到满级共需 <strong>{{ fullUpgradeCost }}</strong> 金币。
            等级系数刻意取得比攻击力封顶更低 ——
            技能伤害与面板攻击走同一个乘区，抬得更高会压制元素反应，
            而反应正是这个项目的核心创新。
          </p>
        </section>

        <section v-show="activeTab === 'mastery'" class="wiki-section">
          <p class="note">
            每层只能选 2 个节点（4 选 2），且第 2、3 层必须先点亮前一层。
            专精点不够点满任何一系 —— <strong>必须做方向选择</strong>。
          </p>
          <div v-for="f in cfg?.mastery_families ?? []" :key="f.family" class="family-block">
            <div class="family-name">{{ f.name }}</div>
            <div v-for="layer in 3" :key="layer" class="layer-row">
              <span class="layer-label">第 {{ layer }} 层</span>
              <span
                v-for="n in f.nodes.filter((x) => x.layer === layer)"
                :key="n.name"
                class="node"
                :class="{ 'node-focus': focus?.kind === 'mastery' && focus.id === n.name }"
              >
                {{ n.name }}
              </span>
            </div>
          </div>
        </section>

        <!-- 关卡 -->
        <section v-show="activeTab === 'levels'" class="wiki-section">
          <el-select v-model="activeChapter" style="width: 160px; margin-bottom: 12px">
            <el-option v-for="c in cfg?.chapters ?? []" :key="c.id" :label="c.name" :value="c.id" />
          </el-select>
          <el-table
            :data="chapterLevels"
            size="small"
            border
            stripe
            :row-class-name="rowFocusClass('level')"
          >
            <el-table-column prop="id" label="关" width="60" />
            <el-table-column prop="name" label="名称" min-width="140" />
            <el-table-column prop="difficulty" label="难度" width="80" />
            <el-table-column prop="base_hp" label="防线血量" width="100" />
            <el-table-column prop="wave_count" label="波次" width="70" />
            <el-table-column prop="energy_cost" label="体力" width="70" />
            <el-table-column prop="element_cap" label="层数上限" width="94" />
            <el-table-column prop="max_reaction_tier" label="反应阶" width="86" />
            <el-table-column label="星级门槛" min-width="160">
              <template #default="{ row }">{{ row.star_targets.join(' / ') }}</template>
            </el-table-column>
            <template #empty>
              <el-empty :description="emptyText" :image-size="60" />
            </template>
          </el-table>
        </section>

        <!-- 实战统计 -->
        <section v-show="activeTab === 'live'" class="wiki-section">
          <p class="note">
            下面是<strong>真实玩家数据</strong>（不是设计意图）。如果某条反应长期没人用，
            说明它的触发条件太苛刻，需要调整 —— 而不是怪玩家不会玩。
          </p>
          <el-table :data="[...reactUsage].sort((a, b) => b.count - a.count)" size="small" border>
            <el-table-column prop="reaction" label="反应" width="180" />
            <el-table-column prop="count" label="触发次数" />
            <el-table-column label="占比">
              <template #default="{ row }">
                <el-progress
                  :percentage="
                    Math.round(
                      (row.count /
                        Math.max(1, reactUsage.reduce((s, x) => s + x.count, 0))) *
                        100,
                    )
                  "
                />
              </template>
            </el-table-column>
            <template #empty>
              <el-empty :description="emptyText" :image-size="60" />
            </template>
          </el-table>
          <p v-if="topReaction" class="note">
            当前主流打法是「{{ topReaction.reaction }}」。若你卡在某一关，
            试试换一个<strong>还没被多数人用到</strong>的搭配。
          </p>
        </section>
      </main>
    </div>
  </div>
</template>

<style scoped>
/*
 * 这一页是壳外的全宽页面（`/wiki` 不走 AdminLayout），
 * 因此内边距、最大宽度都得自己给 —— 此前只有 `max-width`，
 * 于是内容紧贴视口左上角，和后台里那些页面观感不一致。
 */
.wiki {
  max-width: 1180px;
  margin: 0 auto;
  padding: 20px 24px 64px;
}

.note {
  font-size: 12px;
  color: var(--lyp-muted);
  margin: 8px 0 0;
}
code {
  font-family: ui-monospace, monospace;
  font-size: 11px;
  background: #1c2430;
  padding: 1px 4px;
  border-radius: 3px;
}
h4 {
  margin: 18px 0 8px;
  /* 锚点跳转时标题不要正好贴在视口顶端 */
  scroll-margin-top: 16px;
}

/* ---------- 搜索 ---------- */

.wiki-search {
  position: relative;
  max-width: 460px;
  margin-bottom: 18px;
}

.search-input {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid var(--lyp-border);
  border-radius: 6px;
  background: #0b0f14;
  color: var(--lyp-text);
  font-size: 13px;
  outline: none;
}
.search-input:focus {
  border-color: #58a6ff;
}

.search-panel {
  position: absolute;
  z-index: 20;
  left: 0;
  right: 0;
  top: calc(100% + 6px);
  max-height: 380px;
  overflow-y: auto;
  background: var(--lyp-panel);
  border: 1px solid var(--lyp-border);
  border-radius: 8px;
  box-shadow: 0 12px 28px rgba(0, 0, 0, 0.45);
  padding: 4px;
}

.search-empty {
  padding: 12px;
  font-size: 12px;
  color: var(--lyp-muted);
}

.search-hit {
  display: flex;
  align-items: baseline;
  gap: 8px;
  width: 100%;
  padding: 7px 8px;
  border: none;
  border-radius: 5px;
  background: transparent;
  color: var(--lyp-text);
  font-size: 13px;
  text-align: left;
  cursor: pointer;
}
.search-hit:hover {
  background: #1c2430;
}

.hit-kind {
  flex-shrink: 0;
  font-size: 11px;
  color: #58a6ff;
  border: 1px solid #1f6feb55;
  border-radius: 3px;
  padding: 0 4px;
}
.hit-title {
  font-weight: 600;
}
.hit-meta {
  font-size: 11px;
  color: var(--lyp-muted);
  margin-left: auto;
  white-space: nowrap;
}

.search-more {
  padding: 8px;
  font-size: 11px;
  color: var(--lyp-muted);
  border-top: 1px solid var(--lyp-border);
  margin-top: 4px;
}

/* ---------- 目录 + 正文 ---------- */

.wiki-body {
  display: flex;
  align-items: flex-start;
  gap: 24px;
}

.wiki-nav {
  position: sticky;
  top: 16px;
  width: 186px;
  flex-shrink: 0;
  border-right: 1px solid var(--lyp-border);
  padding-right: 12px;
  max-height: calc(100vh - 32px);
  overflow-y: auto;
}

.nav-head {
  font-size: 11px;
  letter-spacing: 1px;
  color: var(--lyp-muted);
  margin-bottom: 8px;
}

.nav-group {
  margin-bottom: 6px;
}

.nav-item {
  display: block;
  padding: 5px 8px;
  border-radius: 5px;
  color: var(--lyp-text);
  font-size: 13px;
  text-decoration: none;
  cursor: pointer;
}
.nav-item:hover {
  background: #1c2430;
}
.nav-item.on {
  background: #1f6feb26;
  color: #79c0ff;
  font-weight: 600;
}

.nav-sub {
  margin: 2px 0 6px 8px;
  border-left: 1px solid var(--lyp-border);
  padding-left: 8px;
}
.nav-sub-item {
  display: block;
  padding: 3px 6px;
  border-radius: 4px;
  font-size: 12px;
  color: var(--lyp-muted);
  text-decoration: none;
}
.nav-sub-item:hover {
  color: var(--lyp-text);
  background: #1c2430;
}

.wiki-main {
  flex: 1;
  min-width: 0;
}

.wiki-section {
  border: 1px solid var(--lyp-border);
  border-radius: 6px;
  background: var(--lyp-panel);
  padding: 4px 18px 18px;
}

/* 搜索跳转后短暂高亮目标行（el-table 的 row-class-name 加上去的） */
.wiki :deep(.row-focus) td {
  background: #1f6feb33 !important;
  transition: background 0.4s;
}

.node-focus {
  outline: 1px solid #58a6ff;
  outline-offset: 1px;
}

/* ---------- 表格内抗性条 ---------- */

.resist-row {
  display: flex;
  align-items: center;
  gap: 6px;
  margin: 1px 0;
}
.resist-name {
  width: 34px;
  font-size: 11px;
  flex-shrink: 0;
}
.resist-bar {
  position: relative;
  flex: 1;
  height: 9px;
  background: #1c2430;
  border-radius: 2px;
  overflow: hidden;
}
.resist-zero {
  position: absolute;
  left: 50%;
  top: 0;
  bottom: 0;
  width: 1px;
  background: #30363d;
}
.resist-fill {
  position: absolute;
  top: 0;
  bottom: 0;
}
.resist-num {
  width: 44px;
  text-align: right;
  font-size: 11px;
  color: var(--lyp-muted);
  font-variant-numeric: tabular-nums;
}

/* ---------- 专精 ---------- */

.family-block {
  border: 1px solid var(--lyp-border);
  border-radius: 6px;
  padding: 10px 14px;
  margin-bottom: 10px;
}
.family-name {
  font-weight: 600;
  margin-bottom: 6px;
}
.layer-row {
  display: flex;
  align-items: center;
  gap: 6px;
  margin: 3px 0;
}
.layer-label {
  width: 54px;
  font-size: 11px;
  color: var(--lyp-muted);
  flex-shrink: 0;
}
.node {
  font-size: 11px;
  background: #1c2430;
  border-radius: 3px;
  padding: 2px 7px;
}

/* ---------- 窄屏：目录变成顶部横向条 ---------- */

@media (max-width: 900px) {
  .wiki {
    padding: 16px 14px 48px;
  }
  .wiki-body {
    flex-direction: column;
    gap: 14px;
    /*
     * ⚠️ 必须把 `align-items: flex-start` 改回 stretch。
     * 桌面布局里它是为了「目录 sticky 不被拉高」而设的；但在 column 方向，
     * flex-start 会让子项按 **max-content** 定宽 —— 于是那张 950px 宽的
     * 表格把整个正文撑到 950px，窄屏上页面横向溢出（文字被切在屏幕外），
     * 而 `min-width: 0` 完全拦不住（它管的是收缩，不管 intrinsic 定宽）。
     */
    align-items: stretch;
  }
  .wiki-main {
    width: 100%;
  }
  .wiki-nav {
    position: static;
    width: 100%;
    max-height: none;
    border-right: none;
    border-bottom: 1px solid var(--lyp-border);
    padding: 0 0 8px;
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
  }
  .nav-head {
    display: none;
  }
  .wiki-nav nav {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
  }
  .nav-group {
    margin: 0;
  }
  .nav-item {
    border: 1px solid var(--lyp-border);
    font-size: 12px;
    padding: 4px 10px;
  }
  /* 窄屏放不下二级目录，横向条只保留章节 */
  .nav-sub {
    display: none;
  }
}
</style>
