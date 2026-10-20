// SPDX-License-Identifier: AGPL-3.0-or-later
//! JSON-RPC 2.0 wire types — canonical forms from `cellmembrane-types`.

pub use cellmembrane_types::{JsonRpcError, JsonRpcRequest, JsonRpcResponse};

/// JSON-RPC 2.0 protocol version.
pub const JSONRPC_VERSION: &str = "2.0";

/// Standard JSON-RPC error codes (re-exported for use with extension methods).
const PARSE_ERROR: i32 = -32_700;
const INVALID_REQUEST: i32 = -32_600;
const METHOD_NOT_FOUND: i32 = -32_601;
const INVALID_PARAMS: i32 = -32_602;
const INTERNAL_ERROR: i32 = -32_603;

/// Extension methods for `JsonRpcResponse` (bingoCube convenience constructors).
pub trait JsonRpcResponseExt {
    /// Build an error response from an error object.
    fn error_obj(id: serde_json::Value, error: JsonRpcError) -> JsonRpcResponse;
    /// Parse error (-32700).
    fn parse_error(id: serde_json::Value, detail: impl Into<String>) -> JsonRpcResponse;
    /// Invalid request (-32600).
    fn invalid_request(id: serde_json::Value, detail: impl Into<String>) -> JsonRpcResponse;
    /// Method not found (-32601).
    fn method_not_found(id: serde_json::Value, method: &str) -> JsonRpcResponse;
    /// Invalid params (-32602).
    fn invalid_params(id: serde_json::Value, detail: impl Into<String>) -> JsonRpcResponse;
    /// Internal error (-32603).
    fn internal(id: serde_json::Value, detail: impl Into<String>) -> JsonRpcResponse;
}

impl JsonRpcResponseExt for JsonRpcResponse {
    fn error_obj(id: serde_json::Value, error: JsonRpcError) -> Self {
        Self {
            jsonrpc: "2.0".into(),
            result: None,
            error: Some(error),
            id,
        }
    }

    fn parse_error(id: serde_json::Value, detail: impl Into<String>) -> Self {
        Self::error(id, PARSE_ERROR, detail)
    }

    fn invalid_request(id: serde_json::Value, detail: impl Into<String>) -> Self {
        Self::error(id, INVALID_REQUEST, detail)
    }

    fn method_not_found(id: serde_json::Value, method: &str) -> Self {
        Self::error(id, METHOD_NOT_FOUND, format!("method not found: {method}"))
    }

    fn invalid_params(id: serde_json::Value, detail: impl Into<String>) -> Self {
        Self::error(id, INVALID_PARAMS, detail)
    }

    fn internal(id: serde_json::Value, detail: impl Into<String>) -> Self {
        Self::error(id, INTERNAL_ERROR, detail)
    }
}
