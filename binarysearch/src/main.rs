fn main() {
    let nums: Vec<i32> = (1..10000).collect();
    let target: i32 = 7001;

    println!("{}",nums.len());
    println!("{:?}", nums[0]);

    let result: i32 = bruteforce(nums, target);

    println!("{}",result)
}

fn bruteforce(nums: Vec<i32>, target: i32) -> i32 {
    let mut result: i32 = -1;
    for i in 0..nums.len() - 1  { 
        if nums[i] == target {
            result = i as i32;
        }
    }
    return result;
}

// impl Solution {
//     pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        
//     }
// }