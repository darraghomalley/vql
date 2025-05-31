# AI Actions via UI - Architectural Overview

## Overview

This document provides the architectural overview for AI-driven actions (review/refactor) triggered from the VS Code UI. The system supports multiple modes to accommodate different user workflows and integrates the VS Code extension, MCP server, and VQL CLI.

## System Architecture

The AI Actions system consists of three main components working together:

1. **VS Code Extension** - User interface and action triggers
2. **VQL MCP Server** - Bridge for AI operations and command processing
3. **VQL CLI** - Core functionality for storage and operations

## Implementation Strategy

We're implementing a multi-mode approach to provide flexibility while building towards full automation:

### Four Modes (Implementation Order)

1. **VQL Virtual Syntax Clipboard** - Copy concise VQL commands
2. **Natural Language Clipboard** - Copy human-readable prompts
3. **Queue for VQL MCP** - Batch processing via `:pq` command
4. **Headless MCP** - Fully automated execution

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           VQL AI Actions System                          │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  ┌─────────────┐     ┌─────────────────┐     ┌───────────────────┐   │
│  │   VS Code   │     │   VQL MCP       │     │    VQL CLI        │   │
│  │  Extension  │────▶│    Server       │────▶│  (Rust Binary)    │   │
│  └─────────────┘     └─────────────────┘     └───────────────────┘   │
│         │                     │                         │              │
│         │                     │                         │              │
│         ▼                     ▼                         ▼              │
│  ┌─────────────┐     ┌─────────────────┐     ┌───────────────────┐   │
│  │   UI/UX     │     │  :pq Command    │     │  VQL Storage      │   │
│  │  - Menus    │     │  Processing     │     │  (JSON Files)     │   │
│  │  - Feedback │     │                 │     │                   │   │
│  └─────────────┘     └─────────────────┘     └───────────────────┘   │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

## Component Responsibilities

### VS Code Extension (`/vscode-extension`)
**Owner of:** User interface, mode selection, command generation

**Key Features:**
- Context menu integration in file explorer and compliance matrix
- Mode selection and user preferences
- Clipboard operations for Modes 1 & 2
- Queue file management for Mode 3
- MCP client integration for Mode 4 (future)

**Implementation Details:** See `vscode-extension/aiActionsViaUi.md`

### VQL MCP Server (`/mcp-server`)
**Owner of:** AI command processing, queue consumption

**Key Features:**
- Existing VQL tools (list, add, review, refactor)
- New `:pq` (process queue) command for Mode 3
- Bridge between Claude sessions and VQL CLI
- Batch processing capabilities

**Required Enhancements:**
1. Add queue reading functionality
2. Implement `:pq` command handler
3. Add batch processing with progress reporting
4. Handle command status updates

### VQL CLI (`/src`)
**Owner of:** Core VQL operations, storage management

**Key Features:**
- All existing VQL functionality remains unchanged
- Provides foundation for all AI operations
- Manages VQL storage format and consistency

**No changes required** - CLI remains the stable foundation

## Integration Points

### Mode 1 & 2: Clipboard Integration
**Data Flow:**
1. VS Code Extension generates command/prompt
2. Copies to system clipboard
3. User pastes into Claude session
4. Claude executes (with or without VQL mode)

**Key Considerations:**
- Platform-specific clipboard handling
- Character escaping and formatting
- Clear user feedback

### Mode 3: Queue Integration
**Data Flow:**
1. VS Code Extension writes to `.vql/pending-commands.json`
2. User invokes `:pq` in Claude session
3. VQL MCP Server reads queue file
4. Processes commands sequentially
5. Updates command status
6. VS Code Extension monitors changes

**Queue File Format:**
```json
{
  "version": "1.0.0",
  "commands": [{
    "id": "cmd_timestamp",
    "action": "review",
    "asset": "uc",
    "principles": ["A", "S", "P"],
    "path": "src/controllers/UserController.ts",
    "status": "pending"
  }]
}
```

### Mode 4: Direct MCP Integration (Future)
**Data Flow:**
1. VS Code Extension starts/connects to `claude mcp serve`
2. Sends prompts via MCP protocol
3. Receives results directly
4. Updates UI in real-time

**Infrastructure Needs:**
- MCP client library in VS Code extension
- Server lifecycle management
- Connection pooling and error handling

## Implementation Roadmap

### Phase 1: Foundation (Weeks 1-2)
**VS Code Extension:**
- [ ] Add mode selection to settings
- [ ] Implement VQL command generator (Mode 1)
- [ ] Implement natural language generator (Mode 2)
- [ ] Add context menu with mode submenu
- [ ] Integrate clipboard API
- [ ] Add success notifications

**Testing:**
- Manual testing with various asset types
- Verify clipboard content across platforms

### Phase 2: Queue System (Weeks 3-4)
**VS Code Extension:**
- [ ] Implement queue file management
- [ ] Add queue status to status bar
- [ ] Build queue viewer panel

**MCP Server:**
- [ ] Add queue reading functionality
- [ ] Implement `:pq` command
- [ ] Add progress reporting
- [ ] Handle status updates

**Testing:**
- Queue persistence and recovery
- Batch processing scenarios
- Error handling

### Phase 3: Refactor Support (Week 5)
**All Components:**
- [ ] Extend modes for refactor operations
- [ ] Add reference asset selection UI
- [ ] Create refactor templates
- [ ] Test complex scenarios

### Phase 4: Headless Mode (Weeks 6-8)
**VS Code Extension:**
- [ ] Add MCP client dependency
- [ ] Implement connection management
- [ ] Build progress tracking
- [ ] Add error recovery

**Testing:**
- End-to-end automation
- Performance optimization
- Error scenarios

## Success Metrics

### User Experience
- Time from intent to action execution
- Number of clicks/steps required
- Error rate and recovery time
- User satisfaction scores

### Technical Metrics
- Command success rate
- Average processing time
- Queue throughput
- System resource usage

### Adoption Metrics
- Mode usage distribution
- Feature adoption rate
- Migration from manual to automated modes

## Design Principles

### 1. Progressive Enhancement
Start with simple, working solutions (clipboard) and build towards ideal automation.

### 2. User Choice
Different users have different needs - support multiple workflows without forcing change.

### 3. Fail Gracefully
Each mode should work independently. If Mode 4 fails, users can fall back to Mode 1.

### 4. Clear Feedback
Users should always know what's happening and what to do next.

### 5. Minimal Configuration
Modes 1 & 2 work out of the box. Advanced modes opt-in.

## Security Considerations

### Clipboard Security
- No sensitive data in prompts
- Clear clipboard after paste (optional)
- Sanitize file paths

### Queue Security
- Validate all commands before processing
- Prevent command injection
- Scope operations to workspace

### MCP Security (Mode 4)
- Run server with minimal permissions
- No network access required
- Validate all inputs

## FAQ

### Why not start with full automation (Mode 4)?
- Clipboard modes ship immediately with zero infrastructure
- Users can start benefiting while we build complex features
- Reduces risk and allows iterative improvement

### Why support multiple modes instead of just one?
- Different users have different workflows
- Some prefer control, others want automation
- Gradual adoption path from manual to automated

### How do modes work together?
- Each mode is independent
- Users can mix modes (e.g., clipboard for quick tasks, queue for batch)
- Settings allow default mode per action type

### What about the existing VQL MCP server?
- Mode 3 enhances it with `:pq` command
- Other modes work alongside existing functionality
- No breaking changes to current workflows

## References

- **Implementation Details**: `vscode-extension/aiActionsViaUi.md`
- **VS Code Extension**: `vscode-extension/README.md`
- **MCP Server**: `mcp-server/README.md`
- **VQL CLI**: `README.md`

## Conclusion

The multi-mode AI Actions system provides a practical path from manual clipboard operations to full automation. By starting simple and building incrementally, we can deliver value immediately while working towards the ideal seamless experience.

Each component (VS Code extension, MCP server, CLI) maintains clear responsibilities while working together to enable powerful AI-driven code quality workflows.