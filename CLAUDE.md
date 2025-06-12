# LeetCode Rust Solutions - Project Context for Claude

## Project Overview
This is a collection of LeetCode problem solutions implemented in Rust. Each problem is organized as a separate Rust crate within the `problems/` directory. The project uses `just` command runner for task automation.

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
├── README.md            # User-facing documentation
└── flake.nix            # Nix development environment
```

## Development Workflow

### Just Commands (Preferred)
The project includes a `justfile` with helpful commands:

- **List problems**: `just list`
- **Create new problem**: `just new <problem_name>`
- **Test specific problem**: `just test <problem_name>`
- **Test all problems**: `just test-all`
- **Watch tests**: `just watch <problem_name>` (auto-runs on changes)
- **Run problem**: `just run <problem_name>` (if it has main function)
- **Clean artifacts**: `just clean`

### Manual Commands (Fallback)
If `just` is not available:
- **Test specific problem**: `cargo test -p <problem_name>`
- **Test all**: `cargo test --workspace`
- **Format**: `cargo fmt`
- **Lint**: `cargo clippy`
- **Check types**: `cargo check`

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