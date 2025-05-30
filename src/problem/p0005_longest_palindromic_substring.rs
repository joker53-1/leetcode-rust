use std::cmp::max;

/**
 * [5] Longest Palindromic Substring
 *
 * Given a string s, return the longest <span data-keyword="palindromic-string">palindromic</span> <span data-keyword="substring-nonempty">substring</span> in s.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "babad"
 * Output: "bab"
 * Explanation: "aba" is also a valid answer.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: s = "cbbd"
 * Output: "bb"
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 1000
 * 	s consist of only digits and English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-palindromic-substring/
// discuss: https://leetcode.com/problems/longest-palindromic-substring/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() < 2 {
            return s;
        }
        let mut dp: Vec<Vec<bool>> = vec![vec![false; s.len()]; s.len()];
        for i in 0..s.len() {
           dp[i][i] = true;
        }
        
        let chars = s.chars().collect::<Vec<char>>();

        let mut len = 1;
        let mut start = 0;
        for l in 2..=s.len() {
            for i in 0..s.len() {
                let j = l + i - 1;
                if j >= s.len() { break }
                if chars[i] != chars[j] { 
                    dp[i][j] = false;
                } else if j - i < 3 {
                    dp[i][j] = true;
                } else {
                    dp[i][j] = dp[i+1][j-1];
                }
                if dp[i][j] && j-i+1 > len {
                    start = i;
                    len = j-i+1;
                }
            }
        }
        s[start..start + len].to_owned()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_5() {
        println!("{}", Solution::longest_palindrome("ac".to_string()));
    }
}
