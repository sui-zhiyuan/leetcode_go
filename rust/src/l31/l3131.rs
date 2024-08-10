pub fn added_integer(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> i32 {
    nums1.sort();
    nums2.sort();

    let diff = nums1
        .iter()
        .zip(nums2.iter())
        .map(|(a, b)| b-a)
        .collect::<Vec<i32>>();

    diff[0]
}
