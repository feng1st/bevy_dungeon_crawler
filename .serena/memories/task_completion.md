# Task Completion Checklist

When completing a task in this project, run commands inside the `devel` distrobox:

1. **Run Clippy**
   ```bash
   distrobox-enter --name devel -- cargo clippy
   ```
   Ensure no warnings.

2. **Format Code**
   ```bash
   distrobox-enter --name devel -- cargo fmt
   ```

3. **Build Check**
   ```bash
   distrobox-enter --name devel -- cargo build
   ```

4. **Run the Game** (to verify functionality)
   ```bash
   distrobox-enter --name devel -- cargo run
   ```

## Note
This project has no automated tests. Manual testing by running the game is required.