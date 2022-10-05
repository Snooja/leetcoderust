fn main() {
    let nums:Vec<i32> = vec![1,2,3,4,5,6,7,8,9,10];
    let target:i32 = 19;
    let result = brute_force(nums, target);
    println!("index1: {:?}, index2: {:?}", result[0], result[1]);
}


// pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     // Returns the indicies in nums which add to target
//     return vec![-1];
        
// }

pub fn brute_force(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // Tries every possible combination N^N complexity worst case
    // println!{"len of nums is {}", nums.len()};

    for j in 0..nums.len() {
        // println!("j = {}", j);
        for k in j+1..nums.len() {
            // println!{"k = {}", k};
            if nums[j] + nums[k] == target {
                println!{"found solution at {}, {}", j, k};
                return vec![j as i32, k as i32];
            }
        }
    }

    return vec![-1];
}