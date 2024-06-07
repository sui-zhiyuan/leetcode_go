pub fn minimum_steps(s: String) -> i64 {
    let mut target = 0;
    let mut m = 0;

    for (i , v) in s.chars().enumerate(){
        if v == '1' {
            continue;
        }

        m += (i - target ) as i64;
        target += 1;
    }

    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "100".to_string();
        assert_eq!(minimum_steps(s), 2);
    }
}