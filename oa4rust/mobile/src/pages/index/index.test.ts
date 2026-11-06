// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

/**
 * 首页公告区的失败态是页面内状态（mobile 测试为 node 环境，无法挂载 .vue 渲染），
 * 这里用源码契约守住两条 R137 判据：加载开始即清空错误；错误分支先于「暂无公告」。
 */
describe('首页公告：弱网失败态不残留（R137）', () => {
  const source = readFileSync(resolve(import.meta.dirname, 'index.vue'), 'utf8')

  it('发起加载前清空上一次错误，重试成功后不会挂着旧错误横幅', () => {
    // 规模下限：防止文件被截断后断言空转。
    expect(source.split('\n').length).toBeGreaterThan(80)
    const clear = source.indexOf("noticesError.value = ''")
    const load = source.indexOf('await loadList')
    expect(clear).toBeGreaterThan(-1)
    expect(load).toBeGreaterThan(clear)
  })

  it('无数据时错误分支先于「暂无公告」；有数据时错误只以横幅形式与列表并存', () => {
    const errorBranch = source.indexOf('notices.length === 0 && noticesError')
    const emptyBranch = source.indexOf('暂无公告')
    expect(errorBranch).toBeGreaterThan(-1)
    expect(emptyBranch).toBeGreaterThan(errorBranch)
    expect(source).toContain('noticesError && notices.length > 0')
  })
})
