# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Rusty Roguelike** - A turn-based dungeon crawler built with Bevy. The player explores procedurally generated dungeons, fights monsters, collects items, and seeks the Amulet of Yala to win.

## Commands

```bash
cargo run             # Run the game
cargo build           # Build the project
cargo clippy          # Lint with clippy (strict pedantic lints enabled)
cargo fmt             # Format code
```

## Development Environment

This project is developed on Bazzite using a distrobox container named `devel` for build dependencies.

To run commands in the distrobox:
```bash
distrobox-enter --name devel -- cargo run
distrobox-enter --name devel -- cargo clippy
```

VSCode uses Dev Containers extension with podman-host. rust-analyzer requires custom `LD_LIBRARY_PATH` for Bevy's dynamic linking.

## Architecture

Workspace with 4 crates using Bevy's ECS (Entity Component System) pattern:

| Crate | Purpose |
|-------|---------|
| `bevy_fov` | Field of view calculation library |
| `game_core` | Core game logic: components, systems, states, events |
| `game_ui` | Rendering: sprites, tilemap, camera, tooltips |
| `game_starter` | Entry point (default member) |

**Dependency flow:** `bevy_fov` ← `game_core` ← `game_ui` ← `game_starter`

### Key Bevy Patterns

- **Components:** Data-only structs with `#[derive(Component)]` in `components/` modules
- **Systems:** Functions operating on components in `systems/` modules
- **Plugins:** Each crate exports a Plugin (`GameCorePlugin`, `GameUiPlugin`)
- **States:** `GameState` (InGame/NextLevel/Victory/GameOver), `GameTurn` (Player/Monster)
- **Events:** Action events (`MoveAction`, `AttackAction`, etc.) drive turn-based logic
- **SystemSets:** `CoreSystemSet` and `UiSystemSet` control system ordering with explicit `apply_deferred` calls

### Code Organization

Each crate follows:
```
crate/
├── lib.rs           # Plugin definition, state/event registration
├── components/      # ECS components
└── systems/         # System functions
```

The `prelude` module pattern is used throughout for clean imports.

## Linting

Strict Clippy pedantic lints are enforced at the workspace level. Run `cargo clippy` before committing.
