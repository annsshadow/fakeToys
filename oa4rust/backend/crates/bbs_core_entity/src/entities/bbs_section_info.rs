// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize)]
#[sea_orm(table_name = "bbs_section_info")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub forum_id: String,
    pub name: String,
    pub description: Option<String>,
    // O10：列实为 BIGINT（migration 032），i32 解码必炸
    pub order_number: i64,
    pub disable: bool,
    // O10：列实为 TEXT（migration 032），DateTime 解码必炸
    pub create_time: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
