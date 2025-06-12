# LeetCode Rust Solutions - Project Context for Claude

## Project Overview
This is a collection of LeetCode problem solutions implemented in Rust. Each problem is organized as a separate Rust crate within the `problems/` directory.

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
├── justfile             # Task automation
└── flake.nix            # Nix development environment
```

## Development Workflow

### Running Tests
Each problem has its own tests. To run tests for a specific problem:
```bash
cd problems/<problem_name>
cargo test
```

To run all tests in the workspace:
```bash
cargo test --workspace
```

### Adding New Problems
1. Create a new directory under `problems/`
2. Use `template.rs` as a starting point
3. Add the new crate to the workspace in the root `Cargo.toml`

### Code Quality Commands
- **Format code**: `cargo fmt`
- **Lint code**: `cargo clippy`
- **Check types**: `cargo check`

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