# VQL VS Code Plugin Development Brief

> **Transforming VQL from CLI utility to essential development workflow component**

## 🎯 Project Vision

The VQL VS Code plugin bridges the gap between VQL's powerful command-line capabilities and the visual, interactive experience developers expect. By integrating quality indicators, smart menus, and review panels directly into VS Code, we transform code quality assessment from a manual CLI process into a seamless, always-on development experience.

**Core Philosophy**: Make code quality visible, actionable, and collaborative without disrupting existing developer workflows.

## 🚀 MVP Features (Priority 1 - 3 Week Sprint)

### 1. Project-Relative Path Management 📁
**Problem**: Full paths break team collaboration and project portability  
**Solution**: Automatic project-relative path conversion and storage

```typescript
// Target Architecture
class VQLPathResolver {
  private workspaceRoot: string;
  
  toProjectRelative(absolutePath: string): string;
  toAbsolute(projectRelativePath: string): string;
  normalize(inputPath: string): string; // Always forward slashes
  validateWorkspaceBoundary(path: string): boolean;
}
```

**Requirements**:
- [x] Auto-detect VS Code workspace root (`.vscode/` directory)
- [x] Convert absolute → project-relative paths in JSON storage  
- [x] Cross-platform path normalization (forward slashes)
- [x] Migration tool for existing absolute paths
- [x] Validate assets exist within workspace

### 2. Principle-Based Status Icons (RAG System) 🎨
**Problem**: No visual feedback for code quality status  
**Solution**: Custom SVG icons with red/amber/green status system

```typescript
// Target Architecture  
interface PrincipleIcon {
  principle: string;           // 'architecture', 'security', 'performance'
  status: 'HIGH'|'MEDIUM'|'LOW'|'NONE';
  iconPath: string;            // Custom SVG file
  color: string;               // Status color (#22c55e, #f59e0b, #ef4444)
}
```

**Visual Design**:
```
📁 src/
  📄 UserController.js    🏗️🔒⚡ (architecture, security, performance)
                         🟢🟢🟡
  📄 ProfileController.js 🏗️🔒   (missing performance review)
                         🟢🔴
```

**Requirements**:
- [x] Custom SVG icons for principle types (🏗️🔒⚡📚🧪♻️)
- [x] File explorer integration with icon arrays
- [x] Editor gutter decorations for line-level indicators
- [x] Hover tooltips with principle details
- [x] Real-time updates on JSON storage changes

### 3. Dynamic Context Menu System 🖱️
**Problem**: CLI command discovery and syntax learning curve  
**Solution**: Context-aware menus that guide users to appropriate VQL actions

```typescript
// Target Architecture
interface VQLContextMenu {
  getMenuItems(context: FileContext): MenuItem[];
  executeVQLCommand(command: string): Promise<void>;
  showReviewSubmenu(assetRef: string): MenuItem[];
}
```

**Menu Behavior**:
```
Unregistered File → Register as Asset → Entity/Type Selection
Registered Asset  → Review/Refactor Actions → MCP Execution  
Review Results    → Show Details → Open Review Panel
```

**Requirements**:
- [x] Context-sensitive menu based on VQL registration status
- [x] Progressive command discovery (simple → advanced)
- [x] MCP command execution from menu selections
- [x] Smart suggestions based on file patterns and project context

### 4. Review Summary Panel 📊
**Problem**: CLI review output lacks rich formatting and accessibility  
**Solution**: Dedicated webview panel with markdown rendering and interactive features

```typescript
// Target Architecture
class VQLReviewPanel {
  createPanel(assetRef: string): vscode.WebviewPanel;
  updateReviewContent(reviews: ReviewData[]): void;
  renderMarkdownReview(review: Review): string;
  addActionButtons(): void; // Re-review, refactor, export
}
```

**Panel Features**:
- [x] Rich markdown rendering for review content
- [x] Expandable sections per principle with timestamps
- [x] Historical review tracking and comparison
- [x] One-click re-review and refactor actions
- [x] Export capabilities for team sharing

## 🛠️ Technical Stack

### Core Extension Framework
```json
{
  "engines": { "vscode": "^1.85.0" },
  "dependencies": {
    "@types/vscode": "^1.85.0",
    "vscode-languageclient": "^9.0.1",
    "@vscode/webview-ui-toolkit": "^1.4.0",
    "chokidar": "^3.5.3",
    "fast-glob": "^3.3.2"
  }
}
```

### JSON Storage Management
```typescript
import * as fs from 'fs/promises';
import { JSONPath } from 'jsonpath-plus';  // Query operations
import Ajv from 'ajv';                     // Schema validation

class VQLStorageManager {
  private readonly storageFile = '.vscode/vql/storage.json';
  private schema: object;
  
  async load(): Promise<VQLData>;
  async save(data: VQLData): Promise<void>;
  async migrate(): Promise<void>;
  validate(data: VQLData): boolean;
}
```

### MCP Client Integration
```typescript
import { Client } from '@modelcontextprotocol/sdk/client/index.js';

class VQLMCPClient {
  private client: Client;
  
  async connect(): Promise<void>;
  async executeCommand(command: string): Promise<VQLResult>;
  async getAssetInfo(assetRef: string): Promise<AssetData>;
  onDisconnect(callback: () => void): void;
}
```

## 📋 Development Timeline

### Week 1: Foundation & Path Management
- [ ] **Day 1-2**: Project setup with TypeScript, dependencies, and build system
- [ ] **Day 3-4**: VQLPathResolver implementation with workspace detection
- [ ] **Day 5**: JSON storage manager with schema validation and migration
- [ ] **Weekend**: MCP client integration and basic command execution

### Week 2: Visual System & Icons  
- [ ] **Day 1-2**: Custom SVG icon creation (5-7 principle types)
- [ ] **Day 3**: File decoration provider with principle icon arrays
- [ ] **Day 4**: Editor gutter integration with hover tooltips
- [ ] **Day 5**: Real-time JSON file watching and UI updates
- [ ] **Weekend**: Context menu system foundation

### Week 3: Interactive Features & Polish
- [ ] **Day 1-2**: Dynamic context menu with VQL actions
- [ ] **Day 3**: Webview panel creation with markdown rendering
- [ ] **Day 4**: Review history and action button integration
- [ ] **Day 5**: Error handling, user feedback, and performance optimization
- [ ] **Weekend**: Testing, documentation, and deployment preparation

## 🎨 Visual Design System

### Icon Specifications
```
Principle Types:
🏗️ Architecture  → building.svg    (Structure, patterns, design)
🔒 Security      → shield.svg      (Auth, validation, encryption) 
⚡ Performance   → lightning.svg   (Speed, efficiency, resources)
📚 Documentation → book.svg        (Comments, README, guides)
🧪 Testing       → test-tube.svg   (Unit, integration, coverage)
♻️ Maintainability → recycle.svg   (Refactoring, tech debt)

Status Colors:
🟢 HIGH    (#22c55e) → Excellent compliance
🟡 MEDIUM  (#f59e0b) → Acceptable, room for improvement  
🔴 LOW     (#ef4444) → Needs attention
⚪ NONE    (#9ca3af) → Not yet reviewed
```

### UI Layout Patterns
```
File Explorer Badge: [🏗️🔒⚡] or "3P" (3 principles)
Editor Gutter: Stacked icons with hover details
Status Bar: "VQL: 🟢🟡🔴 (15 assets, 67% compliance)"
Review Panel: Expandable sections with action buttons
```

## 🔗 Integration Points

### VQL CLI/MCP Commands
```typescript
// Plugin translates UI actions to VQL commands
const menuAction = "Review against security principle";
const vqlCommand = ":uc.rv(s)";  // Review UserController against security
await mcpClient.execute(vqlCommand);
```

### JSON Storage Schema
```json
{
  "vql_version": "1.0",
  "assets": {
    "uc": {
      "entity": "u",
      "type": "c", 
      "path": "src/UserController.js",  // ← Workspace-relative
      "exemplar": true,
      "reviews": {
        "a": { "content": "...", "rating": "HIGH", "timestamp": "..." }
      }
    }
  }
}
```

### File System Structure
```
project-root/
├── .vscode/
│   ├── vql/
│   │   ├── storage.json      ← Main VQL data
│   │   ├── principles.json   ← Custom principles  
│   │   └── settings.json     ← Plugin preferences
│   └── extensions.json       ← Recommend VQL plugin
└── src/
    ├── UserController.js     ← Monitored assets
    └── ...
```

## 📈 Success Metrics

### Technical Performance
- [ ] Extension activation: < 500ms
- [ ] File decoration updates: < 100ms after JSON changes  
- [ ] MCP command execution: < 2s average
- [ ] Memory usage: < 50MB for typical projects
- [ ] Zero JSON corruption incidents

### User Experience  
- [ ] Time to first VQL action: < 2 minutes
- [ ] Context menu usage: > 60% of interactions
- [ ] Review panel engagement: > 30% of sessions
- [ ] Icon recognition: > 90% accuracy in testing
- [ ] Feature discoverability: Users find 80% of features without docs

### Business Impact
- [ ] Daily active users growth
- [ ] Enterprise trial conversion rate  
- [ ] User retention after first week
- [ ] Quality improvement velocity in teams using plugin

## 🛡️ Risk Mitigation

### Technical Risks
- **MCP Connection Issues**: Graceful degradation to CLI fallback mode
- **VS Code API Changes**: Pin to stable API versions, extensive testing
- **JSON Corruption**: Atomic writes with automatic backup/restore
- **Large Project Performance**: Lazy loading, caching, and incremental updates

### User Experience Risks  
- **Learning Curve**: Interactive tutorial and contextual help system
- **Information Overload**: Progressive disclosure, customizable UI density
- **Cross-Platform Issues**: Automated testing on Windows/Mac/Linux
- **Team Adoption**: Clear migration guides and setup documentation

## 🚀 Getting Started with Claude Code

### Prerequisites
```bash
# Ensure you have the VQL CLI and MCP server available
npm install -g @vql/cli
vql --version

# Verify MCP server is running
curl http://localhost:3000/mcp/health
```

### Claude Code Integration
This project is designed for development with Claude Code AI assistant. Key integration points:

1. **Incremental Development**: Each feature builds on previous foundations
2. **Test-Driven**: Every feature should have corresponding test cases
3. **Documentation**: Code should be self-documenting with TypeScript interfaces
4. **Modular Architecture**: Components should be independently testable

### Development Commands
```bash
npm run dev          # Start development with hot reload
npm run test         # Run test suite  
npm run build        # Build extension for distribution
npm run package      # Create VSIX package for installation
```

## 🗺️ Future Roadmap

### Phase 2: Intelligence & Analytics (Weeks 4-8)
- [ ] **Quality Dashboard**: Project-wide health metrics with trend visualization
- [ ] **Smart Suggestions**: AI-powered review recommendations based on patterns
- [ ] **Team Collaboration**: Shared exemplars and principle libraries
- [ ] **Performance Analytics**: Quality velocity and improvement tracking

### Phase 3: Workflow Integration (Weeks 9-16)  
- [ ] **Git Integration**: PR quality checks and pre-commit hooks
- [ ] **CI/CD Integration**: Quality gates and deployment blockers
- [ ] **Task Management**: Auto-create issues from quality findings
- [ ] **Code Actions**: Refactoring suggestions based on exemplar patterns

### Phase 4: Enterprise Features (Months 4-6)
- [ ] **Compliance Reporting**: Audit trails for regulatory requirements
- [ ] **Team Management**: Role-based access and quality governance
- [ ] **Custom Principles**: Industry-specific quality standards
- [ ] **External Integrations**: SonarQube, Checkmarx, etc.

---

**Ready to transform code quality from command-line utility to visual development experience? Let's build the future of AI-assisted coding with quality guardrails built in.**