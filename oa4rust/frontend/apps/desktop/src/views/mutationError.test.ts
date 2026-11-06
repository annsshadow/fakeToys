// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readdirSync, readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { ApiError, PermissionError } from '@oa4rust/sdk'
import { describe, expect, it } from 'vitest'
import { FORBIDDEN_MESSAGE, GENERIC_MESSAGE, mutationErrorMessage, NOT_FOUND_MESSAGE } from './mutationError'

describe('写操作失败文案（403/404 不再静默）', () => {
  it('403 → 明确的无权限文案', () => {
    expect(mutationErrorMessage(new PermissionError('forbidden'))).toBe(FORBIDDEN_MESSAGE)
  })

  it('404 → 目标不存在的文案', () => {
    expect(mutationErrorMessage(new ApiError('HTTP 404', 404))).toBe(NOT_FOUND_MESSAGE)
  })

  it('其它错误 → 通用失败文案', () => {
    expect(mutationErrorMessage(new ApiError('HTTP 500', 500))).toBe(GENERIC_MESSAGE)
    expect(mutationErrorMessage(new Error('network'))).toBe(GENERIC_MESSAGE)
  })
})

describe('每个 useMutation 都必须挂 onError（防静默回归）', () => {
  // 源码契约：views 下每个 useMutation 对应至少一个 onError。
  // 只改代表文件会漏掉生成模板页，这个扫描把「漏改」变成红灯。
  const viewsDir = import.meta.dirname
  const sources = readdirSync(viewsDir)
    .filter((f) => f.endsWith('.vue'))
    .map((f) => ({ f, src: readFileSync(resolve(viewsDir, f), 'utf8') }))
    .filter(({ src }) => src.includes('useMutation('))

  it('mutation 数量与 onError 数量对齐（CollectApp 的独立 helper 同样计入）', () => {
    const missing = sources
      .map(({ f, src }) => ({
        f,
        mutations: src.match(/useMutation\(/g)?.length ?? 0,
        errors: src.match(/onError:/g)?.length ?? 0,
      }))
      .filter(({ mutations, errors }) => errors < mutations)
      .map(({ f, mutations, errors }) => `${f}: useMutation=${mutations} onError=${errors}`)

    expect(missing).toEqual([])
  })

  it('扫描覆盖面足够（防止目录/匹配规则失效后空跑）', () => {
    expect(sources.length).toBeGreaterThanOrEqual(25)
  })
})
