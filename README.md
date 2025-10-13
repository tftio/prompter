# prompter

[![CI](https://github.com/OWNER/REPO/actions/workflows/ci.yml/badge.svg)](https://github.com/OWNER/REPO/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/OWNER/REPO/branch/main/graph/badge.svg?token=CODECOV_TOKEN)](https://codecov.io/gh/OWNER/REPO)

A small Rust CLI to compose reusable prompt snippets from a library and a TOML manifest. Designed for fast local use and simple, explicit behavior.

## Features
- Profiles in TOML under `~/.config/prompter/config.toml`
- Markdown snippets under `~/.local/prompter/library`
- Recursive profile composition with cycle detection
- Path-based deduplication (first occurrence wins)
- Deterministic depth-first order respecting `depends_on`
- Optional output separator with escape support (`\n`, `\t`, `"`, `\`)
- Optional config override via `--config` for alternate manifests
- Utilities: `--list`, `--validate`, `--init`, `--version`

## Install

### Quick Install (Recommended)

Install the latest release directly from GitHub:

```bash
curl -fsSL https://raw.githubusercontent.com/tftio/prompter/main/install.sh | sh
```

Or with a custom install directory:

```bash
INSTALL_DIR=/usr/local/bin curl -fsSL https://raw.githubusercontent.com/tftio/prompter/main/install.sh | sh
```

The install script will:
- Auto-detect your OS and architecture
- Download the latest release
- Verify checksums (when available)
- Install to `$HOME/.local/bin` by default
- Prompt before replacing existing installations
- Guide you on adding the directory to your PATH

### Manual Install Options

**Option A — From source (requires Rust toolchain):**

```bash
git clone https://github.com/tftio/prompter.git
cd prompter
cargo build --release
install -m 0755 target/release/prompter ~/.local/bin/
```

**Option B — Using build script:**

```bash
./scripts/install.sh            # builds and installs to ~/.local/bin
./scripts/install.sh /opt/bin   # custom destination
```

**Option C — Download release manually:**

1. Go to [Releases](https://github.com/tftio/prompter/releases)
2. Download the appropriate `prompter-{target}.zip` for your platform
3. Extract and copy the binary to a directory in your PATH

### Supported Platforms

- **Linux**: x86_64, aarch64
- **macOS**: x86_64 (Intel), aarch64 (Apple Silicon)
- **Windows**: x86_64

### PATH Setup

If the install directory is not in your PATH, add it:

```bash
echo 'export PATH="\$HOME/.local/bin:\$PATH"' >> ~/.bashrc  # or ~/.zshrc
source ~/.bashrc  # or source ~/.zshrc
```

## Initialize
Creates default config and sample library files (non-destructive: only writes if missing):

```bash
prompter init
```

## Configure
Location: `~/.config/prompter/config.toml`

Example:

```toml
[python.api]
depends_on = ["a/b/c.md", "f/g/h.md"]

[general.testing]
depends_on = ["python.api", "a/b/d.md"]
```

- Any `depends_on` entry ending with `.md` is treated as a library file path relative to `~/.local/prompter/library`.
- Any other entry is treated as another profile and expanded recursively.

### Using an Alternate Config

You can point prompter at a specific config file using `--config` (works globally or per command). The library root is resolved relative to that config file when an override is supplied.

```bash
prompter --config demo/config.toml list
prompter run --config demo/config.toml demo.profile
```

## Use

```bash
# List profiles
prompter list

# Validate config and library references
prompter validate

# Render a profile (concatenated file contents)
prompter python.api

# Explicit render command
prompter run python.api

# Render with a separator between files
prompter -s "\n---\n" python.api

# Render with a custom pre-prompt
prompter -p "Custom instruction here.\n" python.api

# Render with both separator and pre-prompt
prompter -s "\n---\n" -p "Custom pre-prompt.\n" python.api

# Override config for a single render
prompter --config demo/config.toml run demo.profile

# Show help
prompter help

# Show version
prompter version
```

### Pre-prompt Feature

By default, prompter adds a pre-prompt to all rendered output:

> "You are an LLM coding agent. Here are invariants that you must adhere to. Please respond with 'Got it' when you have studied these and understand them. At that point, the operator will give you further instructions. You are *not* to do anything to the contents of this directory until you have been explicitly asked to, by the operator."

You can override this with the `-p/--pre-prompt` option or disable it entirely by providing an empty string.

## Behavior
- Missing files or unknown profiles: exits non-zero with clear errors.
- Dedup: first path occurrence included, repeats dropped.
- Order: depth-first traversal, preserves provided `depends_on` order.

## Version Management

**Automated Release Process** - This project uses `versioneer` for atomic version management:

### Required Tools
- **`versioneer`**: Synchronizes versions across Cargo.toml and VERSION files
- **`peter-hook`**: Git hooks enforce version consistency validation
- **`just`**: Task runner for automated release workflow

### Version Management Rules
1. **NEVER manually edit Cargo.toml version** - Use versioneer instead
2. **NEVER create git tags manually** - Use `just release` or versioneer commands
3. **ALWAYS use automated release workflow** - Prevents version/tag mismatches

### Tag Format

This project uses the **`v{version}`** tag format (e.g., `v1.0.10`), not monorepo-style `{project}-v{version}` tags.

**Examples:**
- ✅ `v1.0.10`
- ✅ `v1.1.0`
- ✅ `v2.0.0`
- ❌ `prompter-v1.0.10` (monorepo format, not used here)

### Release Commands
```bash
# Create and push tag (triggers GitHub Actions release)
just release v1.0.11

# Manual version management (advanced)
versioneer patch             # Bump version: 1.0.10 -> 1.0.11
versioneer minor             # Bump version: 1.0.10 -> 1.1.0
versioneer major             # Bump version: 1.0.10 -> 2.0.0
versioneer sync              # Synchronize version files
versioneer verify            # Check version consistency
versioneer tag --tag-format "v{version}"  # Create v{VERSION} tag (REQUIRED: override default prompter-v{version} format)
```

### Release Workflow

The release process is fully automated via GitHub Actions when a tag is pushed:

1. **Prepare release**: Version bump and create tag locally
   ```bash
   versioneer patch        # Bump to next patch version
   git add Cargo.toml Cargo.lock VERSION
   git commit -m "chore: bump version to $(cat VERSION)"
   versioneer tag --tag-format "v{version}"  # Creates v{VERSION} tag
   ```

2. **Trigger release**: Push tag to GitHub
   ```bash
   just release v1.0.11    # Validates and pushes tag
   # OR manually: git push origin v1.0.11
   ```

3. **GitHub Actions automatically**:
   - Validates tag version matches Cargo.toml
   - Creates draft release with generated notes
   - Builds binaries for Linux (x64, ARM64), macOS (Intel, Apple Silicon), Windows
   - Uploads binaries with LICENSE, README.md, CLAUDE.md
   - Generates SHA256 checksums
   - Publishes release

4. **Monitor**: `gh run list --workflow=release.yml`

### Quality Gates
- **Pre-push hooks**: Verify version file synchronization and tag consistency (via peter-hook)
- **GitHub Actions**: Validate tag version matches Cargo.toml before release
- **Binary verification**: Confirm built binary reports expected version
- **CI pipeline**: Full quality checks (format, lint, test, audit) run on main branch

### Troubleshooting
- **Version mismatch errors**: Run `versioneer verify` and `versioneer sync`
- **Tag conflicts**: Use `versioneer tag --tag-format "v{version}"` instead of `git tag`
- **Failed releases**: Check GitHub Actions logs for version validation errors
- **Release not triggered**: Ensure tag format is `v{version}` (e.g., `v1.0.10`), not `prompter-v{version}`
- **Wrong tag format created**: versioneer's default is `prompter-v{version}` - always use `--tag-format "v{version}"` flag

## Development
- Build: `cargo build --release`
- Unit tests: `cargo test`
- Integration tests: `cargo test --test cli`

## Paths
- Config: `~/.config/prompter/config.toml`
- Library root: `~/.local/prompter/library`

## License
MIT License. See LICENSE for details.
