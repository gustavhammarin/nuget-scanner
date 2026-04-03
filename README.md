# nuget-scanner

A CLI tool that recursively resolves all transitive dependencies of a NuGet package and checks them for known vulnerabilities using the [OSV](https://osv.dev) database. Results are displayed in an interactive terminal UI.

## Installation

### Linux / macOS

```sh
curl -fsSL https://raw.githubusercontent.com/gustavhammarin/nuget-scanner/main/install.sh | sh
```

The script detects your OS and architecture, downloads the latest release binary to `~/.local/bin`, and adds it to your `PATH`. Reload your shell after:

```sh
source ~/.bashrc   # or ~/.zshrc
```

### Windows

Download the binary manually from the [releases page](../../releases) — see the table below.

### Manual download

Download a pre-built binary from the [releases page](../../releases) for your platform:

| Platform | Binary |
|---|---|
| Linux x86_64 | `nuget-scanner-x86_64-unknown-linux-gnu` |
| Linux arm64 | `nuget-scanner-aarch64-unknown-linux-gnu` |
| macOS x86_64 | `nuget-scanner-x86_64-apple-darwin` |
| macOS arm64 (Apple Silicon) | `nuget-scanner-aarch64-apple-darwin` |
| Windows x86_64 | `nuget-scanner-x86_64-pc-windows-msvc.exe` |

## Usage

```
nuget-scanner <PACKAGE_ID> <VERSION> <TARGET_FRAMEWORK>
```

### Arguments

| Argument | Description | Example |
|---|---|---|
| `PACKAGE_ID` | The NuGet package ID to scan | `Microsoft.EntityFrameworkCore` |
| `VERSION` | The exact version to scan | `8.0.2` |
| `TARGET_FRAMEWORK` | The target framework moniker | `net8.0` |

### Example

```sh
nuget-scanner Microsoft.EntityFrameworkCore 8.0.2 net8.0
```

Results are shown in an interactive TUI. Navigate with:

| Key | Action |
|---|---|
| `j` / `↓` | Next row |
| `k` / `↑` | Previous row |
| `q` / `Esc` | Quit |

## How it works

1. Fetches the dependency graph of the given package from the NuGet registry, recursively resolving all transitive dependencies for the specified target framework.
2. Queries the OSV API for each resolved dependency to find known vulnerabilities.
3. Displays all findings in an interactive terminal table with OSV ID and summary.

## Build

Requires [Rust](https://www.rust-lang.org/tools/install).

```sh
cargo build --release
./target/release/nuget-scanner <PACKAGE_ID> <VERSION> <TARGET_FRAMEWORK>
```
