// cspell:ignore strobogrammatic
pub fn find_strobogrammatic(n: i32) -> Vec<String> {
    let rev: [(char, char); 5] = [('0', '0'), ('1', '1'), ('6', '9'), ('8', '8'), ('9', '6')];
    let n = n as usize;

    let mut stack: Vec<usize> = vec![];
    let mut state = State::Start;
    let mut result = vec![];
    if n == 1{
        result.push("0".to_string());
    }
    loop {
        match state {
            State::Start => {
                if stack.len() == n / 2 + n % 2 {
                    let copied = &stack[..n / 2];
                    let mut str: Box<dyn Iterator<Item = char>> =
                        Box::new(copied.iter().map(|&i| rev[i].0));
                    if n % 2 == 1 {
                        str = Box::new(str.chain(std::iter::once(rev[stack[n / 2]].0)));
                    }
                    let str = str.chain(copied.iter().rev().map(|&i| rev[i].1));
                    result.push(str.collect::<String>());
                    state = State::LoopAfter;
                    continue;
                }
                stack.push(0);
                state = State::Loop;
                continue;
            }
            State::Loop => {
                let len = stack.len();
                let curr = stack.last_mut().unwrap();
                if len == 1 && *curr == 0 {
                    *curr = 1; // 0 can't be first digit
                }
                if len == n / 2 + n % 2 && n % 2 == 1 {
                    while *curr < rev.len() && rev[*curr].0 != rev[*curr].1 {
                        *curr += 1; // 6 ,9 can be middle digit
                    }
                }
                if *curr == rev.len() {
                    stack.pop();
                    if stack.is_empty() {
                        break;
                    }
                    state = State::LoopAfter;
                    continue;
                }
                state = State::Start;
                continue;
            }

            State::LoopAfter => {
                let curr = stack.last_mut().unwrap();
                *curr += 1;
                state = State::Loop;
                continue;
            }
        }
    }

    result
}

enum State {
    Start,
    Loop,
    LoopAfter,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 2;
        let res = vec!["11", "69", "88", "96"];
        assert_eq!(find_strobogrammatic(n), res);
    }

    #[test]
    fn test2() {
        let n = 1;
        let res = vec!["0", "1", "8"];
        assert_eq!(find_strobogrammatic(n), res);
    }
}
