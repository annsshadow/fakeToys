// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * t13 R2 首屏三页结构性视觉升级契约。
 * 依据：docs/laiyipao/reviews/ROUND1_DESIGN_REVIEW.md F-10-1/F-10-2，
 * docs/laiyipao/design/AI_VISUAL_SYSTEM.md §1A（P8 可见 AI 感）。
 * 证据：docs/laiyipao/audits/round2-t13/screenshots/before-*.png（非物理真机，390×844）。
 */
import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const dir = dirname(fileURLToPath(import.meta.url));
const pagesDir = join(dir, "..", "pages");
const home = readFileSync(join(pagesDir, "index", "index.vue"), "utf8");
const stage = readFileSync(join(pagesDir, "stage", "stage.vue"), "utf8");
const battle = readFileSync(join(pagesDir, "battle", "battle.vue"), "utf8");

function block(src: string, selector: string): string {
  return src.match(new RegExp(selector.replace(/[.*+?^${}()|[\]\\]/g, "\\$&") + "\\s*\\{([^}]*)\\}"))?.[1] ?? "";
}

describe("t13 首屏三页结构性视觉升级", () => {
  it("S1 AI 洞察卡位于资源顶栏之后、公告之前且标识可辨", () => {
    const topbar = home.indexOf('class="topbar"');
    const ai = home.indexOf('id="ai-insight"');
    const announce = home.indexOf("全部 {{ announcements.length }} 条");
    expect(topbar).toBeGreaterThan(-1);
    expect(ai).toBeGreaterThan(topbar);
    expect(ai).toBeLessThan(announce);
    expect(home).toContain('class="card ai-insight-card"');
    expect(home).toContain('class="ai-badge">AI</text>');
    expect(home).toContain("AI 战术洞察");
    expect(home).toContain("规则引擎 · 基于当前构筑");
  });

  it("S2 结论与空态均来自现有 store 计算，数值不写死", () => {
    expect(home).toContain("{{ aiInsight.weak.label }}仅 {{ aiInsight.weak.pct }}%（权重 {{ aiInsight.weak.weightPct }}%）：优先补它，收益 {{ aiInsight.gain }}");
    expect(home).toContain("依据：五维评分 / 补分边际收益 · 更新于本次进入");
    expect(home).toContain("去补强 →");
    expect(home).toContain("go(aiInsight.hint)");
    expect(home).toContain("AI 暂时没有足够构筑数据，先打一场战斗吧");
    expect(home).toContain('v-for="e in elements"');
    const script = home.slice(home.indexOf("<script"), home.indexOf("</script>"));
    expect(script).toMatch(/const aiInsight = computed/);
    expect(script).toMatch(/marginalGain.value/);
    expect(script).toMatch(/dims.value.find/);
  });

  it("S3 AI 卡令牌、尺寸与触控区满足 §1A.6/§1A.7", () => {
    const card = block(home, "#ai-insight.ai-insight-card");
    expect(card).toMatch(/min-height:\s*180rpx/);
    expect(card).toMatch(/var\(--ai-accent\)/);
    expect(card).toMatch(/var\(--ai-glow\)/);
    expect(card).toMatch(/var\(--panel-2\)/);
    expect(block(home, ".ai-badge")).toMatch(/var\(--ai-ink\)/);
    expect(block(home, ".ai-cta")).toMatch(/min-height:\s*88rpx/);
    expect(home).toMatch(/@media\s*\(prefers-reduced-motion:\s*reduce\)/);
  });

  it("S4 选关页新增可感知的作战简报 hero 与五元素关卡分色", () => {
    expect(stage).toContain('class="stage-hero"');
    expect(stage).toContain("作战简报");
    expect(stage).toContain("{{ activeLevelId }}");
    expect(stage).toContain("chapterStars");
    expect(stage).toContain("enter(activeLevelId)");
    expect(block(stage, ".stage-hero")).toMatch(/linear-gradient/);
    expect(block(stage, ".stage-hero-meter-fill")).toMatch(/linear-gradient/);
    expect(stage).toMatch(/\.grid \.level-cell:nth-child\(5n\+1\)/);
    expect(stage).toMatch(/@media\s*\(prefers-reduced-motion:\s*reduce\)/);
  });

  it("S5 战斗页增加有文字标签的五元素连锁 HUD 与升级血条", () => {
    expect(battle).toContain('class="battle-sigil-rail"');
    for (const s of ["sig-fire", "sig-ice", "sig-lightning", "sig-corrosion", "sig-kinetic"])
      expect(battle, s).toContain(s);
    expect(battle).toContain("元素连锁 · 命中挂层触发反应");
    expect(block(battle, ".battle-sigil-rail")).toMatch(/var\(--glass-bg\)/);
    expect(block(battle, ".battle-sigil-rail")).toMatch(/var\(--glass-blur\)/);
    expect(block(battle, ".hp-fill")).toMatch(/linear-gradient/);
  });
});
