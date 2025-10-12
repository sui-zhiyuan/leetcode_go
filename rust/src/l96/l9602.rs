use std::collections::HashMap;

pub fn longest_balanced(s: String) -> i32 {
    let s = s.chars().collect::<Vec<char>>();
    let mut total_count = Vec::<HashMap<char, i32>>::new();
    total_count.push(HashMap::new());

    for (i, c) in s.iter().copied().enumerate() {
        let mut counter = total_count[i].clone();

        *counter.entry(c).or_insert(0) += 1;
        total_count.push(counter);
    }

    let mut result = 0;
    for i in 0..total_count.len() {
        for j in i..total_count.len() {
            let mut count = None;
            let mut vaild = true;
            for (k, end_v) in total_count[j].iter() {
                let curr_count = end_v - total_count[i].get(k).unwrap_or(&0);
                if curr_count == 0 {
                    continue;
                }
                if count == None {
                    count = Some(curr_count);
                } else {
                    if count != Some(curr_count) {
                        vaild = false;
                        break;
                    }
                }
            }

            if vaild {
                result = result.max(j - i);
            }
        }
    }

    result as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(4, longest_balanced("abbac".to_string()));
    }

    #[test]
    fn test2() {
        assert_eq!(3, longest_balanced("kooo".to_string()));
    }
}
