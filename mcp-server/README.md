# VQL MCP Server

This is the Model Context Protocol (MCP) server implementation for VQL (Vibe Query Language). It provides AI assistants with structured access to VQL's code quality management capabilities.

## Installation

### Prerequisites
- VQL CLI must be installed: `cargo install vibe-ql`
- Node.js 18+ and npm

### Install from npm (when published)
```bash
npm install -g @vibe-ql/mcp-server
```

### Install from source
```bash
cd mcp-server
npm install
npm run build
npm link
```

## Configuration

### Claude Desktop Configuration

Add to your Claude Desktop configuration file:

**macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
**Windows**: `%APPDATA%\Claude\claude_desktop_config.json`

```json
{
  "mcpServers": {
    "vql": {
      "command": "vql-mcp"
    }
  }
}
```

If installed from source with npm link:
```json
{
  "mcpServers": {
    "vql": {
      "command": "node",
      "args": ["/path/to/vql/mcp-server/dist/index.js"]
    }
  }
}
```

## Usage

Once configured, Claude will have access to VQL tools:

### Basic Commands
- `list_principles()` - Show all VQL principles
- `list_assets()` - Show all registered assets
- `add_asset()` - Register a new asset
- `store_review()` - Store a review for an asset

### AI Workflow Commands
- `review_asset_all_principles()` - Review an asset against all principles
- `refactor_asset_principles()` - Refactor an asset based on specific principles

### Example Workflow

1. **Setup VQL in your project**:
   ```
   Use setup_vql("/path/to/project")
   ```

2. **Load principles**:
   ```
   Use load_principles_from_markdown("/path/to/principles.md")
   ```

3. **Register assets**:
   ```
   Use add_asset("uc", "u", "c", "src/UserController.js")
   ```

4. **Review assets**:
   ```
   Use review_asset_all_principles("uc")
   ```

## Architecture

The MCP server is a thin protocol adapter that:
- Translates MCP tool calls to VQL CLI commands
- Executes CLI commands via subprocess
- Returns structured responses to the AI

All business logic remains in the VQL CLI, ensuring perfect synchronization between human and AI usage.

## Development

```bash
# Install dependencies
npm install

# Run in development mode
npm run dev

# Build for production
npm run build

# Start production server
npm start
```

## Troubleshooting

### VQL CLI not found
Ensure VQL is installed and in your PATH:
```bash
which vql
# or
cargo install vibe-ql
```

### Permission errors
The MCP server needs permission to:
- Execute the VQL CLI binary
- Read/write to the VQL storage directory

### Storage not found
Run `vql -su .` in your project directory first to initialize VQL.