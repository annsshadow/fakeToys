// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use serde::Serialize;

#[derive(Serialize)]
pub struct Person {
    pub unique: String,
    pub name: String,
    pub mobile: Option<String>,
}

#[derive(Serialize)]
pub struct AuthenticationRequest {
    pub credential: String,
    pub password: String,
}
