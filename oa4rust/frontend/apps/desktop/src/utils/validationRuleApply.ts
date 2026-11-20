// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * R227：把「验证规则构建器」里的 `type:param` 规则应用到字段校验对象。
 * 构建器下拉产出的是 `required:` / `min:1` / `max:100` / `pattern:...` 这类字符串，
 * 而字段校验使用的是结构化 FieldValidation；过去按钮没有处理函数，规则从未落地。
 */
export interface BuilderValidationRule {
  fieldKey: string
  rule: string
  message: string
}

export interface AppliedValidation {
  required?: boolean
  min?: number
  max?: number
  pattern?: string
  patternMsg?: string
}

/** 只按第一个冒号切分，避免正则里的冒号被误截断。 */
export function applyBuilderRules(
  base: AppliedValidation | undefined,
  rules: BuilderValidationRule[],
): AppliedValidation {
  const result: AppliedValidation = { ...(base ?? {}) }
  for (const { rule, message } of rules) {
    const sep = rule.indexOf(':')
    const kind = sep === -1 ? rule : rule.slice(0, sep)
    const param = sep === -1 ? '' : rule.slice(sep + 1)
    if (kind === 'required') result.required = true
    else if (kind === 'min' && Number.isFinite(Number(param))) result.min = Number(param)
    else if (kind === 'max' && Number.isFinite(Number(param))) result.max = Number(param)
    else if (kind === 'pattern' && param) {
      result.pattern = param
      if (message) result.patternMsg = message
    }
    // 未知 kind 静默忽略，不让一条坏规则阻断其余规则应用。
  }
  return result
}
