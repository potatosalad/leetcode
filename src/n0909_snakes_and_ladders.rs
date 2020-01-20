/**
 * [0909] Snakes and Ladders
 *
 * On an N x N board, the numbers from 1 to N*N are written boustrophedonically starting from the bottom left of the board, and alternating direction each row.  For example, for a 6 x 6 board, the numbers are written as follows:
 *
 *
 *
 *
 *
 * You start on square 1 of the board (which is always in the last row and first column).  Each move, starting from square x, consists of the following:
 *
 *
 * 	You choose a destination square S with number x+1, x+2, x+3, x+4, x+5, or x+6, provided this number is <= N*N.
 *
 *
 * 		(This choice simulates the result of a standard 6-sided die roll: ie., there are always at most 6 destinations, regardless of the size of the board.)
 *
 *
 * 	If S has a snake or ladder, you move to the destination of that snake or ladder.  Otherwise, you move to S.
 *
 *
 * A board square on row r and column c has a "snake or ladder" if board[r][c] != -1.  The destination of that snake or ladder is board[r][c].
 *
 * Note that you only take a snake or ladder at most once per move: if the destination to a snake or ladder is the start of another snake or ladder, you do not continue moving.  (For example, if the board is `[[4,-1],[-1,3]]`, and on the first move your destination square is `2`, then you finish your first move at `3`, because you do not continue moving to `4`.)
 *
 * Return the least number of moves required to reach square <font face="monospace">N*N</font>.  If it is not possible, return -1.
 *
 * Example 1:
 *
 *
 * Input: [
 * [-1,-1,-1,-1,-1,-1],
 * [-1,-1,-1,-1,-1,-1],
 * [-1,-1,-1,-1,-1,-1],
 * [-1,35,-1,-1,13,-1],
 * [-1,-1,-1,-1,-1,-1],
 * [-1,15,-1,-1,-1,-1]]
 * Output: 4
 * Explanation:
 * At the beginning, you start at square 1 [at row 5, column 0].
 * You decide to move to square 2, and must take the ladder to square 15.
 * You then decide to move to square 17 (row 3, column 5), and must take the snake to square 13.
 * You then decide to move to square 14, and must take the ladder to square 35.
 * You then decide to move to square 36, ending the game.
 * It can be shown that you need at least 4 moves to reach the N*N-th square, so the answer is 4.
 *
 *
 * Note:
 *
 *
 * 	2 <= board.length = board[0].length <= 20
 * 	board[i][j] is between 1 and N*N or is equal to -1.
 * 	The board square with number 1 has no snake or ladder.
 * 	The board square with number N*N has no snake or ladder.
 *
 *
 */
pub struct Solution {}

// submission codes start here

use std::collections::VecDeque;

impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let mut arr: Vec<i32> = vec![0; n * n];
        let mut i: usize = n - 1;
        let mut j: usize = 0;
        let mut index: usize = 0;
        let mut inc: i32 = 1;
        while index < n * n {
            arr[index] = board[i][j];
            index += 1;
            if inc == 1 && j == n - 1 {
                inc = -1;
                if i > 0 {
                    i -= 1;
                }
            } else if inc == -1 && j == 0 {
                inc = 1;
                if i > 0 {
                    i -= 1;
                }
            } else if inc == 1 {
                j += 1;
            } else if j > 0 {
                j -= 1;
            }
        }
        let mut visited: Vec<bool> = vec![false; n * n];
        let mut queue: VecDeque<usize> = VecDeque::new();
        let start: usize = if arr[0] > -1 {
            (arr[0] - 1) as usize
        } else {
            0
        };
        queue.push_back(start);
        visited[start] = true;
        let mut step = 0;
        while !queue.is_empty() {
            let mut size = queue.len();
            while size > 0 {
                size -= 1;
                let cur = queue.pop_front().unwrap();
                if cur == n * n - 1 {
                    return step;
                }
                for next in (cur + 1)..=usize::min(cur + 6, n * n - 1) {
                    let dest = if arr[next] > -1 {
                        (arr[next] - 1) as usize
                    } else {
                        next
                    };
                    if !visited[dest] {
                        visited[dest] = true;
                        queue.push_back(dest);
                    }
                }
            }
            step += 1;
        }
        -1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0909() {
        assert_eq!(
            4,
            Solution::snakes_and_ladders(vec![
                vec![-1, -1, -1, -1, -1, -1],
                vec![-1, -1, -1, -1, -1, -1],
                vec![-1, -1, -1, -1, -1, -1],
                vec![-1, 35, -1, -1, 13, -1],
                vec![-1, -1, -1, -1, -1, -1],
                vec![-1, 15, -1, -1, -1, -1]
            ])
        );
    }
}
