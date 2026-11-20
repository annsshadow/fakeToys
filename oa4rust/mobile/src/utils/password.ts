// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * 新密码本地预校验，门槛与后端 Service.ChangePassword 一致：
 * 去空白后非空、至少 10 位、且不能与原密码相同。
 * 校验失败返回面向用户的错误文案，通过返回 null。
 */
export function validateNewPassword(oldPassword: string, newPassword: string): string | null {
  const trimmedOld = oldPassword.trim()
  const trimmedNew = newPassword.trim()
  if (!trimmedOld || !trimmedNew) return '请填写完整密码'
  if (trimmedNew.length < 10) return '新密码至少 10 位'
  if (trimmedOld === trimmedNew) return '新密码不能与原密码相同'
  return null
}
