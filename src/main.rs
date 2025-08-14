mod config;
mod deepseek_client;
mod executor;
mod run;
mod tooling;

use anyhow::Result;
use executor::{HttpBridgeExecutor, NoopExecutor, ToolExecutor};
use run::run_once;

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = config::AppConfig::from_env()?;
    let client = deepseek_client::build_client(cfg.deepseek_api_key)?;
    let model = cfg.model.as_str();

    // Choose executor: Noop (log only) or HTTP bridge (recommended)
    let boxed_exec: Box<dyn ToolExecutor> = if let Some(url) = cfg.bridge_executor_url {
        Box::new(HttpBridgeExecutor::new(url))
    } else {
        Box::new(NoopExecutor)
    };

    // --- Example 1: Let the model choose IPLocate tool for a public IP
    let q1 = "Using the IPLocate MCP server, get full details for 8.8.8.8 and summarize.";
    if let Some(answer) = run_once(&client, model, q1, true, boxed_exec.as_ref()).await? {
        println!("\n=== Answer (Example 1) ===\n{answer}\n");
    }

    // --- Example 2: Ask for privacy check (VPN/Proxy) on a specific IP
    let q2 = "Check if 1.1.1.1 is VPN/Proxy/Tor (use the IPLocate MCP server) and summarize safety considerations.";
    if let Some(answer) = run_once(&client, model, q2, true, boxed_exec.as_ref()).await? {
        println!("\n=== Answer (Example 2) ===\n{answer}\n");
    }

    Ok(())
}
