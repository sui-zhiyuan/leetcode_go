pub fn is_array_special(nums: Vec<i32>) -> bool {
    if nums.is_empty() {
        return true;
    }
    let is_even = nums.iter().map(|&v| v % 2 == 0).collect::<Vec<bool>>();

    for i in 0..(nums.len()-1){
        if is_even[i] == is_even[i+1] {
            return false;
        }
    }
    true
}