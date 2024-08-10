pub fn find_integers(n: i32) -> i32 {
    let mut state = vec![0; 66];
    state[0] = 0; // length = 0 , last = 0
    state[1] = 0; // length = 0 , last = 1
    state[2] = 1; // length = 1 , last = 0
    state[3] = 1; // length = 1 , last = 1

    for i in 4..66 {
        let length = i / 2;
        let last = i % 2;
        if last == 0 {
            state[i] = state[(length - 1) * 2] + state[(length - 1) * 2 + 1];
        } else {
            state[i] = state[(length - 1) * 2];
        }
    }

    // for i in 0..66 {
    //     let length = i / 2;
    //     let last = i % 2;
    //     let v = state[i];
    //     println!("{length:<3} {last} : {v}");
    // }

    let n = n as u32;
    let mut prev = 0;
    let mut result = 0;
    for i in (1..=32).rev() {
        let curr = n & (1 << (i - 1));
        if curr > 0 {
            result += state[i * 2];
            if prev > 0 {
                break;
            }
        }

        prev = curr;
        if i == 1 {
            result += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_integers() {
        assert_eq!(find_integers(5), 5);
    }
}
