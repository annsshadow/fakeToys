// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
//
// 写门禁接线尺：把 write_handlers.tsv（写 handler 候选矩阵）与
//   (1) axum 路由注册（handler → 完整路径 + HTTP 方法）
//   (2) shared/middleware/constants.rs 的 AUTH_EXEMPT_PATHS / AUTH_EXEMPT_PREFIXES /
//       ADMIN_WRITE_PREFIXES（中间件级门禁，handler 体内看不到的那层）
// 做确定性对账，输出每个写 handler 的可达性与「有效门禁」分桶。
//
// 用法（在 oa4rust/backend 下）：node scripts/audit_write_gating.mjs [--tsv path]
// 口径：分桶是「机器可证的部分」，不是漏洞判定——桶 B/C 仍需逐格人工取证
// （helper 内部门禁、属主/creator 比对、租户范围都不在本尺扫描面内）。

import fs from "node:fs";
import path from "node:path";

const BACKEND = path.resolve(import.meta.dirname, "..");
const CRATES = path.join(BACKEND, "crates");
const ARTIFACTS = path.join(BACKEND, "docs/audits/artifacts");
fs.mkdirSync(ARTIFACTS, { recursive: true });
const TSV = process.argv.includes("--tsv")
  ? process.argv[process.argv.indexOf("--tsv") + 1]
  : path.join(ARTIFACTS, "write_handlers.tsv");
const CONSTANTS = path.join(CRATES, "shared/src/middleware/constants.rs");

// ── 1. 中间件常量表 ──────────────────────────────────────────────────────────
function readConstList(src, name) {
  const m = new RegExp(`pub const ${name}: &\\[&str\\] = &\\[([\\s\\S]*?)\\];`).exec(src);
  if (!m) throw new Error(`const ${name} not found in constants.rs`);
  return [...m[1].matchAll(/"([^"]*)"/g)].map((x) => x[1]);
}
const csrc = fs.readFileSync(CONSTANTS, "utf8");
const EXEMPT_PATHS = readConstList(csrc, "AUTH_EXEMPT_PATHS");
const EXEMPT_PREFIXES = readConstList(csrc, "AUTH_EXEMPT_PREFIXES");
const ADMIN_WRITE_PREFIXES = readConstList(csrc, "ADMIN_WRITE_PREFIXES");

// rbac.rs 注册表：`authorize_middleware` 命中 Public 会**提前 return**，
// 从而跳过 `check_permission` 里 `Authenticated + requires_admin → is_admin` 那把中间件兜底。
// 所以「Public 前缀 ⊇ 某写路由」= 该写路由的 admin 兜底被吞，handler 层是唯一门禁。
// 注意 get_permission 的前缀匹配是裸 starts_with（无段边界），这里按同一口径复刻。
const RBAC = path.join(CRATES, "shared/src/middleware/rbac.rs");
const rsrc = fs.readFileSync(RBAC, "utf8");
const PUBLIC_EXACT = [
  ...rsrc.matchAll(/registry\.register\("([^"]+)",\s*PermissionLevel::Public\)/g),
].map((m) => m[1]);
const PUBLIC_PREFIXES = [
  ...rsrc.matchAll(/registry\.register_prefix\("([^"]+)",\s*PermissionLevel::Public\)/g),
].map((m) => m[1]);
function registryPublic(p) {
  if (PUBLIC_EXACT.includes(p)) return p;
  let best = null;
  for (const pre of PUBLIC_PREFIXES) {
    if (p.startsWith(pre) && (!best || pre.length > best.length)) best = pre;
  }
  return best;
}

const segs = (p) => p.split("/").filter(Boolean);
// 复刻 shared/middleware/security.rs 的 path_matches：段数相等，模式段 {x} 通配
function pathMatches(path, pattern) {
  const a = segs(path), b = segs(pattern);
  if (a.length !== b.length) return false;
  return a.every((s, i) => (b[i] === "{param}" || b[i].startsWith("{") ? true : s === b[i]));
}
// 复刻 path_starts_with_segment：段边界，避免 /api/person* 误匹配 /api/personal
function segPrefix(path, prefix) {
  return path === prefix || path.startsWith(`${prefix}/`);
}
function isAuthExempt(method, p) {
  return (
    EXEMPT_PATHS.some((pat) => pathMatches(p, pat)) ||
    EXEMPT_PREFIXES.some((pre) => segPrefix(p, pre))
  );
}
function mwAdmin(method, p) {
  return (method === "POST" || method === "PUT" || method === "DELETE") &&
    ADMIN_WRITE_PREFIXES.some((pre) => segPrefix(p, pre));
}

// ── 2. 路由注册扫描 ──────────────────────────────────────────────────────────
function walk(dir, out = []) {
  for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
    if (e.name === "target" || e.name.startsWith(".")) continue;
    const full = path.join(dir, e.name);
    if (e.isDirectory()) walk(full, out);
    else if (e.name.endsWith(".rs")) out.push(full);
  }
  return out;
}
const ROUTE_RE = /\.route\(\s*"([^"]+)"\s*,\s*([\s\S]{0,400}?)\)\s*\)/g;
// 一次 .route() 可链式挂多个方法（`.route("p", delete(a).put(b))`）。必须逐个枚举：
// 只取首方法的旧写法会让「链上第二个方法」从分母里静默消失——四轮取证实测漏掉
// processing 的 work_edit / work_delete / att_edit_id 三把（它们从未进过任何残余名单）。
const METHOD_CHAIN_RE = /(?:^|[^A-Za-z0-9_])(get|post|put|delete|patch|head|options)\s*\(\s*((?:[A-Za-z_][A-Za-z0-9_]*::)*[A-Za-z_][A-Za-z0-9_]*)/gi;
const routes = []; // {crate, file, line, path, method, handler}
const seenRouteFiles = [];
for (const file of walk(CRATES)) {
  const src = fs.readFileSync(file, "utf8");
  if (!src.includes(".route(")) continue;
  seenRouteFiles.push(file);
  const crate = path.relative(CRATES, file).split(path.sep)[0];
  for (const m of src.matchAll(ROUTE_RE)) {
    const rpath = m[1];
    const inner = m[2];
    const line = src.slice(0, m.index).split("\n").length;
    for (const cm of inner.matchAll(METHOD_CHAIN_RE)) {
      routes.push({
        crate,
        file,
        line,
        path: rpath,
        method: cm[1].toUpperCase(),
        handler: cm[2].split("::").pop(),
      });
    }
  }
}

// handler 名 → 路由（同名 handler 可能被多个 crate/路径复用）
const byHandler = new Map();
for (const r of routes) {
  if (!byHandler.has(r.handler)) byHandler.set(r.handler, []);
  byHandler.get(r.handler).push(r);
}

// ── 3. 与写 handler 矩阵对账 ─────────────────────────────────────────────────
const rows = fs
  .readFileSync(TSV, "utf8")
  .split("\n")
  .filter((l) => l.trim())
  .slice(1)
  .map((l) => {
    const [crate, file, line, handler, session, gate, tables] = l.split("\t");
    return { crate, file, line: Number(line), handler, session, gate, tables: tables || "" };
  });

const buckets = new Map();
const residual = [];
for (const row of rows) {
  const cands = (byHandler.get(row.handler) || []).filter((r) => r.crate === row.crate);
  const use = cands.length ? cands : byHandler.get(row.handler) || [];
  let bucket, note;
  if (!use.length) {
    bucket = "E-未注册路由";
    note = "本尺未在 crates/ 的 .route() 里找到该 handler（可能是内部函数/宏注册/别名）";
  } else {
    const r = use[0];
    const exempt = isAuthExempt(r.method, r.path);
    const admin = mwAdmin(r.method, r.path);
    const gated = row.gate === "G";
    const pub = registryPublic(r.path);
    if (admin && pub && !gated) bucket = "G-Public早退吞掉admin兜底且无体内门禁";
    else if (admin && pub) bucket = "H-Public早退但体内有门禁";
    else if (admin) bucket = "A-中间件 admin";
    else if (exempt && !gated) bucket = "B-匿名可达且无体内门禁";
    else if (exempt) bucket = "C-匿名可达但有体内门禁";
    else if (!row.session && !gated) bucket = "D-登录即可写且无体内门禁";
    else bucket = "F-已认证+有门禁信号";
    note = `${r.method} ${r.path}${use.length > 1 ? ` (+${use.length - 1} 处)` : ""}${
      pub ? ` [Public 命中 ${pub}]` : ""
    }`;
  }
  const key = bucket;
  buckets.set(key, (buckets.get(key) || 0) + 1);
  if (
    bucket === "B-匿名可达且无体内门禁" ||
    bucket === "D-登录即可写且无体内门禁" ||
    bucket === "G-Public早退吞掉admin兜底且无体内门禁"
  ) {
    residual.push({ ...row, route: note, bucket });
  }
}

const routeHandlerCount = byHandler.size;
console.log(`路由注册：扫描 ${seenRouteFiles.length} 个含 .route() 的文件，` +
  `解析出 ${routes.length} 条路由、${routeHandlerCount} 个具名 handler`);
console.log(`中间件常量：AUTH_EXEMPT_PATHS=${EXEMPT_PATHS.length} ` +
  `AUTH_EXEMPT_PREFIXES=${EXEMPT_PREFIXES.length} ADMIN_WRITE_PREFIXES=${ADMIN_WRITE_PREFIXES.length}`);
console.log(`写 handler 矩阵：${rows.length} 行`);
console.log("\n分桶：");
for (const [k, v] of [...buckets].sort((a, b) => b[1] - a[1])) console.log(`  ${String(v).padStart(4)}  ${k}`);
console.log(`\n残余待取证：${residual.length} 行`);
const byCrate = new Map();
for (const r of residual) byCrate.set(r.crate, (byCrate.get(r.crate) || 0) + 1);
console.log("按 crate：");
for (const [k, v] of [...byCrate].sort((a, b) => b[1] - a[1])) console.log(`  ${String(v).padStart(4)}  ${k}`);
console.log("\n明细（前 120 行）：");
for (const r of residual.slice(0, 120)) {
  console.log(`${r.bucket.slice(0, 1)}\t${r.crate}\t${r.file}:${r.line}\t${r.handler}\t${r.route}\t${r.tables}`);
}
fs.writeFileSync(
  path.join(ARTIFACTS, "write_gating_residual.tsv"),
  ["bucket\tcrate\tfile\tline\thandler\troute\ttables",
    ...residual.map((r) => `${r.bucket}\t${r.crate}\t${r.file}\t${r.line}\t${r.handler}\t${r.route}\t${r.tables}`)].join("\n") + "\n",
);
console.log("\n全量残余已写 write_gating_residual.tsv");

// ── 4. 路由级对账（真覆盖面：所有写路由 × 有效门禁）───────────────────────────
// tsv 是「含写 SQL 的函数」级，514/991 是内部 helper；本段以 HTTP 写路由为分母，
// 逐条判定中间件级 admin、匿名豁免、handler 体内的会话/门禁信号，
// 并把「匿名可达 + 体内无门禁」与「已认证可达 + 无会话读取」两张残余清单落盘。
const SESSION_RE = /Extension\s*<\s*(?:crate::)?(?:shared::)?session::Session|Extension<Session>|session\s*:\s*Option<Extension/;
// 缺陷 3 修正（四轮 §四）：词表按「完整标识符」匹配。旧写法 `\b(require_admin|…)\b` 认不出
// `require_admin_opt(` —— `require_admin` 后面紧跟 `_`（仍是词字符），尾部 `\b` 落空，
// 于是加了门的 handler 仍被计成「无门禁」。同时补入手工会话校验那一族
// （`extract_token_from_headers` + `validate_session`，auth/bind.rs 走的就是这条路）。
// 尾部**不带** \b：门禁助手的名字会不断长出后缀（require_admin_opt / require_actor_opt /
// u4_require_actor_of …），写全名单必然慢半拍——本批就出现过「代码已修、读数仍挂 R6」两轮。
// 改成前缀匹配，凡 `require_admin*` / `require_owner*` / `require_actor*` / `require_session*`
// / `u4_*` / `u2_require*` / `guard_write*` 等都算体内门禁。
const GATE_RE =
  /\b(?:require_admin|require_owner|require_actor|require_session|u4_session|u4_require|u2_require|u2_check_owner|owner_gate|is_admin|check_permission|assert_admin|require_role|guard_write|validate_session|extract_token_from_headers)/;
const fnCache = new Map();
function fnBody(crate, handler) {
  const key = `${crate}::${handler}`;
  if (fnCache.has(key)) return fnCache.get(key);
  let body = "";
  const dir = path.join(CRATES, crate, "src");
  if (fs.existsSync(dir)) {
    for (const file of walk(dir)) {
      const src = fs.readFileSync(file, "utf8");
      const m = new RegExp(`\\b(?:pub(?:\\([^)]*\\))?\\s+)?(?:async\\s+)?fn ${handler}\\s*\\(`).exec(src);
      if (!m) continue;
      const tail = src.slice(m.index);
      const end = tail.indexOf("\n}");
      body = end > 0 ? tail.slice(0, end + 2) : tail.slice(0, 20000);
      break;
    }
  }
  fnCache.set(key, body);
  return body;
}
const writeRoutes = routes.filter((r) => ["POST", "PUT", "DELETE", "PATCH"].includes(r.method));
const rb = new Map();
const rAnon = [], rAuthNoSession = [];
for (const r of writeRoutes) {
  const body = fnBody(r.crate, r.handler);
  const admin = mwAdmin(r.method, r.path);
  const exempt = isAuthExempt(r.method, r.path);
  const gate = GATE_RE.test(body);
  const sess = SESSION_RE.test(body);
  let bucket;
  if (admin) bucket = "R1-中间件 admin";
  else if (exempt && gate) bucket = "R2-匿名可达+体内门禁";
  else if (exempt && sess) bucket = "R3-匿名可达+读会话";
  else if (exempt) bucket = "R4-匿名可达+体内无门禁";
  else if (gate) bucket = "R5-已认证+体内门禁";
  else if (sess) bucket = "R6-已认证+读会话无门禁";
  else bucket = "R7-已认证+体内无会话无门禁";
  rb.set(bucket, (rb.get(bucket) || 0) + 1);
  const line = `${r.crate}\t${r.method}\t${r.path}\t${r.handler}`;
  if (bucket === "R4-匿名可达+体内无门禁") rAnon.push(line);
  if (bucket === "R7-已认证+体内无会话无门禁") rAuthNoSession.push(line);
}
console.log(`\n路由级：写路由 ${writeRoutes.length} 条（全路由 ${routes.length}）`);
for (const [k, v] of [...rb].sort((a, b) => a[0].localeCompare(b[0]))) console.log(`  ${String(v).padStart(4)}  ${k}`);
console.log(`\nR4 匿名可达且体内无门禁：${rAnon.length} 条`);
console.log(rAnon.join("\n"));
fs.mkdirSync(ARTIFACTS, { recursive: true });
fs.writeFileSync(
  path.join(ARTIFACTS, "write_routes_ungated.tsv"),
  ["crate\tmethod\tpath\thandler\tbucket",
    ...rAnon.map((l) => `${l}\tR4`),
    ...rAuthNoSession.map((l) => `${l}\tR7`)].join("\n") + "\n",
);
console.log(`\nR4(${rAnon.length}) + R7(${rAuthNoSession.length}) 明细已写 write_routes_ungated.tsv`);

// ── 5. GET 上挂的写（A3-N12 跨切族的活读数）─────────────────────────────────
// csrf_middleware 与 requires_admin 只认 POST/PUT/PATCH/DELETE，本段的 R4/R7 分母
// 同样只扫这四个方法：于是注册在 GET 上的状态写对两把中间件和尺子同时隐形。
// 这里单独枚举「GET/HEAD/OPTIONS 路由 + handler 体内含写 SQL」，只点名无门禁的那些。
const WRITE_SQL_RE = /\b(INSERT\s+INTO|UPDATE\s+\w+|DELETE\s+FROM)\b/i;
const getWrites = [];
for (const r of routes) {
  if (["POST", "PUT", "DELETE", "PATCH"].includes(r.method)) continue;
  const body = fnBody(r.crate, r.handler);
  if (!WRITE_SQL_RE.test(body)) continue;
  if (GATE_RE.test(body)) continue;
  const sess = SESSION_RE.test(body);
  const bucket = isAuthExempt(r.method, r.path)
    ? "K1-匿名可达无门禁"
    : sess
      ? "K2-读会话无门禁"
      : "K3-已认证无会话无门禁";
  getWrites.push(`${r.crate}\t${r.method}\t${r.path}\t${r.handler}\t${bucket}`);
}
const kb = new Map();
for (const l of getWrites) kb.set(l.split("\t")[4], (kb.get(l.split("\t")[4]) || 0) + 1);
console.log(`\nGET 写（csrf / requires_admin 双盲区）：体内无门禁 ${getWrites.length} 条`);
for (const [k, v] of [...kb].sort((a, b) => a[0].localeCompare(b[0]))) console.log(`  ${String(v).padStart(4)}  ${k}`);
fs.writeFileSync(
  path.join(ARTIFACTS, "write_routes_get_writes.tsv"),
  ["crate\tmethod\tpath\thandler\tbucket", ...getWrites].join("\n") + "\n",
);
console.log("明细已写 write_routes_get_writes.tsv");

