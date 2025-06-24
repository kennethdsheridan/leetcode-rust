
pub struct Solution;

impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {        // Your solution here
        let original_color = image[sr as usize][sc as usize];

        if original_color == color {
            return image;
        }
        
        // define the search area
        let rows = image.len() as i32;
        let cols = image[0].len() as i32;

        let mut stack = Vec::new();
        stack.push((sr, sc)); 

        // add directions
        const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        while let Some((row, col)) =  stack.pop() {
            if row < 0 || row >= rows || col < 0 || col >= cols {
                continue;
            }

            let r = row as usize;
            let c = col as usize;

            if image[r][c] != original_color {
                continue;
            }

            // modify the pixel "Fill"
            image[r][c] = color;

            // "Flood" the neighbors
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
    fn test_example() {
        let image = vec![vec![1,1,1,], vec![1,1,0], vec![1, 0, 1]];
        let result = Solution::flood_fill(image, 1, 1, 2);
        let expected = vec![vec![2,2,2], vec![2,2,0], vec![2,0,1]];
        assert_eq!(result, expected);
    }
}
