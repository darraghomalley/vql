# VQL - Vibe Query Language and Framework

VQL (Vibe Query Language) is an open source language framework that provides developers with guardrails to ensure software quality during AI-assisted coding sessions. It enables concise commands for code review and refactoring based on user-defined principles, maintaining quality standards while preserving the speed of "vibe coding."

## Key Concepts

VQL manages a lightweight knowledge base that persists across AI sessions:

- **Principles**: Quality criteria for evaluating code (e.g., architecture, security, performance)
- **Entity References**: Business entities in your codebase (e.g., User, Product, Order)
- **Asset Types**: Categories of code files (e.g., Controller, Model, Service)
- **Asset References**: Specific files tracked with their entity/type relationships
- **Asset Reviews**: Principle-based evaluations with compliance ratings (High/Medium/Low)

## Architecture

VQL uses a dual-interface architecture for maximum flexibility:

### 1. Rust CLI (Core Engine)
- High-performance command-line tool written in Rust
- Manages all VQL operations and data storage
- Creates a `VQL/` directory in your project with JSON-based storage
- Provides direct terminal access to all VQL features

### 2. MCP Server (AI Integration)
- TypeScript-based Model Context Protocol server
- Wraps CLI commands for structured AI assistant access
- Provides type-safe tools for Claude and other MCP-compatible assistants
- Enables seamless VQL integration in AI coding sessions

The MCP server acts as a thin wrapper around the CLI, ensuring both interfaces stay in sync while providing the best experience for each use case.

## Features

- **Principle Management**: Define custom quality criteria or load from markdown files
- **Asset Tracking**: Link code files to business entities and types
- **Systematic Reviews**: AI-assisted evaluation against all principles
- **Persistent Storage**: Reviews and ratings survive context resets
- **Exemplar Marking**: Identify best-practice implementations
- **Compliance Ratings**: Track improvement with High/Medium/Low ratings
- **Guided Refactoring**: AI workflows that improve code and update reviews
- **Dual Interface**: Use via CLI or MCP-enabled AI assistants

## Installation

### VQL CLI

```bash
# Clone and build from source
git clone https://github.com/darraghenright/vql.git
cd vql
cargo build --release

# Add to your PATH
cp target/release/vql /usr/local/bin/
# Or use cargo install from the project directory
cargo install --path .
```

### MCP Server (for AI Assistants)

The MCP server enables Claude and other AI assistants to use VQL through structured tools.

```bash
# From the VQL project directory
cd mcp-server
npm install
npm run build

# For global access
npm link
```

#### Configure Claude Desktop

Add VQL to your Claude Desktop MCP servers:

**macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`  
**Windows**: `%APPDATA%\Claude\claude_desktop_config.json`

For a global installation:
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

For a project-specific installation:
```json
{
  "mcpServers": {
    "vql": {
      "command": "node",
      "args": ["../vql/mcp-server/dist/index.js"]
    }
  }
}
```

**Note**: Restart Claude Desktop after adding the MCP server configuration for it to take effect.

## Quick Start

```bash
# Initialize VQL in your project
vql -su "."

# Add a principle
vql -pr -add a Architecture "Clean architecture and separation of concerns"

# Add an entity and asset type
vql -er -add u User
vql -at -add c Controller

# Track an asset
vql -ar -add uc u c "src/UserController.js"

# Store a review (rating auto-extracted from text)
vql -st uc a "The UserController demonstrates HIGH compliance with architecture principles..."

# Query reviews
vql uc?         # All reviews for UserController
vql uc?(a)      # Architecture review for UserController
```

## CLI Command Reference

### Setup and Configuration
```bash
vql -su "path/to/project"    # Initialize VQL in a directory
```

### Principle Management
```bash
vql -pr                      # List all principles
vql -pr -add a Architecture "Description"
vql -pr -get "principles.md" # Load principles from markdown
```

### Entity and Asset Type Management
```bash
vql -er                      # List entities
vql -er -add u User         # Add entity

vql -at                      # List asset types  
vql -at -add c Controller   # Add asset type
```

### Asset Reference Management
```bash
vql -ar                      # List all assets
vql -ar -add uc u c "path/to/UserController.js"
```

### Reviews and Ratings
```bash
vql -st uc a "Review with HIGH compliance..."  # Store review
vql -se uc t                                   # Set as exemplar (t/f)
vql -sc uc a H                                 # Set compliance (H/M/L)
vql uc?                                        # Query all reviews
vql uc?(a,s)                                   # Query specific reviews
```

## AI Assistant Integration

VQL provides powerful integration with AI coding assistants through two methods:

### Method 1: MCP Tools (Recommended)

When the MCP server is configured, AI assistants use structured tools:
- `list_principles()` - Show all principles
- `add_principle(short, long, guidance)` - Add a principle
- `store_review(asset, principle, review)` - Store a review
- `review_asset_all_principles(asset)` - AI workflow to review
- `refactor_asset_principles(asset, principles)` - AI workflow to refactor

The MCP interface provides type safety, better error handling, and direct integration with Claude's tool system.

### Method 2: LLM Command Syntax

AI assistants can also use a special command syntax:

```
# Basic operations
:-pr                     # Show all principles
:-er.add(u, User)        # Add an entity
:-ar.add(uc, u, c, "path/to/file.js")  # Add asset reference
:uc.st(a, "Review...")   # Store a review
:uc.se(t)                # Set as exemplar
:uc?(a,s)                # Query specific reviews

# AI Workflows (multi-step operations)
:uc.rv(*)                # Review asset against all principles
:uc.rv(a,s)              # Review asset against specific principles
:uc.rf(*)                # Refactor asset for all principles
:uc.rf(a,s)              # Refactor asset for specific principles
:-rv(*)                  # Review all assets
:-rf(a,s)                # Refactor all assets for specific principles
```

### Important: Post-Refactoring Reviews

When using refactoring commands, the AI will:
1. Analyze the code against specified principles
2. Apply improvements to the code
3. **MANDATORY**: Review the refactored code and store updated reviews
4. Include "After refactoring:" prefix in review text
5. Update compliance ratings based on the improved state

Refactoring is NOT complete until reviews are updated.

## Principle Examples

VQL principles are flexible quality criteria you define for your codebase:

```markdown
# Architecture Principles (a)
- Clean architecture with clear separation of concerns
- Dependency injection and inversion of control
- DRY (Don't Repeat Yourself) implementation

# Security Principles (s)
- Input validation and sanitization
- Authentication and authorization patterns
- Protection against common vulnerabilities

# Performance Principles (p)
- Efficient algorithms and data structures
- Caching strategies and optimization
- Resource management and cleanup

# Scalability Principles (c)
- Horizontal scaling patterns
- Stateless design principles
- Resilient error handling
```

Load principles from a markdown file:
```bash
vql -pr -get "principles.md"
```

## Working with VQL Data

VQL stores all data in `VQL/vql_storage.json` in your project:

```json
{
  "principles": {
    "a": {
      "short_name": "a",
      "long_name": "Architecture",
      "guidance": "Clean architecture principles..."
    }
  },
  "asset_references": {
    "uc": {
      "short_name": "uc",
      "entity": "u",
      "asset_type": "c",
      "path": "src/UserController.js",
      "exemplar": false,
      "principle_reviews": {
        "a": {
          "rating": "H",
          "analysis": "High compliance with architecture..."
        }
      }
    }
  }
}
```

This JSON structure persists across AI sessions, maintaining your code quality history.

## Use Cases

VQL is designed for:

1. **AI-Assisted Development**: Maintain quality standards during rapid "vibe coding" sessions
2. **Code Reviews**: Systematic evaluation against consistent criteria
3. **Technical Debt Management**: Track and improve compliance over time
4. **Knowledge Persistence**: Maintain context across multiple AI sessions
5. **Team Alignment**: Shared quality standards across developers and AI assistants

## Roadmap

- [ ] Publish to crates.io as `vibe-ql`
- [ ] Publish MCP server to npm as `@vibe-ql/mcp-server`
- [ ] Additional principle templates for common frameworks
- [ ] Integration with popular development tools
- [ ] Metrics dashboard for tracking quality trends
- [ ] Team collaboration features

## Contributing

Contributions are welcome! Here's how you can help:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

Please ensure your code follows Rust best practices and includes tests.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built with Rust for performance and reliability
- MCP server integration for seamless AI assistant support
- Inspired by the need for quality guardrails in AI-assisted development
