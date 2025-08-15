mod config;
mod deepseek_client;
mod executor;
mod logger;
mod run;
mod tooling;

use anyhow::Result;
use logger::Logger;
use run::run_once;

#[tokio::main]
async fn main() -> Result<()> {
    Logger::startup("🌟 DeepSeek MCP IPLocate Application Starting 🌟");
    Logger::separator();

    // Load configuration
    let cfg = config::AppConfig::load()?;

    // Build DeepSeek client
    let client = deepseek_client::build_client(cfg.deepseek_api_key)?;

    // Connect to IPLocate MCP server
    let executor = executor::McpExecutor::connect_iplocate(&cfg.iplocate_dir).await?;

    // List available tools
    Logger::section("Available Tools");
    let _tools = executor.list_tools().await?;

    // Process queries
    Logger::section("Processing IP Queries");
    let queries = [
        "Get full IP details for 8.8.8.8",
        "Check if 1.1.1.1 is VPN, proxy, or Tor",
    ];

    Logger::info(format!("Found {} queries to process", queries.len()));

    for (i, q) in queries.iter().enumerate() {
        Logger::query(format!("Query {}/{}: {}", i + 1, queries.len(), q));

        if let Some(ans) = run_once(&client, &cfg.model, q, &executor).await? {
            Logger::result(format!("Answer for: {}", q), &ans);
        } else {
            Logger::warning(format!("No answer received for query: {}", q));
        }

        Logger::separator();
    }

    Logger::startup("🎉 Application completed successfully! 🎉");
    Ok(())
}
