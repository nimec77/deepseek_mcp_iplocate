use anyhow::Result;
use deepseek_api::{DeepSeekClient, DeepSeekClientBuilder};
use crate::logger::Logger;

pub fn build_client(api_key: String) -> Result<DeepSeekClient> {
    Logger::ai("Building DeepSeek API client...");
    
    let client = DeepSeekClientBuilder::new(api_key).build()?;
    
    Logger::success("DeepSeek client created successfully");
    Ok(client)
}
