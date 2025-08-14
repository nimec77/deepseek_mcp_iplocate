use anyhow::Result;
use deepseek_api::{DeepSeekClient, DeepSeekClientBuilder};

pub fn build_client(api_key: String) -> Result<DeepSeekClient> {
    // DeepSeek API is OpenAI-compatible; the crate handles base URL defaults.
    // You can set a custom base URL with a builder method if needed.  [oai_citation:2‡DeepSeek API Docs](https://api-docs.deepseek.com/?utm_source=chatgpt.com)
    Ok(DeepSeekClientBuilder::new(api_key).build()?)
}
