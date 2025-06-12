pub struct Solution;

impl Solution {
    pub fn is_palindrome_string(input: i32) -> bool {
        // Your solution here
        // negative numbers are not palindroes
        if input < 0 {
            return false;
        }
        let s = input.to_string();
        let reversed = s.chars().rev().collect::<String>();
        s == reversed
    }

    pub fn is_palindrome(input: i32) -> bool {
        // negative numbers and numbers ending in 0 (except 0 itself) are not palindrome
        if input < 0 || (input % 10 == 0 && input != 0) {
            return false;
        }
        let mut num = input;
        let mut reversed_half = 0;

        // only reverse half the numbers
        while num > reversed_half {
            reversed_half = reversed_half * 10 + num % 10;
            num /= 10;
        }
        // for even length: num == reversed_half
        // for odd length: num == reversed_half / 10 (middle digit doesn't matter)
        num == reversed_half || num == reversed_half / 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_palindromes() {
        // Add your test cases here
        assert_eq!(Solution::is_palindrome(121), true);
    }
    #[test]
    fn test_negative_palindromes() {
        assert_eq!(Solution::is_palindrome(-333), false);
    }
}
