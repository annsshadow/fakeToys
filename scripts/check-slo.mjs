#!/usr/bin/env node
// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// check-slo.mjs — R82 玩家主链路 SLO 基线守卫（第 5 条要求：
// 「任何一轮让某条路径 p95 劣化超过 20% 就该在下一轮被复核」的自动化）。
//
// # 干什么
//
// 输入是 `cmd/playertest -json` 的结果（一次实测），与
// `laiyipao/server/scripts/slo_baseline.json`（入库基线）逐路径对比：
//
//   1. 预算线：p95 > budget_ms           → FAIL（SLO 超标，立即归因）
//   2. 漂移线：p95 > baseline.p95 × 1.2  → FAIL（劣化 >20%，下一轮必须复核）
//   3. 错误率：err_rate > 0              → FAIL（延迟样本被错误污染，数字不可信）
//
// 全绿输出对照表；任一 FAIL 非零退出。
//
// # 用法
//
//   node scripts/check-slo.mjs laiyipao/server/scripts/slo_baseline.json /tmp/r82_c20.json
//   node scripts/check-slo.mjs --update laiyipao/server/scripts/slo_baseline.json /tmp/r82_c20.json
//
// --update：用本次实测**刷新基线**（只在新基线更优或路径新增时使用；
// 把劣化后的数字刷进基线等于拆掉守卫 —— 所以 --update 会逐路径打印
// 新旧差，>+5% 的更新需要再给一个 --force 确认）。
//
// # 执行节奏（写进台账）
//
// 基线测量需要 1M 仿真库 + RATE_LIMIT_SCALE=100 的 API，CI 无法承载。
// 触发点：改动玩家主链路任一路径的那一轮，在该轮验证里跑 playertest +
// 本脚本；R76 式的周期性复核沿用「每 5-8 轮一次」的现有节奏。
import { readFileSync, writeFileSync } from 'node:fs';

const args = process.argv.slice(2);
const doUpdate = args[0] === '--update';
const doForce = args.includes('--force');
const positional = args.filter((a) => !a.startsWith('--'));
if (positional.length !== 2) {
  console.error('用法: node scripts/check-slo.mjs [--update] [--force] <baseline.json> <playertest-result.json>');
  process.exit(2);
}
const [baselinePath, resultPath] = positional;

const baseline = JSON.parse(readFileSync(baselinePath, 'utf8'));
const results = JSON.parse(readFileSync(resultPath, 'utf8'));

const DRIFT_LIMIT = 1.2; // p95 劣化 >20% 即复核线

if (doUpdate) {
  const next = structuredClone(baseline);
  let needForce = false;
  for (const r of results) {
    const cur = next.paths[r.label];
    const prevP95 = cur?.p95_ms;
    const delta = prevP95 ? ((r.p95_ms - prevP95) / prevP95) * 100 : null;
    if (delta !== null && delta > 5) needForce = true;
    console.log(
      `${r.label.padEnd(20)} p95 ${prevP95 ?? '—'} → ${r.p95_ms.toFixed(2)}ms` +
        (delta !== null ? ` (${delta >= 0 ? '+' : ''}${delta.toFixed(1)}%)` : ' (新增)'),
    );
    next.paths[r.label] = {
      budget_ms: r.budget_ms,
      p50_ms: r.p50_ms,
      p95_ms: r.p95_ms,
      p99_ms: r.p99_ms,
      qps: r.qps,
    };
  }
  if (needForce && !doForce) {
    console.error('\n有路径 p95 上涨 >5% —— 这会把劣化后的数字写进基线、拆掉守卫。');
    console.error('确认这次上涨是「有意变更」再加 --force；否则先去归因，别刷基线。');
    process.exit(2);
  }
  next.meta = { ...next.meta, updated_at: new Date().toISOString().slice(0, 10) };
  writeFileSync(baselinePath, JSON.stringify(next, null, 2) + '\n');
  console.log(`\n基线已更新: ${baselinePath}`);
  process.exit(0);
}

// ---- 对比模式 ----
let fail = false;
const rows = [];
for (const r of results) {
  const base = baseline.paths[r.label];
  if (!base) {
    rows.push(`FAIL ${r.label}: 基线表里没有这条路径 —— 先 --update 登记，别让它游离在守卫外`);
    fail = true;
    continue;
  }
  const problems = [];
  if (r.err_rate > 0) problems.push(`err_rate=${r.err_rate.toFixed(2)}%（样本被错误污染）`);
  if (r.p95_ms > base.budget_ms) problems.push(`p95 ${r.p95_ms.toFixed(2)}ms 超预算 ${base.budget_ms}ms`);
  const drift = r.p95_ms / base.p95_ms;
  if (drift > DRIFT_LIMIT) {
    problems.push(`p95 ${r.p95_ms.toFixed(2)}ms vs 基线 ${base.p95_ms.toFixed(2)}ms（×${drift.toFixed(2)}，劣化 >20%）`);
  }
  if (problems.length > 0) {
    fail = true;
    rows.push(`FAIL ${r.label}: ${problems.join('; ')}`);
  } else {
    rows.push(
      `ok   ${r.label.padEnd(20)} p50=${r.p50_ms.toFixed(2)} p95=${r.p95_ms.toFixed(2)}ms ` +
        `(预算 ${base.budget_ms}ms / 基线 ${base.p95_ms.toFixed(2)}ms，×${drift.toFixed(2)}) qps=${r.qps.toFixed(1)}`,
    );
  }
}
console.log(rows.join('\n'));
if (fail) {
  console.error('\nSLO 守卫：有路径不达标 —— 按 R82 规则，这必须成为下一轮的复核/优化目标。');
  process.exit(1);
}
console.log('\nSLO 守卫：全部路径在预算内且漂移 ≤20%。');
