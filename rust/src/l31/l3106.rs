pub fn get_smallest_string(s: String, mut k: i32) -> String {
    let mut s = s.chars().collect::<Vec<_>>();
    for v in s.iter_mut(){
        let dis = distance(*v , 'a');
        if dis <= k {
            *v = 'a';
            k -= dis;
            continue;
        }
        *v = char_sub(*v , k);
        break;
    }
    s.into_iter().collect()
}
 
fn distance(a:char , b:char) -> i32 {
    let a = a as i32 - 'a' as i32;
    let b = b as i32 - 'a' as i32;
    let (a , b) = if a > b { (b , a) } else { (a , b) };
    (b-a).min(a+26-b)
}

fn char_sub(a:char , b:i32) -> char{
    let a = a as i32 - 'a' as i32;
    let b = (a - b) % 26;
    (b + 'a' as i32) as u8 as char
}