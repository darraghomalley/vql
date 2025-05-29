# VS Code Extension Development Guide for Claude

This document provides context for Claude when working on the VQL VS Code extension.

## Extension Overview

This VS Code extension provides visual indicators for VQL (Vibe Query Language) compliance ratings directly in the VS Code file explorer. It's part of the larger VQL project that helps teams maintain code quality through principle-based reviews.

## Testing Workflow

1. **Prepare the extension:**
   ```bash
   cd vscode-extension
   npm install
   npm run compile
   ```

2. **Launch Extension Development Host:**
   - Open this folder in VS Code: `code .`
   - Press **F5** to start debugging
   - A new VS Code window opens with the extension loaded

3. **Test with sample data:**
   - In the new window, open: `../testVSCodeUI`
   - This folder contains pre-configured VQL data and sample TypeScript files
   - You should see colored decorations next to files showing their compliance ratings

## Test Data Structure

The `testVSCodeUI/VQL/vql_storage.json` file contains comprehensive test data:

### Asset Examples:
1. **`ext` (extension.ts)** - Exemplar file with all 10 principles reviewed
   - Badge: `APSETMDCRU` with mixed colors (mostly green, some amber, one red)
   - Demonstrates full review coverage

2. **`dec` (decorationProvider.ts)** - Partially reviewed
   - Badge: `APSET-----` (5 principles reviewed, 5 unreviewed)
   - Shows mixed ratings and hyphen display

3. **`str` (storageReader.ts)** - Minimally reviewed
   - Badge: `APS-------` (only 3 principles, all High)
   - Green overall due to all High ratings

4. **`prc` (product_controller.js)** - All Low ratings
   - Badge: `APSETMDCRU` (all red letters)
   - Demonstrates worst-case scenario

5. **`tsc` (test_controller.js)** - Unreviewed
   - Badge: `----------` (all hyphens)
   - Grey overall color

### Testing Points:
- **Unicode rendering**: Mathematical monospace letters (𝙰-𝚉) and hyphen (−)
- **Color mapping**: Each principle letter colored individually
- **Mixed states**: Various combinations of H/M/L/unreviewed
- **File paths**: Relative paths from workspace root
- **Entities & types**: Single-character identifiers following lowercase convention

## Key Files

- `src/extension.ts` - Main entry point, activates when VQL directory is detected
- `src/decorationProvider.ts` - Creates visual decorations based on compliance ratings
- `src/matrixViewProvider.ts` - Read-only compliance matrix view
- `src/vqlMetadataProvider.ts` - Metadata editor that uses CLI for all write operations
- `src/storageWatcher.ts` - Monitors VQL storage changes for real-time updates
- `src/storageReader.ts` - Parses VQL storage JSON files
- `src/types.ts` - TypeScript interfaces matching VQL data structures

## Architecture

The extension maintains a clean separation between read and write operations:
- **Read operations**: Direct JSON parsing for performance (decorations, matrix view)
- **Write operations**: All go through VQL CLI to ensure consistency (metadata editor)
- This ensures the extension never corrupts VQL storage or gets out of sync with CLI changes

## Integration with VQL CLI

The extension reads data from `VQL/vql_storage.json` which is managed by the VQL CLI:
- `vql -su` - Initialize VQL in a project
- `vql -aa` - Add assets to track
- `vql -rap` - Review assets against principles
- `vql -migrate-paths` - Convert absolute paths to relative (for team sharing)

## Color Scheme

- 🟢 Green (#22c55e) - High compliance
- 🟡 Amber (#f59e0b) - Medium compliance  
- 🔴 Red (#ef4444) - Low compliance
- ⚪ Grey (#9ca3af) - Not reviewed

## Common Tasks

### Adding new features:
1. Check `../vsCodeBrief.md` for the overall vision
2. Ensure compatibility with VQL storage format in `../src/models/json_storage.rs`
3. Test with both `testVSCodeUI` and real VQL projects

### Debugging issues:
- Check VS Code Developer Tools (Help → Toggle Developer Tools)
- Look for "VQL:" prefixed messages in the console
- Verify VQL storage file exists and is valid JSON

## Creating Test VQL Storage Files

**CRITICAL**: Test data MUST exactly match the format produced by the Rust CLI. Any deviation will break the extension.

### Format Requirements:
1. **Exact JSON structure** - Must match Rust's serde serialization
2. **snake_case field names** - `last_modified`, `principle_reviews`, etc.
3. **Project-relative paths** - Relative to VQL's parent directory
4. **ISO 8601 timestamps** - With 'Z' suffix
5. **Unified Namespace** - No duplicate identifiers across categories

### Path Examples:
```
testVSCodeUI/           <-- Workspace root (VQL's parent)
  VQL/                  
    vql_storage.json    
  src/                  
    extension.ts        <-- Path: "src/extension.ts" (NOT "../src/extension.ts")
```

For testing the extension, you can create a `vql_storage.json` file manually. Here's a minimal structure with single-character identifiers:

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
    },
    "r": {
      "short_name": "r",
      "description": "Route",
      "last_modified": "2025-01-28T12:00:00Z"
    },
    "v": {
      "short_name": "v",
      "description": "View Component",
      "last_modified": "2025-01-28T12:00:00Z"
    }
  },
  "entities": {
    "u": {
      "short_name": "u",
      "description": "User",
      "last_modified": "2025-01-28T12:00:00Z"
    },
    "x": {
      "short_name": "x",
      "description": "Account",
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
    "P": { "short_name": "P", "long_name": "Performance", "last_modified": "2025-01-28T12:00:00Z" },
    "S": { "short_name": "S", "long_name": "Security", "last_modified": "2025-01-28T12:00:00Z" },
    "E": { "short_name": "E", "long_name": "Error Handling", "last_modified": "2025-01-28T12:00:00Z" },
    "T": { "short_name": "T", "long_name": "Type Safety", "last_modified": "2025-01-28T12:00:00Z" },
    "M": { "short_name": "M", "long_name": "Modularity", "last_modified": "2025-01-28T12:00:00Z" },
    "D": { "short_name": "D", "long_name": "Documentation", "last_modified": "2025-01-28T12:00:00Z" },
    "C": { "short_name": "C", "long_name": "Code Style", "last_modified": "2025-01-28T12:00:00Z" },
    "R": { "short_name": "R", "long_name": "Readability", "last_modified": "2025-01-28T12:00:00Z" },
    "U": { "short_name": "U", "long_name": "Unit Testing", "last_modified": "2025-01-28T12:00:00Z" }
  },
  "asset_references": {
    "um": {
      "short_name": "um",
      "entity": "u",
      "asset_type": "m",
      "path": "src/models/UserSchema.ts",
      "last_modified": "2025-01-28T12:00:00Z",
      "exemplar": true,
      "principle_reviews": {
        "A": {"rating": "H", "last_modified": "2025-01-28T12:00:00Z"},
        "P": {"rating": "H", "last_modified": "2025-01-28T12:00:00Z"},
        "S": {"rating": "M", "last_modified": "2025-01-28T12:00:00Z"},
        "E": {"rating": "H", "last_modified": "2025-01-28T12:00:00Z"},
        "T": {"rating": "H", "last_modified": "2025-01-28T12:00:00Z"},
        "M": {"rating": "H", "last_modified": "2025-01-28T12:00:00Z"},
        "D": {"rating": "M", "last_modified": "2025-01-28T12:00:00Z"},
        "C": {"rating": "H", "last_modified": "2025-01-28T12:00:00Z"},
        "R": {"rating": "H", "last_modified": "2025-01-28T12:00:00Z"},
        "U": {"rating": "L", "last_modified": "2025-01-28T12:00:00Z"}
      }
    },
    "uc": {
      "short_name": "uc",
      "entity": "u",
      "asset_type": "c",
      "path": "src/controllers/UserController.ts",
      "last_modified": "2025-01-28T12:00:00Z",
      "exemplar": false,
      "principle_reviews": {
        "A": {"rating": "M", "last_modified": "2025-01-28T12:00:00Z"},
        "P": {"rating": "L", "last_modified": "2025-01-28T12:00:00Z"},
        "S": {"rating": "L", "last_modified": "2025-01-28T12:00:00Z"},
        "E": {"rating": "M", "last_modified": "2025-01-28T12:00:00Z"},
        "T": {"rating": "M", "last_modified": "2025-01-28T12:00:00Z"}
      }
    },
    "pv": {
      "short_name": "pv",
      "entity": "p",
      "asset_type": "v",
      "path": "src/components/ProductCard.tsx",
      "last_modified": "2025-01-28T12:00:00Z",
      "exemplar": false,
      "principle_reviews": {
        "A": {"rating": "H", "last_modified": "2025-01-28T12:00:00Z"},
        "P": {"rating": "H", "last_modified": "2025-01-28T12:00:00Z"},
        "S": {"rating": "H", "last_modified": "2025-01-28T12:00:00Z"}
      }
    }
  }
}
```

### Key Elements:
1. **Unified Namespace**: All identifiers must be unique across entities, asset types, principles, and asset references
2. **Entities** (lowercase): u (User), x (Account), p (Product)
3. **Asset Types** (lowercase): m (MongoDB Schema), c (Controller), r (Route), v (View Component)
4. **Principles** (UPPERCASE): A,P,S,E,T,M,D,C,R,U to distinguish from entities/asset types
5. **Asset References** (lowercase): entity+assetType (e.g., "um" = User MongoDB Schema)
6. **Badge Display** shows all principles alphabetically with letters colored by compliance status:
   - Asset "um": `APSETMDCRU` where each letter has its rating color (all 10 principles reviewed)
   - Asset "uc": `APSETMDCRU` (A,P,S,E,T reviewed and colored, M,D,C,R,U shown as grey letters)
   - Asset "pv": `APSETMDCRU` (A,P,S reviewed and colored, E,T,M,D,C,R,U shown as grey letters)

### Visual Examples (Badge Display):
- `APSETMDCRU` - All 10 principles displayed alphabetically, each letter colored by rating (green/amber/red)
- `APSETMDCRU` - A,P,S reviewed (green/amber/red), T,M,D,C,R,U unreviewed (grey letters)
- `APSETMDCRU` - No principles reviewed (all grey letters)

### Color Mapping:
- **Green letter** = High compliance (H rating)
- **Amber letter** = Medium compliance (M rating)
- **Red letter** = Low compliance (L rating)
- **Grey letter** = Not reviewed

### Display Rules:
- Principles are displayed **alphabetically left to right** (A, B, C, D, E, etc.)
- All principles always show their letter, colored by compliance status
- Each position maintains consistent spacing using monospace Unicode characters

### Unified Namespace Requirements:
- **CRITICAL**: All short_name values must be unique across the entire VQL system
- **Entities** (lowercase): u, x, p (avoid uppercase letters reserved for principles)
- **Asset Types** (lowercase): m, c, r, v (avoid uppercase letters)
- **Principles** (UPPERCASE): A,P,S,E,T,M,D,C,R,U (avoid lowercase used by entities/types)
- **Asset References** (lowercase): Combinations like "um", "uc", "pv"
- **Why**: Enables mixed references in commands like `:tm.rf(A,S,uc,pc)` where principles and assets are referenced together

## Future Enhancements

Per the project vision, potential additions include:
- Context menu actions for quick reviews
- Review panel with detailed compliance information
- MCP server integration for AI-assisted reviews
- Git integration for tracking quality over time