// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O33：文本列的 keyset 分页——游标键必须与 ORDER BY 的排序键一致。
//!
//! 反例：`WHERE id > $1 ORDER BY create_time DESC`。游标（id）不是排序键
//! （create_time），`id > $1` 先按 id 序砍掉一半集合，再对剩下的行按
//! create_time 排序，于是：
//! - **漏行**：id 序上已经越过游标、但按排序键本应出现在下一页的行被排除；
//! - **重行**：id 序上尚未越过游标、但排序键上已在上一页展示过的行再次出现。
//!
//! 正确做法是让「游标比较」与「排序」使用同一组复合键：
//! `(排序键..., 并列裁决键)`。调用方仍只传上一页末行的 id，本模块把该 id
//! 换算成同一行上的复合键（子查询），对调用方契约零改动。
//!
//! NULL 统一按 NULLS LAST：
//! - 降序列把 NULL 归一为「同型最小值」（文本 `''`、时间戳 `'-infinity'::timestamp`、
//!   数字语义 `'-2147483649'::bigint`），DESC 时自然落末；
//! - 升序列前置 `(归一化值 IS NULL)` 布尔键（false 在前，NULL 落末）。
//!
//! 行值比较遇到 NULL 会让整个比较结果为 NULL（整行被丢弃），归一化同时
//! 避免了这个陷阱。
//!
//! O34：排序键可声明为「数字语义」（[`KeyKind::NumericText`]），生成时统一走
//! [`KeyColumn::normalized`]。游标比较（[`KeySet::predicate`]）与排序
//! （[`KeySet::order_by`]）都只从同一个 `normalized()` 取键，禁止各自拼表达式——
//! 否则会重造 O33 的「排序键与分页键不一致 → 漏行/重行」。

/// 排序方向（按显示顺序）。`Desc` 的分页端点用 `next` 走向更小的键，
/// `Asc` 的端点则走向更大的键。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SortDir {
    Asc,
    Desc,
}

/// 排序键的语义类别。`Null` 归一化字面量的类型必须与键值类型一致，
/// 否则 `COALESCE` 会在运行时做非法类型转换。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum KeyKind {
    /// 文本列：NULL 归一为 `''`。
    Text,
    /// 时间戳列：NULL 归一为 `'-infinity'::timestamp`。
    Timestamp,
    /// TEXT 列按数字语义：值经 [`super::numeric_text_i32_sort_expr`] 转 bigint，
    /// NULL / 空串 / 非数字 / 越界统一归一为 `'-2147483649'::bigint`（低于 i32 下限，
    /// 合法值不可能碰撞），DESC 落末。
    NumericText,
}

/// 排序键列。`expr` 是列引用（只允许写死的列名，不得来自请求参数）；
/// `kind` 决定归一化表达式与 NULL 哨兵。
#[derive(Clone, Copy)]
pub struct KeyColumn {
    expr: &'static str,
    kind: KeyKind,
}

impl KeyColumn {
    /// 文本列（NULLS LAST，NULL 归一为 `''`）。
    pub const fn text(expr: &'static str) -> Self {
        Self {
            expr,
            kind: KeyKind::Text,
        }
    }

    /// 时间戳列（NULLS LAST，NULL 归一为 `-infinity`）。
    pub const fn timestamp(expr: &'static str) -> Self {
        Self {
            expr,
            kind: KeyKind::Timestamp,
        }
    }

    /// TEXT 列按数字语义排序（NULLS LAST）。
    pub const fn numeric_text(expr: &'static str) -> Self {
        Self {
            expr,
            kind: KeyKind::NumericText,
        }
    }

    /// 归一化后的键值表达式（永远非 NULL，且与键值同型）。
    fn normalized(&self) -> String {
        match self.kind {
            KeyKind::Text => format!("COALESCE({}, '')", self.expr),
            KeyKind::Timestamp => format!("COALESCE({}, '-infinity'::timestamp)", self.expr),
            KeyKind::NumericText => format!(
                "COALESCE({}, '-2147483649'::bigint)",
                super::numeric_text_i32_sort_expr(self.expr)
            ),
        }
    }

    /// 判断键值是否为 NULL 的布尔表达式。数字语义下「不可解析/越界」与真 NULL
    /// 同等对待，避免脏值抢在 NULL 之前。
    fn is_null(&self) -> String {
        match self.kind {
            KeyKind::NumericText => {
                format!("({} IS NULL)", super::numeric_text_i32_sort_expr(self.expr))
            }
            _ => format!("({} IS NULL)", self.expr),
        }
    }
}

/// 复合 keyset：`columns` 为排序键，`tiebreak` 为并列裁决键（通常为 id）。
#[derive(Clone, Copy)]
pub struct KeySet<'a> {
    pub table: &'a str,
    pub columns: &'a [KeyColumn],
    pub dir: SortDir,
    pub tiebreak: &'a str,
}

impl KeySet<'_> {
    /// 全部键的 `(null_flag, normalized)` 二元组。游标比较与排序**只从这里取**，
    /// 保证两者逐字一致；tiebreak 按文本列处理（id 为文本）。
    fn key_specs(&self) -> Vec<(String, String)> {
        let mut specs: Vec<(String, String)> = self
            .columns
            .iter()
            .map(|c| (c.is_null(), c.normalized()))
            .collect();
        let tiebreak = self.tiebreak;
        specs.push((
            format!("({tiebreak} IS NULL)"),
            format!("COALESCE({tiebreak}, '')"),
        ));
        specs
    }

    fn key_list(&self) -> String {
        let mut parts: Vec<String> = Vec::new();
        for (null_flag, value) in self.key_specs() {
            match self.dir {
                SortDir::Desc => parts.push(value),
                SortDir::Asc => {
                    parts.push(null_flag);
                    parts.push(value);
                }
            }
        }
        parts.join(", ")
    }

    /// 复合键的行值比较谓词。`after = true` 取显示顺序中位于游标之后的行，
    /// `false` 取之前的行（后者调用方通常需把结果再倒序一次）。
    ///
    /// 游标行不存在（被删除 / 首屏哨兵）时子查询为 NULL，谓词整体为 NULL，
    /// 该页为空——不重行、不越界。首屏哨兵请配合 [`is_first_page_cursor`] 短路。
    pub fn predicate(&self, after: bool) -> String {
        let op = match (self.dir, after) {
            (SortDir::Desc, true) | (SortDir::Asc, false) => "<",
            _ => ">",
        };
        let keys = self.key_list();
        format!(
            "({keys}) {op} (SELECT {keys} FROM {table} WHERE {tiebreak} = $1 LIMIT 1)",
            table = self.table,
            tiebreak = self.tiebreak,
        )
    }

    /// `ORDER BY` 的内容（不含 `ORDER BY` 关键字）。`reverse = true` 用于取
    /// 「游标之前的最近一页」，调用方拿到结果后需 `reverse()` 还原显示顺序。
    pub fn order_by(&self, reverse: bool) -> String {
        let mut parts: Vec<String> = Vec::new();
        let mut push = |null_flag: &str, value: &str| match self.dir {
            SortDir::Desc => {
                let o = if reverse { "ASC" } else { "DESC" };
                parts.push(format!("{value} {o}"));
            }
            SortDir::Asc => {
                let o = if reverse { "DESC" } else { "ASC" };
                parts.push(format!("{null_flag} {o}"));
                parts.push(format!("{value} {o}"));
            }
        };
        for (null_flag, value) in &self.key_specs() {
            push(null_flag, value);
        }
        parts.join(", ")
    }
}

/// 旧契约里的首屏哨兵：调用方以固定 `(0)`（少数端点为 `0` / 空串）请求第一页。
/// 这些值不是真实 id，不能走「按 id 取排序键」的子查询，必须短路成无条件首页。
pub fn is_first_page_cursor(cursor: &str) -> bool {
    matches!(cursor, "" | "0" | "(0)")
}

/// `next` 页的游标条件（占位符 `$1` = 上一页末行 id）：首屏哨兵短路为首页。
pub fn next_where(keyset: &KeySet<'_>) -> String {
    format!(
        "($1 = '0' OR $1 = '(0)' OR $1 = '' OR {})",
        keyset.predicate(true)
    )
}
