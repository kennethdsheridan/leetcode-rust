# LeetCode Rust Solutions - Project Context for Claude

## Project Overview
This is a collection of LeetCode problem solutions implemented in Rust. Each problem is organized as a separate Rust crate within the `problems/` directory. The project uses Nix for reproducible development environments and `just` command runner for task automation.

## Project Structure
```
leetcode-rust/
├── .claude/              # Claude Code configuration
│   └── commands/         # Custom slash commands
├── problems/             # Individual problem solutions
│   ├── two_sum/
│   ├── palindrome_number/
│   ├── longest_substring_without_repeating_characters/
│   ├── binary-tree-inorder-traversal/
│   ├── flood_fill/
│   ├── climbing_stairs/
│   ├── house_robber/
│   ├── merge_strings_alternatively/
│   ├── reverse_words_in_string/
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

### Code Quality Commands
```bash
cargo fmt              # Format all code
cargo clippy           # Run linter
cargo check            # Type check all code
```

## Claude Code Integration

### Custom Slash Commands
This project includes custom slash commands in `.claude/commands/` for efficient development:

- `/lc:new <problem-name>` - Create a new LeetCode problem solution
- `/lc:test <problem-name>` - Test a specific problem
- `/lc:watch <problem-name>` - Auto-test a problem on file changes
- `/lc:fmt` - Format all Rust code
- `/lc:check` - Run full type checking and linting
- `/lc:clean` - Clean build artifacts
- `/lc:list` - List all problems in the project

### Memory Management
This file serves as the primary context for Claude Code's memory system. Key information:

1. **Project Type**: LeetCode problem solutions in Rust
2. **Build System**: Cargo workspace with individual problem crates
3. **Development Environment**: Nix flake with just command runner
4. **Testing Strategy**: Each problem has unit tests with LeetCode examples
5. **Code Style**: Rust 2021 edition, formatted with `cargo fmt`

### AI Assistant Guidelines
When working on this project:

1. **Problem Creation**: Always use `just new <name>` or `/lc:new <name>` 
2. **Testing**: Run tests with `just test <name>` or `/lc:test <name>`
3. **Code Quality**: Format with `cargo fmt` and check with `cargo clippy`
4. **Workspace Management**: Remember to add new problems to root `Cargo.toml` members
5. **Solution Structure**: Each solution should be a `Solution` struct with appropriate methods
6. **Documentation**: Include problem description and constraints as comments

## Problem Solution Patterns

### Standard Structure
```rust
pub struct Solution;

impl Solution {
    pub fn solve(input: InputType) -> OutputType {
        // Implementation here
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // Test cases from LeetCode
    }
}
```

### Common Algorithm Categories
- **Arrays & Hashing**: Hash maps, frequency counting
- **Two Pointers**: In-place array manipulation
- **Sliding Window**: Substring/subarray problems
- **Dynamic Programming**: Optimization problems
- **Tree Traversal**: Binary tree problems
- **Graph Algorithms**: DFS, BFS, flood fill

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

**Manual addition to workspace**:
After creating a new problem, add it to the workspace in root `Cargo.toml`:
```toml
members = [
    # ... existing members
    "problems/problem_name",
]
```

## Git Workflow
- Main branch: `main`
- Commit messages should describe the problem being solved or improved
- Each problem solution should be well-tested before committing
- Use descriptive commit messages: "Add two sum solution" or "Optimize climbing stairs DP"

## Performance Considerations
- Include time/space complexity in comments
- Consider edge cases in tests
- Use appropriate Rust idioms (iterators, pattern matching)
- Prefer `&str` over `String` when possible for string problems
- Use `Vec::with_capacity()` when size is known

## Testing Strategy
Each problem should include:
- Example test cases from LeetCode description
- Edge cases (empty input, single element, etc.)
- Performance stress tests for large inputs when relevant
- Corner cases specific to the algorithm used