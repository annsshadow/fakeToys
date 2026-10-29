// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! ORM 共享层单元测试

#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    #[test]
    fn pagination_helper_is_reexported_at_crate_root() {
        // 本 crate 目前只有占位助手；钉住公开路径，避免 `pub use` 被误删后调用方集体编译失败。
        let _helper = crate::cursor_list::<i32>;
    }
}
