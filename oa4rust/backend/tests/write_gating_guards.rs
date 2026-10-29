// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 写门禁机械守卫（B7① + B7②）。
//!
//! 这两把守卫把「写路由必须带有效门禁」从人工取证变成机器可测，与
//! `tests/test_module_registration_guard.rs`（B7③）同一风格：纯 std 扫描，
//! 不依赖外部命令，`cargo test` 直接可跑。
//!
//! # B7① · GET 写禁令
//!
//! `csrf_middleware` 与 `requires_admin` 只覆盖 POST/PUT/PATCH/DELETE，
//! 注册在 GET 上的状态写对两把中间件同时隐形。凡 `.route("...", get(h))`
//! 且 h 体内含写 SQL（INSERT/UPDATE/DELETE，或 sea_orm ORM 写），且体内
//! 无会话/归属/角色门信号即应红。已知两条协议/引导族的 GET 写在豁免表里
//! 点名（见 `INTENTIONAL_GET_WRITES`），其余一律禁止。
//!
//! # B7② · Public 兜底点名
//!
//! `ADMIN_WRITE_PREFIXES`（person/unit/role/group 等写前缀）与注册表
//! Public 前缀的交集里，写路由没有中间件 admin 兜底——handler 层是唯一
//! 门禁。落在此集合内的写 handler 必须带体内归属/角色/会话门；新增不带
//! 门的写 handler 直接红。注册等设计上必须匿名的路由在豁免表里点名。

use std::path::{Path, PathBuf};

// ──────────────────────────────────────────────────────────────────────────────
// 豁免表
// ──────────────────────────────────────────────────────────────────────────────

/// 有意保留的匿名 GET 写（协议回调 / 系统引导族）。2026-10-30 收口后仅剩两条，
/// 均已在 A4 批逐条定性：
/// - `/api/qiyeweixin/update/person/detail/{code}`：企微扫码登录回调，一次性
///   code 由企微 OAuth 侧验证，GET 是外部回调契约的一部分，必须匿名可达；
/// - `/api/secret/set/cancel`：系统初始化密钥取消，与 `/api/secret/set` 同属
///   首次引导族（仅系统未初始化时可写）。
const INTENTIONAL_GET_WRITES: &[&str] = &[
    "/api/qiyeweixin/update/person/detail/{code}",
    "/api/secret/set/cancel",
];

/// 落在「admin 写前缀 ∩ 注册表 Public」但设计上不设体内门的写路由。
/// 目前只有认证前置族：注册（匿名注册是产品设计，门在中间件的限流与
/// 验证码侧）与密码重置（`/api/organization/assemble/personal/reset*` 是
/// personal crate 的自服务密码重置流程，必须在认证前置阶段可用）。
const INTENTIONAL_UNGATED_INTERSECTION_WRITES: &[&str] = &[
    "/api/person/regist",
    "/api/organization/assemble/personal/regist",
    "/api/organization/assemble/personal/reset",
    "/api/organization/assemble/personal/reset/mockputtopost",
    "/api/organization/assemble/personal/reset/password/anonymous",
];

// ──────────────────────────────────────────────────────────────────────────────
// 门禁词表（与 `scripts/audit_write_gating.mjs` 的 GATE_RE 同名单）。
//
// 四轮审计 §三.5 已把门词从「完整标识符」改成前缀匹配：`require_admin*` /
// `require_owner*` / `require_actor*` / `require_session*` / `u4_*` / `u2_require*` /
// `guard_write*` / `owner_gate*` 等一律算体内门禁。这里复刻该口径：
// - `GATE_PREFIX_IDENTIFIERS`：命中即门（标识符**以该词开头**即可）；
// - `GATE_EXACT_IDENTIFIERS`：`gate_*` 族必须逐个精确点名（全仓 `gate_sql` /
//   `gate_table` 是变量名不是门，绝不能写 `\bgate_` 前缀）。
// ──────────────────────────────────────────────────────────────────────────────
const GATE_PREFIX_IDENTIFIERS: &[&str] = &[
    "require_admin",
    "require_owner",
    "require_actor",
    "require_session",
    "u4_session",
    "u4_require",
    "u2_require",
    "u2_check_owner",
    "u2_gate_by_sql",
    "owner_gate",
    "is_admin",
    "check_permission",
    "assert_admin",
    "require_role",
    "guard_write",
    "validate_session",
    "validate_state",
    "extract_token_from_headers",
    "resolve_current_person",
    "is_conversation_member",
    "person_can_manage_mind",
    "crud_save_owned",
    "crud_delete_owned",
];

const GATE_EXACT_IDENTIFIERS: &[&str] = &[
    "gate_doc_writer",
    "gate_topic_owner",
    "gate_reply_owner",
    "gate_attachment_owner",
    "gate_appdict_writer",
];

// ──────────────────────────────────────────────────────────────────────────────
// 通用工具
// ──────────────────────────────────────────────────────────────────────────────

fn is_word_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || haystack.len() < needle.len() {
        return None;
    }
    (0..=haystack.len() - needle.len()).find(|&i| &haystack[i..i + needle.len()] == needle)
}

fn walk_rs(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).expect("读取目录失败") {
        let entry = entry.expect("遍历目录失败");
        let path = entry.path();
        if path.is_dir() {
            walk_rs(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

/// 扫描 `crates/*/src/**/*.rs`，返回 (crate 名, 文件路径)。
fn all_source_files() -> Vec<(String, PathBuf)> {
    let crates_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("crates");
    let mut out = Vec::new();
    for entry in std::fs::read_dir(&crates_dir).expect("读取 crates/ 失败——测试前提不成立")
    {
        let entry = entry.expect("遍历 crates/ 失败");
        if !entry.file_type().expect("stat 失败").is_dir() {
            continue;
        }
        let crate_name = entry.file_name().to_string_lossy().into_owned();
        let src_dir = entry.path().join("src");
        if !src_dir.is_dir() {
            continue;
        }
        let mut files = Vec::new();
        walk_rs(&src_dir, &mut files);
        for f in files {
            out.push((crate_name.clone(), f));
        }
    }
    out.sort();
    out
}

// ──────────────────────────────────────────────────────────────────────────────
// 路由注册扫描（复刻尺子的 readArg + METHOD_CHAIN_RE 口径）
// ──────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct Route {
    crate_name: String,
    method: String,
    path: String,
    handler: String,
}

/// 读取从 `start` 开始的一个顶层实参：尊重字符串转义与嵌套括号，深度 0 的
/// 逗号/闭括号分隔，并跳过行注释与块注释。返回 (实参文本, 结束位置)。
fn read_arg(src: &[u8], start: usize) -> (String, usize) {
    let mut i = start;
    let n = src.len();
    let mut depth: i32 = 0;
    let mut in_str = false;
    let mut esc = false;
    let mut arg = Vec::new();
    while i < n {
        let c = src[i];
        if in_str {
            arg.push(c);
            if esc {
                esc = false;
            } else if c == b'\\' {
                esc = true;
            } else if c == b'"' {
                in_str = false;
            }
            i += 1;
            continue;
        }
        if c == b'/' && i + 1 < n && src[i + 1] == b'/' {
            while i < n && src[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        if c == b'/' && i + 1 < n && src[i + 1] == b'*' {
            i += 2;
            while i + 1 < n && !(src[i] == b'*' && src[i + 1] == b'/') {
                i += 1;
            }
            i = (i + 2).min(n);
            continue;
        }
        if c == b'"' {
            in_str = true;
            arg.push(c);
            i += 1;
            continue;
        }
        if c == b'(' || c == b'[' || c == b'{' {
            depth += 1;
            arg.push(c);
            i += 1;
            continue;
        }
        if c == b')' || c == b']' || c == b'}' {
            if depth == 0 {
                return (String::from_utf8_lossy(&arg).into_owned(), i + 1);
            }
            depth -= 1;
            arg.push(c);
            i += 1;
            continue;
        }
        if c == b',' && depth == 0 {
            return (String::from_utf8_lossy(&arg).into_owned(), i + 1);
        }
        arg.push(c);
        i += 1;
    }
    (String::from_utf8_lossy(&arg).into_owned(), i)
}

/// 解析 `.route()` 首参：目前支持 `"..."` 字面量；`&fmt(...)` / `format!(...)`
/// 拼接路径不在本守卫扫描面（尺子仍在管），返回 None 表示无法解析。
fn resolve_path_literal(raw: &str) -> Option<String> {
    let arg = raw.trim();
    let lit = arg.strip_prefix('"')?.strip_suffix('"')?;
    if lit.contains('"') {
        return None;
    }
    Some(lit.to_string())
}

fn is_uri_shape(p: &str) -> bool {
    !p.is_empty()
        && p.starts_with('/')
        && p.chars()
            .all(|c| c.is_ascii_alphanumeric() || "/_.{}:%~- ".contains(c))
}

/// 在 `.route()` 的第二个实参里枚举 `(get|post|put|delete|patch|head|options)(handler)`。
/// 复刻尺子的 METHOD_CHAIN_RE：方法名前必须有非标识符边界，handler 是
/// `(?:ident::)*ident`。闭包（`get(|s| ...)`）不匹配，静默跳过。
fn extract_method_handlers(raw: &str) -> Vec<(String, String)> {
    const METHODS: &[&str] = &["get", "post", "put", "delete", "patch", "head", "options"];
    let bytes = raw.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let mut matched = None;
        for m in METHODS {
            if bytes[i..].starts_with(m.as_bytes())
                && (i == 0 || !is_word_char(bytes[i - 1]))
                && i + m.len() < bytes.len()
                && bytes[i + m.len()] == b'('
            {
                matched = Some(m);
                break;
            }
        }
        let Some(method) = matched else {
            i += 1;
            continue;
        };
        let mut j = i + method.len() + 1;
        while j < bytes.len()
            && (bytes[j] == b' ' || bytes[j] == b'\t' || bytes[j] == b'\n' || bytes[j] == b'\r')
        {
            j += 1;
        }
        let start = j;
        let mut end = j;
        while end < bytes.len() && (is_word_char(bytes[end]) || bytes[end] == b':') {
            end += 1;
        }
        if end > start {
            let handler = String::from_utf8_lossy(&bytes[start..end]).into_owned();
            let handler = handler.trim_end_matches(':').to_string();
            if !handler.is_empty() {
                out.push((method.to_ascii_uppercase(), handler));
            }
        }
        i = j;
    }
    out
}

/// 扫描全部 crate，提取 (crate, method, path, handler) 路由表。
/// 仅收集能解析出字面量路径且 handler 是具名函数的注册。
fn extract_routes() -> Vec<Route> {
    let mut routes = Vec::new();
    for (crate_name, file) in all_source_files() {
        let src = std::fs::read(&file).expect("读取源文件失败");
        if !src.windows(7).any(|w| w == b".route(") {
            continue;
        }
        let mut at = 0;
        while let Some(rel) = find_subslice(&src[at..], b".route(") {
            let site = at + rel;
            // 站点本身在行注释/文档注释里（如动词对齐说明）→ 不是注册
            let line_start = src[..site]
                .iter()
                .rposition(|&b| b == b'\n')
                .map(|p| p + 1)
                .unwrap_or(0);
            if src[line_start..site].windows(2).any(|w| w == b"//") {
                at = site + 7;
                continue;
            }
            let (raw_path, after_path) = read_arg(&src, site + 7);
            let (raw_handlers, _) = read_arg(&src, after_path);
            at = after_path;
            let Some(path) = resolve_path_literal(&raw_path) else {
                continue;
            };
            if !is_uri_shape(&path) {
                continue;
            }
            for (method, handler) in extract_method_handlers(&raw_handlers) {
                routes.push(Route {
                    crate_name: crate_name.clone(),
                    method,
                    path: path.clone(),
                    handler,
                });
            }
        }
    }
    routes
}

// ──────────────────────────────────────────────────────────────────────────────
// 函数体提取
// ──────────────────────────────────────────────────────────────────────────────

/// 在 crate 内查找 `fn <name>(` 并返回函数体（含签名，从 `fn` 到匹配的 `}`）。
/// 优先在 `<module>.rs` 中查找（与尺子的 moduleAliases 简化版同思路）。
fn find_handler_body(crate_name: &str, handler: &str) -> Option<String> {
    let parts: Vec<&str> = handler
        .split("::")
        .filter(|p| *p != "crate" && *p != "self")
        .collect();
    let name = *parts.last()?;
    let module = parts.first().copied();
    let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("crates")
        .join(crate_name);
    let mut files = Vec::new();
    walk_rs(&crate_dir.join("src"), &mut files);
    files.sort();
    if let Some(mod_name) = module {
        for f in &files {
            if f.file_stem().is_some_and(|s| s == mod_name) {
                if let Some(body) = slice_fn(&std::fs::read(f).ok()?, name) {
                    return Some(body);
                }
            }
        }
    }
    for f in &files {
        if let Some(body) = slice_fn(&std::fs::read(f).ok()?, name) {
            return Some(body);
        }
    }
    None
}

/// 从源文件字节中切出 `fn <name>(` 起的函数体（括号配平到匹配的 `}`）。
fn slice_fn(src: &[u8], name: &str) -> Option<String> {
    let needle = format!("fn {name}(").into_bytes();
    let mut search_from = 0;
    while let Some(rel) = find_subslice(&src[search_from..], &needle) {
        let pos = search_from + rel;
        // `fn` 前必须是词边界（不是标识符/路径的一部分）
        if pos > 0 && is_word_char(src[pos - 1]) {
            search_from = pos + needle.len();
            continue;
        }
        // 扫参数列表：跳过括号直到参数结束（函数自身的 `(` 已计入 depth=1）
        let mut i = pos + needle.len(); // '(' 之后的第一个字节
        let mut depth: i32 = 1;
        let mut in_str = false;
        let mut in_char = false;
        let mut esc = false;
        while i < src.len() {
            let c = src[i];
            if in_str {
                if esc {
                    esc = false;
                } else if c == b'\\' {
                    esc = true;
                } else if c == b'"' {
                    in_str = false;
                }
            } else if in_char {
                if esc {
                    esc = false;
                } else if c == b'\\' {
                    esc = true;
                } else if c == b'\'' {
                    in_char = false;
                }
            } else {
                match c {
                    b'"' => in_str = true,
                    b'\'' => in_char = true,
                    b'(' => depth += 1,
                    b')' => {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                    }
                    b'/' if i + 1 < src.len() && src[i + 1] == b'/' => {
                        while i < src.len() && src[i] != b'\n' {
                            i += 1;
                        }
                    }
                    b'/' if i + 1 < src.len() && src[i + 1] == b'*' => {
                        i += 2;
                        while i + 1 < src.len() && !(src[i] == b'*' && src[i + 1] == b'/') {
                            i += 1;
                        }
                        i = (i + 2).min(src.len());
                    }
                    _ => {}
                }
            }
            i += 1;
        }
        // 参数结束，找 `{`
        let mut j = i + 1;
        while j < src.len() {
            let c = src[j];
            if c == b'/' && j + 1 < src.len() && src[j + 1] == b'/' {
                while j < src.len() && src[j] != b'\n' {
                    j += 1;
                }
            } else if c == b'/' && j + 1 < src.len() && src[j + 1] == b'*' {
                j += 2;
                while j + 1 < src.len() && !(src[j] == b'*' && src[j + 1] == b'/') {
                    j += 1;
                }
                j = (j + 2).min(src.len());
            } else if c == b'{' {
                break;
            } else if c == b'}' {
                // 没有函数体（fn 指针/特征方法签名），结束本轮查找
                break;
            }
            j += 1;
        }
        if j >= src.len() || src[j] != b'{' {
            search_from = pos + needle.len();
            continue;
        }
        // 括号配平提取函数体
        let body_start = pos;
        let mut k = j;
        let mut brace_depth: i32 = 0;
        let mut in_str2 = false;
        let mut in_char2 = false;
        let mut esc2 = false;
        while k < src.len() {
            let c = src[k];
            if in_str2 {
                if esc2 {
                    esc2 = false;
                } else if c == b'\\' {
                    esc2 = true;
                } else if c == b'"' {
                    in_str2 = false;
                }
            } else if in_char2 {
                if esc2 {
                    esc2 = false;
                } else if c == b'\\' {
                    esc2 = true;
                } else if c == b'\'' {
                    in_char2 = false;
                }
            } else {
                match c {
                    b'"' => in_str2 = true,
                    b'\'' => in_char2 = true,
                    b'{' => brace_depth += 1,
                    b'}' => {
                        brace_depth -= 1;
                        if brace_depth == 0 {
                            return Some(
                                String::from_utf8_lossy(&src[body_start..=k]).into_owned(),
                            );
                        }
                    }
                    b'/' if k + 1 < src.len() && src[k + 1] == b'/' => {
                        while k < src.len() && src[k] != b'\n' {
                            k += 1;
                        }
                    }
                    b'/' if k + 1 < src.len() && src[k + 1] == b'*' => {
                        k += 2;
                        while k + 1 < src.len() && !(src[k] == b'*' && src[k + 1] == b'/') {
                            k += 1;
                        }
                        k = (k + 2).min(src.len());
                    }
                    _ => {}
                }
            }
            k += 1;
        }
        search_from = pos + needle.len();
    }
    None
}

// ──────────────────────────────────────────────────────────────────────────────
// 写 SQL 检测（复刻尺子 WRITE_SQL_RE + 缺陷 29 注释剥离 + 缺陷 28 ORM 判据）
// ──────────────────────────────────────────────────────────────────────────────

/// 剥离 Rust 注释与字符串字面量，避免注释散文里的 "Update the ..." 被当成写语句。
fn strip_comments(src: &str) -> Vec<u8> {
    let bytes = src.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    let n = bytes.len();
    while i < n {
        let c = bytes[i];
        if c == b'/' && i + 1 < n && bytes[i + 1] == b'/' {
            while i < n && bytes[i] != b'\n' {
                i += 1;
            }
        } else if c == b'/' && i + 1 < n && bytes[i + 1] == b'*' {
            i += 2;
            while i + 1 < n && !(bytes[i] == b'*' && bytes[i + 1] == b'/') {
                i += 1;
            }
            i += 1;
        } else if c == b'"' {
            out.push(c);
            i += 1;
            while i < n {
                out.push(bytes[i]);
                if bytes[i] == b'\\' {
                    i += 1;
                    if i < n {
                        out.push(bytes[i]);
                        i += 1;
                    }
                    continue;
                }
                if bytes[i] == b'"' {
                    i += 1;
                    break;
                }
                i += 1;
            }
        } else if c == b'\'' {
            out.push(c);
            i += 1;
            while i < n {
                out.push(bytes[i]);
                if bytes[i] == b'\\' {
                    i += 1;
                    if i < n {
                        out.push(bytes[i]);
                        i += 1;
                    }
                    continue;
                }
                if bytes[i] == b'\'' {
                    i += 1;
                    break;
                }
                i += 1;
            }
        } else {
            out.push(c);
            i += 1;
        }
    }
    out
}

fn skip_ws(hay: &[u8], mut i: usize) -> usize {
    while i < hay.len() && (hay[i] == b' ' || hay[i] == b'\t' || hay[i] == b'\n' || hay[i] == b'\r')
    {
        i += 1;
    }
    i
}

/// 在字节流中查找关键字（整词匹配，大小写不敏感），返回首个命中位置。
fn find_keyword(hay: &[u8], kw: &[u8]) -> Option<usize> {
    let mut i = 0;
    while i + kw.len() <= hay.len() {
        if hay[i..i + kw.len()].eq_ignore_ascii_case(kw)
            && (i == 0 || !is_word_char(hay[i - 1]))
            && (i + kw.len() == hay.len() || !is_word_char(hay[i + kw.len()]))
        {
            return Some(i);
        }
        i += 1;
    }
    None
}

/// 命中尺子的 WRITE_SQL_RE：`INSERT INTO` / `UPDATE <表名|{>` / `DELETE FROM`。
fn match_write_sql(code: &[u8]) -> bool {
    // INSERT INTO
    if let Some(pos) = find_keyword(code, b"INSERT") {
        let after = skip_ws(code, pos + 6);
        if code[after..].starts_with(b"INTO") {
            return true;
        }
    }
    // DELETE FROM
    if let Some(pos) = find_keyword(code, b"DELETE") {
        let after = skip_ws(code, pos + 6);
        if code[after..].starts_with(b"FROM") {
            return true;
        }
    }
    // UPDATE <ident|{>
    let mut i = 0;
    while let Some(pos) = find_keyword(&code[i..], b"UPDATE") {
        let p = i + pos;
        let after = skip_ws(code, p + 6);
        if after < code.len() {
            let mut q = after;
            if code[q] == b'\\' {
                q += 1;
            }
            if q < code.len() && (code[q] == b'"' || code[q] == b'`') {
                q += 1;
            }
            if q < code.len() && (code[q] == b'{' || is_word_char(code[q])) {
                return true;
            }
        }
        i = p + 6;
    }
    false
}

/// sea_orm ORM 写判据（缺陷 28）：`Entity::insert/insert_many/update_many/
/// delete_many/delete_by_id(...)`，或 `let [mut] x[: ...]: ActiveModel = ...`
/// 之后 `x.insert/update/delete/save(...)`。
fn match_orm_write(code: &[u8]) -> bool {
    const ENTITY_OPS: &[&str] = &[
        "Entity::insert(",
        "Entity::insert_many(",
        "Entity::update_many(",
        "Entity::delete_many(",
        "Entity::delete_by_id(",
    ];
    if ENTITY_OPS
        .iter()
        .any(|op| find_subslice(code, op.as_bytes()).is_some())
    {
        return true;
    }
    // ActiveModel：`let [mut] x[: ...]: ActiveModel = ...` 或 `let [mut] x = ActiveModel { ...`
    // 之后该变量调用 insert/update/delete/save。
    let text = String::from_utf8_lossy(code);
    let mut i = 0;
    while let Some(rel) = find_subslice(&code[i..], b"ActiveModel") {
        let pos = i + rel;
        let prefix = &text[..pos];
        let line_start = prefix.rfind('\n').map(|p| p + 1).unwrap_or(0);
        let line = &text[line_start..pos];
        let Some(let_pos) = line.rfind("let ") else {
            i = pos + 10;
            continue;
        };
        let after_let = line[let_pos + 4..].trim_start();
        let after_mut = after_let
            .strip_prefix("mut ")
            .unwrap_or(after_let)
            .trim_start();
        let name: String = after_mut
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
            .collect();
        if name.is_empty() {
            i = pos + 10;
            continue;
        }
        if ["insert(", "update(", "delete(", "save("]
            .iter()
            .any(|op| find_subslice(code, format!("{name}.{op}").as_bytes()).is_some())
        {
            return true;
        }
        i = pos + 10;
    }
    false
}

fn has_write_sql(body: &str) -> bool {
    let code = strip_comments(body);
    match_write_sql(&code) || match_orm_write(&code)
}

/// 调用点是否把拒绝向上传播（复刻尺子 `callPropagates`）：
/// ① `f(...).await?`；② `f(...).await` 且是函数体尾表达式（`.await` 后直接是 `}`）。
fn call_propagates(body: &str, name: &str) -> bool {
    let bytes = body.as_bytes();
    let kw = name.as_bytes();
    let mut search = 0;
    while let Some(pos) = find_subslice(&bytes[search..], kw) {
        let p = search + pos;
        if p > 0 && is_word_char(bytes[p - 1]) {
            search = p + kw.len();
            continue;
        }
        let q = skip_ws(bytes, p + kw.len());
        if q >= bytes.len() || bytes[q] != b'(' {
            search = p + kw.len();
            continue;
        }
        let mut depth = 1i32;
        let mut i = q + 1;
        while i < bytes.len() && depth > 0 {
            match bytes[i] {
                b'(' => depth += 1,
                b')' => depth -= 1,
                _ => {}
            }
            i += 1;
        }
        if depth != 0 {
            search = p + kw.len();
            continue;
        }
        let after = &bytes[i..(i + 80).min(bytes.len())];
        let mut j = 0;
        while j < after.len()
            && (after[j] == b' ' || after[j] == b'\t' || after[j] == b'\n' || after[j] == b'\r')
        {
            j += 1;
        }
        if after[j..].starts_with(b".await") {
            let mut k = j + 6;
            while k < after.len()
                && (after[k] == b' ' || after[k] == b'\t' || after[k] == b'\n' || after[k] == b'\r')
            {
                k += 1;
            }
            if k < after.len() && (after[k] == b'?' || after[k] == b'}') {
                return true;
            }
        }
        search = p + kw.len();
    }
    false
}

/// 路径中的 `{...}` 参数段数（复刻尺子 `pathParamCount`）。
fn path_param_count(p: &str) -> usize {
    p.matches('{').count()
}

/// 签名中 `Path<...>` 提取器需要的顶层参数个数（复刻尺子 `pathExtractArity` 的
/// 简化版：取所有 `Path<` 命中里的最大值，泛型实参按顶层逗号切段计数）。
fn path_extract_arity(sig: &str) -> usize {
    let bytes = sig.as_bytes();
    let mut max = 0;
    let mut search = 0;
    while let Some(rel) = find_subslice(&bytes[search..], b"Path<") {
        let pos = search + rel;
        // 确认是类型标注（`Path` 前最近的可见字符是 `:`，来自 `: Path` 或 `::Path`）
        let before = &sig[..pos];
        let nearest_visible = before.chars().rev().find(|c| !c.is_whitespace());
        if !nearest_visible.is_some_and(|c| c == ':') {
            search = pos + 5;
            continue;
        }
        // 找匹配的 `>`
        let mut depth = 1i32;
        let mut end = pos + 5;
        while end < bytes.len() && depth > 0 {
            match bytes[end] {
                b'<' => depth += 1,
                b'>' => depth -= 1,
                _ => {}
            }
            end += 1;
        }
        if depth != 0 {
            search = pos + 5;
            continue;
        }
        let inner = &sig[pos + 5..end - 1];
        let mut parts: Vec<String> = vec![String::new()];
        let mut d = 0i32;
        for ch in inner.chars() {
            match ch {
                '<' | '[' => d += 1,
                '>' | ']' => d -= 1,
                ',' if d == 0 => parts.push(String::new()),
                '(' | ')' => {}
                _ => {
                    if let Some(last) = parts.last_mut() {
                        last.push(ch);
                    }
                }
            }
        }
        let count = parts.iter().filter(|s| !s.trim().is_empty()).count().max(1);
        max = max.max(count);
        search = pos + 5;
    }
    max
}

/// 死档路由：路径参数段数 < 签名 `Path<...>` 需要的段数时，axum 在进入函数体前
/// 就抛路径参数错误，函数体一条语句都不执行（尺子缺陷 18）。这类档不算写面。
fn is_dead_route(path: &str, body: &str) -> bool {
    if path.contains('*') {
        return false;
    }
    let sig_end = body.find('{').unwrap_or(body.len());
    let sig = &body[..sig_end];
    let need = path_extract_arity(sig);
    if need == 0 {
        return false;
    }
    path_param_count(path) < need
}

/// 自限定写 = 归属门（尺子缺陷 26）：写 SQL 把会话身份绑进 person/person_id/user_id
/// 列，即「只改/建自己名下的行」，对 IDOR 等价于归属门。
fn has_self_scope_gate(body: &str) -> bool {
    let bytes = body.as_bytes();
    let session_refs: &[&[u8]] = &[
        b"&session.person_unique",
        b"&sess.person_unique",
        b"&s.person_unique",
        b"&cur.person_unique",
        b"&current.person_unique",
        b"&me.person_unique",
    ];
    if !session_refs
        .iter()
        .any(|r| find_subslice(bytes, r).is_some())
    {
        return false;
    }
    let code = strip_comments(body);
    for col in ["person", "person_id", "user_id"] {
        let kw = col.as_bytes();
        let mut i = 0;
        while let Some(pos) = find_keyword(&code[i..], kw) {
            let p = i + pos;
            let after = &code[p + kw.len()..];
            let mut j = 0;
            while j < after.len()
                && j < 48
                && after[j] != b';'
                && after[j] != b'\n'
                && after[j] != b'\r'
            {
                if after[j] == b'=' {
                    let mut k = j + 1;
                    while k < after.len() && (after[k] == b' ' || after[k] == b'\t') {
                        k += 1;
                    }
                    if k < after.len()
                        && after[k] == b'$'
                        && k + 1 < after.len()
                        && after[k + 1].is_ascii_digit()
                    {
                        return true;
                    }
                    break;
                }
                j += 1;
            }
            i = p + kw.len();
        }
    }
    false
}

/// 体内是否有会话/归属/角色门信号。
/// ① 前缀词表命中即门（复刻尺子的前缀匹配口径）；`gate_*` 族逐个精确点名。
/// ② 门写在被调用 helper 里（如 `appeal_owner_gate` / `check_owner` /
///    `u2_meeting_owner_gate`）时，沿「调用点且拒绝向上传播」的链递归至多 3 层。
fn has_gate(crate_name: &str, body: &str) -> bool {
    has_gate_depth(crate_name, body, 0)
}

fn has_gate_depth(crate_name: &str, body: &str, depth: usize) -> bool {
    if depth > 3 {
        return false;
    }
    let bytes = body.as_bytes();
    let direct_hit = GATE_PREFIX_IDENTIFIERS.iter().any(|g| {
        let kw = g.as_bytes();
        find_subslice(bytes, kw).is_some_and(|pos| pos == 0 || !is_word_char(bytes[pos - 1]))
    });
    if direct_hit {
        return true;
    }
    let exact_hit = GATE_EXACT_IDENTIFIERS.iter().any(|g| {
        let kw = g.as_bytes();
        find_subslice(bytes, kw).is_some_and(|pos| {
            let before_ok = pos == 0 || !is_word_char(bytes[pos - 1]);
            let after_ok = pos + kw.len() == bytes.len() || !is_word_char(bytes[pos + kw.len()]);
            before_ok && after_ok
        })
    });
    if exact_hit {
        return true;
    }
    // 自限定写：会话身份绑进 person 列的写 SQL ⇒ 只改自己名下的行（等价归属门）
    if has_self_scope_gate(body) {
        return true;
    }
    // 沿被调用且传播拒绝的 helper 递归
    let mut i = 0;
    while i < bytes.len() {
        if is_word_char(bytes[i]) {
            let start = i;
            while i < bytes.len() && is_word_char(bytes[i]) {
                i += 1;
            }
            let name = String::from_utf8_lossy(&bytes[start..i]).into_owned();
            let first = name.chars().next();
            if first.is_some_and(|c| c.is_ascii_lowercase() || c == '_')
                && call_propagates(body, &name)
            {
                if let Some(hb) = find_handler_body(crate_name, &name) {
                    if hb != body && has_gate_depth(crate_name, &hb, depth + 1) {
                        return true;
                    }
                }
            }
        } else {
            i += 1;
        }
    }
    false
}

// ──────────────────────────────────────────────────────────────────────────────
// 中间件常量与注册表（B7② 需要）
// ──────────────────────────────────────────────────────────────────────────────

fn read_const_str_list(src: &str, name: &str) -> Vec<String> {
    let marker = format!("pub const {name}: &[&str] = &[");
    let Some(start) = src.find(&marker) else {
        return Vec::new();
    };
    let rest = &src[start + marker.len()..];
    let Some(end) = rest.find("];") else {
        return Vec::new();
    };
    let body = &rest[..end];
    let mut out = Vec::new();
    let bytes = body.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'"' {
            let mut j = i + 1;
            let mut s = String::new();
            while j < bytes.len() && bytes[j] != b'"' {
                if bytes[j] == b'\\' && j + 1 < bytes.len() {
                    s.push(bytes[j + 1] as char);
                    j += 2;
                } else {
                    s.push(bytes[j] as char);
                    j += 1;
                }
            }
            out.push(s);
            i = j + 1;
        } else {
            i += 1;
        }
    }
    out
}

/// 注册表条目：路径 + 权限级别。
type RegistryEntry = (String, String);

/// 从 rbac.rs 解析注册表：返回 (exact 表, prefix 表)，元素为 (路径, 级别)。
/// 复刻尺子的正则口径：`register(_exact|_prefix)?("path", PermissionLevel::X)`，
/// 容忍跨行与空白。
fn read_registry(src: &str) -> (Vec<RegistryEntry>, Vec<RegistryEntry>) {
    let mut exact = Vec::new();
    let mut prefixes = Vec::new();
    let bytes = src.as_bytes();
    let mut i = 0;
    while let Some(rel) = find_subslice(&bytes[i..], b"registry.register") {
        let pos = i + rel;
        let mut j = pos + b"registry.register".len();
        let mut is_exact = false;
        if bytes[j..].starts_with(b"_exact") {
            is_exact = true;
            j += 6;
        } else if bytes[j..].starts_with(b"_prefix") {
            j += 7;
        }
        // 跳过空白与 `(`
        while j < bytes.len()
            && (bytes[j] == b' ' || bytes[j] == b'\t' || bytes[j] == b'\n' || bytes[j] == b'\r')
        {
            j += 1;
        }
        if j >= bytes.len() || bytes[j] != b'(' {
            i = pos + 1;
            continue;
        }
        j += 1;
        while j < bytes.len()
            && (bytes[j] == b' ' || bytes[j] == b'\t' || bytes[j] == b'\n' || bytes[j] == b'\r')
        {
            j += 1;
        }
        if j >= bytes.len() || bytes[j] != b'"' {
            i = pos + 1;
            continue;
        }
        j += 1;
        let path_start = j;
        while j < bytes.len() && bytes[j] != b'"' {
            j += 1;
        }
        let path = String::from_utf8_lossy(&bytes[path_start..j]).into_owned();
        j += 1;
        while j < bytes.len()
            && (bytes[j] == b' '
                || bytes[j] == b'\t'
                || bytes[j] == b'\n'
                || bytes[j] == b'\r'
                || bytes[j] == b',')
        {
            j += 1;
        }
        let level_start = j;
        while j < bytes.len() && (is_word_char(bytes[j]) || bytes[j] == b':') {
            j += 1;
        }
        let level = String::from_utf8_lossy(&bytes[level_start..j]).into_owned();
        if let Some(short) = level.strip_prefix("PermissionLevel::") {
            if is_exact {
                exact.push((path, short.to_string()));
            } else {
                prefixes.push((path, short.to_string()));
            }
        }
        i = pos + 1;
    }
    (exact, prefixes)
}

/// 注册表级别判定（复刻 get_permission：exact 优先，否则最长前缀优先）。
fn registry_level(
    path: &str,
    exact: &[(String, String)],
    prefixes: &[(String, String)],
) -> Option<String> {
    if let Some((_, lvl)) = exact.iter().find(|(p, _)| p == path) {
        return Some(lvl.clone());
    }
    let mut best: Option<(usize, &str)> = None;
    for (pre, lvl) in prefixes {
        if path.starts_with(pre) && best.is_none_or(|(len, _)| pre.len() > len) {
            best = Some((pre.len(), lvl));
        }
    }
    best.map(|(_, lvl)| lvl.to_string())
}

fn seg_prefix(path: &str, prefix: &str) -> bool {
    path == prefix || path.starts_with(&format!("{prefix}/"))
}

// ──────────────────────────────────────────────────────────────────────────────
// 守卫测试
// ──────────────────────────────────────────────────────────────────────────────

/// B7①：GET 路由挂写 SQL 且体内无门禁 → 红（豁免表除外）。
#[test]
fn get_routes_with_write_sql_are_gated_or_allowlisted() {
    let routes = extract_routes();
    let mut violations = Vec::new();
    let mut checked = 0usize;
    for r in &routes {
        if matches!(r.method.as_str(), "POST" | "PUT" | "DELETE" | "PATCH") {
            continue;
        }
        let Some(body) = find_handler_body(&r.crate_name, &r.handler) else {
            // 体内不可解析（宏生成/跨 crate）不在本守卫扫描面，尺子 R8 仍在管
            continue;
        };
        checked += 1;
        // 死档路由：路径参数段数不足，axum 在进入函数体前就 400，SQL 永不执行
        if is_dead_route(&r.path, &body) {
            continue;
        }
        if !has_write_sql(&body) {
            continue;
        }
        if has_gate(&r.crate_name, &body) {
            continue;
        }
        if INTENTIONAL_GET_WRITES.contains(&r.path.as_str()) {
            continue;
        }
        violations.push(format!(
            "{} {} {} -> {}",
            r.crate_name, r.method, r.path, r.handler
        ));
    }
    violations.sort();
    violations.dedup();
    assert!(
        checked > 0,
        "没有扫到任何 GET 路由——扫描可能失效，请检查测试前提（extract_routes / find_handler_body）"
    );
    assert!(
        violations.is_empty(),
        "以下 GET 路由的 handler 体内含写 SQL 且无任何会话/归属/角色门（csrf 与 requires_admin 对 GET 双盲区）：\n{}\n\
         修复：把写操作改挂 POST/PUT/DELETE，或在 handler 体内加门；\n\
         若该 GET 写必须存在（协议回调/系统引导），把它登记进 INTENTIONAL_GET_WRITES 并写明理由。\n\
         本次共检查 {checked} 条 GET 路由。",
        violations.join("\n")
    );
}

/// B7②：落在「ADMIN_WRITE_PREFIXES ∩ 注册表 Public」内的写路由必须有体内门。
#[test]
fn admin_write_public_intersection_writes_have_in_body_gates() {
    let constants_src = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("crates/shared/src/middleware/constants.rs"),
    )
    .expect("读取 constants.rs 失败");
    let rbac_src = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("crates/shared/src/middleware/rbac.rs"),
    )
    .expect("读取 rbac.rs 失败");
    let admin_prefixes = read_const_str_list(&constants_src, "ADMIN_WRITE_PREFIXES");
    let (exact, prefixes) = read_registry(&rbac_src);
    assert!(
        !admin_prefixes.is_empty(),
        "ADMIN_WRITE_PREFIXES 不应为空——测试前提不成立"
    );

    let routes = extract_routes();
    let mut checked = 0usize;
    let mut violations = Vec::new();
    for r in &routes {
        if !matches!(r.method.as_str(), "POST" | "PUT" | "DELETE") {
            continue;
        }
        // 中间件 admin 兜底是否命中：POST/PUT/DELETE ∧ 段边界前缀
        let admin_hit = admin_prefixes.iter().any(|pre| seg_prefix(&r.path, pre));
        if !admin_hit {
            continue;
        }
        // 注册表级别是否为 Public（Public 早退会吞掉 admin 兜底）
        if registry_level(&r.path, &exact, &prefixes).as_deref() != Some("Public") {
            continue;
        }
        let Some(body) = find_handler_body(&r.crate_name, &r.handler) else {
            continue;
        };
        // 死档路由：路径参数段数不足，写 SQL 永不执行
        if is_dead_route(&r.path, &body) {
            continue;
        }
        // 只读挂写方法（POST 批量查询/详情读）不要求门，真写面才需要
        if !has_write_sql(&body) {
            continue;
        }
        checked += 1;
        if has_gate(&r.crate_name, &body) {
            continue;
        }
        if INTENTIONAL_UNGATED_INTERSECTION_WRITES.contains(&r.path.as_str()) {
            continue;
        }
        violations.push(format!(
            "{} {} {} -> {}",
            r.crate_name, r.method, r.path, r.handler
        ));
    }
    violations.sort();
    violations.dedup();
    assert!(
        checked > 0,
        "「ADMIN_WRITE_PREFIXES ∩ 注册表 Public」内没有扫到任何写路由——扫描可能失效，请检查测试前提"
    );
    assert!(
        violations.is_empty(),
        "以下写路由落在 ADMIN_WRITE_PREFIXES 与注册表 Public 的交集内，但 handler 体内没有归属/角色/会话门：\n{}\n\
         \n\
         Public 早退会让中间件 admin 兜底失效，handler 是唯一门禁。\n\
         修复：给 handler 加 require_owner / require_admin / require_session 等体内门；\n\
         若该路由设计上就不设门（如注册），登记进 INTENTIONAL_UNGATED_INTERSECTION_WRITES 并写明理由。\n\
         本次共检查 {checked} 条交集写路由。",
        violations.join("\n")
    );
}
