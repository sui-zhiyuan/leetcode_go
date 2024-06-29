pub fn smallest_string(s: String) -> String {
    let mut s= s.chars().collect::<Vec<_>>();

    let mut find = false;

    for v in s.iter_mut(){
        if *v == 'a' && find {
            break;
        }

        if *v == 'a'{
            continue;
        }

        find = true;
        *v = ((*v as u8) - 1) as char;
    }

    if !find{
        *s.last_mut().unwrap() = 'z';
    }

    s.into_iter().collect() 
}