// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R503 WCAG 3.3.4 删除断点二次确认契约（源码字符串断言）.
 * 真父 fb973d8a 上 doDeleteCheckpoint 单击直接 deleteCheckpoint，无 Modal.confirm；
 * 修复后必须在 onOk 内才调用 deleteCheckpoint，取消路径不删除。
 * 守卫锁定业务语义：Tag onClose / toast 文案 / testid / loadCheckpoints / 端点零改。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const src = readFileSync(resolve(import.meta.dirname, 'Augmentation.tsx'), 'utf8')

// 花括号配平提取具名箭头函数体（不依赖具体缩进/行数）
function fnBody(name: string): string {
  const at = src.indexOf('const ' + name)
  if (at < 0) return ''
  const open = src.indexOf('{', at)
  if (open < 0) return ''
  let depth = 0
  for (let i = open; i < src.length; i++) {
    if (src[i] === '{') depth++
    else if (src[i] === '}') {
      depth--
      if (depth === 0) return src.slice(open, i + 1)
    }
  }
  return ''
}

describe('R503 删除断点二次确认', () => {
  it('doDeleteCheckpoint 用 Modal.confirm 包裹且提供 onOk 确认回调', () => {
    const body = fnBody('doDeleteCheckpoint')
    expect(body.includes('Modal.confirm(')).toBe(true)
    expect(body.includes('onOk:')).toBe(true)
  })

  it('deleteCheckpoint 只在 onOk 回调内调用，onOk 之前绝不删除', () => {
    const body = fnBody('doDeleteCheckpoint')
    const onOk = body.indexOf('onOk:')
    expect(onOk).toBeGreaterThan(-1)
    const before = body.slice(0, onOk)
    expect(before.includes('deleteCheckpoint(')).toBe(false)
    const call = body.indexOf('deleteCheckpoint(')
    expect(call).toBeGreaterThan(onOk)
  })

  it('确认文案含从头重跑提示，删除调用恰一次且取消无调用', () => {
    const body = fnBody('doDeleteCheckpoint')
    expect(body.includes('从头重跑')).toBe(true)
    expect(body.split('deleteCheckpoint(').length - 1).toBe(1)
    expect(body.includes('onCancel')).toBe(false)
  })

  it('守卫：Tag onClose / testid / toast 文案 / 错误分支零改', () => {
    expect(src.includes('data-testid="checkpoint-list"')).toBe(true)
    expect(src.includes('onClose={(e) => {')).toBe(true)
    expect(src.includes('e.preventDefault()')).toBe(true)
    expect(src.includes('doDeleteCheckpoint(id)')).toBe(true)
    expect(src.includes('message.success(')).toBe(true)
    expect(src.includes('断点已删除')).toBe(true)
    expect(src.includes('删除断点失败')).toBe(true)
    expect(src.includes('apiErrorDetail(err, \'删除断点失败\')')).toBe(true)
  })

  it('守卫：loadCheckpoints 与 antd 既有 import 零改，未新增端点直连', () => {
    const load = fnBody('loadCheckpoints')
    expect(load.includes('getCheckpoints()')).toBe(true)
    expect(load.includes('setCheckpoints(result.checkpoints)')).toBe(true)
    expect(load.includes('加载断点列表失败')).toBe(true)
    expect(src.includes('Card, Button, Space, Select, Switch, message, Progress, Tag, Input')).toBe(true)
    expect(src.includes("from '../services/api'")).toBe(true)
    expect(bodyHasFetch()).toBe(false)
  })
})

function bodyHasFetch(): boolean {
  const body = fnBody('doDeleteCheckpoint')
  return body.includes('fetch(') || body.includes('axios') || body.includes('api.');
}