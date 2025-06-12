# LeetCode Rust Solutions

A collection of LeetCode problem solutions implemented in Rust, organized as a Cargo workspace with each problem in its own crate.

## 📚 Problems Solved

- **Two Sum** - Hash map approach for finding two numbers that sum to a target
- **Palindrome Number** - Check if an integer reads the same backward as forward
- **Longest Substring Without Repeating Characters** - Sliding window technique
- **Best Time to Buy and Sell Stock** - Dynamic programming for maximum profit
- **Climbing Stairs** - Classic DP problem with Fibonacci-like solution
- **House Robber** - Dynamic programming for maximum non-adjacent sum
- **Merge Strings Alternately** - String manipulation with iterators
- **Reverse Words in a String** - String parsing and manipulation

## 🚀 Getting Started

### Prerequisites
- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- Git
- Just command runner (optional but recommended): `cargo install just`

### Clone the Repository
```bash
git clone https://github.com/kennethdsheridan/leetcode-rust.git
cd leetcode-rust
```

### Quick Commands with Just

List all available problems:
```bash
just list
```

Run tests for all problems:
```bash
just test-all
# or using cargo directly:
cargo test --workspace
```

Test a specific problem:
```bash
just test two_sum
# or using cargo:
cargo test -p two_sum
```

Watch and test a problem (auto-runs tests on file changes):
```bash
just watch palindrome_number
```

## 🛠️ Development

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

## 🧪 Testing Strategy

Each solution includes:
- Unit tests with LeetCode examples
- Edge case tests
- Performance considerations documented in comments

## 📈 Problem Categories

- **Arrays & Hashing**: Two Sum
- **Two Pointers**: Palindrome Number
- **Sliding Window**: Longest Substring Without Repeating Characters
- **Dynamic Programming**: Climbing Stairs, House Robber, Best Time to Buy and Sell Stock
- **String Manipulation**: Merge Strings Alternately, Reverse Words in a String

## 🤝 Contributing

Feel free to add new solutions or improve existing ones:
1. Fork the repository
2. Create a feature branch
3. Add your solution with tests
4. Submit a pull request

## 📝 License

This project is open source and available under the MIT License.