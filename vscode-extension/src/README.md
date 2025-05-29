# VQL VS Code Extension

Visual code quality indicators for [VQL (Vibe Query Language)](https://github.com/darinkishore/vql) tracked assets.

## About VQL

VQL is a code quality management system that helps teams maintain high standards through principle-based reviews. The VS Code extension brings VQL's insights directly into your editor, showing compliance ratings as visual indicators in the file explorer.

For more information about VQL:
- [Main VQL Repository](https://github.com/darinkishore/vql)
- [VQL CLI Documentation](../README.md)
- [Architecture Overview](../architecture.md)

## Features

- **File Decorations**: Shows principle letters (A, S, P, etc.) colored by their compliance rating
- **Badge Display**: 
  - **Principle letters** appear in the badge (e.g., "ASP" for Architecture, Security, Performance)
  - **Color indicates rating**: Each letter is colored based on compliance:
    - 🟢 Green letter = High compliance
    - 🟡 Amber letter = Medium compliance
    - 🔴 Red letter = Low compliance
    - ⚪ Grey dot (·) = Not reviewed
- **Examples**:
  - `ASP` where A is green, S is amber, P is red
  - `A·P` where Architecture is green, middle principle unreviewed, Performance is red
  - `···` when no principles are reviewed
- **Overall File Color**: Reflects the lowest compliance rating across all principles
- **Detailed Tooltips**: Hover over badges to see:
  - Full principle names
  - Individual ratings with emoji indicators
  - Example: "🟢 A: Architecture (High)"
- **Real-time Updates**: Automatically refreshes when VQL storage changes

## Requirements

- VS Code 1.85.0 or higher
- [VQL CLI](https://github.com/darinkishore/vql) installed and initialized in your workspace
  - Install: `cargo install vql` (or build from source)
  - Initialize: `vql -su` in your project root

## How It Works

1. The extension activates when it detects a `VQL/vql_storage.json` file in your workspace
2. File decorations show **principle letters colored by rating**:
   - Each position shows a principle letter (A, S, P, etc.)
   - The color of each letter indicates its compliance rating
   - Unreviewed principles show as grey dots (·)
3. **Reading the badges**:
   - `ASP` = Three principles reviewed (Architecture, Security, Performance)
   - Colors tell you ratings: green=High, amber=Medium, red=Low
   - Position matters: 1st position = 1st principle in your VQL config
4. Overall file color indicates the lowest compliance level across all principles
5. Decorations update automatically when reviews are added or changed

## Commands

- `VQL: Refresh File Decorations` - Manually refresh the file decorations

## Development

```bash
cd vscode-extension
npm install
npm run compile
```

### Testing the Extension

To test the VS Code extension using the Extension Development Host:

1. Open VS Code in the extension directory:
   ```bash
   cd vscode-extension
   code .
   ```

2. Press **F5** to launch the Extension Development Host
   - This starts a new VS Code instance with the extension loaded

3. In the Extension Development Host window, open the test workspace:
   - File → Open Folder → Select `testVSCodeUI` folder
   - The extension will activate when it detects the VQL directory

4. You should see file decorations appear next to the TypeScript files showing their compliance ratings

Note: The `testVSCodeUI` folder contains sample VQL data and TypeScript files for testing the extension's functionality.

### Test Data Examples

The `testVSCodeUI/VQL/vql_storage.json` demonstrates various badge displays:

1. **Fully reviewed asset** (`extension.ts`): `APSETMDCRU`
   - Mix of ratings: mostly green (H), some amber (M), one red (L)
   - Shows as overall red due to the Low rating on U (Unit Testing)

2. **Partially reviewed asset** (`decorationProvider.ts`): `APSET-----`
   - First 5 principles reviewed with mixed ratings
   - Last 5 principles show as hyphens (unreviewed)
   - Overall red due to Low ratings

3. **Minimally reviewed asset** (`storageReader.ts`): `APS-------`
   - Only first 3 principles reviewed (all High)
   - Shows as green overall

4. **Unreviewed asset** (`test_controller.js`): `----------`
   - All principles show as hyphens
   - Grey overall color

This test data helps verify:
- Unicode monospace character rendering
- Color mapping for H/M/L ratings
- Hyphen display for unreviewed principles
- Overall file color based on lowest rating

## Building and Publishing

```bash
# Package the extension
npm run package

# This creates a .vsix file that can be:
# - Shared with team members
# - Published to VS Code Marketplace
# - Installed via: code --install-extension vql-vscode-*.vsix
```

## Integration with VQL Workflow

1. **Initialize VQL** in your project: `vql -su`
2. **Add principles** from templates: `vql -alp principles.md`
3. **Track assets**: `vql -aa myfile.ts -e user -t component`
4. **Review code**: `vql -rap myfile.ts`
5. **See results** in VS Code with colored indicators

The extension automatically updates when reviews are added or modified through the CLI.

## Testing with Sample Data

To test the extension without using the VQL CLI, create a `VQL/vql_storage.json` file that **EXACTLY** matches the format produced by the Rust CLI:

### Critical Format Requirements:
1. **Must match Rust CLI JSON format exactly** - field names, structure, everything
2. **Paths must be project-relative** - relative to the parent of the VQL directory
   - ✅ Correct: `"path": "src/controllers/UserController.ts"`
   - ❌ Wrong: `"path": "../src/controllers/UserController.ts"`
3. **All fields use snake_case** (e.g., `last_modified`, `principle_reviews`)
4. **Timestamps in ISO 8601 format** with Z suffix

### Minimal Example:
```json
{
  "version": "1.0.0",
  "created": "2025-01-28T12:00:00Z",
  "last_modified": "2025-01-28T12:00:00Z",
  "commands": {},
  "asset_types": {
    "c": { "short_name": "c", "description": "Controller", "last_modified": "2025-01-28T12:00:00Z" }
  },
  "entities": {
    "u": { "short_name": "u", "description": "User", "last_modified": "2025-01-28T12:00:00Z" }
  },
  "principles": {
    "A": { "short_name": "A", "long_name": "Architecture", "last_modified": "2025-01-28T12:00:00Z" },
    "E": { "short_name": "E", "long_name": "Performance", "last_modified": "2025-01-28T12:00:00Z" },
    "S": { "short_name": "S", "long_name": "Security", "last_modified": "2025-01-28T12:00:00Z" }
  },
  "asset_references": {
    "uc": {
      "short_name": "uc",
      "entity": "u",
      "asset_type": "c",
      "path": "src/controllers/UserController.ts",
      "exemplar": false,
      "last_modified": "2025-01-28T12:00:00Z",
      "principle_reviews": {
        "A": { "rating": "H", "last_modified": "2025-01-28T12:00:00Z" },
        "E": { "rating": "M", "last_modified": "2025-01-28T12:00:00Z" },
        "S": { "rating": "L", "last_modified": "2025-01-28T12:00:00Z" }
      }
    }
  }
}
```

### Understanding Paths:
- **Workspace root** = parent directory of VQL folder
- If your project structure is:
  ```
  myproject/
    VQL/
      vql_storage.json
    src/
      controllers/
        UserController.ts
  ```
- Then path should be: `"src/controllers/UserController.ts"` (from myproject/ perspective)

**Important**: All identifiers must follow the **Unified Namespace** requirement - no duplicates across entities, asset types, principles, and asset references. Note that principles should use **capital letters** (A,E,S) to distinguish them from entities and asset types.