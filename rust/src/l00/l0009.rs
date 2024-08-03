pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let str = x.to_string();
    let rev = str.chars().rev().collect::<String>();
    str == rev
}