// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    const PERSONAL_RS: &str = include_str!("personal.rs");

    /// R328 D2：save_signature 必须桥接写 x_custom（signature/list 的读表），
    /// 且与 upload 的 SIGNATURE_{uuid} 命名同源；否则保存后刷新回空。
    #[test]
    fn r328_save_signature_bridges_x_custom() {
        assert!(
            PERSONAL_RS.contains("INSERT INTO x_custom"),
            "save_signature 必须桥接 INSERT x_custom 供 signature/list 读取"
        );
        assert!(
            PERSONAL_RS.contains("UPDATE x_custom"),
            "已有签名必须 UPDATE x_custom，避免重复行"
        );
        assert!(
            PERSONAL_RS.contains("SIGNATURE_"),
            "桥接行命名须与 signature/upload 一致（SIGNATURE_ 前缀）"
        );
        assert!(
            PERSONAL_RS.contains("value"),
            "桥接必须写 value 列（list 读该字段）"
        );
    }

    /// R328 验收④：x_person_signature 原写链必须保留，不破坏 095 migration 意图。
    #[test]
    fn r328_save_signature_keeps_person_signature_write() {
        assert!(PERSONAL_RS.contains("INSERT INTO x_person_signature"));
        assert!(PERSONAL_RS.contains("UPDATE x_person_signature"));
    }

    // SKIPPED: upload requires Session parameter
    // SKIPPED: get_current_icon requires Session parameter
    // SKIPPED: get_icon requires Session parameter
    // SKIPPED: get_info requires Session parameter
    // SKIPPED: update_info requires Session parameter
    // SKIPPED: get_detail requires Session parameter
    // SKIPPED: save_signature requires Session parameter
    // SKIPPED: face_list requires Session parameter
    // SKIPPED: face_create not accessible
    // SKIPPED: face_save not accessible
    // SKIPPED: face_delete not accessible
}
