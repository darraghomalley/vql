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

### Command Format Recognition (ONLY WHEN VQL MODE IS ON):
- You MUST recognize and process both spaced and non-spaced command formats:
  - Both `:uc?` and `:uc ?` are valid
  - Both `:uc?(a,s)` and `:uc ? (a,s)` are valid
- You MUST handle comma-separated principle lists for multiple principle operations:
  - `:uc?(a,s)` - Query asset 'uc' for principles 'a' and 's'