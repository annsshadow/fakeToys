// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R580 数据管理页编辑弹窗 2 处 TextArea 可访问名契约（源码字符串断言，R503/R578 惯例）。
 * 真父 94c4fa76f：编辑数据 Modal 内 2 处 Input.TextArea 裸 label 无 for/id/aria/placeholder，
 * antd6 对无 placeholder 的 TextArea 不自动补名，CDP AX name=null；修复后开标签补
 * aria-label=问题内容 / 回答内容。
 * 守卫钉死业务 handler/state/交互绑定/2 处 onChange/弹窗开关/Augmentation R578 修复/
 * 本页既有 35 处 aria-label 正面对照零改。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const src = readFileSync(resolve(import.meta.dirname, 'DataManagement.tsx'), 'utf8')

function textareaBlock(n: number): string {
  let at = -1
  for (let i = 0; i < n; i++) at = src.indexOf('<Input.TextArea', at + 1)
  if (at < 0) return ''
  const out: string[] = []
  for (const line of src.slice(at).split('\n')) {
    out.push(line)
    if (line.trimEnd().endsWith('/>')) break
  }
  return out.join('\n')
}

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

function count(haystack: string, needle: string): number {
  return haystack.split(needle).length - 1
}

describe('R580 数据管理编辑弹窗 2 处 TextArea 可访问名', () => {
  it('核心1：问题 TextArea 开标签有 aria-label=问题内容', () => {
    const block = textareaBlock(1)
    expect(block.includes('aria-label="问题内容"')).toBe(true)
    expect(block.includes('rows={3}')).toBe(true)
    expect(block.includes('value={editingItem.instruction}')).toBe(true)
  })

  it('核心2：回答 TextArea 开标签有 aria-label=回答内容', () => {
    const block = textareaBlock(2)
    expect(block.includes('aria-label="回答内容"')).toBe(true)
    expect(block.includes('rows={5}')).toBe(true)
    expect(block.includes('value={editingItem.output}')).toBe(true)
  })

  it('核心3：全页 aria-label 恰 37 处（真父 35），两处文案各一次', () => {
    expect(count(src, 'aria-label=')).toBe(37)
    expect(count(src, 'aria-label="问题内容"')).toBe(1)
    expect(count(src, 'aria-label="回答内容"')).toBe(1)
    expect(count(textareaBlock(1), 'aria-label=')).toBe(1)
    expect(count(textareaBlock(2), 'aria-label=')).toBe(1)
  })

  it('守卫4：业务 handler/坐标映射零改', () => {
    const edit = fnBody('handleEdit')
    expect(edit.includes('setEditingItem({ ...item })')).toBe(true)
    expect(edit.includes('setEditingIndex(index)')).toBe(true)
    expect(edit.includes('setEditModalVisible(true)')).toBe(true)
    const save = fnBody('handleSaveEdit')
    expect(save.includes('await updateDataItem(selectedFile, editingIndex, editingItem)')).toBe(true)
    expect(save.includes("message.success('保存成功')")).toBe(true)
    expect(save.includes('setEditModalVisible(false)')).toBe(true)
    expect(save.includes('loadDataList()')).toBe(true)
    expect(src.includes('const absoluteIndex = (rowIndex: number) => rowIndices[rowIndex] ?? rowIndex')).toBe(true)
  })

  it('守卫5：2 处 onChange 交互/state/弹窗开关零改', () => {
    for (const binding of [
      'onChange={(e) => setEditingItem({ ...editingItem, instruction: e.target.value })}',
      'onChange={(e) => setEditingItem({ ...editingItem, output: e.target.value })}',
      'onClick={() => handleEdit(record, absoluteIndex(index))}',
      'open={editModalVisible}',
      'onOk={handleSaveEdit}',
      'onCancel={() => setEditModalVisible(false)}',
      'const [editModalVisible, setEditModalVisible] = useState(false)',
      'const [editingItem, setEditingItem] = useState<DataItem | null>(null)',
    ]) expect(src.includes(binding)).toBe(true)
  })

  it('守卫6：Augmentation R578 修复零改，无全局增强器', () => {
    const aug = readFileSync(resolve(import.meta.dirname, 'Augmentation.tsx'), 'utf8')
    for (const label of ['输入文件', '输出文件', '质量检查', '智能去重', '断点续传', '异步引擎']) {
      expect(count(aug, 'aria-label="' + label + '"')).toBe(1)
    }
    expect(src.includes('MutationObserver')).toBe(false)
    expect(src.includes('document.querySelector')).toBe(false)
  })

  it('守卫7：本页既有 aria-label 正面对照零改 + 结构数量恒定', () => {
    for (const label of ['上传源格式', '转换目标格式', '分割输出目录']) {
      expect(count(src, 'aria-label="' + label + '"')).toBe(1)
    }
    expect(count(src, '<Input.TextArea')).toBe(2)
    expect(count(src, '<Modal')).toBe(1)
  })
})
