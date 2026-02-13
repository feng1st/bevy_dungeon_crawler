# Suggested Commands

## Development Environment

This project is developed on Bazzite using a distrobox container named `devel`. All cargo commands should run inside the distrobox:

```bash
distrobox-enter --name devel -- cargo build
distrobox-enter --name devel -- cargo run
```

VSCode uses Dev Containers extension with podman-host for containerized development.

## Build and Run
```bash
distrobox-enter --name devel -- cargo build           # Build the project
distrobox-enter --name devel -- cargo run             # Run the game
distrobox-enter --name devel -- cargo run --release   # Run with release optimizations
```

## Linting
```bash
distrobox-enter --name devel -- cargo clippy          # Run clippy lints
distrobox-enter --name devel -- cargo clippy --fix    # Auto-fix clippy warnings
```

## Development
```bash
distrobox-enter --name devel -- cargo check           # Fast compile check
distrobox-enter --name devel -- cargo fmt             # Format code
```

## Workspace Members
```bash
distrobox-enter --name devel -- cargo build -p game_core    # Build specific crate
```

## Notes
- No tests are currently present in this project
- `dynamic_linking` feature is enabled for Bevy for faster compile times during development
- Debug profile has `opt-level = 1` for faster compile, dependencies use `opt-level = 3`