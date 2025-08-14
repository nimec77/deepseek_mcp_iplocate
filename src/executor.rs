use anyhow::{Context, Result};
use serde_json::Value;
use tokio::process::Command;
use modelcontextprotocol_client as mcp;

#[derive(Clone)]
pub struct McpExecutor {
    client: mcp::Client,
}

impl McpExecutor {
    pub async fn connect_iplocate(iplocate_dir: &str) -> Result<Self> {
        let transport = mcp::transport::StdioTransport::spawn(
            "node",
            &[ "build/index.js".to_string() ],
            Some(iplocate_dir),
        )
        .await
        .context("spawn IPLocate MCP server")?;
        let client = mcp::Client::new(transport);
        client.initialize().await?;
        Ok(Self { client })
    }

    pub async fn list_tools(&self) -> Result<Vec<String>> {
        let tools = self.client.list_tools().await?;
        Ok(tools.into_iter().map(|t| t.name).collect())
    }

    pub async fn execute(&self, tool: &str, args: Value) -> Result<Value> {
        let res = self.client.call_tool(tool, args).await?;
        Ok(res)
    }
}
