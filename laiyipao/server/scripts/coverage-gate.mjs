// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// laiyipao 服务端覆盖率门禁。
//
// 用法：
//   node scripts/coverage-gate.mjs coverage.out
//
// 口径（哪些包计入）来自同目录的 coverage-scope.list，不来自 workflow 的 flag。
// 阈值 85 固定在本文件里；改口径与改阈值是两件事，不允许混在一次改动里。
//
// # 为什么门禁要按包看，而不只看总数
//
// 只看总数时，一个包从 85% 掉到 60% 会被另一个包的上涨掩盖；而「0 覆盖的包
// 混进分母」这种结构性问题，在总数上只表现为「莫名其妙永远差几个点」。
// 三条反漂移守卫（见 checkScope）就是为了让口径本身也受门禁约束：
//
//   - 零覆盖包必须显式进清单 —— 否则「新加一个没有测试的 cmd/」会静默拉低分母；
//   - 清单里的包一旦有覆盖必须移除 —— 否则清单会永久陈旧、排除面越缩越大；
//   - 清单里的包必须仍然存在 —— 否则改名/删除后清单指向空气，排除形同虚设。
//
// 这三条都是「安静地失效」的形状，所以本文件配套 coverage-gate.test.mjs
// 做变异测试：该红的必须红（对齐 laiyipao/scripts/audit-gate.cjs 的做法）。

import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

/** 总覆盖率阈值（%）。只许升，不许降；降阈值要单独一轮并写清理由。 */
export const THRESHOLD = 85;

/** Go module 路径；profile 里的文件名都是这个前缀的 import path。 */
const MODULE_PREFIX = "github.com/laiyipao/server";

/**
 * 解析排除清单。
 *
 * 每行格式：`<包目录>  # <理由>`；`#` 之后是理由，整行以 `#` 开头是注释。
 * 理由为空视为错误 —— 「为了到线」不是可以静默剔除的理由。
 */
export function parseScope(text) {
  const entries = [];
  const errors = [];
  const seen = new Map();

  text.split(/\r?\n/).forEach((raw, i) => {
    const line = raw.trim();
    if (!line || line.startsWith("#")) return;

    const hash = line.indexOf("#");
    const pkg = normalizePkg(hash === -1 ? line : line.slice(0, hash));
    const reason = hash === -1 ? "" : line.slice(hash + 1).trim();

    if (!pkg) {
      errors.push(`coverage-scope.list 第 ${i + 1} 行：缺少包路径`);
      return;
    }
    if (!reason) {
      errors.push(
        `coverage-scope.list 第 ${i + 1} 行（${pkg}）：缺少理由 —— ` +
          "排除某段代码必须逐条说明「为什么不可测」，不是「为了到线」",
      );
      return;
    }
    if (seen.has(pkg)) {
      errors.push(
        `coverage-scope.list 第 ${i + 1} 行（${pkg}）：重复项（首次出现在第 ${seen.get(pkg)} 行）`,
      );
      return;
    }
    seen.set(pkg, i + 1);
    entries.push({ pkg, reason, line: i + 1 });
  });

  return { entries, errors };
}

/** 统一包路径写法：去 ./ 前缀、反斜杠转斜杠、去尾部斜杠。 */
function normalizePkg(p) {
  return p.trim().replace(/\\/g, "/").replace(/^\.\//, "").replace(/\/+$/, "");
}

/**
 * 解析 `go test -coverprofile` 产物，按包聚合。
 *
 * profile 每行：`<file>:<l>.<c>,<l>.<c> <numStmt> <count>`；
 * 语句总数 = Σ numStmt，已覆盖 = Σ(numStmt where count > 0)。
 * 这与 `go tool cover -func` 的 total 口径一致。
 */
export function parseProfile(text) {
  const pkgs = new Map();

  for (const raw of text.split(/\r?\n/)) {
    const line = raw.trim();
    if (!line || line.startsWith("mode:")) continue;

    // 从右往左切两个空格：count、numStmt、剩下是位置。
    const i1 = line.lastIndexOf(" ");
    const i2 = i1 < 0 ? -1 : line.lastIndexOf(" ", i1 - 1);
    if (i1 < 0 || i2 < 0) continue;

    const count = Number(line.slice(i1 + 1));
    const stmts = Number(line.slice(i2 + 1, i1));
    if (!Number.isFinite(count) || !Number.isFinite(stmts)) continue;

    const pkg = pkgOfFile(line.slice(0, line.lastIndexOf(":", i2)));
    if (!pkg) continue; // 非本模块文件（正常情况不会出现）

    const cur = pkgs.get(pkg) || { covered: 0, total: 0 };
    cur.total += stmts;
    if (count > 0) cur.covered += stmts;
    pkgs.set(pkg, cur);
  }

  return pkgs;
}

/** `github.com/laiyipao/server/internal/service/foo.go` → `internal/service`。 */
function pkgOfFile(file) {
  if (!file.startsWith(MODULE_PREFIX + "/")) return null;
  const rel = file.slice(MODULE_PREFIX.length + 1);
  const slash = rel.lastIndexOf("/");
  return slash === -1 ? "." : rel.slice(0, slash);
}

function pct(covered, total) {
  return total === 0 ? 0 : (covered / total) * 100;
}

/**
 * 核心判据。返回 { ok, failures, notices, covered, total, pct, rows, entries, threshold }。
 *
 * 纯函数：不读文件，便于变异测试直接喂合成 profile / 合成清单。
 */
export function evaluate(profileText, scopeText, opts = {}) {
  const threshold = opts.threshold ?? THRESHOLD;
  const { entries, errors } = parseScope(scopeText);
  const pkgs = parseProfile(profileText);

  const failures = [...errors];
  const notices = [];
  const excluded = new Map(entries.map((e) => [e.pkg, e]));

  // 守卫 3：清单里的包必须仍然存在（改名/删除后清单会指向空气）。
  for (const e of entries) {
    if (!pkgs.has(e.pkg)) {
      failures.push(
        `清单里的 ${e.pkg} 在 coverage profile 中不存在 —— ` +
          `该包可能已删除或改名，请同步 coverage-scope.list（第 ${e.line} 行）`,
      );
    }
  }

  let covered = 0;
  let total = 0;
  const rows = [];

  for (const [pkg, s] of pkgs) {
    const p = pct(s.covered, s.total);
    rows.push({ pkg, covered: s.covered, total: s.total, pct: p, excluded: excluded.has(pkg) });

    if (excluded.has(pkg)) {
      // 守卫 2：清单陈旧 —— 有人开始测它了，该从清单删掉。
      if (s.covered > 0) {
        failures.push(
          `清单里的 ${pkg} 实际覆盖率 ${p.toFixed(1)}%（${s.covered}/${s.total}）> 0 —— ` +
            "说明它已经有测试了，请从 coverage-scope.list 删除该行",
        );
      }
      continue;
    }

    // 守卫 1：零覆盖包必须显式进清单。
    if (s.covered === 0) {
      failures.push(
        `${pkg} 覆盖率为 0.0%（${s.total} 条语句）且不在 coverage-scope.list 里 —— ` +
          "新包必须写测试；确实不可测的，写明理由再进清单",
      );
    }

    covered += s.covered;
    total += s.total;
  }

  if (pkgs.size === 0) {
    failures.push("coverage profile 为空或格式无法识别 —— 门禁拿不到任何数据");
  }

  // 用整数比较，避免浮点尾差让 84.9999 被四舍五入成 85.00 放行。
  if (total > 0 && covered * 100 < threshold * total) {
    failures.push(
      `总覆盖率 ${pct(covered, total).toFixed(2)}%（${covered}/${total}）< ${threshold}%`,
    );
  }

  rows.sort((a, b) => a.pct - b.pct || a.pkg.localeCompare(b.pkg));

  return {
    ok: failures.length === 0,
    failures,
    notices,
    covered,
    total,
    pct: pct(covered, total),
    rows,
    entries,
    threshold,
  };
}

function main() {
  const here = path.dirname(fileURLToPath(import.meta.url));
  const profilePath = path.resolve(process.argv[2] || "coverage.out");
  const scopePath = path.join(here, "coverage-scope.list");

  const profileText = fs.readFileSync(profilePath, "utf8");
  const scopeText = fs.readFileSync(scopePath, "utf8");
  const res = evaluate(profileText, scopeText);

  console.log(`[coverage] profile=${profilePath}`);
  console.log(`[coverage] 口径=${path.relative(process.cwd(), scopePath)}（排除 ${res.entries.length} 个包）`);
  for (const e of res.entries) {
    console.log(`[coverage]   排除 ${e.pkg}  —— ${e.reason}`);
  }
  console.log("[coverage] 逐包：");
  for (const r of res.rows) {
    const mark = r.excluded ? "排除" : "计入";
    console.log(
      `[coverage]   ${mark}  ${r.pkg.padEnd(34)} ${r.pct.toFixed(1).padStart(6)}%  ${r.covered}/${r.total}`,
    );
  }
  console.log(
    `[coverage] 合计 ${res.pct.toFixed(2)}%（${res.covered}/${res.total}），阈值 ${res.threshold}%`,
  );

  for (const n of res.notices) console.log(`[coverage] 提示：${n}`);

  if (!res.ok) {
    for (const f of res.failures) console.error(`::error::${f}`);
    console.error(`[coverage] FAIL（${res.failures.length} 条判据未通过）`);
    process.exit(1);
  }
  console.log("[coverage] PASS");
}

// 仅当被执行（而不是被测试 import）时才跑 CLI。
const invokedDirectly =
  process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url);
if (invokedDirectly) {
  main();
}
