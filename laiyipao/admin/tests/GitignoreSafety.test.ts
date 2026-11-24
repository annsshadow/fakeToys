// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
import { describe, it, expect } from "vitest"
import { readFileSync } from "node:fs"
import { resolve } from "node:path"

const REPO = resolve(__dirname, "../../..")
const gitignore = readFileSync(resolve(REPO, ".gitignore"), "utf-8")
const lines = gitignore.split(/\r?\n/).map((l) => l.trim())
const hasRule = (rule: string) => lines.includes(rule)

describe("根 .gitignore 防截断守卫", () => {
  it("文件不得从完整版本被截断为极短版本", () => {
    expect(lines.length, ".gitignore 行数低于 360，疑似被截断").toBeGreaterThanOrEqual(360)
  })
  it("通用 secrets 与构建产物忽略规则仍在", () => {
    for (const r of [".env", "*.local.keys", ".agents/", "node_modules/"]) {
      expect(hasRule(r), "缺少忽略规则 " + r).toBe(true)
    }
  })
  it("R15/R30/R31 审计临时真值忽略规则仍在", () => {
    const rules = [
      "docs/laiyipao/audits/round15-reaction-timeline/data/",
      "docs/laiyipao/audits/round15-reaction-timeline/auth-init.js",
      "docs/laiyipao/audits/round30-postdeploy/data/",
      "docs/laiyipao/audits/round30-postdeploy/auth-init.js",
      "docs/laiyipao/audits/round31-wiki-reaction-mechanics/data/",
      "docs/laiyipao/audits/round31-wiki-reaction-mechanics/admin-auth-init.js",
      "docs/laiyipao/audits/round31-postdeploy/data/",
      "docs/laiyipao/audits/round31-postdeploy/admin-auth-init.js",
    ]
    for (const r of rules) expect(hasRule(r), "缺少忽略规则 " + r).toBe(true)
  })
})
