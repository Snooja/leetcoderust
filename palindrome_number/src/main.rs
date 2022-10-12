fn main() {
    let nums: i32 = 12321;
    println!("{:?}", is_palindrome(nums));
}

pub fn is_palindrome(x: i32) -> bool {
    return via_string(x);
}

pub fn via_string(x: i32) -> bool {
    let s:String = x.to_string();
    let reverse:String = s.chars().rev().collect();
    return reverse.eq(&s);
}
