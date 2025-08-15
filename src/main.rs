mod config;
mod deepseek_client;
mod executor;
mod run;
mod tooling;

use anyhow::Result;
use run::run_once;

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = config::AppConfig::load()?;
    let client = deepseek_client::build_client(cfg.deepseek_api_key)?;
    let executor = executor::McpExecutor::connect_iplocate(&cfg.iplocate_dir).await?;

    println!(
        "Available IPLocate tools: {:?}",
        executor.list_tools().await?
    );

    let queries = vec![
        "Get full IP details for 8.8.8.8",
        "Check if 1.1.1.1 is VPN, proxy, or Tor",
    ];

    for q in queries {
        if let Some(ans) = run_once(&client, &cfg.model, q, &executor).await? {
            println!("\n=== Answer for: {q} ===\n{ans}\n");
        }
    }

    Ok(())
}
