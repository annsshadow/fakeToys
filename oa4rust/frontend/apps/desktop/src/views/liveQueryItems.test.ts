// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readdirSync, readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { QueryClient } from '@tanstack/vue-query'
import { describe, expect, it } from 'vitest'
import { ref } from 'vue'
import { describeListError, useListError } from './listQuery'
import { liveQueryItems } from './liveQueryItems'

/** 扫描时确认的 32 个同构生成页：setup 只拷贝一次 data，刷新后列表不更新。 */
const GENERATED_PAGES = [
  'AnnApp.vue',
  'CmsColumnApp.vue',
  'CmsColumnManagerApp.vue',
  'CmsDictDesignerApp.vue',
  'CmsFormDesignerApp.vue',
  'CmsIndexApp.vue',
  'CmsModuleApp.vue',
  'CmsViewDesignerApp.vue',
  'CmsXformApp.vue',
  'CollectApp.vue',
  'CommonApp.vue',
  'DeploymentApp.vue',
  'FaceSetApp.vue',
  'FtSearchApp.vue',
  'LogViewerApp.vue',
  'NoteApp.vue',
  'PortalDictDesignerApp.vue',
  'PortalPageDesignerApp.vue',
  'PortalWidgetDesignerApp.vue',
  'ProcessApplicationApp.vue',
  'ProcessDictDesignerApp.vue',
  'ProcessFormDesignerApp.vue',
  'ProcessManagerApp.vue',
  'ProcessTaskCenterApp.vue',
  'ProcessXformApp.vue',
  'QueryExplorerApp.vue',
  'QueryImporterDesignerApp.vue',
  'QueryQueryApp.vue',
  'QueryStatDesignerApp.vue',
  'ServiceInvokeDesignerApp.vue',
  'TemplateApp.vue',
  'ThreeMemberApp.vue',
]

describe('生成页列表必须随 query 刷新更新（R107）', () => {
  it('数据源替换后列表立即跟随（旧实现只在 setup 拷贝一次）', () => {
    const data = ref<{ id: string }[] | undefined>([{ id: 'a' }])
    const items = liveQueryItems(() => data.value)

    expect(items.value.map((row) => row.id)).toEqual(['a'])
    // TanStack Query refetch 会替换 data.value 指向的新数组
    data.value = [{ id: 'a' }, { id: 'b' }]
    expect(items.value.map((row) => row.id)).toEqual(['a', 'b'])
  })

  it('undefined 与响应形状异常归一为空数组', () => {
    const data = ref<{ id: string }[] | undefined>(undefined)
    const items = liveQueryItems(() => data.value)
    expect(items.value).toEqual([])

    const wrong = ref<unknown>({ total: 0 }) as unknown as { value: { id: string }[] | undefined }
    expect(liveQueryItems(() => wrong.value).value).toEqual([])
  })

  it('32 个同构生成页全部改为 computed，且全目录没有残留的 setup 拷贝', () => {
    const dir = import.meta.dirname
    const vueFiles = readdirSync(dir).filter((name) => name.endsWith('.vue'))
    // 规模下限：目录被截断/扫描范围缩小时直接红，避免「0 个文件也通过」。
    expect(vueFiles.length).toBeGreaterThanOrEqual(80)
    expect(GENERATED_PAGES).toHaveLength(32)

    for (const file of GENERATED_PAGES) {
      const source = readFileSync(resolve(dir, file), 'utf8')
      expect(source, `${file} 未接 liveQueryItems`).toContain('liveQueryItems<Item>')
      expect(source, `${file} 仍是 setup 一次性拷贝`).not.toContain('items.value = Array.isArray(data.value)')
    }

    const stale = vueFiles.filter((file) =>
      readFileSync(resolve(dir, file), 'utf8').includes('items.value = Array.isArray(data.value)'),
    )
    expect(stale, `仍有页面把 data 一次性拷进 items: ${stale.join(', ')}`).toEqual([])
  })
})

describe('桌面列表页失败必须可见（R115）', () => {
  it('describeListError 保留原始原因，缺失时回落未知错误', () => {
    expect(describeListError('模板加载失败', new Error('boom'))).toBe('模板加载失败：boom')
    expect(describeListError('模板加载失败', null)).toBe('模板加载失败：未知错误')
    expect(describeListError('模板加载失败', { message: '   ' })).toBe('模板加载失败：未知错误')
  })

  it('useListError 只在 isError 为真时给出失败文案，状态复位后清空', () => {
    const isError = ref(true)
    const error = ref<unknown>(new Error('Network error'))
    const { loadError } = useListError({
      isError,
      error,
      queryKey: ['views', 'list'],
      label: '模板加载失败',
      client: new QueryClient(),
    })
    expect(loadError.value).toBe('模板加载失败：Network error')
    isError.value = false
    expect(loadError.value).toBe('')
  })

  it('32 个同源生成页统一走 useListError，且错误态分支排在空态之前', () => {
    const dir = import.meta.dirname
    expect(GENERATED_PAGES).toHaveLength(32)

    for (const file of GENERATED_PAGES) {
      const source = readFileSync(resolve(dir, file), 'utf8')
      expect(source, `${file} 未接统一失败口径`).toContain('useListError(')
      expect(source, `${file} 缺错误态分支`).toContain('v-else-if="items.length === 0 && loadError"')
      // 错误分支必须出现在「暂无」空态之前，否则失败仍会被空态吃掉。
      const errorIndex = source.indexOf('v-else-if="items.length === 0 && loadError"')
      const emptyIndex = source.indexOf('v-else-if="items.length === 0" class="empty-state"')
      expect(errorIndex, `${file} 找不到错误分支`).toBeGreaterThan(-1)
      expect(emptyIndex, `${file} 找不到空态分支`).toBeGreaterThan(errorIndex)
    }
  })

  it('Dashboard 的待办取数失败不再折叠成「暂无待办」', () => {
    const source = readFileSync(resolve(import.meta.dirname, 'Dashboard.vue'), 'utf8')
    expect(source).toContain('pendingError')
    expect(source).toContain('v-else-if="pendingItems.length === 0 && pendingError"')
    expect(source.indexOf('pendingItems.length === 0 && pendingError')).toBeGreaterThan(-1)
    expect(source.indexOf('暂无待办事项')).toBeGreaterThan(source.indexOf('pendingItems.length === 0 && pendingError'))
  })
})
