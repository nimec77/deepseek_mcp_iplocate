use anyhow::{Context, Result};
use serde_json::Value;
use rmcp::{
    service::{RunningService, Peer},
    RoleClient, ClientHandler, ServiceExt,
    model::{ClientCapabilities, CallToolRequestParam, PaginatedRequestParam, InitializeRequestParam, 
            ProtocolVersion, Implementation},
    transport::child_process::TokioChildProcess,
};
use crate::logger::Logger;

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
        Logger::network("Connecting to IPLocate MCP server...");
        Logger::info(format!("IPLocate directory: {}", iplocate_dir));
        
        // Create child process transport for the IPLocate MCP server
        Logger::operation_start("Creating child process transport");
        let mut cmd = tokio::process::Command::new("node");
        cmd.arg("dist/index.js")
           .current_dir(iplocate_dir);
        let transport = TokioChildProcess::new(&mut cmd)
            .context("create TokioChildProcess")?;
        Logger::success("Child process transport created");

        // Create client handler
        Logger::operation_start("Initializing MCP client handler");
        let handler = SimpleClientHandler::new();
        
        // Start the client service - this automatically handles initialization
        Logger::network("Starting MCP client service and establishing connection...");
        let service = handler.serve(transport)
            .await
            .context("failed to start MCP client service")?;

        Logger::success("Successfully connected to IPLocate MCP server");
        Ok(Self { service })
    }

    pub async fn list_tools(&self) -> Result<Vec<String>> {
        Logger::tool("Requesting available tools from IPLocate server...");
        
        let params = PaginatedRequestParam::default();
        let result = self.service.peer()
            .list_tools(params)
            .await
            .context("failed to list tools")?;
        
        let tool_names: Vec<String> = result.tools.into_iter().map(|t| t.name.to_string()).collect();
        Logger::success(format!("Found {} available tools", tool_names.len()));
        for tool in &tool_names {
            Logger::info(format!("  - {}", tool));
        }
        
        Ok(tool_names)
    }

    pub async fn execute(&self, tool: &str, args: Value) -> Result<Value> {
        Logger::tool(format!("Executing tool: {}", tool));
        Logger::data(format!("Tool arguments: {}", args));
        
        let arguments = match args {
            Value::Object(map) => Some(map),
            _ => None,
        };
        
        let params = CallToolRequestParam {
            name: tool.to_string().into(),
            arguments,
        };
        
        Logger::network("Sending tool call request to IPLocate server...");
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
            
        let final_result = serde_json::json!({
            "content": content_value,
            "is_error": result.is_error.unwrap_or(false)
        });
        
        if result.is_error.unwrap_or(false) {
            Logger::error(format!("Tool execution failed: {}", tool));
        } else {
            Logger::success(format!("Tool '{}' executed successfully", tool));
        }
        Logger::data(format!("Tool result: {}", final_result));
        
        Ok(final_result)
    }
}
