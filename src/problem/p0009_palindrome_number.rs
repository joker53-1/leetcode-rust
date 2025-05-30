/**
 * [9] Palindrome Number
 *
 * Given an integer x, return true if x is a <span data-keyword="palindrome-integer">palindrome</span>, and false otherwise.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: x = 121
 * Output: true
 * Explanation: 121 reads as 121 from left to right and from right to left.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: x = -121
 * Output: false
 * Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: x = 10
 * Output: false
 * Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
 * 
 *  
 * Constraints:
 * 
 * 	-2^31 <= x <= 2^31 - 1
 * 
 *  
 * Follow up: Could you solve it without converting the integer to a string?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/palindrome-number/
// discuss: https://leetcode.com/problems/palindrome-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let chars= x.to_string().chars().collect::<Vec<char>>();
        let (mut left, mut right) = (0, chars.len() - 1);
        while left <= right {
            if chars[left] == chars[right] {
                left += 1;
                if right < 1 { break }
                right -= 1;
            } else {
                return false;
            }
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_9() {
        println!("Solution: {}", Solution::is_palindrome(0));
    }
}
