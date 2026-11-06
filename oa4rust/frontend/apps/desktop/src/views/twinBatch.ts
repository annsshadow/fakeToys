// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { mutationErrorMessage } from './mutationError'

/**
 * R139：桌面端「孪生端点」自测按钮的批处理收口。
 *
 * 旧写法在 30 个 views 文件里各复制了一份：
 *   const s = <T>(p: Promise<T>) => p.catch(() => null)
 *   const rs = await Promise.all([s(api.put(...)), ...])
 *   toast.success(`孪生端点 ${rs.length} 条已提交`)
 * 三个问题：①失败被吞成 null；②`rs.length` 把失败也算进「已提交」；
 * ③全失败仍弹绿色成功。这里用 allSettled 给出真实成功/失败计数，全失败不得报「已提交」。
 */
export interface BatchOutcome {
  total: number
  okCount: number
  failedCount: number
  failedMessages: string[]
}

/** 统一跑批：不吞错，永远 resolve 成一个可判定的结果。 */
export async function runTwinBatch(requests: Array<Promise<unknown>>): Promise<BatchOutcome> {
  const results = await Promise.allSettled(requests)
  const failed = results.filter((r): r is PromiseRejectedResult => r.status === 'rejected')
  return {
    total: results.length,
    okCount: results.length - failed.length,
    failedCount: failed.length,
    failedMessages: failed.map((r) => mutationErrorMessage(r.reason)),
  }
}

export interface BatchSummary {
  tone: 'success' | 'warning' | 'error'
  message: string
}

/** 把结果翻译成用户可判断的一句话：成功多少、失败多少、为什么失败。 */
export function describeBatchOutcome(label: string, outcome: BatchOutcome): BatchSummary {
  if (outcome.total === 0) return { tone: 'error', message: `${label}：没有可提交的请求` }
  if (outcome.failedCount === 0) return { tone: 'success', message: `${label}：${outcome.okCount} 条已提交` }
  const detail = outcome.failedMessages[0] ? `（${outcome.failedMessages[0]}）` : ''
  if (outcome.okCount === 0) return { tone: 'error', message: `${label}：${outcome.failedCount} 条全部失败${detail}` }
  return { tone: 'warning', message: `${label}：成功 ${outcome.okCount} 条 / 失败 ${outcome.failedCount} 条${detail}` }
}
