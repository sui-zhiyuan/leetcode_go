pub fn smallest_beautiful_string(s: String, k: i32) -> String {
    let mut s = s.chars().map(|c| c as u8 - b'a').collect::<Vec<_>>();
    let k = k as u8;

    let mut curr = s.len() -1;
    s[curr] += 1;

    loop {
        if s[curr] == k{
            s[curr] = 0;
            if curr == 0 {
                return "".to_string();
            }
            curr -= 1;
            s[curr] += 1;
            continue;
        } 

        if curr > 0 && s[curr] == s[curr - 1] || curr > 1 && s[curr] == s[curr - 2]{
            s[curr] += 1;
            continue;
        }

        curr +=1;
        if curr == s.len(){
            break;
        }
    }

    s.into_iter().map(|v| (v + b'a') as char).collect::<String>()
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1(){
        let s = "abcd".to_string();
        let k = 4;
        assert_eq!(smallest_beautiful_string(s, k) , "abda")
    }

    #[test]
    fn test2(){
        let s = "abcz".to_string();
        let k = 26;
        assert_eq!(smallest_beautiful_string(s, k) , "abda")
    }

    #[test]
    fn test3(){
        let s = "dc".to_string();
        let k = 4;
        assert_eq!(smallest_beautiful_string(s, k) , "")
    }

    #[test]
    fn test4(){
        let s = "cd".to_string();
        let k = 4;
        assert_eq!(smallest_beautiful_string(s, k) , "da")
    }
}