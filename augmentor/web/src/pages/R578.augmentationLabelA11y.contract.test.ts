// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R578 数据增强页 6 控件可访问名契约（源码字符串断言，R503 惯例）。
 * 真父 c6c10726e：输入 Select / 输出 input / 4 Switch 的裸 label 无 for/id/aria 关联，
 * CDP AX name=null；修复后每控件开标签补 aria-label。任务 ID Input 靠 placeholder
 * 自动补名，严禁重复补 aria-label（守卫钉死零改）。
 * 守卫钉死业务 handler/state/交互绑定/DataManagement 正面对照零改。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const src = readFileSync(resolve(import.meta.dirname, 'Augmentation.tsx'), 'utf8')

// 从起始锚点向下扫到开标签结束行（trimEnd 以 > 收尾），支持多行开标签
function blockFrom(anchor: string): string {
  const at = src.indexOf(anchor)
  if (at < 0) return ''
  const lines = src.slice(at).split('\n')
  const out: string[] = []
  for (const line of lines) {
    out.push(line)
    const t = line.trimEnd()
    if (t.endsWith('>')) break
  }
  return out.join('\n')
}

// 花括号配平提取具名箭头函数体（R503 惯例）
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

describe('R578 数据增强页 6 控件可访问名', () => {
  it('核心1：输入文件 Select 开标签有 aria-label=输入文件', () => {
    const block = blockFrom('<Select')
    expect(block.includes('aria-label="输入文件"')).toBe(true)
    expect(block.includes('placeholder="选择输入文件"')).toBe(true)
  })

  it('核心2：输出文件原生 input 开标签有 aria-label=输出文件', () => {
    const block = blockFrom('<input')
    expect(block.includes('aria-label="输出文件"')).toBe(true)
    expect(block.includes('type="text"')).toBe(true)
  })

  it('核心3：四个 antd Switch 各有对应 aria-label', () => {
    expect(src.includes('<Switch aria-label="质量检查" checked={useQuality} onChange={setUseQuality} />')).toBe(true)
    expect(src.includes('<Switch aria-label="智能去重" checked={useDedup} onChange={setUseDedup} />')).toBe(true)
    expect(src.includes('<Switch aria-label="断点续传" checked={useCheckpoint} onChange={setUseCheckpoint} />')).toBe(true)
    expect(src.includes('<Switch aria-label="异步引擎" checked={useAsync} onChange={setUseAsync} />')).toBe(true)
  })

  it('核心4：全页 aria-label 恰 6 处（真父 0），6 处文案各一次', () => {
    expect(count(src, 'aria-label=')).toBe(6)
    for (const label of ['输入文件', '输出文件', '质量检查', '智能去重', '断点续传', '异步引擎']) {
      expect(count(src, 'aria-label="' + label + '"')).toBe(1)
    }
  })

  it('守卫5：任务 ID Input 零改，无 aria-label，placeholder 自动补名', () => {
    const block = blockFrom('<Input')
    expect(block.includes('placeholder="增强任务的任务 ID"')).toBe(true)
    expect(block.includes('aria-label')).toBe(false)
    expect(block.includes('value={reportTaskId}')).toBe(true)
    expect(block.includes('onChange={(e) => setReportTaskId(e.target.value)}')).toBe(true)
  })

  it('守卫6：业务 handler/state 零改', () => {
    const loadFiles = fnBody('loadFiles')
    expect(loadFiles.includes('getDataFiles()')).toBe(true)
    expect(loadFiles.includes('setFiles(result.files.map((f) => f.name))')).toBe(true)
    const check = fnBody('checkProgress')
    expect(check.includes('getProgress(taskIdRef.current)')).toBe(true)
    expect(check.includes("if ('task_id' in result)")).toBe(true)
    const start = fnBody('handleStart')
    expect(start.includes('startAugmentation(')).toBe(true)
    expect(start.includes('taskIdRef.current = started.task_id')).toBe(true)
    const report = fnBody('loadReport')
    expect(report.includes('getAugmentReport(id)')).toBe(true)
    expect(report.includes('setReportLoading(false)')).toBe(true)
  })

  it('守卫7：交互绑定/import/api 零改，无全局增强器', () => {
    for (const binding of [
      'onChange={setInputFile}',
      'onChange={(e) => setOutputFile(e.target.value)}',
      'onChange={setUseQuality}',
      'onChange={setUseDedup}',
      'onChange={setUseCheckpoint}',
      'onChange={setUseAsync}',
      'onChange={(e) => setReportTaskId(e.target.value)}',
    ]) expect(src.includes(binding)).toBe(true)
    expect(src.includes("from '../services/api'")).toBe(true)
    expect(src.includes('useEffect')).toBe(true)
    expect(src.includes('MutationObserver')).toBe(false)
    expect(src.includes('document.querySelector')).toBe(false)
  })

  it('守卫8：DataManagement 正面对照零改 + 控件结构数量恒定', () => {
    const dm = readFileSync(resolve(import.meta.dirname, 'DataManagement.tsx'), 'utf8')
    expect(dm.includes('aria-label="上传源格式"')).toBe(true)
    expect(dm.includes('aria-label="转换目标格式"')).toBe(true)
    expect(dm.includes('aria-label="分割输出目录"')).toBe(true)
    expect(count(src, '<Switch')).toBe(4)
    expect(count(src, '<Select\n')).toBe(1) // Select.Option 不计入控件
    expect(count(src, '<Input')).toBe(1)
  })
})
