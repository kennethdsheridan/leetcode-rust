pub struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace() // split on any whitepace (spaces, tabs, newlines)
            .rev() // reverse the iterator of words
            .collect::<Vec<&str>>() // collect into vector of string slices
            .join(" ") // join with single space
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        // Add your test cases here
        assert_eq!(
            Solution::reverse_words("The sky is blue".to_string()),
            "blue is sky The".to_string()
        );
    }
}
