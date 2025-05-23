# VQL Commands for Claude Code

This document describes how Claude should translate between VQL virtual commands and actual CLI commands.

## Command Formats

VQL supports two command formats:

1. **CLI Commands** - Used directly by humans in the terminal
   - Format: `vql -command -subcommand args`
   - Example: `vql -at -add c Controller`

2. **LLM Virtual Commands** - Used by Claude in conversations
   - Format: `:[asset].command(args)` or `:-command(args)`
   - Example: `:uc.rv(a)` or `:-pr`

## Command Format Flexibility

Both the CLI and LLM formats support flexible syntax:

1. **Space Flexibility**: Commands can be written with or without spaces between components:
   - Both `:uc?` and `:uc ?` are valid
   - Both `:uc?(a,s)` and `:uc ? (a,s)` are valid

2. **Multiple Perspectives**: Comma-separated lists can specify multiple perspectives:
   - `:uc?(a,s)` - Query asset 'uc' for perspectives 'a' and 's'
   - `:uc.rv(a,s,p)` - Review perspectives 'a', 's', and 'p' for asset 'uc'

## LLM Virtual Command Translation

When Claude encounters a VQL virtual command, it should translate it to the appropriate CLI command(s) according to the following rules:

### Perspective Commands

| LLM Command | CLI Command | Description |
|-------------|-------------|-------------|
| `:-pr` | `vql -pr` | List all perspectives |
| `:-pr.add(a, Architecture, "Guidelines")` | `vql -pr -add a Architecture "Guidelines"` | Add a perspective |

### Entity Commands

| LLM Command | CLI Command | Description |
|-------------|-------------|-------------|
| `:-er` | `vql -er` | List all entities |
| `:-er.add(u, User)` | `vql -er -add u User` | Add an entity |

### Asset Type Commands

| LLM Command | CLI Command | Description |
|-------------|-------------|-------------|
| `:-at` | `vql -at` | List all asset types |
| `:-at.add(c, Controller)` | `vql -at -add c Controller` | Add an asset type |

### Asset Reference Commands

| LLM Command | CLI Command | Description |
|-------------|-------------|-------------|
| `:-ar` | `vql -ar` | List all asset references |
| `:-ar.add(uc, "path/to/file.js")` | `vql -ar -add uc "path/to/file.js"` | Add an asset reference (inferred entity/type) |
| `:-ar.add(uc, u, c, "path/to/file.js")` | `vql -ar -add uc u c "path/to/file.js"` | Add asset reference (explicit) |

### Review Commands

| LLM Command | CLI Command | Description |
|-------------|-------------|-------------|
| `:uc?` or `:uc ?` | `vql uc?` or `vql uc ?` | Show all reviews for asset |
| `:uc?(a)` or `:uc ? (a)` | `vql uc?(a)` or `vql uc ? (a)` | Show architecture review |
| `:uc?(a,s)` or `:uc ? (a,s)` | `vql uc?(a,s)` or `vql uc ? (a,s)` | Show architecture and security reviews |
| `:uc.st(a, "Review text")` | `vql -st uc a "Review text"` | Store review |
| `:uc.se(t)` | `vql -se uc t` | Set exemplar status |
| `:uc.sc(a, H)` | `vql -sc uc a H` | Set compliance rating |

### Virtual Commands (LLM-only)

These commands don't have CLI equivalents. When Claude sees these, it should:
1. Analyze the asset according to the command
2. Execute appropriate real CLI commands based on the analysis

| Virtual Command | Description | Translation |
|-----------------|-------------|-------------|
| `:uc.rv(*)` | Review all perspectives | Analyze the asset and execute `-st` and `-sc` commands |
| `:uc.rv(a,s)` | Review architecture and security | Analyze the asset and execute commands for each perspective |
| `:-rv(*)` | Review all assets | Generate reviews for all assets |
| `:-rv(a,s)` | Review all assets with specific perspectives | Generate reviews for all assets with the specified perspectives |
| `:uc.rf(*)` | Refactor asset | Analyze exemplars and propose refactoring |
| `:uc.rf(a,s)` | Refactor specific aspects | Analyze exemplars and propose refactoring for specified aspects |
| `:-rf(*)` | Refactor all assets | Analyze exemplars and propose refactoring for all assets |
| `:-rf(a,s)` | Refactor all assets for specific aspects | Analyze exemplars and propose refactoring for all assets with specified aspects |
| `:uc.rf(uc2)` | Refactor based on specific asset | Use uc2 as a reference for refactoring uc |

## Translation Examples

When Claude sees:
```
:uc.rv(a)
```

It should:

1. Analyze the user controller's architecture
2. Generate an analysis text
3. Determine an appropriate rating (H/M/L)
4. Execute the actual CLI commands:
   ```
   vql -sc uc a H
   vql -st uc a "Architecture analysis..."
   ```

When Claude sees:
```
:uc?(a,s)
```

It should execute:
```
vql uc? (a,s)
```

This will show reviews for the "uc" asset for both architecture and security perspectives.

When Claude sees:
```
:-rv(*)
```

It should:

1. Find all assets in the system
2. For each asset and each perspective:
   - Analyze the asset
   - Execute appropriate `-sc` and `-st` commands

## Important Notes

1. The `review` (rv) and `refactor` (rf) commands are "virtual" - they indicate that Claude should perform analysis and then execute actual CLI commands.

2. Always maintain all parameters for the asset reference command when specified explicitly.

3. When storing reviews with `-st`, the analysis text should be concise but detailed, highlighting key aspects of what was analyzed.

4. Commands can be written with or without spaces, and perspective lists use comma separation.

5. The storage command is `-st` not `-str` (corrected from earlier versions).