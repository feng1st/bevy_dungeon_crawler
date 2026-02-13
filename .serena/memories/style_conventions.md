# Code Style and Conventions

## Linting Configuration
The workspace uses strict Clippy lints (configured in root Cargo.toml):
- `clippy::all = "warn"`
- `clippy::pedantic = "warn"`

Allowed exceptions:
- `cast_possible_truncation`
- `cast_possible_wrap`
- `cast_precision_loss`
- `cast_sign_loss`
- `module_name_repetitions`
- `needless_pass_by_value`

## Naming Conventions
- **Components:** PascalCase structs with `#[derive(Component)]`
- **Systems:** snake_case functions
- **Events:** PascalCase with `Action` suffix (e.g., `MoveAction`, `AttackAction`)
- **States:** PascalCase enums deriving `States`
- **SystemSets:** PascalCase enums deriving `SystemSet`

## Module Organization
Each crate follows this pattern:
```
crate/
├── lib.rs           # Exports prelude module, defines Plugin
├── prelude.rs       # Re-exports commonly used items
├── components/
│   ├── mod.rs
│   └── *_components.rs
└── systems/
    ├── mod.rs
    └── *.rs         # One file per system or related group
```

## Bevy Patterns
- Use `prelude` module pattern for clean imports
- Systems use `#[allow(clippy::too_many_arguments)]` and `#[allow(clippy::too_many_lines)]` when needed
- Debug-only code wrapped in `#[cfg(debug_assertions)]`
- System ordering via SystemSets and `apply_deferred` calls
- Events for turn-based game actions