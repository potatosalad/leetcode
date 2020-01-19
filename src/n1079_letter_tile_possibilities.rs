/**
 * [1079] Letter Tile Possibilities
 *
 * You have a set of tiles, where each tile has one letter tiles[i] printed on it.  Return the number of possible non-empty sequences of letters you can make.
 *
 *  
 *
 * Example 1:
 *
 *
 * Input: "AAB"
 * Output: 8
 * Explanation: The possible sequences are "A", "B", "AA", "AB", "BA", "AAB", "ABA", "BAA".
 *
 *
 *
 * Example 2:
 *
 *
 * Input: "AAABBC"
 * Output: 188
 *
 *
 *  
 *
 *
 * Note:
 *
 *
 * 	1 <= tiles.length <= 7
 * 	tiles consists of uppercase English letters.
 *
 */
pub struct Solution {}

// submission codes start here

fn histogram_permutations(buckets: &mut Vec<i32>) -> i32 {
    let mut results: i32 = 0;
    for i in 0..26 {
        if buckets[i] < 1 {
            continue;
        }
        buckets[i] -= 1;
        results += 1 + histogram_permutations(buckets);
        buckets[i] += 1;
    }
    results
}

impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut histogram: Vec<i32> = vec![0; 26];
        for c in tiles.chars() {
            histogram[c as usize - 'A' as usize] += 1;
        }
        histogram_permutations(&mut histogram)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1079() {
        assert_eq!(8, Solution::num_tile_possibilities("AAB".to_string()));
        assert_eq!(188, Solution::num_tile_possibilities("AAABBC".to_string()));
    }
}
