// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readdirSync, readFileSync } from 'node:fs'
import { join, resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const SRC = resolve(import.meta.dirname, '..')

function walk(dir: string): string[] {
  const out: string[] = []
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const full = join(dir, entry.name)
    if (entry.isDirectory()) out.push(...walk(full))
    else if (/\.(ts|tsx|vue)$/.test(entry.name)) out.push(full)
  }
  return out
}

const sources = walk(SRC).filter(
  (file) => !/\.test\.ts$/.test(file) && !/utils[/\\]date\.ts$/.test(file) && !/\.spec\.ts$/.test(file),
)

/** R154：27 个服务端时间展示/格式化调用点，必须统一走桌面共享解析器。 */
const SERVER_TIME_VIEWS: Array<[string, 'formatServerDateTime' | 'parseServerDate']> = [
  ['AIChatApp.vue', 'parseServerDate'],
  ['AnnApp.vue', 'formatServerDateTime'],
  ['AttendanceApp.vue', 'parseServerDate'],
  ['BamApp.vue', 'formatServerDateTime'],
  ['BBSForum.vue', 'parseServerDate'],
  ['CmsColumnApp.vue', 'formatServerDateTime'],
  ['CmsColumnManagerApp.vue', 'formatServerDateTime'],
  ['CmsIndexApp.vue', 'formatServerDateTime'],
  ['CmsModuleApp.vue', 'formatServerDateTime'],
  ['CollectApp.vue', 'formatServerDateTime'],
  ['CommonApp.vue', 'formatServerDateTime'],
  ['ConfigDesignerApp.vue', 'formatServerDateTime'],
  ['Dashboard.vue', 'formatServerDateTime'],
  ['DeploymentApp.vue', 'formatServerDateTime'],
  ['FaceSetApp.vue', 'formatServerDateTime'],
  ['FileManager.vue', 'formatServerDateTime'],
  ['FormApp.vue', 'formatServerDateTime'],
  ['FtSearchApp.vue', 'formatServerDateTime'],
  ['MeetingApp.vue', 'formatServerDateTime'],
  ['NoteApp.vue', 'formatServerDateTime'],
  ['ProcessApplicationApp.vue', 'formatServerDateTime'],
  ['ProcessTaskCenterApp.vue', 'formatServerDateTime'],
  ['ProcessWork.vue', 'parseServerDate'],
  ['QueryManager.vue', 'formatServerDateTime'],
  ['QueryManagerDeep.vue', 'formatServerDateTime'],
  ['QueryStatementDesigner.vue', 'formatServerDateTime'],
  ['RecycleApp.vue', 'parseServerDate'],
]

describe('桌面端服务端时间解析契约（R154）', () => {
  it('全仓不再用 new Date 隐式解析服务端字符串', () => {
    const offenders: string[] = []
    for (const file of sources) {
      const source = readFileSync(file, 'utf8')
      if (/new Date\(\s*String\(/.test(source)) offenders.push(`${file}: new Date(String(...))`)
      if (/new Date\(\s*['"`]/.test(source)) offenders.push(`${file}: new Date(字符串字面量)`)
      if (/Date\.parse\(/.test(source)) offenders.push(`${file}: Date.parse(...)`)
    }
    expect(offenders).toEqual([])
  })

  it('27 个服务端时间调用点统一走 parseServerDate / formatServerDateTime', () => {
    const offenders: string[] = []
    for (const [name, symbol] of SERVER_TIME_VIEWS) {
      const source = readFileSync(resolve(SRC, 'views', name), 'utf8')
      if (!source.includes(`import { ${symbol}`) && !source.includes(`, ${symbol} }`)) {
        offenders.push(`${name}: 未导入 ${symbol}`)
      }
      if (source.split(`${symbol}(`).length - 1 < 1) offenders.push(`${name}: 未调用 ${symbol}`)
      // 调用点回退到隐式解析：字符串变量喂给 new Date。
      if (/new Date\(\s*(?:t|ts|d|value|String\()/.test(source)) {
        offenders.push(`${name}: 回退 new Date(字符串变量)`)
      }
    }
    expect(offenders).toEqual([])
  })
})
