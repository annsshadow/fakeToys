// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { existsSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const viewsDir = resolve(import.meta.dirname, 'views')

/**
 * D 类空壳页面清理（r296 审计 §2）：这 10 个视图是空壳/无业务价值的入口，
 * 已连同其专属测试一并删除。注意：路由声明（main.ts）此刻可能仍引用这些组件名，
 * 由队长在 admin-guard 改动合并后手动清理，本契约不校验 main.ts。
 */
const DELETED_VIEWS = [
  'MindApp.vue',
  'UnitApp.vue',
  'AIChatApp.vue',
  'SelectorApp.vue',
  'HomepageApp.vue',
  'FaceSetApp.vue',
  'AnnApp.vue',
  'FtSearchApp.vue',
  'EmptyApp.vue',
  'SSO.vue',
]

describe('D 类空壳视图已删除（r296 审计 §2）', () => {
  it.each(DELETED_VIEWS)('%s 不应再存在', (name) => {
    expect(existsSync(resolve(viewsDir, name))).toBe(false)
  })
})
