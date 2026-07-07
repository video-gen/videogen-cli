---
name: videogen-custom-commands
description: How to author custom commands for the videogen CLI using the co-generated SDK.
---

# Custom Commands for `videogen`

## Overview

The `videogen` CLI supports user-authored custom commands that are
compiled into the binary alongside the auto-generated API commands.
Custom commands get a fully-wired SDK client that inherits the CLI's
auth, retries, TLS, base URL, and global headers — zero configuration required.

## Architecture

```
cli/videogen/custom.rs    ← Your command handlers (protected by .fernignore)
cli/videogen/sdk.rs       ← Generated bridge: client() + block_on()
cli/videogen/main.rs      ← Generated entrypoint (calls custom::register)
videogen-sdk/             ← Co-generated typed SDK crate
videogen-types/           ← Co-generated typed model crate
```

## Adding a Custom Command

### 1. Edit `cli/videogen/custom.rs`

This file is protected by `.fernignore` — `fern generate` will never
overwrite it. Register commands in the `register()` function:

```rust
use videogen_sdk::api::*;

pub fn register(app: CliApp) -> CliApp {
    let app = app.command(
        clap::Command::new("list-workflow-runs")
            .about("List workflow runs")
        ,
        |matches, ctx| {
            let client = super::sdk::client(ctx);
            let result = super::sdk::block_on(
                client.workflows.list_workflow_runs(),
            )?;
            println!("{}", serde_json::to_string_pretty(&result).unwrap());
            Ok(())
        },
    );
    app
}
```

Then build and test:
```bash
cargo build
videogen list-workflow-runs
```

### 2. Available SDK Clients

The `super::sdk::client(ctx)` call returns a `videogen_sdk::api::Client`
with the following sub-clients:

| Field | Type | Description |
|-------|------|-------------|
| `client.workflows` | `videogen_sdk::api::WorkflowsClient` | workflows operations |
| `client.projects` | `videogen_sdk::api::ProjectsClient` | projects operations |
| `client.tools` | `videogen_sdk::api::ToolsClient` | tools operations |
| `client.files` | `videogen_sdk::api::FilesClient` | files operations |
| `client.entities` | `videogen_sdk::api::EntitiesClient` | entities operations |
| `client.text` | `videogen_sdk::api::TextClient` | text operations |
| `client.resources` | `videogen_sdk::api::ResourcesClient` | resources operations |
| `client.webhooks` | `videogen_sdk::api::WebhooksClient` | webhooks operations |
| `client.account` | `videogen_sdk::api::AccountClient` | account operations |

### 3. Key Patterns

**Get the SDK client** (execution-sharing, fully authenticated):
```rust
let client = super::sdk::client(ctx);
```

**Run an async SDK call from a sync handler:**
```rust
let result = super::sdk::block_on(
    client.some_resource.some_method(args),
)?;
```

**Use typed models for request/response serialization:**
```rust
use videogen_sdk::api::*;
```

### 4. Authentication

Custom commands automatically inherit the CLI's authentication.
The following auth schemes are configured:

- **bearerAuth** (bearer): env `VIDEOGEN_TOKEN`

No manual auth wiring is needed in custom command handlers.

## Regeneration Safety

| File | Regenerated? | Notes |
|------|-------------|-------|
| `cli/videogen/custom.rs` | **No** | Protected by `.fernignore` |
| `cli/videogen/sdk.rs` | Yes | Bridges AppContext → SDK client |
| `cli/videogen/main.rs` | Yes | Calls `custom::register(app)` |
| `videogen-sdk/` | Yes | Co-generated typed SDK crate |
| `videogen-types/` | Yes | Co-generated typed models |

After running `fern generate`, your `custom.rs` is preserved. All
generated code (SDK, types, glue, main.rs) is updated to match the
latest API spec. If the SDK surface changes (renamed methods, new
sub-clients), update your `custom.rs` to match.

## Build & Test

```bash
# Build the CLI (includes custom commands)
cargo build

# Run your custom command
videogen <your-command> [args]

# Run with verbose output for debugging
RUST_LOG=debug videogen <your-command> [args]
```
