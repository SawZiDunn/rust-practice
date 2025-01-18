use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                if nums[j] + nums[i] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]

        // let mut map = HashMap::new();
        // for (i, &num) in nums.iter().enumerate() {
        //     let complement = target - num;
        //     if let Some(&index) = map.get(&complement) {
        //         return vec![index as i32, i as i32];
        //     }
        //     map.insert(num, i);
        // }
        // vec![]
    }
}

fn main() {
    let nums = vec![2, 5, 5, 11];
    let target = 10;
    let result = Solution::two_sum(nums, target);
    println!("{:?}", result);
}
