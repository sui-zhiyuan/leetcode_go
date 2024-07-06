// cspell:ignore harshad

pub fn sum_of_the_digits_of_harshad_number(x: i32) -> i32 {
    let mut sum = 0;
    let mut v = x;
    while v > 0 {
        sum += v % 10;
        v /= 10;
    }

    if x % sum == 0 {
        sum
    } else {
        -1
    }
}
