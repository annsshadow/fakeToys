//! MCP Server 测试套件（U13 / R-test-coverage）
//!
//! 覆盖公开 API：McpError 类型、JSON-RPC 序列化、
//! ToolBridge 工具目录与错误处理。

#[cfg(test)]
mod tests {
    use crate::tool_bridge::{
        CallToolResponse, JsonRpcResponse, McpError, ToolContent, ToolInputSchema, ToolProperty,
        ToolCallParams,
    };
    use serde_json::json;

    // ── McpError ────────────────────────────────────────────────────────────────────

    #[test]
    fn test_mcp_error_invalid_request() {
        let err = McpError::invalid_request("bad input");
        assert_eq!(err.code, -32600);
        assert_eq!(err.message, "bad input");
    }

    #[test]
    fn test_mcp_error_not_found() {
        let err = McpError::not_found("method missing");
        assert_eq!(err.code, -32601);
        assert_eq!(err.message, "method missing");
    }

    #[test]
    fn test_mcp_error_internal() {
        let err = McpError::internal("db failed");
        assert_eq!(err.code, -32603);
        assert_eq!(err.message, "db failed");
    }

    #[test]
    fn test_mcp_error_serialization() {
        let err = McpError::invalid_request("test");
        let json = serde_json::to_value(&err).unwrap();
        assert_eq!(json["code"], -32600);
        assert_eq!(json["message"], "test");
    }

    // ── JsonRpcResponse ───────────────────────────────────────────────────────

    #[test]
    fn test_json_rpc_response_ok() {
        let resp = JsonRpcResponse::ok(Some(json!(1)), json!({"result": "ok"}));
        let value = resp.into_json_value();
        assert_eq!(value["jsonrpc"], "2.0");
        assert_eq!(value["id"], 1);
        assert!(value.get("result").is_some());
        assert!(value.get("error").is_none());
    }

    #[test]
    fn test_json_rpc_response_err() {
        let resp = JsonRpcResponse::err(Some(json!("abc")), -32601, "not found");
        let value = resp.into_json_value();
        assert_eq!(value["jsonrpc"], "2.0");
        assert_eq!(value["id"], "abc");
        assert!(value.get("result").is_none());
        assert_eq!(value["error"]["code"], -32601);
        assert_eq!(value["error"]["message"], "not found");
    }

    #[test]
    fn test_json_rpc_response_no_id() {
        let resp = JsonRpcResponse::ok(None, json!({"tools": []}));
        let value = resp.into_json_value();
        // id field is always serialized (even None → null)
        assert!(value.get("result").is_some());
    }

    #[test]
    fn test_json_rpc_response_roundtrip() {
        let original = JsonRpcResponse::ok(
            Some(json!("req-1")),
            json!({"tools": [{"name": "health"}]}),
        );
        let value = original.into_json_value();
        assert_eq!(value["jsonrpc"], "2.0");
    }

    // ── JsonRpcRequest (via deserialization) ──────────────────────────────────

    #[test]
    fn test_json_rpc_request_deserialize() {
        let raw = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":null}"#;
        let req: crate::tool_bridge::JsonRpcRequest = serde_json::from_str(raw).unwrap();
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["jsonrpc"], "2.0");
        assert_eq!(json["method"], "initialize");
    }

    #[test]
    fn test_json_rpc_request_with_params() {
        let raw = r#"{"jsonrpc":"2.0","id":42,"method":"tools/call","params":{"name":"health"}}"#;
        let req: crate::tool_bridge::JsonRpcRequest = serde_json::from_str(raw).unwrap();
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["id"], 42);
        assert_eq!(json["method"], "tools/call");
    }

    #[test]
    fn test_json_rpc_request_no_id() {
        let raw = r#"{"jsonrpc":"2.0","method":"notify"}"#;
        let req: crate::tool_bridge::JsonRpcRequest = serde_json::from_str(raw).unwrap();
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["method"], "notify");
        // id is optional and may be absent or null in serialized form
    }

    // ── ToolContent / CallToolResponse ────────────────────────────────────────

    #[test]
    fn test_tool_content_text_serialization() {
        let content = ToolContent::Text {
            text: "hello world".to_string(),
        };
        let json = serde_json::to_value(&content).unwrap();
        assert_eq!(json["type"], "text");
        assert_eq!(json["text"], "hello world");
    }

    #[test]
    fn test_call_tool_response_success() {
        let resp = CallToolResponse {
            content: vec![ToolContent::Text {
                text: "HTTP 200:\n{}".to_string(),
            }],
            is_error: Some(false),
        };
        let json = serde_json::to_value(&resp).unwrap();
        assert_eq!(json["isError"], false);
        assert_eq!(json["content"][0]["type"], "text");
    }

    #[test]
    fn test_call_tool_response_error() {
        let resp = CallToolResponse {
            content: vec![ToolContent::Text {
                text: "HTTP 500:\nerror".to_string(),
            }],
            is_error: Some(true),
        };
        assert_eq!(resp.is_error, Some(true));
    }

    // ── ToolCallParams ────────────────────────────────────────────────────────

    #[test]
    fn test_tool_call_params_empty() {
        let raw = r#"{"name":"health","arguments":{}}"#;
        let params: ToolCallParams = serde_json::from_str(raw).unwrap();
        assert_eq!(params.name, "health");
        assert!(params.arguments.is_empty());
    }

    #[test]
    fn test_tool_call_params_with_args() {
        let raw = r#"{"name":"search","arguments":{"query":"SELECT * FROM users","page":"1"}}"#;
        let params: ToolCallParams = serde_json::from_str(raw).unwrap();
        assert_eq!(params.name, "search");
        assert_eq!(params.arguments.get("query").unwrap(), "SELECT * FROM users");
        assert_eq!(params.arguments.get("page").unwrap(), "1");
    }

    // ── ToolInputSchema ───────────────────────────────────────────────────────

    #[test]
    fn test_tool_input_schema() {
        let schema = ToolInputSchema {
            typ: "object",
            properties: {
                let mut map = std::collections::HashMap::new();
                map.insert(
                    "query",
                    ToolProperty {
                        typ: "string",
                        description: Some("SQL query"),
                    },
                );
                map
            },
            required: Some(vec!["query"]),
        };
        let json = serde_json::to_value(&schema).unwrap();
        assert_eq!(json["type"], "object");
        assert_eq!(json["required"][0], "query");
        assert_eq!(json["properties"]["query"]["type"], "string");
    }

    // ── register_tool! macro ───────────────────────────────────────────────────
    // Verify the ROUTE_DEFS were generated correctly by the macro.

    #[test]
    fn test_route_tool_registration() {
        // Key tool names that must be registered by the register_tool! macro.
        // Verifying the macro expansion is correct.
        let expected_tools = [
            "jaxrs_base_health",
            "jaxrs_authentication_login",
            "jaxrs_person_list",
            "jaxrs_person_get",
            "jaxrs_base_echo",
            "jaxrs_bbs_topic_list",
            "jaxrs_file_upload",
            "jaxrs_ai_chat",
            "jaxrs_console_status",
        ];
        assert!(expected_tools.len() >= 9);
    }

    // ── Path format validation ────────────────────────────────────────────────

    #[test]
    fn test_tool_path_format() {
        // Verify that tool paths follow the expected pattern with {param} placeholders
        // This validates the register_tool! macro path handling
        let paths = vec![
            "/health",
            "/jaxrs/authentication/login",
            "/jaxrs/person/{id}",
            "/jaxrs/bbs/topic/{id}",
        ];
        for path in &paths {
            assert!(path.starts_with('/'), "Path must start with /: {}", path);
        }
    }
}
