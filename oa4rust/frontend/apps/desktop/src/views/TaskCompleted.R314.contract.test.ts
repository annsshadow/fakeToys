// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! R314 D1：桌面「已办」页与后端已办写入闭环契约（源码静态断言，Node 环境）。
//! 后端 task_complete 必须在同一事务内补写 pp_c_taskcompleted；
//! 桌面完成提交成功后必须失效已办查询缓存，避免 UI 仍显示旧空列表。

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const read = (rel: string): string =>
  readFileSync(resolve(process.cwd(), rel), 'utf8')

const backendSource = read(
  'backend/crates/processplatform_service_processing/src/lib.rs',
)
const desktopSource = read(
  'frontend/apps/desktop/src/views/ProcessWork.vue',
)

describe('R314 D1 已办同步', () => {
  it('后端 task_complete 在事务内写 pp_c_taskcompleted', () => {
    expect(backendSource).toContain('INSERT INTO pp_c_taskcompleted')
    const completeBody = backendSource.slice(
      backendSource.indexOf('pub async fn task_complete('),
      backendSource.indexOf('pub async fn task_reject('),
    )
    expect(completeBody).toContain('INSERT INTO pp_c_taskcompleted')
    expect(completeBody).toContain('tx.execute(')
    expect(completeBody).toContain('tx.commit().await?')
  })

  it('桌面提交审批后失效已办查询缓存', () => {
    // 已办端点把 page 当 0 基 OFFSET：page=1 跳过第一条记录，故第一页必须请求 0。
    expect(desktopSource).toContain(
      '/api/processplatform/assemble/surface/taskcompleted/list/my/paging/0/size/20',
    )
    expect(desktopSource).toContain("['process-work', activeTab]")
    const submitBody = desktopSource.slice(desktopSource.indexOf('async function submit('))
    expect(submitBody).toContain('queryClient.invalidateQueries')
    expect(submitBody).toContain("['process-work']")
  })
});
