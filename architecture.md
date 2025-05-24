# VQL Architecture

## Philosophy
*"VQL: Enabling growth while preventing decay - exploring the path without losing the way"*

VQL operates at the architectural/design level, not the syntax level. It's not a straightjacket - it's a ratchet: allowing forward movement while preventing backward movement.

## Core Concepts

### Architectural Linting for AI
- Traditional linters: Mechanical rule checking (syntax)
- VQL: Conceptual principle checking (architecture)
  - "Does this code separate concerns properly?"
  - "Are security boundaries maintained?"
  - "Will this scale horizontally?"

### The ON/OFF Toggle
- **VQL OFF** → Exploration Mode: AI and humans experiment freely
- **VQL ON** → Consolidation Mode: Best patterns get enforced
- Mirrors natural development rhythm: explore, then standardize

### Optimal Use Case: Young Brownfield
The sweet spot: Projects that have:
- Recent code with emerging patterns
- Small enough to refactor systematically
- Clear patterns worth codifying
- Fresh memory of what works/doesn't

## MCP Server Integration

### Why MCP for VQL

1. **Natural Fit**: VQL already uses AI-friendly commands (`:vql`, `:um.rv`)
2. **Separation of Concerns**: 
   - Core VQL operations (storage, principles, reviews)
   - AI-assisted workflows (review, refactor)
   - CLI interface (user interaction)
3. **Better UX**: One approval for complex workflows vs many CLI prompts

### Architecture Benefits

1. **Stateful Context**: Maintain context across interactions
2. **Tool Mapping**: Commands naturally map to MCP tools
   - `list_principles`, `add_principle`
   - `list_assets`, `add_asset`
   - `store_review`, `set_compliance`
   - `review_all_assets` (50+ operations in one approval)
3. **Resource Exposure**: Browse principles, assets, reviews

### Implementation Strategy

#### Phase 1: Refactor for Modularity
Extract core business logic:
```
vql/
├── src/
│   ├── core/           # Shared business logic
│   │   ├── storage.rs
│   │   ├── principles.rs
│   │   ├── assets.rs
│   │   └── reviews.rs
│   ├── cli/            # CLI-specific code
│   ├── mcp/            # MCP server implementation
│   └── lib.rs          # Shared library
```

#### Phase 2: Dual Binary Approach (Recommended)
```toml
[[bin]]
name = "vql"
path = "src/main.rs"

[[bin]]
name = "vql-mcp"
path = "src/mcp_server.rs"
```

Benefits:
- Independent processes
- Share codebase at compile time
- Access same storage
- Can run simultaneously

#### Phase 3: Command Translation
User types: `:vql st um a "Review..."`
→ Claude recognizes VQL command
→ Calls MCP tool: `store_review(asset="um", principle="a", review="Review...")`
→ Structured response

### Key Advantages

1. **Reduced Friction**: One approval for entire workflows
2. **Semantic Understanding**: Claude sees intent, not strings
3. **Systematic Approaches**: Reviews against principles, not random changes
4. **State Maintenance**: Track improvements over time

## Value Proposition

### For Brownfield Projects (Optimal)
- Identify best existing patterns
- Apply systematically across codebase
- Measure improvement (L→M→H ratings)
- Prevent style drift in new code

### For Greenfield Projects
- Let patterns emerge first (1 month)
- Then codify and enforce
- Avoid premature optimization
- Enable creativity before consistency

### The Bottom Line
VQL makes AI's code quality **predictable and controllable**. It's architectural guidance, not syntactic enforcement.

## Technical Considerations

### MCP Implementation Decision

#### Initial Rust Approach (Attempted)
- Attempted dual-binary approach with Rust MCP implementation
- Encountered challenges:
  - Immature Rust MCP ecosystem (limited/unstable crates)
  - Complex type mismatches with available crates (rust-mcp-schema, mcpr)
  - Would require significant boilerplate for basic JSON-RPC handling
  - Risk of maintenance burden with evolving MCP spec

#### TypeScript Implementation (Chosen)
- TypeScript offers mature MCP SDK with official support
- Benefits:
  - Well-documented, stable MCP implementation
  - Faster iteration and testing
  - Better aligned with MCP ecosystem (most examples in TS)
  - Can still use VQL Rust CLI via subprocess calls
- Implementation approach:
  - TypeScript MCP server in same repo under `mcp-server/` directory
  - **CLI is a hard dependency** - MCP server cannot function without it
  - MCP is purely a protocol adapter - zero business logic
  - All operations delegate to CLI commands via subprocess
  - Example: `store_review()` → `exec('vql -st ...')`
- Synchronization guaranteed:
  - Single source of truth (CLI)
  - No logic duplication possible
  - Bug fixes/features automatically available to both
  - Version mismatch impossible in same-repo setup
- Trade-offs:
  - Pro: Perfect synchronization - one implementation
  - Pro: MCP benefits from all CLI improvements instantly
  - Con: MCP server requires VQL CLI to be installed
  - Con: Performance overhead from subprocess calls

#### Performance Analysis
- **Subprocess overhead**: ~0.05 seconds per CLI invocation
- **In perspective**:
  - Human typing speed: ~5-10 seconds between commands
  - AI thinking time: ~1-3 seconds per decision
  - Network latency to Claude: ~0.1-0.5 seconds
  - **0.05s overhead is ~1% of typical interaction time**
- **Real-world examples**:
  - `list_principles()`: 0.05s (perfectly fine)
  - `store_review()`: 0.05s (unnoticeable)
  - Reviewing 10 assets: 0.5s total (still fast)
  - Reviewing 100 assets: 5s (might want batching)
- **Optimization if needed**:
  - Batch operations: `vql --batch '[{"cmd":"st","args":...},...]'`
  - Long-running mode: `vql --server` (keeps process warm)
- **Bottom line**: For VQL's use case (design-time quality checks), 
  0.05s is negligible compared to human/AI thinking time

#### Usage Scenarios on Same Machine
- **Human in terminal**: `vql -st um a "This module needs refactoring"`
- **AI via CLI**: Claude can run same CLI commands in terminal
- **AI via MCP**: Claude uses MCP tools which call the same CLI internally
- **Both access same data**: Same VQL/vql_storage.json file
- **Real-time collaboration**: Human and AI can work on same project simultaneously
- Example workflow:
  1. Human: `vql -ar um user model.py` (registers asset)
  2. AI via MCP: `store_review(asset="um", principle="a", review="...")` 
  3. Human: `vql ? um` (sees AI's review immediately)
  4. Both are using the exact same VQL implementation

### Storage
- Shared JSON storage between CLI and MCP
- File locking for concurrent access
- Same data model for both interfaces

### Future Enhancements
- File watching for real-time feedback
- Batch operations for large refactors
- Integration with CI/CD pipelines
- Team collaboration features

## Getting Started

1. Current state: CLI tool with JSON storage
2. Next step: Create TypeScript MCP server that calls VQL CLI
3. Repository structure:
   ```
   vql/
   ├── src/              # Rust CLI source
   ├── mcp-server/       # TypeScript MCP server
   │   ├── src/
   │   ├── package.json
   │   └── README.md
   ├── Cargo.toml
   └── README.md
   ```
4. Distribution options:
   - **Option A**: Separate packages
     - Rust CLI: `cargo install vibe-ql`
     - MCP Server: `npm install -g @vibe-ql/mcp-server`
   - **Option B**: MCP installer that includes both
     - `npm install -g @vibe-ql/mcp-server` also installs CLI binary
5. Configuration: Users configure Claude Desktop to use `vql-mcp` command

The goal: Transform VQL from a CLI tool into a platform for AI-assisted code quality management.