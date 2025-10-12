use crate::common::binary_search;

pub fn longest_balanced(s: String) -> i32 {
    let mut total_count = Vec::<[i32 ; 3]>::new();
    total_count.push([0; 3]);

    for c in s.chars() {
        assert!(c >= 'a' && c <= 'c');
        let mut value = *total_count.last().unwrap();
        value[c as usize - 'a' as usize] +=1;
        total_count.push(value);
    }
    
    binary_search(0usize , s.len() , |v| {
        if v % 2 ==0 {
            for i in 0..total_count.len() {
                let j = i + v;
                if j >= total_count.len() {
                    break;
                }
            }
        }
        
        false
    }) as i32
}