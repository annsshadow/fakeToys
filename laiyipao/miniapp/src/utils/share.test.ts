/**
 * 分享文案纯函数（utils/share.ts）的判据（2026-10-23）。
 *
 * # 为什么必须单独测这一层
 *
 * 页面级测试只能证明「点了分享、返回了一个对象」。而分享的失效方式是
 * **内容错但结构对**：卡片能弹出来，只是标题写着「第 3 关 0 星 · 0 分」，
 * 或者路径指向一个到不了的位置。这种错只有分享出去的人看得见。
 *
 * 纯函数层可以断言具体文案与路径，成本几乎为零。
 *
 * 每一条都对应一个真实会发生的场景，不是凑数：
 *   - `rank=0` → 不能写「第 0 名」（未进榜/无成绩是常态）
 *   - `settled=false` → 不能出现分数（结算前没有权威值）
 *   - path 必须带 type（否则点进去落在默认榜）
 */

import { describe, it, expect } from 'vitest'
import { indexShare, rankShare, battleShare, SHARE_TITLE } from './share'

describe('utils/share', () => {
  it('首页分享：固定标题 + 首页路径', () => {
    const p = indexShare()
    expect(p.title).toBe(SHARE_TITLE)
    expect(p.path).toBe('/pages/index/index')
  })

  it('所有分享路径都以 /pages/ 开头（否则微信无法跳转）', () => {
    const all = [
      indexShare(),
      rankShare({ label: '战力榜', rank: 3, type: 'power' }),
      rankShare({ label: '战力榜', rank: 0, type: 'power' }),
      battleShare({ settled: true, levelId: 3, stars: 3, score: 555 }),
      battleShare({ settled: false, levelId: 3, stars: 0, score: 0 }),
    ]
    for (const p of all) {
      expect(p.path.startsWith('/pages/')).toBe(true)
      expect(p.title.length).toBeGreaterThan(0)
      // 微信分享标题上限 32 字，超了会被截断成「…」
      expect(p.title.length).toBeLessThanOrEqual(32)
    }
  })

  it('排行分享：有名次 → 标题带榜名与名次，路径带 type', () => {
    const p = rankShare({ label: '关卡榜', rank: 12, type: 'stage' })
    expect(p.title).toBe('我在关卡榜排第 12')
    expect(p.path).toBe('/pages/rank/rank?type=stage')
  })

  it('排行分享：无名次（rank=0）→ 不出现「第 0」，退回邀请式文案', () => {
    const p = rankShare({ label: '效率榜', rank: 0, type: 'efficiency' })
    expect(p.title).not.toContain('第 0')
    expect(p.title).toContain('效率榜')
    // 路径仍要带 type —— 接收方点进来应该看到同一个榜
    expect(p.path).toBe('/pages/rank/rank?type=efficiency')
  })

  it('排行分享：type 需要 URL 编码（防止未来出现特殊字符）', () => {
    const p = rankShare({ label: '榜', rank: 1, type: 'a&b' })
    expect(p.path).toBe('/pages/rank/rank?type=a%26b')
  })

  it('战斗分享：已结算 → 带关卡/星级/分数', () => {
    const p = battleShare({ settled: true, levelId: 7, stars: 2, score: 12345 })
    expect(p.title).toContain('第 7 关')
    expect(p.title).toContain('2 星')
    expect(p.title).toContain('12345 分')
    expect(p.path).toBe('/pages/index/index')
  })

  it('战斗分享：未结算 → 绝不出现分数/星级（此时没有权威值）', () => {
    const p = battleShare({ settled: false, levelId: 7, stars: 0, score: 0 })
    expect(p.title).toBe(SHARE_TITLE)
    expect(p.title).not.toContain('分')
    expect(p.title).not.toContain('星')
  })

  it('战斗分享：未结算时即便传入非零分数也不拼进标题（防调用方误传）', () => {
    // 页面在「结算在途」窗口里 settled 可能是 false 而 score 已被本地引擎填过。
    // 判据在函数里，不在调用方 —— 传什么都不能漏出去。
    const p = battleShare({ settled: false, levelId: 7, stars: 3, score: 999 })
    expect(p.title).toBe(SHARE_TITLE)
  })
})
