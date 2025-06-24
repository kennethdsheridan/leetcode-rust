# Check Code Quality

Run type checking and linting for all Rust code in the project.

## Usage
```
/lc:check
```

## Description
This command runs both type checking and linting to ensure code quality across all problem solutions.

## Implementation
```bash
!cargo check && cargo clippy
```

This helps catch potential issues and maintain high code quality standards.