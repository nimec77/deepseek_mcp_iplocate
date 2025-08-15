use anyhow::{Context, Result};
use serde_json::Value;
use rmcp::{
    service::{RunningService, Peer},
    RoleClient, ClientHandler, ServiceExt,
    model::{ClientCapabilities, CallToolRequestParam, PaginatedRequestParam, InitializeRequestParam, 
            ProtocolVersion, Implementation},
    transport::child_process::TokioChildProcess,
};

pub struct McpExecutor {
    service: RunningService<RoleClient, SimpleClientHandler>,
}

// Simple client handler implementation
#[derive(Clone)]
struct SimpleClientHandler {
    peer: Option<Peer<RoleClient>>,
}

impl SimpleClientHandler {
    fn new() -> Self {
        Self { peer: None }
    }
}

impl ClientHandler for SimpleClientHandler {
    fn get_peer(&self) -> Option<Peer<RoleClient>> {
        self.peer.clone()
    }

    fn set_peer(&mut self, peer: Peer<RoleClient>) {
        self.peer = Some(peer);
    }

    fn get_info(&self) -> InitializeRequestParam {
        InitializeRequestParam {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ClientCapabilities::builder().build(),
            client_info: Implementation {
                name: "deepseek_mcp_iplocate".to_string(),
                version: "0.1.0".to_string(),
            },
        }
    }
}

impl McpExecutor {
    pub async fn connect_iplocate(iplocate_dir: &str) -> Result<Self> {
        // Create child process transport for the IPLocate MCP server
        let mut cmd = tokio::process::Command::new("node");
        cmd.arg("dist/index.js")
           .current_dir(iplocate_dir);
        let transport = TokioChildProcess::new(&mut cmd)
            .context("create TokioChildProcess")?;

        // Create client handler
        let handler = SimpleClientHandler::new();
        
        // Start the client service - this automatically handles initialization
        let service = handler.serve(transport)
            .await
            .context("failed to start MCP client service")?;

        Ok(Self { service })
    }

    pub async fn list_tools(&self) -> Result<Vec<String>> {
        let params = PaginatedRequestParam::default();
        let result = self.service.peer()
            .list_tools(params)
            .await
            .context("failed to list tools")?;
        
        Ok(result.tools.into_iter().map(|t| t.name.to_string()).collect())
    }

    pub async fn execute(&self, tool: &str, args: Value) -> Result<Value> {
        let arguments = match args {
            Value::Object(map) => Some(map),
            _ => None,
        };
        
        let params = CallToolRequestParam {
            name: tool.to_string().into(),
            arguments,
        };
        
        let result = self.service.peer()
            .call_tool(params)
            .await
            .context("failed to call tool")?;
            
        // Convert the tool result to a JSON Value
        let content_value = result.content.into_iter()
            .map(|content| {
                // Convert Content to JSON
                // Content is an alias for Annotated<RawContent>
                match &content.raw {
                    rmcp::model::RawContent::Text(text) => {
                        serde_json::json!({
                            "type": "text",
                            "text": text
                        })
                    },
                    rmcp::model::RawContent::Image(image_content) => {
                        serde_json::json!({
                            "type": "image",
                            "data": image_content.data,
                            "mime_type": image_content.mime_type
                        })
                    },
                    rmcp::model::RawContent::Resource(resource_content) => {
                        serde_json::json!({
                            "type": "resource",
                            "resource": resource_content.resource
                        })
                    }
                }
            })
            .collect::<Vec<_>>();
            
        Ok(serde_json::json!({
            "content": content_value,
            "is_error": result.is_error.unwrap_or(false)
        }))
    }
}
