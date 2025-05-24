# VQL (Vibe Query Language) Guidance for LLMs

This document provides comprehensive guidance for understanding and using VQL (Vibe Query Language) commands in LLM-assisted software development sessions.

## MCP Server Integration (Recommended)

VQL is now available as a Model Context Protocol (MCP) server, providing structured tool access for AI assistants. This is the recommended way to use VQL in Claude Code sessions.

### Setting Up VQL MCP Server

1. **Check if VQL MCP is available**:
   ```bash
   claude mcp list
   ```
   Look for a server named `vql` or similar.

2. **If not available, add the VQL MCP server**:
   
   For a published npm package:
   ```bash
   # If @vibe-ql/mcp-server is installed globally
   claude mcp add vql vql-mcp
   ```
   
   For a local development server:
   ```bash
   # From the VQL project directory
   claude mcp add vql node ./mcp-server/dist/index.js
   ```
   
   For a project-relative installation:
   ```bash
   # If VQL is in a sibling directory
   claude mcp add vql node ../vql/mcp-server/dist/index.js
   ```

3. **Verify MCP server is active**:
   ```bash
   claude mcp list
   ```
   You should see the VQL server listed.

### First-Time MCP Setup in New Projects

When starting in a new project where VQL MCP location is unknown:

1. **Check common locations for VQL**:
   ```bash
   # Check if VQL is in a sibling directory
   ls ../vql/mcp-server/dist/index.js 2>/dev/null && echo "Found in sibling directory"
   
   # Check if VQL is in parent directory
   ls ../../vql/mcp-server/dist/index.js 2>/dev/null && echo "Found in parent directory"
   
   # Check if vql-mcp is globally installed
   which vql-mcp 2>/dev/null && echo "Found globally installed"
   ```

2. **If VQL is found, add it based on location**:
   ```bash
   # For sibling directory
   claude mcp add vql node ../vql/mcp-server/dist/index.js
   
   # For parent directory
   claude mcp add vql node ../../vql/mcp-server/dist/index.js
   
   # For global installation
   claude mcp add vql vql-mcp
   ```

3. **If VQL is not found automatically**:
   - Ask the user: "Where is your VQL installation located?"
   - Or ask: "Would you like to proceed with CLI mode instead? Use :cli"
   - If user provides path: `claude mcp add vql node /path/to/vql/mcp-server/dist/index.js`

4. **Verify the MCP server is working**:
   ```bash
   # Try listing principles as a test
   # The AI should attempt to use the list_principles() tool
   ```
   If this fails, suggest: "MCP server setup failed. Switching to CLI mode with :cli"

5. **Building VQL MCP from source** (if needed):
   If the user has VQL source but MCP isn't built:
   ```bash
   cd /path/to/vql/mcp-server
   npm install
   npm run build
   # Then add it
   claude mcp add vql node ./dist/index.js
   ```

**Important**: Always default to MCP mode first, but be ready to fall back to CLI mode if MCP setup fails.

### Using VQL via MCP Tools

When VQL MCP server is available, you'll have access to structured tools instead of parsing command strings:

- `list_principles()` - Show all principles
- `add_principle(short, long, guidance)` - Add a new principle
- `list_assets()` - Show all assets
- `store_review(asset, principle, review)` - Store a review
- `review_asset_all_principles(asset)` - AI workflow to review an asset
- `refactor_asset_principles(asset, principles)` - AI workflow to refactor

**Benefits of MCP over CLI commands**:
- Type-safe parameters
- Structured responses
- Better error handling
- No command string parsing needed
- Direct integration with Claude's tool system

### Fallback to CLI Commands

If the MCP server is not available, you can still use VQL through CLI commands as documented below. However, always prefer the MCP server when available for a better experience.

## Command Formats

VQL supports two primary command formats:

1. **CLI Format** - For direct terminal use
   - Typically prefixed with dash, e.g., `vql -pr` 
   - Question mark query format: `vql um?` or `vql um? (a,s)`

2. **LLM Format** - For AI assistant use
   - Global commands prefixed with colon and dash, e.g., `:-pr`
   - Asset commands with colon and asset name, e.g., `:um?` or `:um.st(a, "review")`

## Command Categories

LLM commands fall into three distinct categories that require different handling:

1. **Standard Commands** - Direct translations of CLI commands (e.g., `:-pr` → `vql -pr`)
   - Simply translate the command format and report the results

2. **Direct Execution Commands** - Must be executed through the CLI (e.g., `:-su`)
   - Translate to CLI format and use the Bash tool to execute the actual command
   - These commands modify the environment and must be directly executed
   - Examples: `:-su` (setup), `:-pr.get` (loading principles)

3. **Virtual Workflow Commands** - LLM-only multi-step processes (e.g., `:-rv`, `:-rf`)
   - Trigger complex AI workflows with multiple steps
   - Execute multiple CLI commands as part of analysis
   - Examples: review commands, refactor commands

**Space Flexibility**: Both formats support optional spaces between command components:
   - Both `:uc?` and `:uc ?` are valid
   - Both `:uc?(a,s)` and `:uc ? (a,s)` are valid
   - Both `vql uc?` and `vql uc ?` are valid
   
   **Important**: While the documentation shows space flexibility, always test the actual CLI behavior. Some commands may have specific space requirements that differ from the documented examples.

**Multiple Principle Filtering**: Use comma-separated principle identifiers:
   - `:uc?(a,s)` - Query asset 'uc' for principles 'a' and 's'
   - `:uc.rv(a,s,p)` - Review principles 'a', 's', and 'p' for asset 'uc'

## Command Translation Reference

This table provides an exact mapping between LLM Virtual VQL syntax and CLI VQL syntax. LLMs should translate from the LLM Virtual VQL Syntax to the CLI VQL Syntax when processing commands.

| Action                                           | LLM Virtual VQL Syntax                                             | CLI VQL Syntax                                                   | Notes                                  |
|--------------------------------------------------|--------------------------------------------------------------------|-----------------------------------------------------------------|----------------------------------------|
| VQL ON                                           | `:-vql on`                                                         | N/A                                                             | LLM-only command                       |
| VQL OFF                                          | `:-vql off`                                                        | N/A                                                             | LLM-only command                       |
| SHOW ALL PRINCIPLES                              | `:-pr`                                                             | `vql -pr`                                                        |                                        |
| GET PRINCIPLES                                   | `:-pr.get([principlesMdPath])` <br> Example: `:-pr.get("C:/Reference/principles.md")` | `vql -pr -get "[principlesMdPath]"` <br> Example: `vql -pr -get "C:/Reference/principles.md"` | DIRECT COMMAND: Must be executed by calling CLI through Bash tool |
| SHOW ALL ENTITIES                                | `:-er`                                                             | `vql -er`                                                        |                                        |
| SHOW ALL ASSET TYPES                             | `:-at`                                                             | `vql -at`                                                        |                                        |
| SHOW ALL ASSET REFERENCES                        | `:-ar`                                                             | `vql -ar`                                                        |                                        |
| ADD PRINCIPLE                                    | `:-pr.add([principleShortName], [principleLongName], [principleGuidance])` <br> Example: `:-pr.add(a, Architecture, "Architecture Principles")` | `vql -pr -add [principleShortName] [principleLongName] "[principleGuidance]"` <br> Example: `vql -pr -add a Architecture "Architecture Principles"` | Guidance must be in quotes             |
| ADD ENTITY                                       | `:-er.add([entityShortName], [entityLongName])` <br> Example: `:-er.add(u, User)` | `vql -er -add [entityShortName] [entityLongName]` <br> Example: `vql -er -add u User` |                                        |
| ADD ASSET TYPE                                   | `:-at.add([assetTypeShortName], [assetTypeLongName])` <br> Example: `:-at.add(c, Controller)` | `vql -at -add [assetTypeShortName] [assetTypeLongName]` <br> Example: `vql -at -add c Controller` |                                        |
| ADD ASSET REFERENCE                              | `:-ar.add([assetRef], [entityShortName], [assetTypeShortName], [assetPath])` <br> Example: `:-ar.add(uc, u, c, "C:/Project/UserController.js")` | `vql -ar -add [assetRef] [entityShortName] [assetTypeShortName] "[assetPath]"` <br> Example: `vql -ar -add uc u c "C:/Project/UserController.js"` | Path must be in quotes |
| STORE ASSET REVIEW                               | `:[assetRef].st([principle1ShortName], [reviewContent])` <br> Example: `:uc.st(a, "Review Content")` | `vql -st [assetRef] [principle1ShortName] "[reviewContent]"` <br> Example: `vql -st uc a "Review Content"` | Content must be in quotes              |
| RETRIEVE ALL REVIEWS FOR SPECIFIC ASSET          | `:[assetRef]?` <br> Example: `:uc?`                                | No direct CLI equivalent - Read from VQL storage file            | See "Handling Unsupported Commands" section |
| RETRIEVE SPECIFIC REVIEWS FOR AN ASSET           | `:[assetRef]?([principle1ShortName] [principle2ShortName])` <br> Example: `:uc?(a,s)` | No direct CLI equivalent - Read from VQL storage file            | See "Handling Unsupported Commands" section |
| SET AN ASSET AS AN EXEMPLAR                      | `:[assetRef].se([t\|f])` <br> Example: `:uc.se(t)`                 | `vql -se [assetRef] [t\|f]` <br> Example: `vql -se uc t`         | t=true, f=false                        |
| SET AN ASSET'S COMPLIANCE                        | `:[assetRef].sc([principle1ShortName] [H\|M\|L])` <br> Example: `:uc.sc(a,H)` | `vql -sc [assetRef] [principle1ShortName] [H\|M\|L]` <br> Example: `vql -sc uc a H` | H=High, M=Medium, L=Low                |
| SETUP VQL                                        | `:-su([ProjectFolderFullPath])` <br> Example: `:-su("C:/Project/Folder")` | `vql -su "[ProjectFolderFullPath]"` <br> Example: `vql -su "C:/Project/Folder"` | DIRECT COMMAND: Must be executed by calling CLI through Bash tool |
| REVIEW ALL ASSETS BY ALL PRINCIPLES              | `:-rv(*)`                                                          | N/A - Virtual LLM command                                        | LLM-only virtual command               |
| REVIEW ALL ASSETS BY SPECIFIED PRINCIPLES        | `:-rv([principle1ShortName] [principle2ShortName])` <br> Example: `:-rv(a,s)` | N/A - Virtual LLM command                                        | LLM-only virtual command               |
| REVIEW SPECIFIC ASSET BY ALL PRINCIPLES          | `:[assetRef].rv(*)` <br> Example: `:uc.rv(*)`                      | N/A - Virtual LLM command                                        | LLM-only virtual command               |
| REVIEW ASSET BY SPECIFIED PRINCIPLES             | `:[assetRef].rv([principle1ShortName] [principle2ShortName])` <br> Example: `:uc.rv(a,s)` | N/A - Virtual LLM command                                        | LLM-only virtual command               |
| REFACTOR ALL ASSETS BY ALL PRINCIPLES            | `:-rf(*)`                                                          | N/A - Virtual LLM command                                        | LLM-only virtual command               |
| REFACTOR ALL ASSETS BY SPECIFIED PRINCIPLES      | `:-rf([principle1ShortName] [principle2ShortName])` <br> Example: `:-rf(a,s)` | N/A - Virtual LLM command                                        | LLM-only virtual command               |
| REFACTOR SPECIFIC ASSET BY ALL PRINCIPLES        | `:[assetRef].rf(*)` <br> Example: `:uc.rf(*)`                      | N/A - Virtual LLM command                                        | LLM-only virtual command               |
| REFACTOR ASSET BY SPECIFIED PRINCIPLES           | `:[assetRef].rf([principle1ShortName] [principle2ShortName])` <br> Example: `:uc.rf(a,s)` | N/A - Virtual LLM command                                        | LLM-only virtual command               |
| REFACTOR ASSET USING ANOTHER ASSET AS REFERENCE  | `:[assetRef].rf([assetRef2])` <br> Example: `:up.rf(uc)`           | N/A - Virtual LLM command                                        | LLM-only virtual command               |

## Command Parameter Placeholders

When translating commands, replace these placeholders with actual values:

| Placeholder               | Description                                    | Example Value    | Format Rules                                       |
|---------------------------|------------------------------------------------|------------------|---------------------------------------------------|
| `[principleShortName]`    | Single-character principle identifier          | `a`, `s`, `p`    | Single letter, lowercase                           |
| `[principleLongName]`     | Full name of the principle                     | `Architecture`   | No quotes unless contains spaces                   |
| `[principleGuidance]`     | Description or guidance text for principle     | `"Clean Code Guidelines"` | Must be in quotes                       |
| `[principlesMdPath]`      | Path to principles markdown file               | `"C:/path/principles.md"` | Must be in quotes; use proper path format |
| `[ProjectFolderFullPath]` | Path to directory for VQL initialization        | `"C:/Project/Folder"`  | Path must be in quotes                        |
| `[entityShortName]`       | Short identifier for entity                    | `u`, `p`         | Usually single letter, lowercase                   |
| `[entityLongName]`        | Full name of the entity                        | `User`           | No quotes unless contains spaces                   |
| `[assetTypeShortName]`    | Short identifier for asset type                | `c`, `m`         | Usually single letter, lowercase                   |
| `[assetTypeLongName]`     | Full name of the asset type                    | `Controller`     | No quotes unless contains spaces                   |
| `[assetRef]`              | Asset reference identifier                     | `uc`, `pm`       | Usually 2-3 characters, no quotes                  |
| `[assetPath]`             | Path to the asset file                         | `"C:/path/file.js"` | Must be in quotes; use proper path format       |
| `[Review Content]`        | Review content or analysis                     | `"Good implementation"` | Must be in quotes                          |
| `[H\|M\|L]`               | Compliance rating (High, Medium, Low)          | `H`, `M`, `L`    | Single uppercase letter, no quotes                 |
| `[t\|f]`                  | Boolean flag (true, false)                     | `t`, `f`         | Single lowercase letter, no quotes                 |

## Command Translation Process

When an LLM encounters a VQL command in the LLM format, it should:

1. **Identify Command Type**: Determine if it's a direct command, direct executable command, or an AI-assisted "virtual" command
2. **For Direct Commands**: Translate to the equivalent CLI command before execution using the mapping table above
3. **For Direct Executable Commands**: Translate to the equivalent CLI command and execute it using the Bash tool
4. **For AI-Assisted Commands**: Perform the required analysis and then execute appropriate CLI commands

### Translation Rules

1. **Parameter Order**: CLI format may have different parameter order than LLM format
2. **Quotes**: Always add quotes around multi-word strings or file paths
3. **Commas**: Some CLI commands require commas between parameters
4. **Spaces**: Ensure spaces between command components
5. **Placeholder Replacement**: Replace all placeholders with actual values
6. **Direct Execution Commands**: Some commands like `:-su` require direct CLI execution via the Bash tool
7. **Path Expansion**: When executing commands through Bash, tilde (~) characters in paths will be automatically expanded

## Handling Unsupported CLI Commands

Some LLM format commands do not have direct CLI equivalents. When encountering these commands, LLMs should use alternative approaches to fulfill the request.

### Asset Query Commands

The asset query commands (`:[assetRef]?` and `:[assetRef]?(principles)`) are not supported by the CLI. Instead:

1. **Read the VQL storage file directly**:
   - Location: `VQL/vql_storage.json` in the project directory
   - Navigate to `asset_references.[assetRef].principle_reviews`
   - Extract and format the review information for presentation

2. **Example workflow for `:tm?`**:
   ```
   1. Read file: VQL/vql_storage.json
   2. Navigate to: asset_references.tm.principle_reviews
   3. Format and present all reviews for asset 'tm'
   ```

3. **Example workflow for `:tm?(a,s)`**:
   ```
   1. Read file: VQL/vql_storage.json
   2. Navigate to: asset_references.tm.principle_reviews
   3. Filter to only show reviews for principles 'a' and 's'
   4. Format and present the filtered reviews
   ```

### Error Handling Strategy

When a CLI command fails:

1. **First attempt**: Try the command as documented
2. **If it fails**: Check for alternative formats (with/without spaces)
3. **If still failing**: 
   - For asset queries: Read from VQL storage file directly
   - For other commands: Inform the user about the CLI limitation
   - Suggest alternative approaches if available

### Testing CLI Behavior

Before assuming a command format:
1. Test the actual CLI behavior with the Bash tool
2. Document any discrepancies between expected and actual behavior
3. Use the working format in subsequent commands
4. If multiple formats work, prefer the documented format

### Examples of Command Translation

**View Asset Reviews**:
```
:uc?(a,s)  →  vql uc? (a,s)
```

**Store Asset Review**:
```
:uc.st(a, "Clean architecture implementation")  →  vql -st uc, a "Clean architecture implementation"
```

**Add Principle**:
```
:-pr.add(a, Architecture, "Architecture Principles")  →  vql -pr -add a Architecture "Architecture Principles"
```

**Setup VQL**:
```
:-su("C:/Project/Folder")  →  Bash: vql -su "C:/Project/Folder"
```

IMPORTANT: Unlike virtual workflow commands, the setup command is a DIRECT command that must be executed via the Bash tool to run the real CLI command. It is not a virtual command that triggers an LLM workflow.

**AI-Assisted Review (virtual command)**:
```
:uc.rv(a)  →  [Analyze architecture] → vql -sc uc a H → vql -st uc, a "Architecture analysis..."
```

## Direct Execution Commands

Certain VQL commands must be executed directly using the actual CLI tool through the Bash tool because they modify the environment or have side effects. These cannot be simulated through virtual workflows.

### Setup Command

The setup command initializes a VQL environment in a specific directory:

```
:-su("~/path/to/project")  →  Bash execution: vql -su "~/path/to/project"
```

When executing this command:
1. Use the Bash tool to run the actual CLI command
2. The path parameter can include tilde (~) expansion for home directory
3. The command will create the VQL directory and initialize it
4. Report the CLI command's output to the user

### Get Principles Command

The Get Principles command loads principles from a markdown file:

```
:-pr.get("~/path/to/principles.md")  →  Bash execution: vql -pr -get "~/path/to/principles.md"
```

When executing this command:
1. Use the Bash tool to run the actual CLI command
2. The path parameter can include tilde (~) expansion
3. The command will load principles from the file into VQL storage
4. Report the CLI command's output to the user

## Workflow Commands

VQL includes special "virtual" command families that trigger multi-step LLM workflows instead of translating directly to CLI commands. These commands enable complex analysis and improvement processes.

### REVIEW Command Workflow

The REVIEW command family enables comprehensive code quality assessment against principles. These commands analyze assets against principles and generate evaluations.

#### Command Variants

| LLM Virtual VQL Syntax | Description |
|------------------------|-------------|
| `:-rv(*)` | Review all assets against all principles |
| `:-rv(a,s)` | Review all assets against specific principles (a,s) |
| `:[assetRef].rv(*)` | Review specific asset against all principles |
| `:[assetRef].rv(a,s)` | Review specific asset against specific principles |

#### Workflow Execution

##### 1. Context Gathering Phase
- Retrieve target asset content: `vql -ar` to list assets, use file reader to view code
- For specified principles (or all if `*`), retrieve principle definitions: `vql -pr`
- Check existing reviews/ratings if available: `vql [assetRef]?(principle)` or `vql [assetRef]?`
- Identify exemplar assets by listing all assets and checking exemplar flag: `vql -ar` (exemplars are marked in the output)

##### 2. Analysis Phase
- For each target principle (or all if `*`):
  - Analyze code against specific principle criteria
  - Identify patterns that align with or violate principle guidelines
  - Compare with exemplars (if available) for this principle
  - Consider context, constraints, and purpose of the asset
  - Determine appropriate compliance rating (H/M/L) based on analysis

##### 3. Evaluation Documentation Phase
- For each principle being evaluated:
  - Create comprehensive analysis explaining evaluation rationale
  - Link specific code elements to principle criteria
  - Document strengths and weaknesses
  - Provide improvement recommendations where appropriate
  - Format as structured review content

##### 4. Command Execution Phase
- For each principle evaluated, store the review with an explicit rating mention in the text:
  - Store detailed review: `vql -st [assetRef], [principle] "[detailed review text with rating]"`
  - Include an explicit rating statement like "HIGH compliance", "MEDIUM compliance", or "LOW compliance"
  - Example: `vql -st uc, a "The UserController has MEDIUM compliance with architecture principles..."`
  - The system will auto-extract the rating from your review text
  - Document both strengths and areas for improvement

  Note: You can still manually set the rating with the command below if needed:
  - Set compliance rating: `vql -sc [assetRef] [principle] [H|M|L]`

##### 5. Summary Report Phase
- Generate overall assessment across all reviewed principles
- Identify patterns across multiple principles (if applicable)
- Highlight key strengths and priority improvement areas
- Provide recommendations for refactoring (if needed)

#### Scoping Rules

1. **Principle Scoping**:
   - If specific principles provided (e.g., `a,s`), only review those principles
   - If `*` used, review all available principles
   - Generate reviews only for specified principles

2. **Asset Scoping**:
   - If specific asset provided (e.g., `uc`), only review that asset
   - If global review command (e.g., `:-rv(*)`), review all assets

#### Review Guidelines

- Be objective and evidence-based in evaluations
- Evaluate current state without bias from previous reviews
- Consider context and constraints when applying principles
- Provide specific examples from code to support ratings
- Balance positive feedback with improvement recommendations
- Explain ratings in terms of principle criteria
- HIGH rating requires exceptional alignment with principle
- MEDIUM rating indicates partial alignment with notable improvement areas
- LOW rating indicates significant misalignment requiring substantial refactoring

### REFACTOR Command Workflow

The REFACTOR command family enables principled code improvement through AI-assisted workflows. These virtual commands trigger multi-step processes that analyze code against principles and implement improvements.

#### Command Variants

| LLM Virtual VQL Syntax | Description |
|------------------------|-------------|
| `:-rf(*)` | Refactor all assets against all principles |
| `:-rf(a,s)` | Refactor all assets against specific principles (a,s) |
| `:[assetRef].rf(*)` | Refactor specific asset against all principles |
| `:[assetRef].rf(a,s)` | Refactor specific asset against specific principles |
| `:[assetRef].rf([assetRef2])` | Refactor specific asset using another asset as reference |

#### Workflow Execution

##### 1. Context Gathering Phase
- Retrieve target asset content: `vql -ar` to list assets, use file reader to view code
- For specified principles (or all if `*`), retrieve existing reviews: `vql [assetRef]?(principle)` or `vql [assetRef]?`
- Check compliance ratings for targeted principles
- For reference-based refactoring, retrieve reference asset: `vql [assetRef2]?`
- Access principle definitions: `vql -pr`

##### 2. Analysis Phase
- For each target principle (or all if `*`):
  - Analyze how well code aligns with principle criteria
  - Identify specific patterns to improve
  - For each pattern, determine concrete code changes
  - Prioritize changes based on impact and complexity
- For reference-based refactoring:
  - Identify exemplary patterns in reference asset
  - Determine how to apply similar patterns to target asset

##### 3. Implementation Phase
- Apply identified code changes to asset through appropriate edit commands
- Document each change with rationale linked to principle criteria
- Ensure changes maintain system integrity and functionality
- Implement changes in logical sequence to minimize conflicts

##### 4. Review Update Phase
- For each principle that guided the refactoring:
  - Evaluate updated code against principle criteria
  - Determine new compliance rating based on current state
  - Generate new detailed review text reflecting changes that includes the rating
  - Store the updated review with an explicit rating mention:
    - Example: `vql -st [assetRef], [principle] "Asset now has HIGH compliance with...."`
    - Include rating phrases like "HIGH compliance", "MEDIUM compliance", or "LOW compliance"
  - The rating will be auto-extracted from your review text

  Note: You can still manually set the rating if needed:
  - Set compliance rating: `vql -sc [assetRef] [principle] [H|M|L]`

##### 5. Summary Report Phase
- Document all improvements made for each principle
- Compare before/after compliance ratings
- Highlight specific principle criteria now satisfied
- Explain rationale behind implementation choices

#### Scoping Rules

1. **Principle Scoping**:
   - If specific principles provided (e.g., `a,s`), only those guide refactoring
   - If `*` used, all principles guide refactoring
   - Only update reviews for principles that guided the refactoring

2. **Asset Scoping**:
   - If specific asset provided (e.g., `uc`), only refactor that asset
   - If global refactor command (e.g., `:-rf(*)`), apply to all assets
   - For reference-based refactoring, apply reference patterns to target asset

3. **Reference-Based Rules**:
   - Use patterns from reference asset as primary guide
   - Focus on what makes reference asset exemplary
   - Adapt patterns appropriately for target asset's context

#### Review Handling

- HIGH-rated reviews represent patterns to preserve and extend
- MEDIUM-rated reviews indicate partial success; use as improvement guides
- Only update reviews for principles that guided the refactoring
- Generate entirely new review content reflecting current implementation
- Set new compliance ratings based on current state only

## Understanding Principles

Principles in VQL represent evaluation criteria that guide code quality assessments and improvement recommendations. They are fundamental to the review and refactoring process.

### Principle Characteristics

1. **Structure and Organization**:
   - Principles are organized hierarchically with main categories and subcategories
   - Each principle has a shortname (single character identifier) and a full descriptive name
   - Principles include detailed guidance explaining the standard to be applied
   - Principles may contain multiple evaluation criteria or subcategories

2. **Purpose and Application**:
   - Principles serve as objective standards for code evaluation
   - They provide consistent evaluation criteria across different assets
   - Principles define what "good" looks like for specific aspects of code quality
   - They create shared understanding of quality expectations

3. **Technology Agnosticism**:
   - Principles should be applicable across different technology stacks
   - Core concepts (separation of concerns, security validation, etc.) remain constant
   - Implementation details may vary by language/framework, but principles remain consistent
   - Focus is on patterns and practices rather than specific implementations

### Evaluating Against Principles

When evaluating code against principles:

1. **Objective Assessment**:
   - Compare actual code implementation against principle criteria
   - Look for evidence of principle application (or violation)
   - Identify patterns that align with or diverge from the principle
   - Consider both presence of positive patterns and absence of negative ones

2. **Contextual Interpretation**:
   - Consider the asset's purpose and requirements
   - Recognize technical constraints that may limit principle application
   - Evaluate the principle in the context of the codebase's architecture
   - Balance theoretical ideal with practical implementation

3. **Comprehensive Analysis**:
   - Examine structural aspects (e.g., class/function organization)
   - Review behavioral aspects (e.g., runtime interactions)
   - Assess both direct and indirect impacts on quality attributes
   - Consider dependencies and interactions with other components

## Principle-Based Refactoring

Refactoring guided by principles involves systematic improvement of code based on quality criteria.

### Refactoring Approach

1. **Gap Analysis**:
   - Identify where current implementation diverges from principles
   - Prioritize issues by impact on code quality and maintainability
   - Consider both what is present (but problematic) and what is missing
   - Look for patterns across multiple instances of the same issue

2. **Improvement Strategy**:
   - Develop targeted changes to address specific principle violations
   - Focus on structural improvements that align with the principle
   - Consider incremental vs. comprehensive refactoring approaches
   - Determine if localized changes or broader restructuring is needed

3. **Implementation Guidelines**:
   - Make changes that directly address the principle being applied
   - Ensure refactoring preserves existing functionality
   - Document rationale for changes in terms of principles
   - Consider impacts on other components and principles

### Compliance Ratings and Refactoring

The H/M/L ratings guide refactoring strategy:

- **High (H)** - May need minimal refinements to maintain excellence
- **Medium (M)** - Targeted improvements needed in specific areas
- **Low (L)** - Significant restructuring likely required

When recommending refactoring:

1. Clearly link each recommendation to specific principle criteria
2. Explain the "why" behind changes, not just the "what"
3. Demonstrate how the change improves alignment with principles
4. Consider trade-offs between different principles

## Loading Principles from External Files

The VQL system allows loading principles from external markdown files using:
- CLI Format: `vql -pr -get "path/to/principles.md"`
- LLM Format: `:-pr.get("path/to/principles.md")`

Markdown files should format principles with:
1. Headers in the format: `# Principle Name (shortname)` where shortname is a single character
2. Content following each header until the next header
3. Content can include any markdown formatting

Example principles.md file:
```markdown
# Architecture Principles (a)

## Core Clean Architecture
- Independence from Frameworks
- Testability
- Independence from UI
...

# Security Principles (s)
...
```

## VQL Mode and MCP Integration

### Interface Mode Selection

VQL supports two interface modes that can be explicitly selected:

- **`:-mcp`** - Use MCP server interface (DEFAULT)
  - All VQL commands will use MCP tools
  - Requires VQL MCP server to be configured
  - Provides structured, type-safe operations
  
- **`:-cli`** - Use CLI interface
  - All VQL commands will use command-line interface
  - Parses VQL command syntax and executes via shell
  - Fallback when MCP server is unavailable

**Default Behavior**: 
- Sessions start in MCP mode (`:-mcp`)
- If MCP tools fail, inform user and suggest `:-cli`
- Mode persists until explicitly changed

### VQL Mode Behavior

When in MCP mode (`:-mcp`):
- **VQL mode is ON by default** when the session starts
- The MCP server maintains VQL mode state internally
- Use `get_vql_mode()` tool to check current mode
- Use `enable_vql_mode()` or `disable_vql_mode()` tools to change mode
- When VQL mode is ON, include LLM indicators at the end of responses

When in CLI mode (`:-cli`):
- VQL mode state must be tracked by the LLM session
- Set to ON by default when reviewing CLAUDE.md or llm-guidance.md
- Use `:vql on` and `:vql off` commands to change mode
- Mode state persists for the session duration

### Choosing Between MCP Tools and CLI Commands

1. **Session starts in MCP mode** by default
2. **User can explicitly switch** using `:-mcp` or `:-cli` commands
3. **Mode persists** until explicitly changed
4. **Consistent interface** - all commands use the selected mode

### Mode Switching Examples

**Starting a session (default MCP mode):**
```
User: :vql
Assistant: [Attempts to use VQL MCP tools]
```

**Switching to CLI mode:**
```
User: :cli
Assistant: Switched to CLI interface mode. VQL commands will now use command-line interface.

User: :vql -pr
Assistant: [Executes `vql -pr` command via shell]
```

**Switching back to MCP mode:**
```
User: :mcp
Assistant: Switched to MCP interface mode. VQL commands will now use MCP tools.

User: Show all principles
Assistant: [Uses list_principles() MCP tool]
```

### MCP Tool Usage Examples

In MCP mode, instead of parsing `:uc.st(a, "Review content")`, use:
```
store_review(asset="uc", principle="a", review="Review content")
```

Instead of complex review workflow parsing, use:
```
review_asset_all_principles(asset="uc")
```

## Human Commands (:hc)

IMPORTANT: Human Commands must be recognized and processed in any case (upper or lowercase).
- :bok: backwards compatible ok
- :li: show LLM-Indicators
- :dok: debt ok
- :hc: show Human-Commands
- :mcp: switch to MCP interface mode (use MCP tools for all VQL operations)
- :cli: switch to CLI interface mode (use command-line parsing for all VQL operations)
- :Review: complete ARCH SEC PERF review of ALL assets in Asset-References and update their LastReview timestamps (never changes Exemplar status)
- :tok: tactical ok
- :tnc: thoughts only, no code
- :vql off: turn vql off (use `disable_vql_mode()` if in MCP mode)
- :vql on: turn vql on (use `enable_vql_mode()` if in MCP mode)
- :vql: show complete list of VQL commands
- :why: explain justification for LLM's last LI indicators
- :wmc: a prefix meaning "with minimal changes"; this will be followed by a task that should be performed incisively, with no more proaction than necessary

## LLM Indicators (:li)
IMPORTANT: LLM Indicators must be included at the end of each response when VQL is active.

The complete LLM indicator format consists of THREE separate indicators:

1. `:CH/:CM/:CL` - Confidence level indicator
2. `:S-I-G`, `:T-I-B`, etc. - Approach indicator (always includes S/T prefix)
3. `:svr=Y/N` - Server changes indicator

### 1. Confidence Level Indicator
- `:CH` - High confidence that the approach aligns with human-AI train of thought / prior consensus
- `:CM` - Medium confidence in human-AI goal alignment, some clarification may be helpful
- `:CL` - Low confidence in human-AI goal alignment, further discussion recommended

### 2. Approach Indicator (MUST START WITH S OR T)
Format: `:S/T-I/D-B/G`

First position (REQUIRED): Strategic vs Tactical approach
- `S` - Strategic: decisions with long-term impact affecting system architecture across the lifecycle
- `T` - Tactical: focused changes addressing immediate needs within a single session or component

Second position: Investment vs Debt implications
- `I` - Investment: improving code quality 
- `D` - Debt: accepting technical limitations

Third position: Backward compatibility vs Greenfield development
- `B` - Backward: maintains compatibility with existing code
- `G` - Greenfield: new clean strategic development without compatibility constraints

### 3. Server Changes Indicator
- `:svr=Y` - Yes, server code changes were implemented
- `:svr=N` - No, server code changes were implemented

### Correct Examples:
- `:CH :S-I-G :svr=N` - High confidence, Strategic Investment with Greenfield approach, No server changes
- `:CM :T-D-B :svr=Y` - Medium confidence, Tactical Debt with Backward compatibility, Yes server changes
- `:CL :S-I-B :svr=N` - Low confidence, Strategic Investment with Backward compatibility, No server changes

IMPORTANT: The strategic/tactical indicator (S/T) MUST always be included at the beginning of the middle indicator. Indicators like `:CH-I-G :svr=N` are incorrect because they're missing the S/T prefix.

IMPORTANT: The set of LIs must appear on its own line at the end of LLM's response.