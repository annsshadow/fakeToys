// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
import { describe, it, expect } from "vitest"
import { readFileSync } from "node:fs"
import { resolve } from "node:path"
const Q = String.fromCharCode(34)
// 真实导入实施模块：模块尚不存在时整个 suite RED（禁止内嵌参考实现）
import { spreadStacksText, armorShredText, knockbackText, MECHANISM_EMPTY } from "../src/reactionMechanicsText"

const REPO = resolve(__dirname, "../../..")
const read = (p: string) => readFileSync(resolve(REPO, p), "utf-8")
const wiki = read("laiyipao/admin/src/views/WikiView.vue")
const genTypes = read("laiyipao/admin/src/api/types.gen.ts")
const goSpec = read("laiyipao/server/internal/domain/elements.go")
const epTest = read("laiyipao/server/internal/httpapi/reactions_endpoint_test.go")
const spec = read("docs/laiyipao/specs/round31-wiki-reaction-mechanics/SPEC.md")
const selfSrc = readFileSync(__filename, "utf-8")

function reactionTable(): string {
  const start = wiki.indexOf("cfg?.reactions")
  expect(start, "找不到绑定 cfg.reactions 的反应表").toBeGreaterThan(0)
  const end = wiki.indexOf("</el-table>", start)
  expect(end, "反应表未闭合").toBeGreaterThan(start)
  return wiki.slice(start, end)
}
function localReactionType(): string {
  const m = wiki.match(/reactions:\s*Array<\{([^}]*)\}>/)
  expect(m, "找不到 WikiView 本地 reactions 类型声明").not.toBeNull()
  return m![1]
}

describe("R31 SPEC 钉死服务端真源字段与显示单位", () => {
  it("SPEC 写清三字段 json tag 与真实值", () => {
    for (const k of ["spread_stacks", "armor_shred_permille", "knockback", "AoeRadius: 120, SpreadStacks: 1", "ArmorShredPermille: 150, Knockback: 4000", ":125", ":135", ":138"]) expect(spec).toContain(k)
  })
  it("SPEC 钉死真实单位与零值诚实空态、burn_cloud C1 阻塞", () => {
    for (const k of ["传播 N 层", "N‰", "值 / 1000", "4px", "2.5px", "MECHANISM_EMPTY", "—", "夸大 10 倍", "C1 产品阻塞", "不得暗示 burn_cloud 火区已实现"]) expect(spec).toContain(k)
  })
  it("Wiki 权威真源是公开 /config，/admin/reactions 仅同源旁证", () => {
    // WikiView 实际调用公开配置端点（:619），页面自述与游戏内数值同源（:673）
    expect(wiki).toContain("api.get<ConfigResp>('/config', { auth: false })")
    expect(wiki).toContain("/api/v1/config")
    // 服务端 service.go:1601-1609 LoadGameConfig + :1638 随全量配置下发三字段
    expect(spec).toContain("service.go:1601-1609 LoadGameConfig")
    expect(spec).toContain(":1638")
    expect(spec).toContain("Reactions: domain.AllReactionSpecs()")
    expect(spec).toContain("/api/v1/config")
    expect(spec).toContain("同源旁证")
    // 防回退：SPEC 不得把 /admin/reactions 说成 Wiki 真源或对账基准
    expect(spec).not.toContain("admin 从 /admin/reactions 读取真值")
    expect(spec).not.toContain("三列值与 /admin/reactions 的")
  })

  it("SPEC 钉死 RED→GREEN、三端不回退与零真值", () => {
    for (const k of ["Cannot find module", "ERR_MODULE_NOT_FOUND", "/api/v1/config", "service.go:1638", "同源旁证", "375", "390", "documentElement.scrollWidth", "lyp-tokens-v1", "lyp-tokens-v3-r3", "lyp-tokens-r6"]) expect(spec).toContain(k)
  })
})

describe("R31 显示辅助纯函数（真实导入，单位换算正确）", () => {
  it("MECHANISM_EMPTY 逐字为破折号", () => {
    expect(MECHANISM_EMPTY).toBe("—")
  })
  it("spreadStacksText：正整数显示层数，零/负数/小数/非数/缺失一律空态", () => {
    expect(spreadStacksText(1)).toBe("传播 1 层")
    expect(spreadStacksText(3)).toBe("传播 3 层")
    for (const v of [0, null, undefined, -1, Number.NaN, 1.5, Number.POSITIVE_INFINITY, Number.NEGATIVE_INFINITY]) expect(spreadStacksText(v)).toBe(MECHANISM_EMPTY)
  })
  it("armorShredText：千分比原值加千分号，不得当百分比拼百分号", () => {
    expect(armorShredText(150)).toBe("150‰")
    expect(armorShredText(50)).toBe("50‰")
    expect(armorShredText(150)).not.toContain("%")
    for (const v of [0, null, undefined, -1, Number.NaN]) expect(armorShredText(v)).toBe(MECHANISM_EMPTY)
  })
  it("knockbackText：定点 /1000 转像素", () => {
    expect(knockbackText(4000)).toBe("4px")
    expect(knockbackText(2500)).toBe("2.5px")
    for (const v of [0, null, undefined, -1, Number.NaN]) expect(knockbackText(v)).toBe(MECHANISM_EMPTY)
  })
})

describe("R31 WikiView.vue 源码契约：类型/列/文案/不回退", () => {
  it("本地 reactions 类型补齐三字段且 snake_case 与服务端一致", () => {
    const t = localReactionType()
    for (const k of ["spread_stacks", "armor_shred_permille", "knockback", "base_coef", "descr"]) expect(t).toContain(k)
    expect(t).not.toMatch(/spreadStacks|armorShredPermille/)
  })
  it("反应表新增三列并真实消费三个 helper", () => {
    const table = reactionTable()
    for (const label of ["层数传播", "削甲", "击退"]) expect(table).toContain("label=" + Q + label + Q)
    for (const call of ["spreadStacksText(row.spread_stacks)", "armorShredText(row.armor_shred_permille)", "knockbackText(row.knockback)"]) expect(table).toContain(call)
    expect(wiki).toContain("reactionMechanicsText")
  })
  it("陈旧没有特殊效果文案消失，层数传播如实标为已实现", () => {
    expect(wiki).not.toContain("仍**没有特殊效果**的只剩两条")
    expect(wiki).not.toContain("腐蚀扩散的层数传播）—— 具体缺什么见")
    expect(wiki).toContain("层数传播已实现")
    expect(wiki).toContain("burn_cloud")
  })
  it("既有 8 列、说明列、文案函数与空态原样保留", () => {
    const table = reactionTable()
    for (const label of ["反应", "系数", "攻击力权重", "控制效果", "受击增伤", "驱散护盾", "溅射半径", "说明"]) expect(table).toContain("label=" + Q + label + Q)
    expect(table).toContain("prop=" + Q + "descr" + Q)
    expect(table).toContain("statusText(row.key, row.status_duration_ms)")
    expect(table).toContain("amplifyPctText(row.amplify_pct)")
    expect(table).toContain("emptyText")
  })
  it("目录/深链/搜索/表格布局不回退", () => {
    for (const k of ["SECTIONS", "targetFromQuery", "scrollToAnchor", "scrollToFocus", "searchIndex", "searchMatches", "wiki-body", ":data-focus", "border", "size=" + Q + "small" + Q]) expect(wiki).toContain(k)
    expect(wiki).toMatch(/\{\s*id:\s*['"]reactions['"]\s*,\s*label:\s*['"]反应链一览['"]\s*\}/)
  })
})

describe("R31 服务端真源与三端不回退、零真值", () => {
  it("服务端 ReactionSpec / 轻量端点白名单 / 生成类型含三字段", () => {
    for (const k of ["json:" + Q + "spread_stacks" + Q, "json:" + Q + "armor_shred_permille" + Q, "json:" + Q + "knockback" + Q]) expect(goSpec).toContain(k)
    for (const k of ["spread_stacks", "armor_shred_permille", "knockback"]) { expect(epTest).toContain(k); expect(genTypes).toContain(k) }
  })
  it("tokens 三锚点保留且无 r31 变量", () => {
    const tokens = read("laiyipao/admin/src/styles/tokens.css")
    for (const k of ["--lyp-token-version: lyp-tokens-v1", "--lyp-r3-version: lyp-tokens-v3-r3", "--lyp-r6-version: lyp-tokens-r6"]) expect(tokens).toContain(k)
    expect(tokens).not.toContain("--r31-")
  })
  it("零真值：SPEC 与测试不含 JWT/IP/口令/本机绝对路径", () => {
    const both = spec + selfSrc.split("\n").filter(l => !l.includes("toMatch(")).join("\n")
    expect(both).not.toMatch(/eyJ[A-Za-z0-9_-]{20,}/)
    expect(both).not.toMatch(/\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b/)
    expect(both).not.toMatch(/postgres:\/\/[^\s]+:[^\s]+@/)
    const bs = String.fromCharCode(92); expect(both.includes(bs + 'Users' + bs)).toBe(false)
  })
})

describe("R31 审计脚本零真值：不得含本机绝对路径，统一 git rev-parse", () => {
  const audits = [
    "docs/laiyipao/audits/round31-wiki-reaction-mechanics/e2e-viewports.sh",
    "docs/laiyipao/audits/round31-wiki-reaction-mechanics/e2e-viewports-auth.sh",
    "docs/laiyipao/audits/round31-wiki-reaction-mechanics/mint-admin-token.sh",
  ]
  const body = (p: string) => read(p)

  it("三份审计脚本不含盘符或用户目录等本机绝对路径", () => {
    const forbidden = /[A-Za-z]:\/|\/Users\/|\/home\//
    const noUrl = (s: string) => s.replace(/https?:\/\//g, "")
    for (const p of audits) expect(forbidden.test(noUrl(body(p))), p + " 含本机绝对路径污染").toBe(false)
  })
  it("三份审计脚本统一用 git rev-parse 发现仓库根", () => {
    for (const p of audits) expect(body(p), p + " 未用 git rev-parse 发现仓库根").toContain("git rev-parse --show-toplevel")
  })
})