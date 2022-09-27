pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut answer: Vec<i32> = Vec::new();
        
        for (i, value1) in nums.iter().enumerate() {
            for (j, value2) in nums.iter().skip(i + 1).enumerate() {
                if target - value2 == *value1 {
                    answer.push(j as i32 + 1 + i as i32);
                    answer.push(i as i32);
                }
            }
        }
        
        answer


    }
}

fn main() {
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
}
