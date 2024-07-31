pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        return s;
    }
    let s = s.chars().collect::<Vec<char>>();

    let mut result = vec![' '; s.len()];
    let mut curr = 0;
    let num_rows = num_rows as usize;
    let step = num_rows * 2 - 2;
    for v in s.iter().step_by(step) {
        result[curr] = *v;
        curr += 1;
    }

    for i in 1..=num_rows - 2 {
        let mut j = i;
        while j < s.len() {
            result[curr] = s[j];
            curr += 1;
            let k = j + (num_rows -2 - i) * 2 + 2;
            if k < s.len() {
                result[curr] = s[k];
                curr += 1;
            }
            j += step;
        }
    }

    for v in s.iter().skip(num_rows - 1).step_by(step) {
        result[curr] = *v;
        curr += 1;
    }

    result.iter().collect()
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1(){
        let s = "PAYPALISHIRING".to_string();
        let num_rows = 3;
        assert_eq!(convert(s, num_rows), "PAHNAPLSIIGYIR");
    }
}