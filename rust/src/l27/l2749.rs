pub fn make_the_integer_zero(num1: i32, num2: i32) -> i32 {
    // consider num1 = 10^9 and num2 as 10^9 , should pow(2, k) >= num1 - num2*k
    for k in 1..37 {
        let x = num1 as i64 - num2 as i64 * k;
        if k > x {
            return -1;
        }
        if k as u32 >= x.count_ones() {
            return k as i32;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {

    #[test]
    fn get_max_result() {
        for k in 1i64.. {
            const NUM1: i64 = 1_000_000_000;
            const NUM2: i64 = -1_000_000_000;

            if 1i64 << k as usize > NUM1 - k * NUM2 {
                println!("max k {k}");
                return;
            }
        }
    }
}
