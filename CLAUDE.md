# LeetCode Rust Solutions - Project Context for Claude

## Project Overview
This is a collection of LeetCode problem solutions implemented in Rust. Each problem is organized as a separate Rust crate within the `problems/` directory. The project uses Nix for reproducible development environments and `just` command runner for task automation.

## Project Structure
```
leetcode-rust/
├── problems/              # Individual problem solutions
│   ├── two_sum/
│   ├── palindrome_number/
│   ├── longest_substring_without_repeating_characters/
│   └── ... (other problems)
├── Cargo.toml            # Workspace configuration
├── template.rs           # Template for new problems
├── justfile             # Task automation commands
├── flake.nix            # Nix development environment
├── README.md            # User-facing documentation
└── CLAUDE.md            # This file - AI context
```

## Development Environment

### Quick Start (Copy & Paste)
```bash
# Install Nix with flakes enabled (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install

# Enter development shell with all tools
nix develop
```

The Nix environment provides:
- Rust toolchain (stable)
- rust-analyzer
- cargo-watch
- cargo-edit
- just command runner

### Available Commands
Once in the Nix shell, these commands are available:

```bash
just list              # List all problems
just new <name>        # Create new problem
just test <name>       # Test specific problem
just test-all          # Test all problems
just watch <name>      # Auto-test on changes
just run <name>        # Run problem (if has main)
just clean             # Clean build artifacts
```

### Direct Cargo Commands (Fallback)
```bash
cargo test -p <problem_name>    # Test specific problem
cargo test --workspace          # Test all
cargo fmt                       # Format code
cargo clippy                    # Lint code
cargo check                     # Type check
```

### Adding New Problems
**Preferred method** (using just):
```bash
just new problem_name
```

This command:
1. Creates `problems/problem_name/` directory structure
2. Sets up `Cargo.toml` with correct configuration
3. Copies `template.rs` to `src/lib.rs`
4. Displays the file path for editing

**Note**: You still need to manually add the problem to the workspace in root `Cargo.toml`:
```toml
members = [
    # ... existing members
    "problems/problem_name",
]
```

## Common Patterns
- Each problem solution is implemented as a struct called `Solution`
- Most solutions include unit tests with example cases from LeetCode
- Problems are categorized by difficulty and algorithm type (e.g., dynamic programming, sliding window)

## Important Notes
- The project uses Rust 2021 edition
- Nix flake is available for reproducible development environment
- The `justfile` contains useful shortcuts for common tasks
- Each problem crate is independent and can be worked on separately

## Git Workflow
- Main branch: `main`
- Commit messages should describe the problem being solved or improved
- Each problem solution should be well-tested before committing