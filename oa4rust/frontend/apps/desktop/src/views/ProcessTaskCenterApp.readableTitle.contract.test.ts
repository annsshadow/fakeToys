// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// t33 RED→GREEN：流程任务列表展示层必须可读标题优先、原始 ID 折叠为副文本。
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(
  resolve(__dirname, './ProcessTaskCenterApp.vue'),
  'utf8',
)

function scriptOf(sfc: string): string {
  return sfc.match(/<script setup[^>]*>([\s\S]*?)<\/script>/)?.[1] ?? ''
}

describe('ProcessTaskCenterApp 可读流程标题契约', () => {
  it('列表标题单元格带稳定 testid 且原始 ID 只作为副文本', () => {
    expect(source).toMatch(/<td[^>]*data-testid="ptc-title-cell"[^>]*>/)
    expect(source).toMatch(/<td[^>]*data-testid="ptc-title-cell"[\s\S]*?<span class="task-title"[^>]*>{{ displayTitle\(item\) }}<\/span>[\s\S]*?<span class="task-id"[^>]*>/)
    expect(source).not.toMatch(/<td>{{ displayTitle\(item\) }}<\/td>/)
  })

  it('displayTitle 对 start:S3 与 s3-process 原始标识回退为可读标题', () => {
    const body = scriptOf(source)
    expect(body).toMatch(/function displayTitle\(item: Item\): string/)
    expect(body).toMatch(/\^start:/)
    expect(body).toMatch(/s3-process/)
    expect(body).toMatch(/return 'S3 运行闭环'/)
    expect(body).not.toMatch(/if \(\/\^work-\/\.test\(raw\) \|\| \/\^work-reject-\/\.test\(raw\)\) return '工作实例'/)
  })

  it('所有 start: 前缀与 UUID 均识别为原始键', () => {
    const body = scriptOf(source)
    expect(body).toMatch(/\^start:/)
    expect(body).toMatch(/0-9a-f/i)
  })

  it('工作实例 chips 使用统一可读标题且不显示裸 ID', () => {
    expect(source).not.toMatch(/<span v-for="w in workItems"[\s\S]*?\{\{ w\.title \|\| w\.id \}\}/)
    expect(source).toMatch(/<span v-for="w in workItems"[\s\S]*?\{\{ workTitle\(w\) \}\}/)
    expect(scriptOf(source)).toMatch(/function workTitle\(/)
  })

  it('搜索仍覆盖标题/流程/处理人，保留原过滤能力', () => {
    for (const k of ['title', 'work', 'person']) {
      expect(scriptOf(source)).toContain(`i.${k}`)
    }
  })
})
