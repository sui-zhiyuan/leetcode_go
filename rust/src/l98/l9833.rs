pub fn max_operations(s: String) -> i32 {
    let mut merged = Vec::new();

    for v in s.chars(){
        if merged.is_empty(){
            merged.push(v);
            continue;
        } 
        let last = merged.last().unwrap();
        if *last == '0' && v == '0' {
            continue;
        }
        merged.push(v);
    }

    let mut result = 0;
    let mut count_1 = 0;

    for v in merged {
        if v == '1' {
            count_1 +=1;
            continue;
        }
        assert!(v == '0');
        result += count_1;
    }

    result
}