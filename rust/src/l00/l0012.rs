pub fn int_to_roman(num: i32) -> String {
    let chars = [('I', 'V'), ('X', 'L'), ('C', 'D'), ('M', '_') , ('_', '_')];
    let map = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];

    let mut num = num;
    let mut result = String::new();
    for i in (0..4).rev() {
        let base = 10_i32.pow(i as u32);
        let digit = num / base;
        num %= base;
        let digit = map[digit as usize].chars().map(|c| {
            match c {
                'I' => chars[i].0,
                'V' => chars[i].1,
                'X' => chars[i+1].0,
                _ => unreachable!(),
            }
        }).collect::<String>();
        result.push_str(&digit);
    }

    result
}
