pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashSet;

        // for safe accessing of the characters in the string
        let chars: Vec<char> = s.chars().collect();

        // stores the max valid substring length found so far
        let mut max_length = 0;

        // left side of the sliding window - tracks strat of valid substring
        let mut left = 0;

        // HasSet to keep track of which characters are in current window
        let mut char_set = HashSet::new();

        // right pointer expands our window - iterates through each character
        // 0..chars.len() creates range [0, length] - exclusive end
        for right in 0..chars.len() {
            //shrink window from left while we have duplicate characters
            // keep removing leftmost characters until duplicate is gone
            while char_set.contains(&chars[right]) {
                // remove the leftmost character from our tracking set
                // &chars[left] borrows the charater
                char_set.remove(&chars[left]);

                // move left boundary one position right to shring window
                left += 1;
            }

            // now safe to add current character - no duplicates in window
            char_set.insert(chars[right]);

            // calculate current window size and update max if larger
            // right - left + 1 = window size (+1 because indices are 0-based)
            // .max() returns larger of current max_length vs current window size
            max_length = max_length.max(right - left + 1);
        }

        max_length as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        // Add your test cases here
        assert_eq!(
            Solution::length_of_longest_substring("abcdddefsfcadssdccdeeaa".to_string()),
            5
        );
    }
}
