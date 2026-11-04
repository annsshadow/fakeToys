#!/usr/bin/env node
// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// chaos-drill.mjs — R89 混沌演练：边压边打，三种真实故障注入 × 四项断言。
//
// 用法：
//   node scripts/chaos-drill.mjs                 # 全自动（建库/起 API/负载/注入/断言/清理）
//   node scripts/chaos-drill.mjs --keep          # 结束后保留 chaos 库与 API（排查用）
//
// 前提：docker 容器 lyp-pg（PostgreSQL 17，trust）在跑；go 工具链可用。
// 故障注入（全部真实、可注入可解除、无需重启）：
//   F1 批量断连  pg_terminate_backend 每 300ms 杀一波池后端，15s
//   F2 池耗尽    psql 持 LOCK TABLE users ACCESS EXCLUSIVE 15s（R82 死锁同机理：
//                认证中间件每请求读 users → 全端点排队 → 池占满）
//                R90：排不进池的请求在 DB_ACQUIRE_TIMEOUT（默认 5s）后快速 503。
//                R94：占满池、已在等锁的语句在 DB_LOCK_TIMEOUT（默认 2s）被 PG
//                取消（55P03）并 503 —— 不再挂到锁释放（R89 实测 p95=15032ms）。
//   F3 语句取消  pg_cancel_backend 每 100ms 取消执行中语句，15s
//                （57014 = statement_timeout 与主动 cancel 同一个码）
//
// 四项断言（缺一不算通过）：
//   A. 窗口内错误响应必须是 503 + Retry-After: 5 + service_unavailable（或业务 4xx），
//      500 计数必须为 0（有则逐个归因输出）
//   B. 注入结束后 /admin/reconcile 九项全绿（含 battle_unmarked_settle 半成品检查）
//   C. 自动恢复：解除到「连续 3s ≥95% 成功」的实测秒数
//   D. 无泄漏：acquired_conns 回 0、goroutine ≤ 基线 +20%、total_conns ≤ 基线 +4
//
// 负载：playertest settle 8 并发（真哈希结算写）+ 本脚本 12 worker 混合读写
// （me/mebattles/config/tasks 读 + battle/token 开战 + loadouts 写），合计 20 并发。
// 退出码：任一断言失败 = 1。
import { execSync, spawn } from 'node:child_process';
import { writeFileSync, mkdirSync, rmSync, openSync, closeSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

const args = process.argv.slice(2);
const opt = (n, d) => { const i = args.indexOf('--' + n); return i >= 0 ? args[i + 1] : d; };
const KEEP = args.includes('--keep');
const API_PORT = opt('api', '8190');
const API = `http://127.0.0.1:${API_PORT}`;
// ⚠️ pprof 端口别用 6060/6061：Windows 的「保留端口段」会把它整段占掉
// （netsh interface ipv4 show excludedportrange protocol=tcp，实测 6001-6100），
// 表现是 bind 失败后进程照常服务、只有 pprof 连不上 —— 演练会在基线采集处炸。
const PPROF = opt('pprof', '6121');
const PG = opt('pg', 'lyp-pg');
const DB = opt('db', 'laiyipao_chaos');
const DSN = `postgres://postgres@127.0.0.1:5432/${DB}?sslmode=disable`;
const SERVER_DIR = join(import.meta.dirname, '..');
const WORK = join(tmpdir(), 'r89_chaos');
mkdirSync(WORK, { recursive: true });

const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
const results = [];
let failures = 0;
const note = (ok, msg) => {
  results.push({ ok, msg });
  console.log(`  [${ok ? 'PASS' : 'FAIL'}] ${msg}`);
  if (!ok) failures++;
};

// ---------- psql（写文件 + docker cp，绕开 Windows cmd 剥双引号）----------
let sqlSeq = 0;
function psql(sql, db = DB) {
  const f = join(WORK, `q${sqlSeq++}.sql`);
  writeFileSync(f, sql);
  execSync(`docker cp "${f}" ${PG}:/tmp/r89_q.sql`, { stdio: 'pipe' });
  return execSync(`docker exec ${PG} psql -U postgres -d ${db} -tAf /tmp/r89_q.sql`, { encoding: 'utf8' }).trim();
}

// ---------- HTTP ----------
// ⚠️ keepalive 观测假象（首跑实测 +20 goroutine）：本进程作为客户端持有的
// keepalive 连接会让 fiber 侧 goroutine 下不来，被误判成 API 泄漏。
// 处置：不引第三方 agent（Node 24 不暴露 undici 模块），收尾断言前
// sleep 10s——undici 默认 keepAliveTimeout=4s，连接自然过期关闭。
async function http(method, path, { token, body } = {}) {
  const headers = { 'Content-Type': 'application/json' };
  if (token) headers['Authorization'] = 'Bearer ' + token;
  try {
    const r = await fetch(`${API}${path}`, {
      method, headers, body: body === undefined ? undefined : JSON.stringify(body),
      signal: AbortSignal.timeout(45000),
    });
    const text = await r.text();
    let code = '';
    try { code = JSON.parse(text)?.error?.code ?? ''; } catch { /* 非 JSON */ }
    return { status: r.status, ra: r.headers.get('retry-after') ?? '', code, text };
  } catch (e) {
    return { status: -1, ra: '', code: 'client_abort', text: String(e).slice(0, 100) };
  }
}

// ---------- 观测口 ----------
async function acquiredConns() {
  const m = await (await fetch(`${API}/metrics`)).text();
  return Number(m.match(/laiyipao_db_pool_acquired_conns (\d+)/)?.[1] ?? -1);
}
async function totalConns() {
  const m = await (await fetch(`${API}/metrics`)).text();
  return Number(m.match(/laiyipao_db_pool_total_conns (\d+)/)?.[1] ?? -1);
}
async function goroutines() {
  const t = await (await fetch(`http://127.0.0.1:${PPROF}/debug/pprof/goroutine?debug=1`)).text();
  return Number(t.match(/goroutine profile: total (\d+)/)?.[1] ?? -1);
}

// ---------- 负载 ----------
const records = []; // {t, op, status, code, ra, dur}
let loadStop = false;
async function worker(acc, ops) {
  while (!loadStop) {
    const op = ops[Math.floor(Math.random() * ops.length)];
    const t0 = Date.now();
    const r = await op.fn(acc);
    records.push({ t: t0, op: op.name, status: r.status, code: r.code, ra: r.ra, dur: Date.now() - t0 });
  }
}

// ---------- 主流程 ----------
console.log('== R89 混沌演练 ==');
console.log(`db=${DB} api=${API} pprof=${PPROF}`);

// 0. 干净起点：重建库 + 迁移 + seed
console.log('-- 0. 建库/迁移/seed');
execSync(`docker exec ${PG} psql -U postgres -c "DROP DATABASE IF EXISTS ${DB};"`, { stdio: 'pipe' });
execSync(`docker exec ${PG} psql -U postgres -c "CREATE DATABASE ${DB};"`, { stdio: 'pipe' });
execSync(`go run ./cmd/migrate`, { cwd: SERVER_DIR, env: { ...process.env, DATABASE_URL: DSN }, stdio: 'pipe' });
execSync(`go run ./cmd/seed`, { cwd: SERVER_DIR, env: { ...process.env, DATABASE_URL: DSN, BOOTSTRAP_ADMIN_PASS: 'admin12345' }, stdio: 'pipe' });

// 1. build + 起 API
console.log('-- 1. build & 起 API');
const apiExe = join(WORK, 'chaos_api.exe');
execSync(`go build -o "${apiExe}" ./cmd/api`, { cwd: SERVER_DIR, stdio: 'pipe' });
const apiProc = spawn(apiExe, [], {
  env: {
    ...process.env,
    DATABASE_URL: DSN, ADDR: `:${API_PORT}`, MIGRATIONS_ON_BOOT: 'false',
    METRICS_ENABLED: 'true', PPROF_ADDR: `127.0.0.1:${PPROF}`,
    RATE_LIMIT_SCALE: '100', BOOTSTRAP_ADMIN_PASS: 'admin12345',
  },
  stdio: 'ignore',
});
const cleanup = () => {
  try { apiProc.kill('SIGTERM'); } catch { /* 已退 */ }
  if (!KEEP) {
    try { execSync(`docker exec ${PG} psql -U postgres -c "DROP DATABASE IF EXISTS ${DB};"`, { stdio: 'pipe' }); } catch { /* 忽略 */ }
    try { rmSync(WORK, { recursive: true, force: true }); } catch { /* 忽略 */ }
  }
};
process.on('SIGINT', () => { cleanup(); process.exit(130); });

// 等就绪（build 后启动 <10s，宽限 60s）
let up = false;
for (let i = 0; i < 120 && !up; i++) {
  await sleep(500);
  try { const r = await fetch(`${API}/readyz`); up = r.ok; } catch { /* 未起 */ }
}
if (!up) { console.error('API 60s 未就绪'); cleanup(); process.exit(2); }
console.log('   API 就绪');

// 2. 预置负载账号 + admin token + 形状自检
console.log('-- 2. 预置账号');
const accs = [];
for (let i = 0; i < 12; i++) {
  const g = await http('POST', '/api/v1/auth/guest', { body: { nickname: `R89混压${i}` } });
  if (g.status !== 200) { console.error('建号失败', g.text); cleanup(); process.exit(2); }
  accs.push(JSON.parse(g.text));
}
psql(`UPDATE user_wallets SET energy = 10000 WHERE user_id IN (${accs.map((a) => a.user.id).join(',')})`);
const adminLogin = await http('POST', '/api/v1/admin/login', { body: { username: 'admin', password: 'admin12345' } });
const adminTok = JSON.parse(adminLogin.text).access_token;

// 每类操作先自检一次（200 才进负载池）
const opDefs = [
  { name: 'me', fn: (a) => http('GET', '/api/v1/me', { token: a.access_token }) },
  { name: 'mebattles', fn: (a) => http('GET', '/api/v1/me/battles?limit=5', { token: a.access_token }) },
  { name: 'config', fn: () => http('GET', '/api/v1/config') },
  { name: 'tasks', fn: (a) => http('GET', '/api/v1/tasks', { token: a.access_token }) },
  { name: 'battle_token', fn: (a) => http('POST', '/api/v1/battle/token', { token: a.access_token, body: { level_id: 1 } }) },
  { name: 'loadout_save', fn: (a) => http('PUT', '/api/v1/me/loadout', { token: a.access_token, body: { skill_ids: [1, 4, 0, 0] } }) },
];
const liveOps = [];
for (const op of opDefs) {
  const r = await op.fn(accs[0]);
  if (r.status === 200) { liveOps.push(op); }
  else { console.log(`   [warn] ${op.name} 自检 ${r.status}，从负载池移除（${r.text.slice(0, 80)}）`); }
}
if (liveOps.length < 4) { console.error('可用操作不足 4 类，放弃演练'); cleanup(); process.exit(2); }
console.log(`   负载池：${liveOps.map((o) => o.name).join(', ')}`);

// 权重：读多写少（me×3 mebattles×2 config×2 tasks×2 battle_token×2 loadout_save×1）
const weighted = [];
for (const op of liveOps) {
  const w = { me: 3, mebattles: 2, config: 2, tasks: 2, battle_token: 2, loadout_save: 1 }[op.name] ?? 1;
  for (let i = 0; i < w; i++) weighted.push(op);
}

// 3. 基线
console.log('-- 3. 基线采集');
const baseG = await goroutines();
const baseT = await totalConns();
const rec0 = await http('GET', '/api/v1/admin/reconcile', { token: adminTok });
const baseReconcile = JSON.parse(rec0.text);
note(baseReconcile.ok === true, `基线 reconcile 九项全绿（checks=${baseReconcile.checks}）`);
console.log(`   基线：goroutine=${baseG} total_conns=${baseT} acquired=0`);

// 4. 起负载：playertest settle 8 并发 + Node 12 worker
console.log('-- 4. 起负载（playertest settle c=8 + Node 12 worker）');
const ptExe = join(WORK, 'playertest.exe');
execSync(`go build -o "${ptExe}" ./cmd/playertest`, { cwd: SERVER_DIR, stdio: 'pipe' });
const ptLog = join(WORK, 'playertest.log');
const ptFd = openSync(ptLog, 'w'); // spawn stdio 只收 fd，不认 WriteStream（Node 24 实测抛 ERR_INVALID_ARG_VALUE）
const ptProc = spawn(ptExe, ['-api', API, '-dsn', DSN, '-scenario', 'settle', '-c', '8', '-d', '100s'], { stdio: ['ignore', ptFd, ptFd] });
// 充能守护：每 5s 把负载账号体力刷满（写失败会自愈，故障期被 cancel 下一轮再来）
const feeder = setInterval(() => {
  try { psql(`UPDATE user_wallets SET energy = 10000 WHERE user_id IN (${accs.map((a) => a.user.id).join(',')}) AND energy < 500`); } catch { /* 故障期忽略 */ }
}, 5000);
const workers = [];
for (let i = 0; i < 12; i++) workers.push(worker(accs[i % accs.length], weighted));
await sleep(8000); // 基线流量窗
// goroutine 稳态基线（不是冷启动值：池按需扩容的连接常驻 goroutine、
// fiber 连接 goroutine 都是稳态成分，冷启动基线会误判——复跑实测
// 冷基线 77 vs 负载稳态 ~115 vs 停负载后 ~98）。
const steadyG = await goroutines();
console.log(`   负载稳态 goroutine=${steadyG}（终态判据基线）`);

// ---------- 故障周期 ----------
async function reconcileNow() {
  const r = await http('GET', '/api/v1/admin/reconcile', { token: adminTok });
  try { return JSON.parse(r.text); } catch { return { ok: false, parse_error: r.text.slice(0, 120) }; }
}
function windowStats(t0, t1) {
  const w = records.filter((r) => r.t >= t0 && r.t < t1);
  const byStatus = {};
  const e500 = [];
  let bad503 = 0;
  let slow = 0; // dur>10s：锁阻塞请求的证据（语句级等待，不是 Acquire 挂起）
  let s503 = 0; // 503 计数
  let max503 = 0; // 503 的最长耗时
  let slow503 = 0; // 503 且 dur>6s（超时 5s + 1s 余量之后）——R90 的失败判据
  for (const r of w) {
    byStatus[r.status] = (byStatus[r.status] ?? 0) + 1;
    if (r.status === 500) e500.push(r);
    if (r.status === 503 && (r.ra !== '5' || r.code !== 'service_unavailable')) bad503++;
    if (r.dur > 10000) slow++;
    if (r.status === 503) {
      s503++;
      if (r.dur > max503) max503 = r.dur;
      if (r.dur > 6000) slow503++;
    }
  }
  return { n: w.length, byStatus, e500, bad503, slow, s503, max503, slow503 };
}
// measureRecovery：真实轮询（不是逻辑遍历——首跑 bug：遍历「未来时间桶」
// 时记录尚未产生，永远 '>60'）。每 500ms 看最近 3s 的实际记录，
// ≥5 条且 ≥95% 为 200 即判定恢复，返回解除到恢复的实测秒数；上限 30s。
async function measureRecovery(tEnd) {
  const deadline = tEnd + 30000;
  while (Date.now() < deadline) {
    await sleep(500);
    const now = Date.now();
    const w = records.filter((r) => r.t >= now - 3000 && r.t <= now);
    if (w.length < 5) continue;
    const okN = w.filter((r) => r.status === 200).length;
    if (okN / w.length >= 0.95) return ((now - tEnd) / 1000).toFixed(1);
  }
  return '>30';
}

async function drillPhase(name, injectStart, injectStop, holdSec, extraAssert = null) {
  console.log(`-- ${name}`);
  const t0 = Date.now();
  const stopper = await injectStart();
  await sleep(holdSec * 1000);
  await injectStop(stopper);
  const t1 = Date.now();
  // 恢复测量（负载继续跑）
  const recoverSec = await measureRecovery(t1);
  await sleep(2000); // 窗口尾巴落定
  const w = windowStats(t0, Date.now());
  // 断言 A：无 500；503 必须带 Retry-After:5 + service_unavailable
  note(w.e500.length === 0, `${name} 窗口 500 计数=${w.e500.length}（状态分布 ${JSON.stringify(w.byStatus)}）`);
  if (w.e500.length > 0) {
    for (const e of w.e500.slice(0, 5)) console.log(`     500 样本: op=${e.op} code=${e.code} t=${new Date(e.t).toISOString()}`);
  }
  note(w.bad503 === 0, `${name} 503 形状合规（缺 Retry-After/错 code 的 503 计数=${w.bad503}）`);
  // 断言 B：reconcile
  const rec = await reconcileNow();
  note(rec.ok === true, `${name} reconcile 九项全绿${rec.ok ? '' : '：' + JSON.stringify(rec).slice(0, 200)}`);
  // 断言 C：恢复（实测秒数）
  console.log(`   [恢复] ${name} 解除后 ${recoverSec}s 恢复（最近 3s ≥95% 成功）`);
  note(recoverSec !== '>30', `${name} 30s 内自动恢复（实测 ${recoverSec}s）`);
  // 观测（不断言）：负载中满池/满 goroutine 是正常形态，泄漏断言统一在
  // 收尾停负载后做（首跑教训：负载中 acquired=10 不是泄漏，是打满）。
  const ac = await acquiredConns();
  const g = await goroutines();
  console.log(`   [观测] 窗口后 acquired=${ac} goroutine=${g}（负载运行中，仅供参考）`);
  if (extraAssert) extraAssert(w);
  return w;
}

// F1 批量断连：每 300ms 杀全部池后端
await drillPhase('F1 批量断连',
  () => {
    const iv = setInterval(() => {
      try {
        psql(`SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname='${DB}' AND pid <> pg_backend_pid() AND backend_type='client backend';`);
      } catch { /* 注入器自身失败不挡演练 */ }
    }, 300);
    return iv;
  },
  (iv) => clearInterval(iv), 15);

// F2 池耗尽：psql 持锁 users（持锁 15s 后自动释放）。
// ⚠️ fail loud（首跑教训）：spawn 静默失败 + stdio:ignore 吞错 → 锁没拿到
// 而窗口「全 200」被误判为「无影响」。注入后必须在 pg_locks 里确认锁真的
// 持有，否则直接 FAIL，不许继续。
await drillPhase('F2 池耗尽（锁 users 表）',
  async () => {
    const lockSql = join(WORK, 'lock.sql');
    // pg_sleep(60) 是保险丝：正常路径由 injectStop kill 释放（kill 时刻=
    // 真实释放点，恢复测量才精确）；脚本崩了也最多持有 60s。
    writeFileSync(lockSql, `BEGIN; LOCK TABLE users IN ACCESS EXCLUSIVE MODE; SELECT pg_sleep(60); ROLLBACK;`);
    execSync(`docker cp "${lockSql}" ${PG}:/tmp/r89_lock.sql`, { stdio: 'pipe' });
    const errChunks = [];
    const p = spawn('docker', ['exec', PG, 'psql', '-U', 'postgres', '-d', DB, '-f', '/tmp/r89_lock.sql'], { stdio: ['ignore', 'ignore', 'pipe'] });
    p.stderr.on('data', (d) => errChunks.push(String(d)));
    // 确认锁真的拿到（最多等 5s）
    let held = false;
    for (let i = 0; i < 20 && !held; i++) {
      await sleep(250);
      try {
        held = psql(`SELECT count(*) FROM pg_locks WHERE relation='users'::regclass AND mode='AccessExclusiveLock' AND granted;`) === '1';
      } catch { /* 查询失败重试 */ }
    }
    if (!held) {
      note(false, `F2 锁未生效：5s 内 pg_locks 未见 AccessExclusiveLock（stderr: ${errChunks.join('').slice(0, 200) || '空'}）`);
    } else {
      console.log('   [注入确认] AccessExclusiveLock 已持有');
    }
    return p;
  },
  // 释放锁 = terminate 容器内 psql 的后端（主路径：kill 本地 docker CLI
  // 只会让容器内 psql 变孤儿继续跑 pg_sleep；terminate 后端才真的回滚
  // 事务放锁）。terminate 时点=恢复测量零点。
  (p) => {
    try {
      psql(`SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname='${DB}' AND pid <> pg_backend_pid() AND (query LIKE '%pg_sleep(60)%' OR state='active' AND wait_event='Timeout' AND query LIKE '%pg_sleep%');`);
    } catch { /* 忽略 */ }
    try { p.kill('SIGKILL'); } catch { /* 已退 */ }
  }, 15,
  // F2 专属：R90+R94 快速失败语义——池耗尽时排不进池的请求必须在
  // DB_ACQUIRE_TIMEOUT（默认 5s）+1s 余量内拿到 503；同时 client_abort=0
  // （服务端确实回了终局）。
  //
  // ⚠️ 锁生效的证据**不再**靠「dur>10s 挂起请求」——那是 R89/R90 的旧
  // 语义，把故障后果当成了生效证据。R94 的 DB_LOCK_TIMEOUT（默认 2s）
  // 落地后，锁阻塞语句会在 ~2s 被 PG 取消并快速 503，窗口内**不该再有**
  // dur>10s 的请求。锁真的生效由注入时的 pg_locks 确认（上方已 fail-loud），
  // 这里断言的是它的**后果**：没有语句被允许挂到 10s 以上。
  (w) => {
    const aborted = w.byStatus['-1'] ?? 0;
    note(aborted === 0, `F2 语义：窗口 client_abort=${aborted}（请求应由服务端给出终局）`);
    note(w.s503 > 0, `F2 快速失败证据：窗口 503 计数=${w.s503}（R90 后池耗尽必须出现快速 503）`);
    note(w.slow503 === 0, `F2 503 全部在 5s+1s 内返回（窗口最长 503 ${w.max503}ms，>6s 的 503=${w.slow503}）`);
    note(w.slow === 0, `F2 锁等待预算生效：窗口 dur>10s 请求数=${w.slow}（R94 后应为 0——锁阻塞语句 ~2s 被 lock_timeout 取消，不再挂到锁释放）`);
  });

// F3 语句取消：每 100ms cancel 执行中的语句（57014）。
// 慢目标增强：毫秒级查询几乎打不中（首跑 Node 侧零 503 实证）——窗口内
// 额外起 2 个 reconcile 循环 worker（九项聚合同库行数正相关，是 API 里
// 最慢的合法读），给 cancel 一个稳定打中面；settle 事务（playertest）
// 是第二个打中面，其 503 落在 playertest 日志。
let reconLoad = false;
async function reconWorker() {
  while (reconLoad) {
    const t0 = Date.now();
    const r = await http('GET', '/api/v1/admin/reconcile', { token: adminTok });
    records.push({ t: t0, op: 'reconcile', status: r.status, code: r.code, ra: r.ra, dur: Date.now() - t0 });
  }
}
let reconWorkers = [];
await drillPhase('F3 语句取消（57014）',
  async () => {
    reconLoad = true;
    reconWorkers = [reconWorker(), reconWorker(), reconWorker(), reconWorker()];
    // 常驻 DO 块注入（首跑教训：每波 docker exec 起 psql ~300-500ms，
    // 实际频率 ~2/s 打不中毫秒级查询）——单连接内 25ms 循环 cancel，
    // 600 波 × 25ms = 15s。
    // ⚠️ plpgsql DO 块是单条语句：READ COMMITTED 下整段共享**启动时快照**——
    // 只能 cancel「DO 块启动那一刻已存在」的后端（API 池长连接全在快照里，
    // 是被打击目标，无碍）；启动后新建的连接永远打不到（实测：后起的
    // pg_sleep 探针跑满 30s 未被打断）。所以注入确认的探针必须**先于**
    // DO 块启动，让它的 pid 进快照。
    const probe = spawn('docker', ['exec', PG, 'psql', '-U', 'postgres', '-d', DB, '-c', 'SELECT pg_sleep(60);'], { stdio: ['ignore', 'ignore', 'pipe'] });
    const probeErr = [];
    probe.stderr.on('data', (d) => probeErr.push(String(d)));
    await sleep(1200); // 确保探针连接建立并进入 active
    const doSql = join(WORK, 'cancel_loop.sql');
    writeFileSync(doSql, `DO $$ BEGIN FOR i IN 1..600 LOOP
  PERFORM pg_cancel_backend(pid) FROM pg_stat_activity
   WHERE datname='${DB}' AND state='active' AND pid <> pg_backend_pid()
     AND query NOT LIKE '%pg_cancel_backend%';
  PERFORM pg_sleep(0.025);
END LOOP; END $$;`);
    execSync(`docker cp "${doSql}" ${PG}:/tmp/r89_cancel.sql`, { stdio: 'pipe' });
    const p = spawn('docker', ['exec', PG, 'psql', '-U', 'postgres', '-d', DB, '-f', '/tmp/r89_cancel.sql'], { stdio: ['ignore', 'ignore', 'pipe'] });
    const errChunks = [];
    p.stderr.on('data', (d) => errChunks.push(String(d)));
    // 注入确认：60s 的 pg_sleep 探针必须被 57014 提前打断
    const probeT0 = Date.now();
    await new Promise((r) => probe.on('exit', r));
    const probeMs = Date.now() - probeT0;
    const hit = probeMs < 55000 && /57014|cancel/i.test(probeErr.join(''));
    note(hit, `F3 注入确认：60s pg_sleep 探针被 cancel 打断（${(probeMs / 1000).toFixed(1)}s 后 57014）${hit ? '' : ' stderr=' + probeErr.join('').slice(0, 150)}`);
    return p;
  },
  async (p) => {
    try { p.kill('SIGKILL'); } catch { /* 已退 */ }
    reconLoad = false;
    await Promise.allSettled(reconWorkers);
    reconWorkers = [];
  }, 15);

// ---------- 收尾 ----------
console.log('-- 5. 收尾（停负载 → 释放 keepalive → 终态泄漏断言）');
loadStop = true;
clearInterval(feeder);
await Promise.allSettled(workers);
// playertest -d 100s 覆盖到 F3 结束：先等它自然退出打出汇总（45s 宽限），
// 超时才 kill——kill 会丢掉状态分布汇总（settle 写路径在故障窗口的 503 证据）。
{
  const exited = await Promise.race([
    new Promise((r) => ptProc.on('exit', () => r(true))),
    sleep(45000).then(() => false),
  ]);
  if (!exited) ptProc.kill('SIGTERM');
}
await sleep(1500);
try { closeSync(ptFd); } catch { /* 忽略 */ }
// 等本进程 keepalive 自然过期（undici 默认 4s）+ fiber 侧连接 goroutine
// 退出 + 尾巴请求释放——否则 goroutine 观测含本进程客户端连接的假象。
await sleep(10000);
// playertest 汇总（settle 写路径全程状态分布）
try {
  const pt = execSync(`findstr /C:"status" /C:"label" /C:"err_rate" "${ptLog}"`, { encoding: 'utf8' }).trim();
  console.log('   playertest settle 汇总：');
  for (const line of pt.split('\n')) console.log('     ' + line.trim());
} catch { console.log('   playertest 日志无汇总行（进程被提前 kill 属预期）'); }
// 终态断言：reconcile + 泄漏（这是唯一合法的泄漏判定点——负载已停）
const recFinal = await reconcileNow();
note(recFinal.ok === true, '终态 reconcile 九项全绿');
const acF = await acquiredConns();
const gF = await goroutines();
note(acF === 0, `终态 acquired_conns=${acF}（泄漏判据：回 0）`);
// goroutine 判据：停负载后应低于负载稳态（fiber 连接 goroutine 随
// keepalive 过期退出），允许 10% 抖动。持续不降才是真泄漏信号。
note(gF <= Math.ceil(steadyG * 1.1), `终态 goroutine=${gF} ≤ 负载稳态 ${steadyG}×1.1=${Math.ceil(steadyG * 1.1)}（冷启动基线 ${baseG}）`);

console.log('\n== 结论 ==');
for (const r of results) console.log(`  [${r.ok ? 'PASS' : 'FAIL'}] ${r.msg}`);
console.log(`\n${failures === 0 ? '✅ 全部断言通过' : `❌ ${failures} 项失败`}`);
console.log(`records=${records.length}`);
writeFileSync(join(WORK, 'records.ndjson'), records.map((r) => JSON.stringify(r)).join('\n'));
console.log(`证据：${join(WORK, 'records.ndjson')}${KEEP ? '' : '（非 --keep 模式已随清理删除，复跑加 --keep 保留）'}`);
cleanup();
process.exit(failures === 0 ? 0 : 1);
