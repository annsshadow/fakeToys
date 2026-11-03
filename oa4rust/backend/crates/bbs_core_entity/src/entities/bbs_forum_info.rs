// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize)]
#[sea_orm(table_name = "bbs_forum_info")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    // O10：列实为 BIGINT（migration 032）——i32 会在 insert RETURNING 解码时
    // 恒定失败（INT8→i32 不兼容），create_forum 曾因此生产 500。
    pub order_number: i64,
    pub disable: bool,
    // O10：列实为 TEXT（migration 032）——DateTime 解码 TEXT 必炸，改 String
    // （写入格式与 raw SQL 的 NOW() 前缀兼容："%Y-%m-%d %H:%M:%S"）。
    pub create_time: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
