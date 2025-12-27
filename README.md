# nexum-chrome-sys

[![Crates.io](https://img.shields.io/crates/v/nexum-chrome-sys.svg)](https://crates.io/crates/nexum-chrome-sys)
[![Documentation](https://docs.rs/nexum-chrome-sys/badge.svg)](https://docs.rs/nexum-chrome-sys)
[![CI](https://github.com/nxm-rs/nexum-chrome-sys/actions/workflows/ci.yml/badge.svg)](https://github.com/nxm-rs/nexum-chrome-sys/actions/workflows/ci.yml)
[![License: AGPL-3.0-or-later](https://img.shields.io/badge/License-AGPL--3.0--or--later-blue.svg)](LICENSE)

**Rust bindings for Chrome Extension APIs using wasm-bindgen.**

Auto-generated from Google's official [chrome-types](https://github.com/GoogleChrome/chrome-types) schema, following the same patterns as [web-sys](https://docs.rs/web-sys).

## Features

- **133 Chrome API namespaces** - tabs, runtime, storage, bookmarks, and more
- **Feature-gated** - only compile what you use
- **Type-safe** - full Rust type checking for Chrome APIs
- **Zero-cost abstractions** - thin wasm-bindgen wrappers over JS objects
- **Auto-updated** - weekly regeneration from latest Chrome API definitions

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
nexum-chrome-sys = { version = "0.1", features = ["tabs", "runtime", "storage"] }
```

## Quick Start

```rust
use nexum_chrome_sys::tabs;
use wasm_bindgen_futures::JsFuture;

// Query for the active tab
let query_info = tabs::QueryInfo::new();
query_info.set_active(true);
query_info.set_current_window(true);

let promise = tabs::query(query_info);
let result = JsFuture::from(promise).await?;
```

## Available APIs

Each Chrome API namespace is behind a feature flag:

| Feature | Chrome API | Description |
|---------|------------|-------------|
| `tabs` | chrome.tabs | Tab management |
| `runtime` | chrome.runtime | Extension lifecycle |
| `storage` | chrome.storage | Persistent storage |
| `windows` | chrome.windows | Window management |
| `bookmarks` | chrome.bookmarks | Bookmark management |
| `cookies` | chrome.cookies | Cookie access |
| `alarms` | chrome.alarms | Scheduled events |
| `notifications` | chrome.notifications | System notifications |
| `webRequest` | chrome.webRequest | Network request interception |
| `scripting` | chrome.scripting | Script injection |

...and 123 more! See [`Cargo.toml`](crates/nexum-chrome-sys/Cargo.toml) for the complete list of available features.

## How It Works

This crate uses [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) to create JavaScript bindings following the [web-sys](https://docs.rs/web-sys) pattern:

- **Enums** - `#[wasm_bindgen]` with string discriminants, passed by value
- **Dictionaries** - Opaque JS Object wrappers with getter/setter methods
- **Functions** - Return `Promise` for async operations
- **Cross-namespace types** - Feature-gated with proper type paths

### Example: Dictionary Pattern

```rust
use nexum_chrome_sys::tabs;

// Create a new dictionary (wraps JS Object)
let create_props = tabs::CreateProperties::new();

// Use setters to configure
create_props.set_url("https://example.com");
create_props.set_active(true);
create_props.set_pinned(false);

// Pass to Chrome API
let promise = tabs::create(create_props);
```

## Regeneration

Bindings are automatically regenerated weekly via GitHub Actions. To manually regenerate:

```bash
# Clone chrome-types and generate schema
git clone --depth 1 https://github.com/GoogleChrome/chrome-types.git
cd chrome-types && pnpm install
node tools/prepare.js > ../chrome-api.json
cd .. && rm -rf chrome-types

# Generate Rust bindings
cargo run -p nexum-chrome-sys-gen -- -i chrome-api.json -o crates/nexum-chrome-sys
```

## Development

This project uses [Nix](https://nixos.org/) for reproducible development:

```bash
# Enter development shell
nix develop

# Build all features
cargo build --package nexum-chrome-sys --all-features

# Run lints
cargo clippy --all-targets --all-features -- -D warnings

# Format code
cargo fmt --all
```

## License

This project is licensed under the [GNU Affero General Public License v3.0 or later](LICENSE).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

---

<sub>Built with [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) | Generated from [chrome-types](https://github.com/GoogleChrome/chrome-types)</sub>
