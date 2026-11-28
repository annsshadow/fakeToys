// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import ts from 'typescript'
import { describe, expect, it } from 'vitest'
import { effectScope, nextTick, ref, watch } from 'vue'

const src = readFileSync(resolve(import.meta.dirname, 'MeetingApp.vue'), 'utf8')

interface BldgLike {
  id: string
  name: string
}

// 从源码提取 useQuery 段到"代际令牌"注释前的回填逻辑，真实执行
function setupWithAsyncBldgs() {
  const buildings = ref<BldgLike[]>([])
  const bData = ref<BldgLike[] | undefined>(undefined)
  const scope = effectScope()
  scope.run(() => {
    const start = src.indexOf('const { data: bData } = useQuery({')
    const end = src.indexOf('// 代际令牌', start)
    if (start < 0 || end < 0) throw new Error('源码锚点缺失')
    const logic = src.slice(start, end)
    const js = ts.transpileModule(logic, {
      compilerOptions: { target: ts.ScriptTarget.ES2020, module: ts.ModuleKind.None },
    }).outputText
    const fn = new Function('useQuery', 'api', 'watch', 'buildings', js)
    fn(() => ({ data: bData }), { get: async () => ({ data: [] }) }, watch, buildings)
  })
  return { buildings, bData, scope }
}

describe('R639b MeetingApp 楼栋列表异步回填真修复', () => {
  it('异步到达三栋楼后 buildings 从 [] 回填为三栋', async () => {
    const { buildings, bData, scope } = setupWithAsyncBldgs()
    expect(buildings.value).toEqual([])
    bData.value = [
      { id: 'b1', name: '楼栋A' },
      { id: 'b2', name: '楼栋B' },
      { id: 'b3', name: '楼栋C' },
    ]
    await nextTick()
    expect(buildings.value).toHaveLength(3)
    expect(buildings.value.map((b: BldgLike) => b.name)).toEqual(['楼栋A', '楼栋B', '楼栋C'])
    scope.stop()
  })

  it('首屏 bData undefined 不报错且 buildings 为空数组', () => {
    const { buildings, scope } = setupWithAsyncBldgs()
    expect(buildings.value).toEqual([])
    scope.stop()
  })

  it('源码以 watch(bData 注册回填且不再同步取值', () => {
    expect(src).toMatch(/watch\s*\(\s*bData\s*,/)
    expect(src).not.toContain('buildings.value = bData.value ?? []')
  })

  it('import 从 vue 引入 watch', () => {
    const imp = src.slice(src.indexOf('import { computed'), src.indexOf("} from 'vue'"))
    expect(imp).toContain('watch')
  })

  it('对照 :548 addBuilding 异步回填写法仍保留', () => {
    expect(src).toContain('buildings.value = (r.data ?? []) as Bldg[]')
  })
})
