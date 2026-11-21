// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R314 D2 文件上传 415 修复契约：桌面真实 multipart 上传端点必须被后端以 Multipart 接收，
 * 真实字节经 BlobStorage 落盘并写 x_file，前端上传/列表刷新语义保持不变。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const read = (name: string) => readFileSync(resolve(import.meta.dirname, name), 'utf8')
const backend = read('../../../../../backend/crates/file_assemble_control/src/lib.rs')
const frontend = read('FileManager.vue')

describe('R314 D2 文件上传 415 修复契约', () => {
  it('后端 upload_file 以 Multipart 接收 multipart/form-data，并把真实字节写入存储与 x_file', () => {
    const handler = backend.slice(backend.indexOf('pub async fn upload_file'), backend.indexOf('pub async fn create_file'))
    expect(handler).toMatch(/\bmultipart: axum::extract::Multipart\b/)
    expect(handler).not.toContain('axum::extract::Json(body)')
    expect(handler).toContain('axum::extract::Multipart')
    // 字节读取复用同 crate 的 u2_read_multipart_file（其中 next_field/file_name/bytes）。
    expect(backend).toContain('async fn u2_read_multipart_file(')
    expect(backend).toMatch(/u2_read_multipart_file[\s\S]{0,400}next_field\(\)/)
    expect(backend).toMatch(/u2_read_multipart_file[\s\S]{0,400}file_name\(\)/)
    expect(backend).toMatch(/u2_read_multipart_file[\s\S]{0,500}bytes\(\)/)
    expect(handler).toContain('storage_with_pool')
    expect(handler).toContain('u2_persist_verified')
    expect(handler).toMatch(/INSERT INTO x_file \(id, name, path, size, creator, create_time, folder_id\)/)
    expect(handler).toContain('folder_id')
    // 写入的 folder_id 必须与前端根目录列表口径（loadFiles 的 folderId || 'root'）一致，否则上传后列表不可见。
    expect(handler).toMatch(/let folder_id = "root"\.to_string\(\)/)
    // 响应键与既有前端 upload 流程一致，不要求改前端
    expect(handler).toContain('uploaded')
    expect(handler).toContain('storageKey')
  })

  it('前端上传链路保持 multipart + uploadWithProgress + 上传后刷新列表', () => {
    expect(frontend).toContain("formData.append('file', file)")
    expect(frontend).toContain('.uploadWithProgress(\'/api/file/assemble/control/file/upload\', formData')
    expect(frontend).toContain('loadFiles(currentFolder.value)')
  })
})