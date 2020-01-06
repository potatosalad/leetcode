/**
 * [0017] Letter Combinations of a Phone Number
 *
 * Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent.
 *
 * A mapping of digit to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.
 *
 * <img src="http://upload.wikimedia.org/wikipedia/commons/thumb/7/73/Telephone-keypad2.svg/200px-Telephone-keypad2.svg.png" />
 *
 * Example:
 *
 *
 * Input: "23"
 * Output: ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"].
 *
 *
 * Note:
 *
 * Although the above answer is in lexicographical order, your answer could be in any order you want.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let table: [Vec<char>; 10] = [
            vec![],                   // 0
            vec![],                   // 1
            vec!['a', 'b', 'c'],      // 2
            vec!['d', 'e', 'f'],      // 3
            vec!['g', 'h', 'i'],      // 4
            vec!['j', 'k', 'l'],      // 5
            vec!['m', 'n', 'o'],      // 6
            vec!['p', 'q', 'r', 's'], // 7
            vec!['t', 'u', 'v'],      // 8
            vec!['w', 'x', 'y', 'z'], // 9
        ];
        let mut combinations = vec![String::with_capacity(digits.len())];
        for cs in digits
            .chars()
            .filter_map(|d| d.to_digit(10))
            .filter_map(|i| table.get(i as usize))
            .filter(|cs| !cs.is_empty())
        {
            let possibilities = cs.len() - 1;
            let mut new_combinations: Vec<String> =
                Vec::with_capacity(possibilities * combinations.len());
            for combination in combinations.iter_mut() {
                for (i, &c) in cs.iter().enumerate() {
                    if i == possibilities {
                        combination.push(c);
                    } else {
                        let mut new_combination = (*combination).clone();
                        new_combination.push(c);
                        new_combinations.push(new_combination);
                    }
                }
            }
            combinations.append(&mut new_combinations);
        }
        if combinations.len() == 1 && combinations[0].is_empty() {
            vec![]
        } else {
            combinations
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0017() {
        assert_eq!(
            vec!["cf", "af", "bf", "cd", "ce", "ad", "ae", "bd", "be"],
            Solution::letter_combinations("23".to_string())
        );

        assert!(Solution::letter_combinations("".to_string()).is_empty());
    }
}
