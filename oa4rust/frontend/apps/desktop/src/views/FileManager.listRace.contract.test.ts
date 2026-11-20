// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * R270 文件管理列表竞态：快速切换文件夹时旧响应不得覆盖新目录。
 *
 * 缺陷证据：FileManager.vue 的 openItem/navigateTo 同步更新 currentFolder 与 breadcrumbs，
 * 再异步调用 loadFiles(folderId)。loadFiles 原先无任何「只认最新一次」守卫：
 * 用户双击进入文件夹 A 后立即点面包屑返回根目录，若 A 的 GET 晚于根目录的 GET 返回，
 * A 的文件列表会覆盖 files.value，而面包屑仍显示「根目录」——渲染内容与导航状态错乱。
 * 删除后的 loadFiles(currentFolder.value) 刷新同样可能与导航竞态。
 *
 * 参照 IMChat.vue 的 msgRequestSeq 递增序号先例，契约要求 loadFiles 用序号只认最新响应：
 * 过期响应既不写 files，也不写 loadError，也不复位新请求的 loading。
 */
import { readFileSync } from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const here = path.dirname(fileURLToPath(import.meta.url))
const source = readFileSync(path.join(here, 'FileManager.vue'), 'utf8')

/** 截取 loadFiles 函数体（不含其后的顶层调用）。 */
function loadFilesBody(): string {
  const start = source.indexOf('async function loadFiles(')
  const end = source.indexOf('\nloadFiles()', start)
  if (start < 0 || end < 0) return ''
  return source.slice(start, end)
}

describe('文件管理列表只认最新一次（R270）', () => {
  const body = loadFilesBody()

  it('能定位 loadFiles 函数体（防止契约空转）', () => {
    expect(body.length, '未找到 loadFiles 函数体').toBeGreaterThan(100)
  })

  it('loadFiles 用递增序号登记每次请求', () => {
    expect(source).toMatch(/let\s+fileListSeq\s*=\s*0/)
    expect(body).toMatch(/const\s+seq\s*=\s*\+\+fileListSeq/)
  })

  it('过期的成功响应不写 files', () => {
    const guard = body.indexOf('if (seq !== fileListSeq) return')
    expect(guard, '成功分支缺少过期响应守卫').toBeGreaterThan(-1)
    const filesWrite = body.indexOf('files.value =')
    expect(filesWrite, '未找到 files 写入').toBeGreaterThan(guard)
  })

  it('过期的失败响应不写 loadError', () => {
    const guards = body.split('if (seq !== fileListSeq) return').length - 1
    expect(guards, '成功与失败分支都应有过期守卫').toBeGreaterThanOrEqual(2)
    const catchIdx = body.indexOf('catch (e)')
    expect(catchIdx).toBeGreaterThan(-1)
    const catchBody = body.slice(catchIdx)
    expect(catchBody).toContain('if (seq !== fileListSeq) return')
    expect(catchBody).toContain('loadError.value =')
  })

  it('finally 只在最新请求时复位 loading，旧请求不提前复位', () => {
    expect(body).toContain('if (seq === fileListSeq) loading.value = false')
  })

  it('导航、删除刷新均经 loadFiles，守卫覆盖所有入口', () => {
    expect(source).toContain('loadFiles(f.id)')
    expect(source).toContain('loadFiles(currentFolder.value)')
    expect((source.match(/loadFiles\(/g) || []).length).toBeGreaterThanOrEqual(4)
  })
})
