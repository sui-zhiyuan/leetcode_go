pub fn largest_palindrome(n: i32, k: i32) -> String {
    let n = n as usize;

    if k == 6 && n >=5 {
        let half = '9'.to_string().repeat(n /2 + n % 2 -2);
        let mid = if n.is_multiple_of(2) {
            "77".to_string()
        }else {
            "8".to_string()
        };
        return format!("8{half}{mid}{half}8");
    }
    
    let mut d_mod = vec![0; n];
    d_mod[0] = 1;
    for i in 1..n {
        d_mod[i] = d_mod[i - 1] * 10 % k;
    }

    let half_n = n / 2 + n % 2;
    for i in 0..half_n{
        if i == n -i -1 {
            continue;
        }
        d_mod[i] = (d_mod[i] + d_mod[n - i - 1]) % k;
    }

    let mut values = vec![9; half_n];

    let mut sum = 0;
    for i in 0..half_n {
        sum = (sum + values[i] * d_mod[i]) % k;
    }
    let mut curr = half_n -1;
    while sum != 0{
        while d_mod[curr] == 0{
            curr -= 1;
        }

        values[curr] -= 1;
        sum = (sum + k - d_mod[curr]) % k;
        let mut pos = curr;
        while values[pos] < 0{
            values[pos] += 10;
            sum = (sum + 10* d_mod[pos]) % k;
            values[pos - 1] -= 1;
            sum = (sum + k - d_mod[pos - 1]) % k;
            pos -= 1;
        }
    }

    let first_half = values.iter().map(|&x| x.to_string()).collect::<String>();
    let end = half_n - n % 2;
    let next_half = values[..end].iter().rev().map(|&x| x.to_string()).collect::<String>();
    format!("{}{}", first_half, next_half)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 3;
        let k = 5;
        let res = "595".to_string();
        assert_eq!(largest_palindrome(n, k), res);
    }

    #[test]
    fn test2() {
        let n = 5;
        let k = 6;
        let res = "89898".to_string();
        assert_eq!(largest_palindrome(n, k), res);
    }

    #[test]
    fn test3() {
        for n in 1..=8 {
            let v = largest_palindrome(n, 6);
            println!("{}: {}", n, v);
        }
    }
}