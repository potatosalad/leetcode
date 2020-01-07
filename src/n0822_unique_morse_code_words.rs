/**
 * [0822] Unique Morse Code Words
 *
 * International Morse Code defines a standard encoding where each letter is mapped to a series of dots and dashes, as follows: "a" maps to ".-", "b" maps to "-...", "c" maps to "-.-.", and so on.
 *
 * For convenience, the full table for the 26 letters of the English alphabet is given below:
 *
 *
 * [".-","-...","-.-.","-..",".","..-.","--.","....","..",".---","-.-",".-..","--","-.","---",".--.","--.-",".-.","...","-","..-","...-",".--","-..-","-.--","--.."]
 *
 * Now, given a list of words, each word can be written as a concatenation of the Morse code of each letter. For example, "cba" can be written as "-.-..--...", (which is the concatenation "-.-." + "-..." + ".-"). We'll call such a concatenation, the transformation of a word.
 *
 * Return the number of different transformations among all words we have.
 *
 *
 * Example:
 * Input: words = ["gin", "zen", "gig", "msg"]
 * Output: 2
 * Explanation:
 * The transformation of each word is:
 * "gin" -> "--...-."
 * "zen" -> "--...-."
 * "gig" -> "--...--."
 * "msg" -> "--...--."
 *
 * There are 2 different transformations, "--...-." and "--...--.".
 *
 *
 * Note:
 *
 *
 * 	The length of words will be at most 100.
 * 	Each words[i] will have length in range [1, 12].
 * 	words[i] will only consist of lowercase letters.
 *
 *
 */
pub struct Solution {}

// submission codes start here

use std::collections::HashSet;

fn morse_transform(s: &str) -> String {
    let table: [&'static str; 26] = [
        ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
        "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--",
        "--..",
    ];
    s.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .flat_map(|c| (&table[(c.to_ascii_lowercase() as usize - 'a' as usize)]).chars())
        .collect()
}

impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let set: HashSet<String> = words
            .into_iter()
            .map(|word| morse_transform(&word))
            .collect();
        set.len() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0822() {
        assert_eq!(
            2,
            Solution::unique_morse_representations(vec_string!["gin", "zen", "gig", "msg"])
        );
    }
}
