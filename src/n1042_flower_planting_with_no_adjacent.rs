/**
 * [1042] Flower Planting With No Adjacent
 *
 * You have N gardens, labelled 1 to N.  In each garden, you want to plant one of 4 types of flowers.
 *
 * paths[i] = [x, y] describes the existence of a bidirectional path from garden x to garden y.
 *
 * Also, there is no garden that has more than 3 paths coming into or leaving it.
 *
 * Your task is to choose a flower type for each garden such that, for any two gardens connected by a path, they have different types of flowers.
 *
 * Return any such a choice as an array answer, where answer[i] is the type of flower planted in the (i+1)-th garden.  The flower types are denoted <font face="monospace">1</font>, <font face="monospace">2</font>, <font face="monospace">3</font>, or <font face="monospace">4</font>.  It is guaranteed an answer exists.
 *
 *  
 *
 *
 * Example 1:
 *
 *
 * Input: N = 3, paths = [[1,2],[2,3],[3,1]]
 * Output: [1,2,3]
 *
 *
 *
 * Example 2:
 *
 *
 * Input: N = 4, paths = [[1,2],[3,4]]
 * Output: [1,2,1,2]
 *
 *
 *
 * Example 3:
 *
 *
 * Input: N = 4, paths = [[1,2],[2,3],[3,4],[4,1],[1,3],[2,4]]
 * Output: [1,2,3,4]
 *
 *
 *  
 *
 * Note:
 *
 *
 * 	1 <= N <= 10000
 * 	0 <= paths.size <= 20000
 * 	No garden has 4 or more paths coming into or leaving it.
 * 	It is guaranteed an answer exists.
 *
 *
 *
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn garden_no_adj(n: i32, paths: Vec<Vec<i32>>) -> Vec<i32> {
        let mut answer: Vec<i32> = vec![0; n as usize];
        let mut graph: Vec<Vec<i32>> = vec![vec![]; n as usize];
        for path in paths {
            let u = path[0];
            let v = path[1];
            if u < v {
                graph[(v - 1) as usize].push(u - 1);
            } else {
                graph[(u - 1) as usize].push(v - 1);
            }
        }
        for i in 0..(n as usize) {
            if i >= graph.len() || graph[i].is_empty() {
                answer[i] = 1;
            } else {
                let exclude: Vec<i32> = graph[i].iter().map(|&l| answer[l as usize]).collect();
                answer[i] = (1_i32..5_i32)
                    .filter(|j| !exclude.contains(j))
                    .nth(0)
                    .unwrap();
            }
        }
        answer
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1042() {
        assert_eq!(
            vec![1, 2, 3],
            Solution::garden_no_adj(3, vec![vec![1, 2], vec![2, 3], vec![3, 1]])
        );
        assert_eq!(
            vec![1, 2, 1, 2],
            Solution::garden_no_adj(4, vec![vec![1, 2], vec![3, 4]])
        );
        assert_eq!(
            vec![1, 2, 3, 4],
            Solution::garden_no_adj(
                4,
                vec![
                    vec![1, 2],
                    vec![2, 3],
                    vec![3, 4],
                    vec![4, 1],
                    vec![1, 3],
                    vec![2, 4]
                ]
            )
        );
    }
}
