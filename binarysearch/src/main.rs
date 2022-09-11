fn main() {
    let nums: Vec<i32> = (1..=100).collect();
    let target: i32 = 77;

    println!("array has: {} elements",nums.len());
    println!("first element of array at index 0 is: {:?}", nums[0]);

// let result: i32 = bruteforce(nums, target);
    let result:i32 = iterative(&nums,target);

    println!("index is: {:?} and value at index = {} which is equal to target ({})", result, nums[(result as usize)], target)
}

// fn bruteforce(nums: Vec<i32>, target: i32) -> i32 {
//     let mut result: i32 = -1;
//     for i in 0..nums.len() - 1  { 
//         if nums[i] == target {
//             result = i as i32;
//         }
//     }
//     return result;
// }

fn iterative(nums: &Vec<i32>, target: i32) -> i32{
    let mut low:usize = 1;
    let mut high:usize = nums.len();
    let mut mid:usize;

    while low <= high {
        mid = low +  ((high - low) / 2);

        if nums[mid] < target {
            low = mid + 1;
        }

        if nums[mid] > target {
            high = mid - 1;
        }

        if nums[mid] == target {
            return mid as i32;
        }

    }
    return -1;
}
// impl Solution {
//     pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        
//     }
// }