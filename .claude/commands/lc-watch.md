# Watch LeetCode Problem

Auto-run tests for a LeetCode problem when files change.

## Usage
```
/lc:watch <problem-name>
```

## Description
This command starts a file watcher that automatically runs tests whenever you make changes to the problem's source code.

## Implementation
```bash
!just watch $ARGUMENTS
```

This is useful for TDD-style development where you want immediate feedback as you work on your solution.