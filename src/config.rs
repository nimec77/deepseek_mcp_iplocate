use std::env;

pub struct AppConfig {
    pub deepseek_api_key: String,
    pub model: String,
    pub iplocate_dir: String,
}

impl AppConfig {
    /// Load configuration from .env file and environment variables
    /// Environment variables take precedence over .env file values
    pub fn load() -> anyhow::Result<Self> {
        // Load .env file if it exists (silently ignore if it doesn't exist)
        let _ = dotenvy::dotenv();
        
        // Load from environment variables (which now includes values from .env file)
        Self::from_env()
    }

    /// Load configuration directly from environment variables only
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            deepseek_api_key: env::var("DEEPSEEK_API_KEY")?,
            model: env::var("DEEPSEEK_MODEL").unwrap_or_else(|_| "deepseek-chat".into()),
            iplocate_dir: env::var("IPLOCATE_DIR")
                .unwrap_or_else(|_| "./mcp-server-iplocate".into()),
        })
    }
}
