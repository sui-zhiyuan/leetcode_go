pub fn triangle_number(nums: Vec<i32>) -> i32 {
    let mut result = 0;

    for (i, &v1) in nums.iter().enumerate() {
        for (j, &v2) in nums.iter().enumerate().skip(i + 1) {
            for &v3 in nums.iter().skip(j + 1) {
                if v1 + v2 > v3 && v1 + v3 > v2 && v2 + v3 > v1 {
                    result += 1;
                }
            }
        }
    }

    result
}
