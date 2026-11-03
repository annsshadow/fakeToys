// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize)]
#[sea_orm(table_name = "x_application")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub name: String,
    pub category: String,
    pub sub_category: String,
    pub version: String,
    pub publisher: String,
    pub creator_person: String,
    // O10：x_application.deleted_at 列实为 TEXT（同族 cte_*/script 是 TIMESTAMP
    // 真列，不动）。DateTime 解码 TEXT 必炸；column_type="Text" 标注与 DateTime
    // 字段自相矛盾。改 String。
    pub deleted_at: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
