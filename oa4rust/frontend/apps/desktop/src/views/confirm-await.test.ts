// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

function read(rel: string): string {
  return readFileSync(resolve(import.meta.dirname, rel), 'utf8')
}

describe('confirmMsg 必须 await（Promise 恒 truthy 使确认框变 no-op）', () => {
  const sites: Array<[string, string]> = [
    ['QueryManager.vue', 'if (await confirmMsg('],
    ['ProcessDesigner.vue', "!(await confirmMsg('清空画布？所有节点和连线将删除。'))"],
    ['QueryManagerDeep.vue', '!(await confirmMsg(`确定删除该${kind}？`))'],
    ['QueryManagerDeep.vue', "!(await confirmMsg('确定删除此查询？'))"],
    ['ScriptWorkbench.vue', "!(await confirmMsg('确定删除该脚本？'))"],
  ]

  it.each(sites)('%s 删除/清空确认已 await', (file, needle) => {
    const rel = file.startsWith('Script') ? `../components/${file}` : `./${file}`
    expect(read(rel)).toContain(needle)
  })

  it('全 desktop 无残留未 await 的 confirmMsg（同步函数守卫）', () => {
    // 逐文件断言不再存在「!confirmMsg(」且同一行无 await 的写法
    const files = [
      './QueryManager.vue',
      './QueryManagerDeep.vue',
      './ProcessDesigner.vue',
      '../components/ScriptWorkbench.vue',
    ]
    for (const f of files) {
      const src = read(f)
      const bad = src
        .split('\n')
        .filter(
          (l) => l.includes('confirmMsg(') && !l.includes('await confirmMsg') && !l.includes('function confirmMsg'),
        )
      expect(bad, `${f} 存在未 await 的 confirmMsg 调用`).toEqual([])
    }
  })
})
