use crate::logger::Logger;
use anyhow::{Context, Result};
use rmcp::{
    RoleClient, ServiceExt,
    model::{
        CallToolRequestParam, ClientCapabilities, Implementation, InitializeRequestParam,
        PaginatedRequestParam, ProtocolVersion,
    },
    service::RunningService,
    transport::child_process::TokioChildProcess,
};
use serde_json::Value;
use std::time::Duration;

pub struct McpExecutor {
    service: RunningService<RoleClient, InitializeRequestParam>,
}

impl McpExecutor {
    pub async fn connect_iplocate(iplocate_dir: &str) -> Result<Self> {
        Logger::network("Connecting to IPLocate MCP server...");
        Logger::info(format!("IPLocate directory: {}", iplocate_dir));

        // Create child process transport for the IPLocate MCP server
        Logger::operation_start("Creating child process transport");
        let mut cmd = tokio::process::Command::new("node");
        cmd.arg("dist/index.js").current_dir(iplocate_dir);
        let transport = TokioChildProcess::new(cmd).context("create TokioChildProcess")?;
        Logger::success("Child process transport created");

        // Create client handler using InitializeRequestParam which implements ClientHandler
        Logger::operation_start("Initializing MCP client handler");
        let handler = InitializeRequestParam {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ClientCapabilities::builder().build(),
            client_info: Implementation {
                name: "deepseek_mcp_iplocate".to_string(),
                version: "0.1.0".to_string(),
            },
        };

        // Start the client service - this automatically handles initialization
        Logger::network("Starting MCP client service and establishing connection...");
        let service = handler
            .serve(transport)
            .await
            .context("failed to start MCP client service")?;

        Logger::success("Successfully connected to IPLocate MCP server");

        // Give the server more time to fully initialize
        Logger::info("Waiting for server to fully initialize...");
        tokio::time::sleep(Duration::from_secs(2)).await;

        Ok(Self { service })
    }

    pub async fn list_tools(&self) -> Result<Vec<String>> {
        Logger::tool("Requesting available tools from IPLocate server...");

        // Try to get tools from the server with a shorter timeout
        let params = Some(PaginatedRequestParam::default());
        let timeout_duration = Duration::from_secs(5);

        match tokio::time::timeout(timeout_duration, self.service.peer().list_tools(params)).await {
            Ok(Ok(result)) => {
                let tool_names: Vec<String> = result
                    .tools
                    .into_iter()
                    .map(|t| t.name.to_string())
                    .collect();
                Logger::success(format!(
                    "Found {} available tools from server",
                    tool_names.len()
                ));
                for tool in &tool_names {
                    Logger::info(format!("  - {}", tool));
                }
                Ok(tool_names)
            }
            Ok(Err(e)) => {
                Logger::warning(format!("Failed to get tools from server: {}", e));
                self.get_fallback_tools()
            }
            Err(_) => {
                Logger::warning("Server took too long to respond, using fallback tool list");
                self.get_fallback_tools()
            }
        }
    }

    fn get_fallback_tools(&self) -> Result<Vec<String>> {
        // Based on our manual testing, we know these tools are available
        let fallback_tools = vec![
            "lookup_ip_address_details".to_string(),
            "lookup_ip_address_location".to_string(),
            "lookup_ip_address_privacy".to_string(),
            "lookup_ip_address_network".to_string(),
            "lookup_ip_address_company".to_string(),
            "lookup_ip_address_abuse_contacts".to_string(),
        ];

        Logger::info("Using fallback tool list based on known IPLocate server capabilities");
        Logger::success(format!(
            "Found {} available tools (fallback)",
            fallback_tools.len()
        ));
        for tool in &fallback_tools {
            Logger::info(format!("  - {}", tool));
        }

        Ok(fallback_tools)
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

        // Add a timeout to prevent indefinite hanging
        let timeout_duration = Duration::from_secs(60); // Longer timeout for tool execution
        let result = tokio::time::timeout(timeout_duration, self.service.peer().call_tool(params))
            .await
            .context("timeout while executing tool on MCP server")?
            .context("failed to call tool")?;

        // Convert the tool result to a JSON Value - use serde_json to serialize the whole result
        let final_result =
            serde_json::to_value(&result).context("failed to serialize tool result")?;

        if result.is_error.unwrap_or(false) {
            Logger::error(format!("Tool execution failed: {}", tool));
        } else {
            Logger::success(format!("Tool '{}' executed successfully", tool));
        }
        Logger::data(format!(
            "Tool result: {}",
            serde_json::to_string_pretty(&final_result)
                .unwrap_or_else(|_| final_result.to_string())
        ));

        Ok(final_result)
    }
}
