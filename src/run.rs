use anyhow::Result;
use deepseek_api::{
    CompletionsRequestBuilder, RequestBuilder,
    request::{MessageRequest, ToolMessageRequest},
    response::{FinishReason, ModelType},
};
use serde_json::{Value, json};

use crate::{executor::McpExecutor, tooling::mcp_invoke_tool, logger::Logger};

pub async fn run_once(
    client: &deepseek_api::DeepSeekClient,
    model: &str,
    query: &str,
    executor: &McpExecutor,
) -> Result<Option<String>> {
    Logger::separator();
    Logger::query(format!("Processing query: \"{}\"", query));
    Logger::ai(format!("Using model: {}", model));
    let mut messages = vec![
        MessageRequest::sys(
            "You may call tools only via `mcp_invoke` with server='iplocate'. \
             Available IPLocate tools:\n\
             - lookup_ip_address_details: Get comprehensive info about an IP\n\
             - lookup_ip_address_location: Get geographic location of an IP\n\
             - lookup_ip_address_privacy: Check if IP is VPN/proxy/Tor\n\
             - lookup_ip_address_network: Get network/ASN info for an IP\n\
             - lookup_ip_address_company: Get company/organization info for an IP\n\
             - lookup_ip_address_abuse_contacts: Get abuse contact info for an IP\n\
             Each tool takes an optional 'ip' parameter (IPv4/IPv6 address).",
        ),
        MessageRequest::user(query),
    ];

    let tools = vec![mcp_invoke_tool()?];

    let model_type = match model {
        "deepseek-chat" => ModelType::DeepSeekChat,
        "deepseek-reasoner" => ModelType::DeepSeekReasoner,
        _ => ModelType::DeepSeekChat, // Default to DeepSeekChat for unknown models
    };
    
    Logger::ai("Sending initial request to DeepSeek API...");
    let first = CompletionsRequestBuilder::new(&messages)
        .use_model(model_type.clone())
        .tools(&tools)
        .do_request(client)
        .await?
        .must_response();
    
    Logger::success("Received response from DeepSeek API");

    let choice = &first.choices[0];
    if choice.finish_reason == FinishReason::ToolCalls {
        Logger::ai("DeepSeek wants to make tool calls");
        let Some(assistant_msg) = &choice.message else {
            Logger::warning("No assistant message found in response");
            return Ok(None);
        };
        let Some(tool_calls) = &assistant_msg.tool_calls else {
            Logger::warning("No tool calls found in assistant message");
            return Ok(None);
        };

        Logger::tool(format!("Processing {} tool call(s)", tool_calls.len()));
        messages.push(MessageRequest::Assistant(assistant_msg.clone()));

        for (i, call) in tool_calls.iter().enumerate() {
            Logger::tool(format!("Executing tool call {}/{}: {}", i + 1, tool_calls.len(), call.function.name));
            let args: Value = serde_json::from_str(&call.function.arguments).unwrap_or(json!({}));
            let _server = args
                .get("server")
                .and_then(|v| v.as_str())
                .unwrap_or("iplocate");
            let tool_name = args
                .get("tool")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            let tool_args = args.get("arguments").cloned().unwrap_or(json!({}));

            let result = executor.execute(tool_name, tool_args).await?;
            messages.push(MessageRequest::Tool(ToolMessageRequest::new(
                &result.to_string(),
                &call.id,
            )));
        }

        Logger::ai("Sending final request to DeepSeek with tool results...");
        let final_resp = CompletionsRequestBuilder::new(&messages)
            .use_model(model_type)
            .tools(&tools)
            .do_request(client)
            .await?
            .must_response();

        Logger::success("Received final response from DeepSeek");
        let response_content = final_resp.choices[0]
            .message
            .as_ref()
            .map(|m| m.content.clone());
        
        Logger::operation_complete("Query processing completed with tool calls");
        return Ok(response_content);
    }

    Logger::success("DeepSeek provided direct response without tool calls");
    Logger::operation_complete("Query processing completed");
    Ok(choice.message.as_ref().map(|m| m.content.clone()))
}
