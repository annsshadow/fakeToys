// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

// mobile vitest 未装配 .vue 插件，沿用仓内源码断言惯例（见 writeActions.test.ts / detailRows.test.ts）。
const source = readFileSync(resolve(import.meta.dirname, 'attendance.vue'), 'utf8')

describe('月度考勤并发加载（R277）', () => {
  it('loadMine 用请求序号保证只有最新一次能写权威态', () => {
    // onShow 自动加载 + 两个无禁用态的重试按钮共三个入口。并发时旧响应后到会覆盖新数据；
    // 单纯 busy 守卫又会丢弃 onShow 触发的更新加载，因此用序号只让最新一次回写。
    expect(source).toContain('let loadMineSeq = 0')
    expect(source).toContain('const seq = ++loadMineSeq')

    const fnStart = source.indexOf('async function loadMine()')
    expect(fnStart).toBeGreaterThan(-1)
    const fnBody = source.slice(fnStart, source.indexOf('\nfunction statText()', fnStart))

    const awaitAt = fnBody.indexOf('await Promise.allSettled')
    const staleGuard = fnBody.indexOf('if (seq !== loadMineSeq) return')
    const statAt = fnBody.indexOf('myStat.value =')
    const detailAt = fnBody.indexOf('myDetails.value = toMyDetailRows(detailResp.value.data)')
    const verAt = fnBody.indexOf('apiVersion.value =')
    expect(awaitAt).toBeGreaterThan(-1)
    expect(staleGuard).toBeGreaterThan(-1)
    // 新鲜度检查必须在 await 之后、所有权威态写回之前：否则旧响应后到会覆盖最新数据。
    expect(staleGuard).toBeGreaterThan(awaitAt)
    expect(statAt).toBeGreaterThan(staleGuard)
    expect(detailAt).toBeGreaterThan(staleGuard)
    expect(verAt).toBeGreaterThan(staleGuard)
    // 网络异常也要复位 loading，否则「加载中…」永久卡住；但只复位最新一次。
    expect(fnBody).toContain('} catch (e) {')
    expect(fnBody).toContain('} finally {')
    expect(fnBody).toContain('if (seq === loadMineSeq) myLoading.value = false')
  })
})
