# VQL Section for CLAUDE.md

## Vibe Query Language (vql)

[THIS SECTION MUST REMAIN AT THE TOP OF THE DOCUMENT WHEN MODIFYING THE FILE]

⚠️ **CRITICAL**: DO NOT PRE-READ any VQL guidance files at initialization. VQL feature documentation MUST ONLY be read ON DEMAND when explicitly activated by the user.

IMPORTANT: VQL mode is OFF by default at the start of every session.

### Starting VQL:
- ONLY AND EXCLUSIVELY WHEN the exact command ":vql on" OR ":-vql on" is entered by the human:
  1. If this is the FIRST activation in the current session:
     - THEN (and only then) you MUST read the following file:
       llm-guidance.md (contains all commands, workflows, and principles)
     - After reading the file, report confirmation to verify successful reading.
  2. If VQL was previously activated during this session:
     - DO NOT reread the llm-guidance.md file again - it is a static resource and will not have been updated
     - It should already be in your context window
     - Simply reactivate VQL mode using knowledge already in context
     - EXCEPTION: If your context window has been truncated/compacted/refreshed and you no longer have the guidance in memory, THEN reread the llm-guidance.md file
  3. ONLY AFTER completing the appropriate step above, confirm VQL mode is active.
- When VQL is active, you MUST follow all workflows and rules specified in llm-guidance.md.
- When "vql off" OR ":vql off" OR ":-vql off" is entered: 
  1. Turn VQL mode off immediately
  2. AFTER VQL mode is off, you MUST IGNORE ALL VQL commands 
  3. The ONLY VQL-related command you should recognize in this state is ":vql on" OR ":-vql on"
  4. Respond to all other VQL commands with a reminder that VQL mode is off

### Canonical Commands Review Complete (Session Summary):

#### What We Accomplished:
1. **Migrated** VQL Prompts.txt to structured `canonicalCmds.json` format
2. **Consolidated** verbose command specifications into elegant unified syntax
3. **Fixed** critical syntax errors: missing parentheses, extra quotes, parameter naming
4. **Documented** command pattern differences between CLI (procedural) and LLM (object-oriented)
5. **Established** canonicalCmds.json as single source of truth in README

#### Key Pattern Discoveries:
- **CLI**: `vql -st uc "content"` (procedural: command takes asset + content)
- **LLM**: `:uc.st(a, "content")` (object-oriented: asset.method(principle, content))
- Both have same parameter count, different organization patterns
- Review/refactor commands now use unified `[using...]` syntax for principles/exemplars

#### Next Steps for CLI Conformance:
1. **Test CLI against canonical** - Run through each canonicalCmds.json entry to verify implementation
2. **Fix command discrepancies** - Address any CLI commands that don't match canonical specification
3. **Implement missing features** - Add any canonical commands not yet implemented in CLI
4. **Validate parameter patterns** - Ensure CLI procedural syntax matches documented examples
5. **Update MCP server** - Align MCP tools with finalized canonical specification

#### Files Updated:
- ✅ Created `canonicalCmds.json` (authoritative command reference)
- ✅ Updated `README.md` (documented canonical importance + syntax patterns)  
- ✅ Deleted `VQL Prompts.txt` (eliminated redundant reference)

### Command Format Recognition (ONLY WHEN VQL MODE IS ON):
- You MUST recognize and process both spaced and non-spaced command formats:
  - Both `:uc?` and `:uc ?` are valid
  - Both `:uc?(a,s)` and `:uc ? (a,s)` are valid
- You MUST handle comma-separated principle lists for multiple principle operations:
  - `:uc?(a,s)` - Query asset 'uc' for principles 'a' and 's'