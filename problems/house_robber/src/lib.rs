pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        // Your solution here
        // if we have no houses or only one house
        if nums.is_empty() {
            return 0; // no houses to rob
        }
        if nums.len() == 1 {
            return nums[0]; // only one house to rob
        }

        // track the max amount of money we can rob up to the previous two houses
        // this follows the pattern: max_at_1 = max(max_at_i-1, max_at_i-2 + nums[i])
        let mut max_two_houses_ago = nums[0]; // Max money if we only had house 0
        let mut max_one_house_ago = nums[0].max(nums[1]); // max money from houses 0 and 1 

        // iterate through remaining houses starting from house 3 (index 2)
        for current_house_index in 2..nums.len() {
            // calculate the max money if we include the current house 
            // we can rob current house + best we could do 2 houses ago
            let rob_current_house = max_two_houses_ago + nums[current_house_index];

            // calculate max money if we skip current houses
            // we take the best we could do up to the previous house
            let skip_current_house = max_one_house_ago;

            // take the maximum of robbing or skipping current house
            let max_at_current_house = rob_current_house.max(skip_current_house);

            // shift the tracking variables forward for the next iteration
            max_two_houses_ago = max_one_house_ago; // Old "one ago" becomes "two ago" 
            max_one_house_ago = max_at_current_house; // current max becomes "one ago"
        }
        // return the maximum amount of money we can rob considering all houses 
        max_one_house_ago

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        // Add your test cases here
        assert_eq!(Solution::rob(), 0);
    }
}
