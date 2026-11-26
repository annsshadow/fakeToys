// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// t33 RED→GREEN：H5 待办列表标题可读优先，ID 折叠副文本；不修改 message.vue。
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(__dirname, './process.vue'), 'utf8')
const script = source.match(/<script setup[^>]*>([\s\S]*?)<\/script>/)?.[1] ?? ''

describe('H5 process 可读流程标题契约', () => {
  it('titleOf 识别 start:S3 与 s3-process 原始标识并回退可读标题', () => {
    expect(script).toMatch(/function titleOf\(row: Record<string, unknown>, i: number\): string/)
    expect(script).toMatch(/\^start:/i)
    expect(script).toMatch(/s3-process/i)
    expect(script).toMatch(/return '运行闭环'/)
    expect(script).toMatch(/return \`流程 #\$\{i \+ 1\}\`/)
  })

  it('所有 start: 前缀与 UUID 均识别为原始键并回退可读标题', () => {
    expect(script).toMatch(/\^start:/)
    expect(script).toMatch(/0-9a-f/i)
  })

  it('列表与详情标题带稳定 testid，ID 作为副文本', () => {
    expect(source).toMatch(/<view class="title"[^>]*data-testid="h5-process-title"[^>]*>/)
    expect(source).toMatch(/data-testid="h5-process-title"[\s\S]*?<text[^>]*class="task-id"[^>]*>\{\{\s*taskIdOf\(row\)\s*\}\}<\/text>/)
    expect(source).toMatch(/<text class="sheet-title"[^>]*data-testid="h5-process-detail-title"/)
  })

  it('提供 taskIdOf 且只做副文本折叠', () => {
    expect(script).toMatch(/function taskIdOf\(/)
    expect(script).toMatch(/function shortId\(/)
  })

  it('H5 附件记录仍使用 textOf，未移除原能力', () => {
    expect(source).toMatch(/textOf\(\s*r,\s*\['title', 'id'\], '记录'\s*\)/)
    expect(source).toMatch(/textOf\(\s*w,\s*\['activityName', 'title', 'id'\], '日志'\s*\)/)
  })
})
