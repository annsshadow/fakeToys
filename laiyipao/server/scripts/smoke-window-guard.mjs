#!/usr/bin/env node
// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// smoke-window-guard.mjs — 冒烟窗口下限守卫（R103 前的结构性守卫）。
//
// # 守的是什么
//
// 「声称覆盖启动期」的脚本/工作流，最后一条检查必须发生在进程启动后
// **≥ 2× 最长内部超时**。否则它只能证明「进程在第几秒还活着」，
// 而生命周期缺陷恰恰通常在更晚的固定秒数才显形（oa4rust 的样本是 +10s，
// 短冒烟在第 2 秒 exit 0，永远看不到）。
//
// 当前最长内部超时取自 `cmd/api/main.go` 的启动 ctx（60s），
// 下限因此是 120s。守卫**静态对拍**这个数字与 main.go：
// 改了启动 ctx 却不改脚本下限 → 这里红，而不是悄悄放宽。
//
// # 分类标记（每个「会起 API」的脚本都必须写一行）
//
//   LIFECYCLE-COVERAGE: full window_ms=120000 reason=...
//   LIFECYCLE-COVERAGE: none reason=...
//
// `full` 必须满足：声明窗口 ≥ 下限，且文件里存在数值常量等于该窗口
// （防止「标记写 120s、代码只等 5s」）。`none` 必须写明理由 ——
// 「不覆盖」是**正式结论**，不是沉默。没有标记的候选文件直接红：
// 否则新加一个 4 秒的冒烟脚本会默认被当成「覆盖了启动期」。
//
// # 范围
//
// 扫三类位置里有 API 启动签名的文件：
//   - 仓库根 `scripts/`
//   - `.github/workflows/`
//   - `laiyipao/server/scripts/`
//
// 并单独断言 CI 的 server job 真的会跑 lifecycle-survival-check.mjs
// （带 PostgreSQL service）—— 「守卫接进 CI」这件事本身也要被守卫。
import { readFileSync, readdirSync, statSync } from 'node:fs';
import { dirname, join, relative } from 'node:path';
import { fileURLToPath } from 'node:url';

const ROOT = (() => {
  const here = dirname(fileURLToPath(import.meta.url)); // laiyipao/server/scripts
  return dirname(dirname(dirname(here)));
})();

const failures = [];
const errors = (msg) => failures.push(msg);

// ── 1) 最长内部超时：从 cmd/api/main.go 静态抽取 ────────────────────────
const mainGo = join(ROOT, 'laiyipao', 'server', 'cmd', 'api', 'main.go');
const mainSrc = readFileSync(mainGo, 'utf8');

const ctxRe = /context\.WithTimeout\(\s*context\.Background\(\s*\)\s*,\s*(\d+)\s*\*\s*time\.(Second|Minute|Millisecond)\s*\)/g;
const timeouts = [];
for (const m of mainSrc.matchAll(ctxRe)) {
  const n = Number(m[1]);
  const unit = m[2];
  const ms = unit === 'Minute' ? n * 60_000 : unit === 'Second' ? n * 1000 : n;
  timeouts.push(ms);
}
if (timeouts.length === 0) {
  errors('锚点失效：cmd/api/main.go 里没有找到 `context.WithTimeout(context.Background(), N*time.Second)` —— ' +
    '启动超时的写法变了，本守卫无法计算下限');
}
const longestMs = Math.max(0, ...timeouts);
const MIN_WINDOW_MS = 2 * longestMs;

// 停机实现仍必须在：否则「窗口下限」守的是一个不存在的生命周期。
for (const anchor of ['signal.Notify(stop, os.Interrupt, syscall.SIGTERM)', 'app.Listen(cfg.Addr)']) {
  if (!mainSrc.includes(anchor)) {
    errors(`锚点失效：cmd/api/main.go 缺少 \`${anchor}\` —— 优雅停机/监听被改写了，请同步更新本守卫`);
  }
}
if (MIN_WINDOW_MS < 60000) {
  errors(`下限算出来只有 ${MIN_WINDOW_MS}ms（最长内部超时 ${longestMs}ms）—— 明显低于预期，锚点可能失效`);
}

// ── 2) 收集候选文件 ─────────────────────────────────────────────────────
const roots = [
  { dir: join(ROOT, 'scripts'), exts: ['.mjs', '.sh'] },
  { dir: join(ROOT, '.github', 'workflows'), exts: ['.yml', '.yaml'] },
  { dir: join(ROOT, 'laiyipao', 'server', 'scripts'), exts: ['.mjs', '.sh'] },
];

const files = [];
for (const r of roots) {
  for (const name of readdirSync(r.dir)) {
    const p = join(r.dir, name);
    if (!statSync(p).isFile()) continue;
    if (!r.exts.some((e) => name.endsWith(e))) continue;
    files.push(p);
  }
}

const LAUNCH = /cmd\/api|api\.exe|lifecycle-survival-check\.mjs/;
const MARKER = /LIFECYCLE-COVERAGE:\s*(full|none)(.*)$/m;
const SELF = relative(ROOT, fileURLToPath(import.meta.url)).replace(/\\/g, '/');
// 守卫自己的文件里有标记格式的示例，会被 LAUNCH 命中 —— 排除自己，
// 否则「示例」会被当成真实声明来校验。
const candidates = files.filter((p) => {
  const rel = relative(ROOT, p).replace(/\\/g, '/');
  return rel !== SELF && LAUNCH.test(readFileSync(p, 'utf8'));
});

if (files.length < 8) {
  errors(`只扫到 ${files.length} 个脚本/工作流文件（下限 8）—— 扫描目录或扩展名可能被改窄了`);
}
if (candidates.length < 4) {
  errors(`只识别到 ${candidates.length} 个「会起 API」的候选文件（下限 4）—— ` +
    '启动签名正则可能失效，或有人删掉了演练脚本；两种情况都要人工确认');
}

const anchors = [
  'scripts/multi-instance-check.mjs',
  'laiyipao/server/scripts/chaos-drill.mjs',
  'laiyipao/server/scripts/db_restore_drill.sh',
  'laiyipao/server/scripts/lifecycle-survival-check.mjs',
  '.github/workflows/laiyipao-ci.yml',
];
for (const a of anchors) {
  if (!candidates.some((p) => relative(ROOT, p).replace(/\\/g, '/') === a)) {
    errors(`候选清单缺少锚点 ${a} —— 它要么不再起 API（请更新锚点），要么启动签名检测失效`);
  }
}

// ── 3) 逐个校验标记 ─────────────────────────────────────────────────────
const report = [];
for (const p of candidates.sort()) {
  const src = readFileSync(p, 'utf8');
  const rel = relative(ROOT, p).replace(/\\/g, '/');
  const m = src.match(MARKER);
  if (!m) {
    errors(`${rel} 会起 API 但没有 LIFECYCLE-COVERAGE 标记 —— ` +
      '新冒烟脚本必须显式声明「覆盖生命周期(full)」或「不覆盖(none, 写明理由)」');
    report.push({ file: rel, class: '(缺失)', windowMs: '-', note: '未声明' });
    continue;
  }
  const kind = m[1];
  const rest = m[2] || '';
  const win = rest.match(/window_ms=(\d+)/);
  const reason = rest.match(/reason=([^\n]+)/);

  if (kind === 'full') {
    if (!win) {
      errors(`${rel} 标记为 full 但缺少 window_ms=...`);
      continue;
    }
    const declared = Number(win[1]);
    if (declared < MIN_WINDOW_MS) {
      errors(`${rel} 声明窗口 ${declared}ms < 下限 ${MIN_WINDOW_MS}ms ` +
        '（2× 最长内部超时）—— 它覆盖不到生命周期缺陷可能出现的时刻');
    }
    // 数值常量必须与声明一致：防「标记写 120s、代码只等 5s」。
    // 工作流没有常量，它靠下面的 script= 委托检查。
    const isWorkflow = rel.endsWith('.yml') || rel.endsWith('.yaml');
    if (!isWorkflow) {
      const constRe = /(?:MIN_WINDOW_MS|WINDOW_MS)\s*=\s*(\d{4,})/g;
      const consts = [...src.matchAll(constRe)].map((x) => Number(x[1]));
      if (!consts.includes(declared)) {
        errors(`${rel} 标记声明 window_ms=${declared}，但文件里没有等于该值的 WINDOW_MS/MIN_WINDOW_MS 常量 ` +
          `（找到 ${consts.length ? consts.join(', ') : '无'}）—— 声明与代码不符`);
      }
    }
    if (!reason || reason[1].trim().length < 8) {
      errors(`${rel} 标记为 full 但 reason 缺失或过短 —— 必须写清覆盖的是什么`);
    }
    // 工作流的 full 必须真的委托给一个 full 脚本，而不是自己等 5 秒。
    if (isWorkflow) {
      const target = rest.match(/script=([^\s]+)/);
      if (!target || !src.includes(target[1])) {
        errors(`${rel} 是工作流且标记为 full，必须用 script=<路径> 指向被调用的脚本`);
      }
    }
    report.push({ file: rel, class: 'full', windowMs: declared, note: (reason && reason[1].trim()) || '' });
  } else {
    if (!reason || reason[1].trim().length < 8) {
      errors(`${rel} 标记为 none 但没写 reason —— 「不覆盖生命周期」必须说明它实际覆盖什么`);
    }
    report.push({ file: rel, class: 'none', windowMs: '-', note: (reason && reason[1].trim()) || '' });
  }
}

// ── 4) 接进 CI：server job 必须真的跑存活守卫 ──────────────────────────
const ci = join(ROOT, '.github', 'workflows', 'laiyipao-ci.yml');
const ciSrc = readFileSync(ci, 'utf8');
if (!ciSrc.includes('lifecycle-survival-check.mjs')) {
  errors('.github/workflows/laiyipao-ci.yml 没有调用 lifecycle-survival-check.mjs —— ' +
    '守卫只有被真正会跑的流程执行才有意义（multi-instance-check 是手工脚本）');
}
if (!/services:[\s\S]*?postgres:/.test(ciSrc)) {
  errors('.github/workflows/laiyipao-ci.yml 没有 PostgreSQL service —— 存活守卫会因连不上库而假红');
}

// ── 5) 输出 ─────────────────────────────────────────────────────────────
console.log(`最长内部超时 = ${longestMs}ms（取自 cmd/api/main.go，共 ${timeouts.length} 处）`);
console.log(`冒烟窗口下限 = 2× = ${MIN_WINDOW_MS}ms`);
console.log('');
console.table(report);
console.log('');

if (failures.length > 0) {
  for (const f of failures) console.error('::error::' + f);
  console.error(`\n${failures.length} 项不满足 —— 冒烟窗口/声明不可信，必须修。`);
  process.exit(1);
}
console.log(`PASS：${candidates.length} 个候选文件全部显式分类；full 类窗口 ≥ ${MIN_WINDOW_MS}ms；CI 已接入存活守卫。`);
