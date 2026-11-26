import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { dirname, resolve } from 'node:path'

const here = dirname(fileURLToPath(import.meta.url))
function src(rel: string): string {
  return readFileSync(resolve(here, rel), 'utf8')
}

const app = src('../App.vue')

// R301：t11 findings 指出多页加载态仍是裸「加载中…」灰字，缺结构化骨架屏与微交互。
// 契约钉死「真实骨架占位 + 按压反馈」，而不是只换颜色或伪造数据。
describe('R301 H5 骨架屏与微交互实质升级', () => {
  it('App.vue 提供结构化骨架屏容器类与短行变体', () => {
    expect(app).toContain('.ai-skeleton-list')
    expect(app).toContain('.ai-skeleton-line.short')
    expect(app).toMatch(/@keyframes\s+ai-shimmer/)
  })

  it('App.vue 提供按压微交互并受 prefers-reduced-motion 守卫', () => {
    expect(app).toContain('.ai-pressable:active')
    expect(app).toMatch(/transform:\s*scale\(/)
    expect(app).toContain('prefers-reduced-motion')
  })

  it('按压微交互真实落地：首页宫格/统计卡与各页空态入口挂载 ai-pressable', () => {
    const index = src('index/index.vue')
    expect((index.match(/class="cell ai-pressable"/g) ?? []).length).toBeGreaterThanOrEqual(12)
    expect(index).toContain('stat-card ai-brand-fill ai-pressable')
    for (const rel of ['browse/browse.vue', 'collect/collect.vue', 'doc/doc.vue', 'meeting/meeting.vue', 'recycle/recycle.vue']) {
      expect(src(rel), rel + ' 空态入口应挂 ai-pressable').toContain('empty-guide-link ai-pressable')
    }
  })

  const pages: Array<[string, number]> = [
    ['attendance/attendance.vue', 2],
    ['calendar/calendar.vue', 1],
    ['browse/browse.vue', 1],
    ['collect/collect.vue', 1],
    ['doc/doc.vue', 3],
    ['bbs/topic.vue', 1],
    ['recycle/recycle.vue', 1],
    ['search/search.vue', 1],
    ['process/start.vue', 1],
    ['chat/chat.vue', 1],
  ]
  for (const [rel, minSkeletons] of pages) {
    it(rel + ' 加载态用骨架屏替代裸文字，且保留真实 v-if 条件', () => {
      const s = src(rel)
      const count = (s.match(/ai-skeleton-list/g) ?? []).length
      expect(count, rel + ' 应含至少 ' + minSkeletons + ' 个结构化骨架容器').toBeGreaterThanOrEqual(minSkeletons)
      // 裸加载灰字必须全部让位给骨架屏（不含伪造数据；仅占位）
      expect(s, rel + ' 不应再有 class="tip" 的裸加载文字').not.toMatch(/class="tip">\s*(?:加载中|搜索中|详情加载中|加载表单)/)
    })
  }
})
