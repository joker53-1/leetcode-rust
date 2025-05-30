/**
 * [8] String to Integer (atoi)
 *
 * Implement the myAtoi(string s) function, which converts a string to a 32-bit signed integer.
 * The algorithm for myAtoi(string s) is as follows:
 * <ol>
 * 	Whitespace: Ignore any leading whitespace (" ").
 * 	Signedness: Determine the sign by checking if the next character is '-' or '+', assuming positivity if neither present.
 * 	Conversion: Read the integer by skipping leading zeros until a non-digit character is encountered or the end of the string is reached. If no digits were read, then the result is 0.
 * 	Rounding: If the integer is out of the 32-bit signed integer range [-2^31, 2^31 - 1], then round the integer to remain in the range. Specifically, integers less than -2^31 should be rounded to -2^31, and integers greater than 2^31 - 1 should be rounded to 2^31 - 1.
 * </ol>
 * Return the integer as the final result.
 *  
 * <strong class="example">Example 1:
 * <div class="example-block">
 * Input: <span class="example-io">s = "42"</span>
 * Output: <span class="example-io">42</span>
 * Explanation:
 * 
 * The underlined characters are what is read in and the caret is the current reader position.
 * Step 1: "42" (no characters read because there is no leading whitespace)
 *          ^
 * Step 2: "42" (no characters read because there is neither a '-' nor '+')
 *          ^
 * Step 3: "<u>42</u>" ("42" is read in)
 *            ^
 * </div>
 * <strong class="example">Example 2:
 * <div class="example-block">
 * Input: <span class="example-io">s = " -042"</span>
 * Output: <span class="example-io">-42</span>
 * Explanation:
 * 
 * Step 1: "<u>   </u>-042" (leading whitespace is read and ignored)
 *             ^
 * Step 2: "   <u>-</u>042" ('-' is read, so the result should be negative)
 *              ^
 * Step 3: "   -<u>042</u>" ("042" is read in, leading zeros ignored in the result)
 *                ^
 * </div>
 * <strong class="example">Example 3:
 * <div class="example-block">
 * Input: <span class="example-io">s = "1337c0d3"</span>
 * Output: <span class="example-io">1337</span>
 * Explanation:
 * 
 * Step 1: "1337c0d3" (no characters read because there is no leading whitespace)
 *          ^
 * Step 2: "1337c0d3" (no characters read because there is neither a '-' nor '+')
 *          ^
 * Step 3: "<u>1337</u>c0d3" ("1337" is read in; reading stops because the next character is a non-digit)
 *              ^
 * </div>
 * <strong class="example">Example 4:
 * <div class="example-block">
 * Input: <span class="example-io">s = "0-1"</span>
 * Output: <span class="example-io">0</span>
 * Explanation:
 * 
 * Step 1: "0-1" (no characters read because there is no leading whitespace)
 *          ^
 * Step 2: "0-1" (no characters read because there is neither a '-' nor '+')
 *          ^
 * Step 3: "<u>0</u>-1" ("0" is read in; reading stops because the next character is a non-digit)
 *           ^
 * </div>
 * <strong class="example">Example 5:
 * <div class="example-block">
 * Input: <span class="example-io">s = "words and 987"</span>
 * Output: <span class="example-io">0</span>
 * Explanation:
 * Reading stops at the first non-digit character 'w'.
 * </div>
 *  
 * Constraints:
 * 
 * 	0 <= s.length <= 200
 * 	s consists of English letters (lower-case and upper-case), digits (0-9), ' ', '+', '-', and '.'.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/string-to-integer-atoi/
// discuss: https://leetcode.com/problems/string-to-integer-atoi/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut i = 0;
        let mut flag = 1;
        let mut ans = 0;
        let mut start = false;
        
        while i < chars.len() {
            if chars[i] == ' ' {
                i += 1;
                continue;
            }
            if chars[i] == '-' && !start {
                flag = -1;
                start = true;
            } else if chars[i] == '+' && !start {
                flag = 1;
                start = true;
            }
            else if chars[i] >= '0' && chars[i] <= '9' {
                if ans > i32::MAX / 10 || (ans == i32::MAX / 10 && (chars[i] as i32 - '0' as i32) > i32::MAX  % 10) {
                    return i32::MAX;
                }
                if (ans < i32::MIN / 10 || (ans == i32::MIN / 10 && (chars[i] as i32 - '0' as i32) > -(i32::MIN % 10))) {
                    return i32::MIN;
                }
                ans = ans * 10 + flag * (chars[i] as i32 - '0' as i32);

                start = true;
            } else {
                break;
            }
            i += 1;
        }
        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_8() {
    }
}
