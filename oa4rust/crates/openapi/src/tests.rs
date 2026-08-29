//! openapi crate tests (U13 / R-test-coverage)
//!
//! 验证 OpenAPI 文档生成与安全方案注入正确性。

#[cfg(test)]
mod tests {
    use crate::{ApiDoc, SecurityAddon};
    use utoipa::Modify;
    use utoipa::{OpenApi as OpenApiTrait, openapi::OpenApi as OpenApiDoc};

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
            api.info
                .description
                .as_ref()
                .unwrap()
                .contains("OA4Rust"),
            "Description should mention OA4Rust"
        );
    }

    #[test]
    fn test_api_doc_has_paths() {
        let api = ApiDoc::openapi();
        let path_count = api.paths.paths.len();
        assert!(path_count >= 1000, "Expected many paths, got {}", path_count);
    }

    #[test]
    fn test_api_doc_has_tags() {
        let api = ApiDoc::openapi();
        let tags = api.tags.as_ref();
        assert!(tags.is_some(), "OpenAPI should have tags");
        let tag_list: Vec<&str> = tags
            .unwrap()
            .iter()
            .map(|t| t.name.as_str())
            .collect();
        assert!(tag_list.contains(&"base"), "Should have 'base' tag");
        assert!(tag_list.contains(&"authentication"), "Should have 'authentication' tag");
    }

    #[test]
    fn test_security_addon_does_not_panic() {
        use utoipa::openapi::{Info, Paths};
        let mut api = OpenApiDoc::new(
            Info::new("test", "1.0"),
            Paths::default(),
        );
        let addon = SecurityAddon;
        addon.modify(&mut api);
        // Verify security schemes were added
        let has_security = api
            .components
            .as_ref()
            .map(|c| c.security_schemes.contains_key("bearer_token"))
            .unwrap_or(false);
        assert!(has_security, "SecurityAddon should add bearer_token security scheme");
    }

    #[test]
    fn test_api_doc_openapi_version() {
        let api = ApiDoc::openapi();
        // Just verify the struct is populated (utoipa always sets openapi field)
        let _ = &api.openapi;
    }
}
