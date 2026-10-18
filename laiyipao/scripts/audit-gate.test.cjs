// 守卫的守卫：验证 audit-gate.cjs 该绿绿、该红红。
//
// 为什么要有这个：本仓已经吃过一次「安静地不检查」的亏——
// laiyipao-ci.yml 里 "type-check must actually fail on a type error"
// 那一步记录的，miniapp 的 vue-tsc 2.0.22 与 TS 5.9 不兼容时报
// "Search string not found" 然后**退出 0**，类型检查在 CI 里一直是假绿。
//
// 闸门本身如果写错了（比如判据写反、永远返回 pass），它会比没有闸门
// 更糟——因为它会给出「已保护」的假象。所以这里对它做变异测试。
//
// 跑法：node laiyipao/scripts/audit-gate.test.cjs

const fs = require("fs");
const path = require("path");
const gate = require("./audit-gate.cjs");

const REPO = path.resolve(__dirname, "..", "..");
const BASE = path.join(REPO, "laiyipao", "audit-baseline.ini");
const WS = path.join(REPO, "laiyipao", "miniapp", "pnpm-workspace.yaml");

const baseTxt = fs.readFileSync(BASE, "utf8");
const yamlTxt = fs.readFileSync(WS, "utf8");

const mk = (c, h, m, l) =>
  JSON.stringify({ metadata: { vulnerabilities: { critical: c, high: h, moderate: m, low: l } } });

let bad = 0;
function check(name, got, wantFail) {
  const ok = got.fail === wantFail;
  if (!ok) bad++;
  console.log(`  ${ok ? "PASS" : "FAIL"}  ${name}`);
  console.log(`        fail=${got.fail} 期望=${wantFail}   ${got.msg || got.notice || ""}`);
}

console.log(`基线 ${baseTxt.length} 字节 / workspace ${yamlTxt.length} 字节\n`);

console.log("=== checkCounts ===");
check("A miniapp 现状 3/13/16/7 (=39) 对基线 39",
  gate.checkCounts("miniapp", mk(3, 13, 16, 7), baseTxt), false);
check("B 变体 删掉 vite override 后回到 3/18/28/9 (=58)",
  gate.checkCounts("miniapp", mk(3, 18, 28, 9), baseTxt), true);
check("C 变体 新增 1 条 critical (=40)",
  gate.checkCounts("miniapp", mk(4, 13, 16, 7), baseTxt), true);
check("D 变体 降到 3/10/10/5 (=28) 应提示更新基线而不拦",
  gate.checkCounts("miniapp", mk(3, 10, 10, 5), baseTxt), false);
check("E admin 现状 0/0/0/0 对基线 0",
  gate.checkCounts("admin", mk(0, 0, 0, 0), baseTxt), false);
check("F 变体 admin 冒出 1 条 high",
  gate.checkCounts("admin", mk(0, 1, 0, 0), baseTxt), true);
check("G 变体 基线里没有该项目段 -> 必须拦下而不是静默放行",
  gate.checkCounts("nonexistent", mk(0, 0, 0, 0), baseTxt), true);

console.log("\n=== checkOverrides ===");
check("H 现状 4 条齐全", gate.checkOverrides(yamlTxt, baseTxt), false);
check("I 变体 删掉 vite 那行", gate.checkOverrides(yamlTxt.replace(/^\s*"vite": 5\.4\.21$/m, ""), baseTxt), true);
check("J 变体 把 vite 改回 5.2.8", gate.checkOverrides(yamlTxt.replace('"vite": 5.4.21', '"vite": 5.2.8'), baseTxt), true);
check("K 变体 删掉 ws 那行", gate.checkOverrides(yamlTxt.replace(/^\s*"ws": 8\.21\.0$/m, ""), baseTxt), true);
check("L 变体 把 postcss 退回 8.5.6", gate.checkOverrides(yamlTxt.replace('"postcss": 8.5.28', '"postcss": 8.5.6'), baseTxt), true);
check("M 变体 overrides 整段清空", gate.checkOverrides("", baseTxt), true);

const total = 13;
console.log(`\n${bad === 0 ? "全部通过" : `${bad} 条不符`}：${total} 个用例`);
process.exit(bad === 0 ? 0 : 1);
