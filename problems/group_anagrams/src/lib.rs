/*
49. Group Anagrams
Medium

Given an array of strings strs, group the anagrams together.
You can return the answer in any order.

Time Complexity: O(n × m) where n = number of strings, m = max string length
Space Complexity: O(n × m) for storing all strings in the HashMap

Key insight: Two strings are anagrams if they have the same character frequencies.
We use character counting instead of sorting for better performance.
*/

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // Pre-allocate HashMap capacity to reduce rehashing
        // Why: We know we'll have at most strs.len() groups, so this prevents
        // multiple memory reallocations as the HashMap grows
        let mut groups: HashMap<[u8; 26], Vec<String>> = HashMap::with_capacity(strs.len());

        for s in strs {
            // Use fixed-size array for character frequency counting
            // Why: [u8; 26] is Copy trait, making it efficient as HashMap key
            // Why: u8 is sufficient since max string length is 100 per constraints
            let mut char_count = [0u8; 26];

            // Count character frequencies using bytes() instead of chars()
            // Why: Problem states "lowercase English letters only", so ASCII bytes work
            // Why: bytes() is faster than chars() since no UTF-8 decoding needed
            for byte in s.bytes() {
                // Convert ASCII byte to array index (a=0, b=1, ..., z=25)
                // Why: Direct array indexing is O(1) vs HashMap lookup which could be slower
                char_count[(byte - b'a') as usize] += 1;
            }

            // Use entry().or_default() pattern for clean insert-or-update
            // Why: Avoids separate contains_key() check followed by insert/get_mut
            // Why: or_default() creates empty Vec if key doesn't exist
            groups.entry(char_count).or_default().push(s);
        }

        // Convert HashMap values to Vec, consuming the HashMap
        // Why: into_values() takes ownership, avoiding cloning
        // Why: collect() converts iterator to Vec as required by return type
        groups.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to normalize results for testing
    // Why: LeetCode allows returning groups in any order, so we need order-independent comparison
    fn normalize_result(mut result: Vec<Vec<String>>) -> Vec<Vec<String>> {
        // Sort each group internally
        for group in &mut result {
            group.sort();
        }
        // Sort groups by their first element for consistent comparison
        result.sort_by(|a, b| a[0].cmp(&b[0]));
        result
    }

    #[test]
    fn test_example_1() {
        let input = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        let result = Solution::group_anagrams(input);
        let normalized = normalize_result(result);

        let expected = vec![
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
            vec!["bat".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
        ];
        let expected_normalized = normalize_result(expected);

        assert_eq!(normalized, expected_normalized);
    }

    #[test]
    fn test_example_2_empty_string() {
        let input = vec!["".to_string()];
        let result = Solution::group_anagrams(input);
        assert_eq!(result, vec![vec!["".to_string()]]);
    }

    #[test]
    fn test_example_3_single_char() {
        let input = vec!["a".to_string()];
        let result = Solution::group_anagrams(input);
        assert_eq!(result, vec![vec!["a".to_string()]]);
    }

    #[test]
    fn test_no_anagrams() {
        let input = vec!["abc".to_string(), "def".to_string(), "ghi".to_string()];
        let result = Solution::group_anagrams(input);
        // Each string should be in its own group
        assert_eq!(result.len(), 3);
        for group in result {
            assert_eq!(group.len(), 1);
        }
    }

    #[test]
    fn test_all_anagrams() {
        let input = vec!["abc".to_string(), "bca".to_string(), "cab".to_string()];
        let result = Solution::group_anagrams(input);
        // All should be in one group
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 3);
    }

    #[test]
    fn test_edge_case_duplicates() {
        let input = vec!["abc".to_string(), "abc".to_string()];
        let result = Solution::group_anagrams(input);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 2);
    }
}

/*
Alternative Approaches Considered:

1. Sorting Approach:
   ```rust
   let mut chars: Vec<char> = s.chars().collect();
   chars.sort_unstable();
   let key: String = chars.into_iter().collect();
   ```
   Why not chosen: O(m log m) per string vs O(m) for counting

2. Prime Number Hashing:
   ```rust
   let primes = [2, 3, 5, 7, 11, ...]; // 26 primes
   let mut hash = 1u64;
   for c in s.chars() {
       hash *= primes[(c as u8 - b'a') as usize];
   }
   ```
   Why not chosen: Risk of integer overflow, more complex, no significant benefit

3. String as Key After Sorting:
   Why not chosen: String allocation overhead vs fixed array

Performance Notes:
- Character counting: O(n × m) time, O(n × m) space - optimal for this problem
- Fixed array key: O(1) hash computation, Copy trait benefits
- Pre-allocated HashMap: Reduces rehashing overhead
- bytes() iteration: Faster than chars() for ASCII-only input
*/
