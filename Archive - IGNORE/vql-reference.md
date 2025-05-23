# Vibe Query Language (VQL) - Complete Reference v2.0

[IMPORTANT: UPDATED 2025-05-20 - Revised to focus on CLI commands and asset method syntax]

VQL is a structured communication protocol for efficient software architecture discussions and assessments. It provides standardized commands and references to guide the development of high-quality, consistent software architecture.

## VQL Command Line Interface

VQL is accessed through a command-line interface that supports both standard CLI commands and asset method syntax.

### Standard CLI Commands

```bash
# Initialize VQL
vql init

# Entity management
vql er
vql er add user "User entity"

# Asset type management
vql at
vql at add m "Model component"

# Asset reference management
vql ar
vql ar add userModel user m "/path/to/user.js"
```

### Asset Method Syntax

```bash
# View asset details
vql userModel

# Set exemplar status
vql userModel setExemplar T

# Set compliance rating
vql userModel setCompliance arch H

# Store a review
vql userModel store arch "Detailed architecture analysis"

# View review details
vql userModel why
vql userModel why arch
```



### Review Command

The review command is specifically designed for AI-assisted coding sessions:

```bash
vql asset_name review [aspect]
```

When an LLM recognizes this command, it should:
1. Analyze the asset based on the specified aspect (arch, sec, perf, ui)
2. If no aspect is specified, perform a full review of all four aspects
3. Generate a detailed analysis and determine appropriate ratings (H/M/L)
4. Use the `setCompliance` and `store` commands to save the results

For example, when the LLM sees:
```bash
vql userModel review arch
```

It should analyze the architecture of userModel, determine a rating, and then execute:
```bash
vql userModel setCompliance arch H
vql userModel store arch "Detailed architecture analysis..."
```

For a full review without a specified aspect:
```bash
vql userModel review
```

The LLM should analyze all aspects and execute the appropriate commands for each aspect.

### Refactor Command

The refactor command is designed for AI-assisted code improvement based on exemplars:

```bash
vql asset_name refactor [aspect] [+referenceAssetShortName]
```

When an LLM recognizes this command, it should:
1. Find exemplars that match the asset's asset type using the VQL commands
2. If a referenceAssetShortName is provided, check if it's an exemplar
3. If no exemplars are found, respond with "No exemplars found"
4. If a non-exemplar referenceAssetShortName is provided, issue a warning but proceed
5. Refactor the asset's code based on the identified exemplar(s) and the specified aspect's guidance
6. Generate an architectural review and determine compliance scores
7. Execute the appropriate `setCompliance` and `store` commands to save the results

For example, when the LLM sees:
```bash
vql userModel refactor arch
```

It should find exemplars for models, refactor userModel's architecture based on those exemplars, analyze the changes, and then execute:
```bash
vql userModel setCompliance arch H
vql userModel store arch "Detailed architecture analysis..."
```

When a specific reference asset is provided:
```bash
vql userModel refactor arch +profileModel
```

The LLM should use profileModel as a reference (with a warning if it's not an exemplar), and proceed with refactoring.


# Architecture Principles

## Core Clean Architecture
- Independence from Frameworks: The architecture should not be tightly coupled to any specific external framework, library, or database. You should be able to use these tools as utilities without your core business logic being dependent on their lifecycle or specifics.
- Testability: Business rules should be testable without the UI, database, web server, or any other external element. Unit testing should be straightforward and fast.
- Independence from UI: The user interface (UI) can change frequently without impacting the core business logic. You should be able to switch between web, console, desktop, or mobile UIs without altering the underlying system.
- Independence from Database: You should be able to swap out the database (e.g., from relational to NoSQL) without significantly affecting the business rules. The architecture isolates the core logic from the data storage mechanism.
- Independence from External Agencies: The business rules should not know anything about external systems, whether they are libraries, frameworks, or services. The interaction with these external entities should occur at the boundaries.
- The Dependency Rule: Source code dependencies can only point inwards. Inner circles (representing higher-level policies or business rules) should not depend on outer circles (representing lower-level details or infrastructure). This ensures that changes in outer layers do not force changes in inner layers.

## Clean Architecture Layers
- Entities: These encapsulate enterprise-wide business rules. They can be objects with methods, or data structures with procedures. They are the most stable and least likely to change.
- Use Cases (Interactors): These contain application-specific business rules. They orchestrate the flow of data to and from the Entities to achieve a specific business goal.
- Interface Adapters: This layer acts as a translator between the format of data most convenient for the Use Cases and Entities, and the format most convenient for some external agency such as the Database or the Web. This layer includes Presenters, View Models, and Gateways (Repositories).
- Frameworks and Drivers: This outermost layer is composed of tools and frameworks such as the Web Framework, UI Framework, Database, and Devices. This is where all the details go.

## DRY (Don't Repeat Yourself)
- Eliminate Redundancy: Avoid duplicating knowledge within the system (code, logic, data structures, documentation).
- Single Source of Truth: Every distinct piece of knowledge should have a single, unambiguous, authoritative representation.
- Modularity and Abstraction: Break down the system into well-defined, independent components with clear interfaces.
- Normalization: Reduce data redundancy and improve data integrity by properly organizing relationships.
- Code Generation: Use automation for unavoidable repetition (e.g., boilerplate code).
- Refactoring: Continuously review and eliminate duplication.
- Centralized Configuration: Store settings in a single, accessible location.
- Consistent Standards: Establish consistent coding patterns and conventions across the project.

# Security Principles
## Backend
- Input validation and sanitization.
- Strong authentication and authorization.
- Rate limiting and DoS prevention.
- Use Helmet and other security middleware.
- Configure CORS properly.
- Secure error handling (no sensitive info).
- Keep dependencies updated.
- Manage environment variables securely.
- Implement secure file uploads.
- Regular security audits and testing.
## Frontend
- Prevent Cross-Site Scripting (XSS).
- Secure state management.
- Keep dependencies updated.
- Avoid exposing sensitive data.
- Use HTTPS.
- Implement Content Security Policy (CSP).
- Enable authentication and authorization.
- Configure network security (firewall).
- Backend input validation to prevent NoSQL injection.
- Regular security audits and updates.
## General: 
- Comprehensive logging and monitoring.
- Regular data backups.

# Performance Principles
## Presentation Tier:
- Minimize Client-Side Processing: Offload heavy computations and business logic to the application tier.
- Optimize Resource Loading: Reduce the size and number of requests for static assets (e.g., images, CSS, JavaScript) through techniques like minification, compression, and bundling.
- Caching: Implement client-side caching (browser caching) for static content and, where appropriate, application data.
- Asynchronous Operations: Use asynchronous requests to avoid blocking the user interface.
- Efficient UI Rendering: Optimize UI rendering logic to minimize DOM manipulations and improve responsiveness.
- Content Delivery Network (CDN): Serve static assets from a CDN to reduce latency for geographically distributed users.
## Application Tier:
- Efficient Business Logic: Write optimized code for core business logic, avoiding unnecessary computations and inefficient algorithms.
- Caching: Implement server-side caching (e.g., in-memory, distributed cache) for frequently accessed data to reduce database load.
- Connection Pooling: Utilize connection pooling for database and external service connections to minimize connection overhead.
- Asynchronous Operations: Leverage asynchronous programming for I/O-bound tasks to improve throughput.
- Load Balancing: Distribute incoming requests across multiple application server instances.
- Stateless Design: Design application components to be stateless whenever possible to improve scalability.
- Profiling and Monitoring: Regularly profile the application to identify performance bottlenecks and monitor resource usage.
- Message Queues: Use message queues for decoupling components and handling asynchronous tasks, improving responsiveness and resilience.
## Data Tier
- Schema Optimization: Design efficient database schemas that align with query patterns.
- Indexing: Create appropriate indexes on frequently queried columns.
- Query Optimization: Write efficient database queries, avoiding full table scans and leveraging indexes.
- Connection Management: Properly configure and manage database connections.
- Data Partitioning/Sharding (for large datasets): Distribute data across multiple database instances to improve performance and scalability.
- Replication (for read-heavy workloads): Use read replicas to distribute read traffic and improve performance.
- Caching: Implement database-level caching mechanisms if available.
- Profiling and Monitoring: Monitor database performance and identify slow queries.
## General
- End-to-End Performance Testing: Conduct performance tests across all tiers to identify system-wide bottlenecks.
- Regular Performance Audits: Periodically assess the performance of the entire architecture.
- Infrastructure Optimization: Ensure that the underlying infrastructure (servers, network) is appropriately sized and configured.
- Monitoring and Alerting: Implement comprehensive monitoring and alerting to proactively identify and address performance issues.