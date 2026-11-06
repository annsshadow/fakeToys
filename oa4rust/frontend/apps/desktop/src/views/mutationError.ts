// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { ApiError, PermissionError } from '@oa4rust/sdk'

/**
 * R138：后端安全修复把一批写端点从「200 + 业务布尔」改成 403/404。
 * mutation 没有 onError 时，vue-query 会静默吞掉失败——用户只看到「点了没反应」，
 * 或更糟：onSuccess 里仍弹「已提交」的假成功。这里统一给出可读、可行动的失败文案。
 */
export const FORBIDDEN_MESSAGE = '没有权限执行该操作'
export const NOT_FOUND_MESSAGE = '目标不存在或已被删除'
export const GENERIC_MESSAGE = '操作失败，请重试'

export function mutationErrorMessage(err: unknown): string {
  if (err instanceof PermissionError) return FORBIDDEN_MESSAGE
  if (err instanceof ApiError && err.status === 404) return NOT_FOUND_MESSAGE
  return GENERIC_MESSAGE
}
