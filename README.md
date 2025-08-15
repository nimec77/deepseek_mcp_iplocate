# DeepSeek MCP IPLocate

A Rust-based AI-powered IP analysis tool that combines the DeepSeek language model with IPLocate geolocation services through the Model Context Protocol (MCP).

## Overview

This project demonstrates how to build an intelligent IP analysis system that can understand natural language queries and automatically invoke the appropriate IP geolocation tools. It uses:

- **DeepSeek AI Model** - For understanding natural language queries about IP addresses
- **Model Context Protocol (MCP)** - For seamless integration between AI and external tools
- **IPLocate MCP Server** - Specialized tools for IP geolocation, VPN detection, and network analysis
- **Async Rust** - High-performance concurrent execution with tokio

## Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────────┐
│   User Query    │───▶│   DeepSeek AI    │───▶│   IPLocate Tools    │
│ "Check if this  │    │     Model        │    │  • get_ip_details   │
│  IP is from a   │    │                  │    │  • check_vpn_proxy  │
│  VPN service"   │    │  (via MCP)       │    │  • get_location     │
└─────────────────┘    └──────────────────┘    └─────────────────────┘
```

The AI model processes natural language queries about IP addresses and automatically selects and calls the appropriate IPLocate tools through the MCP protocol to provide comprehensive answers.

## Features

- **Natural Language Processing**: Ask questions about IP addresses in plain English
- **Comprehensive IP Analysis**: 
  - Detailed geolocation information
  - VPN/Proxy/Tor detection
  - ISP and organization details
  - Network security analysis
- **Async Performance**: Built with tokio for high-performance concurrent operations
- **MCP Integration**: Seamless tool integration using the Model Context Protocol
- **Configurable**: Environment-based configuration for different models and setups

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

3. Set up environment variables:
```bash
export DEEPSEEK_API_KEY="your-deepseek-api-key"
export DEEPSEEK_MODEL="deepseek-chat"  # or "deepseek-reasoner"
export IPLOCATE_DIR="/path/to/servers/src/iplocate"
```

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
- Geographic location (country, city, coordinates)
- ISP and organization information
- Time zone data
- Network details

### Example 2: Security Analysis
```rust
"Check if 1.1.1.1 is VPN, proxy, or Tor"
```

**Expected Output:**
- VPN detection results
- Proxy service analysis
- Tor network detection
- Security risk assessment

### Example 3: Custom Queries (extend the code)
```rust
let custom_queries = vec![
    "What country is IP 203.0.113.0 located in?",
    "Is 192.168.1.1 a private IP address?",
    "Give me the ISP information for 1.2.3.4",
    "Analyze the security profile of multiple IPs: 8.8.8.8, 1.1.1.1",
];
```

## Configuration

The application uses environment variables for configuration:

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
├── main.rs           # Entry point and example usage
├── config.rs         # Environment configuration
├── deepseek_client.rs # DeepSeek API client setup  
├── executor.rs       # MCP client and tool execution
├── run.rs           # Main conversation logic
└── tooling.rs       # Tool schema definitions
```

### Key Components

- **McpExecutor**: Manages the connection to the IPLocate MCP server
- **DeepSeekClient**: Handles API communication with DeepSeek models
- **Tool Integration**: Automatically converts AI tool calls to MCP server invocations

## Extending the Project

### Adding New MCP Servers
```rust
impl McpExecutor {
    pub async fn connect_custom_server(server_dir: &str) -> Result<Self> {
        let mut cmd = tokio::process::Command::new("node");
        cmd.arg("build/index.js")
           .current_dir(server_dir);
        // ... connection logic
    }
}
```

### Adding New Query Types
```rust
let advanced_queries = vec![
    "Compare the geographic distance between these IPs",
    "Find all IPs in the same subnet as 192.168.1.0/24",
    "Generate a security report for this IP range",
];
```

### Custom Tool Definitions
```rust
pub fn custom_analysis_tool() -> anyhow::Result<ToolObject> {
    // Define new tool schemas for specialized analysis
}
```

## Performance Considerations

- **Async Operations**: All network calls are async for optimal performance
- **Connection Pooling**: MCP connections are maintained for multiple queries
- **Error Handling**: Comprehensive error handling with context preservation
- **Resource Management**: Proper cleanup of child processes and connections

## Troubleshooting

### Common Issues

1. **MCP Server Connection Failed**
   - Ensure the IPLocate server is built and the path is correct
   - Check that Node.js is installed and accessible

2. **DeepSeek API Errors**
   - Verify your API key is valid and has sufficient credits
   - Check network connectivity and API rate limits

3. **Tool Call Failures**
   - Ensure the MCP server is running and responsive
   - Check tool argument formats and required parameters

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
