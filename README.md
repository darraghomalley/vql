# VQL - Vibe Query Language and Framework

VQL is a tool built in Rust for managing software quality through principle-based code reviews and refactoring. It provides a structured way to evaluate and improve code quality based on defined architectural, security, performance, and scalability principles.

## Architecture

VQL is built on a Rust-based CLI architecture with the following components:

- **Rust CLI Application**: A command-line interface for executing VQL commands directly
- **MCP Server** (TypeScript): Model Context Protocol server that enables AI assistants to use VQL through structured tools
- **VQL Folder Structure**: Creates a `VQL` directory in your project to store configuration and data
- **JSON Object Store**: Local JSON-based storage that maintains principles, entities, asset types, and reviews
- **LLM Guidance Integration**: Special commands and syntax for AI assistant integration

The system manages several key data types:
- **Principles**: Quality criteria for evaluating code (architecture, security, performance, scalability)
- **Entity References**: Business entities referenced in the codebase (User, Product, etc.)
- **Asset Types**: Categories of code assets (Controller, Model, View, etc.)
- **Asset References**: Specific files being evaluated, linked to entities and asset types
- **Asset Reviews**: Quality evaluations of specific assets against principles

This architecture enables both direct CLI usage and powerful AI-assisted workflows for code review and refactoring through the MCP server.

## Features

- Define and manage principles for code quality assessment
- Associate assets (code files) with entities and asset types
- Perform systematic reviews of code against defined principles
- Store and retrieve code quality reviews
- Mark assets as exemplars for specific principles
- Rate asset compliance with principles (High/Medium/Low)
- Execute principle-guided refactoring workflows

## Installation

### VQL CLI

```bash
# Install from crates.io (when published)
cargo install vibe-ql

# Or build from source
git clone https://github.com/your-username/vql.git
cd vql
cargo build --release
cp target/release/vql /usr/local/bin/
```

### MCP Server (for AI assistants)

```bash
# Install from npm (when published)
npm install -g @vibe-ql/mcp-server

# Or from source
cd mcp-server
npm install
npm run build
npm link
```

#### Configure Claude Desktop

Add to your Claude Desktop configuration:

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

## CLI Commands

VQL supports a CLI command format for direct terminal use:

```bash
# Show all principles
vql -pr

# Add an entity
vql -er -add u User

# Add an asset type
vql -at -add c Controller

# Add an asset reference
vql -ar -add uc u c "path/to/UserController.js"

# Store a review
vql -st uc a "Review content"

# Set an asset as exemplar
vql -se uc t

# Set compliance rating
vql -sc uc a H

# Query reviews
vql uc?         # All reviews for an asset
vql uc?(a,s)    # Specific reviews for an asset
```

## LLM Commands (for use with Claude Code or other AI assistants)

VQL also supports a special syntax for AI assistant use:

```
# Show all principles
:-pr

# Add an entity
:-er.add(u, User)

# Add an asset type
:-at.add(c, Controller)

# Add an asset reference
:-ar.add(uc, u, c, "path/to/UserController.js")

# Store a review
:uc.st(a, "Review content")

# Set an asset as exemplar
:uc.se(t)

# Set compliance rating
:uc.sc(a,H)

# Query reviews
:uc?         # All reviews for an asset
:uc?(a,s)    # Specific reviews for an asset
```

## Virtual Commands

AI assistants can also execute special "virtual" workflows that combine multiple VQL operations:

```
# Review workflows
:-rv(*)                  # Review all assets against all principles
:-rv(a,s)                # Review all assets against specific principles
:uc.rv(*)                # Review specific asset against all principles
:uc.rv(a,s)              # Review specific asset against specific principles

# Refactor workflows
:-rf(*)                  # Refactor all assets against all principles
:-rf(a,s)                # Refactor all assets against specific principles
:uc.rf(*)                # Refactor specific asset against all principles
:uc.rf(a,s)              # Refactor specific asset against specific principles
:uc.rf(pc)               # Refactor one asset using another as reference
```

## Project Structure

- `/src` - Source code for the VQL Rust CLI application
  - `/bin` - Binary executables and utilities
  - `/commands` - Command handlers for VQL operations 
  - `/models` - Data models for VQL (assets, principles, etc.)
  - `/utils` - Utility functions and helpers
  - `/tests` - Test suite for VQL functionality
- `/mcp-server` - TypeScript MCP server implementation
  - `/src` - MCP server source code
  - `/dist` - Compiled JavaScript output

## Principles

VQL is designed around the concept of principles that serve as evaluation criteria for code quality:

- Architecture Principles (a) - Clean architecture, DRY, and structural considerations
- Security Principles (s) - Backend and frontend security best practices
- Performance Principles (p) - Optimizations for all application tiers
- Scalability Principles (c) - Design patterns for horizontal scaling and resilience

## Contributing

Contributions are welcome! Here's how you can help:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

Please ensure your code follows the project's coding standards and includes appropriate tests.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
