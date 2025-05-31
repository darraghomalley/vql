# AI Actions via UI - Multi-Mode Approach

## Overview

This document outlines the implementation of AI-driven actions (review/refactor) triggered from the VS Code UI using multiple modes to accommodate different user workflows. Starting with reviews as the simpler use case, we'll implement four modes in order of complexity.

## Problem Statement

- Users want to trigger AI review/refactor workflows directly from VS Code UI
- Different users have different preferences for automation vs control
- Need to support various Claude session types (desktop, CLI, web)

## Solution: Four-Mode Implementation

### Modes Overview (Implementation Order)

1. **VQL Virtual Syntax Clipboard** - Copy VQL commands to clipboard
2. **Natural Language Clipboard** - Copy human-readable prompts to clipboard  
3. **Queue for VQL MCP** - Queue commands for batch processing via `:pq`
4. **Headless MCP** - Fully automated execution via `claude mcp serve`

### Architecture

```
┌─────────────┐     ┌─────────────────┐     ┌──────────────┐     ┌──────────┐
│   VS Code   │────▶│  Mode Handler   │────▶│   Target     │────▶│   VQL    │
│  Extension  │     │  (1,2,3, or 4)  │     │  (Various)   │     │   CLI    │
└─────────────┘     └─────────────────┘     └──────────────┘     └──────────┘
                            │
                            ├─ Mode 1: → Clipboard (VQL syntax)
                            ├─ Mode 2: → Clipboard (Natural language)
                            ├─ Mode 3: → Queue file → VQL MCP → Claude
                            └─ Mode 4: → MCP Client → claude mcp serve
```

### Components

#### 1. Mode Selection Configuration
Users can set their preferred mode in VS Code settings:
```json
{
  "vql.aiActionMode": "clipboard-vql" | "clipboard-natural" | "queue" | "headless",
  "vql.defaultReviewMode": "clipboard-natural"  // Can differ from refactor mode
}
```

#### 2. VS Code Extension Enhancements

##### Context Menu Actions
Add right-click menu items with submenu for mode selection:

**File Explorer (for tracked assets):**
- "VQL: Review → [Mode]"
  - "Copy VQL Command"
  - "Copy Natural Language"
  - "Add to Queue"
  - "Execute Now (Headless)"
- "VQL: Review Specific Principles... → [Mode]"

**Compliance Matrix cells:**
- Same options but context-aware for specific asset/principle combinations

**Quick Pick Alternative:**
- Single menu item that opens mode selector
- Remembers last choice per action type

##### Action Handler Manager
New module: `src/aiActionHandler.ts`
```typescript
interface AIActionHandler {
  // Mode 1: Generate VQL virtual syntax
  generateVQLCommand(
    action: 'review' | 'refactor',
    assetId: string,
    principles?: string[],
    options?: RefactorOptions
  ): string;
  
  // Mode 2: Generate natural language prompt
  generateNaturalLanguage(
    action: 'review' | 'refactor',
    assetPath: string,
    assetId: string,
    principles?: string[],
    principleDetails?: Map<string, string>,
    options?: RefactorOptions
  ): string;
  
  // Mode 3: Add to command queue
  async queueCommand(
    action: 'review' | 'refactor',
    assetId: string,
    assetPath: string,
    principles?: string[],
    options?: RefactorOptions
  ): Promise<void>;
  
  // Mode 4: Execute via MCP (implemented last)
  async executeHeadless(
    action: 'review' | 'refactor',
    assetPath: string,
    assetId: string,
    principles?: string[],
    options?: RefactorOptions
  ): Promise<void>;
}
```

##### Visual Feedback
- **Mode 1 & 2**: "Copied to clipboard: [preview of command]"
- **Mode 3**: "Added to queue: Review 'uc' (3 commands pending)"
- **Mode 4**: Progress notification with real-time updates

#### 3. Mode Implementations

##### Mode 1: VQL Virtual Syntax (Clipboard)
Generates concise VQL commands that users can paste into any Claude session with VQL enabled.

**Example outputs:**
```
// Review all principles
:uc.r()

// Review specific principles
:uc.r(A,S,P)

// Refactor with references
:uc.rf(A,S).ref(um,pm)
```

**Benefits:**
- Extremely concise
- Works in any VQL-enabled Claude session
- Power users can modify before execution

##### Mode 2: Natural Language (Clipboard)
Generates human-readable prompts that work in any Claude session.

**Example output:**
```
Please review the UserController file (asset ID: uc) located at src/controllers/UserController.ts 
against the following VQL principles:
- A (Architecture): Ensure proper separation of concerns and layering
- S (Security): Check for vulnerabilities and proper validation
- P (Performance): Evaluate efficiency and optimization

After analysis, store the reviews with appropriate compliance ratings (H/M/L).
```

**Benefits:**
- Self-documenting
- Works without VQL mode
- Users can customize the prompt

##### Mode 3: Command Queue
Adds commands to `.vql/pending-commands.json` for batch processing.

**Queue format:**
```json
{
  "commands": [{
    "id": "cmd_1234567890",
    "timestamp": "2025-01-29T10:30:00Z",
    "action": "review",
    "asset": "uc",
    "principles": ["A", "S", "P"],
    "status": "pending"
  }]
}
```

**Processing:** User runs `:pq` in Claude session with VQL MCP enabled.

##### Mode 4: Headless MCP (Implemented Last)
Direct execution via `claude mcp serve` - fully automated with no user intervention.

## Implementation Plan

### Phase 1: Clipboard Modes (Mode 1 & 2)
1. Implement VQL command generator for common patterns
2. Create natural language prompt templates
3. Add clipboard integration with VS Code API
4. Create context menu items with mode selection
5. Test with review operations only

### Phase 2: Queue Mode (Mode 3)
1. Design command queue JSON structure
2. Implement queue file management (read/write/lock)
3. Add queue status indicators in VS Code
4. Implement `:pq` command in VQL MCP server
5. Test batch processing workflows

### Phase 3: Refactor Support
1. Extend all modes to support refactor operations
2. Add reference asset selection UI
3. Create refactor-specific prompt templates
4. Test complex refactor scenarios

### Phase 4: Headless Mode (Mode 4)
1. Add `@modelcontextprotocol/client` dependency
2. Implement MCP client connection management
3. Create server lifecycle management
4. Build progress tracking and error handling
5. Full integration testing

## User Experience Flow

### Mode 1: VQL Syntax Clipboard
1. User right-clicks on `UserController.ts`
2. Selects "VQL: Review → Copy VQL Command"
3. VS Code copies: `:uc.r()` to clipboard
4. Shows notification: "Copied: :uc.r()"
5. User pastes into Claude session with VQL enabled
6. Command executes immediately

### Mode 2: Natural Language Clipboard
1. User right-clicks on `UserController.ts`
2. Selects "VQL: Review → Copy Natural Language"
3. VS Code generates detailed prompt with file path and principles
4. Shows notification: "Review prompt copied to clipboard"
5. User pastes into any Claude session
6. Reviews are performed and stored

### Mode 3: Queue for Batch Processing
1. User right-clicks multiple files/cells
2. Selects "VQL: Review → Add to Queue" for each
3. Status bar shows: "VQL Queue: 5 pending"
4. User opens Claude with VQL MCP
5. Types `:pq` to process entire queue
6. All reviews execute in sequence

### Mode 4: Headless Execution (Future)
1. User right-clicks on `UserController.ts`
2. Selects "VQL: Review → Execute Now"
3. VS Code shows progress in real-time
4. Reviews complete without leaving VS Code
5. Decorations update automatically

## Benefits

1. **Progressive Automation**: Start simple (clipboard), evolve to full automation
2. **User Choice**: Different modes for different workflows and preferences
3. **Immediate Availability**: Clipboard modes work today, no setup required
4. **Batch Efficiency**: Queue mode enables reviewing entire codebases
5. **Future-Proof**: Headless mode ready when infrastructure matures
6. **Low Risk**: Each mode is independent, reducing implementation complexity

## Technical Considerations

### Mode-Specific Concerns

**Clipboard Modes (1 & 2):**
- Handle large prompts that exceed clipboard limits
- Escape special characters properly
- Consider platform differences (Windows/Mac/Linux)

**Queue Mode (3):**
- File locking for concurrent access
- Queue size limits and cleanup policies
- Recovery from partial processing

**Headless Mode (4):**
- MCP server lifecycle management
- Connection stability and retry logic
- Progress tracking across async operations

### Cross-Mode Considerations
- Consistent command generation across modes
- Settings migration as users adopt new modes
- Telemetry to understand mode usage patterns

## Alternative Approaches Considered

### Single Mode Only
- Pros: Simpler implementation, consistent UX
- Cons: Doesn't accommodate different user preferences

### Web-based Review UI
- Pros: Rich interactive interface
- Cons: Complex architecture, leaves VS Code context

### Direct API Integration
- Pros: Could use Anthropic API directly
- Cons: Requires API keys, additional cost, bypasses Claude Desktop

## Future Enhancements

1. **Smart Mode Selection**: Auto-select mode based on context
2. **Prompt Templates**: User-customizable review/refactor prompts
3. **History Tracking**: View previous reviews and changes
4. **Diff Preview**: Show proposed changes before applying (Mode 4)
5. **Multi-Asset Selection**: Review/refactor multiple files at once
6. **Principle Groups**: Quick selection of related principles
7. **Integration with Git**: Review only changed files

## Example Implementation

### Mode 1: VQL Command Generator
```typescript
function generateVQLCommand(action: string, assetId: string, principles?: string[]): string {
  if (action === 'review') {
    return principles?.length 
      ? `:${assetId}.r(${principles.join(',')})`
      : `:${assetId}.r()`;
  }
  // Add refactor cases...
}
```

### Mode 2: Natural Language Generator
```typescript
function generateNaturalPrompt(asset: AssetInfo, principles: PrincipleInfo[]): string {
  return `Please review the ${asset.type} file "${asset.name}" (asset ID: ${asset.id}) 
located at ${asset.path} against the following VQL principles:

${principles.map(p => `- ${p.short} (${p.long}): ${p.guidance}`).join('\n')}

After analysis, store the reviews using the VQL CLI with appropriate compliance ratings (H/M/L).`;
}
```

### Mode 3: Queue Management
```typescript
async function addToQueue(command: QueuedCommand): Promise<void> {
  const queuePath = path.join(vqlDir, 'pending-commands.json');
  const queue = await readQueue(queuePath);
  queue.commands.push({
    id: `cmd_${Date.now()}`,
    timestamp: new Date().toISOString(),
    ...command,
    status: 'pending'
  });
  await writeQueue(queuePath, queue);
}
```

## Conclusion

The multi-mode approach provides maximum flexibility by:
- Meeting users where they are (clipboard for immediate use)
- Growing with user needs (queue for power users, headless for teams)
- Reducing implementation risk (ship working features early)
- Maintaining user control while enabling automation

This creates a practical path from manual reviews to full automation, letting users choose their comfort level while we build toward the ideal seamless experience.