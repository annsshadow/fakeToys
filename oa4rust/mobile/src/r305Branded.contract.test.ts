import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'

const root = resolve(process.cwd())
const read = (rel: string) => readFileSync(resolve(root, 'src/pages', rel), 'utf8')

const targets: Array<[string, RegExp]> = [
  ['message/message.vue', /class="offline-banner[^"]*ai-brand-fill"/],
  ['chat/chat.vue', /class="head[^"]*ai-brand-fill"/],
  ['process/process.vue', /class="tab[^"]*ai-brand-fill"/],
  ['process/start.vue', /class="submit[^"]*ai-brand-fill"/],
  ['attendance/attendance.vue', /class="btn[^"]*ai-brand-fill"/],
  ['doc/doc.vue', /class="empty-guide[^"]*ai-brand-fill"/],
  ['contacts/contacts.vue', /class="go[^"]*ai-brand-fill"/],
  ['meeting/meeting.vue', /class="tab[^"]*ai-brand-fill"/],
  ['calendar/calendar.vue', /class="tab[^"]*ai-brand-fill"/],
  ['bbs/bbs.vue', /class="new-btn[^"]*ai-brand-fill"/],
  ['bbs/topic.vue', /class="composer-send[^"]*ai-brand-fill"/],
  ['bbs/new.vue', /class="submit[^"]*ai-brand-fill"/],
  ['collect/collect.vue', /class="empty-guide[^"]*ai-brand-fill"/],
  ['browse/browse.vue', /class="tab[^"]*ai-brand-fill"/],
  ['search/search.vue', /class="btn[^"]*ai-brand-fill"[^>]*@tap="doSearch"/],
  ['recycle/recycle.vue', /class="empty-guide[^"]*ai-brand-fill"/],
  ['mine/mine.vue', /class="card[^"]*ai-brand-fill"/],
]

describe('R305 H5 品牌渐变实心深度消费', () => {
  it.each(targets)('%s 主锚点挂载 ai-brand-fill', (rel, pattern) => {
    expect(read(rel)).toMatch(pattern)
  })

  it('on 态页签容器持有品牌渐变锚点（scoped .tab.on 仍为选中底色）', () => {
    for (const rel of ['meeting/meeting.vue', 'calendar/calendar.vue', 'browse/browse.vue']) {
      expect(read(rel)).toContain('<view class="tabs ai-brand-fill">')
    }
  })

  it('不回退 t25 日历 data 信封归一化', () => {
    const source = read('calendar/calendar.vue')
    expect(source).toContain("import { normalizeCalendarRows } from './calendarEnvelope'")
    expect(source).toContain('calendars.value = normalizeCalendarRows(cals.data)')
  })

  it('不回退 t29 会话标题可读遮蔽', () => {
    const source = read('message/message.vue')
    expect(source).toContain("for (const k of ['id', 'conversationId', 'uuid', 'flag'])")
    // R542: a11y 属性追加后逐字字面量不再成立;保留语义意图(空态引导入口与文案仍在)
    expect(source).toContain('class="empty-guide-link"')
    expect(source).toContain('@tap="load"')
    expect(source).toContain('刷新会话</view>')
  })

  it('不回退 t32 空态引导文案与入口', () => {
    const cases: Array<[string, string]> = [
      ['collect/collect.vue', '暂无收藏'],
      ['browse/browse.vue', '暂无数据'],
      ['recycle/recycle.vue', '回收站为空'],
      ['doc/doc.vue', 'empty-guide-link'],
      ['meeting/meeting.vue', 'empty-guide-link'],
    ]
    for (const [rel, literal] of cases) expect(read(rel)).toContain(literal)
  })

  it('骨架屏两套 class 均已被 t50 脚本口径 [class*=skeleton] 覆盖', () => {
    const app = readFileSync(resolve(root, 'src/App.vue'), 'utf8')
    expect(app).toContain('.o2-skeleton')
    expect(app).toContain('.ai-skeleton-list')
    const pages = [
      'message/message.vue', 'bbs/bbs.vue', 'contacts/contacts.vue',
      'meeting/meeting.vue', 'process/process.vue', 'attendance/attendance.vue',
      'calendar/calendar.vue', 'collect/collect.vue', 'browse/browse.vue',
      'doc/doc.vue', 'chat/chat.vue', 'search/search.vue', 'recycle/recycle.vue',
      'process/start.vue', 'bbs/topic.vue',
    ]
    for (const rel of pages) {
      const source = read(rel)
      expect(/class="[^"]*(o2-skeleton|ai-skeleton-list|ai-skeleton-line)[^"]*"/.test(source), rel).toBe(true)
    }
  })
})
