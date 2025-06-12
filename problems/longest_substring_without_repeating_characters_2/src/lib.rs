pub struct Solution;

impl Solution {
    pub fn longest_substring(s: String) -> i32 {
        use std::collections::HashSet;

        // define the variables for window
        let chars: Vec<char> = s.chars().collect();

        let mut max_length = 0;
        let mut left = 0;
        let mut char_set = HashSet::new();

        for right in 0..chars.len() {
            while char_set.contains(&chars[right]) {
                char_set.remove(&chars[left]);

                left += 1;
            }

            char_set.insert(chars[right]);

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
        assert_eq!(Solution::longest_substring("abcdeessefcs".to_string()), 5);
    }
}
