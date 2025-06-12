# LeetCode Rust Solutions

A collection of LeetCode problem solutions implemented in Rust, organized as a Cargo workspace with each problem in its own crate.

## Quick Start (Copy & Paste)

```bash
# Install Nix with flakes enabled (if you don't have it)
curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install

# Clone the repository
git clone https://github.com/kennethdsheridan/leetcode-rust.git
cd leetcode-rust

# Enter the development environment (includes Rust, just, cargo-watch)
nix develop

# Start coding!
just list              # See all problems
just new my_problem    # Create a new problem
just test two_sum      # Test a specific problem
just watch two_sum     # Auto-test on changes
```

## Problems Solved

- **Two Sum** - Hash map approach for finding two numbers that sum to a target
- **Palindrome Number** - Check if an integer reads the same backward as forward
- **Longest Substring Without Repeating Characters** - Sliding window technique
- **Best Time to Buy and Sell Stock** - Dynamic programming for maximum profit
- **Climbing Stairs** - Classic DP problem with Fibonacci-like solution
- **House Robber** - Dynamic programming for maximum non-adjacent sum
- **Merge Strings Alternately** - String manipulation with iterators
- **Reverse Words in a String** - String parsing and manipulation

## Getting Started

The quick start section above has everything you need. The Nix shell includes Rust, cargo-watch, just, and all necessary tools.

### Alternative Setup (Without Nix)

Prerequisites:
- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- Git
- Just command runner: `cargo install just`
- Cargo watch: `cargo install cargo-watch`

### Quick Commands

Once in the development environment:

```bash
just list              # List all problems
just new two_sum       # Create new problem
just test two_sum      # Test specific problem
just test-all          # Test all problems
just watch two_sum     # Auto-test on file changes
```

## Development

### Project Structure
```
.
├── problems/           # Individual problem solutions
│   └── <problem_name>/
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs
├── Cargo.toml         # Workspace configuration
├── template.rs        # Template for new problems
├── justfile          # Task automation commands
└── flake.nix         # Nix development environment
```

### Adding New Problems

Using just (recommended):
```bash
just new problem_name
```

This automatically:
- Creates the problem directory structure
- Copies the template
- Sets up Cargo.toml
- Shows you where to edit the solution

Manual approach:
1. Create a new directory: `mkdir -p problems/new_problem_name/src`
2. Copy the template: `cp template.rs problems/new_problem_name/src/lib.rs`
3. Add to workspace in root `Cargo.toml`:
   ```toml
   members = [
       # ... existing members
       "problems/new_problem_name",
   ]
   ```
4. Create `problems/new_problem_name/Cargo.toml`:
   ```toml
   [package]
   name = "new_problem_name"
   version = "0.1.0"
   edition = "2021"
   ```

### Code Quality
Format code:
```bash
cargo fmt
```

Run linter:
```bash
cargo clippy
```

Clean build artifacts:
```bash
just clean
# or using cargo:
cargo clean
```

## Testing Strategy

Each solution includes:
- Unit tests with LeetCode examples
- Edge case tests
- Performance considerations documented in comments

## Problem Categories

- **Arrays & Hashing**: Two Sum
- **Two Pointers**: Palindrome Number
- **Sliding Window**: Longest Substring Without Repeating Characters
- **Dynamic Programming**: Climbing Stairs, House Robber, Best Time to Buy and Sell Stock
- **String Manipulation**: Merge Strings Alternately, Reverse Words in a String

## Contributing

Feel free to add new solutions or improve existing ones:
1. Fork the repository
2. Create a feature branch
3. Add your solution with tests
4. Submit a pull request

## License

This project is open source and available under the MIT License.