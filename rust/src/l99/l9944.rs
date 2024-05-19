pub fn ways_to_reach_stair(k: i32) -> i32 {
    let value= [2,4,4,3,2,4,6,4,1,0];
    if k < value.len() as i32{
        return value[k as usize];
    }
    
    let k = k as u32;
    let pow =32 - (k-2).leading_zeros();
    let pv = 1 << pow;
    let end = pv;
    let start = pv + 1 - pow - 3 +1;
    if k >= start && k <= end {
        combination(pow+1, k-start) as i32
    }else {
        0
    }
}

fn combination(n: u32, k: u32) -> u32 {
    let mut res = 1u32;
    for i in 0..k {
        res *= n - i;
        res /= i + 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let mut res = [0;100];
        let base = 1;
    
        for jump_times in 0..8{
            let mut jmp = 1 << jump_times;
            jmp -=1;
            for j in 0..=jump_times+1 {
                if jmp+base -j >= res.len(){
                    continue;
                }
                res[jmp+base -j] += combination(jump_times as u32+1, j as u32);
            }
        }
        
        for (i ,v) in res.iter().enumerate(){
            let k = i as i32;
            let pow = if i< 10 {30} else {(k-2).leading_zeros()};
            let pow =32 - pow;
            let pv = 1 << pow;
            let end = pv;
            let start = pv + 1 - pow - 3 +1;
            println!("{i:3}:{v:3}:{pow}:{pv}:{start}:{end}");
        }
    }
    #[test]
    fn test() {
        let k = 4;
        let res = ways_to_reach_stair(k);
        assert_eq!(res, 5);
    }
}