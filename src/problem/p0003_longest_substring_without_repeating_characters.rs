use std::cmp::max;
use std::collections::{HashMap, HashSet};

/**
 * [3] Longest Substring Without Repeating Characters
 *
 * Given a string s, find the length of the longest <span data-keyword="substring-nonempty">substring</span> without duplicate characters.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "abcabcbb"
 * Output: 3
 * Explanation: The answer is "abc", with the length of 3.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: s = "bbbbb"
 * Output: 1
 * Explanation: The answer is "b", with the length of 1.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: s = "pwwkew"
 * Output: 3
 * Explanation: The answer is "wke", with the length of 3.
 * Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
 * 
 *  
 * Constraints:
 * 
 * 	0 <= s.length <= 5 * 10^4
 * 	s consists of English letters, digits, symbols and spaces.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-substring-without-repeating-characters/
// discuss: https://leetcode.com/problems/longest-substring-without-repeating-characters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chars = s.chars().collect::<Vec<char>>();
        let mut map = HashMap::new();
        let mut ans = 0;
        let mut left = 0;
        for i in 0..chars.len() {
            if map.contains_key(&chars[i]) {
                left = max(map[&chars[i]] + 1, left);
            }
            map.insert(chars[i], i);
            ans = max(ans, i - left + 1);
        }
        ans as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3() {
        println!("{}", Solution::length_of_longest_substring("abba".to_string()));
    }
}
