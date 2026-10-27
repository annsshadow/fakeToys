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
// 2026-10-27 第四轮：基线降到 0（0/0/0/0），用例跟着现量走。
// 基线为 0 之后「降了提示更新基线」这个分支在真实基线上无法触发，
// 所以 D 用例改用**合成基线**（total=3）单独覆盖那条分支——分支还在，
// 只是不再由真实数据顺带覆盖。
const SYNTH_BASE = "[miniapp]\ncritical = 0\nhigh = 1\nmoderate = 2\nlow = 0\ntotal = 3\n";
check("A miniapp 现状 0/0/0/0 (=0) 对基线 0",
  gate.checkCounts("miniapp", mk(0, 0, 0, 0), baseTxt), false);
check("B 变体 涨到 0/1/0/0 (=1) 必须拦下",
  gate.checkCounts("miniapp", mk(0, 1, 0, 0), baseTxt), true);
check("C 变体 冒出 1 条 critical (=1)",
  gate.checkCounts("miniapp", mk(1, 0, 0, 0), baseTxt), true);
check("D 变体 降到 0/0/1/0 (=1) 对合成基线 3 应提示更新基线而不拦",
  gate.checkCounts("miniapp", mk(0, 0, 1, 0), SYNTH_BASE), false);
check("E admin 现状 0/0/0/0 对基线 0",
  gate.checkCounts("admin", mk(0, 0, 0, 0), baseTxt), false);
check("F 变体 admin 冒出 1 条 high",
  gate.checkCounts("admin", mk(0, 1, 0, 0), baseTxt), true);
check("G 变体 基线里没有该项目段 -> 必须拦下而不是静默放行",
  gate.checkCounts("nonexistent", mk(0, 0, 0, 0), baseTxt), true);

console.log("\n=== checkOverrides ===");
check("H 现状 30 条齐全", gate.checkOverrides(yamlTxt, baseTxt), false);
check("I 变体 删掉 vite 那行", gate.checkOverrides(yamlTxt.replace(/^\s*"vite": 6\.4\.4$/m, ""), baseTxt), true);
check("J 变体 把 vite 改回 5.2.8（dcloudio 的 peer 锁）", gate.checkOverrides(yamlTxt.replace('"vite": 6.4.4', '"vite": 5.2.8'), baseTxt), true);
check("K 变体 删掉 ws 那行", gate.checkOverrides(yamlTxt.replace(/^\s*"ws": 8\.21\.0$/m, ""), baseTxt), true);
check("L 变体 把 postcss 退回 8.5.6", gate.checkOverrides(yamlTxt.replace('"postcss": 8.5.28', '"postcss": 8.5.6'), baseTxt), true);
check("M 变体 overrides 整段清空", gate.checkOverrides("", baseTxt), true);
check("N 变体 删掉 vue 家族那条（跨大版本那批也受同一道闸门管）",
  gate.checkOverrides(yamlTxt.replace(/^\s*"@vue\/server-renderer": 3\.5\.42$/m, ""), baseTxt), true);

const total = 14;
console.log(`\n${bad === 0 ? "全部通过" : `${bad} 条不符`}：${total} 个用例`);
process.exit(bad === 0 ? 0 : 1);
