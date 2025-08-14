use deepseek_api::request::{Function, ToolObject, ToolType};
use schemars::schema::SchemaObject;
use serde_json::json;

/// Our single "virtual" tool the model can call. The app/bridge actually executes it.
pub fn mcp_invoke_tool() -> anyhow::Result<ToolObject> {
    let parameters: SchemaObject = serde_json::from_value(json!({
        "type": "object",
        "required": ["server", "tool", "arguments"],
        "properties": {
            "server": {
                "type": "string",
                "description": "MCP server alias, e.g. 'iplocate'"
            },
            "tool": {
                "type": "string",
                "description": "MCP tool name (e.g., 'lookup_ip_address_details')"
            },
            "arguments": {
                "type": "object",
                "description": "Tool arguments JSON"
            }
        }
    }))?;
    Ok(ToolObject {
        tool_type: ToolType::Function,
        function: Function {
            name: "mcp.invoke".to_string(),
            description: "Invoke a tool on a specified MCP server. The client/bridge will execute it and return the result."
                .to_string(),
            parameters,
        },
    })
}
