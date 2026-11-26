// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R582 版本页三个原生 select 可访问名契约（源码字符串断言，R503/R578/R580 惯例）。
 * 真父 be7038d33：创建版本 Modal 的选择文件 select、版本对比 Modal 的版本 1/版本 2 select
 * 裸 label 无 htmlFor/id/aria，CDP AX role=combobox name 为空；修复后开标签追加
 * aria-label=选择输入文件 / 版本 1 / 版本 2。
 * 守卫钉死业务 value/onChange/handler/state/样式/R578/R580 既有计数零改，另立契约不混改。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const src = readFileSync(resolve(import.meta.dirname, 'Versions.tsx'), 'utf8')

function selectBlock(valueExpr: string): string {
  const needle = String.raw`value={${valueExpr}}`
  const at = src.indexOf(needle)
  if (at < 0) return ''
  const start = src.lastIndexOf(String.raw`<select`, at)
  const out: string[] = []
  for (const line of src.slice(start).split('\n')) {
    out.push(line)
    if (line.trimEnd().endsWith('>')) break
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
    else if (src[i] === '}') { depth--; if (depth === 0) return src.slice(open, i + 1) }
  }
  return ''
}

function count(haystack: string, needle: string): number {
  return haystack.split(needle).length - 1
}

describe('R582 版本页三个原生 select 可访问名', () => {
  it('核心1：选择文件 select 开标签有 aria-label=选择输入文件', () => {
    const b = selectBlock('selectedFile')
    expect(b.includes(String.raw`aria-label="选择输入文件"`)).toBe(true)
    expect(b.includes('value={selectedFile}')).toBe(true)
    expect(b.includes('onChange={(e) => setSelectedFile(e.target.value)}')).toBe(true)
  })

  it('核心2：版本 1 select 开标签有 aria-label=版本 1', () => {
    const b = selectBlock('diffVersion1')
    expect(b.includes(String.raw`aria-label="版本 1"`)).toBe(true)
    expect(b.includes('value={diffVersion1}')).toBe(true)
    expect(b.includes('onChange={(e) => setDiffVersion1(e.target.value)}')).toBe(true)
  })

  it('核心3：版本 2 select 开标签有 aria-label=版本 2，全页恰 3 处', () => {
    const b = selectBlock('diffVersion2')
    expect(b.includes(String.raw`aria-label="版本 2"`)).toBe(true)
    expect(b.includes('value={diffVersion2}')).toBe(true)
    expect(b.includes('onChange={(e) => setDiffVersion2(e.target.value)}')).toBe(true)
    expect(count(src, 'aria-label=')).toBe(3)
    expect(count(src, String.raw`aria-label="选择输入文件"`)).toBe(1)
    expect(count(src, String.raw`aria-label="版本 1"`)).toBe(1)
    expect(count(src, String.raw`aria-label="版本 2"`)).toBe(1)
  })

  it('守卫4：业务 handler 零改（create/diff/load/show）', () => {
    const create = fnBody('handleCreate')
    expect(create.includes("message.warning('请选择文件')")).toBe(true)
    expect(create.includes('await createVersion(selectedFile, newVersionLabel, newVersionDesc)')).toBe(true)
    expect(create.includes("message.success('创建版本成功')")).toBe(true)
    const diff = fnBody('handleDiff')
    expect(diff.includes("message.warning('请选择两个版本')")).toBe(true)
    expect(diff.includes('await diffVersions(diffVersion1, diffVersion2)')).toBe(true)
    const loadV = fnBody('loadVersions')
    expect(loadV.includes('await getVersions()')).toBe(true)
    const loadF = fnBody('loadFiles')
    expect(loadF.includes('await getDataFiles()')).toBe(true)
    expect(fnBody('handleShowDetail').includes('await getVersion(versionId)')).toBe(true)
    expect(fnBody('handleShowData').includes('await getVersionData(versionId)')).toBe(true)
  })

  it('守卫5：select 原生结构与 value/onChange 全量保留', () => {
    for (const binding of [
      'value={selectedFile}', 'onChange={(e) => setSelectedFile(e.target.value)}',
      'value={diffVersion1}', 'onChange={(e) => setDiffVersion1(e.target.value)}',
      'value={diffVersion2}', 'onChange={(e) => setDiffVersion2(e.target.value)}',
      '<label>选择文件：</label>', '<label>版本 1：</label>', '<label>版本 2：</label>',
    ]) expect(src.includes(binding)).toBe(true)
    expect(count(src, '<select')).toBe(3)
    expect(count(src, '</select>')).toBe(3)
    expect(count(src, '<option value=\"\">请选择文件</option>')).toBe(1)
  })

  it('守卫6：R578/R580 既有修复零改、无全局增强器', () => {
    const aug = readFileSync(resolve(import.meta.dirname, 'Augmentation.tsx'), 'utf8')
    const dm = readFileSync(resolve(import.meta.dirname, 'DataManagement.tsx'), 'utf8')
    expect(count(aug, 'aria-label=')).toBe(6)
    expect(count(dm, 'aria-label=')).toBe(37)
    for (const label of ['输入文件', '输出文件', '质量检查', '智能去重', '断点续传', '异步引擎']) {
      expect(count(aug, 'aria-label=\"' + label + '\"')).toBe(1)
    }
    for (const label of ['问题内容', '回答内容']) {
      expect(count(dm, 'aria-label=\"' + label + '\"')).toBe(1)
    }
    expect(src.includes('MutationObserver')).toBe(false)
    expect(src.includes('document.querySelector')).toBe(false)
  })

  it('守卫7：他页 System 正面对照零改、state/import 零回归', () => {
    const system = readFileSync(resolve(import.meta.dirname, 'System.tsx'), 'utf8')
    expect(system.includes('aria-label=\"刷新依赖诊断\"')).toBe(true)
    expect(system.includes('aria-label=\"备份源文件\"')).toBe(true)
    for (const s of [
      "const [selectedFile, setSelectedFile] = useState('')",
      "const [diffVersion1, setDiffVersion1] = useState('')",
      "const [diffVersion2, setDiffVersion2] = useState('')",
      'getVersions,', 'createVersion,', 'diffVersions,', 'getDataFiles,',
      "<Modal", 'onOk={handleCreate}', 'onOk={handleDiff}',
    ]) expect(src.includes(s)).toBe(true)
  })
})
