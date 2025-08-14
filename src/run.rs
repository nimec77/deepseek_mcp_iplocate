use anyhow::{Context, Result};
use deepseek_api::{
    CompletionsRequestBuilder,
    RequestBuilder,
    request::{MessageRequest, ToolMessageRequest},
    response::{FinishReason, ModelType},
};
use serde_json::{Value, json};

use crate::{
    executor::{BridgeInvokePayload, ToolExecutor},
    tooling::mcp_invoke_tool,
};

/// One-shot round that (a) asks DeepSeek with tools, (b) executes tool_calls via executor (optional),
/// and (c) returns final assistant text if a tool was executed.
pub async fn run_once<E: ToolExecutor + ?Sized>(
    client: &deepseek_api::DeepSeekClient,
    model: &str,
    user_query: &str,
    prefer_iplocate_hint: bool,
    executor: &E,
) -> Result<Option<String>> {
    // System prompt nudges the model to call our tool first.
    // We also hint the IPLocate MCP server and its tool names.  [oai_citation:3‡GitHub](https://github.com/iplocate/mcp-server-iplocate)
    let mut messages = vec![
        MessageRequest::sys(
            "You can use external tools only via `mcp.invoke`. \
             If a tool can help, you MUST call it before answering. \
             Preferred MCP server alias: 'iplocate'. Common tools: \
             'lookup_ip_address_details', 'lookup_ip_address_location', \
             'lookup_ip_address_privacy', 'lookup_ip_address_network', \
             'lookup_ip_address_company', 'lookup_ip_address_abuse_contacts'.",
        ),
        MessageRequest::user(user_query),
    ];

    if prefer_iplocate_hint {
        messages.push(MessageRequest::sys(
            "If the user asks about an IP, use 'mcp.invoke' with server='iplocate'. \
             Choose the most specific IPLocate tool and pass arguments like {\"ip\": \"1.2.3.4\"}.",
        ));
    }

    let tools = vec![mcp_invoke_tool()?];

    let first = CompletionsRequestBuilder::new(&messages)
        .use_model(match model {
            "deepseek-reasoner" => ModelType::DeepSeekReasoner,
            _ => ModelType::DeepSeekChat,
        })
        .tools(&tools)
        .do_request(client)
        .await?
        .must_response();

    let choice = &first.choices[0];
    if choice.finish_reason == FinishReason::ToolCalls {
        let Some(assistant_msg) = &choice.message else {
            return Ok(None);
        };
        let Some(tool_calls) = &assistant_msg.tool_calls else {
            return Ok(None);
        };

        // record assistant tool-call request
        messages.push(MessageRequest::Assistant(assistant_msg.clone()));

        // execute each call (optionally through a bridge)
        for call in tool_calls {
            let args: Value =
                serde_json::from_str(&call.function.arguments).unwrap_or_else(|_| json!({}));

            let payload = BridgeInvokePayload {
                server: extract(&args, "server").unwrap_or_else(|| "iplocate".into()),
                tool: extract(&args, "tool").unwrap_or_default(),
                arguments: args.get("arguments").cloned().unwrap_or_else(|| json!({})),
            };

            let result = executor
                .execute(&payload)
                .await
                .with_context(|| format!("executing {}", payload.tool))?;

            // Feed result back
            let tool_msg = ToolMessageRequest::new(&result.to_string(), &call.id);
            messages.push(MessageRequest::Tool(tool_msg));
        }

        // Ask model to finalize
        let final_resp = CompletionsRequestBuilder::new(&messages)
            .use_model(match model {
                "deepseek-reasoner" => ModelType::DeepSeekReasoner,
                _ => ModelType::DeepSeekChat,
            })
            .tools(&tools)
            .do_request(client)
            .await?
            .must_response();

        let content = final_resp.choices[0].message.as_ref().and_then(|m| {
            if m.content.is_empty() {
                None
            } else {
                Some(m.content.clone())
            }
        });

        return Ok(content);
    }

    // If model answered directly (unlikely with our prompts), return it.
    Ok(choice.message.as_ref().map(|m| m.content.clone()))
}

fn extract(v: &serde_json::Value, key: &str) -> Option<String> {
    v.get(key).and_then(|x| x.as_str()).map(|s| s.to_string())
}
