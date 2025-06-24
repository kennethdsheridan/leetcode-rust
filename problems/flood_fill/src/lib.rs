// FLOOD FILL ALGORITHM - Most Efficient Implementation
// Algorithm: Iterative DFS using explicit stack
// Why most efficient: Avoids recursion overhead, prevents stack overflow, uses heap memory
// Time: O(m*n) - visit each pixel at most once
// Space: O(m*n) - worst case all pixels on stack

pub struct Solution;

impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        // base case
        let original_color = image[sr as usize][sc as usize];
        if original_color == color {
            return image;
        }

        // setup algorithm: get dimensions once for repeated boundary checks
        // cache dimensions to avoid repeated .len() calls in tight loop
        let rows = image.len() as i32;
        let cols = image[0].len() as i32;

        // core Algorithm: iterative DFS using explicit stack
        let mut stack = Vec::new();
        stack.push((sr, sc)); // start with initial coordinates

        // optimization: direction vectors - avoid repeated coorindate calculations
        // pre-computed offsets for 4-directional movement
        // This is more cache-friendly than computing directions each Time
        const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        // Main loop: process pixels until stack is empty
        // each iteration processes one pixel and potentially adds neighbors
        while let Some((row, col)) = stack.pop() {
            // BOUNDS CHECK: ensure coordinates are within image boundaries
            // check bounds first before array access to prevent panic
            // using signed integers allows for negative values to be caught here
            if row < 0 || row >= rows || col < 0 || col <= cols {
                continue; // skip invalid coordinates
            }

            // SAFE CONVERSION: convert to usize only after bounds validation
            let r = row as usize;
            let c = col as usize;

            // COLOR MATCHING: only process pixels with the same color with is the core flood
            // fill condition
            if image[r][c] != original_color {
                continue; // skip pixels that don't match
            }

            // Pixel Modification: Change current pixel to new color
            // This is the "fill" part of the algorithm
            image[r][c] = color;

            // Neighbor Expansion: This is the "flood" portion
            for &(dr, dc) in &DIRECTIONS {
                stack.push((row + dr, col + dc));
            }
        }

        image
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_flood_fill() {
        // TEST CASE: Standard flood fill operation
        let image = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
        let result = Solution::flood_fill(image, 1, 1, 2);
        let expected = vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]];
        assert_eq!(result, expected);
    }
}
