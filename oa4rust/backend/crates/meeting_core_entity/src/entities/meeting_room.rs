// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize)]
#[sea_orm(table_name = "x_meeting_room")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub name: String,
    pub building_id: Option<String>,
    pub floor: Option<String>,
    pub capacity: Option<i32>,
    // x_meeting_room.equipment 实列是 jsonb（information_schema 现场确认）；
    // 旧声明 Option<String> 在 Entity::find 解码时报
    // "error deserializing column equipment" → GET room/list、room/{id} 500。
    pub equipment: Option<Json>,
    pub description: Option<String>,
    pub photo: Option<String>,
    pub open_meeting: Option<bool>,
    pub order_number: Option<i32>,
    pub create_time: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
