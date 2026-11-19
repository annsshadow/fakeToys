// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

describe('移动端会议页状态渲染接线（R290）', () => {
  it('卡片状态委托映射工具，不再在组件内裸比较数字状态', () => {
    const source = readFileSync(resolve(import.meta.dirname, 'meeting.vue'), 'utf8')
    // 规模下限：防止文件被截断后断言空转。
    expect(source.split('\n').length).toBeGreaterThan(250)
    expect(source).toContain("import { meetingStatusText } from './meetingStatus'")
    expect(source).toContain('return meetingStatusText(m)')
    expect(source).toContain('{{ statusText(m) }}')
    // 旧实现：meeting.vue 内直接 if (s === '1'/'2'/'3') 裸比较数字值。
    expect(source).not.toContain("if (s === '1') return '进行中'")
    expect(source).not.toContain("if (s === '2') return '已结束'")
    expect(source).not.toContain("if (s === '3') return '已取消'")
  })
})
