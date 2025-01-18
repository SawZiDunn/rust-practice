struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let x = x.to_string();

        let mut count = x.len() - 1;
        for i in 0..x.len() / 2 {
            if x.chars().nth(i).unwrap() != x.chars().nth(count).unwrap() {
                return false;
            }
            count -= 1;
        }
        true
    }
}

fn main() {
    let isPalindrome = Solution::is_palindrome(10);
    println!("{}", isPalindrome);
}
