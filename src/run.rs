use anyhow::Result;
use deepseek_api::{
    CompletionsRequestBuilder,
    request::{MessageRequest, ToolMessageRequest},
    response::FinishReason,
};
use serde_json::{Value, json};

use crate::{executor::McpExecutor, tooling::mcp_invoke_tool};

pub async fn run_once(
    client: &deepseek_api::DeepSeekClient,
    model: &str,
    query: &str,
    executor: &McpExecutor,
) -> Result<Option<String>> {
    let mut messages = vec![
        MessageRequest::system(
            "You may call tools only via `mcp.invoke`. \
             For IP-related queries, always use server='iplocate' \
             and one of the known IPLocate tools.",
        ),
        MessageRequest::user(query),
    ];

    let tools = vec![mcp_invoke_tool()?];

    let first = CompletionsRequestBuilder::new(&messages)
        .model(model)
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

        messages.push(MessageRequest::Assistant(assistant_msg.clone()));

        for call in tool_calls {
            let args: Value = serde_json::from_str(&call.function.arguments).unwrap_or(json!({}));
            let server = args
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

        let final_resp = CompletionsRequestBuilder::new(&messages)
            .model(model)
            .tools(&tools)
            .do_request(client)
            .await?
            .must_response();

        return Ok(final_resp.choices[0]
            .message
            .as_ref()
            .map(|m| m.content.clone()));
    }

    Ok(choice.message.as_ref().map(|m| m.content.clone()))
}
