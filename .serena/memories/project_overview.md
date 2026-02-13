# Project Overview

**Name:** Rusty Roguelike  
**Type:** Dungeon crawler game with procedurally generated levels

## Description
A turn-based dungeon crawler where the player explores procedurally generated dungeons, fights monsters, collects items, and seeks the Amulet of Yala to win.

## Tech Stack
- **Language:** Rust (edition 2021)
- **Game Engine:** Bevy 0.12
- **Key Dependencies:**
  - `bevy_ecs_tilemap` - Tilemap rendering
  - `pathfinding` - A* pathfinding for monster AI
  - `rand` - Random generation

## Architecture

Workspace with 4 crates:

```
bevy_dungeon_crawler/
├── bevy_fov/          # Field of view calculation library (standalone)
├── game_core/         # Core game logic: components, systems, states, events
├── game_ui/           # UI rendering: sprites, tilemap, camera, tooltips
└── game_starter/      # Entry point, asset loading
```

### Dependency Graph
```
bevy_fov ← game_core ← game_ui ← game_starter
                  ↑_________________|
```

### Bevy Architecture Patterns
- **Components:** Data structs stored in `components/` modules
- **Systems:** Functions in `systems/` modules that operate on components
- **Plugins:** Each crate exports a Plugin struct (GameCorePlugin, GameUiPlugin)
- **States:** GameState (InGame, NextLevel, Victory, GameOver) and GameTurn (Player, Monster)
- **SystemSets:** UiSystemSet and CoreSystemSet for ordering and scheduling
- **Events:** Action events (MoveAction, AttackAction, etc.) for turn-based logic