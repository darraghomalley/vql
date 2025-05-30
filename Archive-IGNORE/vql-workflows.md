# VQL Workflow Guide

This document contains detailed workflow instructions for using VQL commands. It is used as reference during AI-assisted coding sessions.

## Command Workflows

### Asset Types

VQL supports dynamic asset types. Each asset type:
1. Is identified by a single alphabetic character (e.g., 'm' for model, 'c' for controller)
2. Has its own storage entry in the JSON storage file
3. Can be created with the 'vql at add' command

When working with specific asset types, be aware of the asset type character at the end of the asset name (e.g., 'um' for User model).

### List/Display Commands

VQL provides simple commands to list all available entities, asset types, and asset references:

```bash
# List entities
vql er

# List asset types
vql at

# List asset references
vql ar
```

### Asset Method Commands

VQL offers a concise asset method syntax for working with specific assets:

```bash
vql asset_name method [args]
```

Available methods:
- Simply specifying the asset name shows its details: `vql um`
- `why [aspect]` - Show reviews (all aspects if none specified): `vql um why`
- `setExemplar T|F` - Set exemplar status: `vql um setExemplar T`
- `setCompliance aspect rating` - Set compliance: `vql um setCompliance arch H`
- `store aspect "analysis" [rating]` - Store a review: `vql um store arch "Detailed analysis" M`

### AI-Assisted Review Command Workflow

When the LLM encounters a review command in this format:

```bash
vql asset_name review [aspect]
```

It should perform the following workflow:

1. **For a Single-Aspect Review** (when aspect is specified):

   a. Analyze the asset's code for the specified aspect (arch, sec, perf, or ui)
   b. Generate a detailed analysis following best practices for that aspect
   c. Determine an appropriate rating (H/M/L) based on the analysis
   d. Execute the actual CLI commands to save the results:
   ```bash
   vql asset_name setCompliance aspect rating
   vql asset_name store aspect "Detailed analysis text" rating
   ```
   e. Provide a summary of the review to the user

2. **For a Full Comprehensive Review** (when no aspect is specified):

   a. Analyze the asset's code for ALL four aspects:
      - Architecture (arch)
      - Security (sec)
      - Performance (perf)
      - UI (ui)
   b. For each aspect, generate a detailed analysis and determine a rating
   c. Execute the actual CLI commands to save ALL four aspects:
   ```bash
   vql asset_name setCompliance arch rating_arch
   vql asset_name store arch "Architecture analysis text" rating_arch
   
   vql asset_name setCompliance sec rating_sec
   vql asset_name store sec "Security analysis text" rating_sec
   
   vql asset_name setCompliance perf rating_perf
   vql asset_name store perf "Performance analysis text" rating_perf
   
   vql asset_name setCompliance ui rating_ui
   vql asset_name store ui "UI analysis text" rating_ui
   ```
   d. Provide a summary of the complete review to the user

3. **Review Command Examples**:

   For a single aspect review:
   ```bash
   vql userModel review arch
   ```
   
   For a full comprehensive review:
   ```bash
   vql userModel review
   ```

IMPORTANT: The review command itself doesn't exist in the actual CLI - it's a shorthand instruction for the LLM to perform the analysis and then use the real CLI commands (`setCompliance` and `store`) to save the results.

### AI-Assisted Refactor Command Workflow

When the LLM encounters a refactor command in this format:

```bash
vql asset_name refactor [aspect] [+referenceAssetShortName]
```

It should perform the following workflow:

1. **Find appropriate exemplars:**
   
   a. Determine the asset's type by examining its name or information
   b. Find exemplars that match this asset type using VQL commands
   c. If a referenceAssetShortName is provided with the + prefix, check if it exists and is an exemplar
   d. If no exemplars are found, respond with "No exemplars found"
   e. If a non-exemplar reference is provided, issue a warning but continue using it as a template

2. **For a specific aspect refactor** (when aspect is specified):

   a. Analyze the asset's code against the exemplar(s) focusing on the specified aspect (arch, sec, perf, or ui)
   b. Refactor the code to align with the exemplar's patterns and the best practices for that aspect
   c. Generate a detailed analysis of the changes made and their benefits
   d. Determine an appropriate compliance rating (H/M/L) based on the refactored code
   e. Execute the actual CLI commands to save the results:
   ```bash
   vql asset_name setCompliance aspect rating
   vql asset_name store aspect "Detailed analysis of refactoring..." rating
   ```
   f. Provide a summary of the refactoring and improvements to the user

3. **For a comprehensive refactor** (when no aspect is specified):

   a. Analyze and refactor the asset's code for ALL four aspects, prioritizing in this order:
      - Architecture (arch)
      - Security (sec)
      - Performance (perf)
      - UI (ui)
   b. For each aspect, refactor the code based on exemplars and best practices
   c. Generate detailed analyses and determine ratings for each aspect
   d. Execute the actual CLI commands to save ALL four aspects:
   ```bash
   vql asset_name setCompliance arch rating_arch
   vql asset_name store arch "Architecture refactoring analysis..." rating_arch
   
   vql asset_name setCompliance sec rating_sec
   vql asset_name store sec "Security refactoring analysis..." rating_sec
   
   vql asset_name setCompliance perf rating_perf
   vql asset_name store perf "Performance refactoring analysis..." rating_perf
   
   vql asset_name setCompliance ui rating_ui
   vql asset_name store ui "UI refactoring analysis..." rating_ui
   ```
   e. Provide a comprehensive summary of all refactoring changes to the user

4. **Refactor Command Examples**:

   For a single aspect refactor using available exemplars:
   ```bash
   vql userModel refactor arch
   ```
   
   For a single aspect refactor using a specific reference asset:
   ```bash
   vql userModel refactor sec +profileModel
   ```
   
   For a full comprehensive refactor:
   ```bash
   vql userModel refactor
   ```

IMPORTANT: Like the review command, the refactor command doesn't exist in the actual CLI - it's a shorthand instruction for the LLM to perform the refactoring analysis and then use the real CLI commands (`setCompliance` and `store`) to save the results.

### Standard Review Workflow

When manually reviewing an asset without using the AI-assisted review command:

1. Use the `why` command to see current ratings and analysis:
   ```bash
   vql um why
   ```

2. Store a review for each aspect you want to analyze:
   ```bash
   vql um store arch "The architecture follows clean architecture principles with clear separation of concerns." H
   vql um store sec "Properly validates input and implements authorization checks." M
   vql um store perf "Uses efficient database queries but lacks caching." M
   vql um store ui "Maintains consistent UI patterns with good error feedback." H
   ```

3. Confirm to the user that the review is complete

### Exemplar Setting Workflow

When setting an asset as an exemplar:

1. First evaluate if the asset meets exemplar criteria (if setting to T)

2. Set the exemplar status:
   ```bash
   vql um setExemplar T
   ```

3. Explain to the user why the asset is a good exemplar (or why it's no longer one)

### Entity and Asset Type Management

To add new entities, asset types, and asset references:

1. Add a new entity:
   ```bash
   vql er add user "User entity"
   ```

2. Add a new asset type (single character):
   ```bash
   vql at add s "Service component"
   ```

3. Add a new asset reference:
   ```bash
   vql ar add userService user s "/services/user.js"
   ```

## Architecture Assessment Guidelines

When performing reviews, consider these key areas:

### Architecture Assessment
- Clean architecture principles
- Dependency direction correctness
- Separation of concerns
- DRY (Don't Repeat Yourself) principles
- Appropriate use of design patterns
- Domain model integrity
- Testability
- Clear boundaries between layers

### Security Assessment
- Input validation and sanitization
- Authentication and authorization mechanisms
- Rate limiting and DoS prevention
- Secure error handling
- Data sensitivity handling
- Use of security middleware
- Dependency vulnerabilities
- Environment variable management

### Performance Assessment
- Algorithm efficiency
- Resource utilization
- Caching strategies
- Connection handling
- Asynchronous operations
- Load distribution approaches
- Data access patterns
- Client-side optimizations

### UI Assessment
- Consistency with design patterns
- Accessibility compliance
- Responsive design implementation
- Error message clarity
- Navigation intuitiveness
- Form validation feedback
- Visual hierarchy
- Layout efficiency and responsiveness

## General VQL Mode Behavior

When VQL mode is active during AI-assisted coding:

1. Include LLM indicators at the end of EVERY response
2. Automatically act on any VQL-related queries or commands
3. Maintain awareness of the asset references and their current state
4. Use the asset method syntax for working with specific assets

## Rating Scale Guidelines

When assigning H/M/L ratings to aspects:

### High (H) Rating
- Follows all relevant principles for the aspect
- Implements best practices throughout the code
- No significant issues or vulnerabilities
- Can serve as an example for other assets

### Medium (M) Rating
- Follows most relevant principles with minor deviations
- Implements many best practices but has room for improvement
- Minor issues that don't significantly impact functionality
- Acceptable quality but not exemplary

### Low (L) Rating
- Deviates from multiple relevant principles
- Missing important best practices
- Significant issues affecting functionality or security
- Requires substantial improvement

## Example Workflows

### Example 1: Manual Assessment Workflow

Example of a manual review workflow for a Model component:

1. First, check the current status:
   ```bash
   vql userModel why
   ```

2. Analyze the architecture:
   ```bash
   vql userModel store arch "The model implements proper data validation and provides clear interfaces for controllers. It maintains single responsibility principle and follows domain-driven design. The schema is well-organized but has some redundant methods." M
   ```

3. Review security aspects:
   ```bash
   vql userModel store sec "The model implements input validation and sanitization. It handles sensitive data appropriately by not exposing password fields. Authentication logic is properly encapsulated." H
   ```

4. Assess performance:
   ```bash
   vql userModel store perf "Database queries are optimized with appropriate indexes. The model implements some caching strategies but could benefit from more efficient data access patterns for large result sets." M
   ```

5. If it's a good example, set as exemplar:
   ```bash
   vql userModel setExemplar T
   ```

6. Verify the updated status:
   ```bash
   vql userModel why
   ```

### Example 2: AI-Assisted Review Workflow

When a user requests an AI-assisted review:

```bash
vql userModel review
```

The LLM should respond with a comprehensive analysis of all aspects:

1. Analyze each aspect (arch, sec, perf, ui) of the userModel
2. Generate detailed analyses and determine appropriate ratings
3. Execute the CLI commands to save the ratings and analyses:
   ```bash
   vql userModel setCompliance arch H
   vql userModel store arch "Detailed architecture analysis..."
   
   vql userModel setCompliance sec M
   vql userModel store sec "Detailed security analysis..."
   
   vql userModel setCompliance perf M
   vql userModel store perf "Detailed performance analysis..."
   
   vql userModel setCompliance ui H
   vql userModel store ui "Detailed UI analysis..."
   ```
4. Provide a summary of the review to the user

For a single-aspect review:

```bash
vql userModel review arch
```

The LLM would only perform steps 1-4 for the architecture aspect.

### Example 3: AI-Assisted Refactor Workflow

When a user requests an AI-assisted refactor:

```bash
vql userController refactor arch
```

The LLM should respond with:

1. Find exemplars for controller components:
   ```bash
   vql ar
   ```
   (Examine results to identify exemplar controllers marked with T under Exemplar column)

2. If exemplar controller found (e.g., profileController):
   a. Analyze userController against profileController's architecture patterns
   b. Refactor userController to align with the exemplar's architecture
   c. Generate detailed architectural analysis
   d. Execute commands to save the compliance rating and analysis:
   ```bash
   vql userController setCompliance arch H
   vql userController store arch "Refactored controller to follow clean architecture principles..."
   ```

3. If no exemplar found:
   ```
   No exemplars found for controller components. Refactoring cannot proceed without exemplars.
   ```

For a refactor with a specific reference asset:

```bash
vql userController refactor arch +adminController
```

The LLM would check if adminController is an exemplar, issue a warning if not, but still use it as a reference for refactoring.

Remember to focus on asset method syntax for all operations on specific assets.