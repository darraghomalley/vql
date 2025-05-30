# VQL Development Guide for Claude

This document contains important information for Claude when working on the VQL project.

## Testing the VS Code Extension

When working on or testing the VS Code extension:

1. **Open the extension directory in VS Code:**
   ```bash
   cd vscode-extension
   code .
   ```

2. **Launch the Extension Development Host:**
   - Press **F5** in VS Code
   - This starts a new VS Code instance with the extension loaded for testing

3. **Open the test workspace:**
   - In the Extension Development Host window: File → Open Folder
   - Select the `testVSCodeUI` folder
   - The extension activates automatically when it detects the VQL directory

4. **Verify the extension is working:**
   - Look for colored file decorations next to TypeScript files in the Explorer
   - Decorations show compliance ratings (green/amber/red/grey)
   - Hover over decorations to see principle details

## Important Commands

### Building and Installing
- Build: `cargo build --release`
- Install: `./install.sh` (updates both cargo and user bin locations)
- Check version: `vql --version`

### Linting and Type Checking
- Rust: `cargo check` and `cargo clippy`
- TypeScript (in vscode-extension): `npm run compile`

### Running Tests
- Rust: `cargo test`
- VS Code Extension: Use F5 to test in Extension Development Host

## Version Management
**Important**: Always use `./install.sh` after building to ensure all VQL binary locations are updated. The project may have binaries in:
- `/home/darragh/.cargo/bin/vql` (cargo install location)
- `/home/darragh/bin/vql` (user bin location)

The VS Code extension uses whichever `vql` is first in PATH, so keeping these synchronized is critical.

## Project Structure
- `/src` - Rust CLI source code
- `/vscode-extension` - VS Code extension TypeScript code
- `/testVSCodeUI` - Test workspace for VS Code extension
- `/mcp-server` - MCP server implementation
- `/VQL` - VQL storage directory (created by `vql -su`)

## Key Features Being Developed
- Workspace-relative paths for team collaboration
- Visual compliance indicators in VS Code
- Real-time updates when VQL storage changes
- Migration support for existing absolute paths (`vql -migrate-paths`)

## VS Code Extension Architecture
- **Read operations**: Direct JSON parsing for fast visualization
- **Write operations**: All modifications use VQL CLI commands to ensure consistency
- The Matrix view provides interactive compliance visualization with selection support
- The Metadata view properly uses CLI for editing principles, entities, asset types, and assets

### VS Code Extension Features

#### File Explorer Integration
- **Right-click context menu**: "VQL → Add Asset Ref." on any file
- Opens metadata panel with path pre-populated
- Checks if file is already tracked before allowing addition

#### Compliance Matrix View
- **Interactive cell selection**: 
  - Click any cell to view principle and asset review details
  - Ctrl+click (Cmd+click on Mac) to select/unselect cells
  - Selected cells show blue border overlay
- **Batch operations** (when cells selected):
  - Review button (placeholder for bulk review)
  - Refactor button (placeholder for bulk refactor) 
  - Clear button to deselect all
- **Smart panel layout**: Shows principle details and asset review side-by-side

#### Metadata Editor
- **2x2 grid layout**:
  - Top-left: Asset Types (30%)
  - Bottom-left: Asset References (70%)
  - Top-right: Entities (30%)
  - Bottom-right: Principles (70%)
- **Resizable panes** with no minimum constraints
- **Minimal UI**: Simple "+" buttons for adding items

## Next Work Item: AI Actions via UI
We are implementing a Command Queue Pattern to enable AI-driven actions from VS Code:
- See `aiActionsViaUi.md` for detailed design proposal
- Allows right-click → Review/Refactor actions in VS Code
- Commands are queued in `.vql/pending-commands.json`
- User triggers processing in Claude with "Process VQL commands"
- MCP server will get new tools: `get_pending_commands`, `update_command_status`, `clear_command_queue`

## Creating Test VQL Storage Files

When testing the VS Code extension or VQL functionality, you may need to create test `vql_storage.json` files. **CRITICAL**: The format must EXACTLY match what the Rust CLI produces.

### Format Requirements:
1. **Exact Rust CLI format** - Any deviation will break VS Code extension
2. **Project-relative paths** - Paths are relative to VQL's parent directory
   - Example: `"path": "src/main.ts"` (NOT `"../src/main.ts"`)
3. **snake_case fields** - All JSON fields use snake_case (e.g., `last_modified`, `principle_reviews`)
4. **ISO 8601 timestamps** - Format: `"2025-01-28T12:00:00Z"`

### Path Understanding:
```
myproject/              <-- Workspace root (find_workspace_root() returns this)
  VQL/                  <-- VQL directory
    vql_storage.json    
  src/                  
    main.ts             <-- Path stored as: "src/main.ts"
```

### Basic Structure (with minimal identifiers):
```json
{
  "version": "1.0.0",
  "created": "2025-01-28T12:00:00Z",
  "last_modified": "2025-01-28T12:00:00Z",
  "commands": {},
  "asset_types": {
    "m": {
      "short_name": "m",
      "description": "MongoDB Schema",
      "last_modified": "2025-01-28T12:00:00Z"
    },
    "c": {
      "short_name": "c",
      "description": "Controller",
      "last_modified": "2025-01-28T12:00:00Z"
    }
  },
  "entities": {
    "u": {
      "short_name": "u",
      "description": "User",
      "last_modified": "2025-01-28T12:00:00Z"
    },
    "p": {
      "short_name": "p",
      "description": "Product",
      "last_modified": "2025-01-28T12:00:00Z"
    }
  },
  "principles": {
    "A": { "short_name": "A", "long_name": "Architecture", "last_modified": "2025-01-28T12:00:00Z" },
    "E": { "short_name": "E", "long_name": "Performance", "last_modified": "2025-01-28T12:00:00Z" },
    "S": { "short_name": "S", "long_name": "Security", "last_modified": "2025-01-28T12:00:00Z" }
  },
  "asset_references": {
    "um": {
      "short_name": "um",
      "entity": "u",
      "asset_type": "m",
      "path": "src/models/UserSchema.ts",
      "last_modified": "2025-01-28T12:00:00Z",
      "exemplar": false,
      "principle_reviews": {
        "A": { "rating": "H", "last_modified": "2025-01-28T12:00:00Z" },
        "E": { "rating": "M", "last_modified": "2025-01-28T12:00:00Z" },
        "S": { "rating": "L", "last_modified": "2025-01-28T12:00:00Z" }
      }
    }
  }
}
```

### Important Notes:
- Place in `VQL/vql_storage.json` (or `vql/vql_storage.json`)
- **CRITICAL: Unified Namespace** - All short_name values must be unique across the entire system
- **Entities** = domain objects (u=User, p=Product, x=Account) - lowercase
- **Asset Types** = code artifacts (m=MongoDB Schema, c=Controller, r=Route, v=View) - lowercase
- **Principles** = CAPITAL LETTERS (A,E,S,H,T,O,D,Y,B,N - avoiding entity/asset type identifiers)
- **Asset References** = entity+assetType (um=User MongoDB Schema, pc=Product Controller) - lowercase
- Use single characters for all short_name fields to keep UI compact
- Paths should be relative to the workspace root
- Ratings: H (High/Green), M (Medium/Amber), L (Low/Red), · (Not reviewed)
- All timestamps should be in ISO 8601 format
- The `commands` object stores CLI command history but can be empty for testing

### VS Code Badge Display:
- Badges show **principle letters colored by rating**: `AES` where each letter has a color
- Green letter = High compliance, Amber = Medium, Red = Low, Grey dot = Not reviewed
- Example: File with `AES` badge where A is green, E is amber, S is red
- With 10 principles: `AESHTOBYDN` where each letter is individually colored