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
import AiInsightCard from "@/components/AiInsightCard.vue"
import { fetchAdminRecommendation, insightFromRecommendation, buildLocalRecommendation } from '@/utils/recommendation'
import type { AiInsight } from '@/utils/aiInsight'
import { spreadStacksText, armorShredText, knockbackText } from '@/reactionMechanicsText'
import { kindText, cooldownText, pierceText, aoeRadiusText, chainText, buildSkillMechanicsAi, SKILL_AI_CTA } from '@/skillMechanicsText'
import { speedText, armorText, shieldHpText, attackText, attackRangeText, attackIntervalText, flyHeightText, boolText, MECHANISM_EMPTY, buildEnemyMechanicsAi, ENEMY_AI_CTA } from '@/enemyMechanicsText'
// R34 关卡机制导入锚点：bossText, terrainText, armorPermilleText, maxScoreText, MECHANISM_EMPTY, buildLevelMechanicsAi, LEVEL_AI_CTA
// MECHANISM_EMPTY 与 R33 同源同值（'—'），复用上方标识避免重复声明。
import { bossText as level_bossText, terrainText, armorPermilleText, maxScoreText, buildLevelMechanicsAi, LEVEL_AI_CTA } from '@/levelMechanicsText'
// R35 宝石图鉴导入锚点：attrText, qualityText, buildGemMechanicsAi, GEM_AFFIX_NOTE, GEM_AI_BASIS, GEM_AI_HONESTY, GEM_AI_CTA
// MECHANISM_EMPTY 与 R33 同源同值（'—'），复用上方标识避免重复声明。
import { attrText, qualityText, buildGemMechanicsAi, GEM_AFFIX_NOTE, GEM_AI_BASIS, GEM_AI_HONESTY, GEM_AI_CTA } from '@/gemMechanicsText'
import { compositeUnlockText, buildCompositeMechanicsAi, COMPOSITE_AI_BASIS, COMPOSITE_AI_HONESTY, COMPOSITE_AI_CTA } from '@/compositeMechanicsText'
// R37：专精树节点复用上方 MECHANISM_EMPTY（与 @/enemyMechanicsText 同源同值），避免重复声明。
// R37：以命名空间导入专精格式化，避免与 R32 技能表的 kindText 同名冲突；模板调用仍逐字可读。
import * as masteryMechanics from '@/masteryMechanicsText'
// R38：配方表复用上方 MECHANISM_EMPTY（与 @/enemyMechanicsText 同源同值），避免重复声明。
// 单参包装只在模板里出现，底层格式化调用 recipeTextFromHelper(row, nameById.value)。
import { MECHANISM_EMPTY as RECIPE_EMPTY, recipeMaterialsText as recipeTextFromHelper, tierText, buildRecipeMechanicsAi, RECIPE_AI_BASIS, RECIPE_AI_HONESTY, RECIPE_AI_CTA } from '@/recipeMechanicsText'
// R39：皮肤图鉴与掉落配置；MECHANISM_EMPTY 复用上方导入（与 @/enemyMechanicsText 同源同值）。
import { rarityText, passiveText, dropRatesText, buildSkinDropMechanicsAi, SKIN_DROP_AI_BASIS, SKIN_DROP_AI_HONESTY, SKIN_DROP_AI_CTA } from '@/skinDropMechanicsText'
// R40：评分权重表；MECHANISM_EMPTY 复用上方导入（与 @/enemyMechanicsText 同源同值），避免重复声明。
import { RATING_KEYS, ratingLabel, ratingValue, buildRatingMechanicsAi, RATING_AI_BASIS, RATING_AI_HONESTY, RATING_AI_CTA } from '@/ratingMechanicsText'
// R41：章节六字段表导入锚点。bossText 与 R34 同名，用小写后缀别名，模板调用仍逐字可读。
import { rangeText, terrainKindText, bossText, buildChapterMechanicsAi, CHAPTER_AI_BASIS, CHAPTER_AI_HONESTY, CHAPTER_AI_CTA } from '@/chapterMechanicsText'

// R40：评分表行键类型；el-table 槽位的 row 是 any，收窄后才能索引封闭五字段类型。
type RatingWeightKey = (typeof RATING_KEYS)[number]

interface ConfigResp {
  levels: Array<{ id: number; chapter: number; name: string; is_boss: boolean; terrain: unknown[]; difficulty: number; base_hp: number; wave_count: number; energy_cost: number; element_cap: number; max_reaction_tier: number; armor_permille: number; max_score: number; star_targets: number[] }>
  enemies: Array<{ id: number; code: string; name: string; category: string; hp: number; resist: Record<string, number>; speed: number; armor: number; shield_hp: number; attack: number; attack_range: number; attack_interval: number; fly_height: number; burrow: boolean; is_boss: boolean; descr: string }>
  skills: Array<{ id: number; name: string; element: string; family: string; kind: string; base_damage: number; heat_cost: number; cooldown_ms: number; pierce: number; aoe_radius: number; chain: number; apply_stacks: number; descr: string }>
  composite_skills: Array<{ id: number; name: string; element: string; family?: string; descr: string; kind: string; base_damage: number; heat_cost: number; cooldown_ms: number; pierce: number; aoe_radius: number; apply_stacks: number; chain: number; unlock_level: number }>
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
  reactions: Array<{ key: string; name: string; base_coef: number; attack_weight_pct: number; status_duration_ms: number; aoe_radius: number; dispel_shield: boolean; amplify_pct: number; descr: string; spread_stacks: number; armor_shred_permille: number; knockback: number }>
  gems: Array<{ id: number; code: string; name: string; attr: string; descr: string }>
  gem_qualities: Array<{ name: string; affix_count: number; mult: number }>
  chapters: Array<{ id: number; name: string; start_level: number; end_level: number; terrain_kind: string; boss_enemy_id: number }>
  // R40：评分权重与 power.go RatingWeights / types.gen.ts 逐字对齐的封闭五字段，
  // 禁止任意字符串键（旧 Record<string, number> 会把裸英文 key 混进评分表）。
  rating_weights: {
    element_coverage: number
    reaction_coverage: number
    mastery_done: number
    equipment_synergy: number
    mechanic_depth: number
  }
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
  mastery_families: Array<{ family: string; name: string; nodes: Array<{ id: number; family: string; layer: number; slot: number; name: string; kind: string; value: number; prereq_family: string; prereq_layer: number }> }>
  recipes: Array<{ output: number; a: number; b: number; out_tier: number }>
  skins: Array<{ id: number; code: string; name: string; rarity: string; passive: string; descr: string }>
  drop_rates: {
    coin_base: number
    coin_per_star: number
    coin_per_kill: number
    reaction_bonus: number
    keys_on_three_star: number
    energy_on_win: number
  }
}

// ---------- 章节与目录 ----------

type SectionKey =
  | 'basics'
  // R40：构筑评分仍渲染在 basics 区块内，但 CTA/深链需要独立 tab 值（?tab=rating）。
  | 'rating'
  | 'enemies'
  | 'skills'
  | 'gems'
  | 'equipment'
  | 'skins'
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
  { key: 'gems', label: '宝石图鉴' },
  { key: 'equipment', label: '装备图鉴' },
  { key: 'skins', label: '皮肤图鉴' },
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
type HitKind = 'level' | 'enemy' | 'skill' | 'composite' | 'equip' | 'reaction' | 'mastery' | 'gem' | 'quality' | 'recipe' | 'skin' | 'rating' | 'chapter'

const HIT_KINDS: HitKind[] = ['level', 'enemy', 'skill', 'composite', 'equip', 'reaction', 'mastery', 'gem', 'quality', 'recipe', 'skin', 'rating', 'chapter']

const KIND_LABEL: Record<HitKind, string> = {
  level: '关卡',
  enemy: '敌人',
  skill: '技能',
  composite: '合成',
  equip: '装备',
  reaction: '反应',
  mastery: '专精',
  gem: '宝石',
  quality: '宝石品质',
  recipe: '配方',
  skin: '皮肤',
  // t29：评分权重五维度也纳入全站搜索
  rating: '评分',
  // R41：章节六字段（terrain_kind / boss_enemy_id 等）纳入全站搜索
  chapter: '章节',
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
    // 反应表用 key（字符串），宝石品质表用 name（品质名），其余表用 id（数字）
    // R35修复：品质名（gold/gray/...）是搜索主键，没有 id 字段，必须走 name。
    const id = kind === 'quality' ? scope.row.name : scope.row.id ?? scope.row.key
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
  for (const g of c.gems ?? []) {
    hits.push({
      kind: 'gem',
      id: String(g.id),
      title: g.name,
      meta: `宝石 · ${attrText(g.attr)}`,
      body: g.descr ?? '',
      section: 'gems',
    })
  }
  // R35修复：品质名（gold/gray/blue/purple/red）也必须是搜索域，
  // body 带词条数/倍率文本，让「gold」与倍率词都能命中。
  for (const q of c.gem_qualities ?? []) {
    hits.push({
      kind: 'quality',
      id: q.name,
      title: q.name,
      meta: `宝石品质 · ${qualityText(q)}`,
      body: `${qualityText(q)} 词条数 倍率`,
      section: 'gems',
    })
  }
  for (const r of c.recipes ?? []) {
    const map = nameById.value
    const output = recipeOutputText(r.output)
    const materials = recipeTextFromHelper(r, map)
    hits.push({
      kind: 'recipe',
      id: String(r.output),
      title: output,
      meta: '合成配方',
      body: `${materials} ${tierText(r.out_tier)} 合成配方${output}`,
      section: 'skills',
      anchor: 'composite',
    })
  }
  for (const s of c.skins ?? []) {
    hits.push({
      kind: 'skin',
      id: String(s.id),
      title: s.name,
      meta: `皮肤 · ${rarityText(s.rarity)}`,
      body: `${s.name} ${s.code} 皮肤`,
      section: 'skins',
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
  // t29：评分权重五维度纳入全站搜索。key / 中文标签 / 权重整数都在 body 里，
  // 让「反应链」「mechanic_depth」「元素覆盖」等中英文词都能命中；
  // click 后 pick() 写入 focus=rating:<key>，评分表行按 key 高亮。
  for (const key of RATING_KEYS) {
    hits.push({
      kind: 'rating',
      id: key,
      title: ratingLabel(key),
      meta: `评分 · ${key}`,
      body: `${key} ${ratingLabel(key)} 权重 ${ratingValue(c?.rating_weights?.[key])} 评分权重`,
      section: 'rating',
    })
  }
  // R41：章节六字段纳入全站搜索；点击后跳关卡 tab 的章节表并按 id 高亮。
  for (const ch of c.chapters ?? []) {
    hits.push({
      kind: 'chapter',
      id: String(ch.id),
      title: ch.name,
      meta: `${rangeText(ch.start_level, ch.end_level)} · 章节 ${ch.id}`,
      body: `${ch.name} ${ch.id} ${terrainKindText(ch.terrain_kind)} ${ch.terrain_kind} 章节`,
      section: 'levels',
      anchor: 'chapter-mechanics',
    })
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

/**
 * 受击增伤是千分比：服务端 AmplifyPermille（superconduct=600 → +60%），
 * 直接拼 % 会把效果夸大 10 倍，与 miniapp 反应图鉴口径也不一致。
 */
function amplifyPctText(v: number): number {
  return v / 10
}

/** 空态文案（2026-10-21 P5-8）：空表要说清「为什么空」。 */
const emptyText = '暂无数据'
const loading = ref(false)
const cfg = ref<ConfigResp | null>(null)

// R38：技能 id → 名称映射。技能表与合成技能表都是公开 /config 的一部分，
// 合并进同一张 Map 后，材料/产物列能显示真实技能名而不是 #id。
const nameById = computed<Map<number, string>>(() => {
  const m = new Map<number, string>()
  for (const s of cfg.value?.skills ?? []) m.set(s.id, s.name)
  for (const s of cfg.value?.composite_skills ?? []) m.set(s.id, s.name)
  return m
})

/** 配方材料列的单参包装：底层 helper 仍是可测的纯二参函数。 */
function recipeMaterialsText(row: unknown): string {
  return recipeTextFromHelper(row, nameById.value)
}

/** 产物列复用同一张映射，未知 id 逐字标注，绝不猜测技能名。 */
function recipeOutputText(output: unknown): string {
  if (typeof output !== 'number' || !Number.isInteger(output) || output <= 0) return RECIPE_EMPTY
  const name = nameById.value.get(output)
  return typeof name === 'string' && name.length > 0 ? name : `（未知技能#${output}）`
}
const reactUsage = ref<Array<{ reaction: string; count: number }>>([])

/** R6 §1E：知识缺口建议（接口优先，未上线回退 /config 真实数据）。 */
const recInsight = ref<AiInsight | null>(null)
const aiInsight = computed<AiInsight>(() => recInsight.value ?? buildLocalRecommendation('wiki', {
  levels: cfg.value?.levels ?? [], enemies: cfg.value?.enemies ?? [], skills: cfg.value?.skills ?? [],
}))

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
    // R6：推荐接口锦上添花，未上线/失败都不影响文档站
    try {
      recInsight.value = insightFromRecommendation(await fetchAdminRecommendation('wiki'), aiInsight.value)
    } catch { recInsight.value = null }
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
    <AiInsightCard :insight="aiInsight" :loading="loading" />
    <!-- R9-WIKI(t89)：内容区眉题（§1B.2 SectionKicker），沿用全局 .section-kicker 令牌样式 -->
    <div class="section-kicker">
      <span class="section-kicker-cn">玩法知识库</span>
      <span class="section-kicker-en">PLAYER GUIDE</span>
    </div>
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
        placeholder="搜索关卡 / 敌人 / 技能 / 装备 / 反应 / 专精 / 评分（回车跳第一条）"
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
        <section v-show="activeTab === 'basics' || activeTab === 'rating'" class="wiki-section">
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
                  +{{ amplifyPctText(row.amplify_pct) }}%
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
            <!-- R31：服务端 /config 已下发、引擎已消费的三个机制字段，
                 此处只做真实展示；单位口径见 @/reactionMechanicsText，零值显示破折号。 -->
            <el-table-column label="层数传播" width="90">
              <template #default="{ row }">{{ spreadStacksText(row.spread_stacks) }}</template>
            </el-table-column>
            <el-table-column label="削甲" width="80">
              <template #default="{ row }">{{ armorShredText(row.armor_shred_permille) }}</template>
            </el-table-column>
            <el-table-column label="击退" width="80">
              <template #default="{ row }">{{ knockbackText(row.knockback) }}</template>
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
            表示该反应没有溅射。腐蚀扩散的**层数传播已实现**（传播 1 层、
            半径 120px、不造成伤害、不触发新反应）；「层数传播 / 削甲 / 击退」
            三列显示的是已实现机制的真实参数。**燃烧云 burn_cloud 的火区仍未实现**
            （C1 产品阻塞，本轮不实现），Wiki 不暗示其已生效。
            <br />
            「攻击力权重」就是这一条反应里由面板攻击力贡献的比例上限。全部 ≤30% ——
            这是保证低养成玩家能靠技巧翻盘的结构性约束，不是可以随数值膨胀绕过的软性限制。
          </p>

          <section class="r40-rating-mechanics">
            <h4 id="rating">构筑评分（I-7）</h4>
            <p>主指标不是总战力，而是这五个维度：</p>
            <div class="r40-rating-table-scroll">
              <el-table :data="RATING_KEYS.map((key) => ({ key }))" size="small" border :row-class-name="rowFocusClass('rating')">
                <el-table-column label="维度">
                  <template #default="{ row }">{{ ratingLabel(row.key) }}</template>
                </el-table-column>
                <el-table-column label="权重">
                  <template #default="{ row }">{{ ratingValue(cfg?.rating_weights?.[row.key as RatingWeightKey]) }}</template>
                </el-table-column>
                <template #empty>
                  <el-empty :description="emptyText" :image-size="60" />
                </template>
              </el-table>
            </div>
            <div class="r40-ai-note">
              <p class="r40-ai-conclusion"><strong>AI 机制概览：</strong>{{ buildRatingMechanicsAi(cfg?.rating_weights ?? {}) }}</p>
              <p class="r40-ai-basis" :title="RATING_AI_BASIS">依据：公开 /api/v1/config 的 rating_weights，与服务端 RatingWeights / DefaultRatingWeights 同源；仅展示配置基值</p>
              <p class="r40-ai-honesty" :data-r40-honesty="RATING_AI_HONESTY">评分权重是配置基值，不是权威结算；实际收益与边界以游戏内为准；权威回放以 replay_hash 为准</p>
              <p class="r40-ai-cta" :data-r40-cta="RATING_AI_CTA"><a :href="urlFor('rating')" @click.prevent="goTo({ tab: 'rating' })">查看构筑评分 →</a></p>
            </div>
          </section>
        </section>

        <!-- 元素矩阵 -->
        <section v-show="activeTab === 'enemies'" class="wiki-section r33-enemy-mechanics">
          <p class="note">
            绿色为<strong>弱点</strong>（负抗性，伤害放大），蓝色为<strong>抗性</strong>（减伤）。
            全部 BOSS 都被设计成「必有解」：任意一只都至少有两到三种元素是弱点。
          </p>
          <div class="r33-enemy-table-scroll">
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
            <el-table-column label="速度" width="100">
              <template #default="{ row }">{{ speedText(row.speed) }}</template>
            </el-table-column>
            <el-table-column label="护甲" width="80">
              <template #default="{ row }">{{ armorText(row.armor) }}</template>
            </el-table-column>
            <el-table-column label="护盾" width="80">
              <template #default="{ row }">{{ shieldHpText(row.shield_hp) }}</template>
            </el-table-column>
            <el-table-column label="攻击" width="80">
              <template #default="{ row }">{{ attackText(row.attack) }}</template>
            </el-table-column>
            <el-table-column label="射程" width="80">
              <template #default="{ row }">{{ attackRangeText(row.attack_range) }}</template>
            </el-table-column>
            <el-table-column label="攻击间隔" width="100">
              <template #default="{ row }">{{ attackIntervalText(row.attack_interval) }}</template>
            </el-table-column>
            <el-table-column label="飞行高度" width="100">
              <template #default="{ row }">{{ flyHeightText(row.fly_height) }}</template>
            </el-table-column>
            <el-table-column label="潜地" width="70">
              <template #header>潜地<span class="r33-empty-hint">（{{ MECHANISM_EMPTY }} = 无此机制）</span></template>
              <template #default="{ row }">{{ boolText(row.burrow) }}</template>
            </el-table-column>
            <el-table-column label="首领" width="70">
              <template #default="{ row }">{{ boolText(row.is_boss) }}</template>
            </el-table-column>
            <el-table-column prop="descr" label="说明" min-width="150" />
            <template #empty>
              <el-empty :description="emptyText" :image-size="60" />
            </template>
          </el-table>
          </div>
          <div class="r33-ai-note">
            <p class="r33-ai-conclusion"><strong>AI 机制概览：</strong>{{ buildEnemyMechanicsAi(cfg?.enemies ?? []) }}</p>
            <p class="r33-ai-basis">依据：公开 /api/v1/config 的 enemies（与 H5/Go 引擎同源 SeedEnemy）；展示为配置基值，实际效果以引擎结算为准</p>
            <p class="r33-ai-cta" :data-r33-cta="ENEMY_AI_CTA"><a :href="urlFor('enemies')" @click.prevent="goTo({ tab: 'enemies' })">查看敌人列表 →</a></p>
            <p class="r33-ai-honesty">辅助展示非权威战斗数据，实际效果以引擎结算为准；权威回放以 replay_hash 为准</p>
          </div>
        </section>

        <!-- 技能 -->
        <section v-show="activeTab === 'skills'" class="wiki-section">
          <div id="skill-table" class="r32-skill-mechanics">
            <div class="r32-skill-table-scroll">
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
                <el-table-column label="冷却" width="86">
                  <template #default="{ row }">{{ cooldownText(row.cooldown_ms) }}</template>
                </el-table-column>
                <el-table-column label="类型" width="80">
                  <template #default="{ row }">{{ kindText(row.kind) }}</template>
                </el-table-column>
                <el-table-column label="穿透" width="80">
                  <template #default="{ row }">{{ pierceText(row.pierce) }}</template>
                </el-table-column>
                <el-table-column label="范围" width="90">
                  <template #default="{ row }">{{ aoeRadiusText(row.aoe_radius) }}</template>
                </el-table-column>
                <el-table-column label="连锁" width="80">
                  <template #default="{ row }">{{ chainText(row.chain) }}</template>
                </el-table-column>
                <el-table-column prop="apply_stacks" label="层数" width="66" />
                <el-table-column prop="descr" label="说明" min-width="180" />
                <template #empty>
                  <el-empty :description="emptyText" :image-size="60" />
                </template>
              </el-table>
            </div>
            <div class="r32-ai-note">
              <p class="r32-ai-conclusion"><strong>AI 机制概览：</strong>{{ buildSkillMechanicsAi(cfg?.skills ?? []) }}</p>
              <p class="r32-ai-basis">依据：公开 /api/v1/config 的 skills（与 H5 引擎同源 SeedSkill）；格式化仅展示，不改结算</p>
              <p class="r32-ai-cta" :data-r32-cta="SKILL_AI_CTA"><a :href="urlFor('skills', 'composite')" @click.prevent="goTo({ tab: 'skills', anchor: 'composite' })">查看合成技能 →</a></p>
              <p class="r32-ai-honesty">辅助展示非权威战斗数据，实际效果以引擎结算为准；权威回放以 replay_hash 为准</p>
            </div>
          </div>

          <div id="composite" class="r36-composite-mechanics" style="margin-top: 16px">
            <h4>合成技能</h4>
            <div class="r36-composite-table-scroll">
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
                    <span :style="{ color: ELEMENT_COLOR[row.element] }">{{ ELEMENT_LABEL[row.element] ?? row.element }}</span>
                  </template>
                </el-table-column>
                <el-table-column prop="descr" label="说明" min-width="220" />
                <el-table-column label="类型" width="80">
                  <template #default="{ row }">{{ kindText(row.kind) }}</template>
                </el-table-column>
                <el-table-column label="冷却" width="86">
                  <template #default="{ row }">{{ cooldownText(row.cooldown_ms) }}</template>
                </el-table-column>
                <el-table-column prop="base_damage" label="伤害" width="76" />
                <el-table-column prop="heat_cost" label="热量" width="70" />
                <el-table-column label="穿透" width="80">
                  <template #default="{ row }">{{ pierceText(row.pierce) }}</template>
                </el-table-column>
                <el-table-column label="范围" width="90">
                  <template #default="{ row }">{{ aoeRadiusText(row.aoe_radius) }}</template>
                </el-table-column>
                <el-table-column label="连锁" width="80">
                  <template #default="{ row }">{{ chainText(row.chain) }}</template>
                </el-table-column>
                <el-table-column label="层数" width="66">
                  <template #default="{ row }">
                    {{ typeof row.apply_stacks === 'number' && Number.isInteger(row.apply_stacks) && row.apply_stacks > 0 ? row.apply_stacks : MECHANISM_EMPTY }}
                  </template>
                </el-table-column>
                <el-table-column label="解锁等级" width="90">
                  <template #default="{ row }">{{ compositeUnlockText(row.unlock_level) }}</template>
                </el-table-column>
                <template #empty>
                  <el-empty :description="emptyText" :image-size="60" />
                </template>
              </el-table>
            </div>
            <div class="r36-ai-note">
              <p class="r36-ai-conclusion"><strong>AI 机制概览：</strong>{{ buildCompositeMechanicsAi(cfg?.composite_skills ?? []) }}</p>
              <p class="r36-ai-basis" :title="COMPOSITE_AI_BASIS" :data-ai-basis="COMPOSITE_AI_BASIS">依据：公开 /api/v1/config 的 composite_skills（与服务端 SeedSkill / SeedCompositeSkills 同源）；格式化仅展示，不改结算</p>
              <p class="r36-ai-cta" :data-r36-cta="COMPOSITE_AI_CTA"><a :href="urlFor('skills', 'composite')" @click.prevent="goTo({ tab: 'skills', anchor: 'composite' })">查看合成技能 →</a></p>
              <p class="r36-ai-honesty" :title="COMPOSITE_AI_HONESTY" :data-ai-honesty="COMPOSITE_AI_HONESTY">辅助展示非权威战斗数据，展示为配置基值，实际效果以引擎结算为准；权威回放以 replay_hash 为准</p>
            </div>
            <div class="r38-recipe-mechanics" style="margin-top: 16px">
              <h4>合成配方</h4>
              <p class="note">
                配方只说明「由哪两个技能熔炼、产物几阶」：<strong>能否熔炼以游戏内材料拥有状态为准</strong>；
                这里不展示战斗数值或词条范围，配置基值非权威战斗数据。
              </p>
              <div class="r38-recipe-table-scroll">
                <el-table :data="cfg?.recipes ?? []" size="small" border stripe :row-class-name="rowFocusClass('recipe')">
                  <el-table-column label="产物" min-width="140">
                    <template #default="{ row }">{{ recipeOutputText(row.output) }}</template>
                  </el-table-column>
                  <el-table-column label="材料A" min-width="140">
                    <template #default="{ row }">{{ recipeMaterialsText(row).split(' + ')[0] ?? RECIPE_EMPTY }}</template>
                  </el-table-column>
                  <el-table-column label="材料B" min-width="140">
                    <template #default="{ row }">{{ recipeMaterialsText(row).split(' + ')[1] ?? RECIPE_EMPTY }}</template>
                  </el-table-column>
                  <el-table-column label="阶数" width="90">
                    <template #default="{ row }">{{ tierText(row.out_tier) }}</template>
                  </el-table-column>
                  <template #empty>
                    <el-empty :description="emptyText" :image-size="60" />
                  </template>
                </el-table>
              </div>
              <div class="r38-ai-note">
                <p class="r38-ai-conclusion"><strong>AI 机制概览：</strong>{{ buildRecipeMechanicsAi(cfg?.recipes ?? [], cfg?.skills ?? [], cfg?.composite_skills ?? []) }}</p>
                <p class="r38-ai-basis" :title="RECIPE_AI_BASIS" :data-ai-basis="RECIPE_AI_BASIS">依据：公开 /api/v1/config 的 recipes（与服务端 SeedRecipe / SeedRecipes 同源）；仅展示配方，不改熔炼结算</p>
                <p class="r38-ai-cta" :data-r38-cta="RECIPE_AI_CTA"><a :href="urlFor('skills', 'composite')" @click.prevent="goTo({ tab: 'skills', anchor: 'composite' })">查看合成配方 →</a></p>
                <p class="r38-ai-honesty" :title="RECIPE_AI_HONESTY" :data-ai-honesty="RECIPE_AI_HONESTY">能否熔炼以游戏内材料拥有状态为准；配置基值非权威战斗数据，实际效果以引擎结算为准，权威回放以 replay_hash 为准</p>
              </div>
            </div>
          </div>
        </section>

        <!-- 装备（P6-12）-->
        <section v-show="activeTab === 'equipment'" class="wiki-section">
          <el-alert
            type="info"
            :closable="false"
            show-icon
            title="装备怎么生效"
            description="装备提供护甲（减少漏怪造成的伤害）与通用攻击加成。**元素标签当前不参与任何结算** —— 按标签给加成的机制没有实现，所以这里不宣传它。一个部位同时只能装一件。" class="mb-2"
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

        <!-- R39：皮肤图鉴（SECTIONS 顺序：equipment → skins → rules） -->
        <section v-show="activeTab === 'skins'" class="wiki-section r39-skin-mechanics">
          <el-alert
            type="info"
            :closable="false"
            show-icon
            title="皮肤图鉴"
            description="皮肤被动是机制型词条 code，原样展示不翻译；实际结算与获得以游戏内为准。"
            class="mb-2"
          />
          <div class="r39-skin-table-scroll">
            <el-table :data="cfg?.skins ?? []" size="small" border stripe :row-class-name="rowFocusClass('skin')">
              <el-table-column prop="id" label="ID" width="60" />
              <el-table-column prop="name" label="名称" min-width="130" />
              <el-table-column label="稀有度" width="90">
                <template #default="{ row }">{{ rarityText(row.rarity) }}</template>
              </el-table-column>
              <el-table-column label="被动" min-width="140">
                <template #default="{ row }">{{ passiveText(row.passive) }}</template>
              </el-table-column>
              <el-table-column prop="descr" label="说明" min-width="220" />
              <template #empty>
                <el-empty :description="emptyText" :image-size="60" />
              </template>
            </el-table>
          </div>
          <div class="r39-ai-note">
            <p class="r39-ai-conclusion"><strong>AI 机制概览：</strong>{{ buildSkinDropMechanicsAi(cfg?.skins ?? [], cfg?.drop_rates ?? {}) }}</p>
            <p class="r39-ai-basis" :title="SKIN_DROP_AI_BASIS" :data-ai-basis="SKIN_DROP_AI_BASIS">依据：公开 /api/v1/config 的 skins（与服务端 SeedSkin / SeedSkins 同源）与 drop_rates（与服务端 DropRates / DefaultDropRates 同源）；仅展示配置基值</p>
            <p class="r39-ai-cta" :data-r39-cta="SKIN_DROP_AI_CTA"><a :href="urlFor('skins')" @click.prevent="goTo({ tab: 'skins' })">查看皮肤图鉴 →</a></p>
            <p class="r39-ai-honesty" :title="SKIN_DROP_AI_HONESTY" :data-ai-honesty="SKIN_DROP_AI_HONESTY">皮肤被动与掉落均为配置基值，实际结算与获得以游戏内为准；权威回放以 replay_hash 为准</p>
          </div>
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
          <el-table :data="starRatioRows" size="small" border class="mb-1">
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

          <h4 id="drop-rates">结算掉落配置</h4>
          <p class="note">
            下列是 <code>/api/v1/config</code> 下发的<strong>配置基值</strong>（不是百分比、也不是千分比）；实际获得以游戏内结算为准。
          </p>
          <div class="r39-drop-table">
            <el-table
              :data="[
                { label: '基础金币', value: dropRatesText(cfg?.drop_rates?.coin_base) },
                { label: '每星金币', value: dropRatesText(cfg?.drop_rates?.coin_per_star) },
                { label: '每次击杀金币', value: dropRatesText(cfg?.drop_rates?.coin_per_kill) },
                { label: '反应加成', value: dropRatesText(cfg?.drop_rates?.reaction_bonus) },
                { label: '三星钥匙', value: dropRatesText(cfg?.drop_rates?.keys_on_three_star) },
                { label: '通关返还体力', value: dropRatesText(cfg?.drop_rates?.energy_on_win) },
              ]"
              size="small"
              border
            >
              <el-table-column prop="label" label="规则" />
              <el-table-column prop="value" label="数值" />
              <template #empty>
                <el-empty :description="emptyText" :image-size="60" />
              </template>
            </el-table>
          </div>

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

        <section v-show="activeTab === 'mastery'" class="wiki-section r37-mastery-mechanics">
          <p class="note">
            每层只能选 2 个节点（4 选 2），且第 2、3 层必须先点亮前一层。
            专精点不够点满任何一系 —— <strong>必须做方向选择</strong>。
            下表展示公开配置基值（效果口径按 power.go），实际效果以引擎结算为准。
          </p>
          <div
            v-for="f in cfg?.mastery_families ?? []"
            :key="f.family"
            class="family-block r37-mastery-table-scroll"
          >
            <div class="family-name">{{ f.name }}</div>
            <el-table
              :data="f.nodes ?? []"
              size="small"
              border
              stripe
            >
              <el-table-column label="系" width="100">
                <template #default>{{ f.name }}</template>
              </el-table-column>
              <el-table-column label="层" width="60">
                <template #default="{ row: n }">{{ n.layer }}</template>
              </el-table-column>
              <el-table-column label="槽" width="60">
                <template #default="{ row: n }">{{ n.slot }}</template>
              </el-table-column>
              <el-table-column label="节点" min-width="120">
                <template #default="{ row: n }">
                  <span :class="{ 'node-focus': focus?.kind === 'mastery' && focus.id === n.name }">{{ n.name }}</span>
                </template>
              </el-table-column>
              <el-table-column label="效果" width="110">
                <template #default="{ row: n }">{{ masteryMechanics.kindText(n.kind) }}</template>
              </el-table-column>
              <el-table-column label="数值" width="140">
                <template #default="{ row: n }">{{ masteryMechanics.valueText(n.kind, n.value) }}</template>
              </el-table-column>
              <el-table-column label="前置" width="150">
                <template #default="{ row: n }">{{ masteryMechanics.prereqText(n.prereq_family, n.prereq_layer) }}</template>
              </el-table-column>
              <template #empty>
                <el-empty :description="emptyText" :image-size="60" />
              </template>
            </el-table>
          </div>
          <div class="r37-ai-note">
            <p class="r37-ai-conclusion"><strong>AI 机制概览：</strong>{{ masteryMechanics.buildMasteryMechanicsAi(cfg?.mastery_families ?? []) }}</p>
            <p class="r37-ai-basis" :title="masteryMechanics.MASTERY_AI_BASIS" :data-ai-basis="masteryMechanics.MASTERY_AI_BASIS">依据：公开 /api/v1/config 的 mastery_families（与服务端 MasteryFamily / MasteryNode 同源）；效果按 power.go 口径展示，实际以引擎结算为准</p>
            <p class="r37-ai-cta" :data-r37-cta="masteryMechanics.MASTERY_AI_CTA"><a :href="urlFor('mastery')" @click.prevent="goTo({ tab: 'mastery' })">查看专精树 →</a></p>
            <p class="r37-ai-honesty" :title="masteryMechanics.MASTERY_AI_HONESTY" :data-ai-honesty="masteryMechanics.MASTERY_AI_HONESTY">辅助展示配置基值，非权威战斗数据；实际效果以引擎结算为准，权威回放以 replay_hash 为准</p>
          </div>
        </section>

        <!-- 关卡 -->
        <section v-show="activeTab === 'levels'" class="wiki-section r34-level-mechanics">
          <el-select v-model="activeChapter" style="width: 160px; margin-bottom: 12px">
            <el-option v-for="c in cfg?.chapters ?? []" :key="c.id" :label="c.name" :value="c.id" />
          </el-select>

          <!-- R41：章节元数据六字段表（配置基值，非权威结算） -->
          <div class="r41-chapter-mechanics" id="chapter-mechanics">
            <h3 class="r41-chapter-title">章节一览（六字段 · 配置基值）</h3>
            <div class="r41-chapter-table-scroll">
              <el-table
                :data="cfg?.chapters ?? []"
                size="small"
                border
                stripe
                :row-class-name="rowFocusClass('chapter')"
              >
                <el-table-column prop="id" label="ID" width="60" />
                <el-table-column prop="name" label="名称" min-width="140" />
                <el-table-column label="关卡区间" min-width="110">
                  <template #default="{ row }">{{ rangeText(row.start_level, row.end_level) }}</template>
                </el-table-column>
                <el-table-column label="地形（配置基值）" min-width="150">
                  <template #default="{ row }">{{ terrainKindText(row.terrain_kind) }}</template>
                </el-table-column>
                <el-table-column label="首领敌人" min-width="100">
                  <template #default="{ row }">{{ bossText(row.boss_enemy_id) }}</template>
                </el-table-column>
                <el-table-column label="说明" min-width="180">
                  <template #default="{ row }">{{ rangeText(row.start_level, row.end_level) }} · {{ terrainKindText(row.terrain_kind) }}（配置基值）</template>
                </el-table-column>
                <template #empty>
                  <el-empty :description="emptyText" :image-size="60" />
                </template>
              </el-table>
            </div>
            <div class="r41-ai-note">
              <p class="r41-ai-conclusion"><strong>AI 章节概览：</strong>{{ buildChapterMechanicsAi(cfg?.chapters ?? []) }}</p>
              <p class="r41-ai-basis" :title="CHAPTER_AI_BASIS" :data-ai-basis="CHAPTER_AI_BASIS">依据：公开 /api/v1/config 的 chapters（与服务端 domain.AllChapters 同源）；章节 terrain_kind 为配置基值，实际地形触发以结算 terrain_used 为准</p>
              <p class="r41-ai-cta" :data-r41-cta="CHAPTER_AI_CTA"><a :href="urlFor('levels')" @click.prevent="goTo({ tab: 'levels' })">查看章节一览 →</a></p>
              <p class="r41-ai-honesty" :title="CHAPTER_AI_HONESTY" :data-ai-honesty="CHAPTER_AI_HONESTY">章节 terrain_kind 是配置基值，与关卡 terrain_used（参与 replay_hash）不同概念，不得混同权威战斗数据；示例值不硬编码，差异以公开 /api/v1/config 为准</p>
            </div>
          </div>

          <div class="r34-level-table-scroll">
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
              <el-table-column label="首领" width="70">
                <template #default="{ row }">{{ level_bossText(row.is_boss) }}</template>
              </el-table-column>
              <el-table-column label="地形（配置基值）" min-width="180">
                <template #default="{ row }">{{ terrainText(row.terrain) }}</template>
              </el-table-column>
              <el-table-column label="护甲‰" width="82">
                <template #default="{ row }">{{ armorPermilleText(row.armor_permille) }}</template>
              </el-table-column>
              <el-table-column label="满分" width="78">
                <template #default="{ row }">{{ maxScoreText(row.max_score) }}</template>
              </el-table-column>
              <template #empty>
                <el-empty :description="emptyText" :image-size="60" />
              </template>
            </el-table>
          </div>
          <div class="r34-ai-note">
            <p><strong>AI 机制概览：</strong>{{ buildLevelMechanicsAi(cfg?.levels ?? []) }}</p>
            <p class="r34-ai-basis">依据：公开 /api/v1/config 的 levels（与 H5/Go 引擎同源 SeedLevel）；地形实际触发以结算 terrain_used 为准</p>
            <p class="r34-ai-cta"><a data-r34-cta :href="urlFor('levels')" @click.prevent="goTo({ tab: 'levels' })">{{ LEVEL_AI_CTA }}</a></p>
            <p class="r34-ai-honesty">辅助展示非权威战斗数据，实际效果以引擎结算为准；权威回放以 replay_hash 为准</p>
          </div>
        </section>

        <!-- 宝石图鉴 -->
        <section v-show="activeTab === 'gems'" class="wiki-section r35-gem-mechanics">
          <p class="note">
            宝石属性与品质档位来自公开 /api/v1/config，与服务端 SeedGem / GemQualities 同源。
            <strong>词条随机，具体数值以游戏内为准</strong> —— Wiki 不展示具体词条数值范围。
            系数为千分比（1000‰ 即原始系数），不换算成百分比。
          </p>
          <div class="r35-gem-table-scroll">
            <el-table
              :data="cfg?.gems ?? []"
              size="small"
              border
              stripe
              :row-class-name="rowFocusClass('gem')"
            >
              <el-table-column prop="id" label="ID" width="55" />
              <el-table-column prop="name" label="名称" min-width="120" />
              <el-table-column label="属性" width="120">
                <template #default="{ row }">{{ attrText(row.attr) }}</template>
              </el-table-column>
              <el-table-column label="说明" min-width="180">
                <template #default="{ row }">{{ row.descr }}</template>
              </el-table-column>
              <template #empty>
                <el-empty :description="emptyText" :image-size="60" />
              </template>
            </el-table>
          </div>
          <h4 style="margin-top: 16px">宝石品质</h4>
          <div class="r35-gem-table-scroll">
            <el-table :data="cfg?.gem_qualities ?? []" size="small" border stripe :row-class-name="rowFocusClass('quality')">
              <el-table-column prop="name" label="品质" width="140" />
              <el-table-column label="词条数" width="110">
                <template #default="{ row }">{{ qualityText(row).split(' · ')[1] ?? MECHANISM_EMPTY }}</template>
              </el-table-column>
              <el-table-column label="系数" width="120">
                <template #default="{ row }">{{ qualityText(row).split(' · ')[2] ?? MECHANISM_EMPTY }}</template>
              </el-table-column>
              <template #empty>
                <el-empty :description="emptyText" :image-size="60" />
              </template>
            </el-table>
          </div>
          <div class="r35-ai-note">
            <p class="r35-affix-note" :data-affix-note="GEM_AFFIX_NOTE">词条随机，具体数值以游戏内为准</p>
            <p class="r35-ai-conclusion"><strong>AI 机制概览：</strong>{{ buildGemMechanicsAi(cfg?.gems ?? [], cfg?.gem_qualities ?? []) }}</p>
            <p class="r35-ai-basis" :data-basis="GEM_AI_BASIS">依据：公开 /api/v1/config 的 gems 与 gem_qualities（与服务端 SeedGem / GemQualities 同源）；词条随机，具体数值以游戏内为准</p>
            <p class="r35-ai-cta"><a data-r35-cta :href="urlFor('gems')" @click.prevent="goTo({ tab: 'gems' })">{{ GEM_AI_CTA }}</a></p>
            <p class="r35-ai-honesty" :data-honesty="GEM_AI_HONESTY">辅助展示非权威战斗数据，实际效果以引擎结算为准；权威回放以 replay_hash 为准</p>
          </div>
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

/* R9-WIKI(t89)：内容区眉题与标题的间距（全局 .section-kicker 样式在 main.css） */
.wiki > .section-kicker {
  margin-top: 20px;
}
code {
  font-family: ui-monospace, monospace;
  font-size: 11px;
  background: var(--lyp-panel-2);
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
  background: var(--lyp-bg);
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
  background: var(--lyp-panel-2);
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
  background: var(--lyp-panel-2);
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
  background: var(--lyp-panel-2);
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
  background: var(--lyp-panel-2);
  border-radius: 2px;
  overflow: hidden;
}
.resist-zero {
  position: absolute;
  left: 50%;
  top: 0;
  bottom: 0;
  width: 1px;
  background: var(--lyp-border);
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
  background: var(--lyp-panel-2);
  border-radius: 3px;
  padding: 2px 7px;
}

/* ---------- 表格：窄屏横向滚动，而不是被压扁 ---------- */

.wiki-main :deep(.el-table) {
  min-width: var(--wiki-table-min, 0);
}

@media (max-width: 900px) {
  /* ⚠️ 与 AdminLayout 的表格策略一致：滚动发生在这一层，页面本身不横溢。
     这里不用 `min-width` 撑宽表格（列本来就带 min-width，会自己撑开），
     只负责给一个可滚动的容器。 */
  .wiki-main :deep(.el-table__body-wrapper),
  .wiki-main :deep(.el-scrollbar__wrap) {
    overflow-x: auto;
  }
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

/* R32：技能机制表表内横向滚动，不造成页面级溢出 */
.r32-skill-table-scroll {
  overflow-x: auto;
  max-width: 100%;
}
.r32-ai-note {
  margin-top: 12px;
  padding: 10px 12px;
  border: 1px solid var(--lyp-border);
  border-radius: 6px;
  font-size: 12px;
  line-height: 1.6;
}
.r32-ai-note p {
  margin: 2px 0;
}
.r32-ai-basis,
.r32-ai-honesty {
  color: var(--lyp-muted);
}
.r32-ai-cta a {
  color: var(--lyp-accent);
}
/* R33：敌人机制表表内横向滚动，不造成页面级溢出 */
.r33-enemy-table-scroll {
  overflow-x: auto;
  max-width: 100%;
}
.r33-ai-note {
  margin-top: 12px;
  padding: 10px 12px;
  border: 1px solid var(--lyp-border);
  border-radius: 6px;
  font-size: 12px;
  line-height: 1.6;
}
.r33-ai-note p {
  margin: 2px 0;
}
.r33-ai-basis,
.r33-ai-honesty {
  color: var(--lyp-muted);
}
.r33-ai-cta a {
  color: var(--lyp-accent);
}
/* R41：章节六字段表表内横向滚动，不造成页面级溢出 */
.r41-chapter-table-scroll {
  overflow-x: auto;
  max-width: 100%;
}
.r41-chapter-title {
  font-size: 13px;
  margin: 4px 0 8px;
}
.r41-ai-note {
  margin: 10px 0 14px;
  padding: 10px 12px;
  border: 1px solid var(--lyp-border);
  border-radius: 6px;
  font-size: 12px;
  line-height: 1.6;
}
.r41-ai-note p {
  margin: 2px 0;
}
.r41-ai-basis,
.r41-ai-honesty {
  color: var(--lyp-muted);
}
.r41-ai-cta a {
  color: var(--lyp-accent);
}
/* R34：关卡机制表表内横向滚动，不造成页面级溢出 */
.r34-level-table-scroll {
  overflow-x: auto;
  max-width: 100%;
}
.r34-ai-note {
  margin-top: 12px;
  padding: 10px 12px;
  border: 1px solid var(--lyp-border);
  border-radius: 6px;
  font-size: 12px;
  line-height: 1.6;
}
.r34-ai-note p {
  margin: 2px 0;
}
.r34-ai-basis,
.r34-ai-honesty {
  color: var(--lyp-muted);
}
.r34-ai-cta a {
  color: var(--lyp-accent);
}
/* R35：宝石图鉴两表表内横向滚动，不造成页面级溢出 */
.r35-gem-table-scroll {
  overflow-x: auto;
  max-width: 100%;
}
/* R36：合成技能表表内横向滚动，不造成页面级溢出 */
.r36-composite-table-scroll {
  overflow-x: auto;
  max-width: 100%;
}
.r36-ai-note {
  margin-top: 12px;
  padding: 10px 12px;
  border: 1px solid var(--lyp-border);
  border-radius: 6px;
  font-size: 12px;
  line-height: 1.6;
}
.r36-ai-note p {
  margin: 2px 0;
}
.r36-ai-basis,
.r36-ai-honesty {
  color: var(--lyp-muted);
}
.r36-ai-cta a {
  color: var(--lyp-accent);
}
/* R37：专精节点表表内横向滚动，不造成页面级溢出 */
.r37-mastery-table-scroll {
  overflow-x: auto;
  max-width: 100%;
}
.r37-ai-note {
  margin-top: 12px;
  padding: 10px 12px;
  border: 1px solid var(--lyp-border);
  border-radius: 6px;
  font-size: 12px;
  line-height: 1.6;
}
.r37-ai-note p {
  margin: 2px 0;
}
.r37-ai-basis,
.r37-ai-honesty {
  color: var(--lyp-muted);
}
.r37-ai-cta a {
  color: var(--lyp-accent);
}
/* R38：合成配方表表内横向滚动，不造成页面级溢出 */
.r38-recipe-table-scroll {
  overflow-x: auto;
  max-width: 100%;
}
.r38-ai-note {
  margin-top: 12px;
  padding: 10px 12px;
  border: 1px solid var(--lyp-border);
  border-radius: 6px;
  font-size: 12px;
  line-height: 1.6;
}
.r38-ai-note p {
  margin: 2px 0;
}
.r38-ai-basis,
.r38-ai-honesty {
  color: var(--lyp-muted);
}
.r38-ai-cta a {
  color: var(--lyp-accent);
}
.r35-ai-note {
  margin-top: 12px;
  padding: 10px 12px;
  border: 1px solid var(--lyp-border);
  border-radius: 6px;
  font-size: 12px;
  line-height: 1.6;
}
.r35-ai-note p {
  margin: 2px 0;
}
.r35-affix-note,
.r35-ai-basis,
.r35-ai-honesty {
  color: var(--lyp-muted);
}
.r35-ai-cta a {
  color: var(--lyp-accent);
}
/* R39：皮肤图鉴表表内横向滚动，不造成页面级溢出 */
.r39-skin-table-scroll {
  overflow-x: auto;
  max-width: 100%;
}
.r39-ai-note {
  margin-top: 12px;
  padding: 10px 12px;
  border: 1px solid var(--lyp-border);
  border-radius: 6px;
  font-size: 12px;
  line-height: 1.6;
}
.r39-ai-note p {
  margin: 2px 0;
}
.r39-ai-basis,
.r39-ai-honesty {
  color: var(--lyp-muted);
}
.r39-ai-cta a {
  color: var(--lyp-accent);
}
/* R40：评分权重表表内横向滚动，不造成页面级溢出；两列窄表默认不滚动 */
.r40-rating-table-scroll {
  overflow-x: auto;
  max-width: 100%;
}
.r40-ai-note {
  margin-top: 12px;
  padding: 10px 12px;
  border: 1px solid var(--lyp-border);
  border-radius: 6px;
  font-size: 12px;
  line-height: 1.6;
}
.r40-ai-note p {
  margin: 2px 0;
}
.r40-ai-basis,
.r40-ai-honesty {
  color: var(--lyp-muted);
}
.r40-ai-cta a {
  color: var(--lyp-accent);
}
</style>
