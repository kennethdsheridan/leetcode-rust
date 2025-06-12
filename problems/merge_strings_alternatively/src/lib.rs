pub struct Solution;

impl Solution {
    pub fn merge_strings(word1: String, word2: String) -> String {
        let mut result = String::with_capacity(word1.len() + word2.len());

        // Get the iterators for both words
        let mut chars1 = word1.chars();
        let mut chars2 = word2.chars();

        loop {
            match chars1.next() {
                Some(c1) => {
                    result.push(c1);
                    if let Some(c2) = chars2.next() {
                        result.push(c2);
                    }
                }
                None => {
                    result.extend(chars2); // if word1 is exhausted, append remaining characteres
                    break;
                }
            }
        }

        // FIXED: Remove "as String" - result is already a String
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        // FIXED: Add proper test cases with actual parameters
        assert_eq!(
            Solution::merge_strings("abc".to_string(), "pqr".to_string()),
            "apbqcr".to_string()
        );

        assert_eq!(
            Solution::merge_strings("ab".to_string(), "pqrs".to_string()),
            "apbqrs".to_string()
        );

        assert_eq!(
            Solution::merge_strings("abcd".to_string(), "pq".to_string()),
            "apbqcd".to_string()
        );

        // Edge cases
        assert_eq!(
            Solution::merge_strings("".to_string(), "abc".to_string()),
            "abc".to_string()
        );

        assert_eq!(
            Solution::merge_strings("abc".to_string(), "".to_string()),
            "abc".to_string()
        );
    }
}
