use anyhow::Result;
use deepseek_api::{DeepSeekClient, DeepSeekClientBuilder};

pub fn build_client(api_key: String) -> Result<DeepSeekClient> {
    Ok(DeepSeekClientBuilder::new(api_key).build()?)
}
