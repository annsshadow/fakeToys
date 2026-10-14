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
        // 缺陷 11：`handler` 只用于与 write_handlers.tsv 的短名对账和产物展示；
        // 取体内信号必须用**带模块限定符**的 `handlerRef`（见 fnBody）。
        handler: cm[2].split("::").pop(),
        handlerRef: cm[2],
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
// 缺陷 13 补的词：宏模板里的属主/写者门（gate_doc_writer / gate_topic_owner / …）。
// 全仓 `gate_` 前缀普查只有 6 个标识符，其中 gate_sql / gate_table 是**变量名不是门**，
// 所以这里绝不写 `\bgate_` 前缀，只逐个点名（闭集，新增须同步）。
// 缺陷 23（第十四批补）：cms 的归属判定基座 `u2_gate_by_sql` 不在词表里，于是
// 「handler 自己调 u2_gate_by_sql 取门、再把 U2Gate 交给形状 helper」这一族**整体看不见门**——
// 现场是已加门的正档 `categoryinfo_u2_delete` / `viewfieldconfig_u2_delete` 长期挂 R6 无门禁行动面
// （假阳），第十四批新加的 categoryinfo / viewfieldconfig 三把 GET 别名同样进不了有门名单（假阴）。
// 它是逐个点名 SQL 的属主判定入口（admin 旁路 + 非属主 Forbidden），按门计；
// 反向见证钉在「体内确无判定入口」的 document_id_view_count 上，防止词表被扩成恒真。
const GATE_RE =
  /\b(?:require_admin|require_owner|require_actor|require_session|u4_session|u4_require|u2_require|u2_check_owner|u2_gate_by_sql|owner_gate|is_admin|check_permission|assert_admin|require_role|guard_write|validate_session|extract_token_from_headers|resolve_current_person|gate_doc_writer|gate_topic_owner|gate_reply_owner|gate_attachment_owner|gate_appdict_writer)/;
const fnCache = new Map();
// 尺子缺陷 11（本批修）：注册点写的是 `post(u2::review_create_work)`，旧实现入表时把 `u2::`
// 限定符 `.split("::").pop()` 剥掉，`fnBody` 再按「遍历 crates/<crate>/src 命中的**第一个**同名 fn」
// 取体内信号。processplatform_service_processing 的 lib.rs 里躺着一把同名、未注册、未加门的
// 死 handler，且先被遍历命中——于是真实加了门的 /review/create/work 与 /review/create/workcompleted
// 两条路由仍被计成 R7「体内无门禁」。这一族错是双向的：既能把已修报成漏洞（本批实测两条），
// 也能把真漏洞报成已修（同名加门的那个先命中时）。
// 修法：带限定符的引用按 Rust 模块路径直接解析到 `src/<mod>.rs` 或 `src/<mod>/mod.rs`；
// 裸名**维持**旧的全 crate 扫描口径（否则整张矩阵会无谓搬家，掩盖真实迁移）。
// 限定符解不出对应文件时不静默：计入 unresolvedRefs 在末尾当面点名，同时回落旧扫描保证读数不缺席。
const unresolvedRefs = [];
const noBodyRefs = [];
// Rust 允许 `use u2 as u2h;` 给模块起别名（query_service_processing 的 routes 就是这么引的），
// 所以限定符首段还要过一次 crate 内的别名表，否则又会退回「第一个同名 fn」的旧口径。
const aliasCache = new Map();
function moduleAliases(crate) {
  if (aliasCache.has(crate)) return aliasCache.get(crate);
  const m = new Map();
  const dir = path.join(CRATES, crate, "src");
  if (fs.existsSync(dir))
    for (const f of walk(dir))
      for (const am of fs.readFileSync(f, "utf8").matchAll(
        /\buse\s+(?:crate::|self::|super::)?([A-Za-z_][\w]*(?:::[A-Za-z_][\w]*)*)\s+as\s+([A-Za-z_][\w]*)\s*;/g
      ))
        m.set(am[2], am[1].split("::").pop());
  aliasCache.set(crate, m);
  return m;
}
// 尺子缺陷 13（本批修）：缺陷 12 承认了「宏生成的 handler 看不见体内」，但「看不见」不等于「没有」——
// cms 的 data_path_write_handlers!（68 条）、pp 的 dict_set_*/dict_del_*、bbs 的 topic_toggle_endpoints!
// 这三族把 body 写在 macro_rules! 模板里。现按 Rust 语义补一级解析：
// ① 收集本 crate 内模板文本含 `fn $` 的 `macro_rules! X`（只挑会生成函数项的宏）；
// ② 找 `X!` 之后第一个平衡实参组，组内出现裸标识符 <handler> 即认定该 handler 由该宏生成；
// ③ 取宏模板全文作为体内文本，并把这条 (crate, ref) 记入 macroBodies 单独落盘——
//    读数从此可审：靠模板得到的门禁信号必须能被点名，不能混进普通 R5。
const macroBodies = new Set();
const macroBoundBodies = new Set(); // 缺陷 21：其中「拿到自己那一段」的（其余是整张模板，须单独点名）
function balancedGroup(src, open) {
  const close = { "(": ")", "[": "]", "{": "}" }[src[open]];
  if (!close) return null;
  let depth = 0, inStr = false, esc = false;
  for (let i = open; i < src.length; i++) {
    const c = src[i];
    if (inStr) {
      if (esc) esc = false;
      else if (c === "\\") esc = true;
      else if (c === '"') inStr = false;
      continue;
    }
    if (c === '"' || c === "'") { inStr = true; continue; }
    if (c === "(" || c === "[" || c === "{") depth++;
    else if (c === ")" || c === "]" || c === "}") {
      depth--;
      if (depth === 0) return src.slice(open + 1, i);
    }
  }
  return null;
}
const macroCache = new Map();
// ── 尺子缺陷 21（第十二批抓到并修）：宏模板体必须**按生成函数切段**再绑定给 handler ──────
// 缺陷 13 的旧做法是「取整张 `macro_rules!` 模板文本当体内」。模板里每写一个 `pub async fn $x`
// 就生成一把 handler，而一把宏常常一次生成 **5 把**（cms `appdict_depth_family!` 的
// `$anon_read / $auth_read / $put / $post / $del`、`data_path_write_handlers!` 的
// `$mockdel / $del / $put / $create / $mockput`）。整张模板当体内有两个方向的错：
// ① 假绿（致命）：读档 `$anon_read` 借了同模板 `$put` 的门禁信号，被读成「已加门的写面」而退出 K 名单；
// ② 虚报：读档借了 `$del` 的写 SQL，被算进「GET 上挂写」行动面。
// 本批实测 67 条 GET 路由靠 `appdict_data_write` 作证，全部是 ①。
// 修法：把模板按 `fn $param` 切成段，再用「matcher 的 `$` 形参表 × 调用点的顶层实参表」
// 做位置绑定，handler 只拿自己那一段。matcher 含重复展开 `$(` 的宏（bbs `topic_toggle_endpoints`、
// query `touch_endpoints`、org_express 三族）**只生成一种 fn 形状**，整张模板等价于该段，
// 维持旧读法但单独计数，不许混进「段内取证」。绑不上的一律当面点名，不静默回落。
function splitTopLevel(group) {
  const out = [];
  let depth = 0, cur = "";
  for (const c of group) {
    if (c === "(" || c === "[" || c === "{") depth++;
    else if (c === ")" || c === "]" || c === "}") depth--;
    if ((c === "," || c === ";") && depth === 0) {
      if (cur.trim()) out.push(cur.trim());
      cur = "";
      continue;
    }
    cur += c;
  }
  if (cur.trim()) out.push(cur.trim());
  return out;
}
function matcherParams(matcher) {
  if (matcher.includes("$(")) return null; // 重复展开族：单形状模板，整张即该段
  return [...matcher.matchAll(/\$([a-z_]\w*)/g)].map((m) => m[1]);
}
function templateSegments(defText) {
  const segs = [];
  for (const m of defText.matchAll(/\bfn\s+\$([a-z_]\w*)/g)) {
    const paren = defText.indexOf("(", m.index);
    if (paren < 0) continue;
    let i = paren + 1, depth = 1;
    while (i < defText.length && depth > 0) {
      if (defText[i] === "(") depth++;
      else if (defText[i] === ")") depth--;
      i++;
    }
    if (depth !== 0) continue;
    const brace = defText.indexOf("{", i);
    if (brace < 0) continue;
    let b = brace, bd = 0, end = -1;
    while (b < defText.length) {
      if (defText[b] === "{") bd++;
      else if (defText[b] === "}") { bd--; if (bd === 0) { end = b; break; } }
      b++;
    }
    if (end < 0) continue;
    segs.push({ param: m[1], text: defText.slice(m.index, end + 1) });
  }
  return segs;
}
const macroAmbiguous = [];
// 返回 `{ text, bound }`：bound=true 表示拿到的是该 handler 自己那一段。
function macroTemplateFor(crate, name) {
  if (!macroCache.has(crate)) {
    const defs = [];
    const dir = path.join(CRATES, crate, "src");
    if (fs.existsSync(dir))
      for (const f of walk(dir)) {
        const src = fs.readFileSync(f, "utf8");
        for (const m of src.matchAll(/macro_rules!\s*([A-Za-z_][\w]*)\s*\{/g)) {
          // 定义体：从 `{` 起取平衡块（同一把尺的 readArg 只吃 `.route(` 的实参，这里另起一臂）
          const open = m.index + m[0].length - 1;
          let depth = 0, end = -1;
          for (let i = open; i < src.length; i++) {
            if (src[i] === "{") depth++;
            else if (src[i] === "}") { depth--; if (depth === 0) { end = i; break; } }
          }
          if (end < 0) continue;
          const defText = src.slice(m.index, end + 1);
          if (!/\bfn\s+\$/.test(defText)) continue; // 不生成函数项的宏与本尺无关
          const arrow = defText.indexOf("=>");
          const params = matcherParams(arrow > 0 ? defText.slice(open - m.index + 1, arrow) : "");
          const segments = templateSegments(defText);
          const bindings = [];
          let wholeTemplate = false; // 该宏一次调用只生成一种 fn 形状 → 整张模板即体内
          const inv = new RegExp(`\\b${m[1]}!\\s*[([{]`, "g");
          for (const im of src.matchAll(inv)) {
            const g = balancedGroup(src, im.index + im[0].length - 1);
            if (g == null) continue;
            const toks = splitTopLevel(g);
            // 位置绑定按 **matcher 的形参顺序**（不是模板里 `fn $x` 的出现顺序）——
            // `$arity` 可能排在最前（appdict_depth_family）也可能排在最后（data_path_write_handlers），
            // 按段序 +1 偏移会在后者上整体错一档，把写档绑成读档。
            if (params && params.length === toks.length) {
              const map = new Map();
              params.forEach((p, ix) => map.set(p, toks[ix]));
              bindings.push(map);
            } else {
              const names = [];
              for (const t of toks) for (const mm of String(t).matchAll(/\b([a-z_]\w*)\b/gi)) names.push(mm[1]);
              bindings.push(names);
              wholeTemplate = wholeTemplate || segments.length <= 1;
            }
          }
          defs.push({ defText, segments, bindings, params, wholeTemplate, macro: m[1] });
        }
      }
    macroCache.set(crate, defs);
  }
  for (const d of macroCache.get(crate)) {
    const hit = (list) => list.some((v) => new RegExp(`\\b${name}\\b`).test(String(v)));
    if (d.bindings.length === 0 || !hit(d.bindings.flatMap((b) => (Array.isArray(b) ? b : [...b.values()]))))
      continue;
    if (d.segments.length === 1 || d.wholeTemplate) return { text: d.defText, bound: false };
    for (const b of d.bindings) {
      if (Array.isArray(b)) continue;
      for (const [param, concrete] of b)
        if (concrete === name) {
          const seg = d.segments.find((s) => s.param === param);
          if (seg) return { text: seg.text, bound: true };
        }
    }
    macroAmbiguous.push(`${crate}\t${name}\t${d.macro}`);
    return { text: d.defText, bound: false };
  }
  return null;
}
const sliceFn = (src, name) => {  const m = new RegExp(`\\b(?:pub(?:\\([^)]*\\))?\\s+)?(?:async\\s+)?fn ${name}\\s*\\(`).exec(src);
  if (!m) return null;
  const tail = src.slice(m.index);
  const end = tail.indexOf("\n}");
  return end > 0 ? tail.slice(0, end + 2) : tail.slice(0, 20000);
};
function fnBody(crate, ref) {
  const key = `${crate}::${ref}`;
  if (fnCache.has(key)) return fnCache.get(key);
  let body = "";
  const dir = path.join(CRATES, crate, "src");
  const parts = String(ref).split("::").filter(Boolean);
  const name = parts[parts.length - 1];
  const mods = parts.slice(0, -1).filter((p) => p !== "crate" && p !== "self");
  const scanAll = () => {
    for (const file of walk(dir)) {
      const b = sliceFn(fs.readFileSync(file, "utf8"), name);
      if (b) return b;
    }
    return "";
  };
  if (fs.existsSync(dir)) {
    if (!mods.length) {
      body = scanAll();
    } else {
      const al = moduleAliases(crate);
      const variants = [
        ...new Set([
          mods.join("::"),
          mods.map((p) => al.get(p) || p).join("::"),
          // 限定符首段可能就是**另一个 crate**（如 `shared::…::handler`）：按 crate 名换根目录
          ...(fs.existsSync(path.join(CRATES, mods[0], "src")) ? [`@${mods.join("::")}`] : []),
        ]),
      ];
      const tried = [];
      for (const v of variants) {
        const cross = v.startsWith("@");
        const parts = (cross ? v.slice(1) : v).split("::");
        const root = cross ? path.join(CRATES, parts.shift(), "src") : dir;
        for (const f of [path.join(root, ...parts) + ".rs", path.join(root, ...parts, "mod.rs")]) {
          if (!fs.existsSync(f)) continue;
          tried.push(path.relative(BACKEND, f));
          const b = sliceFn(fs.readFileSync(f, "utf8"), name);
          if (b) {
            body = b;
            break;
          }
        }
        if (body) break;
      }
      if (!body) {
        unresolvedRefs.push(
          `${crate}\t${ref}\t${tried.length ? `${tried.join(" / ")} 内无该 fn` : "限定符（含别名）无对应模块文件"}`
        );
        body = scanAll();
      }
    }
  }
  // 尺子缺陷 12（本批修）：宏生成的 handler（`macro_rules!` 展开出 `pub async fn $name`，
  // 实测 bbs 的 18 把 topic toggle、pp 的 16 把、query_service_processing 的 12 把）在全 crate
  // 里**没有字面 fn**，scanAll 也只能返回空串。旧尺子把空 body 一律读成「体内无会话无门禁」→
  // 落进 R7 行动面，等于把「看不见」冒充成「有漏洞」——反向也会漏报（空 body 让 GET-写探测恒假）。
  // 现单独成桶 R8 并落盘点名：本尺对这类路由不背书，只能人工取证。
  if (!body) {
    // 缺陷 13：先试宏模板，仍取不到才承认「体内不可解析」
    const tpl = macroTemplateFor(crate, name);
    if (tpl) {
      body = tpl.text;
      macroBodies.add(`${crate}\t${ref}`);
      if (tpl.bound) macroBoundBodies.add(`${crate}\t${ref}`);
    } else noBodyRefs.push(`${crate}\t${ref}`);
  }
  fnCache.set(key, body);
  return body;
}
// ── 尺子缺陷 15（第十批抓到并修）：门禁在同 crate 的 helper 里，单函数体判据看不见 ──────
// 现场：surface `attachment_u2b_change_order_number` / `_change_site` 已按属主收口
// （`u2_gate_att_or_business_error(...).await?`，该 helper 体内 `u2_check_owner` 且拒时
// `return Err(AppError::Forbidden)`），但旧判据只看 handler 本体的字面量，把这两条读成
// 「读会话无门禁」的行动面。反向风险同样存在：helper 里只查存在性、不拒人，也会被误当门。
// 因此间接作证要**两个条件同时成立**，并把作证的 helper 名逐条落盘（与 macroBodies 同口径）：
//   A. handler 体内以 `.await?` 形式调用它（拒绝会向上传播，不是调了不理返回值）；
//   B. 该 helper 自己体内命中 GATE_RE，且含 `AppError::Forbidden|Unauthorized` 的拒径。
// 只在**同 crate**内解析 helper（跨 crate 限定名不在注册表里出现，留给缺陷 11 的限定符路径）。
const crateFnIndex = new Map();
function indexCrateFns(crate) {
  if (crateFnIndex.has(crate)) return crateFnIndex.get(crate);
  const idx = new Map();
  const dir = path.join(CRATES, crate, "src");
  if (fs.existsSync(dir)) {
    for (const file of walk(dir)) {
      const src = fs.readFileSync(file, "utf8");
      for (const m of src.matchAll(/\b(?:pub\s+)?(?:async\s+)?fn\s+([A-Za-z_]\w*)/g)) {
        const n = m[1];
        if (idx.has(n)) continue;
        const b = sliceFn(src, n);
        if (b) idx.set(n, b);
      }
    }
  }
  crateFnIndex.set(crate, idx);
  return idx;
}
// 调用点的拒绝是否向上传播。两种形态都算（尺子缺陷 17，第十一批修）：
//   ① `f(...).await?` —— 旧判据只认这一种；
//   ② `f(...).await` 且它整个表达式是函数体的**尾表达式**（`.await` 之后到闭括号为空）——
//     helper 的 `Err` 直接成为 handler 的返回值，拒绝同样传播，只是不写 `?`。
// 现场：cms `surface_appdict_*_data_delete` 45+11 条注册点整体转发给 `appdict_data_write`，
// 那把 helper 体内 `gate_appdict_writer(...).await?` 且拒时 `return Err(AppError::Forbidden)`；
// 旧判据因缺 ② 认不出这道门，把已加门的写面读成 R6「读会话无门禁」的行动面。
// 反向风险（helper 里只查存在性、不拒人）由条件 B 与逐条落盘兜住。
function callPropagates(body, name) {
  const re = new RegExp(`\\b${name}\\s*\\(`, "g");
  let m;
  while ((m = re.exec(body))) {
    let depth = 1;
    let i = m.index + m[0].length;
    while (i < body.length && depth > 0) {
      const c = body[i];
      if (c === "(") depth++;
      else if (c === ")") depth--;
      i++;
    }
    if (depth !== 0) continue;
    // 尺子缺陷 19（第十二批修）：旧写法先把 40 字符窗口 `.replace(/\s+/g, "")` 再判
    // `tail.startsWith(".await")`，然后用 **未剥离** 的原串按 `i + 6` 切片去找闭括号。
    // rustfmt 会把长参数调用折行成 `)\n            .await\n        }`，
    // 此时 `)` 与 `.await` 之间已有 12 个以上空白字符，`i + 6` 仍落在空白里，
    // `/^\s*\}/` 面对的是 `.await…` 而非 `}` → 尾表达式臂恒假。
    // 现场：cms `surface_appdict_*_data{,_mockputtopost}` 32 条注册点整体转发给
    // `appdict_data_write`（体内 `gate_appdict_writer(...).await?` 且拒时 `return Err(Forbidden)`），
    // 缺陷 17 补的 ② 一条也没作证，读数仍挂 R6 行动面。
    // 修法：两个臂都在**原始文本**上用同一条正则判，空白位置由 `\s*` 自己吃掉，不再做位置切片。
    const after = body.slice(i, i + 80);
    if (/^\s*\.await\s*\?/.test(after)) return true;
    // 尾表达式：`.await` 之后（可有换行/缩进）就是函数体的闭括号
    if (/^\s*\.await\s*\}/.test(after)) return true;
  }
  return false;
}
// 返回 false（无门禁）、true（本体内直接命中门禁）、或作证的 helper 名（间接门禁）。
// 递归至多 3 层：门常写在「转发 helper」的下一层（handler → data_write → gate_writer）。
//
// 尺子缺陷 20（第十二批抓到并修）：旧写法要求「作证的 helper 体内必须字面含
// `AppError::Forbidden|Unauthorized`」才算它有拒绝能力。现场：attendance 的
// `delete_admin_record` 体内是 `require_admin(pool, session).await?`、
// `delete_owned_record` 体内是 `shared::middleware::require_owner(...).await?`——
// **拒绝由门自己产出并经 `?` 上抛，helper 本体一行 Forbidden 字面量都不写**。
// 于是这 2 把 helper 覆盖的 12+ 张表 292 条注册点整体被读成 R6「读会话无门禁」的行动面，
// 其中 39 条进了本批的「真落笔」队列——照这张表去加门就是白加。
// 修法：把「有拒绝能力」改成两条并列——① 旧的字面量分支（保留，别把已认出的读数赶走）；
// ② helper 体内存在「匹配门词表的调用点且拒绝会向上传播」（`?` 或尾表达式）。
// ② 仍要求传播，所以 `let ok = is_admin(…).await;` 这种「读了标志却照样落笔」不作证。
function propagatesGateName(body) {
  for (const m of body.matchAll(/([A-Za-z_]\w*)\s*\(/g)) {
    const n = m[1];
    if (!GATE_RE.test(n)) continue;
    if (callPropagates(body, n)) return n;
  }
  return null;
}
function helperCanReject(hb) {
  return /AppError::(Forbidden|Unauthorized)/.test(hb) || propagatesGateName(hb) !== null;
}
function gateEvidence(crate, body, depth = 0) {
  if (!body) return false;
  if (GATE_RE.test(body)) return true;
  if (depth >= 3) return false;
  const idx = indexCrateFns(crate);
  for (const m of body.matchAll(/(?<![.\w:])([a-z_]\w*)\s*\(/g)) {
    const n = m[1];
    const hb = idx.get(n);
    if (!hb || hb === body || !helperCanReject(hb)) continue;
    if (!callPropagates(body, n)) continue;
    if (gateEvidence(crate, hb, depth + 1) !== false) return n;
  }
  return false;
}
const indirectGated = [];
const writeRoutes = routes.filter((r) => ["POST", "PUT", "DELETE", "PATCH"].includes(r.method));
// 尺子缺陷 10（本批修）：R2/R3/R4 的「匿名可达」旧写法只看第一把锁 `isAuthExempt`，
// 与 F1 之后的 auth 层实际判据不符——现在匿名打得通必须是「豁免 ∧ 注册表 Public」两把锁同开，
// 只命中豁免表的路由其实**要求有效会话**，应当落进 R5/R6/R7 那一族。
// 不修的话 R4 会把「需登录但体内无门禁」（可修的行动面）和「中间件已兜住登录」混成一张名单。
function anonReach(method, p) {
  return isAuthExempt(method, p) && registryPublic(p);
}
const rb = new Map();
const rAnon = [], rAuthNoSession = [], rUnparsable = [], rMacro = [];
const routeMatrix = []; // 全 7 桶逐条留档，见下方 write_routes_matrix.tsv 的说明
// R 桶按**注册方法**分母（POST/PUT/DELETE/PATCH），所以「handler 体内只有 SELECT、
// 却被挂在写方法上」的 o2server 只读详情档也算 R7——它们是信息面，不是落笔面。
// 本段按体内是否含写 SQL 再切一刀，把 R4/R6/R7 的**真行动面**与「只读挂写方法」分开计数，
// 免得待修队列被虚报（第十批 surface applicationdict 正式路由 24 条就是这么被算进 R7 的）。
// 保守口径：只看本函数体内 SQL，经 helper/跨函数落笔不在此判据内（故只读名单要抽查后才销账）。
const READ_ONLY_ON_WRITE = new Map(); // bucket -> [line]
const NO_BODY_BY_BUCKET = new Map(); // bucket -> 体内取不到的条数
// 尺子缺陷 14（第十批抓到并修）：旧判据 `UPDATE\s+\w+` 认不出 Rust 字符串里的**转义引号表名**
// `UPDATE \x5C"pp_c_attachment\x5C"`（反斜杠不是 \w），于是把真落笔的写 handler 判成「只读」。
// 全仓实测这种写法 25 处（集中在 surface lib.rs 23 处），本批 strict/wide 自证当场喊红抓到 5 条路由。
// 修法是让表名位允许「可选反斜杠 + 可选引号」前缀，仍要求后面跟标识符首字符，
// 免得把散文里的 "update " 也算成写语句。
// 尺子缺陷 22（第十三批抓到并修）：表名走 `format!` 插值时写作 `UPDATE {} SET deleted_at = NOW()`，
// `{` 不是标识符首字符 ⇒ 旧判据认不出。这类写法集中在「表名当参数传」的 helper（cms
// `soft_delete_by_id(pool, table, id)` 一类，全仓 14 个函数这样落笔），handler 体内只有转调，
// 于是连「含写 SQL」都读不到：真落笔少算 14 条、GET 无门禁面少算 10 条（方向是**漏量**）。
// 修法：UPDATE 的表名位再允许一条 `{` 前缀（`{}` 与 `{ident}` 同形状）。
const WRITE_SQL_RE = /\b(INSERT\s+INTO|UPDATE\s+(?:\x5C?["\x60]?[A-Za-z_]\w*|\{)|DELETE\s+FROM)/i;
// 定向自证用：只要体内出现「转义引号表名」写法，WRITE_SQL_RE 就必须判它是写语句
const ESCAPED_TABLE_RE = /(UPDATE|DELETE\s+FROM|INSERT\s+INTO)\s+\x5C["\x60][A-Za-z_]/i;
// 定向自证用（缺陷 22）：体内出现 `format!` 动态表名的写语句，同样必须判它是写语句
const DYNAMIC_TABLE_RE = /(UPDATE|DELETE\s+FROM|INSERT\s+INTO)\s+\{/i;
// 尺子缺陷 16（第十一批抓到并修）：写 SQL 落在同 crate 的 helper（`*_core` 一类）里，
// handler 本体只是转发调用，旧判据（只看本函数体）于是把真落笔的路由读成「只读挂写方法」，
// GET 段同样漏量。现场：surface `workcompleted_list_count_application_*` 注册在 **GET** 上、
// 本体内一条 SQL 都没有，转调的 `..._core` 体内是
// `UPDATE PP_C_WORKCOMPLETED SET "xupdateTime" = NOW() WHERE xid = $1`。
// 判据与缺陷 15 同形，两条件同时成立才算：A. 调用点以 `.await?` 传播（`callPropagates`）；
// B. 被调 helper 体内判为写。递归至多 3 层（helper 再转调 helper），明细逐条落盘。
const indirectWrites = [];
// 调用点是否真把 helper 跑起来：`)` 之后紧跟 `.await`（带不带 `?` 都算）。
// 与缺陷 15 的 `callPropagates` 有意不同：门要「拒径向上传播」才算作证，
// 而写 SQL 只要函数被 await 跑起来就已落笔，`let _ = f().await` 同样拦不住。
function awaitCall(body, name) {
  const re = new RegExp(`\\b${name}\\s*\\(`, "g");
  let m;
  while ((m = re.exec(body))) {
    let depth = 1;
    let i = m.index + m[0].length;
    while (i < body.length && depth > 0) {
      const c = body[i];
      if (c === "(") depth++;
      else if (c === ")") depth--;
      i++;
    }
    if (depth !== 0) continue;
    if (body.slice(i, i + 40).replace(/\s+/g, "").startsWith(".await")) return true;
  }
  return false;
}
function writeEvidence(crate, body, depth = 0) {
  if (!body) return false;
  if (WRITE_SQL_RE.test(body)) return true;
  if (depth >= 3) return false;
  const idx = indexCrateFns(crate);
  for (const m of body.matchAll(/(?<![.\w:])([a-z_]\w*)\s*\(/g)) {
    const n = m[1];
    const hb = idx.get(n);
    if (!hb || hb === body || !awaitCall(body, n)) continue;
    if (writeEvidence(crate, hb, depth + 1)) return n;
  }
  return false;
}
// 尺子缺陷 18（第十一批抓到并修）：注册路径的**参数段数少于 handler 签名里 `Path<…>` 需要的段数**
// 时，axum 在进入函数体之前就抛「Wrong number of path arguments for `Path`」，函数体一条语句都不执行
// ——SQL 够不着，门禁也用不上。旧判据只看「方法 + 体内有无写 SQL」，把这类**死档**算成活的写面：
// 本批实测 9 条 `GET /…/data/job/job[/pathN…]/mockputtopost`（注册时把 `{job}` 槽写成字面量 `job`、
// 又缺 `{id}` 段），据此「GET 写面」虚高 9 条。反向同样危险：一旦有人给死档补上参数段，它立刻变活，
// 所以判据必须可机器复算，不能靠名单。
// 口径：need = **签名**（第一个 `{` 之前）里形如 `: Path<…>` 的参数标注中泛型实参按顶层逗号切段后的非空段数，
// 取各处的最大值；pathParams = 路径里 `{…}` 段数。含 `*` 通配尾段的路径本尺不判（段数语义不同）。
// 两道收窄是踩过坑加的：① 只看签名——body 里 `Vec<u8>`、`impl Future<Output=…>` 这类尖括号
// 会让「找不到闭合 `>`」的前扫一路吞掉十几处逗号，实测把 paging 族读成「需要 16 个参数」；
// ② 括号不计入深度、抽出实参后剥掉 `(` `)` 再数逗号——`Path<(String, String)>` 的元组括号
// 若算深度，元组档会被数成 1，反向漏报。
function pathParamCount(p) {
  return (p.match(/\{[^}]*\}/g) || []).length;
}
function fnSignature(text) {
  const i = text.indexOf("{");
  return i < 0 ? text : text.slice(0, i);
}
function pathExtractArity(sig) {
  let max = 0;
  for (const m of sig.matchAll(/:\s*(?:axum::)?extract::Path\s*<|:\s*Path\s*<\(/g)) {
    const start = m.index + m[0].length - 1; // 指向 `<`
    let depth = 0;
    let end = -1;
    for (let i = start; i < sig.length; i++) {
      const c = sig[i];
      if (c === "<") depth++;
      else if (c === ">") {
        depth--;
        if (depth === 0) { end = i; break; }
      }
    }
    if (end < 0) continue;
    const arg = sig.slice(start + 1, end).replace(/[()]/g, "");
    // 按顶层逗号切段后**丢掉空段**再计数：rustfmt 会把长元组类型拆成
    // `Path<(\n String,\n String,\n)>`，尾巴上那个逗号若按「逗号数 + 1」算会多出一档，
    // 实测把 152 条活路由误判成死档（本尺第一版就栽在这里）。
    let d = 0;
    const parts = [""];
    for (const c of arg) {
      if (c === "<" || c === "[") d++;
      else if (c === ">" || c === "]") d--;
      else if (c === "," && d === 0) parts.push("");
      else parts[parts.length - 1] += c;
    }
    max = Math.max(max, parts.filter((s) => s.trim() !== "").length || 1);
  }
  return max;
}
const deadRows = new Set();
function isDeadAlias(r, body) {
  if (!body || r.path.includes("*")) return false;
  const need = pathExtractArity(fnSignature(body));
  if (need === 0) return false;
  return pathParamCount(r.path) < need;
}
const deadAliasLines = [];
for (const r of routes) {
  const b = fnBody(r.crate, r.handlerRef);
  const need = pathExtractArity(fnSignature(b || ""));
  if (!isDeadAlias(r, b)) continue;
  deadRows.add(`${r.crate}\t${r.method}\t${r.path}`);
  deadAliasLines.push(
    `${r.crate}\t${r.method}\t${r.path}\t${r.handler}\tpath参数段 ${pathParamCount(r.path)} < 签名需要 ${need}`,
  );
}
console.log(`\n死档路由（Path 段数不足，进不到函数体，缺陷 18）：${deadAliasLines.length} 条`);
console.log(deadAliasLines.join("\n"));

for (const r of writeRoutes) {
  const body = fnBody(r.crate, r.handlerRef);
  const admin = mwAdmin(r.method, r.path);
  const exempt = anonReach(r.method, r.path);
  const gateEv = gateEvidence(r.crate, body);
  const gate = gateEv !== false;
  if (typeof gateEv === "string") indirectGated.push(`${r.crate}\t${r.method}\t${r.path}\t${r.handler}\t${gateEv}`);
  const sess = SESSION_RE.test(body);
  let bucket;
  // 缺陷 18：死档先出列——它的门禁/写 SQL 读数都无意义（函数体根本不执行），
  // 但仍要在矩阵里点名，免得「从分母消失」无法对账。
  if (deadRows.has(`${r.crate}\t${r.method}\t${r.path}`)) {
    const dl = `${r.crate}\t${r.method}\t${r.path}\t${r.handler}`;
    rb.set("R9-死档（Path 段数不足，进不到函数体）", (rb.get("R9-死档（Path 段数不足，进不到函数体）") || 0) + 1);
    routeMatrix.push(`${dl}\tR9-死档`);
    continue;
  }
  // 顺序口径：R1（中间件 admin）与 R2-R4（匿名可达）先判——**不能**让「体内不可解析」
  // 把匿名可达的路由从 R4 行动面里摘走（首版把 !body 放在 exempt 之前，实测吞掉 2 条 R4）。
  if (admin) bucket = "R1-中间件 admin";
  else if (exempt && gate) bucket = "R2-匿名可达+体内门禁";
  else if (exempt && sess) bucket = "R3-匿名可达+读会话";
  else if (exempt) bucket = "R4-匿名可达+体内无门禁";
  else if (!body) bucket = "R8-已认证+体内不可解析（宏生成/跨 crate，本尺不背书）";
  else if (gate) bucket = "R5-已认证+体内门禁";
  else if (sess) bucket = "R6-已认证+读会话无门禁";
  else bucket = "R7-已认证+体内无会话无门禁";
  rb.set(bucket, (rb.get(bucket) || 0) + 1);
  const line = `${r.crate}\t${r.method}\t${r.path}\t${r.handler}`;
  routeMatrix.push(`${line}\t${bucket}`);
  // 只切「无门禁」三桶（R4/R6/R7）——R5 已放行侧达标，分它没有行动意义
  if (body && ["R4-匿名可达+体内无门禁", "R6-已认证+读会话无门禁", "R7-已认证+体内无会话无门禁"].includes(bucket)) {
    const wEv = writeEvidence(r.crate, body);
    if (wEv === false) {
      if (!READ_ONLY_ON_WRITE.has(bucket)) READ_ONLY_ON_WRITE.set(bucket, []);
      READ_ONLY_ON_WRITE.get(bucket).push(line);
    } else if (typeof wEv === "string") {
      indirectWrites.push(`${line}\t${bucket.slice(0, 2)}\t${wEv}`);
    }
  }
  if (!body) NO_BODY_BY_BUCKET.set(bucket, (NO_BODY_BY_BUCKET.get(bucket) || 0) + 1);
  if (bucket === "R4-匿名可达+体内无门禁") rAnon.push(line);
  if (bucket === "R7-已认证+体内无会话无门禁") rAuthNoSession.push(line);
  // 缺陷 12 名单：凡体内取不到（含被 R4 抢先归桶的匿名可达路由）一律点名，按桶不重不漏
  if (!body) rUnparsable.push(`${line}\t${bucket.slice(0, 2)}\t${path.basename(r.file)}:${r.line}`);
  // 缺陷 13：靠宏模板得到的读数单独点名，R5 里哪几条是「模板作证」必须可枚举
  if (macroBodies.has(`${r.crate}\t${r.handlerRef}`)) rMacro.push(`${line}\t${bucket}\t${r.handlerRef}`);
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
console.log(`全 ${rb.size} 桶矩阵 ${routeMatrix.length} 行已写 write_routes_matrix.tsv`);

// ── 4b. 无门禁三桶的「体内有写 SQL / 只读挂写方法」拆分 ──────────────────────
// 写进 R7 的 1158 条里，有多少是 handler 体内根本不落笔的只读档？不分这一刀，
// 「还剩 1158 条要修」就是虚报——修法只有对真落笔面才成立。
const readOnlyLines = [];
// 刀口自证（缺陷 14 的守卫）：体内凡出现「转义引号表名」写 SQL 的路由，判据必须认它是写面。
// 这条不变式一旦被新写法破坏（比如换成了本判据没覆盖的第三种 quoting），就当场喊红，
// 而不是留下一个「只读挂写方法 = 803」的虚高数字让后续批次照着修错队列。
{
  const disagree = [];
  let escapedSites = 0;
  for (const r of routes) {
    const b = fnBody(r.crate, r.handlerRef);
    if (!b || !ESCAPED_TABLE_RE.test(b)) continue;
    escapedSites++;
    if (!WRITE_SQL_RE.test(b)) disagree.push(`${r.crate}\t${r.method}\t${r.path}`);
  }
  if (disagree.length)
    throw new Error(
      `写 SQL 判据漏量 ${disagree.length} 条（转义引号表名被判成只读）：\n${disagree.slice(0, 10).join("\n")}`
    );
  console.log(
    `写 SQL 判据自证：含转义引号表名写法的 ${escapedSites} 条路由全部判为写面，0 条漏量`
  );
}
// 刀口自证（缺陷 22 的守卫）：判据要量在 **helper 层**——动态表名写语句几乎都藏在
// 「表名当参数传」的 helper 里（`soft_delete_by_id` 一类），handler 体内只有转调，
// 所以沿 handler 扫既扫不到、也容易随名单漂移。这里直接扫全仓 fn 索引：
// 凡体内出现 `(UPDATE|DELETE FROM|INSERT INTO) {` 的函数，WRITE_SQL_RE 必须判它是写面。
{
  const disagree = [];
  let dynamicSites = 0;
  for (const crate of new Set(routes.map((r) => r.crate))) {
    for (const [n, b] of indexCrateFns(crate)) {
      if (!DYNAMIC_TABLE_RE.test(b)) continue;
      dynamicSites++;
      if (!WRITE_SQL_RE.test(b)) disagree.push(`${crate}\t${n}`);
    }
  }
  if (disagree.length)
    throw new Error(
      `写 SQL 判据漏量 ${disagree.length} 处（format! 动态表名被判成只读）：\n${disagree.slice(0, 10).join("\n")}`
    );
  console.log(
    `写 SQL 判据自证（缺陷 22）：含 format! 动态表名写法的 ${dynamicSites} 个函数全部判为写面，0 处漏量`
  );
}
for (const name of ["R4-匿名可达+体内无门禁", "R6-已认证+读会话无门禁", "R7-已认证+体内无会话无门禁"]) {
  const total = rb.get(name) || 0;
  const lines = READ_ONLY_ON_WRITE.get(name) || [];
  const noBody = NO_BODY_BY_BUCKET.get(name) || 0;
  const real = total - lines.length - noBody;
  if (real + lines.length + noBody !== total)
    throw new Error(`拆分不闭合：${name} 总数 ${total} ≠ 写SQL ${real} + 只读 ${lines.length} + 不可解析 ${noBody}`);
  console.log(
    `${name.slice(0, 2)} 拆分：含写 SQL（本体内或经 helper 转调）${real} 条 / 只读挂写方法 ${lines.length} 条 / 体内不可解析 ${noBody} 条（合计 ${total}）`
  );
  readOnlyLines.push(...lines.map((l) => `${l}\t${name.slice(0, 2)}`));
}
fs.writeFileSync(
  path.join(ARTIFACTS, "write_routes_readonly_on_writemethod.tsv"),
  ["crate\tmethod\tpath\thandler\tbucket", ...readOnlyLines.sort()].join("\n") + "\n",
);
console.log(`只读挂写方法 ${readOnlyLines.length} 条已写 write_routes_readonly_on_writemethod.tsv`);
// 间接门禁的落盘在脚本末尾（缺陷 15）：GET 段（§5）也往里加条目，写早了会静默丢读数。

// ── 5. GET 上挂的写（A3-N12 跨切族的活读数）─────────────────────────────────
// csrf_middleware 与 requires_admin 只认 POST/PUT/PATCH/DELETE，本段的 R4/R7 分母
// 同样只扫这四个方法：于是注册在 GET 上的状态写对两把中间件和尺子同时隐形。
// 这里单独枚举「GET/HEAD/OPTIONS 路由 + handler 体内含写 SQL」，只点名无门禁的那些。
// `WRITE_SQL_RE` 已在 4. 路由级分桶处定义（第十批为 R 桶拆分上提），此处复用。
const getWrites = [];
for (const r of routes) {
  if (["POST", "PUT", "DELETE", "PATCH"].includes(r.method)) continue;
  // 缺陷 18：死档 GET 不算 GET 写面（函数体进不去，SQL 永不执行）
  if (deadRows.has(`${r.crate}\t${r.method}\t${r.path}`)) continue;
  const body = fnBody(r.crate, r.handlerRef);
  const wEv = writeEvidence(r.crate, body);
  if (wEv === false) continue;
  if (typeof wEv === "string")
    indirectWrites.push(`${r.crate}\t${r.method}\t${r.path}\t${r.handler}\tK\t${wEv}`);
  const gEv = gateEvidence(r.crate, body);
  if (gEv !== false) {
    if (typeof gEv === "string") indirectGated.push(`${r.crate}\t${r.method}\t${r.path}\t${r.handler}\t${gEv}`);
    continue;
  }
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
  const body = fnBody(r.crate, r.handlerRef);
  const write = writeEvidence(r.crate, body) !== false;
  const gate = gateEvidence(r.crate, body) !== false;
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
  const body = fnBody(r.crate, r.handlerRef);
  const write = writeEvidence(r.crate, body) !== false;
  const guarded = gateEvidence(r.crate, body) !== false || SESSION_RE.test(body);
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
// 缺陷 16 修好后本名单从 3 条涨到 9 条：新增 6 条全部来自「写 SQL 在 helper 里、
// handler 本体只做转发」的匿名入口（OAuth 三方登录 4 条经 `provider_login`/`oauth_code_store`，
// 密码重置 2 条经 `apply_reset`）。它们**是否算洞**要单独定拍（匿名可达本身是业务要求，
// 要盯的是「能否无界建行」与「重置 token 是否可被第三方读取」），本断言只保证
// 「匿名落笔面不许悄悄变多」——多一条就要有人显式写进名单并说明为什么可以公开。
const A1_UNCONDITIONAL_WRITE = [
  "auth\tPOST\t/api/authentication",
  "auth\tPOST\t/api/authentication/login",
  "auth\tGET\t/api/authentication/oauth/login/qywx/code/{code}",
  "auth\tGET\t/api/authentication/oauth/login/dingding/code/{code}",
  "auth\tGET\t/api/authentication/oauth/login/name/{name}/code/{code}/redirecturi/{redirectUri}",
  "auth\tGET\t/api/authentication/oauth/bind/name/{name}/code/{code}/redirecturi/{redirectUri}",
  "personal\tPOST\t/api/person/regist",
  "personal\tPUT\t/api/reset",
  "personal\tPOST\t/api/reset/password/anonymous",
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
  const body = fnBody(r.crate, r.handlerRef);
  const write = writeEvidence(r.crate, body) !== false;
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

// ── 缺陷 11 自证（限定符解析）─────────────────────────────────────────────────
// 带限定符的注册引用有多少、有没有回落，必须当面读数：回落=0 才算这一族解析全部命中模块文件。
const qualified = routes.filter((r) => r.handlerRef.includes("::"));
console.log(
  `\n限定符解析自证（缺陷 11）：${routes.length} 条路由中 ${qualified.length} 条注册引用带模块限定符` +
    `（去重 ${new Set(qualified.map((r) => `${r.crate}::${r.handlerRef}`)).size} 个），` +
    `解不出模块文件而回落全 crate 扫描 ${unresolvedRefs.length} 处`
);
if (unresolvedRefs.length) console.log(unresolvedRefs.join("\n"));
// 落盘点名：回落清单不留在 stdout 里 evaporate，后续批次要能机械核对它是否变短。
fs.writeFileSync(
  path.join(ARTIFACTS, "route_handler_unresolved_refs.tsv"),
  "crate\thandler_ref\twhy_fell_back\n" + unresolvedRefs.sort().join("\n") + "\n",
);
console.log("回落明细已写 route_handler_unresolved_refs.tsv");
// 缺陷 12：R8 名单同样落盘——这些写路由的体内信号本尺看不见，只能逐条人工取证。
console.log(`\nR8 体内不可解析（缺陷 12，宏生成/跨 crate）：${rUnparsable.length} 条（含被 R1-R4 抢先归桶的 ${rUnparsable.filter((l) => !l.includes("\tR8")).length} 条，按桶点名）`);
fs.writeFileSync(
  path.join(ARTIFACTS, "write_routes_unparsable.tsv"),
  "crate\tmethod\tpath\thandler\tbucket\troute_site\n" + rUnparsable.sort().join("\n") + "\n",
);
console.log("明细已写 write_routes_unparsable.tsv");
// 缺陷 13 自证：宏模板作证的写路由逐条落盘（读数可枚举，不许混进普通 R5）
console.log(`\n宏模板取证（缺陷 13）：${rMacro.length} 条写路由的体内信号取自 macro_rules! 模板`);
{
  const bound = rMacro.filter((l) => macroBoundBodies.has(`${l.split("\t")[0]}\t${l.split("\t")[5]}`)).length;
  console.log(
    `  其中按生成函数切段取证（缺陷 21）${bound} 条 / 整张模板取证（单形状宏，等价）${rMacro.length - bound} 条`
  );
  if (macroAmbiguous.length)
    console.log(`  绑不上段的宏生成 handler ${macroAmbiguous.length} 把（回落整张模板，须人工复核）：` +
      [...new Set(macroAmbiguous.map((l) => l.split("\t")[0] + "/" + l.split("\t")[2]))].join("、"));
}
fs.writeFileSync(
  path.join(ARTIFACTS, "write_routes_macro_resolved.tsv"),
  "crate\tmethod\tpath\thandler\tbucket\thandler_ref\n" + rMacro.sort().join("\n") + "\n",
);
console.log("明细已写 write_routes_macro_resolved.tsv");

// 缺陷 15 的落盘（必须在所有分桶段之后）：间接门禁逐条点名，作证的是哪把 helper 必须可查——
// 这一层的读数不许靠「尺子说它有门」背书。
{
  const helpers = new Map();
  for (const l of indirectGated) {
    const h = l.split("\t")[4];
    helpers.set(h, (helpers.get(h) || 0) + 1);
  }
  fs.writeFileSync(
    path.join(ARTIFACTS, "write_routes_indirect_gated.tsv"),
    ["crate\tmethod\tpath\thandler\tgate_helper", ...indirectGated.sort()].join("\n") + "\n",
  );
  console.log(
    `间接门禁作证 ${indirectGated.length} 条（缺陷 15，helper 体内判定）：` +
    [...helpers].sort((a, b) => b[1] - a[1]).map(([k, v]) => `${k} ${v}`).join("、") +
    `；明细已写 write_routes_indirect_gated.tsv`
  );
}

// 缺陷 16 的落盘（同样必须在所有分桶段之后）：经 helper 转调落笔的路由逐条点名，
// 记的是哪把 helper 体内有写 SQL——「只读」名单不许再靠「本体内看不见」背书。
{
  const helpers = new Map();
  for (const l of indirectWrites) {
    const h = l.split("\t")[5];
    helpers.set(h, (helpers.get(h) || 0) + 1);
  }
  fs.writeFileSync(
    path.join(ARTIFACTS, "write_routes_indirect_writes.tsv"),
    ["crate\tmethod\tpath\thandler\tstage\twrite_helper", ...indirectWrites.sort()].join("\n") + "\n",
  );
  console.log(
    `\n间接写 SQL ${indirectWrites.length} 条（缺陷 16，helper 体内落笔）：` +
    [...helpers].sort((a, b) => b[1] - a[1]).map(([k, v]) => `${k} ${v}`).join("、") +
    "；明细已写 write_routes_indirect_writes.tsv"
  );
}

// 缺陷 18 的落盘（同样排在所有分桶段之后）：死档逐条点名 + 段数对照，
// 「这条 GET 别名是不是真能落笔」从此可复核，不必再靠人肉数 `{}`。
fs.writeFileSync(
  path.join(ARTIFACTS, "routes_dead_alias.tsv"),
  ["crate\tmethod\tpath\thandler\tarity", ...deadAliasLines.sort()].join("\n") + "\n",
);
console.log(
  `\n死档 ${deadAliasLines.length} 条（缺陷 18）已从 R1-R8 与 GET 写面出列，改记 R9；明细已写 routes_dead_alias.tsv`
);

// ── 缺陷 16 / 17 / 18 的见证名单（非恒真自证）────────────────────────────────
// 这三条判据都是「看不见 → 看得见」型的修复，只看一次读数无法证明判据本身成立：
// 把 `awaitCall` 摘掉、把 `gateEvidence` 的递归压回一层、把 `isDeadAlias` 恒假，
// 分母照样能跑完、退出码照样是 0，只有见证名单会空。所以在这里点名，注入即红。
// 每条都取自本轮真实产物，且分属不同 crate，避免「一把 helper 撑满三格」。
const WRITE_HELPER_WITNESSES = [
  "appdict_data_write", // cms：写 SQL 在 helper 里，56 条路由靠它作证
  "workcompleted_list_count_application_applicationFlag_process_core", // surface：GET 上挂的 helper 落笔
  "provider_login", // auth：匿名 OAuth 的写经 helper 转调
];
const GATE_HELPER_WITNESSES = [
  "set_topic_flag", // bbs：门禁在 helper 体内且拒绝经 `.await?` 传播
  "check_owner", // 递归层内作证
  "u2_attachment_guard", // file：同族间接门禁
];
const DEAD_ALIAS_WITNESSES = [
  "processplatform_assemble_surface\tGET\t/api/processplatform/assemble/surface/data/job/job/mockputtopost",
  "processplatform_assemble_surface\tGET\t/api/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/path4/path5/path6/path7/mockputtopost",
  "processplatform_assemble_surface\tGET\t/api/processplatform/assemble/surface/draft/mockputtopost",
  "portal_assemble_surface\tPOST\t/api/portal/assemble/surface/dict/portal/path/data/mockputtopost/{dictFlag}/{portalFlag}",
  "program_center\tDELETE\t/api/program_center/dict/dictFlag/path/data/mockdeletetoget",
  "file_assemble_control\tGET\t/api/file/share/shield/id",
];
const witnessGaps = [];
// 反方向同样要能红：段数判据若把「逗号数 + 1」当成元组长度（rustfmt 换行后的尾逗号会被数成一段），
// 活路由会被误判成死档，行动面凭空缩小。这里点名几把**必须保持可达**的档，多判即红。
const LIVE_ALIAS_WITNESSES = [
  "processplatform_assemble_surface\tPOST\t/api/processplatform/assemble/surface/data/job/{job}/mockputtopost",
  "processplatform_assemble_surface\tPOST\t/api/processplatform/assemble/surface/data/workcompleted/{id}/mockputtopost",
  "processplatform_assemble_surface\tGET\t/api/processplatform/assemble/surface/data/work/{id}",
  "correlation_service_processing\tPOST\t/api/correlation/service/processing/unlink/{sourceType}/{sourceId}/{targetType}/{targetId}",
];
const helperNamesOf = (lines, col) => new Set(lines.map((l) => l.split("\t")[col]));
const wHelpers = helperNamesOf(indirectWrites, 5);
const gHelpers = helperNamesOf(indirectGated, 4);
const routeKeyOf = (l) => l.split("\t").slice(0, 4).join("\t");
const bucketByKey = new Map(routeMatrix.map((l) => { const p = l.split("\t"); return [p.slice(0, 4).join("\t"), p[4]]; }));
const gatedByRoute = new Map(indirectGated.map((l) => [routeKeyOf(l), l.split("\t")[4]]));
// 缺陷 19 正向：`)` 与 `.await` 之间有折行空白（rustfmt 折行 / 宏模板里的多参转调）时，
// 「helper 的 Err 直接成为 handler 返回值」这一路必须仍然作证。摘掉 ② 臂，这四条整体退回 R6。
for (const [key, helper] of [
  ["cms_assemble_control\tDELETE\t/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/data\tsurface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data_delete", "appdict_data_write"],
  ["cms_assemble_control\tPUT\t/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/data\tsurface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data_put", "appdict_data_write"],
  ["cms_assemble_control\tPOST\t/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/data\tsurface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data_post", "appdict_data_write"],
  ["cms_assemble_control\tDELETE\t/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/{path1}/{path2}/data\tsurface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data_delete", "appdict_data_write"],
])
  if (gatedByRoute.get(key) !== helper)
    witnessGaps.push(`缺陷 19 见证缺失（尾表达式转调未被 ${helper} 作证）：${key.replace(/\t/g, " ")}`);
// 缺陷 20 正向：门在 helper 里、helper 本体不写 `AppError::Forbidden` 字面量
// （`require_admin(...).await?` / `shared::middleware::require_owner(...).await?` 把拒绝直接上抛）。
// 把 `helperCanReject` 压回「只认字面量」，这两条当场退回 R6。
for (const [key, helper] of [
  ["attendance_assemble_control\tDELETE\t/api/attendance/assemble/control/attendanceadmin/{id}\tattendanceadmin_delete", "delete_admin_record"],
  ["attendance_assemble_control\tDELETE\t/api/attendance/assemble/control/attendancedetail/{id}\tattendancedetail_delete", "delete_owned_record"],
]) {
  if (gatedByRoute.get(key) !== helper)
    witnessGaps.push(`缺陷 20 见证缺失（无字面量拒绝分支的 helper 未作证 ${helper}）：${key.replace(/\t/g, " ")}`);
  if (!gHelpers.has(helper))
    witnessGaps.push(`缺陷 20 见证缺失（间接门禁名单未见 helper ${helper}）`);
}
// 缺陷 20 的**独立反证尺**：逐条复核「被拿来作证的 helper 真的有拒绝能力」。
// 这里的判据是故意**重新写一遍**的字面量 + 传播两条，不调用 `helperCanReject`——
// 若有人把它放宽成恒真（本批注入臂 T20c 实测：全仓 0 条自然样本会被误证，
// 所以三条点名式反向见证抓不住那次放松），这张尺会因为「作证的 helper 体内既无拒绝字面量、
// 也没有传播的门禁调用」而当场喊红。恒真型放松只能靠独立判据兜，不能靠样本兜。
const LITERAL_REJECT_RE = /AppError::(Forbidden|Unauthorized)/;
for (const l of indirectGated) {
  const p = l.split("\t");
  const hb = indexCrateFns(p[0]).get(p[4]) || "";
  if (!hb) {
    witnessGaps.push(`缺陷 20 独立反证：作证的 helper ${p[0]}::${p[4]} 体内取不到`);
    continue;
  }
  let propagatesGate = false;
  for (const g of hb.matchAll(/([A-Za-z_]\w*)\s*\(/g)) {
    if (!GATE_RE.test(g[1])) continue;
    if (callPropagates(hb, g[1])) { propagatesGate = true; break; }
  }
  if (!LITERAL_REJECT_RE.test(hb) && !propagatesGate)
    witnessGaps.push(`缺陷 20 独立反证：${p[0]} ${p[1]} ${p[2]} 被 ${p[4]} 作证，但该 helper 无拒绝能力`);
}
// 缺陷 19/20 的点名式反向见证：这三条体内确无门禁，读数必须停在 R6/R7，
// 也不许出现在间接门禁名单里（防「把真洞洗成已修」）。
for (const key of [
  "query_assemble_designer\tDELETE\t/api/query/assemble/designer/delete\tdelete_designer_bare",
  "processplatform_assemble_surface\tPOST\t/api/processplatform/assemble/surface/attachment/doc/to/word/work/{workId}\tattachment_u2b_doc_to_word",
  "message_assemble_communicate\tDELETE\t/api/message/assemble/communicate/im/conversation/{id}/group/mockdeletetoget\tim_conversation_id_group_mockdeletetoget",
]) {
  if (gatedByRoute.has(key))
    witnessGaps.push(`缺陷 19/20 反向见证失效（无门路由被误作证 ${gatedByRoute.get(key)}）：${key.replace(/\t/g, " ")}`);
  if (/^R5/.test(bucketByKey.get(key) || ""))
    witnessGaps.push(`缺陷 19/20 反向见证失效（无门路由被误判成 R5）：${key.replace(/\t/g, " ")}`);
}
// 缺陷 21：宏模板按生成函数切段之后，**读档不许借同模板写档的门禁/写 SQL 作证**。
// cms `appdict_depth_family!` 一次生成 5 把（anon_read / auth_read / put / post / del），
// 旧做法取整张模板当体内 → 67 条 anonymous/auth 的 GET 读档被 `appdict_data_write` 读成「已加门的写面」。
// 切段生效后这几条必须从间接门禁名单里消失。
for (const key of [
  "cms_assemble_control\tGET\t/api/anonymous/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/data\tanonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data",
  "cms_assemble_control\tGET\t/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/data\tsurface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data",
])
  if (gatedByRoute.has(key))
    witnessGaps.push(`缺陷 21 见证缺失（宏模板读档仍被写档 ${gatedByRoute.get(key)} 借证）：${key.replace(/\t/g, " ")}`);
// 缺陷 21 正向：写档切段后仍要拿到自己那一段，不许因为换段而丢证。
for (const key of [
  "cms_assemble_control\tDELETE\t/api/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/{path0}/data\tsurface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data_delete",
])
  if (!gatedByRoute.has(key))
    witnessGaps.push(`缺陷 21 见证缺失（宏模板写档切段后失去作证）：${key.replace(/\t/g, " ")}`);
// 缺陷 22：表名走 `format!` 插值（`UPDATE {} SET deleted_at = NOW()`）的写语句必须判为写面。
// cms `soft_delete_by_id(pool, table, id)` 一类 helper 全仓被 14 个函数这样落笔，
// 修前这些 handler 连「含写 SQL」都读不到 ⇒ 既不进 R 行动面，也不进 GET 面（GET 面最危险，
// csrf / requires_admin 双盲区）。摘掉表名位的 `\{` 分支，下面的写证据当场消失。
// 第十四批把这几把 GET 别名加门后，它们**退出 GET 无门禁名单**是预期的，所以正向见证从
// 「必须挂 K 桶」改成「必须仍带写证据」：判据一旦退化，indirectWrites 里那行就没了，
// 这条见证照样红——见证钉的是判据，不是某一轮的读数。
const kRoutes = new Set(getWrites.map(routeKeyOf));
const iwRoutes = new Set(indirectWrites.map(routeKeyOf));
for (const key of [
  "cms_assemble_control\tGET\t/api/appinfo/{id}/mockdeletetoget\tappinfo_id_mockdeletetoget",
  "cms_assemble_control\tGET\t/api/comment/{id}/mockdeletetoget\tcomment_id_mockdeletetoget",
  "cms_assemble_control\tGET\t/api/viewfieldconfig/{id}/mockdeletetoget\tviewfieldconfig_id_mockdeletetoget",
]) {
  if (!iwRoutes.has(key))
    witnessGaps.push(`缺陷 22 见证缺失（动态表名的 GET 写档丢了写证据）：${key.replace(/\t/g, " ")}`);
}
if (!wHelpers.has("soft_delete_by_id") && !wHelpers.has("soft_delete_after_gate"))
  witnessGaps.push("缺陷 22 见证缺失（动态表名落笔的 helper 不在间接写 SQL 名单：soft_delete_by_id / soft_delete_after_gate 都没有）");
// 反向：判据放宽到 `{` 之后，同族里**有门或只读**的档不许被拖进 GET 无门禁面（防把有门洗成无门）。
for (const key of [
  "cms_assemble_control\tGET\t/api/data/document/{id}/mockdeletetoget\tdata_document_id_mockdeletetoget",
  "cms_assemble_control\tGET\t/api/appinfo/{id}/control\tappinfo_id_control",
  "organization_assemble_control\tGET\t/api/organization/assemble/control/person/{flag}/mockdeletetoget\tperson_mock_delete_to_get",
]) {
  if (kRoutes.has(key))
    witnessGaps.push(`缺陷 22 反向见证失效（有门或只读档被误判成 GET 写面）：${key.replace(/\t/g, " ")}`);
}
// 缺陷 23：`u2_gate_by_sql` 是 cms 归属判定的入口，必须按门计。
// 正向：体内只有它、没有别的门名的档——摘掉这个词，这三条当场退回无门禁行动面。
for (const key of [
  "cms_assemble_control\tDELETE\t/api/categoryinfo/{id}\tcategoryinfo_u2_delete",
  "cms_assemble_control\tDELETE\t/api/viewfieldconfig/{id}\tviewfieldconfig_u2_delete",
  "cms_assemble_control\tGET\t/api/viewfieldconfig/{id}/mockdeletetoget\tviewfieldconfig_id_mockdeletetoget",
]) {
  const bucket = bucketByKey.get(key) || "";
  const inGetUngated = kRoutes.has(key);
  if (/^R[67]/.test(bucket) || inGetUngated)
    witnessGaps.push(`缺陷 23 见证缺失（u2_gate_by_sql 定门未被认出，桶 ${bucket || "K"}）：${key.replace(/\t/g, " ")}`);
}
// 反向：体内确无判定入口的 GET 写档不许被认成有门（防词表扩成恒真）。
for (const key of [
  "cms_assemble_control\tGET\t/api/document/{id}/view/count\tdocument_id_view_count",
]) {
  if (!kRoutes.has(key))
    witnessGaps.push(`缺陷 23 反向见证失效（无门档被误判成已加门）：${key.replace(/\t/g, " ")}`);
}
for (const w of WRITE_HELPER_WITNESSES)
  if (!wHelpers.has(w)) witnessGaps.push(`缺陷 16 见证缺失（间接写 SQL 未见 ${w}）`);
for (const w of GATE_HELPER_WITNESSES)
  if (!gHelpers.has(w)) witnessGaps.push(`缺陷 17 见证缺失（间接门禁未见 ${w}）`);
for (const w of DEAD_ALIAS_WITNESSES)
  if (!deadRows.has(w)) witnessGaps.push(`缺陷 18 见证缺失（死档未认出 ${w}）`);
for (const w of LIVE_ALIAS_WITNESSES)
  if (deadRows.has(w)) witnessGaps.push(`缺陷 18 反向见证失效（活档被误判成死档 ${w}）`);
console.log(
  `\n判据见证：缺陷 16 ${WRITE_HELPER_WITNESSES.length} 条 / 缺陷 17 ${GATE_HELPER_WITNESSES.length} 条 / ` +
  `缺陷 18 正向 ${DEAD_ALIAS_WITNESSES.length} 条 + 反向 ${LIVE_ALIAS_WITNESSES.length} 条 / ` +
  `缺陷 19 正向 4 条 + 缺陷 20 正向 2 条 + 反向 3 条 / 缺陷 21 反向 2 条 + 正向 1 条 / ` +
  `缺陷 22 正向 3 条 + helper 1 条 + 反向 3 条 / 缺陷 23 正向 3 条 + 反向 1 条，` +
  `缺口 ${witnessGaps.length} 处`
);
for (const g of witnessGaps) console.error(`  ${g}`);
if (witnessGaps.length) {
  console.error("判据见证不齐——某条「修尺子」的判据被摘掉或退化，读数不可信");
  process.exitCode = 1;
}
