// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// collect.listRace.contract.test.ts: 钉住收藏页列表加载的「只认最新一次」契约。
// 与 message.vue(R278) 同构：onShow 每次显示 + 错误态两处无禁用重试按钮共三个入口，
// load() 无 busy 守卫（不能用 busy 丢弃 onShow 的刷新），改用递增序号让过期响应
// 不写 rows/loadError/loading/loaded，避免旧响应后到把新数据覆盖回旧列表/卡住 loading。
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const src = readFileSync(fileURLToPath(new URL('./collect.vue', import.meta.url)), 'utf8')

function loadBody(): string {
  const start = src.indexOf('async function load()')
  expect(start).toBeGreaterThanOrEqual(0)
  const next = src.indexOf('\nfunction ', start + 1)
  const end = next === -1 ? src.indexOf('</script>', start) : next
  expect(end).toBeGreaterThan(start)
  return src.slice(start, end)
}

describe('移动端收藏列表只认最新一次（R279）', () => {
  it('加载入口可重复触发：onShow 每次显示 + 两处无禁用重试按钮，竞态真实存在', () => {
    expect(src).toContain('onShow(async () => {')
    expect(loadBody().includes('if (loading.value)')).toBe(false)
    const retryLines = src.split('\n').filter((l) => l.includes('@tap="load"'))
    expect(retryLines.length).toBeGreaterThanOrEqual(2)
    for (const line of retryLines) expect(line.includes('disabled')).toBe(false)
  })

  it('load() 在发起请求前登记请求序号', () => {
    expect(src).toContain('let loadSeq = 0')
    const body = loadBody()
    const seqAt = body.indexOf('const seq = ++loadSeq')
    expect(seqAt).toBeGreaterThan(-1)
    expect(seqAt).toBeLessThan(body.indexOf('await loadCollectRows'))
  })

  it('过期响应不写 rows/loadError/loading/loaded', () => {
    const body = loadBody()
    const guard = body.indexOf('if (seq !== loadSeq) return')
    expect(guard).toBeGreaterThan(-1)
    const awaitAt = body.indexOf('await loadCollectRows')
    expect(guard).toBeGreaterThan(awaitAt)
    const afterGuard = body.slice(guard)
    for (const write of [
      'rows.value = result.rows',
      'loadError.value = result.error',
      'loading.value = false',
      'loaded.value = true',
    ]) {
      expect(afterGuard).toContain(write)
    }
  })
})
