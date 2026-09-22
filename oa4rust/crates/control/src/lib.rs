// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

pub mod group;
pub mod pagination;
pub mod person;
pub mod role;
pub mod routes;
pub mod unit;

pub use routes::control_router;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;
