// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

function view(name: string): string {
  return readFileSync(resolve(import.meta.dirname, name), 'utf8')
}

// R153 已为 Collect/Note/Document/File/Recycle/Calendar 加过重入守卫，
// 但同族 CRUD 列表页的删除动作仍无守卫：confirmMsg 是异步的，快速双击会让
// delM.mutate 执行两次（非幂等 DELETE）。这批页面必须补齐与 CollectApp 同构的守卫。
const CRUD_DELETE_PAGES = [
  'CmsColumnApp.vue',
  'CmsColumnManagerApp.vue',
  'CmsDictDesignerApp.vue',
  'CmsFormDesignerApp.vue',
  'CmsIndexApp.vue',
  'CmsModuleApp.vue',
  'CmsViewDesignerApp.vue',
  'CmsXformApp.vue',
  'CommonApp.vue',
  'DeploymentApp.vue',
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

describe('CRUD 列表页删除重入守卫（R156 / backlog B2）', () => {
  it.each(CRUD_DELETE_PAGES)('%s：删除有按行守卫、异步确认在 mutate 前、onSettled 复位、按钮禁用', (file) => {
    const source = view(file)
    // 页面自身维护进行中标记，不依赖 mutation.isPending（同页多行的 isPending 会互相串扰）。
    expect(source).toContain("const deletingId = ref('')")
    // 只检查 deleteItem 函数体：部分页面在第 400+ 行有其它 deleteXxx 的 confirmMsg，
    // 用全文件 indexOf 会误取到别的函数，故先切出本页的 deleteItem。
    const body = source.slice(source.indexOf('async function deleteItem(item: Item) {'))
    // 已进入删除流程的再次点击直接返回，防止重复 DELETE。
    expect(body).toContain('if (deletingId.value) return')
    // 必须先 await 用户确认，再占用守卫并发请求。
    const guardIdx = body.indexOf('if (deletingId.value) return')
    const confirmIdx = body.indexOf('await confirmMsg(')
    const occupyIdx = body.indexOf('deletingId.value = item.id')
    const mutateIdx = body.indexOf("delM.mutate(item.id, { onSettled: () => (deletingId.value = '') })")
    expect(guardIdx).toBeGreaterThan(-1)
    expect(confirmIdx).toBeGreaterThan(guardIdx)
    expect(occupyIdx).toBeGreaterThan(confirmIdx)
    expect(mutateIdx).toBeGreaterThan(occupyIdx)
    // 请求结束（成功或失败）都必须复位守卫，防止按钮永久卡死。
    expect(body).toContain("onSettled: () => (deletingId.value = '')")
    expect(source).toContain(':disabled="deletingId === item.id"')
  })
})
