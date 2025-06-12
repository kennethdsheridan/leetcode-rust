// Define a public struct to hold our solution methods (LeetCode convention)
pub struct Solution;

// Implementation block containing our solution methods
impl Solution {
    // Main function: calculates number of distinct ways to climb n stairs
    // Takes 1 or 2 steps at a time, returns total number of ways
    pub fn climb_stairs(n: i32) -> i32 {
        
        // Handle base cases: if we have 1 or 2 steps total
        // For n=1: only 1 way [1]
        // For n=2: 2 ways [1,1] or [2]
        if n <= 2 {
            return n; // Return the number itself (1 for n=1, 2 for n=2)
        }
        
        // Initialize variables to track ways to reach the last two steps
        // This follows the pattern: ways(n) = ways(n-1) + ways(n-2)
        let mut ways_to_step_1 = 1; // There's 1 way to reach step 1: [1]
        let mut ways_to_step_2 = 2; // There are 2 ways to reach step 2: [1,1] or [2]
        
        // Iterate from step 3 up to step n (inclusive)
        // We start at 3 because we already know steps 1 and 2
        for current_step in 3..=n {
            
            // Calculate ways to reach current step using Fibonacci formula
            // To reach current step, we can come from (current-1) OR (current-2)
            let ways_to_current_step = ways_to_step_2 + ways_to_step_1;
            
            // Shift our tracking variables forward for the next iteration
            // The old "step 2" becomes our new "step 1" (we moved forward one step)
            ways_to_step_1 = ways_to_step_2;
            
            // Our current calculation becomes the new "step 2" for next iteration  
            ways_to_step_2 = ways_to_current_step;
        }
        
        // After the loop, ways_to_step_2 contains the number of ways to reach step n
        ways_to_step_2
    }
}

// Conditional compilation: only include tests when running tests
#[cfg(test)]
mod tests {
    // Import all items from the parent module (brings Solution into scope)
    use super::*;
    
    // Test function to verify our solution works correctly
    #[test]
    fn test_example() {
        
        // Test case 1: 2 steps
        // Expected ways: [1,1] and [2] = 2 total ways
        assert_eq!(Solution::climb_stairs(2), 2);
        
        // Test case 2: 3 steps  
        // Expected ways: [1,1,1], [1,2], [2,1] = 3 total ways
        assert_eq!(Solution::climb_stairs(3), 3);
        
        // Test case 3: Base case with 1 step
        // Expected ways: [1] = 1 total way
        assert_eq!(Solution::climb_stairs(1), 1);
        
        // Test case 4: Larger example with 4 steps
        // Expected ways: [1,1,1,1], [1,1,2], [1,2,1], [2,1,1], [2,2] = 5 total ways
        assert_eq!(Solution::climb_stairs(4), 5);
        
        // Test case 5: Another larger example with 5 steps
        // Following Fibonacci: 1,2,3,5,8 so for n=5 answer is 8
        assert_eq!(Solution::climb_stairs(5), 8);
    }
}

/*
ALGORITHM EXPLANATION:

🔢 This is a Dynamic Programming problem that follows the Fibonacci sequence!

📋 The key insight:
   To reach step n, you can only arrive from:
   - Step (n-1) by taking 1 step
   - Step (n-2) by taking 2 steps
   
   Therefore: ways(n) = ways(n-1) + ways(n-2)

⚡ Time Complexity: O(n) - we iterate through n-2 steps
💾 Space Complexity: O(1) - we only use 3 variables regardless of input size

🎯 Why this approach is optimal:
   - No recursion (avoids stack overflow)
   - No arrays (saves memory)
   - Only calculates each value once (efficient)
   - Handles the constraint 1 <= n <= 45 perfectly

📊 Trace example for n=5:
   Start: ways_to_step_1=1, ways_to_step_2=2
   
   Step 3: ways_to_current_step = 2+1 = 3
          ways_to_step_1=2, ways_to_step_2=3
   
   Step 4: ways_to_current_step = 3+2 = 5  
          ways_to_step_1=3, ways_to_step_2=5
   
   Step 5: ways_to_current_step = 5+3 = 8
          ways_to_step_1=5, ways_to_step_2=8
   
   Result: 8 ways to reach step 5
*/
