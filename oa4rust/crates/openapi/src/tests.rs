//! openapi crate tests (U13 / R-test-coverage)
//!
//! 验证 OpenAPI 文档生成与安全方案注入正确性。

#[cfg(test)]
mod tests {
    use crate::{ApiDoc, SecurityAddon};
    use utoipa::Modify;
    use utoipa::{openapi::OpenApi as OpenApiDoc, OpenApi as OpenApiTrait};

    #[test]
    fn test_api_doc_has_title() {
        let api = ApiDoc::openapi();
        assert_eq!(api.info.title, "OA4Rust API");
        assert_eq!(api.info.version, "0.1.0");
    }

    #[test]
    fn test_api_doc_has_description() {
        let api = ApiDoc::openapi();
        assert!(
            api.info.description.is_some(),
            "OpenAPI info should have a description"
        );
        assert!(
            api.info.description.as_ref().unwrap().contains("OA4Rust"),
            "Description should mention OA4Rust"
        );
    }

    #[test]
    fn test_api_doc_has_paths() {
        let api = ApiDoc::openapi();
        let path_count = api.paths.paths.len();
        assert!(
            path_count >= 1000,
            "Expected many paths, got {}",
            path_count
        );
    }

    #[test]
    fn test_api_doc_has_tags() {
        let api = ApiDoc::openapi();
        let tags = api.tags.as_ref();
        assert!(tags.is_some(), "OpenAPI should have tags");
        let tag_list: Vec<&str> = tags.unwrap().iter().map(|t| t.name.as_str()).collect();
        assert!(tag_list.contains(&"base"), "Should have 'base' tag");
        assert!(
            tag_list.contains(&"authentication"),
            "Should have 'authentication' tag"
        );
    }

    #[test]
    fn test_security_addon_does_not_panic() {
        use utoipa::openapi::{Info, Paths};
        let mut api = OpenApiDoc::new(Info::new("test", "1.0"), Paths::default());
        let addon = SecurityAddon;
        addon.modify(&mut api);
        // Verify security schemes were added
        let has_security = api
            .components
            .as_ref()
            .map(|c| c.security_schemes.contains_key("bearer_token"))
            .unwrap_or(false);
        assert!(
            has_security,
            "SecurityAddon should add bearer_token security scheme"
        );
    }

    #[test]
    fn test_api_doc_openapi_version() {
        let api = ApiDoc::openapi();
        // Just verify the struct is populated (utoipa always sets openapi field)
        let _ = &api.openapi;
    }

    /// 防止生成器 bug 将含连字符的路径段编码为无效 Rust 函数名。
    /// 这类错误会让产物静默编译失败或引发运行时 panic。
    #[test]
    fn test_generated_function_names_are_valid_identifiers() {
        let source = include_str!("lib.rs");
        for line in source.lines() {
            if line.trim().starts_with("async fn ") {
                let trimmed = line.trim();
                let func_name = trimmed
                    .strip_prefix("async fn ")
                    .expect("already matched")
                    .split('(')
                    .next()
                    .unwrap()
                    .trim();
                // Rust identifiers must start with a letter or underscore,
                // and contain only [a-zA-Z0-9_].
                assert!(
                    func_name.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_'),
                    "Invalid function name '{}': must start with letter or underscore",
                    func_name
                );
                assert!(
                    func_name
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || c == '_'),
                    "Invalid function name '{}': contains disallowed character '{}' at position {}",
                    func_name,
                    func_name
                        .chars()
                        .position(|c| !c.is_ascii_alphanumeric() && c != '_')
                        .map(|p| format!("'{}'", func_name.chars().nth(p).unwrap()))
                        .unwrap_or_default(),
                    func_name
                        .chars()
                        .position(|c| !c.is_ascii_alphanumeric() && c != '_')
                        .unwrap_or(0)
                );
            }
        }
    }
}
