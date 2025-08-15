use crate::logger::Logger;
use anyhow::Result;
use deepseek_api::{DeepSeekClient, DeepSeekClientBuilder};

pub fn build_client(api_key: String) -> Result<DeepSeekClient> {
    Logger::ai("Building DeepSeek API client...");

    let client = DeepSeekClientBuilder::new(api_key).build()?;

    Logger::success("DeepSeek client created successfully");
    Ok(client)
}
