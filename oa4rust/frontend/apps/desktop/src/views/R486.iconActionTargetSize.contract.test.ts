// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R486 FileManager .icon-action 目标尺寸契约：
 * WCAG 2.5.8 AA 要求 24x24px 最小点击区域；真父版无 .icon-action 尺寸规则必 RED，
 * 修复版新增 min-width/min-height:24px 必 GREEN；业务属性零改守卫防回归。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'FileManager.vue'), 'utf8')

function ruleBody(src: string, selector: string): string {
  const at = src.indexOf(selector + '{')
  if (at < 0) return ''
  const open = src.indexOf('{', at)
  const close = src.indexOf('}', open)
  if (open < 0 || close < 0) return ''
  return src.slice(open + 1, close)
}

describe('R486 FileManager 图标按钮目标尺寸契约', () => {
  const iconActionBody = ruleBody(source, '.icon-action')

  it('.icon-action 规则块必须声明 min-width:24px 与 min-height:24px（核心缺陷断言）', () => {
    expect(iconActionBody, '必须新增 .icon-action 尺寸规则块').not.toBe('')
    expect(iconActionBody).toMatch(/min-width:\s*24px/)
    expect(iconActionBody).toMatch(/min-height:\s*24px/)
  })

  it('文件行 4 个图标按钮业务属性零改（@click.stop/title/符号）', () => {
    expect(source).toContain('<button class="icon-action" @click.stop="openDetail(f)" title="详情/预览">ℹ</button>')
    expect(source).toContain('<button class="icon-action" @click.stop="downloadFile(f)" title="下载">⬇</button>')
    expect(source).toContain('<button class="icon-action" @click.stop="shareFile(f)" title="分享">↗</button>')
    expect(source).toContain('<button class="icon-action danger" :disabled="deletingId === f.id" @click.stop="deleteFile(f)" title="删除" aria-label="删除文件">🗑</button>')
  })

  it('可清理文件删除按钮业务属性零改', () => {
    expect(source).toContain('<button class="icon-action danger" :disabled="deletingId === f.id" @click="deleteFile(f)" title="删除（进入回收站）" aria-label="删除可清理文件">🗑</button>')
  })

  it('两处关闭按钮 aria-label 与处理器零改', () => {
    expect(source).toContain('<button class="icon-action" aria-label="关闭引用面板" @click="refPanel.open = false">✕</button>')
    expect(source).toContain('<button class="icon-action" aria-label="关闭文件详情" @click="detail.open = false">✕</button>')
  })

  it('7 处图标按钮数量零改，col-actions 与 data-testid 保留', () => {
    const btns = source.match(/<button class="icon-action/g) || []
    expect(btns.length, 'icon-action 按钮必须仍为 7 处').toBe(7)
    expect(source).toContain('<span class="col-actions">')
    expect(source).toContain('data-testid="files-recyclable-list"')
  })
})
