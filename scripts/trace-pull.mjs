#!/usr/bin/env node
// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// trace-pull.mjs — 按 trace id 从日志里拉出一次请求的全部行（R84 验收工具）。
//
// # 用法
//
//   node scripts/trace-pull.mjs <日志文件> <trace-id> [--expect=access,error,slow]
//
// 把匹配 `req=<id>` 的日志行按文件顺序输出，并按来源分组：
//   [http] 访问日志 / 错误详情、[settle-slow] 慢结算、[recompute] 重算不符。
//
// --expect：要求这几类行至少各出现一次（access=访问日志、error=错误详情、
// slow=慢结算、recompute=重算），缺一即非零退出 —— 验收「拼得出完整故事」，
// 而不是「日志里多了个字段」。
//
// # 排障日常用法
//
// 客户端报障时让它提供响应头的 X-Request-Id，然后：
//   node scripts/trace-pull.mjs server.log <id>
// 一条请求的全部服务端痕迹（访问/错误/慢结算/重算）即时拼成完整故事。
import { readFileSync } from 'node:fs';

const [file, id, ...rest] = process.argv.slice(2);
if (!file || !id) {
  console.error('用法: node scripts/trace-pull.mjs <日志文件> <trace-id> [--expect=access,error,slow,recompute]');
  process.exit(2);
}
// id 只按白名单形状接受（与中间件同口径；拼进正则前防注入）。
if (!/^[A-Za-z0-9_-]{1,64}$/.test(id)) {
  console.error(`trace id ${JSON.stringify(id)} 不合法（白名单 A-Za-z0-9_-，1~64 字符）`);
  process.exit(2);
}
const expectArg = rest.find((a) => a.startsWith('--expect='));
const expect = expectArg ? expectArg.slice('--expect='.length).split(',') : [];

const lines = readFileSync(file, 'utf8').split('\n');
const needle = `req=${id}`;
const hits = lines.filter((l) => l.includes(needle));

if (hits.length === 0) {
  console.error(`没有找到 req=${id} 的任何日志行（${file}）`);
  process.exit(1);
}

const groups = { access: [], error: [], slow: [], recompute: [], other: [] };
for (const l of hits) {
  if (/\[http\].*-> \d{3}: /.test(l)) groups.error.push(l);       // 错误详情（状态码后带冒号原因）
  else if (l.includes('[http]')) groups.access.push(l);          // 访问日志
  else if (l.includes('[settle-slow]')) groups.slow.push(l);     // 慢结算
  else if (l.includes('[recompute]')) groups.recompute.push(l);  // 重算不符
  else groups.other.push(l);
}

console.log(`== req=${id} 共 ${hits.length} 行 ==`);
const show = (name, arr) => {
  if (arr.length === 0) return;
  console.log(`\n-- ${name} (${arr.length}) --`);
  for (const l of arr) console.log(l);
};
show('访问日志', groups.access);
show('错误详情', groups.error);
show('慢结算', groups.slow);
show('重算', groups.recompute);
show('其它', groups.other);

const missing = expect.filter((k) => (groups[k] ?? []).length === 0);
if (missing.length > 0) {
  console.error(`\n故事不完整：缺 ${missing.join(', ')} 类日志行`);
  process.exit(1);
}
console.log('\n故事完整：' + (expect.length > 0 ? `expect=[${expect}] 全部在案` : '（未指定 --expect）'));
