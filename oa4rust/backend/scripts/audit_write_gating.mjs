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
// 缺陷 9 修正（四轮 §三.7）：注册表原本按 `registry.register("…", PermissionLevel::Public)`
// 一条正则读，而 rbac.rs 里实际有三种注册口——`register_exact` / `register_prefix` /
// `register`，并且大量调用是**跨行**书写（`register_prefix(\n  "…",\n  PermissionLevel::Public,\n)`）。
// 于是 Public 集合被系统性读漏，直接产出两类错：真匿名面（A）偏低、「豁免 ∧ ¬Public」的
// 死豁免面（D）偏高——首跑把 `/api/queryview/list`、`/api/person/regist`、登录元数据路径
// 这些生产明明活着的路由报成死路由。现按 `get_permission` 的真实语义复刻：
// exact 表优先命中，否则取 `starts_with` 命中的**最长**前缀（裸 starts_with，无段边界）。
const EXACT_LEVELS = new Map();
const PREFIX_LEVELS = [];
for (const m of rsrc.matchAll(
  /registry\.register(_exact|_prefix)?\s*\(\s*"([^"]+)"\s*,\s*PermissionLevel::(\w+)/g
)) {
  const [, kind, p, level] = m;
  if (kind === "_prefix") PREFIX_LEVELS.push([p, level]);
  else EXACT_LEVELS.set(p, level); // `register` 在 Rust 侧不存在，出现即按 exact 处理
}
function registryPublic(p) {
  if (EXACT_LEVELS.get(p) === "Public") return `exact:${p}`;
  let best = null;
  for (const [pre, level] of PREFIX_LEVELS) {
    if (level !== "Public") continue;
    if (p.startsWith(pre) && (!best || pre.length > best.length)) best = pre;
  }
  return best ? `prefix:${best}` : null;
}
// 复刻 get_permission 的完整判定（含非 Public 级别），用于给 D 桶点名「两份权威互相打架」
function registryLevel(p) {
  if (EXACT_LEVELS.has(p)) return `exact:${EXACT_LEVELS.get(p)}`;
  let best = null;
  for (const [pre, level] of PREFIX_LEVELS) {
    if (p.startsWith(pre) && (!best || pre.length > best[0].length)) best = [pre, level];
  }
  return best ? `prefix:${best[0]}→${best[1]}` : "未注册（默认需 Session）";
}
console.log(
  `注册表自证：解析到 ${EXACT_LEVELS.size} 条 exact + ${PREFIX_LEVELS.length} 条 prefix 注册（含跨行与 register_exact/register_prefix 三种写法）`
);

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
// 缺陷 7 + 8 修正（四轮 §三.7）：路由表原本是**一条正则**，先天量不了两类事实：
// ① 正则内层写成裸 `[\s\S]{0,400}?`，会跨过相邻 `.route()` 的边界去找第一个 `)\s*\)`，
//    于是把下一条路由的 handler 配到本条路径上（实测 `register` 被配成
//    `POST /api/reset/password/anonymous`，它真实注册在 `/api/person/regist`），
//    而被吞掉的那条 `.route()` 因 lastIndex 已越过，整条不再入分母；
// ② 终止符 `)\s*\)` 不容忍 Rust 尾逗号，rustfmt 常见的多行 `.route("p", get(a),)`
//    根本匹配不上——全仓 `.route(` 出现 4723 处，旧正则只匹配 3870 处
//    （`bbs_assemble_control` 132 处只匹配 6、`calendar_assemble_control` 34→1、
//    `hotpic_assemble_control` 35→4、`jpush_assemble_control` 30→10）。
// ③ 更麻烦的是「路径由表达式算出来」的注册：`&fmt("user/role/{id}")`（前缀常量拼接，46 处）
//    与 `format!("{p}/design/search").as_str()`（let 绑定前缀，24 处）首参不是字面量，
//    正则直接跳过——这两族的写路由**从来没有进过任何分母**。
// 现在改成按实参扫描：读第一个顶层实参（容忍字符串转义与嵌套括号）解析成路径，
// 再在第二个实参上逐个枚举链式方法（`.route("p", delete(a).put(b))` 只取首方法的
// 旧写法会让链上第二个方法静默消失——四轮实测漏掉 processing 的 work_edit /
// work_delete / att_edit_id 三把）。解析不出来的首参**单独点名计数**，不静默丢弃。
const METHOD_CHAIN_RE = /(?:^|[^A-Za-z0-9_])(get|post|put|delete|patch|head|options)\s*\(\s*((?:[A-Za-z_][A-Za-z0-9_]*::)*[A-Za-z_][A-Za-z0-9_]*)/gi;
const routes = []; // {crate, file, line, path, method, handler}
const seenRouteFiles = [];
const unresolvedPaths = [];

// 文件级常量表：`const API_BASE`、`let/const p = "..."`，用于把拼接路径还原成真实 URI
function fileCtx(src) {
  const base = /const\s+API_BASE\s*:\s*&str\s*=\s*"([^"]+)"/.exec(src);
  const binds = new Map();
  for (const m of src.matchAll(/\b(?:let|const)\s+([A-Za-z_][A-Za-z0-9_]*)\s*(?::\s*&?str\s*)?=\s*"([^"]+)"/g)) {
    binds.set(m[1], m[2]);
  }
  return { base: base ? base[1] : null, binds };
}
// 复刻 bbs 的 fmt()/normalize_legacy_path()：去空段 + 去相邻重复段，再挂到 API_BASE 下
function legacyJoin(base, rel) {
  const segs = [];
  for (const s of `${base}/${rel}`.split("/")) {
    if (!s || segs[segs.length - 1] === s) continue;
    segs.push(s);
  }
  return `/${segs.join("/")}`;
}
function resolvePath(raw, ctx) {
  const arg = raw.trim();
  const lit = /^"([^"]*)"$/.exec(arg);
  if (lit) return lit[1];
  const fm = /^&?fmt\(\s*"([^"]+)"\s*\)$/.exec(arg);
  if (fm && ctx.base) return legacyJoin(ctx.base, fm[1]);
  const fstr = /^format!\(\s*"([^"]+)"\s*\)(?:\.as_str\(\))?$/.exec(arg);
  if (fstr) {
    // Rust 的 `format!` 里 `{{`/`}}` 是字面花括号（`/table/{{flag}}/insert` → 路径段 `{flag}`），
    // 只有单花括号才是插值。顺序：先把 `{{`/`}}` 保护成哨兵，再换绑已知变量，
    // 残留的 `{x}` 就是解不出的前缀 → 返回 null；最后把哨兵还原成 `{`/`}`。
    const LB = "\u0000", RB = "\u0001";
    let s = fstr[1].replace(/\{\{/g, LB).replace(/\}\}/g, RB);
    s = s.replace(/\{(\w+)\}/g, (m, k) => (ctx.binds.has(k) ? ctx.binds.get(k) : m));
    if (/\{\w+\}/.test(s)) return null;
    return s.replace(new RegExp(LB, "g"), "{").replace(new RegExp(RB, "g"), "}");
  }
  return null;
}
// 读 `.route(` 之后的一个顶层实参：尊重字符串转义与嵌套括号，深度 0 的逗号/闭括号分隔，
// 并跳过 Rust 行注释与块注释（`personal/src/lib.rs` 将说明注释直接写在 `.route(` 的参数区里，
// 不跳会把注释当成路径的一部分）。
function readArg(src, i) {
  let depth = 0, inStr = false, esc = false, arg = "";
  for (; i < src.length; i++) {
    const c = src[i], d = src[i + 1];
    if (inStr) {
      arg += c;
      if (esc) esc = false;
      else if (c === "\\") esc = true;
      else if (c === '"') inStr = false;
      continue;
    }
    if (c === "/" && d === "/") { const n = src.indexOf("\n", i); i = n < 0 ? src.length : n - 1; continue; }
    if (c === "/" && d === "*") { const n = src.indexOf("*/", i + 2); i = n < 0 ? src.length : n + 1; continue; }
    if (c === '"') { inStr = true; arg += c; continue; }
    if (c === "(" || c === "[" || c === "{") { depth++; arg += c; continue; }
    if (c === ")" || c === "]" || c === "}") {
      if (depth === 0) return [arg, i + 1, true];
      depth--; arg += c; continue;
    }
    if (c === "," && depth === 0) return [arg, i + 1, false];
    arg += c;
  }
  return [arg, i, false];
}
const URI_SHAPE = /^\/[A-Za-z0-9_./{}:%~ -]*$/; // 允许空格/百分号：attendance 确有
// `/…/check/ from/out` 与 `/…/check/%20from/out` 这种带空格的注册路径（见该文件说明注释）
for (const file of walk(CRATES)) {
  const src = fs.readFileSync(file, "utf8");
  if (!src.includes(".route(")) continue;
  seenRouteFiles.push(file);
  const crate = path.relative(CRATES, file).split(path.sep)[0];
  const ctx = fileCtx(src);
  let at = 0;
  while ((at = src.indexOf(".route(", at)) >= 0) {
    const line = src.slice(0, at).split("\n").length;
    // 站点本身在行注释/文档注释里（如 attendance routes.rs 的动词对齐说明）→ 不是注册
    if (src.slice(src.lastIndexOf("\n", at) + 1, at).includes("//")) { at += 7; continue; }
    const [rawPath, afterPath] = readArg(src, at + 7);
    const [rawHandlers] = readArg(src, afterPath);
    at = afterPath;
    const rpath = resolvePath(rawPath, ctx);
    // 形状不合法（含注释/字符串残片）或前缀变量解不出 → 单独点名，绝不入表也不静默丢
    if (rpath === null || !URI_SHAPE.test(rpath)) {
      unresolvedPaths.push(`${crate}\t${path.basename(file)}:${line}\t${rawPath.trim().replace(/\s+/g, " ").slice(0, 80)}`);
      continue;
    }
    for (const cm of rawHandlers.matchAll(METHOD_CHAIN_RE)) {
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

// 分母自证：注册点总数（去掉注释行）与入表条数、以及「首参解析不了」的残留必须当面说出来
const rawRouteSites = seenRouteFiles.reduce(
  (n, f) =>
    n +
    fs
      .readFileSync(f, "utf8")
      .split("\n")
      .filter((l) => l.includes(".route(") && !l.trimStart().startsWith("//")).length,
  0
);
console.log(
  `\n路由表自证：注册点 ${rawRouteSites} 处 → 入表 ${routes.length} 条（方法级），首参解析不了 ${unresolvedPaths.length} 处`
);
if (unresolvedPaths.length) console.log(unresolvedPaths.join("\n"));

// handler 名 → 路由（同名 handler 可能被多个 crate/路径复用）
const byHandler = new Map();for (const r of routes) {
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
  /\b(?:require_admin|require_owner|require_actor|require_session|u4_session|u4_require|u2_require|u2_check_owner|owner_gate|is_admin|check_permission|assert_admin|require_role|guard_write|validate_session|extract_token_from_headers|resolve_current_person)/;
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
// 尺子缺陷 10（本批修）：R2/R3/R4 的「匿名可达」旧写法只看第一把锁 `isAuthExempt`，
// 与 F1 之后的 auth 层实际判据不符——现在匿名打得通必须是「豁免 ∧ 注册表 Public」两把锁同开，
// 只命中豁免表的路由其实**要求有效会话**，应当落进 R5/R6/R7 那一族。
// 不修的话 R4 会把「需登录但体内无门禁」（可修的行动面）和「中间件已兜住登录」混成一张名单。
function anonReach(method, p) {
  return isAuthExempt(method, p) && registryPublic(p);
}
const rb = new Map();
const rAnon = [], rAuthNoSession = [];
const routeMatrix = []; // 全 7 桶逐条留档，见下方 write_routes_matrix.tsv 的说明
for (const r of writeRoutes) {
  const body = fnBody(r.crate, r.handler);
  const admin = mwAdmin(r.method, r.path);
  const exempt = anonReach(r.method, r.path);
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
  routeMatrix.push(`${line}\t${bucket}`);
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

// `write_routes_ungated.tsv` 只留 R4/R7 两桶（行动面），所以**桶间迁移无法从产物对账**：
// 上一批把 GATE_RE 改成前缀匹配、本批把「匿名可达」改成双锁判据时，几百条路由在 R5/R6/R7 之间
// 搬家，而产物里只看得到 R7 的进出——据此一度误判成「70 条路由从分母里消失」。
// 全 7 桶逐条留档，迁移才能被机械核对（同一 `(crate, method, path)` 在两版产物里换桶 = 重分类；
// 整行消失 = 解析回归，必须当场查）。
fs.writeFileSync(
  path.join(ARTIFACTS, "write_routes_matrix.tsv"),
  ["crate\tmethod\tpath\thandler\tbucket", ...routeMatrix.sort()].join("\n") + "\n",
);
const r7byCrate = new Map();
for (const l of rAuthNoSession) {
  const c = l.split("\t")[0];
  r7byCrate.set(c, (r7byCrate.get(c) || 0) + 1);
}
console.log(
  `R7 无会话无门禁写面按 crate 集中度（前 10）：` +
  [...r7byCrate].sort((a, b) => b[1] - a[1]).slice(0, 10).map(([k, v]) => `${k} ${v}`).join("、")
);
console.log(`全 7 桶矩阵 ${routeMatrix.length} 行已写 write_routes_matrix.tsv`);

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

// ── 6. 豁免通配段碰撞普查（AUTH_EXEMPT_PATHS 的 {param} 吞掉同层字面量路由）────
// 中间件只对**实际请求路径**做段匹配（security.rs `path_matches`：段数相等 + 模式段
// 以 `{` 开头即通配），它并不知道也不想这条路由注册时那段是字面量。
// 于是「为只读详情写的豁免 `/api/x/invoice/{id}`」会把 `/api/x/invoice/upload`
// 整条匿名放行——本批 R4 里的 `invoice_upload`（匿名 INSERT + creator 取 body）就是这么来的。
// 这里按同一口径复刻，只点名「豁免命中里至少有一个通配段正对着路由的字面量段」的碰撞，
// 并按体内是否含写 SQL / 是否读会话 / 是否有门禁分三档。
function exemptCollisions(p) {
  const a = segs(p);
  const out = [];
  for (const pat of EXEMPT_PATHS) {
    const b = segs(pat);
    if (a.length !== b.length || !pathMatches(p, pat)) continue;
    const eaten = [];
    for (let i = 0; i < a.length; i++) {
      if (b[i].startsWith("{") && !a[i].startsWith("{")) eaten.push(`${b[i]}→${a[i]}`);
    }
    if (eaten.length) out.push(`${pat} [${eaten.join(", ")}]`);
  }
  return out;
}
const collisions = [];
const cb = new Map();
for (const r of routes) {
  const eatenBy = exemptCollisions(r.path);
  if (!eatenBy.length) continue;
  const body = fnBody(r.crate, r.handler);
  const write = WRITE_SQL_RE.test(body);
  const gate = GATE_RE.test(body);
  const sess = SESSION_RE.test(body);
  const bucket = write
    ? gate || sess
      ? "W2-碰撞豁免+含写 SQL 但体内读会话或有门禁"
      : "W1-碰撞豁免+含写 SQL 且体内无会话无门禁"
    : "W3-碰撞豁免+只读";
  cb.set(bucket, (cb.get(bucket) || 0) + 1);
  collisions.push(
    `${r.crate}\t${r.method}\t${r.path}\t${r.handler}\t${bucket}\t${eatenBy.join(" ; ")}`
  );
}
console.log(`\n豁免通配段碰撞：${collisions.length} 条路由被「本意给只读详情的 {param} 豁免」匿名放行`);
for (const [k, v] of [...cb].sort((a, b) => a[0].localeCompare(b[0]))) console.log(`  ${String(v).padStart(4)}  ${k}`);
console.log(collisions.filter((l) => l.includes("W1")).join("\n"));
fs.writeFileSync(
  path.join(ARTIFACTS, "auth_exempt_wildcard_collisions.tsv"),
  ["crate\tmethod\tpath\thandler\tbucket\teaten_by", ...collisions].join("\n") + "\n",
);
console.log("明细已写 auth_exempt_wildcard_collisions.tsv");

// 6b. 真匿名面：双锁模型（缺陷 5 修正）
// 旧口径把「auth 层豁免」当成「匿名可达」，但请求还要过 `authorize_middleware`：
// 它只在**注册表判定为 Public** 时早退放行，否则无 Session 一律 401
// （rbac.rs `authorize_middleware`：`level == Some(Public)` → next.run，否则取
// `request.extensions()::<Session>`，取不到就 `Unauthorized`）。
// 所以「匿名可达」= `is_auth_exempt(path)` **∧** 注册表命中 Public，两把锁都要开。
// `AUTH_EXEMPT_PATHS` 只开了第一把，之前 R4/K1 里那些「只读详情豁免被通配段吞掉」的
// 写路由其实被第二把锁兜住了（`invoice_upload` 实测 401，见 §三.7）——旧读数把两类混在一起报，
// 既虚高了行动面，也掩盖了真正两把锁同开的少数几条。
// 另有 `SECRET_INIT_PATHS` 的条件锁：命中它且系统已初始化时，auth 层不再豁免（token.rs:142-147）。
const SECRET_INIT = readConstList(csrc, "SECRET_INIT_PATHS");
function anonState(p) {
  const exempt = isAuthExempt(null, p);
  const pub = registryPublic(p);
  const secret = SECRET_INIT.some((q) => pathMatches(p, q));
  if (!exempt || !pub) return null;
  return secret ? "条件匿名（仅未初始化窗口）" : "真匿名";
}
const anonWrites = [];
const ab = new Map();
for (const r of routes) {
  const state = anonState(r.path);
  if (!state) continue;
  const body = fnBody(r.crate, r.handler);
  const write = WRITE_SQL_RE.test(body);
  const guarded = GATE_RE.test(body) || SESSION_RE.test(body);
  const bucket = write && !guarded
    ? `A1-${state}+体内写 SQL 且无门禁无会话`
    : write
      ? `A2-${state}+体内写 SQL 但读会话或有门禁`
      : `A3-${state}+只读`;
  ab.set(bucket, (ab.get(bucket) || 0) + 1);
  anonWrites.push(`${r.crate}\t${r.method}\t${r.path}\t${r.handler}\t${bucket}`);
}
console.log(`\n真匿名可达面（auth 豁免 ∧ 注册表 Public）：${anonWrites.length} 条`);
for (const [k, v] of [...ab].sort((a, b) => a[0].localeCompare(b[0]))) console.log(`  ${String(v).padStart(4)}  ${k}`);
console.log(anonWrites.filter((l) => l.includes("\tA1-")).join("\n"));
fs.writeFileSync(
  path.join(ARTIFACTS, "anonymous_write_routes.tsv"),
  ["crate\tmethod\tpath\thandler\tbucket", ...anonWrites].join("\n") + "\n",
);
console.log("明细已写 anonymous_write_routes.tsv");

// A1 里「真匿名（不受初始化窗口约束）+ 体内写 SQL + 无门禁无会话」是匿名侧唯一真能落笔的
// 行动面，必须逐条点名。断言写成**集合相等**而非「数量 ≤ N」：新增一条匿名写路由时尺子必须
// 红到有人把它显式写进名单（并说明为什么可以公开），名单里的条目被修掉时也必须红一次。
// `GET /api/secret/set/cancel` 曾因不在 `SECRET_INIT_PATHS` 里落进这一格（补锁后转条件匿名），
// 就是这个名单要盯的形状。
const A1_UNCONDITIONAL_WRITE = [
  "auth\tPOST\t/api/authentication",
  "auth\tPOST\t/api/authentication/login",
  "personal\tPOST\t/api/person/regist",
];
const a1Now = anonWrites
  .filter((l) => l.includes("\tA1-真匿名+体内写 SQL 且无门禁无会话"))
  .map((l) => l.split("\t").slice(0, 3).join("\t"));
const a1Extra = a1Now.filter((k) => !A1_UNCONDITIONAL_WRITE.includes(k));
const a1Gone = A1_UNCONDITIONAL_WRITE.filter((k) => !a1Now.includes(k));
console.log(
  `\nA1 真匿名写行动面（点名名单）：实测 ${a1Now.length} 条 / 名单 ${A1_UNCONDITIONAL_WRITE.length} 条`
);
for (const k of a1Extra) console.log(`  未登记的新匿名写面：${k.replace(/\t/g, " ")}`);
for (const k of a1Gone) console.log(`  名单里已消失的条目：${k.replace(/\t/g, " ")}`);
if (a1Extra.length || a1Gone.length) {
  console.error("A1 真匿名写行动面与点名名单不一致——要么新增匿名写路由，要么有一条被修掉却没更新名单");
  process.exitCode = 1;
}

// ── 6c. 两份权威的差集（auth 豁免 ∧ ¬注册表 Public）──────────────────────────
// 这一桶的**含义取决于 token.rs 的 auth 层是否只按豁免表早退**，所以尺子自己把这个
// 事实从代码里读出来（`F1_WIRED`），而不是把某一种语义钉死在桶名上：
//  · 未接线（四轮 §三.7 的旧态）：auth 命中豁免就早退且**不插 Session**，authorize 又只
//    在注册表 Public 时放行（rbac.rs:538-544）⇒ 半开的双锁，这批路由对**已登录的 admin
//    也永久 401**（实测：带真会话打 `POST /api/general/assemble/control/invoice/create`）。
//    可用性缺陷，不是安全兜底。
//  · 已接线（现况，F1）：auth 层要求「豁免 ∧ 注册表 Public」才早退 ⇒ 差集里的豁免条目
//    是**惰性条目**（豁免了个寂寞），路由回归「需登录」；`tests/exempt_authorize_parity.rs`
//    逐条实测这局面成立（红名单 0 条）。
// 真正的残留风险变成组合式的：哪天有人把这些路径（或其前缀）标成 Public，惰性条目会
// 瞬间变成匿名可达——所以 D1（体内写 SQL）那批仍是需要盯的名单，只是定性从"缺陷"变"引信"。
// （`SECRET_INIT_PATHS` 命中的按设计只算初始化窗口，单列 D3。）
const TOKEN = path.join(CRATES, "shared/src/middleware/token.rs");
const F1_WIRED = /is_auth_exempt\(&path\)\s*&&[\s\S]{0,240}?is_registry_public/.test(
  fs.readFileSync(TOKEN, "utf8"),
);
const deadLabel = F1_WIRED ? "惰性豁免条目（需登录，豁免本身失效）" : "死态：所有用户（含已登录）不可达";
const deadExempt = [];
const db = new Map();
for (const r of routes) {
  if (!isAuthExempt(r.method, r.path)) continue;
  if (registryPublic(r.path)) continue;
  const secret = SECRET_INIT.some((q) => pathMatches(r.path, q));
  const who = EXEMPT_PATHS.find((pat) => pathMatches(r.path, pat))
    || `prefix:${EXEMPT_PREFIXES.find((pre) => segPrefix(r.path, pre))}`;
  const body = fnBody(r.crate, r.handler);
  const write = WRITE_SQL_RE.test(body);
  const bucket = secret
    ? "D3-条件豁免（仅初始化窗口），窗口外需 token"
    : write
      ? `D1-${deadLabel}+体内写 SQL`
      : `D2-${deadLabel}+只读`;
  db.set(bucket, (db.get(bucket) || 0) + 1);
  deadExempt.push(
    `${r.crate}\t${r.method}\t${r.path}\t${r.handler}\t${bucket}\t${who}\t${registryLevel(r.path)}`
  );
}
console.log(
  `\n豁免 ∧ ¬Public 差集（F1 接线自证：${F1_WIRED ? "已接线 → 差集=惰性豁免条目" : "未接线 → 差集=全员 401 死态"}）：${deadExempt.length} 条`
);
for (const [k, v] of [...db].sort((a, b) => a[0].localeCompare(b[0]))) console.log(`  ${String(v).padStart(4)}  ${k}`);
fs.writeFileSync(
  path.join(ARTIFACTS, "dead_exempt_routes.tsv"),
  ["crate\tmethod\tpath\thandler\tbucket\teaten_by\tregistry_level", ...deadExempt].join("\n") + "\n",
);
console.log("明细已写 dead_exempt_routes.tsv");

