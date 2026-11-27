use axum::{
    response::Html,
    routing::{get, post},
    Json, Router,
};
use echo_presentation::Slide;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub service: &'static str,
}

#[derive(Deserialize)]
pub struct IngestRequest {
    pub text: String,
}

#[derive(Serialize)]
pub struct IngestResponse {
    pub slides: Vec<Slide>,
}

#[derive(Deserialize)]
pub struct McpRpcRequest {
    pub jsonrpc: String,
    pub id: Option<serde_json::Value>,
    pub method: String,
    pub params: Option<serde_json::Value>,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "echo-presenter-api",
    })
}

async fn ingest_text(Json(req): Json<IngestRequest>) -> Json<IngestResponse> {
    let beats = echo_ingest::chunk_text_to_beats(&req.text);
    let slides = echo_ingest::beats_to_slides(&beats);
    Json(IngestResponse { slides })
}

async fn mcp_handler(Json(req): Json<McpRpcRequest>) -> Json<serde_json::Value> {
    let id = req.id.unwrap_or(serde_json::Value::Null);
    match req.method.as_str() {
        "tools/list" => {
            let tools = echo_mcp::list_mcp_tools();
            Json(serde_json::json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": { "tools": tools }
            }))
        }
        "tools/call" => {
            let tool_name = req
                .params
                .as_ref()
                .and_then(|p| p["name"].as_str())
                .unwrap_or("unknown");
            Json(serde_json::json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "content": [{ "type": "text", "text": format!("Executed MCP tool: {tool_name}") }]
                }
            }))
        }
        _ => Json(serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": { "code": -32601, "message": "Method not found" }
        })),
    }
}

async fn remote_ui() -> Html<&'static str> {
    Html(
        r#"<!doctype html>
<html><head><meta name="viewport" content="width=device-width,initial-scale=1">
<title>EchoPresenter Remote</title>
<style>body{font-family:system-ui;background:#09090b;color:#f4f4f5;padding:20px;text-align:center}
button{padding:16px 24px;margin:8px;font-size:16px;border-radius:8px;border:1px solid #3f3f46;background:#18181b;color:#fff}</style>
</head><body><h2>EchoPresenter Remote</h2>
<button onclick="fetch('/api/v1/health')">Ping</button>
</body></html>"#,
    )
}

async fn show_state() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ready",
        "active_look": "main-fullscreen"
    }))
}

pub fn build_router() -> Router {
    Router::new()
        .route("/api/v1/health", get(health))
        .route("/api/v1/show/state", get(show_state))
        .route("/api/v1/ingest", post(ingest_text))
        .route("/mcp", post(mcp_handler))
        .route("/remote", get(remote_ui))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_and_mcp_endpoints() {
        let app = build_router();
        let res = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/v1/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);

        let mcp_res = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/mcp")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"jsonrpc":"2.0","id":1,"method":"tools/list"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(mcp_res.status(), StatusCode::OK);
    }
}
