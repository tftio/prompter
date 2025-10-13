# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build and Development Commands

- **Build:** `cargo build --release`
- **Run tests:** `cargo test` (unit tests), `cargo test --test cli` (integration tests)
- **Format:** `cargo fmt --all -- --check`
- **Lint:** `cargo clippy --all-targets --all-features -- -D warnings`
- **Security audit:** `cargo audit`
- **Dependency check:** `cargo deny check`
- **Unused deps:** `cargo machete`
- **TOML format:** `taplo format --check`
- **TOML lint:** `taplo check`
- **Coverage:** `cargo tarpaulin --all --out Xml --engine llvm --timeout 300 --fail-under 80` (Linux only)

### Aggressive Linting Setup

**Git Hooks (peter-hook):**
- Pre-commit hooks run comprehensive checks: TOML formatting, Rust formatting, compilation, aggressive clippy, security audit, dependency compliance, documentation, and full test suite
- Commit message validation enforces proper length limits
- All hooks configured in `hooks.toml`

**Quality Standards:**
- Aggressive clippy linting with pedantic and nursery lints enabled
- Comprehensive documentation required for all public APIs
- Security vulnerability scanning on every commit
- License compliance enforcement
- Zero tolerance for unsafe code (except in specifically marked tests)

## Architecture

**prompter** is a CLI tool for composing reusable prompt snippets from a library using TOML profiles.

### Core Components

- **`main.rs`**: CLI entry point, argument parsing, and mode dispatch
- **`lib.rs`**: Core logic split into several key areas:
  - **Config parsing**: Custom TOML parser that handles profiles and dependencies
  - **Profile resolution**: Recursive dependency resolution with cycle detection and deduplication
  - **File rendering**: Concatenation of resolved files with optional separators and system info prefix

### Key Data Flow

1. **Config**: Profiles map to lists of dependencies (`.md` files or other profile names)
2. **Resolution**: Depth-first traversal respects `depends_on` order, deduplicates by path (first occurrence wins)
3. **Output**: Pre-prompt + System prefix + file contents with optional separators

### Output Structure

All rendered profiles follow this format:
```
[Pre-prompt text]

[System info: "Today is YYYY-MM-DD, and you are running on a ARCH/OS system."]

[File 1 content]
[Optional separator]
[File 2 content]
[Optional separator]
...
```

The pre-prompt defaults to LLM coding agent instructions but can be customized or disabled.

### File Locations

- **Config**: `~/.config/prompter/config.toml`
- **Library**: `~/.local/prompter/library/` (markdown snippets)

### Current CLI Design (Compliant with User Standards)

Uses subcommand pattern with clap:
- `prompter <profile>` - render profile (backward compatible)
- `prompter run <profile>` - explicit render command
- `prompter list` - list profiles
- `prompter validate` - validate config
- `prompter init` - create default config/library (with progress spinner)
- `prompter version` - show version
- `prompter help` - show help (built-in)
- `prompter completions <shell>` - generate shell completions (bash/zsh/fish)
- `prompter doctor` - health check and update notifications
- `prompter update` - self-update to latest version
- `prompter -s <sep> <profile>` - render with separator
- `prompter -p <text> <profile>` - render with custom pre-prompt

**Features**:
- ✅ Subcommand pattern using clap
- ✅ Built-in help and version subcommands
- ✅ PTY detection with `is-terminal` crate
- ✅ Terminal effects using `colored` and `indicatif`
- ✅ Colorful output with emojis when interactive
- ✅ Clean output when piped/redirected
- ✅ Progress spinners during operations
- ✅ Pre-prompt injection (defaults to LLM coding agent instructions)

### Testing Strategy

- **Unit tests**: Embedded in `lib.rs` with `#[cfg(test)]`, use temporary directories
- **Integration tests**: `tests/cli.rs` tests full binary with `Command::new(bin_path())`
- **Coverage**: CI enforces 80% minimum via tarpaulin

### Profile Resolution Logic

Recursive expansion with these behaviors:
- **Cycle detection**: Maintains stack to detect circular dependencies
- **Deduplication**: `HashSet<PathBuf>` ensures first occurrence wins
- **Error handling**: Clear messages for missing files, unknown profiles, cycles
- **Order preservation**: Depth-first maintains `depends_on` sequence

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
# Validate and push tag (triggers GitHub Actions release)
just release v1.0.11

# Manual version management (advanced)
versioneer patch             # Bump version: 1.0.10 -> 1.0.11
versioneer minor             # Bump version: 1.0.10 -> 1.1.0
versioneer major             # Bump version: 1.0.10 -> 2.0.0
versioneer sync              # Synchronize version files
versioneer verify            # Check version consistency
git tag v1.0.11              # Create tag (use v{version} format)
```

### Release Workflow

The release process is fully automated via GitHub Actions when a tag is pushed:

1. **Prepare release**: Version bump and create tag locally
   ```bash
   versioneer patch        # Bump to next patch version
   git add Cargo.toml Cargo.lock VERSION
   git commit -m "chore: bump version to $(cat VERSION)"
   git tag v1.0.11         # Create tag matching VERSION file
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
- **Tag conflicts**: Delete incorrect tag with `git tag -d TAG_NAME` and recreate with correct format
- **Failed releases**: Check GitHub Actions logs for version validation errors
- **Release not triggered**: Ensure tag format is `v{version}` (e.g., `v1.0.10`), not `prompter-v{version}`
- **Wrong tag format**: Always use `git tag v{version}` format, not versioneer's default `prompter-v{version}`