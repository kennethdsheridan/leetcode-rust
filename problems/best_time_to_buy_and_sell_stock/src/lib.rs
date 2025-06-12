pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // Your solution here
        // edge case: need at least two days to make a transaction
        if prices.len() < 2 {
            return 0;
        }
        // track two key states as we iterate through the prices
        let mut lowest_price_so_far = prices[0]; // cheapeast price we've seen so lowest_price_so_far
        let mut max_profit_so_far = 0; // Best profit we've seen so far.

        // iterate through all prices starting from day 1 (since we have day 0 already)
        for current_day in 1..prices.len() {
            let today_price = prices[current_day];

            // calculate the potential profit if we sold today at teh best buy price we've found
            let profit_if_sell_today = today_price - lowest_price_so_far;

            // update our max profit if selling today gives us a better deal
            max_profit_so_far = max_profit_so_far.max(profit_if_sell_today);

            // update our lowest price if todays price is the cheapest we've seen
            // this gives us potentially better buying opportunitiy for future days
            lowest_price_so_far = lowest_price_so_far.min(today_price);
        }
        //return the max profit we found across all possible buy/sell combinations
        max_profit_so_far
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        // Add your test cases here
        assert_eq!(Solution::max_profit(vec![30, 34, 50, 34, 68, 69]), 50);
        assert_eq!(Solution::max_profit(vec![5]), 0); // can't buy any stock
    }
}
