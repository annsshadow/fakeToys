// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// 守卫的守卫：验证 coverage-gate.mjs 该绿绿、该红红。
//
// 为什么必须有这个文件：一个判据写反、或者永远返回 pass 的闸门，比没有闸门
// 更糟 —— 它给出「已保护」的假象。本仓在 miniapp 的 vue-tsc 上吃过一次
// 「安静地不检查」的亏（见 laiyipao-ci.yml 的 type-check 探针注释），所以
// 覆盖率门禁也要做变异测试：三条反漂移守卫各有一个「必须红」的用例。
//
// 跑法：node scripts/coverage-gate.test.mjs（CI 里紧跟在门禁之后执行）

import { parseProfile, evaluate } from "./coverage-gate.mjs";

const MOD = "github.com/laiyipao/server";

let bad = 0;
function check(name, res, wantOk) {
  const ok = res.ok === wantOk;
  if (!ok) bad++;
  console.log(`  ${ok ? "PASS" : "FAIL"}  ${name}`);
  const detail = wantOk ? "" : ` → ${res.failures.join(" | ")}`;
  console.log(
    `        ok=${res.ok} 期望=${wantOk} 合计=${res.pct.toFixed(2)}%（${res.covered}/${res.total}）${detail}`,
  );
}

/** 断言布尔条件（用于不经过 evaluate 的细粒度检查）。 */
function checkCond(name, cond, detail) {
  if (!cond) bad++;
  console.log(`  ${cond ? "PASS" : "FAIL"}  ${name}`);
  if (!cond && detail) console.log(`        ${detail}`);
}

/** 造一行 profile：<pkg>/<file>:1.1,2.2 <stmts> <count> */
function pl(pkg, file, stmts, count) {
  return `${MOD}/${pkg}/${file}:1.1,2.2 ${stmts} ${count}`;
}

function profile(...lines) {
  return "mode: atomic\n" + lines.join("\n") + "\n";
}

function scope(...rows) {
  return "# 合成清单（测试用）\n" + rows.join("\n") + "\n";
}

// ── A 基线：无排除、全绿 ───────────────────────────────────────────────
console.log("=== A 基线 ===");
check(
  "A 两个包各 10 条语句全覆盖 → 放行",
  evaluate(profile(pl("a", "a.go", 10, 3), pl("b", "b.go", 10, 1)), scope()),
  true,
);

// ── B 守卫 1：零覆盖包不在清单里，必须红 ───────────────────────────────
console.log("=== B 守卫1：零覆盖包必须进清单 ===");
check(
  "B 未列出的 cmd/tool 0 覆盖（20 条语句）→ 拦下并点名",
  evaluate(
    profile(pl("internal/svc", "svc.go", 20, 5), pl("cmd/tool", "main.go", 20, 0)),
    scope(),
  ),
  false,
);

// 反例：进清单且带理由 → 放行，且**不计入分母**（分母只剩 internal/svc 的 20 条）。
{
  const res = evaluate(
    profile(pl("internal/svc", "svc.go", 20, 5), pl("cmd/tool", "main.go", 20, 0)),
    scope("cmd/tool  # 无测试的运维工具，CI 不执行它"),
  );
  check("B2 同一个零覆盖包进清单（带理由）→ 放行", res, true);
  checkCond(
    "B3 排除包不计入分母（20/20 = 100%）",
    res.total === 20 && res.covered === 20,
    `实际 ${res.covered}/${res.total}`,
  );
}

// ── C 守卫 2：清单里的包一旦有覆盖，必须红 ─────────────────────────────
console.log("=== C 守卫2：清单陈旧必须红 ===");
check(
  "C 清单里的包实际 50% 覆盖 → 拦下并提示从清单删除",
  evaluate(
    profile(pl("internal/svc", "svc.go", 20, 5), pl("cmd/tool", "main.go", 4, 2)),
    scope("cmd/tool  # 曾是零覆盖工具"),
  ),
  false,
);

// ── D 守卫 3：清单里的包已不存在，必须红 ───────────────────────────────
console.log("=== D 守卫3：清单指向不存在的包必须红 ===");
check(
  "D 清单里有 cmd/ghost，profile 里没有 → 拦下",
  evaluate(profile(pl("internal/svc", "svc.go", 20, 20)), scope("cmd/ghost  # 已删除的包")),
  false,
);

// ── E 清单本身的格式守卫 ──────────────────────────────────────────────
console.log("=== E 清单格式 ===");
check(
  "E1 排除行没有理由 → 拦下",
  evaluate(profile(pl("cmd/tool", "main.go", 5, 0)), scope("cmd/tool")),
  false,
);
check(
  "E2 同一个包出现两次 → 拦下",
  evaluate(profile(pl("cmd/tool", "main.go", 5, 0)), scope("cmd/tool  # 理由一", "cmd/tool  # 理由二")),
  false,
);

// ── F 阈值边界（整数比较，84.99 不许四舍五入成 85） ───────────────────
console.log("=== F 阈值 85% ===");
check(
  "F1 恰好 85.0%（170/200）→ 放行",
  evaluate(profile(pl("internal/svc", "a.go", 170, 2), pl("internal/svc", "b.go", 30, 0)), scope()),
  true,
);
check(
  "F2 84.5%（169/200）→ 拦下",
  evaluate(profile(pl("internal/svc", "a.go", 169, 2), pl("internal/svc", "b.go", 31, 0)), scope()),
  false,
);

// ── G 异常输入 ────────────────────────────────────────────────────────
console.log("=== G 异常输入 ===");
check("G1 空 profile → 拦下（门禁拿不到数据）", evaluate("mode: atomic\n", scope()), false);
{
  const crlf = "mode: atomic\r\n" + pl("internal/svc", "svc.go", 20, 20) + "\r\n";
  const res = evaluate(crlf, scope());
  check("G2 CRLF 换行（Windows 生成的 profile）→ 仍能解析并放行", res, true);
}

// parseProfile 的聚合口径：count>0 才计入已覆盖语句数。
{
  const m = parseProfile(profile(pl("x", "a.go", 3, 1), pl("x", "a.go", 2, 0), pl("x", "b.go", 5, 0)));
  const s = m.get("x");
  checkCond(
    "G3 同包多文件聚合：covered=3 / total=10",
    s.covered === 3 && s.total === 10,
    `实际 covered=${s.covered} total=${s.total}`,
  );
}

console.log(bad === 0 ? "\n全部用例通过" : `\n${bad} 个用例失败`);
process.exit(bad === 0 ? 0 : 1);
