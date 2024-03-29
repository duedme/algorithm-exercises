pub struct Solution;

impl Solution  {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut used: HashMap<i32, i32> = HashMap::new();
        let mut answer: Vec<i32> = Vec::new();

        for (index, num) in nums.iter().enumerate() {

                if used.contains_key(&(target - num)) { 
                    answer.push(index as i32);
                    answer.push(*used.get(&(target - num)).unwrap());

                    return answer
                    
                } else {
                    used.insert(*num, index as i32);
                }
        }

        answer
    }
}

fn main() {
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![2, 1]);
}
