# DeepSeek MCP IPLocate

A Rust-based AI-powered IP analysis tool that combines the DeepSeek language model with IPLocate geolocation services through the Model Context Protocol (MCP).

## Overview

This project demonstrates how to build an intelligent IP analysis system that can understand natural language queries and automatically invoke the appropriate IP geolocation tools. It features a robust, production-ready implementation with comprehensive error handling, rich logging, and fallback mechanisms. It uses:

- **DeepSeek AI Model** - For understanding natural language queries about IP addresses
- **Model Context Protocol (MCP)** - For seamless integration between AI and external tools via the `rmcp` crate
- **IPLocate MCP Server** - Specialized tools for comprehensive IP analysis and geolocation
- **Async Rust & Tokio** - High-performance concurrent execution with proper timeout and error handling
- **Rich Logging System** - Colorful, emoji-enhanced console output for excellent developer experience
- **Robust Configuration** - Flexible environment-based configuration with `.env` file support

## Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────────┐
│   User Query    │───▶│   DeepSeek AI    │───▶│   IPLocate Tools    │
│ "Check if this  │    │     Model        │    │  • lookup_ip_...    │
│  IP is from a   │    │                  │    │  • _details         │
│  VPN service"   │    │  (via MCP)       │    │  • _location        │
│                 │    │                  │    │  • _privacy         │
│                 │    │                  │    │  • _network         │
│                 │    │                  │    │  • _company         │
│                 │    │                  │    │  • _abuse_contacts  │
└─────────────────┘    └──────────────────┘    └─────────────────────┘
          │                       │                        │
          │              ┌────────▼────────┐              │
          │              │  Rich Logger    │              │
          │              │ (Emojis/Colors) │              │
          │              └─────────────────┘              │
          │                       │                        │
          │              ┌────────▼────────┐              │
          └─────────────▶│ Error Handling  │◀─────────────┘
                         │ & Fallbacks     │
                         └─────────────────┘
```

The AI model processes natural language queries about IP addresses and automatically selects and calls the appropriate IPLocate tools through the MCP protocol to provide comprehensive answers.

## Features

- **Natural Language Processing**: Ask questions about IP addresses in plain English
- **Comprehensive IP Analysis**: 
  - Detailed geolocation information (lookup_ip_address_location)
  - VPN/Proxy/Tor detection (lookup_ip_address_privacy)
  - ISP and organization details (lookup_ip_address_company)
  - Network and ASN information (lookup_ip_address_network)
  - Abuse contact information (lookup_ip_address_abuse_contacts)
  - Complete IP details (lookup_ip_address_details)
- **Rich Developer Experience**: 
  - 🎨 Colorful console output with emojis
  - 📊 Detailed operation logging
  - 🔍 Query processing visualization
  - ⚡ Real-time status updates
- **Robust Error Handling**: 
  - Timeout protection for all operations
  - Graceful fallback mechanisms
  - Comprehensive error context preservation
- **Production-Ready Architecture**: 
  - Async performance with tokio
  - Proper resource management
  - Child process lifecycle management
- **Flexible Configuration**: 
  - Environment-based configuration
  - `.env` file support with override capability
  - Multiple model support (deepseek-chat, deepseek-reasoner)
- **MCP Integration**: Seamless tool integration using the Model Context Protocol via the `rmcp` crate

## Prerequisites

1. **DeepSeek API Key**: Get your API key from [DeepSeek](https://platform.deepseek.com/)
2. **IPLocate MCP Server**: Clone and build the [mcp-server-iplocate](https://github.com/modelcontextprotocol/servers/tree/main/src/iplocate) repository
3. **Node.js**: Required to run the IPLocate MCP server

## Installation

1. Clone this repository:
```bash
git clone <repository-url>
cd deepseek_mcp_iplocate
```

2. Set up the IPLocate MCP server:
```bash
git clone https://github.com/modelcontextprotocol/servers.git
cd servers/src/iplocate
npm install
npm run build
```

3. Set up configuration:

**Option A: Using a .env file (recommended)**
```bash
# Create a .env file in the project root
cat > .env << EOF
DEEPSEEK_API_KEY=your-deepseek-api-key
DEEPSEEK_MODEL=deepseek-chat
IPLOCATE_DIR=/path/to/servers/src/iplocate
EOF
```

**Option B: Using environment variables**
```bash
export DEEPSEEK_API_KEY="your-deepseek-api-key"
export DEEPSEEK_MODEL="deepseek-chat"  # or "deepseek-reasoner"
export IPLOCATE_DIR="/path/to/servers/src/iplocate"
```

Note: Environment variables take precedence over .env file values if both are set.

4. Build and run:
```bash
cargo build --release
cargo run
```

## Usage Examples

The application comes with built-in example queries, but you can modify them or extend the functionality:

### Example 1: Basic IP Information
```rust
"Get full IP details for 8.8.8.8"
```

**Expected Output:**
```
🔍 Query 1/2: Get full IP details for 8.8.8.8
🤖 Using model: deepseek-chat
🔨 Executing tool call 1/1: mcp_invoke
🔨 Executing tool: lookup_ip_address_details
✅ Tool 'lookup_ip_address_details' executed successfully
📋 Answer for: Get full IP details for 8.8.8.8
```
- Geographic location (country, city, coordinates)
- ISP and organization information
- Time zone data
- Network details

### Example 2: Security Analysis
```rust
"Check if 1.1.1.1 is VPN, proxy, or Tor"
```

**Expected Output:**
```
🔍 Query 2/2: Check if 1.1.1.1 is VPN, proxy, or Tor
🤖 DeepSeek wants to make tool calls
🔨 Processing 1 tool call(s)
🔨 Executing tool: lookup_ip_address_privacy
📊 Tool result: {detailed privacy analysis}
```
- VPN detection results
- Proxy service analysis
- Tor network detection
- Security risk assessment

### Example 3: Custom Queries (extend the code)
```rust
// In main.rs, replace the queries array:
let queries = [
    "What country is IP 203.0.113.0 located in?",
    "Is 192.168.1.1 a private IP address?",
    "Give me the ISP information for 1.2.3.4",
    "Get abuse contact information for 8.8.8.8",
    "Analyze the network details of 1.1.1.1",
];

for (i, q) in queries.iter().enumerate() {
    Logger::query(format!("Query {}/{}: {}", i + 1, queries.len(), q));
    // ... processing logic
}
```

## Configuration

The application supports configuration through both `.env` files and environment variables. Environment variables take precedence over `.env` file values.

### Using .env File (Recommended)

Create a `.env` file in the project root directory:

```bash
# DeepSeek API Key (required)
DEEPSEEK_API_KEY=your_deepseek_api_key_here

# DeepSeek Model (optional, defaults to "deepseek-chat")
DEEPSEEK_MODEL=deepseek-chat

# IPLocate MCP Server Directory (optional, defaults to "./mcp-server-iplocate")
IPLOCATE_DIR=./mcp-server-iplocate
```

### Configuration Variables

| Variable | Description | Default Value | Required |
|----------|-------------|---------------|----------|
| `DEEPSEEK_API_KEY` | Your DeepSeek API key | None | ✅ Yes |
| `DEEPSEEK_MODEL` | Model to use | `deepseek-chat` | No |
| `IPLOCATE_DIR` | Path to IPLocate MCP server | `./mcp-server-iplocate` | No |

### Supported Models
- `deepseek-chat` - General purpose conversational model (default)
- `deepseek-reasoner` - Advanced reasoning model for complex queries

## Code Structure

```
src/
├── main.rs           # Entry point with hardcoded demo queries
├── config.rs         # Environment configuration (.env + env vars)
├── deepseek_client.rs # DeepSeek API client builder
├── executor.rs       # MCP client, connection management, and tool execution
├── run.rs           # Conversation logic with tool calling support
├── tooling.rs       # Tool schema definitions for MCP invoke
└── logger.rs        # Rich logging system with emojis and colors
```

### Key Components

- **McpExecutor**: 
  - Manages connection to IPLocate MCP server via child process
  - Implements timeout protection and fallback mechanisms
  - Handles tool execution with comprehensive error handling
  - Features graceful server initialization waiting
- **DeepSeekClient**: Handles API communication with DeepSeek models
- **AppConfig**: Flexible configuration system supporting `.env` files and environment variables
- **Logger**: Rich console output system with:
  - 🎨 Color-coded operation types
  - 📊 Structured data display
  - 🔍 Query processing visualization
  - ⚡ Real-time progress indicators
- **Tool Integration**: 
  - Automatically converts AI tool calls to MCP server invocations
  - Supports all IPLocate server tools with proper parameter mapping
  - Robust error handling and result formatting

## Dependencies

The project uses the following key dependencies:

```toml
[dependencies]
anyhow = "1"                     # Error handling with context
tokio = { version = "1.38",      # Async runtime
  features = ["macros", "rt-multi-thread", "process"] }
serde = { version = "1",         # Serialization framework
  features = ["derive"] }
serde_json = "1"                 # JSON handling
schemars = "0.8"                 # JSON schema generation
deepseek-api = "0.1"             # DeepSeek API client
rmcp = { version = "0.1",        # MCP client library
  features = ["client", "transport-child-process"] }
dotenvy = "0.15"                 # Environment variable loading
owo-colors = "4.0"               # Terminal colors and styling
```

### Key Dependencies Explained

- **rmcp**: The Rust MCP (Model Context Protocol) client library for connecting to MCP servers
- **deepseek-api**: Official DeepSeek API client for Rust
- **tokio**: Provides async runtime with child process support for MCP server management
- **owo-colors**: Enables the rich, colorful console output with emoji support
- **dotenvy**: Handles `.env` file loading with environment variable precedence
- **anyhow**: Provides excellent error handling with context preservation

## Extending the Project

### Adding New MCP Servers
```rust
impl McpExecutor {
    pub async fn connect_custom_server(server_dir: &str) -> Result<Self> {
        Logger::network("Connecting to custom MCP server...");
        
        let mut cmd = tokio::process::Command::new("node");
        cmd.arg("dist/index.js")
           .current_dir(server_dir);
        let transport = TokioChildProcess::new(&mut cmd)
            .context("create TokioChildProcess")?;
        
        let handler = SimpleClientHandler::new();
        let service = handler.serve(transport).await
            .context("failed to start MCP client service")?;
        
        // Wait for server initialization
        tokio::time::sleep(Duration::from_secs(2)).await;
        
        Logger::success("Successfully connected to custom MCP server");
        Ok(Self { service })
    }
}
```

### Adding New Query Types
```rust
// In main.rs, modify the queries array:
let advanced_queries = [
    "Compare the geographic distance between these IPs",
    "Find all IPs in the same subnet as 192.168.1.0/24", 
    "Generate a security report for this IP range",
    "Get comprehensive analysis for multiple IPs: 8.8.8.8, 1.1.1.1, 9.9.9.9",
];

// The system will automatically:
// 1. Parse natural language queries
// 2. Determine appropriate IPLocate tools to call
// 3. Execute tools via MCP
// 4. Present results with rich logging
for (i, q) in advanced_queries.iter().enumerate() {
    Logger::query(format!("Query {}/{}: {}", i + 1, advanced_queries.len(), q));
    if let Some(ans) = run_once(&client, &cfg.model, q, &executor).await? {
        Logger::result(format!("Answer for: {}", q), &ans);
    }
}
```

### Custom Tool Definitions
```rust
// In tooling.rs, you can add additional MCP invoke tools:
pub fn custom_mcp_tool(server_name: &str) -> anyhow::Result<ToolObject> {
    let parameters: SchemaObject = serde_json::from_value(json!({
        "type": "object",
        "required": ["server", "tool", "arguments"],
        "properties": {
            "server": {
                "type": "string",
                "description": format!("MCP server alias (always '{}' here)", server_name)
            },
            "tool": {
                "type": "string", 
                "description": "Tool name on that MCP server"
            },
            "arguments": {
                "type": "object",
                "description": "Tool arguments JSON"
            }
        }
    }))?;
    Ok(ToolObject {
        tool_type: ToolType::Function,
        function: Function {
            name: "mcp_invoke".to_string(),
            description: format!("Invoke a tool on the {} MCP server", server_name),
            parameters,
        },
    })
}
```

## Performance Considerations

- **Async Operations**: All network calls are async for optimal performance using tokio
- **Connection Management**: 
  - MCP connections maintained for multiple queries
  - Proper child process lifecycle management
  - Graceful server initialization with configurable delays
- **Timeout Protection**: 
  - 5-second timeout for tool listing
  - 60-second timeout for tool execution
  - Prevents indefinite hanging operations
- **Error Handling**: 
  - Comprehensive error handling with context preservation
  - Fallback mechanisms for server communication failures
  - Graceful degradation with informative logging
- **Resource Management**: 
  - Proper cleanup of child processes and connections
  - Memory-efficient JSON processing
  - Structured logging to avoid console spam

## Troubleshooting

### Common Issues

1. **MCP Server Connection Failed**
   ```
   ❌ Failed to start MCP client service
   ```
   - Ensure the IPLocate server is built: `npm run build` in the server directory
   - Verify the path in `IPLOCATE_DIR` is correct
   - Check that Node.js is installed and accessible: `node --version`
   - Look for `dist/index.js` file in the server directory

2. **DeepSeek API Errors**
   ```
   ❌ DeepSeek API request failed
   ```
   - Verify your API key is valid: check `DEEPSEEK_API_KEY`
   - Ensure sufficient credits in your DeepSeek account
   - Check network connectivity and API rate limits
   - Try switching models: `DEEPSEEK_MODEL=deepseek-chat` vs `deepseek-reasoner`

3. **Tool Call Failures**
   ```
   ⚠️ Server took too long to respond, using fallback tool list
   ```
   - This is normal behavior - the system uses fallback tools
   - If tools still fail, check MCP server logs in the terminal
   - Verify tool argument formats match expected parameters
   - Increase timeout values in `executor.rs` if needed

4. **Environment Configuration Issues**
   ```
   ❌ Missing required environment variable: DEEPSEEK_API_KEY
   ```
   - Create a `.env` file in the project root with required variables
   - Or export environment variables before running: `export DEEPSEEK_API_KEY=..`
   - Check that `.env` file is in the correct location (project root)

## Contributing

Contributions are welcome! Areas for improvement:

- Additional MCP server integrations
- Enhanced error handling and retry logic
- More sophisticated query processing
- Performance optimizations
- Additional IP analysis tools

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Related Projects

- [Model Context Protocol](https://modelcontextprotocol.io/)
- [DeepSeek API](https://platform.deepseek.com/)
- [MCP Server IPLocate](https://github.com/modelcontextprotocol/servers/tree/main/src/iplocate)

## Support

For issues and questions:
1. Check the troubleshooting section above
2. Review the MCP and DeepSeek documentation
3. Open an issue in this repository
