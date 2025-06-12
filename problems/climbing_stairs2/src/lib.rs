pub struct Solution;

impl Solution {
    pub fn climbing_stairs(n: i32) -> i32 {
        // Your solution here
        if n <= 2 {
            return n;
        }
        // init variables to track ways to reach the last two steps
        let mut ways_to_step_1 = 1;
        let mut ways_to_step_2 = 2;

        // iterate from step 3 up to step n
        for current_step in 3..=n {
            // calculate the ways to the current steps
            // using the Fibonacci formula
            let ways_to_current_step = ways_to_step_1 + ways_to_step_2;

            // sfhit the tracking variables forward for the next iteration
            ways_to_step_1 = ways_to_step_2;
            ways_to_step_2 = ways_to_current_step;
        }
        ways_to_step_2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        // Add your test cases here
        assert_eq!(Solution::climbing_stairs(3), 3);
    }
}
