// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'OrgDirectoryApp.vue'), 'utf8')

/** 从源码提取 s 的箭头函数实现，去掉 TS 类型注解后转为可执行 JS。 */
function extractS(src: string): (p: unknown) => Promise<unknown> {
  const line = src.match(/const s = [^\n]+/)?.[0]
  if (!line) throw new Error('未找到 s 的定义')
  // 只提取箭头函数体：签名里第一个 => 之后的表达式，避开泛型与类型注解。
  const arrow = line.indexOf('=>')
  if (arrow < 0) throw new Error('s 不是箭头函数')
  const body = line.slice(arrow + 2).trim()
  // 本测试只执行从自身源码提取的纯表达式，参数名固定为 p（与源码一致）。
  return new Function('p', `return (${body})`) as (p: unknown) => Promise<unknown>
}

describe('BUG-D：组织通讯录查询 await 位置错误', () => {
  it('collect 的 API Promise 参数不得被 await 提前解析', () => {
    // 旧代码：collect('label', await api.get/post(...))。
    // 参数位置的 await 把 Promise 解析成普通值，s(p) 里 p.catch 为 undefined → TypeError；
    // 若 API reject，异常在进入 collect 前抛出（runAll 不捕获）→ 未处理拒绝 + 无「查询失败」提示。
    expect(source).not.toMatch(/collect\([^)]*await\s+api\.(get|post)\(/)
    expect(source).not.toMatch(/collect\([\s\S]{0,160}?await\s+api\.(get|post)\(/)
  })

  it('每个 collect 调用都把 api Promise 直接作为第二参数传入', () => {
    const params = [...source.matchAll(/collect\(\s*'[^']*'\s*,\s*([\s\S]{0,90}?)\)/g)].map((m) => m[1].trim())
    expect(params.length).toBeGreaterThanOrEqual(40)
    for (const p of params) expect(p).toMatch(/^api\.(get|post)\(/)
  })

  it('s 对已解析的普通值安全（不再出现 p.catch is not a function）', async () => {
    const s = extractS(source)
    await expect(s({ data: [{ id: 1 }] })).resolves.toEqual({ data: [{ id: 1 }] })
  })

  it('s 把 reject 转成 null，使 collect 显示「查询失败」', async () => {
    const s = extractS(source)
    await expect(s(Promise.reject(new Error('boom')))).resolves.toBeNull()
  })
})
