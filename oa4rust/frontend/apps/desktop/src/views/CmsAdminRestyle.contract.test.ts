// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R298 t2：CMS/内容族管理页业务化重构契约。
 * 15 个页面必须复用 t1 共享管理原语（页头 / 工具条 / 标准表格 / 分区），
 * 同时原有 API 端点能力一个不丢——只重组 UI，不删功能。
 * node 环境无法加载 .vue SFC，沿用仓库既有的源码级契约风格。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const read = (name: string): string => readFileSync(resolve(import.meta.dirname, name), 'utf8')

type CrudCase = { file: string; endpoints: string[] }

const crudPages: CrudCase[] = [
  {
    file: 'AppInfoApp.vue',
    endpoints: ['/api/appinfo/list/all', '/api/appinfo', '/api/file', '/api/appinfo/${id}/permission'],
  },
  {
    file: 'CategoryApp.vue',
    endpoints: ['/api/categoryinfo/list/all', '/api/categoryinfo', '/api/categoryinfo/${item.id}'],
  },
  {
    file: 'DocumentApp.vue',
    endpoints: ['/api/document/list/document', '/api/document', '/api/document/${item.id}/commend', '/api/uuid/random'],
  },
  {
    file: 'TemplateApp.vue',
    endpoints: [
      '/api/templateform/list',
      '/api/templateform/list/category',
      '/api/templateform/save/${editingId.value}',
      '/api/templateform/delete/${id}',
    ],
  },
  {
    file: 'CollectApp.vue',
    endpoints: [
      '/api/program_center/collect/list',
      '/api/program_center/collect/create',
      '/api/program_center/collect/delete/${id}',
    ],
  },
  {
    file: 'HotpicApp.vue',
    endpoints: [
      '/api/hotpic/list/hotpics',
      '/api/hotpic/create/hotpic',
      '/api/hotpic/core/entity/delete/${encodeURIComponent(id)}',
    ],
  },
  {
    file: 'CmsColumnApp.vue',
    endpoints: [
      '/api/cms/core/entity/column/list',
      '/api/cms/core/entity/column/create',
      '/api/cms/core/entity/column/delete/${id}',
    ],
  },
  {
    file: 'CmsColumnManagerApp.vue',
    endpoints: ['/api/cms/core/entity/column_manager/list', '/api/cms/core/entity/column_manager/create'],
  },
  { file: 'CmsModuleApp.vue', endpoints: ['/api/cms/core/entity/module/list', '/api/cms/core/entity/module/create'] },
  {
    file: 'CmsXformApp.vue',
    endpoints: ['/api/cms/assemble/control/xform/list', '/api/cms/assemble/control/xform/create'],
  },
  {
    file: 'CmsDictDesignerApp.vue',
    endpoints: ['/api/cms/assemble/control/dict/list', '/api/cms/assemble/control/dict/create'],
  },
  {
    file: 'CmsFormDesignerApp.vue',
    endpoints: ['/api/cms/assemble/control/form/list', '/api/cms/assemble/control/form/create'],
  },
  {
    file: 'CmsViewDesignerApp.vue',
    endpoints: ['/api/cms/assemble/control/view/list', '/api/cms/assemble/control/view/create'],
  },
]

describe('R298 t2 CMS/内容族管理页业务化重构', () => {
  it.each(crudPages)('$file 复用管理原语且保留全部端点能力', ({ file, endpoints }) => {
    const src = read(file)
    expect(src, `${file} 必须从共享 barrel 导入管理原语`).toMatch(/from '\.\.\/components\/admin\/'/)
    expect(src, `${file} 必须使用 AdminPageHeader 标准页头`).toContain('<AdminPageHeader')
    expect(src, `${file} 必须使用 AdminTable 标准列表`).toContain('<AdminTable')
    expect(src, `${file} 必须保留加载/错误态绑定`).toMatch(/:error=/)
    for (const endpoint of endpoints) {
      expect(src, `${file} 必须保留端点 ${endpoint}`).toContain(endpoint)
    }
  })

  it('工作台页套管理页头与分区，且保留业务组件与端点', () => {
    const script = read('CmsScriptDesignerApp.vue')
    expect(script).toContain('<AdminPageHeader')
    expect(script).toContain('<AdminFormSection')
    expect(script, '脚本工作台组件不能被删').toContain('<ScriptWorkbench')
    expect(script).toContain('/api/script/list/manager')

    const index = read('CmsIndexApp.vue')
    expect(index).toContain('<AdminPageHeader')
    expect(index).toContain('<AdminFormSection')
    expect(index, 'CMS 索引原有设计按钮不能被删').toContain('@click="cmsRoot(\'formUpdate\')"')
    expect(index).toContain('/api/cms/core/entity/index/list')
  })
})
