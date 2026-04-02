# nuget-scanner

A CLI tool that recursively resolves all transitive dependencies of a NuGet package and checks them for known vulnerabilities using the [OSV](https://osv.dev) database.

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

Output:

```
Gathering data...
Checking for vulnerabilites...
+---------------+-------------------------------+
| ID            | SUMMARY                       |
+===============+===============================+
| GHSA-xxxx-... | Some vulnerability summary    |
+---------------+-------------------------------+
```

## How it works

1. Fetches the dependency graph of the given package from the NuGet registry, recursively resolving all transitive dependencies for the specified target framework.
2. Queries the OSV API for each resolved dependency to find known vulnerabilities.
3. Prints a table of all findings with their OSV ID and summary.

## Build

Requires [Rust](https://www.rust-lang.org/tools/install).

```sh
cargo build --release
./target/release/nuget-scanner <PACKAGE_ID> <VERSION> <TARGET_FRAMEWORK>
```
