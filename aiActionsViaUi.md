# AI Actions via UI - Command Queue Pattern

## Overview

This document outlines the implementation of AI-driven actions (review/refactor) triggered from the VS Code UI using a command queue pattern. This approach allows users to right-click on assets in the VS Code extension and queue AI workflows that Claude can process when requested.

## Problem Statement

- Users want to trigger AI review/refactor workflows directly from VS Code UI
- MCP architecture is one-way (AI → MCP Server, not vice versa)
- Need a bridge between user actions in VS Code and AI execution in Claude

## Solution: Command Queue Pattern

### Architecture

```
┌─────────────┐     ┌──────────────┐     ┌─────────────────┐     ┌──────────┐
│   VS Code   │────▶│ Command Queue│◀────│   VQL MCP      │◀────│  Claude  │
│  Extension  │     │  (.json file) │     │    Server      │     │    AI    │
└─────────────┘     └──────────────┘     └─────────────────┘     └──────────┘
     Write              Read/Write            Read/Clear              Call
```

### Components

#### 1. Command Queue File
- Location: `.vql/pending-commands.json` (gitignored)
- Format:
```json
{
  "version": "1.0.0",
  "commands": [
    {
      "id": "cmd_1234567890",
      "timestamp": "2025-01-29T10:30:00Z",
      "action": "review",
      "asset": "uc",
      "principles": ["A", "S", "P"],
      "context": {
        "file_path": "src/controllers/UserController.ts",
        "entity": "u",
        "asset_type": "c"
      },
      "status": "pending"
    },
    {
      "id": "cmd_1234567891",
      "timestamp": "2025-01-29T10:31:00Z",
      "action": "refactor",
      "asset": "um",
      "principles": ["A"],
      "reference_assets": ["uc", "pm"],
      "context": {
        "file_path": "src/models/UserModel.ts",
        "entity": "u",
        "asset_type": "m"
      },
      "status": "pending"
    }
  ]
}
```

#### 2. VS Code Extension Enhancements

##### Context Menu Actions
Add right-click menu items in:
- File Explorer (for tracked assets)
- Compliance Matrix cells
- Asset tree view

Menu items:
- "VQL: Review All Principles"
- "VQL: Review Specific Principles..."
- "VQL: Refactor All Principles"
- "VQL: Refactor Specific Principles..."
- "VQL: Refactor Using References..."

##### Command Queue Manager
New module: `src/commandQueueManager.ts`
```typescript
interface VQLCommand {
  id: string;
  timestamp: string;
  action: 'review' | 'refactor';
  asset: string;
  principles?: string[];
  reference_assets?: string[];
  context: {
    file_path: string;
    entity: string;
    asset_type: string;
  };
  status: 'pending' | 'processing' | 'completed' | 'failed';
}

class CommandQueueManager {
  queueCommand(command: VQLCommand): Promise<void>
  getQueuedCommands(): Promise<VQLCommand[]>
  clearQueue(): Promise<void>
  updateCommandStatus(id: string, status: string): Promise<void>
}
```

##### Visual Feedback
- Status bar item showing queue count: "VQL: 3 commands queued"
- Notification when commands are queued
- Different file decoration when asset has queued commands

#### 3. MCP Server Enhancements

New tools to add:

##### `get_pending_commands`
- Returns all pending commands from the queue
- No parameters needed
- Returns array of command objects

##### `update_command_status`
- Parameters: `command_id`, `status`
- Updates status to 'processing', 'completed', or 'failed'
- Used by Claude to track progress

##### `clear_command_queue`
- Removes completed/failed commands
- Optional parameter to force clear all

#### 4. Claude AI Workflows

##### User Triggers Processing
User says: "Process pending VQL commands" or "Check for VQL tasks"

##### Claude Workflow
```python
# Pseudocode for Claude's execution
commands = vql.get_pending_commands()
for command in commands:
    vql.update_command_status(command.id, "processing")
    
    if command.action == "review":
        vql.review_asset_principles(command.asset, command.principles)
    elif command.action == "refactor":
        vql.refactor_asset_principles_with_references(
            command.asset, 
            command.principles,
            command.reference_assets
        )
    
    vql.update_command_status(command.id, "completed")

vql.clear_command_queue()
```

## Implementation Plan

### Phase 1: Core Infrastructure
1. Create command queue file structure
2. Implement CommandQueueManager in VS Code extension
3. Add basic write operations from context menus

### Phase 2: MCP Server Integration
1. Add queue management tools to MCP server
2. Implement file locking for concurrent access
3. Add status tracking capabilities

### Phase 3: UI Enhancements
1. Add context menu items
2. Implement status bar indicator
3. Add visual feedback for queued items

### Phase 4: Advanced Features
1. Principle selection dialog
2. Reference asset picker
3. Queue management view
4. Batch operations

## User Experience Flow

### Basic Review Flow
1. User right-clicks on `UserController.ts` in VS Code
2. Selects "VQL: Review All Principles"
3. VS Code shows notification: "Review queued for asset 'uc'"
4. Status bar shows: "VQL: 1 command queued"
5. User switches to Claude
6. Types: "Process my VQL commands"
7. Claude executes review and stores results
8. VS Code automatically updates decorations

### Advanced Refactor Flow
1. User right-clicks on compliance matrix cell (asset: um, principle: A)
2. Selects "VQL: Refactor Using References..."
3. Dialog appears to select reference assets
4. User selects 'uc' and 'pm' as references
5. Command queued with full context
6. User can queue multiple commands before processing
7. Single Claude command processes entire queue

## Benefits

1. **Seamless Integration**: Natural VS Code workflow
2. **Batch Processing**: Queue multiple commands, process once
3. **Context Preservation**: Full context passed to AI
4. **User Control**: Explicit trigger for AI processing
5. **Async Workflow**: Queue now, process later
6. **Audit Trail**: Command history in JSON file

## Technical Considerations

### File Locking
- Use file system locks to prevent corruption
- VS Code and MCP server need coordinated access

### Error Handling
- What if Claude fails mid-queue?
- Recovery mechanisms for partial completion
- Timeout handling for stale commands

### Performance
- Queue file size limits
- Efficient JSON parsing/writing
- File watcher for real-time updates

### Security
- Validate command structure
- Prevent command injection
- Sanitize file paths

## Alternative Approaches Considered

### Direct API Integration
- Pros: Immediate execution, no queue needed
- Cons: Requires API keys in VS Code, bypasses Claude Desktop

### URL Protocol Handler
- Pros: Direct Claude Desktop integration
- Cons: Complex setup, platform-specific, limited data passing

### WebSocket Server
- Pros: Real-time bidirectional communication
- Cons: Overly complex, requires always-running server

## Future Enhancements

1. **Command Templates**: Save common review/refactor patterns
2. **Scheduled Processing**: Auto-process queue at intervals
3. **Command History**: View past executions and results
4. **Bulk Operations**: Select multiple files for batch processing
5. **Custom Workflows**: User-defined AI action sequences

## Conclusion

The Command Queue Pattern provides a pragmatic solution that:
- Works within MCP's architectural constraints
- Provides excellent user experience
- Maintains clean separation of concerns
- Scales well for future enhancements

This approach balances technical feasibility with user needs, creating a powerful bridge between VS Code's UI and Claude's AI capabilities.