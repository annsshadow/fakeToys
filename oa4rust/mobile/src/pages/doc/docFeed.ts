// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { type LoadOutcome, loadList } from '@/utils/loadList'

export type DocRow = Record<string, unknown>

export interface DocListSources {
  fileList: () => Promise<{ data?: unknown } | undefined>
  attachmentList: () => Promise<{ data?: unknown } | undefined>
}

export interface DocListSnapshot {
  files: LoadOutcome<DocRow>
  attachments: LoadOutcome<DocRow>
}

/**
 * 「我的文件」首屏要拉两份互相独立的清单：x_file（我的文件）与 FILE_FILE（附件存储）。
 * 二者没有先后依赖，必须并发发起——串行时第二块要等第一块完整返回，
 * 弱网下首屏可见时间会退化成「两个 RTT 之和」。
 */
export async function loadDocLists(
  sources: DocListSources,
  previous: { files: readonly DocRow[]; attachments: readonly DocRow[] },
): Promise<DocListSnapshot> {
  const [files, attachments] = await Promise.all([
    loadList<DocRow>(sources.fileList, { previousRows: previous.files, label: '文件加载失败' }),
    loadList<DocRow>(sources.attachmentList, { previousRows: previous.attachments, label: '附件加载失败' }),
  ])
  return { files, attachments }
}
