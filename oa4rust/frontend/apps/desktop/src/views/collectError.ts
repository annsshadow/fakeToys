// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { PermissionError } from '@oa4rust/sdk'

/** 403（非属主/无权限）与其它失败分开提示，避免「点了没反应」的静默失败。 */
export const COLLECT_FORBIDDEN_MESSAGE = '没有权限操作这条收藏'
export const COLLECT_GENERIC_ERROR_MESSAGE = '操作失败，请重试'

export function collectMutationErrorMessage(err: unknown): string {
  return err instanceof PermissionError ? COLLECT_FORBIDDEN_MESSAGE : COLLECT_GENERIC_ERROR_MESSAGE
}
