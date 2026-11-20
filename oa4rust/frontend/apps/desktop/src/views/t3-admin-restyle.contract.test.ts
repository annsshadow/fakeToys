// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R298 t3：流程/查询族管理页业务化重构契约。
 * 列表页必须复用 t1 原语（页头/工具条/标准表格/表单分区），
 * 设计器页必须套标准管理页头；同时所有端点能力一个不丢——只重组 UI，不删功能。
 * node 环境无法加载 .vue SFC，沿用仓库既有的源码级契约风格。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const read = (name: string): string => readFileSync(resolve(import.meta.dirname, name), 'utf8')

type CrudCase = { file: string; endpoints: string[] }

const crudPages: CrudCase[] = [
  {
    file: 'ProcessApplicationApp.vue',
    endpoints: [
      '/api/program_center/application/list',
      '/api/program_center/application/create',
      '/api/program_center/application/delete/',
      '/api/processplatform/assemble/designer/application/permission/',
    ],
  },
  {
    file: 'ProcessDictDesignerApp.vue',
    endpoints: [
      '/api/processplatform/assemble/designer/dict/list',
      '/api/processplatform/assemble/designer/dict/save',
      '/api/processplatform/assemble/designer/dict/delete',
    ],
  },
  {
    file: 'ProcessFormDesignerApp.vue',
    endpoints: [
      '/api/processplatform/assemble/designer/form/list',
      '/api/processplatform/assemble/designer/form/save',
      '/api/processplatform/assemble/designer/form/delete',
    ],
  },
  {
    file: 'ProcessXformApp.vue',
    endpoints: [
      '/api/processplatform/assemble/designer/xform/list',
      '/api/processplatform/assemble/designer/xform/save',
      '/api/processplatform/assemble/designer/xform/delete',
    ],
  },
  {
    file: 'QueryImporterDesignerApp.vue',
    endpoints: [
      '/api/query/assemble/designer/importer/list',
      '/api/query/assemble/designer/importer/save',
      '/api/query/assemble/designer/importer/delete',
    ],
  },
  {
    file: 'QueryQueryApp.vue',
    endpoints: [
      '/api/query/assemble/designer/list',
      '/api/query/assemble/designer/save',
      '/api/query/assemble/designer/delete',
      '/api/query/assemble/surface/list',
    ],
  },
  {
    file: 'QueryStatDesignerApp.vue',
    endpoints: [
      '/api/query/assemble/designer/stat/list',
      '/api/query/assemble/designer/stat/save',
      '/api/query/assemble/designer/stat/delete',
    ],
  },
]

type ListCase = { file: string; hooks: string[]; endpoints: string[] }

const listPages: ListCase[] = [
  {
    file: 'ProcessManagerApp.vue',
    hooks: [
      'loadRunningProcesses',
      'loadManagedApps',
      'loadAppProcesses',
      'loadOrphans',
      'loadProcessDetails',
      'pdCreate(',
      'pdEdit(',
      'pdDelete(',
      'pdPerm(',
      'psSerial(',
      'psHandover(',
      'psSign(',
      'psDraft(',
      'psCompleted(',
      'pdMisc(',
      'pdEngine(',
      'dataWrite(',
      'pdDesigner2(',
    ],
    endpoints: ['/api/processplatform/assemble/surface/process_manager/list'],
  },
  {
    file: 'ProcessTaskCenterApp.vue',
    hooks: [
      'loadTaskCursorFilters',
      'loadTaskFullCursors',
      'loadTaskCompletedFullCursors',
      'loadWorkDetail',
      'loadProcessingEngineReads',
    ],
    endpoints: ['/api/processplatform/service/processing/task/list'],
  },
  {
    file: 'QueryExplorerApp.vue',
    hooks: ['liveQueryItems<Item>', 'useListError(', 'loadData'],
    endpoints: ['/api/query/assemble/surface/explorer/list'],
  },
  {
    file: 'QueryViewApp.vue',
    hooks: [
      'qvWrite(',
      'qvMore(',
      'qvRows(',
      'qvDesignerRows(',
      'loadQueryList',
      'loadQueryByFlag',
      'loadQvDetails',
      'loadQueryTwin',
      'loadStatementStat',
      'loadTables',
      'loadQueryViewDeep',
    ],
    endpoints: ['/api/queryview/list'],
  },
]

type DesignerCase = { file: string; component: string; markers: string[] }

const designerPages: DesignerCase[] = [
  {
    file: 'ProcessDesigner.vue',
    component: '',
    markers: [
      '/api/processplatform/assemble/designer/*',
      'class="exec-bar"',
      'class="tr-progress"',
      'async function copySvg()',
    ],
  },
  {
    file: 'FormDesigner.vue',
    component: '',
    markers: ['fd-header glass-card', 'placeholder="搜索..."', '.list-input'],
  },
  {
    file: 'ProcessScriptDesignerApp.vue',
    component: '<ScriptWorkbench',
    markers: ['title="流程脚本"', 'with-category', '/api/processplatform/assemble/designer/script/list/paging/1/50/50'],
  },
  {
    file: 'QueryManagerDeep.vue',
    component: '',
    markers: [
      'qm-header glass-card',
      'qm-actions qm-write-actions',
      'placeholder="搜索查询..."',
      'class="qs-input"',
      'toggleSort(h)',
    ],
  },
  {
    file: 'QueryStatementDesigner.vue',
    component: '<SqlEditor',
    markers: [
      'smd-header glass-card',
      'class="smd-actions"',
      'placeholder="筛选结果..."',
      'class="result-filter"',
      'sortResult(h)',
    ],
  },
  {
    file: 'QueryTableDesignerApp.vue',
    component: '',
    markers: [
      'tablePayload(form.value.name, form.value.queryFlag, form.value.columns)',
      'designerPaths.tableExecute(activeFlag.value)',
    ],
  },
  {
    file: 'QueryViewDesignerApp.vue',
    component: '',
    markers: [
      'form.definition.filters',
      'designerPaths.viewSimulate(activeId.value)',
      'designerPaths.viewBundle(activeId.value)',
    ],
  },
]

describe('R298 t3 流程/查询族管理页业务化重构', () => {
  it.each(crudPages)('$file 复用管理原语且保留端点能力', ({ file, endpoints }) => {
    const src = read(file)
    expect(src, `${file} 必须从共享 barrel 导入管理原语`).toMatch(/from '\.\.\/components\/admin\//)
    expect(src, `${file} 必须使用 AdminPageHeader 标准页头`).toContain('<AdminPageHeader')
    expect(src, `${file} 必须使用 AdminToolbar 标准工具条`).toContain('<AdminToolbar')
    expect(src, `${file} 必须使用 AdminTable 标准列表`).toContain('<AdminTable')
    expect(src, `${file} 必须使用 AdminFormSection 表单分区`).toContain('<AdminFormSection')
    expect(src, `${file} 必须保留加载/错误态绑定`).toMatch(/:error=/)
    expect(src, `${file} 必须保留 ListLoadError`).toContain('ListLoadError')
    expect(src, `${file} 必须保留骨架屏`).toMatch(/skeleton|skel|loading-state/)
    for (const endpoint of endpoints) {
      expect(src, `${file} 必须保留端点 ${endpoint}`).toContain(endpoint)
    }
  })

  it.each(listPages)('$file 套标准页头/工具条且原端点按钮不丢', ({ file, hooks, endpoints }) => {
    const src = read(file)
    expect(src, `${file} 必须从共享 barrel 导入管理原语`).toMatch(/from '\.\.\/components\/admin\//)
    expect(src, `${file} 必须使用 AdminPageHeader 标准页头`).toContain('<AdminPageHeader')
    expect(src, `${file} 必须使用 AdminToolbar 标准工具条`).toContain('<AdminToolbar')
    expect(src, `${file} 必须保留 ListLoadError`).toContain('ListLoadError')
    expect(src, `${file} 必须保留骨架屏`).toMatch(/skeleton|skel|loading/)
    for (const hook of hooks) {
      expect(src, `${file} 必须保留处理器 ${hook}`).toContain(hook)
    }
    for (const endpoint of endpoints) {
      expect(src, `${file} 必须保留端点 ${endpoint}`).toContain(endpoint)
    }
  })

  it.each(designerPages)('$file 套标准管理页头且业务组件/关键标记不丢', ({ file, component, markers }) => {
    const src = read(file)
    expect(src, `${file} 必须使用 AdminPageHeader 标准页头`).toContain('<AdminPageHeader')
    if (component) {
      expect(src, `${file} 业务组件不能被删`).toContain(component)
    }
    for (const marker of markers) {
      expect(src, `${file} 必须保留 ${marker}`).toContain(marker)
    }
  })
})
