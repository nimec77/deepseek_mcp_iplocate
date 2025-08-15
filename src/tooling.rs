use deepseek_api::request::{Function, ToolObject, ToolType};
use schemars::schema::SchemaObject;
use serde_json::json;

pub fn mcp_invoke_tool() -> anyhow::Result<ToolObject> {
    let parameters: SchemaObject = serde_json::from_value(json!({
        "type": "object",
        "required": ["server", "tool", "arguments"],
        "properties": {
            "server": {
                "type": "string",
                "description": "MCP server alias (always 'iplocate' here)"
            },
            "tool": {
                "type": "string",
                "description": "Tool name on that MCP server"
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
            name: "mcp_invoke".to_string(),
            description: "Invoke a tool on the IPLocate MCP server".to_string(),
            parameters,
        },
    })
}
