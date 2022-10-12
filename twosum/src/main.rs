//use std::collections::HashMap;

fn main() {
    let nums:Vec<i32> = vec![3,2,4];
    let target:i32 = 6;
    let result = hash_daddy(nums,target);
    println!("index1: {:?}, index2: {:?}", result[0], result[1]);
}



pub fn hash_daddy(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // Create a hash map and insert each item
    let nums_len = nums.len();
    let mut hash: std::collections::HashMap<i32, i32> = std::collections::HashMap::with_capacity(nums_len);
    
    // Insert each element into the hash map
    // We will be looking up by the number itself later
    // So we use the number as the key (not the index!)
    for (idx, value) in nums.iter().enumerate() {
        hash.insert(*value, idx as i32);
    }

    // Now loop through again
    // And for each check if the other half of the sum exists in the hash map
    // If it does pull it out and we're done!

    for (idx, value) in nums.iter().enumerate() {
        let compliment = target - value;

        
        match hash.get(&compliment) {
            // Found
            Some(&compliment_idx) => {
                // Cannot use the same element twice
                if idx == compliment_idx as usize {
                    continue;
                } else {
                    return vec![idx as i32, compliment_idx as i32];
                }
                
            }
            // Not found
            None => {
                continue;
            }
        }
    }

    unreachable!("dear god how did this happen")

}

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