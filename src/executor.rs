use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Minimal interface to keep the app *decoupled* from any MCP server.
/// Implementations may call an internal bridge that actually talks to MCP.
pub trait ToolExecutor: Send + Sync {
    fn name(&self) -> &'static str;
    fn is_enabled(&self) -> bool { true }
    fn can_execute(&self, tool_name: &str) -> bool;
    fn execute(&self, payload: &BridgeInvokePayload) -> std::pin::Pin<Box<dyn std::future::Future<Output=Result<Value>> + Send + '_>>;
}

/// A generic payload you can POST to your bridge/executor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeInvokePayload {
    pub server: String,       // e.g. "iplocate"
    pub tool: String,         // e.g. "lookup_ip_address_details"
    pub arguments: Value,     // e.g. {"ip":"8.8.8.8"}
}

/// No-op executor that just logs (default, keeps client from touching MCP)
pub struct NoopExecutor;
impl ToolExecutor for NoopExecutor {
    fn name(&self) -> &'static str { "noop" }
    fn can_execute(&self, _tool_name: &str) -> bool { true }
    fn execute(&self, payload: &BridgeInvokePayload) -> std::pin::Pin<Box<dyn std::future::Future<Output=Result<Value>> + Send + '_>> {
        let p = payload.clone();
        Box::pin(async move {
            // Return a stub so the flow can continue if you want to simulate.
            Ok(serde_json::json!({"note":"Noop executor; not calling any MCP server","payload":p}))
        })
    }
}

/// Example HTTP executor that forwards to your *own* bridge.
/// Bridge contract: POST {server,tool,arguments} -> {result: <json>}
pub struct HttpBridgeExecutor {
    url: String,
    http: Client,
}

impl HttpBridgeExecutor {
    pub fn new(url: String) -> Self { Self { url, http: Client::new() } }
}

impl ToolExecutor for HttpBridgeExecutor {
    fn name(&self) -> &'static str { "http-bridge" }
    fn can_execute(&self, _tool_name: &str) -> bool { true }
    fn execute(&self, payload: &BridgeInvokePayload) -> std::pin::Pin<Box<dyn std::future::Future<Output=Result<Value>> + Send + '_>> {
        let url = self.url.clone();
        let http = self.http.clone();
        let body = payload.clone();
        Box::pin(async move {
            let resp = http.post(url).json(&body).send().await?;
            let val: Value = resp.json().await?;
            Ok(val)
        })
    }
}
