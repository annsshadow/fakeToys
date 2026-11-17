// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * 表单字段值校验的边界统一实现（R168）。
 *
 * 历史上 FormDesigner 的预览提交（validatePreview）与设计态校验（validateField）
 * 各写一套 falsy 判断，共用三类真实缺陷：
 * - `!value` 把 0 / false / '0' 这些已填写的合法值误判为「未填必填项」；
 * - 范围校验同样挂在 `value &&` 上，0 直接绕过 min（滑块/评分为 0 时最小值为 1 永不报警）；
 * - 非数字（NaN / 'abc'）经 Number() 变 NaN 后比较恒 false，静默放行；
 * - 非法正则 `new RegExp('(')` 直接抛异常，中断整表校验并弹到用户面前。
 *
 * 本模块把「空值」与「有效值」分开判定，返回首条错误字符串或 null（通过），
 * 供预览提交与设计态校验共用，保证两条路径口径一致。
 */

export interface FieldValidationLike {
  label?: string
  key?: string
  required?: boolean
  min?: number
  max?: number
  minLength?: number
  maxLength?: number
  pattern?: string
  patternMsg?: string
}

/** 只有 undefined / null / 空白串 / 空数组算「未填」；0 / false / '0' 均为有效值。 */
function isBlank(value: unknown): boolean {
  if (value === undefined || value === null) return true
  if (typeof value === 'string') return value.trim() === ''
  if (Array.isArray(value)) return value.length === 0
  return false
}

function isNumeric(value: unknown): boolean {
  if (typeof value === 'number') return Number.isFinite(value)
  if (typeof value === 'string') return value.trim() !== '' && Number.isFinite(Number(value))
  return false
}

/**
 * @returns 首条校验错误文本；通过时返回 null。
 */
export function validateFieldValue(field: FieldValidationLike, value: unknown): string | null {
  const name = field.label || field.key || ''
  if (field.required && isBlank(value)) return `${name} 不能为空`
  // 缺失值不再被 min/max/pattern/length 规则误报；必填缺失已由上一行拦截。
  if (isBlank(value)) return null

  if (field.min !== undefined || field.max !== undefined) {
    if (!isNumeric(value)) return `${name} 不是有效数字`
    const n = Number(value)
    if (field.min !== undefined && n < field.min) return `${name} 不能小于${field.min}`
    if (field.max !== undefined && n > field.max) return `${name} 不能大于${field.max}`
  }

  if (typeof value === 'string') {
    if (field.minLength !== undefined && value.length < field.minLength) return `${name} 至少${field.minLength}字符`
    if (field.maxLength !== undefined && value.length > field.maxLength) return `${name} 最多${field.maxLength}字符`
  }

  if (field.pattern) {
    try {
      if (!new RegExp(field.pattern).test(String(value))) return field.patternMsg || `${name} 格式错误`
    } catch {
      return `${name} 校验表达式无效`
    }
  }
  return null
}
