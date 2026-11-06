// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { describe, expect, it } from 'vitest'
import { loadDocLists } from './docFeed'

describe('loadDocLists', () => {
  it('并发发起：第一份清单未返回前，第二份必须已经开始', async () => {
    let releaseFiles!: () => void
    const filesGate = new Promise<void>((resolve) => {
      releaseFiles = resolve
    })
    const started: string[] = []
    const request = loadDocLists(
      {
        fileList: async () => {
          started.push('files')
          await filesGate
          return { data: [{ id: 'f1' }] }
        },
        attachmentList: async () => {
          started.push('attachments')
          return { data: [{ id: 'a1' }] }
        },
      },
      { files: [], attachments: [] },
    )
    await new Promise((resolve) => setTimeout(resolve, 0))
    const startedBeforeRelease = [...started]
    releaseFiles()
    const snapshot = await request
    expect(startedBeforeRelease).toEqual(['files', 'attachments'])
    expect(snapshot.files).toEqual({ ok: true, rows: [{ id: 'f1' }] })
    expect(snapshot.attachments).toEqual({ ok: true, rows: [{ id: 'a1' }] })
  })

  it('单侧失败不拖累另一侧，并保留上一次成功的数据', async () => {
    const snapshot = await loadDocLists(
      {
        fileList: async () => {
          throw new Error('Network error')
        },
        attachmentList: async () => ({ data: [{ id: 'a1' }] }),
      },
      { files: [{ id: 'old' }], attachments: [] },
    )
    expect(snapshot.files.ok).toBe(false)
    if (!snapshot.files.ok) {
      expect(snapshot.files.rows).toEqual([{ id: 'old' }])
      expect(snapshot.files.error).toBe('文件加载失败：Network error')
    }
    expect(snapshot.attachments).toEqual({ ok: true, rows: [{ id: 'a1' }] })
  })

  it('两侧都失败时各自给出自己的错误文案', async () => {
    const snapshot = await loadDocLists(
      {
        fileList: async () => {
          throw new Error('network down')
        },
        attachmentList: async () => {
          throw new Error('403')
        },
      },
      { files: [], attachments: [] },
    )
    expect(snapshot.files.ok).toBe(false)
    expect(snapshot.attachments.ok).toBe(false)
    if (!snapshot.files.ok) expect(snapshot.files.error).toBe('文件加载失败：network down')
    if (!snapshot.attachments.ok) expect(snapshot.attachments.error).toBe('附件加载失败：403')
  })
})
