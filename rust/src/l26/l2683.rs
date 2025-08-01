pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
    // assume first value is 0
    let mut value = 0;
    for v in derived {
        value ^= v;
    }
    value == 0
}
