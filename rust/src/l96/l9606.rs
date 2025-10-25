use std::io::Write;

pub fn max_sum_of_squares(num: i32, sum: i32) -> String {
    if num * 9 < sum || sum == 0 {
        return "".to_string();
    }

    let mut buffer = Vec::<u8>::new();
    let mut sum = sum;
    for _ in 0..num {
        if sum > 9 {
            sum -= 9;
            write!(&mut buffer, "9").unwrap();
        }else {
            write!(&mut buffer, "{}" , sum).unwrap();
            sum = 0;
        }
    }
    String::from_utf8(buffer).unwrap()
}
