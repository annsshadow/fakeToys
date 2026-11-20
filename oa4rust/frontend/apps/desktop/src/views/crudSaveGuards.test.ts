// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

function view(name: string): string {
  return readFileSync(resolve(import.meta.dirname, name), 'utf8')
}

// R157：这批 CRUD 页的 saveM 曾写成
//   saving.value = true; try { return api.post(...) } finally { saving.value = false }
// async 函数 return promise 而不 await 时，finally 在请求 settle 前同步执行，
// 所以 :disabled="saving" 根本挡不住双击。守卫必须落在 saveItem（mutate 前同步占用），
// 复位必须交给 useMutation 的 onSettled。
const CRUD_SAVE_PAGES = [
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
  'NoteApp.vue',
  'PortalDictDesignerApp.vue',
  'PortalPageDesignerApp.vue',
  'PortalWidgetDesignerApp.vue',
  'ProcessApplicationApp.vue',
  'ProcessDictDesignerApp.vue',
  'ProcessFormDesignerApp.vue',
  'ProcessXformApp.vue',
  'QueryImporterDesignerApp.vue',
  'QueryQueryApp.vue',
  'QueryStatDesignerApp.vue',
  'ServiceInvokeDesignerApp.vue',
  'TemplateApp.vue',
  'ThreeMemberApp.vue',
]

function optionBlock(source: string, marker: string): string {
  const from = source.indexOf(marker)
  if (from < 0) return ''
  const end = source.indexOf('\n})', from)
  return source.slice(from, end < 0 ? undefined : end + 3)
}

describe('CRUD 列表页保存重入守卫（R157）', () => {
  it.each(CRUD_SAVE_PAGES)('%s：保存同步守卫、onSettled 复位、按钮禁用', (file) => {
    const source = view(file)
    expect(source).toMatch(/(?:const |,\s*)saving = ref\(false\)/)

    const saveM = optionBlock(source, 'const saveM = useMutation({')
    expect(saveM).not.toBe('')
    // 复位只能出现一次，且必须来自 onSettled；mutationFn 里的同步 finally 必须移除。
    expect(saveM).toContain('onSettled: () => (saving.value = false)')
    expect(saveM).not.toContain('finally {')
    expect(saveM.match(/saving\.value = false/g)?.length ?? 0).toBe(1)

    const from = source.indexOf('function saveItem() {')
    expect(from).toBeGreaterThan(-1)
    const saveItem = source.slice(from, source.indexOf('\n}', from) + 2)
    const guardIdx = saveItem.indexOf('if (saving.value) return')
    const occupyIdx = saveItem.indexOf('saving.value = true')
    const mutateIdx = saveItem.indexOf('saveM.mutate(')
    expect(guardIdx).toBeGreaterThan(-1)
    expect(occupyIdx).toBeGreaterThan(guardIdx)
    expect(mutateIdx).toBeGreaterThan(occupyIdx)

    expect(source).toMatch(/<button[^>]*class="btn-save"[^>]*disabled="[^"]*\bsaving\b[^"]*"[^>]*>/)
  })
})
