// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R300 t3：流程 / 门户 / 运维 / 组织页标题去重契约。
 * 页面不得再渲染与 AdminLayout 路由标题重复的本地 h1 或 /api/* 副标题；
 * 原操作按钮与关键 API 必须保持不变。含 AdminPageHeader 的页面通过共享
 * ADMIN_HIDE_PAGE_TITLE 开关隐藏页头标题/副标题，并在卸载时复位。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const read = (name: string): string => readFileSync(resolve(import.meta.dirname, name), 'utf8')

type PageCase = {
  file: string
  endpoints: string[]
  actions: string[]
  keep?: string[]
  selfHeldTitle?: boolean
}

const pages: PageCase[] = [
  {
    file: 'CommonApp.vue',
    endpoints: ['/api/general/assemble/control/list', '/api/general/assemble/control/create'],
    actions: ['@click="openCreate"'],
  },
  {
    file: 'AppInfoApp.vue',
    endpoints: ['/api/appinfo/list/all', '/api/output/list'],
    actions: ['@click="createApp"'],
  },
  {
    file: 'ProcessXformApp.vue',
    endpoints: [
      '/api/processplatform/assemble/designer/xform/list',
      '/api/processplatform/assemble/designer/xform/create',
    ],
    actions: ['@click="openCreate"'],
  },
  {
    file: 'ProcessDesigner.vue',
    endpoints: ['/api/processplatform/assemble/designer/', '/api/processplatform/assemble/designer/list/all'],
    actions: ['@click="saveProcess"', '@click="createVersion()"'],
  },
  {
    file: 'ProcessTaskCenterApp.vue',
    endpoints: [
      '/api/processplatform/service/processing/task/list',
      '/api/processplatform/assemble/surface/anonymous/read/',
    ],
    actions: ['@click="loadWorkList"', '@click="loadCounts"'],
  },
  {
    file: 'ProcessManagerApp.vue',
    endpoints: [
      '/api/processplatform/assemble/surface/process_manager/list',
      '/api/processplatform/service/processing/list/running',
    ],
    actions: ['@click="loadRunningProcesses"', '@click="loadData"'],
  },
  {
    file: 'ProcessFormDesignerApp.vue',
    endpoints: [
      '/api/processplatform/assemble/designer/form/list',
      '/api/processplatform/assemble/designer/form/create',
    ],
    actions: ['@click="openCreate"'],
  },
  {
    file: 'ProcessDictDesignerApp.vue',
    endpoints: [
      '/api/processplatform/assemble/designer/dict/list',
      '/api/processplatform/assemble/designer/dict/create',
    ],
    actions: ['@click="openCreate"'],
  },
  {
    file: 'ProcessApplicationApp.vue',
    endpoints: ['/api/program_center/application/', '/api/processplatform/assemble/designer/application/list'],
    actions: ['@click="openCreate"', '@click="readWithFlag"'],
  },
  {
    file: 'PortalDesigner.vue',
    endpoints: [
      '/api/portal/assemble/designer/widget/list/portal/',
      '/api/portal/assemble/designer/script/list/portal/',
    ],
    actions: ['@click="saveDesign"', '@click="pdDesignerOps(\'fileUpload\')"'],
  },
  {
    file: 'PortalDictDesignerApp.vue',
    endpoints: ['/api/portal/assemble/designer/dict/list', '/api/portal/assemble/designer/dict/create'],
    actions: ['@click="openCreate"'],
  },
  {
    file: 'PortalPageDesignerApp.vue',
    endpoints: ['/api/portal/assemble/surface/save/layout', '/api/portal/assemble/designer/page/list'],
    actions: ['@click="layoutOp(\'savePost\')"', '@click="openCreate"'],
  },
  {
    file: 'PortalWidgetDesignerApp.vue',
    endpoints: ['/api/portal/assemble/designer/widget/list', '/api/portal/assemble/designer/widget/create'],
    actions: ['@click="openCreate"'],
  },
  {
    file: 'JPushApp.vue',
    endpoints: ['/api/jpush_assemble_control/device/list/jpush', '/api/jpush/template/list'],
    actions: ['@click="loadJpushEntities"', '@click="jpushWrite(\'messageSend\')"'],
    keep: ['class="subtitle">{{ entitiesText }}</span>'],
  },
  {
    file: 'DeploymentApp.vue',
    endpoints: ['/api/server/deploy/list', '/api/server/deploy/create'],
    actions: ['@click="openCreate"'],
    // t38 a8cc1a56c：页面自持唯一 h1 + 注入 ADMIN_LAYOUT_HIDE_TITLE 隐藏 shell h1（合法第二模式）。
    selfHeldTitle: true,
  },
  {
    file: 'ThreeMemberApp.vue',
    endpoints: [
      '/api/organization/assemble/control/threemember/list',
      '/api/organization/assemble/control/threemember/create',
    ],
    actions: ['@click="openCreate"'],
  },
]

const adminHeaderPages = [
  'AppInfoApp.vue',
  'ProcessXformApp.vue',
  'ProcessDesigner.vue',
  'ProcessTaskCenterApp.vue',
  'ProcessManagerApp.vue',
  'ProcessFormDesignerApp.vue',
  'ProcessDictDesignerApp.vue',
  'ProcessApplicationApp.vue',
]

describe('R300 t3 流程/门户/运维/组织页标题去重', () => {
  it.each(pages)('$file 不渲染重复标题，且保留操作与 API', ({ file, endpoints, actions, keep, selfHeldTitle }) => {
    const src = read(file)
    if (selfHeldTitle) {
      // 页面自持唯一 h1 合法模式：允许恰好 1 个本地 h1，但必须注入并开启隐藏 shell 标题开关。
      expect((src.match(/<h1[\s>]/g) ?? []).length).toBe(1)
      expect(src, file + ' 自持 h1 须 inject ADMIN_LAYOUT_HIDE_TITLE').toMatch(
        /inject\(ADMIN_LAYOUT_HIDE_TITLE, undefined\)/,
      )
      expect(src, file + ' 自持 h1 须开启隐藏 shell 标题').toMatch(/\.value = true/)
    } else {
      expect(src, file + ' 不得保留本地 h1 大标题').not.toMatch(/<h1[\s>]/)
    }
    expect(src, file + ' 不得保留 /api 副标题').not.toMatch(/<p class="subtitle">\s*\/api/)
    for (const endpoint of endpoints) {
      expect(src, file + ' 必须保留端点 ' + endpoint).toContain(endpoint)
    }
    for (const action of actions) {
      expect(src, file + ' 必须保留操作 ' + action).toContain(action)
    }
    for (const fragment of keep ?? []) {
      expect(src, file + ' 必须保留动态信息 ' + fragment).toContain(fragment)
    }
  })

  it.each(adminHeaderPages)('%s 使用共享开关隐藏 AdminPageHeader 标题/副标题', (file) => {
    const src = read(file)
    expect(src, file + ' 必须使用 AdminPageHeader').toContain('<AdminPageHeader')
    expect(src, file + ' 必须导入 ADMIN_HIDE_PAGE_TITLE').toMatch(
      /import \{[^}]*ADMIN_HIDE_PAGE_TITLE[^}]*\} from '\.\.\/components\/admin\//s,
    )
    expect(src, file + ' 必须注入隐藏开关').toMatch(/inject\(ADMIN_HIDE_PAGE_TITLE, undefined\)/)
    expect(src, file + ' 必须开启隐藏开关').toMatch(/hidePageTitle!\.value = true/)
    expect(src, file + ' 卸载时必须复位隐藏开关').toMatch(
      /onUnmounted\(\(\) => \{[\s\S]*hidePageTitle!\.value = false[\s\S]*\}\)/,
    )
  })
})
