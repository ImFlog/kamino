# Claude.md - Kamino Project Guide

## Project Overview

T-triste is a competitive puzzle game built with the Bevy game engine. The project is structured as a Rust workspace
with three crates.

## Tech Stack

- **Language**: Rust (stable channel)
- **Game Engine**: Bevy 0.17
- **Edition**: Rust 2021
- **Build System**: Cargo (workspace)

## Workspace Structure

```
t-triste/
├── t-triste/           # Main game binary
├── t-triste-lib/       # Core game library
├── t-triste-macro/     # Procedural macros
├── rustfmt.toml        # Code formatting configuration
├── .clippy.toml        # Linting configuration
└── rust-toolchain.toml # Rust toolchain specification
```

## Architecture

### Camera and Coordinate Systems

In this Bevy project, we use two main coordinate systems:

1. **World Coordinates:** These are used by `Transform` for all game objects (pieces, board). By default, **(0,0) is at
   the center of the screen**. X increases to the right, and Y increases upwards.
2. **Window/Viewport Coordinates:** These are used by mouse and window events. (0,0) is at the bottom-left of the
   window, with coordinates measured in pixels.

To make them work together, we must convert the window-space mouse position into world-space.

#### Cursor Position Conversion

The `cursor_state` system in `t-triste-lib/src/cursor.rs` handles the conversion from window coordinates to world
coordinates using the `camera.viewport_to_world_2d` method. This ensures that when you click or move the mouse, the
`Cursor` resource contains the correct world position, allowing it to interact accurately with game objects like pieces
and the board.

```rust
if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, event.position) {
cursor.current_pos = world_pos;
}
```

#### Board and Piece Spawning

The game board is centered at (0,0) in world coordinates. Consequently, pieces are spawned at specific world
coordinates (e.g., negative values for the left/bottom parts of the screen) to be visible and correctly positioned
relative to the centered board.

## Development Setup

### Prerequisites

- Rust stable toolchain (automatically managed via `rust-toolchain.toml`)
- Cargo

### Running the Game

```bash
# Standard run
cargo run
```

### Building

```bash
# Development build
cargo build

# Release build
cargo build --release --all-features
```

## Code Quality Standards

### Formatting

The project uses `rustfmt` with the following configuration:

- **Max line width**: 100 characters
- **Edition**: 2018

**Before committing, always run:**

```bash
cargo fmt --all
```

**To check formatting without modifying files:**

```bash
cargo fmt --all -- --check
```

### Linting

The project uses Clippy with strict settings:

- All warnings are treated as errors (`-D warnings`)
- Configuration in `.clippy.toml`

**Before committing, always run:**

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

**Fix auto-fixable issues:**

```bash
cargo clippy --all-targets --all-features --fix
```

## Testing

### Running Tests

```bash
# Run all tests
cargo test --all-features

# Run tests for a specific crate
cargo test -p t-triste-lib --all-features

# Run a specific test
cargo test test_name --all-features
```

### Test Requirements

- All new features should include appropriate tests
- Tests must pass in CI before merging
- Use `--all-features` flag to ensure compatibility

## CI/CD Pipeline

### Pull Request Checks

Every PR runs the following checks:

1. **Formatting**: `cargo fmt --all -- --check`
2. **Linting**: `cargo clippy --all-targets --all-features -- -D warnings`
3. **Tests**: `cargo test --all-features`

### Main Branch Pipeline

On merge to main/master, additional steps run:

1. All PR checks (formatting, linting, tests)
2. **Release Build**: `cargo build --release --all-features`

### Important: All Checks Must Pass

- CI treats warnings as errors
- PRs cannot be merged if any check fails
- Always run formatting, clippy, and tests locally before pushing

## Pre-Commit Checklist

Before committing code, ensure:

1. Code is formatted: `cargo fmt --all`
2. Clippy passes: `cargo clippy --all-targets --all-features -- -D warnings`
3. Tests pass: `cargo test --all-features`
4. Code builds: `cargo build --all-features`

**Quick check command:**

```bash
cargo fmt --all && cargo clippy --all-targets --all-features -- -D warnings && cargo test --all-features
```

## Git Commit Standards

### Conventional Commits

This repository follows the [Conventional Commits](https://www.conventionalcommits.org/) format for all commit messages.

**Format:**

```
<type>(<optional scope>): <description>

[optional body]

[optional footer(s)]
```

**Common types:**

- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation only changes
- `style`: Changes that don't affect code meaning (white-space, formatting, etc)
- `refactor`: Code change that neither fixes a bug nor adds a feature
- `perf`: Performance improvements
- `test`: Adding missing tests or correcting existing tests
- `build`: Changes to build system or dependencies
- `ci`: Changes to CI configuration files and scripts
- `chore`: Other changes that don't modify src or test files

**Examples:**

```
feat: add new piece rotation system
fix: correct collision detection in grid
docs: update README with installation instructions
test: add unit tests for piece builders
refactor: simplify cursor movement logic
```

**Best practices:**

- Use lowercase for the type and description
- Keep the subject line (first line) under 72 characters
- Use the imperative mood ("add" not "added" or "adds")
- Don't end the subject line with a period
- Separate subject from body with a blank line (if body is needed)

## Common Development Commands

```bash
# Format code
cargo fmt --all

# Check code without building
cargo check --all-features

# Run clippy
cargo clippy --all-targets --all-features

# Run tests
cargo test --all-features

# Build release
cargo build --release --all-features

# Clean build artifacts
cargo clean
```

## Bevy Development Tips

### Dynamic Linking (Faster Iteration)

For faster compile times during development:

```bash
cargo run --features bevy/dynamic
```

### Bevy Features

The project uses minimal Bevy features to reduce compile times:

- `bevy_ui_render`
- `bevy_render`
- `bevy_sprite`
- `bevy_winit`
- `x11`
- `bevy_asset`
- `bevy_scene`
- `bevy_core_pipeline`
- `png`
