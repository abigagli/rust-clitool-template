# Rust CLI Template

A [cargo-generate](https://github.com/cargo-generate/cargo-generate) template for bootstrapping simple Rust CLI applications with a lib+bin crates structure and just recipes for cross-compilation support.

## Host platform
Only tested on MacOS/Intel, should work on Linux as well.

## Features

- **Lib + Bin architecture**:
- **CLI parsing**: Pre-configured example [clap](https://docs.rs/clap) setup with subcommands, arguments, and value enums
- **Logging**: Use [tracing](https://docs.rs/tracing) with environment-based filtering via `RUST_LOG`
- **Error handling**: Use [anyhow](https://docs.rs/anyhow) in the binary, setup for custom errors in the library
- **Strict lints**: Clippy configuration in Cargo.toml
- **Separate build directory**: Use Cargo [build.build-dir](https://doc.rust-lang.org/cargo/reference/config.html#buildbuild-dir) to keep build artifacts in `cargo_build_dir/` instead of `target/`
- **Cross-compilation recipes**: Ready-to-use [just](https://github.com/casey/just) recipes for multiple targets

## Requirements

- [cargo-generate](https://github.com/cargo-generate/cargo-generate)
- [just](https://github.com/casey/just) (for build recipes)
- [jq](https://jqlang.org/) (used by justfile to extract binary name)

For cross-compilation:

- [cargo-zigbuild](https://crates.io/crates/cargo-zigbuild) — for Raspberry Pi (aarch64-linux-gnu)
- [rustx Docker image](https://github.com/tindzk/alpine-rustx) — for Windows and Apple Intel/Silicon targets
- [cargo-zigbuild Docker image](https://hub.docker.com/r/messense/cargo-zigbuild) — for Apple Intel/Silicon targets

## Usage

### Generate a new project

```bash
cargo generate --git https://github.com/abigagli/rust-clitool-template
```

You will be prompted for:

| Placeholder            | Description                                               |
| ---------------------- | --------------------------------------------------------- |
| `project-name`         | Crate name (derived from `--name` argument)               |
| `author`               | Author name for Cargo.toml                                |
| `include_build_script` | Include `build.rs` with git/version info (default: false) |
| `rpi_host`             | Default SSH hostname/IP for Raspberry Pi deployment       |

> **Note**: The `rpi_host` value is only used by the `deploy_rpi` and `upload_rpi` justfile recipes. If you don't plan to deploy to a Raspberry Pi, you can safely accept the default or enter any placeholder value — it won't affect building or other functionality.

### Optional: Build Script with Version Info

If you enable `include_build_script`, a `build.rs` is added that captures build-time metadata:

| Environment Variable | Content                                                             |
| -------------------- | ------------------------------------------------------------------- |
| `BUILD_GIT_HASH`     | Short git commit hash (with `-dirty` suffix if uncommitted changes) |
| `BUILD_GIT_DATE`     | Commit date (ISO 8601)                                              |
| `BUILD_TIMESTAMP`    | Build time (UTC)                                                    |

Access these in your code:

```rust
const VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    " (", env!("BUILD_GIT_HASH"), " ", env!("BUILD_TIMESTAMP"), ")"
);
```

### Build locally

```bash
cargo build
cargo build --release
```

### Cross-compilation

The template provides justfile recipes for three cross-compilation scenarios:

#### Raspberry Pi (via cargo-zigbuild installed on the local machine)

Uses [cargo-zigbuild](https://crates.io/crates/cargo-zigbuild) which leverages Zig as a linker for cross-compilation to Linux targets without requiring a separate toolchain.

```bash
# Build for aarch64-unknown-linux-gnu
just build_for_rpi
just build_for_rpi --release

# Build and deploy to configured RPI host
just deploy_rpi debug
just deploy_rpi release
```

#### Windows & Apple Intel/Silicon (via rustx Docker)

Uses the [alpine-rustx](https://github.com/tindzk/alpine-rustx) Docker image which provides pre-configured cross-compilation toolchains.

```bash
# Build for Windows (x86_64-pc-windows-gnu)
just build_for_windows
just build_for_windows --release

# Build for Apple Silicon (aarch64-apple-darwin)
just build_for_macos_aarch64
just build_for_macos_aarch64 --release

# Build for Apple Intel (x86_64-apple-darwin)
just build_for_macos_intel
just build_for_macos_intel --release
```

#### Apple Intel/Silicon (via cargo-zigbuild Docker)

Uses the [cargo-zigbuild Docker image](https://hub.docker.com/r/messense/cargo-zigbuild) which provides pre-configured cross-compilation toolchains.

```bash
# Build for Apple Silicon (aarch64-apple-darwin)
just zigbuild_for_macos_aarch64
just zigbuild_for_macos_aarch64 --release

# Build for Apple Intel (x86_64-apple-darwin)
just zigbuild_for_macos_intel
just zigbuild_for_macos_intel --release
```

> **Note**: When using alpine-rustx, the Docker image must be available locally as `rustx_crosscompiler:latest`. See the [rustx repository](https://github.com/tindzk/alpine-rustx) for build instructions.

## Project Structure

```
my-tool/
├── .cargo/
│   └── config.toml      # Cargo aliases and cross-compilation settings
├── .vscode/
│   └── settings.json    # Editor configuration
├── src/
│   ├── main.rs          # Entry point with logging initialization
│   ├── lib.rs           # Library crate root
│   ├── args.rs          # CLI argument definitions (clap)
│   └── error.rs         # Error handling utilities
├── build.rs             # (optional) Git/version info at compile time
├── Cargo.toml           # Dependencies and lint configuration
└── justfile             # Build and deployment recipes
```

## Included Dependencies

| Crate                                                    | Purpose                                          |
| -------------------------------------------------------- | ------------------------------------------------ |
| [clap](https://docs.rs/clap)                             | Command-line argument parsing with derive macros |
| [anyhow](https://docs.rs/anyhow)                         | Flexible error handling                          |
| [tracing](https://docs.rs/tracing)                       | Structured logging                               |
| [tracing-subscriber](https://docs.rs/tracing-subscriber) | Log output formatting with env-filter            |
| [serde](https://docs.rs/serde)                           | Serialization framework                          |
| [tokio](https://docs.rs/tokio)                           | Async runtime (minimal features)                 |
| [chrono](https://docs.rs/chrono)                         | Date and time handling                           |
| [humantime](https://docs.rs/humantime)                   | Human-readable duration parsing                  |

## Customization

After generating your project:

1. Update `src/args.rs` to define your CLI interface
2. Add business logic to `src/lib.rs`
3. Modify lint levels in `Cargo.toml` as needed
4. Adjust cross-compilation targets in `justfile` and `.cargo/config.toml`

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
