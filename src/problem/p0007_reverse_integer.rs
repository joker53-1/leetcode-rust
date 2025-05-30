/**
 * [7] Reverse Integer
 *
 * Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value to go outside the signed 32-bit integer range [-2^31, 2^31 - 1], then return 0.
 * Assume the environment does not allow you to store 64-bit integers (signed or unsigned).
 *  
 * <strong class="example">Example 1:
 * 
 * Input: x = 123
 * Output: 321
 * 
 * <strong class="example">Example 2:
 * 
 * Input: x = -123
 * Output: -321
 * 
 * <strong class="example">Example 3:
 * 
 * Input: x = 120
 * Output: 21
 * 
 *  
 * Constraints:
 * 
 * 	-2^31 <= x <= 2^31 - 1
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reverse-integer/
// discuss: https://leetcode.com/problems/reverse-integer/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut ans = 0;
        let mut x = x;
        while x != 0 {
            if !(i32::MIN / 10..=i32::MAX / 10).contains(&ans) {
                return 0;
            }
            let tmp = x % 10;
            x /= 10;
            ans = ans * 10 + tmp;
        }
        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_7() {
    }
}
