import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const read = (name: string) => readFileSync(resolve(import.meta.dirname, name), 'utf8')

// Baseline after the 2026-09 auto-query pruning pass: every view now issues only
// queries the template actually consumes. Stub blocks that auto-fire on mount
// (placeholder ids, module roots, write handlers, unconsumed mirrors) were removed.
const baseline: Array<[string, number]> = [
  ['AIChatApp.vue', 0],
  ['AppInfoApp.vue', 0],
  ['BBSForum.vue', 3],
  ['CalendarApp.vue', 1],
  ['CategoryApp.vue', 0],
  ['CmsModuleApp.vue', 1],
  ['CommonApp.vue', 1],
  ['ConfigDesignerApp.vue', 0],
  ['Dashboard.vue', 2],
  ['DesignCenterApp.vue', 0],
  ['DocumentApp.vue', 0],
  ['FormApp.vue', 0],
  ['HotpicApp.vue', 0],
  ['IMChat.vue', 1],
  ['JPushApp.vue', 0],
  ['MeetingApp.vue', 1],
  ['MindApp.vue', 0],
  ['PortalDesigner.vue', 0],
  ['ProcessDesigner.vue', 0],
  ['ProcessWork.vue', 1],
  ['ProgramCenterApp.vue', 0],
  ['QueryManagerDeep.vue', 0],
  ['QueryViewApp.vue', 0],
  ['RoleManager.vue', 0],
  ['ServerApp.vue', 0],
  ['UnitApp.vue', 0],
]

describe('view auto-query invariants', () => {
  it('keeps the pruned use-query baseline per view', () => {
    for (const [view, expected] of baseline) {
      const source = read(view)
      const calls = source.match(/= useQuery\(/g)?.length ?? 0
      expect(calls, `${view}: expected ${expected} useQuery calls, found ${calls}`).toBe(expected)
    }
  })

  it('keeps the consumed first-screen queries', () => {
    expect(read('BBSForum.vue')).toContain("api.get('/jaxrs/bbs/assemble/control/section/list')")
    expect(read('CalendarApp.vue')).toContain('/jaxrs/calendar_assemble_control/event/list/filter')
    expect(read('Dashboard.vue')).toContain('/jaxrs/processplatform/assemble/surface/work/count/currentperson')
    expect(read('Dashboard.vue')).toContain('/jaxrs/message/unread/count/im')
    expect(read('IMChat.vue')).toContain('/jaxrs/message/assemble/communicate/im/conversation/list/my')
    expect(read('MeetingApp.vue')).toContain('/jaxrs/meeting/assemble/control/building/list')
    expect(read('JPushApp.vue')).toContain("api.get('/jaxrs/jpush_assemble_control/device/list/jpush')")
    expect(read('JPushApp.vue')).toContain("api.get('/jaxrs/jpush/template/list')")
  })

  it('does not auto-query write handlers or placeholder identifiers on mount', () => {
    for (const [view, forbidden] of [
      ['JPushApp.vue', ['/jaxrs/jpush/assemble/control/update/control/config']],
      [
        'BBSForum.vue',
        [
          '/jaxrs/bbs/assemble/control/topic/create',
          '/jaxrs/bbs/assemble/control/delete/subject',
          '/jaxrs/bbs/assemble/control/shutup/create',
          '/jaxrs/comment/c-1/commend',
        ],
      ],
      ['AIChatApp.vue', ['/jaxrs/ai_assemble_control/config/delete/mcp/flag', '/jaxrs/ai/chat']],
      ['HotpicApp.vue', ['/jaxrs/hotpic/save/hotpic', '/jaxrs/hotpic/delete/hotpic']],
      ['DocumentApp.vue', ['/jaxrs/document/d-1/update', '/jaxrs/document/publish/d-1']],
      ['ServerApp.vue', ['/jaxrs/cache/commonscript/flush', '/jaxrs/cache/config/flush']],
      ['RoleManager.vue', ['/jaxrs/permission/management/refresh/all']],
      ['AppInfoApp.vue', ['/jaxrs/appinfo/a-1/permission', '/jaxrs/appinfo/a-1/icon/size/64']],
      ['QueryViewApp.vue', ['/jaxrs/queryview/query/qf-1', '/jaxrs/queryview/importmodel/record/r-1']],
      ['PortalDesigner.vue', ['/jaxrs/portal/assemble/designer/get/design-1']],
      ['MindApp.vue', ['/jaxrs/mind/core/entity/folder/folder-001']],
      ['UnitApp.vue', []],
      ['ProgramCenterApp.vue', ['/jaxrs/program_center/market/m-1/install/log']],
      ['ProcessWork.vue', ['/jaxrs/processplatform/task/processing/task-001']],
      ['CommonApp.vue', ['/jaxrs/general/assemble/control/securityclearance/enable']],
      ['CalendarApp.vue', ['/jaxrs/calendar/core/entity/calendar/remove']],
    ] as Array<[string, string[]]>) {
      const source = read(view)
      for (const path of forbidden) {
        expect(source, `${view} must not auto-query ${path}`).not.toContain(`'${path}'`)
      }
    }
  })
})
