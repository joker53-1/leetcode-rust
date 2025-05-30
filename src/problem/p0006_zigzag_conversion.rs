/**
 * [6] Zigzag Conversion
 *
 * The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)
 * 
 * P   A   H   N
 * A P L S I I G
 * Y   I   R
 * 
 * And then read line by line: "PAHNAPLSIIGYIR"
 * Write the code that will take a string and make this conversion given a number of rows:
 * 
 * string convert(string s, int numRows);
 * 
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "PAYPALISHIRING", numRows = 3
 * Output: "PAHNAPLSIIGYIR"
 * 
 * <strong class="example">Example 2:
 * 
 * Input: s = "PAYPALISHIRING", numRows = 4
 * Output: "PINALSIGYAHRPI"
 * Explanation:
 * P     I    N
 * A   L S  I G
 * Y A   H R
 * P     I
 * 
 * <strong class="example">Example 3:
 * 
 * Input: s = "A", numRows = 1
 * Output: "A"
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 1000
 * 	s consists of English letters (lower-case and upper-case), ',' and '.'.
 * 	1 <= numRows <= 1000
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/zigzag-conversion/
// discuss: https://leetcode.com/problems/zigzag-conversion/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let mut ans = String::new();
        let mut list = vec![String::new(); num_rows as usize];
        let (mut flag, mut i) = (-1i32, 0i32);
        let chars: Vec<char> = s.chars().collect();
        for char in chars {
            list[i as usize].push(char);
            if i == 0 || i as usize == num_rows as usize - 1 { 
                flag = -flag
            }
            i += flag
        }
        list.iter().for_each(|s| ans.push_str(s));
        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_6() {
    }
}
