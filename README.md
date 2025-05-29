# VQL - Vibe Query Language and Framework

VQL (Vibe Query Language) is an open source language framework that provides developers with guardrails to ensure software quality during AI-assisted coding sessions. It enables concise commands for code review and refactoring based on user-defined principles, maintaining quality standards while preserving the speed of "vibe coding."

## Key Concepts

VQL manages a lightweight knowledge base that persists across AI sessions:

- **Principles**: Quality criteria for evaluating code (e.g., architecture, security, performance)
- **Entity References**: Business entities in your codebase (e.g., User, Product, Order)
- **Asset Types**: Categories of code files (e.g., Controller, Model, Service)
- **Asset References**: Specific files tracked with their entity/type relationships
- **Asset Reviews**: Principle-based evaluations with compliance ratings (High/Medium/Low)

## Recent Updates

### Command Syntax Evolution
The latest version replaces the ambiguous `*` wildcard with the clearer `-pr` token:
- **Old**: `:uc.rv(*)` - unclear if `*` means all principles or all assets
- **New**: `:uc.rv(-pr)` - clearly means "all principles" following VQL's hyphen convention

### Reference-Based Refactoring
Refactor commands now support using other assets as references:
- `:uc.rf(-pr, pc)` - refactor uc using all principles with pc as an example
- `:uc.rf(a,s,pc,tm)` - refactor uc for principles a,s using patterns from pc and tm

This enables pattern-based improvements where exemplar implementations guide refactoring.

## Design Philosophy

### The Hyphen Convention

VQL uses a hyphen prefix (`-`) for all system commands to create distinct namespaces:
- **System commands**: `-pr`, `-ar`, `-su`, `-st` etc. (reserved by VQL)
- **User identifiers**: `a`, `uc`, `um` etc. (freely chosen by users)

This design allows users to name their principles, assets, and entities anything without conflicting with VQL commands. For example, you could have a principle named "add" and safely use `vql add?` to query it, while `vql -pr -add` remains the command to add principles.

### Unified Namespace

All user-defined short names (principles, entities, asset types, and assets) share a single namespace and must be unique. This ensures:
- No ambiguity when referencing items in commands or reviews
- Clear mental model - each short name identifies exactly one thing
- Clean syntax like `:uc.rf(um, a)` where types are inferred from context

### Command Syntax Patterns

VQL supports two distinct command interfaces with different design patterns:

#### CLI Syntax (Procedural)
Uses procedural syntax where commands take parameters:
```bash
vql -st uc "Review Content"  # store(asset, content)
vql -se uc true              # setExemplar(asset, status)
```

#### LLM Syntax (Object-Oriented)
Uses object-oriented syntax where assets are objects with methods:
```bash
:uc.st(a, "Review Content")  # asset.store(principle, content)
:uc.se(true)                 # asset.setExemplar(status)
```

Both interfaces have the same parameter count but organize them differently - CLI uses procedural patterns while LLM uses asset-centric method calls.

## Canonical Commands Reference

The `canonicalCmds.json` file serves as the **authoritative specification** for all VQL commands. This file is critical because:

- **Single Source of Truth**: Defines exact syntax for both CLI and LLM interfaces
- **Implementation Guide**: Used to ensure CLI and MCP server stay synchronized
- **Documentation**: Provides examples and placeholder patterns for all operations
- **Quality Assurance**: Helps identify inconsistencies and missing features

The canonical format includes:
- **ACTION**: Human-readable description of what the command does
- **CLI**: Command-line syntax with examples
- **LLMP**: LLM placeholder syntax with parameter descriptions
- **LLME**: LLM example syntax with concrete values

Any changes to VQL commands should first be reflected in the canonical file to maintain consistency across all interfaces.

## Architecture

VQL uses a multi-interface architecture for maximum flexibility:

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
- Supports new refactoring tools with reference assets for pattern-based improvements

### 3. VS Code Extension (Visual Integration)
- Real-time visual compliance indicators in the file explorer
- Color-coded badges showing principle ratings at a glance
- Interactive compliance matrix for comprehensive overview
- Automatic updates when VQL storage changes
- Seamless integration with the VQL CLI

The MCP server and VS Code extension act as complementary interfaces to the CLI, ensuring all tools stay in sync while providing the best experience for each use case.

## Features

- **Principle Management**: Define custom quality criteria or load from markdown files
- **Asset Tracking**: Link code files to business entities and types
- **Systematic Reviews**: AI-assisted evaluation against all principles
- **Persistent Storage**: Reviews and ratings survive context resets
- **Exemplar Marking**: Identify best-practice implementations
- **Compliance Ratings**: Track improvement with High/Medium/Low ratings
- **Guided Refactoring**: AI workflows that improve code and update reviews
- **Reference-Based Refactoring**: Use exemplar assets as patterns for improvements
- **Flexible Principle Selection**: Use `-pr` for all principles or specify individual ones
- **Visual Indicators**: VS Code extension shows compliance badges in file explorer
- **Compliance Matrix**: Interactive matrix view for comprehensive quality overview
- **Multi-Interface**: Use via CLI, VS Code extension, or MCP-enabled AI assistants

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

### VS Code Extension

The VQL VS Code extension provides visual compliance indicators directly in your editor:

```bash
# From the VQL project directory
cd vscode-extension
npm install
npm run compile

# Package the extension
vsce package

# Install in VS Code
code --install-extension vql-vscode-*.vsix
```

Or install directly from the VS Code marketplace (coming soon).

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

#### New in Latest Release:
- **`-pr` token**: Use `-pr` instead of `*` to mean "all principles" in review/refactor commands
- **Reference-based refactoring**: Specify exemplar assets to guide refactoring patterns
- **Enhanced MCP tools**: New tools for refactoring with multiple reference assets

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
:-rv(-pr)                # Review all assets against all principles
:-rv(a,s)                # Review all assets against specific principles
:uc.rv(-pr)              # Review asset against all principles
:uc.rv(a,s)              # Review asset against specific principles
:-rf(-pr)                # Refactor all assets for all principles
:-rf(a,s)                # Refactor all assets for specific principles
:uc.rf(-pr)              # Refactor asset for all principles
:uc.rf(a,s)              # Refactor asset for specific principles
:uc.rf(-pr, pc)          # Refactor asset using all principles with pc as reference
:uc.rf(a,s,pc,tm)        # Refactor asset for principles a,s with pc,tm as references
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
6. **Visual Quality Tracking**: See compliance status at a glance in VS Code
7. **Quality Dashboards**: Use the compliance matrix to identify improvement areas

## Roadmap

- [ ] Publish to crates.io as `vibe-ql`
- [ ] Publish MCP server to npm as `@vibe-ql/mcp-server`
- [ ] Publish VS Code extension to marketplace
- [ ] Additional principle templates for common frameworks
- [ ] Integration with popular development tools
- [x] VS Code extension with visual compliance indicators
- [x] Interactive compliance matrix view
- [ ] Git integration for tracking quality over time
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
