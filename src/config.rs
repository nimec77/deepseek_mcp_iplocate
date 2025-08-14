use std::env;

pub struct AppConfig {
    pub deepseek_api_key: String,
    pub model: String,
    pub iplocate_dir: String,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            deepseek_api_key: env::var("DEEPSEEK_API_KEY")?,
            model: env::var("DEEPSEEK_MODEL").unwrap_or_else(|_| "deepseek-chat".into()),
            iplocate_dir: env::var("IPLOCATE_DIR")
                .unwrap_or_else(|_| "./mcp-server-iplocate".into()),
        })
    }
}
