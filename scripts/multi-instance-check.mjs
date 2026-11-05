#!/usr/bin/env node
// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// multi-instance-check.mjs — R87 多副本部署就绪性矩阵（可复跑）。
//
// LIFECYCLE-COVERAGE: none reason=多副本就绪矩阵（最后一条检查实测 4.9s），只测共享状态，不覆盖进程生命周期
//
// # 干什么
//
// 起两个真实 API 实例（共用同一个库，不同端口），逐项实测六类进程内
// 状态在双实例下的真实行为，输出就绪矩阵表并给出退出码：
// 任一「多副本就绪」项实测不符预期即 FAIL。
//
// # 用法
//
//   node scripts/multi-instance-check.mjs [--dsn postgres://...] [--port-a 18090] [--port-b 18091]
//
// 默认连本机 dev 库（laiyipao），实例端口 18090/18091。
// 前置：dev 库已 migrate + seed（含 admin/admin12345 引导账号）。
//
// # 各项判据（实测而非推断；R87 探针数字见台账）
//
//   ① 限流（per-IP/per-user/全局，进程内）：双实例合计放行 = 2× 单实例
//      → 标「可多副本但防护变弱」；解法=RATE_LIMIT_SCALE=1/N（心跳验证过）。
//   ② 缓存新鲜度（TTL 进程内）：A 写公告后 B 立即查**不可见**
//      （最大陈旧=TTL；30s 收敛已在 R87 探针验证，脚本不等收敛）。
//   ③ singleflight：两实例同打冷 key，misses 各 +1（底层查询 ×N）。
//   ④ 幂等键（PG 表）：A 执行 → B 重放逐字节同、余额只动一次 → 多副本就绪。
//   ⑤ 看板/榜单（PG 表）：两实例 battles.total 一致 → 多副本就绪。
//   ⑥ access 聚合（进程内）：A 打 5、B 打 3，各记各的 → 需 sum() 聚合。
//
// 退出码：④⑤（共享正确性）不符即 1；其余项是「标定」而非成败，总输出矩阵。
import { spawn, execSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

const args = process.argv.slice(2);
const opt = (name, def) => {
  const i = args.indexOf('--' + name);
  return i >= 0 ? args[i + 1] : def;
};
const DSN = opt('dsn', 'postgres://postgres@127.0.0.1:5432/laiyipao?sslmode=disable');
const PORT_A = Number(opt('port-a', 18090));
const PORT_B = Number(opt('port-b', 18091));
const SERVER_DIR = new URL('../laiyipao/server', import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, '$1');

const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
const rows = [];
const record = (item, single, dual, verdict, severity) => {
  rows.push({ item, single, dual, verdict, severity });
  console.log(`${verdict.startsWith('FAIL') ? '✗' : '✓'} ${item}\n    单实例: ${single}\n    双实例: ${dual}\n    判定: ${verdict}（${severity}）`);
};

async function http(port, method, path, { token, key, body } = {}) {
  const headers = { 'Content-Type': 'application/json' };
  if (token) headers['Authorization'] = 'Bearer ' + token;
  if (key) headers['Idempotency-Key'] = key;
  const resp = await fetch(`http://127.0.0.1:${port}${path}`, {
    method, headers, body: body === undefined ? undefined : JSON.stringify(body),
  });
  const text = await resp.text();
  return { status: resp.status, text, replay: resp.headers.get('x-idempotent-replay') === 'true' };
}

async function waitReady(port, name, timeoutMs = 60000) {
  const t0 = Date.now();
  for (;;) {
    try {
      const r = await fetch(`http://127.0.0.1:${port}/api/v1/config`);
      if (r.status === 200) return;
    } catch { /* not yet */ }
    if (Date.now() - t0 > timeoutMs) throw new Error(`${name} 启动超时`);
    await sleep(500);
  }
}

function psql(sql) {
  return execSync(
    `docker exec lyp-pg psql -U postgres -d ${DSN.match(/\/(\w+)\?/)[1]} -tAc "${sql}"`,
    { encoding: 'utf8' },
  ).trim();
}

async function main() {
  // 0) 构建一次 exe（起实例比 go run 快且稳）。
  const binDir = join(tmpdir(), 'r87-multi');
  mkdirSync(binDir, { recursive: true });
  const exe = join(binDir, process.platform === 'win32' ? 'api.exe' : 'api');
  console.log('构建 api …');
  execSync(`go build -o "${exe}" ./cmd/api`, { cwd: SERVER_DIR, stdio: 'inherit' });

  // 1) 起双实例（同库、LOG_LEVEL=error 降噪、METRICS 打开）。
  const env = (port) => ({
    ...process.env,
    DATABASE_URL: DSN, ADDR: `:${port}`, LOG_LEVEL: 'error', METRICS_ENABLED: 'true',
  });
  const logA = join(binDir, 'a.log'), logB = join(binDir, 'b.log');
  const a = spawn(exe, [], { env: env(PORT_A), stdio: ['ignore', 'pipe', 'pipe'] });
  const b = spawn(exe, [], { env: env(PORT_B), stdio: ['ignore', 'pipe', 'pipe'] });
  a.stderr.on('data', (d) => writeFileSync(logA, d, { flag: 'a' }));
  b.stderr.on('data', (d) => writeFileSync(logB, d, { flag: 'a' }));
  const cleanup = () => { try { a.kill(); b.kill(); } catch {} };
  process.on('exit', cleanup);

  try {
    await waitReady(PORT_A, 'A');
    await waitReady(PORT_B, 'B');
    console.log(`双实例就绪：A:${PORT_A} B:${PORT_B}\n`);

    // 登录管理端（引导账号 admin/admin12345，dev 默认）。
    const login = await http(PORT_A, 'POST', '/api/v1/admin/login',
      { body: { username: 'admin', password: 'admin12345' } });
    if (login.status !== 200) throw new Error('admin 登录失败: ' + login.text);
    const adminTok = JSON.parse(login.text).access_token;

    // ① 限流：per-IP 60/分/实例，各打 61 → 双实例合计应为 120（2×）。
    let okA = 0, okB = 0;
    for (let i = 0; i < 61; i++) {
      if ((await http(PORT_A, 'POST', '/api/v1/auth/guest', { body: { nickname: 'R87矩阵' } })).status === 200) okA++;
      if ((await http(PORT_B, 'POST', '/api/v1/auth/guest', { body: { nickname: 'R87矩阵' } })).status === 200) okB++;
    }
    record('① 限流（per-IP/per-user/全局，进程内计数）',
      '放行 60/分', `A=${okA} B=${okB} 合计=${okA + okB}`,
      okA + okB === 120 ? '可多副本但防护变弱（额度×N；解法 RATE_LIMIT_SCALE=1/N，已心跳验证）' : 'FAIL 翻倍机理不符',
      '中：防滥用变弱，不产生资金错误（业务幂等兜底）');

    // ② 缓存新鲜度：A 发公告 → B 立即查不可见（TTL 30s 窗口）。
    await http(PORT_A, 'GET', '/api/v1/announcements');
    await http(PORT_B, 'GET', '/api/v1/announcements');
    const title = 'R87矩阵' + Date.now();
    await http(PORT_A, 'POST', '/api/v1/admin/announcements',
      { token: adminTok, body: { title, body: 'R87矩阵正文', published: true } });
    const seeB = (await http(PORT_B, 'GET', '/api/v1/announcements')).text.includes(title);
    record('② 缓存新鲜度（TTL 进程内）',
      '写后立即可见', `B 立即查含新公告=${seeB}`,
      !seeB ? '可多副本（最大陈旧=TTL=30s，探针已验证到期收敛）' : 'FAIL B 立即可见（缓存共享了？）',
      '低：运营配置类陈旧 30s 无资金影响');

    // ③ singleflight：同打冷榜 kind=clear，misses 各 +1（底层查询 ×2）。
    const miss = async (p) => {
      const m = (await http(p, 'GET', '/metrics')).text
        .match(/laiyipao_leaderboard_cache_misses_total (\d+)/);
      return m ? Number(m[1]) : -1;
    };
    const mA0 = await miss(PORT_A), mB0 = await miss(PORT_B);
    // 换未用过的 kind 保证冷（kind=clear 可能已被预热；用 kind=power + 等 TTL 不如直接看增量）。
    for (let i = 0; i < 3; i++) {
      await http(PORT_A, 'GET', '/api/v1/leaderboard?kind=clear');
      await http(PORT_B, 'GET', '/api/v1/leaderboard?kind=clear');
    }
    const mA1 = await miss(PORT_A), mB1 = await miss(PORT_B);
    record('③ singleflight（进程内）',
      '冷启动 1 次底层查询', `misses 增量 A=+${mA1 - mA0} B=+${mB1 - mB0}`,
      mA1 - mA0 >= 1 && mB1 - mB0 >= 1 ? '可多副本（惊群×N，TTL 边界多 N-1 次底层查询）' : 'FAIL misses 未增',
      '低');

    // ④ 幂等键（PG 表）：A 执行 → B 重放逐字节同、余额只 +100。
    // INSERT...RETURNING 在 psql -tA 下输出两行（id + command tag），取首行。
    const uid = Number(psql(
      `INSERT INTO users (guest_token, nickname, is_guest) VALUES ('r87mi_${Date.now()}', 'r87', true) RETURNING id`
    ).split('\n')[0]);
    psql(`INSERT INTO user_wallets (user_id, coin, gem, keys, energy) VALUES (${uid}, 2000, 50, 1, 100)`);
    psql(`INSERT INTO user_progress (user_id, max_stage, level_exp) VALUES (${uid}, 1, 0)`);
    const key = 'r87mi-' + Date.now();
    const g1 = await http(PORT_A, 'POST', `/api/v1/admin/users/${uid}/grant`,
      { token: adminTok, key, body: { currency: 'coin', amount: 100 } });
    const g2 = await http(PORT_B, 'POST', `/api/v1/admin/users/${uid}/grant`,
      { token: adminTok, key, body: { currency: 'coin', amount: 100 } });
    const coin = Number(psql(`SELECT coin FROM user_wallets WHERE user_id=${uid}`));
    record('④ 幂等键（PG 表）',
      '重放同响应', `B replay=${g2.replay} 字节同=${g1.text === g2.text} 余额=${coin}`,
      g1.status === 200 && g2.status === 200 && g2.replay && g1.text === g2.text && coin === 2100
        ? '多副本就绪（天然共享）' : 'FAIL 跨实例重放失效',
      '无（若 FAIL 则资金级事故，必须单实例）');

    // ⑤ 看板增量表：两实例 battles.total 一致。
    const dA = await http(PORT_A, 'GET', '/api/v1/admin/dashboard', { token: adminTok });
    const dB = await http(PORT_B, 'GET', '/api/v1/admin/dashboard', { token: adminTok });
    const tA = JSON.parse(dA.text).battles.total, tB = JSON.parse(dB.text).battles.total;
    record('⑤ 看板/榜单增量表（PG 表）',
      '一致', `A.total=${tA} B.total=${tB}`,
      tA === tB ? '多副本就绪（同源表；各自缓存期内瞬时差异，到期收敛）' : 'FAIL 读数不一致',
      '低');

    // ⑥ access 聚合：A 打 5、B 打 3，各记各的。
    const anon = async (p) => {
      const m = (await http(p, 'GET', '/metrics')).text
        .match(/laiyipao_access_requests_total\{identity="anon",status="2xx"\} (\d+)/);
      return m ? Number(m[1]) : -1;
    };
    const a0 = await anon(PORT_A), b0 = await anon(PORT_B);
    for (let i = 0; i < 5; i++) await http(PORT_A, 'GET', '/api/v1/config');
    for (let i = 0; i < 3; i++) await http(PORT_B, 'GET', '/api/v1/config');
    const a1 = await anon(PORT_A), b1 = await anon(PORT_B);
    record('⑥ access/结算聚合（进程内）',
      '单份', `A +${a1 - a0}、B +${b1 - b0}（各记各）`,
      a1 - a0 >= 5 && b1 - b0 >= 3 ? '可多副本，但必须 sum() 聚合（单实例面板只见 1/N 流量）' : 'FAIL 计数异常',
      '低-中');

    // 汇总矩阵
    console.log('\n================ 就绪矩阵 ================');
    console.table(rows.map(({ item, verdict, severity }) => ({ 项: item, 判定: verdict, 严重度: severity })));
    const fails = rows.filter((r) => r.verdict.startsWith('FAIL'));
    if (fails.length > 0) {
      console.error(`\n${fails.length} 项 FAIL —— 当前形态必须单实例部署（详见各项）`);
      process.exit(1);
    }
    console.log('\n就绪矩阵全绿：④⑤为共享正确性（硬约束）；①②③⑥为标定项（部署时按注释处理）。');
    console.log('部署约束：单实例为默认；多副本时 ① RATE_LIMIT_SCALE=1/N，⑥ metrics 用 sum() 聚合。');
  } finally {
    cleanup();
  }
}

main().catch((e) => { console.error(e); process.exit(2); });
