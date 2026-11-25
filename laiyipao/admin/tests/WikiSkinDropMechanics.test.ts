// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
import { describe, it, expect } from "vitest"
import { readFileSync } from "node:fs"
import { resolve } from "node:path"
// 真实导入实施模块：模块不存在时整个 suite RED（禁止内嵌参考实现）
import { MECHANISM_EMPTY, rarityText, passiveText, dropRatesText, buildSkinDropMechanicsAi, SKIN_DROP_AI_BASIS, SKIN_DROP_AI_HONESTY, SKIN_DROP_AI_CTA } from "../src/skinDropMechanicsText"
const REPO = resolve(__dirname, "../../..")
const read = (p: string) => readFileSync(resolve(REPO, p), "utf-8")
const spec = read("docs/laiyipao/specs/round39-wiki-skins-drop-rates/SPEC.md")
const wiki = read("laiyipao/admin/src/views/WikiView.vue")
const service = read("laiyipao/server/internal/service/service.go")
const content = read("laiyipao/server/internal/domain/content.go")
const battle = read("laiyipao/server/internal/domain/battle.go")
const generated = read("laiyipao/admin/src/api/types.gen.ts")
const stageTest = read("laiyipao/miniapp/src/test/stage_drop_preview_feature.test.ts")
const stage = read("laiyipao/miniapp/src/pages/stage/stage.vue")
const enemyHelper = read("laiyipao/admin/src/enemyMechanicsText.ts")
const tokens = read("laiyipao/admin/src/styles/tokens.css")
const dropKeys = ["coin_base", "coin_per_star", "coin_per_kill", "reaction_bonus", "keys_on_three_star", "energy_on_win"] as const
const dropValues = { coin_base: 200, coin_per_star: 150, coin_per_kill: 20, reaction_bonus: 30, keys_on_three_star: 1, energy_on_win: 5 }
const skins = [
  { id: 1, code: "skin_ranger", name: "游猎者", rarity: "rare", passive: "kinetic_bonus", descr: "动能系技能额外附带一次破甲击退。" },
  { id: 2, code: "skin_pyro", name: "炽炎使徒", rarity: "epic", passive: "fire_burn", descr: "焰系技能命中时，额外附加一层燃烧。" },
  { id: 3, code: "skin_cryo", name: "霜行者", rarity: "epic", passive: "ice_freeze", descr: "冰系技能有 20% 概率直接冻结目标。" },
  { id: 4, code: "skin_volt", name: "雷鸣者", rarity: "epic", passive: "volt_chain", descr: "电系技能弹射次数 +1。" },
  { id: 5, code: "skin_blight", name: "腐沼行者", rarity: "legendary", passive: "blight_spread", descr: "毒系技能命中时，向周围传播 1 层毒。" },
  { id: 6, code: "skin_chalk", name: "白垩女王", rarity: "legendary", passive: "boss_breaker", descr: "对 BOSS 的反应伤害提高 30%。" },
]
const skinStart = wiki.indexOf("r39-skin-mechanics")
const skinSection = skinStart >= 0 ? wiki.slice(skinStart, wiki.indexOf("</section>", skinStart)) : ""
const dropStart = wiki.indexOf("r39-drop-table")
const dropSection = dropStart >= 0 ? wiki.slice(dropStart, dropStart + 6000) : ""
const searchStart = wiki.indexOf("const searchIndex = computed")
const searchIndex = searchStart >= 0 ? wiki.slice(searchStart, wiki.indexOf("const doSearch", searchStart) > searchStart ? wiki.indexOf("const doSearch", searchStart) : searchStart + 6000) : ""
describe("R39 wiki skins and drop rates contract", () => {
  it("MECHANISM_EMPTY 复用 R33 同源同值", () => {
    expect(MECHANISM_EMPTY).toBe("—")
    expect(spec).toContain("MECHANISM_EMPTY 从 @/enemyMechanicsText 同源复用")
    expect(spec).toContain("禁止在本模块重新声明")
    expect(enemyHelper).toContain("export const MECHANISM_EMPTY = '—'")
  })
  it("权威 skins/drop_rates 结构、公开 config 与玩家端消费有据", () => {
    expect(service).toContain('json:"skins"')
    expect(service).toContain('json:"drop_rates"')
    expect(service).toContain("Skins:      domain.SeedSkins")
    expect(service).toContain("DropRates: domain.DefaultDropRates()")
    expect(wiki).toContain("api.get<ConfigResp>('/config', { auth: false })")
    expect(content).toContain("type SeedSkin struct")
    for (const f of ['json:"id"', 'json:"code"', 'json:"name"', 'json:"rarity"', 'json:"passive"', 'json:"descr"']) expect(content).toContain(f)
    expect(content).toContain("var SeedSkins = []SeedSkin{")
    expect(content).toContain("{ID: 1, Code: \"skin_ranger\", Name: \"游猎者\", Rarity: \"rare\",")
    expect(content).toContain("{ID: 6, Code: \"skin_chalk\", Name: \"白垩女王\", Rarity: \"legendary\",")
    expect(content).toContain("Passive: \"boss_breaker\"")
    expect(battle).toContain("type DropRates struct")
    for (const f of dropKeys) expect(battle).toContain('json:"' + f + '"')
    expect(battle).toContain("CoinBase: 200, CoinPerStar: 150, CoinPerKill: 20,")
    expect(battle).toContain("ReactionBonus: 30, KeysOnThreeStar: 1, EnergyOnWin: 5,")
    expect(generated).toContain("export interface SeedSkin")
    for (const f of ["id: number", "code: string", "name: string", "rarity: string", "passive: string", "descr: string"]) expect(generated).toContain(f)
    expect(generated).toContain("skins: SeedSkin[]")
    expect(generated).toContain("drop_rates: { coin_base: number; coin_per_star: number; coin_per_kill: number; reaction_bonus: number; keys_on_three_star: number; energy_on_win: number }")
    expect(stageTest).toContain("store.config.drop_rates")
    expect(stage).toContain("store.config?.drop_rates")
  })
  it("rarityText 三档映射、未知档原样、全部非法边界为 —", () => {
    expect(rarityText("rare")).toBe("稀有")
    expect(rarityText("epic")).toBe("史诗")
    expect(rarityText("legendary")).toBe("传说")
    expect(rarityText("mythic")).toBe("mythic")
    for (const v of ["", "   ", 1, 0, true, false, null, undefined, NaN, Infinity, -Infinity, {}, [], () => 1]) expect(rarityText(v as unknown)).toBe(MECHANISM_EMPTY)
  })
  it("passiveText 只展示机制型 code 原值，空与非法为 —", () => {
    expect(passiveText("kinetic_bonus")).toBe("kinetic_bonus")
    expect(passiveText("boss_breaker")).toBe("boss_breaker")
    for (const v of ["", "   ", 1, true, null, undefined, NaN, Infinity, -Infinity, {}, []]) expect(passiveText(v as unknown)).toBe(MECHANISM_EMPTY)
  })
  it("dropRatesText 六键整数配置基值直显，0 合法，脏值为 —", () => {
    for (const key of dropKeys) expect(dropRatesText(dropValues[key])).toBe(String(dropValues[key]))
    expect(dropRatesText(0)).toBe("0")
    for (const v of [null, undefined, 0.5, -1, NaN, Infinity, -Infinity, "200", true, {}, []]) expect(dropRatesText(v as unknown)).toBe(MECHANISM_EMPTY)
    expect(dropRatesText("")).toBe(MECHANISM_EMPTY)
  })
  it("buildSkinDropMechanicsAi 确定性聚合，无脏值残留", () => {
    const expected = "共 6 款皮肤：稀有 1、史诗 3、传说 2；掉落 6/6 项齐备"
    expect(buildSkinDropMechanicsAi(skins, dropValues)).toBe(expected)
    expect(buildSkinDropMechanicsAi(skins, dropValues)).toBe(buildSkinDropMechanicsAi(skins, dropValues))
    expect(buildSkinDropMechanicsAi([], {})).toBe("共 0 款皮肤：稀有 0、史诗 0、传说 0；掉落 0/6 项齐备")
    const partial = { coin_base: 200, coin_per_star: 0, coin_per_kill: null, reaction_bonus: 30, keys_on_three_star: 1.5, energy_on_win: NaN }
    expect(buildSkinDropMechanicsAi(skins, partial)).toBe("共 6 款皮肤：稀有 1、史诗 3、传说 2；掉落 3/6 项齐备")
    expect(buildSkinDropMechanicsAi(null as unknown as typeof skins, null)).toBe("共 0 款皮肤：稀有 0、史诗 0、传说 0；掉落 0/6 项齐备")
    for (const value of [buildSkinDropMechanicsAi(skins, dropValues), expected]) {
      for (const bad of ["undefined", "NaN", "Infinity", "{total}", "{rare}", "{epic}", "{legendary}", "{present}"]) expect(value).not.toContain(bad)
    }
  })
  it("逐字常量：依据、诚实红线、CTA 固定文案", () => {
    expect(SKIN_DROP_AI_BASIS).toBe("依据：公开 /api/v1/config 的 skins（与服务端 SeedSkin / SeedSkins 同源）与 drop_rates（与服务端 DropRates / DefaultDropRates 同源）；仅展示配置基值")
    expect(SKIN_DROP_AI_HONESTY).toBe("皮肤被动与掉落均为配置基值，实际结算与获得以游戏内为准；权威回放以 replay_hash 为准")
    expect(SKIN_DROP_AI_CTA).toBe("查看皮肤图鉴 →")
  })
  it("ConfigResp 补 skins 六字段与 drop_rates 六键，snake_case 防漂移", () => {
    expect(wiki).toContain("skins: Array<{")
    for (const f of ["id:", "code:", "name:", "rarity:", "passive:", "descr:"]) expect(wiki).toContain(f)
    expect(wiki).toContain("drop_rates: {")
    for (const f of dropKeys) expect(wiki).toContain(f + ": number")
    expect(wiki).not.toContain("keysOnThreeStar")
    expect(wiki).not.toContain("energyOnWin")
  })
  it("R39 皮肤表、掉落小表、四选择器与真实调用齐全", () => {
    for (const s of ["r39-skin-mechanics", "r39-skin-table-scroll", "r39-drop-table", "r39-ai-note"]) expect(wiki).toContain(s)
    const css = wiki.slice(wiki.indexOf(".r39-skin-table-scroll"))
    expect(css).toContain("overflow-x: auto")
    expect(css).toContain("max-width: 100%")
    for (const label of ["ID", "名称", "稀有度", "被动", "说明"]) expect(skinSection).toContain('label="' + label + '"')
    for (const label of ["规则", "数值"]) expect(dropSection).toContain('label="' + label + '"')
    for (const label of ["基础金币", "每星金币", "每次击杀金币", "反应加成", "三星钥匙", "通关返还体力"]) expect(dropSection).toContain(label)
    for (const call of ["rarityText(row.rarity)", "passiveText(row.passive)", "dropRatesText(cfg?.drop_rates?.coin_base)", "buildSkinDropMechanicsAi(cfg?.skins ?? [], cfg?.drop_rates ?? {})"]) expect(wiki).toContain(call)
    expect(wiki).toContain("@/skinDropMechanicsText")
    expect(wiki).toContain("key: 'skins'")
    expect(wiki).toContain("label: '皮肤图鉴'")
  })
  it("AI 四要素同屏、hash 安全 CTA、searchIndex 可搜皮肤", () => {
    const ai = skinSection.slice(skinSection.indexOf("r39-ai-note"))
    expect(ai).toContain("AI 机制概览")
    expect(ai).toContain(SKIN_DROP_AI_BASIS)
    expect(ai).toContain(SKIN_DROP_AI_HONESTY)
    expect(ai).toContain(SKIN_DROP_AI_CTA)
    expect(ai).toContain("urlFor('skins')")
    expect(ai).toContain("goTo({ tab: 'skins' })")
    expect(ai).not.toContain('href="#')
    for (const ph of ["{total}", "{rare}", "{epic}", "{legendary}", "{present}"]) expect(ai).not.toContain(ph)
    expect(searchIndex).toContain("skin")
    expect(searchIndex).toContain("皮肤")
    expect(searchIndex).toContain("c.skins ?? []")
    expect(searchIndex).toContain("section: 'skins'")
  })
  it("SPEC 钉死诚实红线、零改动面、三视口与令牌锚点", () => {
    for (const s of ["禁止内嵌参考实现", "禁改 engine/后端/replay/hash/score/settle/loot/tokens", "皮肤被动与掉落均为配置基值，实际结算与获得以游戏内为准", "配置基值", "replay_hash", "MinValue/MaxValue", "非独立真源", "r39-skin-mechanics", "r39-skin-table-scroll", "overflow-x:auto", "max-width:100%", "searchIndex", "禁止新增 --r39-* 令牌", "390×844", "RED"]) expect(spec).toContain(s)
    for (const anchor of ["--lyp-token-version: lyp-tokens-v1", "--lyp-r3-version: lyp-tokens-v3-r3", "--lyp-r6-version: lyp-tokens-r6"]) expect(tokens).toContain(anchor)
    expect(tokens).not.toContain("--r39-")
    expect(wiki).not.toContain("--r39-")
  })
})
