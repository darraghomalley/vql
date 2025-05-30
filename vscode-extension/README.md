# VQL VS Code Extension

Visual compliance indicators for [VQL (Vibe Query Language)](https://github.com/darraghenright/vql) in Visual Studio Code.

## Overview

The VQL VS Code extension brings real-time code quality visualization to your editor. It displays compliance ratings as colored badges in the file explorer and provides an interactive matrix view for comprehensive quality tracking.

## Features

### 🎯 File Explorer Badges
- **Colored Square Indicators**: Shows overall compliance at a glance
  - 🟩 Green = All reviewed principles are High compliance
  - 🟨 Amber = Lowest rating is Medium
  - 🟥 Red = At least one Low rating
  - 🟪 Purple = No principles reviewed yet

### 📊 Interactive Compliance Matrix
- **Comprehensive Overview**: See all assets and principles in a grid view
- **Color-Coded Cells**: Each cell shows compliance status
  - 🟩 High compliance
  - 🟨 Medium compliance  
  - 🟥 Low compliance
  - **—** Bold hyphen for not reviewed (subtle grey)
- **Interactive Selection**:
  - Click any cell to view principle and asset review details
  - Ctrl+click (Cmd+click on Mac) to select multiple cells
  - Selected cells show blue border overlay
  - Batch action buttons appear when cells are selected (Review, Refactor, Clear)
- **Click Navigation**: Click any file name to open it in the editor
- **Resizable Panes**: Adjust the layout to your preference
- **Dual Panel Details**: View principle guidance alongside asset review details

### 🔄 Real-Time Updates
- Automatically refreshes when VQL storage changes
- Instant feedback as you review and refactor code
- Seamless integration with VQL CLI operations

### 🎨 Theme Integration
- Uses VS Code's theme colors for consistent appearance
- Adapts to light and dark themes
- Respects user's color preferences

### 📁 File Explorer Integration
- **Right-click Context Menu**: Add files to VQL tracking directly
  - Right-click any file → "VQL" → "Add Asset Ref."
  - Opens metadata editor with file path pre-populated
  - Automatically checks if file is already tracked

### 📝 Metadata Editor
- **2x2 Grid Layout**: Organized editing interface
  - Asset Types (top-left) and Asset References (bottom-left)
  - Entities (top-right) and Principles (bottom-right)
- **Resizable Panes**: Adjust to your workflow needs
- **Minimal UI**: Clean "+" buttons for adding new items
- **CLI Integration**: All edits use VQL commands for consistency

## Installation

### From VSIX Package
```bash
# From the vscode-extension directory
npm install
npm run compile
vsce package

# Install the generated VSIX file
code --install-extension vql-vscode-*.vsix
```

### From Source (Development)
```bash
# Clone the VQL repository
git clone https://github.com/darraghenright/vql.git
cd vql/vscode-extension

# Install dependencies and compile
npm install
npm run compile

# Open in VS Code
code .

# Press F5 to launch Extension Development Host
```

## Requirements

- VS Code 1.85.0 or higher
- VQL CLI installed and project initialized (`vql -su`)
- Active VQL storage file (`VQL/vql_storage.json`)

## Usage

### Getting Started

1. **Initialize VQL** in your project:
   ```bash
   vql -su
   ```

2. **Add principles**:
   ```bash
   vql -pr -add A Architecture "Clean architecture and separation of concerns"
   vql -pr -add S Security "Input validation and protection against vulnerabilities"
   vql -pr -add P Performance "Efficient algorithms and resource management"
   ```

3. **Track assets**:
   ```bash
   vql -ar -add uc u c "src/controllers/UserController.ts"
   ```

4. **Review code** (via CLI or AI assistant):
   ```bash
   vql :uc.rv(-pr)  # Review all principles
   ```

5. **See results** in VS Code:
   - File explorer shows colored badges
   - Open Command Palette → "VQL: Show Compliance Matrix"

### Understanding the Indicators

#### File Explorer Badges
- Shows a colored square emoji based on the lowest compliance rating
- Hover to see detailed breakdown of all principle ratings
- Purple square (🟪) indicates no reviews yet

#### Compliance Matrix
- **Rows**: Your tracked assets (files)
- **Columns**: Your defined principles
- **Cells**: Compliance ratings (🟩🟨🟥 or —)
- **File Names**: Colored by overall compliance
- **Interaction**:
  - Click cells to see review details and principle guidance
  - Ctrl+click to select multiple cells for batch operations
  - Use Review/Refactor buttons for bulk actions (coming soon)
  - Clear button to deselect all

### Commands

Access via Command Palette (Ctrl/Cmd+Shift+P):
- `VQL: Show Compliance Matrix` - Open the interactive matrix view (read-only)
- `VQL: Show Metadata` - Edit principles, entities, asset types, and assets
- `VQL: Refresh Decorations` - Manually refresh file badges
- `VQL: Toggle Decorations` - Show/hide compliance badges

## Configuration

The extension reads from your project's `VQL/vql_storage.json` file. No additional configuration is needed - it automatically detects and uses your VQL setup.

### Custom Theme Colors

You can customize the compliance colors in your VS Code settings:

```json
{
  "workbench.colorCustomizations": {
    "vql.highCompliance": "#22c55e",
    "vql.mediumCompliance": "#f59e0b",
    "vql.lowCompliance": "#ef4444"
  }
}
```

## Development

### Architecture
The VS Code extension follows a clean separation of concerns:
- **Read Operations**: Direct JSON parsing for fast visualization
- **Write Operations**: All modifications go through the VQL CLI to ensure consistency
- The Metadata view (`vqlMetadataProvider.ts`) properly uses CLI commands for all edits
- The Matrix view is read-only for viewing compliance data

### Project Structure
```
vscode-extension/
├── src/
│   ├── extension.ts          # Main extension entry point
│   ├── decorationProvider.ts # File badge logic
│   ├── matrixViewProvider.ts # Compliance matrix webview (read-only)
│   ├── vqlMetadataProvider.ts # Metadata editor (uses CLI for writes)
│   ├── storageReader.ts      # VQL storage parser
│   ├── storageWatcher.ts     # File system watcher
│   └── types.ts              # TypeScript interfaces
├── package.json              # Extension manifest
└── tsconfig.json            # TypeScript configuration
```

### Testing

1. Open the extension folder in VS Code
2. Press F5 to launch Extension Development Host
3. Open the `testVSCodeUI` folder in the new window
4. Verify badges appear and matrix view works

### Building for Distribution

```bash
# Install vsce globally if needed
npm install -g vsce

# Package the extension
vsce package

# Creates: vql-vscode-{version}.vsix
```

## Troubleshooting

### No badges appearing?
- Ensure VQL is initialized: `vql -su`
- Check for `VQL/vql_storage.json` in your workspace
- Try manual refresh: Command Palette → "VQL: Refresh Decorations"

### Badges show wrong colors?
- Verify your reviews are saved: `vql {asset}?`
- Check compliance ratings are set (H/M/L)
- Ensure paths in VQL storage are relative to workspace root

### Matrix view empty?
- Confirm you have tracked assets: `vql -ar`
- Check browser console for errors (Help → Toggle Developer Tools)

## Contributing

We welcome contributions! Please:
1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Submit a pull request

## License

MIT License - see the [LICENSE](../LICENSE) file for details.

## Acknowledgments

- Built for the VQL ecosystem
- Uses VS Code's decoration and webview APIs
- Inspired by the need for visual quality tracking in AI-assisted development