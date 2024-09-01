pub fn string_hash(s: String, k: i32) -> String {
    let k  = k as usize;
    let round = s.len() / k;
    let s = s.chars().collect::<Vec<char>>();

    let mut result = String::new();
    for i in 0..round{
        let v = s[i*k..k + i*k].iter().map(|&c| to_usize(c)).sum::<usize>() % 26;
        result.push(to_char(v));
    }

    result
}

fn to_usize(c: char) -> usize {
    c as usize - 'a' as usize
}

fn to_char(n: usize) -> char {
    (n as u8 + b'a') as char
}