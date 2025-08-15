use owo_colors::OwoColorize;
use std::fmt::Display;

/// Logger utility for colorful console output
pub struct Logger;

impl Logger {
    /// Log startup/initialization messages in bright green
    pub fn startup<T: Display>(msg: T) {
        println!("🚀 {}", msg.to_string().bright_green().bold());
    }

    /// Log successful operations in green
    pub fn success<T: Display>(msg: T) {
        println!("✅ {}", msg.to_string().green());
    }

    /// Log informational messages in blue
    pub fn info<T: Display>(msg: T) {
        println!("ℹ️  {}", msg.to_string().blue());
    }

    /// Log warning messages in yellow
    pub fn warning<T: Display>(msg: T) {
        println!("⚠️  {}", msg.to_string().yellow());
    }

    /// Log error messages in red
    pub fn error<T: Display>(msg: T) {
        println!("❌ {}", msg.to_string().red().bold());
    }

    /// Log operation start in cyan
    pub fn operation_start<T: Display>(msg: T) {
        println!("🔧 {}", msg.to_string().cyan());
    }

    /// Log operation completion in bright blue
    pub fn operation_complete<T: Display>(msg: T) {
        println!("🎯 {}", msg.to_string().bright_blue());
    }

    /// Log network operations in magenta
    pub fn network<T: Display>(msg: T) {
        println!("🌐 {}", msg.to_string().magenta());
    }

    /// Log tool/MCP operations in purple
    pub fn tool<T: Display>(msg: T) {
        println!("🔨 {}", msg.to_string().purple());
    }

    /// Log AI/DeepSeek operations in bright magenta
    pub fn ai<T: Display>(msg: T) {
        println!("🤖 {}", msg.to_string().bright_magenta());
    }

    /// Log data processing in bright cyan
    pub fn data<T: Display>(msg: T) {
        println!("📊 {}", msg.to_string().bright_cyan());
    }

    /// Log configuration operations in yellow
    pub fn config<T: Display>(msg: T) {
        println!("⚙️  {}", msg.to_string().yellow());
    }

    /// Log query operations in bright white
    pub fn query<T: Display>(msg: T) {
        println!("🔍 {}", msg.to_string().bright_white().bold());
    }

    /// Log results in bright yellow with formatting
    pub fn result<T: Display>(label: T, content: &str) {
        println!(
            "📋 {} {}",
            label.to_string().bright_yellow().bold(),
            "=".repeat(50).dimmed()
        );
        println!("{}", content.white());
        println!("{}", "=".repeat(60).dimmed());
    }

    /// Log separator for readability
    pub fn separator() {
        println!("{}", "─".repeat(80).dimmed());
    }

    /// Log section headers
    pub fn section<T: Display>(title: T) {
        println!();
        println!("📄 {}", title.to_string().bright_white().bold().underline());
        Self::separator();
    }
}
