# Create a new LeetCode problem
new problem_name:
    mkdir -p problems/{{problem_name}}/src
    printf '[package]\nname = "{{problem_name}}"\nversion = "0.1.0"\nedition = "2021"\n' > problems/{{problem_name}}/Cargo.toml
    cp template.rs problems/{{problem_name}}/src/lib.rs
    echo "Created problem: {{problem_name}}"
    echo "Edit with: $EDITOR problems/{{problem_name}}/src/lib.rs"

# Test a specific problem
test problem_name:
    cargo test -p {{problem_name}}

# Test all problems
test-all:
    cargo test

# Run a specific problem (if it has a main function)
run problem_name:
    cargo run -p {{problem_name}}

# Watch and test a specific problem
watch problem_name:
    cargo watch -x "test -p {{problem_name}}"

# List all problems
list:
    @echo "Available problems:"
    @ls -1 problems/ 2>/dev/null || echo "No problems created yet"

# Clean build artifacts
clean:
    cargo clean
