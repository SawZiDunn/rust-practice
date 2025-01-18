struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut final_string = String::new();
        let mut temp_string = String::new();

        if s.len() == 1 {
            return s;
        }

        for i in 0..s.len() {
            for j in i..s.len() {
                let temp = &s[i..=j].to_string();
                let mut count = temp.len() - 1;

                let mut is_palindrome = || -> bool {
                    for k in 0..temp.len() / 2 {
                        if temp.chars().nth(k).unwrap() != temp.chars().nth(count).unwrap() {
                            if temp.len() == 2 {
                                temp_string = s[..1].to_string();
                            }
                            return false;
                        }

                        count -= 1;
                    }
                    true
                };

                if is_palindrome() {
                    temp_string = temp.clone();
                }
            }
            if temp_string.len() > final_string.len() {
                final_string = temp_string.clone();
            }
        }

        final_string
    }
}

fn main() {
    let ans = Solution::longest_palindrome("aaaaaaa".to_owned());
    println!("{:?}", ans);
}
