pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
    let mut result = 0;
    let mut swap_len = 0;
    let mut prev = colors[0];

    let len = colors.len() + k as usize -1;
    
    for &v in colors.iter().cycle().take(len){
        if prev == v {
            if swap_len >= k {
                result += swap_len - k + 1;
            }

            swap_len = 1;
            prev = v;
            continue;
        }

        swap_len += 1;
        prev = v;
    }
    if swap_len >= k {
        result += swap_len - k + 1;
    }
    result
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test1(){
        let colors = vec![0,1,1];
        let k = 3;
        let result = number_of_alternating_groups(colors, k);
        assert_eq!(result, 1);
    }

}