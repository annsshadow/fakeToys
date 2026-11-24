import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'
import { attributeLiveReactions, attributionText, buildLiveFeedAi, ATTRIBUTION_UNAVAILABLE, LIVE_FEED_BASIS, LIVE_FEED_HONESTY } from './liveReactionFeed'

const specPath = join(__dirname, '../../../../../docs/laiyipao/specs/round28-live-reaction-attribution/SPEC.md')
const srcRoot = join(__dirname, '../../..')
const readSrc = (p: string) => readFileSync(join(srcRoot, p), 'utf-8')
const spec = readFileSync(specPath, 'utf-8')
const battleSrc = readSrc('src/pages/battle/battle.vue')
const engineSrc = readSrc('src/game/engine.ts')
const tokensSrc = readSrc('src/styles/tokens.css')
const LIVE_FEED_CTA = '查看完整反应时间线 →'

type TL = { wave: number; tick: number; reaction: string; enemy_uid: number; damage: number }
const tl = (wave: number, reaction: string, enemy_uid: number, damage: number, tick = 0): TL => ({ wave, tick, reaction, enemy_uid, damage })
const ev = (reaction: string, damage: number | bigint) => ({ reaction, damage })

describe('R28 归因纯函数（真实模块 liveReactionFeed）', () => {
  it('多事件同 reaction 同 damage 按发射顺序从时间线尾部一一对应，不串位', () => {
    const timeline = [tl(1, 'steam_burst', 5, 100, 1), tl(2, 'overheat', 9, 50, 2), tl(3, 'steam_burst', 12, 100, 3)]
    const r = attributeLiveReactions([ev('steam_burst', 100), ev('overheat', 50), ev('steam_burst', 100)], timeline, false)
    expect(r.rows.map(x => [x.wave, x.enemy_uid])).toEqual([[1, 5], [2, 9], [3, 12]])
    expect(r.matchedCount).toBe(3)
    expect(r.unavailableCount).toBe(0)
    expect(r.rows.every(x => x.available)).toBe(true)
  })

  it('不匹配（reaction 或 damage 不同）→ available=false 且 wave/uid 为 null', () => {
    const r = attributeLiveReactions([ev('overheat', 999)], [tl(2, 'overheat', 9, 50)], false)
    expect(r.rows[0].available).toBe(false)
    expect(r.rows[0].wave).toBeNull()
    expect(r.rows[0].enemy_uid).toBeNull()
    expect(attributionText(r.rows[0])).toBe(ATTRIBUTION_UNAVAILABLE)
  })

  it('空 timeline（null/undefined/[]）→ 全部诚实不可用', () => {
    for (const t of [null, undefined, []]) {
      const r = attributeLiveReactions([ev('overheat', 50)], t as any, false)
      expect(r.rows[0].available).toBe(false)
      expect(r.matchedCount).toBe(0)
    }
  })

  it('truncated=true 时即使有记录也不可用（尾部已丢弃）', () => {
    const r = attributeLiveReactions([ev('overheat', 50)], [tl(2, 'overheat', 9, 50)], true)
    expect(r.rows[0].available).toBe(false)
    expect(r.unavailableCount).toBe(1)
    expect(attributionText(r.rows[0])).toBe(ATTRIBUTION_UNAVAILABLE)
  })

  it('非法 wave/enemy_uid（0/负数/小数）→ 不可用，不输出伪造数字', () => {
    const bad = [tl(0, 'overheat', 9, 50), tl(-1, 'overheat', 9, 50), tl(1.5, 'overheat', 9, 50), tl(2, 'overheat', 0, 50), tl(2, 'overheat', -3, 50), tl(2, 'overheat', 8.5, 50)]
    for (const b of bad) {
      const r = attributeLiveReactions([ev('overheat', 50)], [b], false)
      expect(r.rows[0].available).toBe(false)
      expect(r.rows[0].wave).toBeNull()
      expect(r.rows[0].enemy_uid).toBeNull()
    }
  })

  it('命中即消费游标：最新非法记录仍占用游标，较早合法记录只归较早事件（两同 reaction 同 damage 事件）', () => {
    // timeline 尾部为最新：最新记录 wave=0 非法；较早记录 wave=1/enemy_uid=5 合法。
    const timeline = [tl(1, 'steam_burst', 5, 100, 1), tl(0, 'steam_burst', 9, 100, 2)]
    const r = attributeLiveReactions([ev('steam_burst', 100), ev('steam_burst', 100)], timeline, false)
    // events 正序：rows[0]=较早事件配较早合法记录；rows[1]=最新事件配最新非法记录。
    expect(r.rows[0].available).toBe(true)
    expect(r.rows[0].wave).toBe(1)
    expect(r.rows[0].enemy_uid).toBe(5)
    expect(r.rows[1].available).toBe(false)
    expect(r.rows[1].wave).toBeNull()
    expect(r.rows[1].enemy_uid).toBeNull()
    expect(r.matchedCount).toBe(1)
    expect(r.unavailableCount).toBe(1)
  })

  it('attributionText 可用时逐字「第N波 · #uid」，不可用时逐字归因不可用', () => {
    expect(attributionText({ reaction: 'x', damage: 1, wave: 3, enemy_uid: 12, available: true })).toBe('第3波 · #12')
    expect(attributionText({ reaction: 'x', damage: 1, wave: null, enemy_uid: null, available: false })).toBe('归因不可用')
  })

  it('buildLiveFeedAi 结论来自真实匹配计数，依据/红线用逐字常量', () => {
    const r = attributeLiveReactions([ev('a', 1), ev('b', 2)], [tl(1, 'a', 1, 1), tl(2, 'b', 2, 2)], false)
    const ai = buildLiveFeedAi(r)
    expect(ai.conclusion).toContain('2/2')
    expect(ai.basis).toBe(LIVE_FEED_BASIS)
    expect(ai.honesty).toBe(LIVE_FEED_HONESTY)
    const ai0 = buildLiveFeedAi(attributeLiveReactions([], [], false))
    expect(ai0.conclusion).toContain('无')
  })
})

describe('R28 真实源码守卫（防止只测 SPEC 文本）', () => {
  it('battle.vue 增强现有 R9 feed：保留字段与三条上限并新增 R28 归因锚点', () => {
    for (const s of ['.reaction-feed', '.reaction-line', '.reaction-line-main', '.reaction-name', '.reaction-damage', '.reaction-cue', 'r28-attribution', 'data-r28-wave', 'data-r28-enemy', 'r28-feed-ai']) expect(battleSrc).toContain(s)
    expect(battleSrc).toContain('slice(0, 3)')
    expect(battleSrc).toContain('归因不可用')
    expect(battleSrc).toContain(LIVE_FEED_HONESTY)
    expect(battleSrc).toContain(LIVE_FEED_BASIS)
  })

  it('.r28-feed-ai 四要素 + CTA 逐字 + @click.stop=openHelp + CTA 子元素 r28-feed-cta', () => {
    const i = battleSrc.indexOf('r28-feed-ai')
    expect(i).toBeGreaterThanOrEqual(0)
    const aiBlock = battleSrc.slice(i, i + 3000)
    expect(aiBlock).toContain('AI')
    expect(aiBlock).toContain(LIVE_FEED_BASIS)
    expect(aiBlock).toContain(LIVE_FEED_HONESTY)
    expect(aiBlock).toContain(LIVE_FEED_CTA)
    expect(aiBlock).toContain('r28-feed-cta')
    expect(aiBlock).toMatch(/@click\.stop="openHelp"/)
    expect(spec).toContain(LIVE_FEED_CTA)
    expect(spec).toContain('@click.stop="openHelp"')
    expect(spec).toContain('r28-feed-cta')
  })

  it('battle.vue 真实调用三个纯函数（调用正则，仅 import 不算）', () => {
    expect(battleSrc).toContain("from './liveReactionFeed'")
    for (const fn of ['attributeLiveReactions', 'attributionText', 'buildLiveFeedAi']) {
      expect(battleSrc).toMatch(new RegExp('\\b' + fn + '\\s*\\('))
    }
  })

  it('R28 CSS 精确守卫：attr nowrap + font<=20rpx；feed-ai 可换行 + max-width:100% + pointer-events:none；CTA pointer-events:auto', () => {
    const rule = (sel: string) => {
      const m = battleSrc.match(new RegExp(sel.replace(/\./g, '\\.') + '\\s*\\{([^}]*)\\}'))
      return m ? m[1] : ''
    }
    const attr = rule('.r28-attribution')
    expect(attr).toMatch(/white-space\s*:\s*nowrap/)
    const fs = attr.match(/font-size\s*:\s*(\d+(?:\.\d+)?)rpx/)
    expect(fs).not.toBeNull()
    expect(Number(fs ? fs[1] : 999)).toBeLessThanOrEqual(20)
    const ai = rule('.r28-feed-ai')
    expect(ai).toMatch(/max-width\s*:\s*100%/)
    expect(ai).toMatch(/pointer-events\s*:\s*none/)
    expect(ai).not.toMatch(/white-space\s*:\s*nowrap/)
    const cta = rule('.r28-feed-cta')
    expect(cta).toMatch(/pointer-events\s*:\s*auto/)
  })

  it('engine.ts 只用现有只读 API，未被新模块反向写入权威字段', () => {
    for (const k of ['reactionTimeline()', 'reactionTimelineTruncated()', 'REACTION_TIMELINE_LIMIT = 200']) expect(engineSrc).toContain(k)
    expect(engineSrc).not.toContain('attributeLiveReactions')
  })

  it('R15/R24/R26/R27 共存结构不回退', () => {
    for (const k of ['r24-element-hud', 'r26-wave-brief', 'r27-wave-progress']) expect(battleSrc).toContain(k)
    expect(engineSrc).toContain('reactionTimeline')
  })

  it('令牌三锚点保留且无 --r28- 新令牌', () => {
    for (const k of ['lyp-tokens-v1', 'lyp-tokens-v3-r3', 'lyp-tokens-r6']) expect(tokensSrc).toContain(k)
    expect(tokensSrc).not.toContain('--r28-')
  })
})

describe('R28 SPEC 选择器与逐字文案契约', () => {
  it('导出常量与 SPEC 逐字文案严格一致（防止实施模块改文案而测试仍绿）', () => {
    expect(ATTRIBUTION_UNAVAILABLE).toBe('归因不可用')
    expect(LIVE_FEED_BASIS).toBe('依据：R15 reactionTimeline 逐次事件（wave/enemy_uid/reaction/damage）按事件发射顺序配对；截断或不匹配如实标注归因不可用')
    expect(LIVE_FEED_HONESTY).toBe('辅助归因非权威判据，权威以 replay_hash 为准')
  })

  it('SPEC 钉死选择器与归因格式', () => {
    for (const s of ['.r28-attribution', '[data-r28-wave]', '[data-r28-enemy]', '.r28-feed-ai', '.r28-feed-cta', '.reaction-feed', '第N波 · #']) expect(spec).toContain(s)
  })

  it('SPEC 钉死不可用文案、依据行与逐字红线', () => {
    expect(spec).toContain('归因不可用')
    expect(spec).toContain(LIVE_FEED_BASIS)
    expect(spec).toContain(LIVE_FEED_HONESTY)
  })

  it('SPEC 钉死算法口径与 RED 门禁', () => {
    for (const k of ['尾部', '截断', 'waveIndex', '非法 wave', 'RED', 'ERR_MODULE_NOT_FOUND', '禁止内嵌参考实现']) expect(spec).toContain(k)
  })
})

describe('R28 AI 四判据与零真值', () => {
  it('SPEC 含 AI 四判据关键词', () => {
    for (const k of ['可见汉字', '真实数据', '依据行', 'CTA']) expect(spec).toContain(k)
  })

  it('SPEC 与源码不含域名/IP/JWT/口令/个人绝对路径', () => {
    for (const text of [spec, battleSrc, engineSrc, tokensSrc]) {
      expect(text).not.toMatch(/eyJ[A-Za-z0-9_-]{8,}\.[A-Za-z0-9_-]{8,}\.[A-Za-z0-9_-]{8,}/)
      expect(text).not.toMatch(/\b(?:\d{1,3}\.){3}\d{1,3}\b/)
      expect(text).not.toMatch(/postgres:\/\/[^\s]+:[^\s]+@/)
      expect(text).not.toMatch(/C:\\Users\\[^\s]+/)
    }
  })
})