/**
 * [1036] Rotting Oranges
 *
 * In a given grid, each cell can have one of three values:
 *
 *
 * 	the value 0 representing an empty cell;
 * 	the value 1 representing a fresh orange;
 * 	the value 2 representing a rotten orange.
 *
 *
 * Every minute, any fresh orange that is adjacent (4-directionally) to a rotten orange becomes rotten.
 *
 * Return the minimum number of minutes that must elapse until no cell has a fresh orange.  If this is impossible, return -1 instead.
 *
 *  
 *
 * <div>
 * Example 1:
 *
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/02/16/oranges.png" style="width: 712px; height: 150px;" />
 *
 *
 * Input: <span id="example-input-1-1">[[2,1,1],[1,1,0],[0,1,1]]</span>
 * Output: <span id="example-output-1">4</span>
 *
 *
 * <div>
 * Example 2:
 *
 *
 * Input: <span id="example-input-2-1">[[2,1,1],[0,1,1],[1,0,1]]</span>
 * Output: <span id="example-output-2">-1</span>
 * Explanation:  The orange in the bottom left corner (row 2, column 0) is never rotten, because rotting only happens 4-directionally.
 *
 *
 * <div>
 * Example 3:
 *
 *
 * Input: <span id="example-input-3-1">[[0,2]]</span>
 * Output: <span id="example-output-3">0</span>
 * Explanation:  Since there are already no fresh oranges at minute 0, the answer is just 0.
 *
 *
 *  
 *
 * Note:
 *
 * <ol>
 * 	1 <= grid.length <= 10
 * 	1 <= grid[0].length <= 10
 * 	grid[i][j] is only 0, 1, or 2.
 * </ol>
 * </div>
 * </div>
 * </div>
 */
pub struct Solution {}

// submission codes start here

use std::collections::VecDeque;

fn has_fresh_orange(grid: &[Vec<i32>]) -> bool {
    grid.iter().any(|row| row.iter().any(|&val| val == 1))
}

fn is_valid(x: usize, y: usize, r: usize, c: usize) -> bool {
    x >= 0 && y >= 0 && x < r && y < c
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() || grid[0].is_empty() {
            return -1;
        }
        let mut grid = grid.clone();
        let r = grid.len();
        let c = grid[0].len();
        let mut queue: VecDeque<Option<Coord>> = VecDeque::new();
        let mut minutes: i32 = 0;
        for i in 0..r {
            for j in 0..c {
                if grid[i][j] == 2 {
                    queue.push_back(Some(Coord { x: i, y: j }));
                }
            }
        }
        queue.push_back(None);
        while !queue.is_empty() {
            let mut flag = false;
            while let Some(Some(temp)) = queue.pop_front() {
                let mut temp = temp.clone();
                if is_valid(temp.x + 1, temp.y, r, c) && grid[temp.x + 1][temp.y] == 1 {
                    if !flag {
                        minutes += 1;
                        flag = true;
                    }
                    grid[temp.x + 1][temp.y] = 2;
                    temp.x += 1;
                    queue.push_back(Some(temp.clone()));
                    temp.x -= 1;
                }
                if temp.x > 0 && is_valid(temp.x - 1, temp.y, r, c) && grid[temp.x - 1][temp.y] == 1
                {
                    if !flag {
                        minutes += 1;
                        flag = true;
                    }
                    grid[temp.x - 1][temp.y] = 2;
                    temp.x -= 1;
                    queue.push_back(Some(temp.clone()));
                    temp.x += 1;
                }
                if is_valid(temp.x, temp.y + 1, r, c) && grid[temp.x][temp.y + 1] == 1 {
                    if !flag {
                        minutes += 1;
                        flag = true;
                    }
                    grid[temp.x][temp.y + 1] = 2;
                    temp.y += 1;
                    queue.push_back(Some(temp.clone()));
                    temp.y -= 1;
                }
                if temp.y > 0 && is_valid(temp.x, temp.y - 1, r, c) && grid[temp.x][temp.y - 1] == 1
                {
                    if !flag {
                        minutes += 1;
                        flag = true;
                    }
                    grid[temp.x][temp.y - 1] = 2;
                    temp.y -= 1;
                    queue.push_back(Some(temp.clone()));
                    temp.y += 1;
                }
            }
            if !queue.is_empty() {
                queue.push_back(None);
            }
        }
        if has_fresh_orange(&grid) {
            -1
        } else {
            minutes
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1036() {
        assert_eq!(
            4,
            Solution::oranges_rotting(vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]])
        );
        assert_eq!(
            -1,
            Solution::oranges_rotting(vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]])
        );
        assert_eq!(0, Solution::oranges_rotting(vec![vec![0, 2]]));
    }
}
