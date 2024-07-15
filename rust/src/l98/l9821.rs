pub fn get_smallest_string(s: String) -> String {
    let mut s = s.into_bytes();
    for i in 1..s.len(){
        if s[i] < s[i-1] && s[i] %2 == s[i-1] % 2{
            (s[i] , s[i-1]) = (s[i-1], s[i]);
            break;
        }
    }

    String::from_utf8(s).unwrap()
}