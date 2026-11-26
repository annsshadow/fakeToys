import { describe, it, expect } from "vitest"
import { readFileSync } from "node:fs"
import { join } from "node:path"

const specPath = join(__dirname, "../../../../../docs/laiyipao/specs/round30-reaction-feed-mobile-fit/SPEC.md")
const srcRoot = join(__dirname, "../../..")
const readSrc = (p: string) => readFileSync(join(srcRoot, p), "utf-8")
const spec = readFileSync(specPath, "utf-8")
const battleSrc = readSrc("src/pages/battle/battle.vue")
const tokensSrc = readSrc("src/styles/tokens.css")

// 只取移动端媒体查询块；修复属于 390×844 窄屏规则，不得改桌面基线
function mobileBlock(src: string): string {
  const start = src.indexOf("@media (max-width: 430px)")
  if (start < 0) return ""
  let depth = 0, end = -1
  for (let i = start; i < src.length; i++) {
    if (src[i] === "{") depth++
    else if (src[i] === "}") { depth--; if (depth === 0) { end = i; break } }
  }
  return end > start ? src.slice(start, end + 1) : ""
}
// 提取选择器完整 CSS 规则体（含选择器文本），避免跨规则误判
function ruleOf(block: string, selector: string): string {
  const re = new RegExp("(^|\\})\\s*" + selector.replace(/[.*+?^${}()|[\]\\]/g, "\\$&") + "\\s*\\{([^}]*)\\}", "m")
  const m = block.match(re)
  return m ? m[0] : ""
}
const mobile = mobileBlock(battleSrc)
const line = ruleOf(mobile, ".reaction-line")
const main = ruleOf(mobile, ".reaction-line-main")
const attr = ruleOf(mobile, ".r28-attribution")
const cue = ruleOf(mobile, ".reaction-cue")
// 桌面规则在移动端媒体查询之后；用 lastIndexOf 取桌面 .reaction-line，避免首匹配命中移动端
const desktopLine = battleSrc.slice(battleSrc.lastIndexOf(".reaction-line {"), battleSrc.indexOf(".reaction-name {"))
// 逗号并列选择器也能定位（移动端现有 .reaction-name, .reaction-damage 共用规则）
function cssOf(block: string, selector: string): string {
  const out: string[] = []
  let i = 0
  while (i < block.length) {
    const open = block.indexOf("{", i)
    if (open < 0) break
    const close = block.indexOf("}", open + 1)
    if (close < 0) break
    const head = block.slice(i, open).trim()
    if (head.split(",").map(s => s.trim()).includes(selector)) out.push(block.slice(i, close + 1))
    i = close + 1
  }
  return out.join("\n")
}
const name = cssOf(mobile, ".reaction-name")
const damage = cssOf(mobile, ".reaction-damage")

describe("R30 SPEC 钉死窄屏盒模型根因与几何判据", () => {
  it("SPEC 记录 6px 溢出根因与既有 width:100% + padding 无 border-box", () => {
    for (const k of ["390×844", "right=396", "scrollWidth=386", "clientWidth=369", "width:100%", "box-sizing:border-box", "@media (max-width: 430px)"]) expect(spec).toContain(k)
  })
  it("SPEC 钉死修复属性、QA 几何与 RED→GREEN 命令", () => {
    for (const k of ["box-sizing: border-box", "max-width: 100%", "min-width: 0", "white-space: normal", "桌面/全局", "长英文/无空格", "right <= 390", "scrollWidth <= .reaction-feed.clientWidth + 1", "RED→GREEN", "npx vitest run --config vitest.config.ts"]) expect(spec).toContain(k)
  })
  it("SPEC 禁止隐藏/截断与逻辑/令牌回退", () => {
    for (const k of ["overflow:hidden", "text-overflow:ellipsis", "display:none", "replay_hash", "不得加 `--r30-`"]) expect(spec).toContain(k)
  })
})

describe("R30 battle.vue 移动端盒模型修复契约（当前源码应 RED）", () => {
  it("mobile 媒体查询与 .reaction-line 规则可定位", () => {
    expect(mobile).not.toBe("")
    expect(line).not.toBe("")
  })
  it(".reaction-line 三条修复属性齐备且保留既有移动规则", () => {
    expect(line).toMatch(/box-sizing\s*:\s*border-box/)
    expect(line).toMatch(/max-width\s*:\s*100%/)
    expect(line).toMatch(/min-width\s*:\s*0/)
    expect(line).toMatch(/width\s*:\s*100%/)
    expect(line).toMatch(/flex-wrap\s*:\s*wrap/)
    expect(line).toMatch(/padding\s*:\s*4rpx 10rpx/)
  })
  it("长文本子项约束齐备：main/归因/cue", () => {
    expect(main).toMatch(/min-width\s*:\s*0/)
    expect(main).toMatch(/flex-wrap\s*:\s*wrap/)
    expect(attr).toMatch(/white-space\s*:\s*normal/)
    expect(attr).toMatch(/max-width\s*:\s*100%/)
    expect(attr).toMatch(/overflow-wrap\s*:\s*anywhere/)
    expect(cue).toMatch(/overflow-wrap\s*:\s*anywhere/)
  })
  it("移动端 .r28-attribution 仅一条 white-space 声明且为 normal（无重复 nowrap hack）", () => {
    const declarations = attr.match(/white-space\s*:\s*[a-z]+/g) ?? []
    expect(declarations).toHaveLength(1)
    expect(declarations[0]).toMatch(/white-space\s*:\s*normal/)
    expect(attr).not.toMatch(/white-space\s*:\s*nowrap/)
  })
  it("长英文/无空格反应名与伤害数字串受约束（移动端 name/damage）", () => {
    for (const src of [name, damage]) {
      expect(src).not.toBe("")
      expect(src).toMatch(/min-width\s*:\s*0/)
      expect(src).toMatch(/overflow-wrap\s*:\s*anywhere/)
    }
  })
  it("禁止用隐藏/截断掩盖溢出", () => {
    for (const src of [line, main, attr, cue, name, damage]) {
      expect(src).not.toMatch(/overflow\s*:\s*hidden/)
      expect(src).not.toMatch(/text-overflow\s*:\s*ellipsis/)
      expect(src).not.toMatch(/display\s*:\s*none/)
    }
  })
  it("桌面基线规则未被改写", () => {
    expect(desktopLine).toMatch(/padding\s*:\s*6rpx 14rpx/)
    expect(desktopLine).not.toMatch(/box-sizing/)
    // 桌面/全局 nowrap 保留；移动端才用 normal 覆盖，二者不矛盾
    expect(battleSrc).toMatch(/\.r28-attribution\s*\{[^}]*white-space\s*:\s*nowrap/)
  })
  it("模板 R9/R28/R29 结构原样保留，CTA 仍绑定 openReactionTimeline", () => {
    for (const k of ["class=\"reaction-feed\"", "class=\"reaction-line\"", "class=\"reaction-line-main\"", "class=\"r28-attribution\"", "class=\"r28-feed-cta\"", "@click.stop=\"openReactionTimeline\"", "class=\"r29-timeline-overlay\""]) expect(battleSrc).toContain(k)
  })
})

describe("R30 三端不回退与零真值", () => {
  it("R24/R26/R27/R28/R29 选择器与 CTA 文案保留", () => {
    for (const k of ["r24-element-hud", "r26-wave-brief", "r27-wave-progress", "r28-attribution", "r28-feed-ai", "r28-feed-cta", "r29-timeline-overlay", "查看完整反应时间线 →"]) expect(battleSrc).toContain(k)
  })
  it("R28 归因说明块 pointer-events 与 CSS 红线保留", () => {
    expect(battleSrc).toMatch(/\.r28-feed-ai\s*\{[^}]*pointer-events\s*:\s*none/)
    expect(battleSrc).toMatch(/\.r28-feed-cta\s*\{[^}]*pointer-events\s*:\s*auto/)
    expect(battleSrc).toContain("辅助归因非权威判据，权威以 replay_hash 为准")
  })
  it("tokens 三锚点保留且无 --r30-", () => {
    expect(tokensSrc).toContain("--lyp-token-version: lyp-tokens-v1")
    expect(tokensSrc).toContain("--lyp-r3-version: lyp-tokens-v3-r3")
    expect(tokensSrc).toContain("--lyp-r6-version: lyp-tokens-r6")
    expect(tokensSrc).not.toContain("--r30-")
  })
  it("零真值：SPEC 与测试不含 JWT/IP/口令/本机绝对路径", () => {
    const both = spec + readFileSync(__filename, "utf-8")
    expect(both).not.toMatch(/eyJ[A-Za-z0-9_-]{8,}\.[A-Za-z0-9_-]{8,}\.[A-Za-z0-9_-]{8,}/)
    expect(both).not.toMatch(/\b(?:\d{1,3}\.){3}\d{1,3}\b/)
    expect(both).not.toMatch(/postgres:\/\/[^ ]+:[^ ]+@/)
    expect(both).not.toMatch(/C:\\Users\\/)
  })
})