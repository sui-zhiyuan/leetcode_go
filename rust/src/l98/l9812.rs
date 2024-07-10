pub fn valid_strings(n: i32) -> Vec<String> {
    let values = &mut vec!['0'; n as usize];
    let mut result = vec![];
    generate(values, 0, '1', &mut result);
    result
}

fn generate(values: & mut[char] , index: usize , prev: char, result: &mut Vec<String>) {
    if index == values.len(){
        result.push(values.iter().collect());
        return;
    }
    if prev == '1' {
        values[index] = '0';
        generate(values, index + 1, '0', result);
        values[index] = '1';
        generate(values, index + 1, '1', result);
    } else {
        values[index] = '1';
        generate(values, index + 1, '1', result);
    }
}