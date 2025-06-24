# Create New LeetCode Problem

Create a new LeetCode problem solution with proper structure and add it to the workspace.

## Usage
```
/lc:new <problem-name>
```

## Description
This command creates a new LeetCode problem solution by:
1. Running `just new $ARGUMENTS` to create the problem structure
2. Opening the generated file for editing
3. Reminding to add the problem to the workspace in root Cargo.toml

## Implementation
```bash
!just new $ARGUMENTS
```

After running this command, remember to:
1. Add the new problem to the `members` array in the root `Cargo.toml`
2. Implement your solution in the generated `src/lib.rs` file
3. Add appropriate test cases