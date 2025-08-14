use std::env;

pub struct AppConfig {
    pub deepseek_api_key: String,
    pub model: String,
    pub bridge_executor_url: Option<String>, // optional: where to POST tool calls
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            deepseek_api_key: env::var("DEEPSEEK_API_KEY")?,
            model: env::var("DEEPSEEK_MODEL").unwrap_or_else(|_| "deepseek-chat".into()),
            bridge_executor_url: env::var("BRIDGE_EXECUTOR_URL")
                .ok()
                .filter(|s| !s.is_empty()),
        })
    }
}
