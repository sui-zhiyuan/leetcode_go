pub fn get_encrypted_string(s: String, k: i32) -> String {
    let s = s.chars().collect::<Vec<char>>();
    let n = s.len();
    let mut t =  s.clone();
    for (i , v) in t.iter_mut().enumerate() {
        let next = i + k as usize;
        let next = next % n;

        *v = s[next];
    } 

    t.iter().collect()  
}