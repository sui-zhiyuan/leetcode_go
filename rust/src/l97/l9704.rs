pub fn maximum_subarray_xor(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = nums.len();
    let mut states = vec![vec![State::new(0); n]; n+1];

    for (i, &v) in nums.iter().enumerate(){
        states[1][i] = State::new(v);
    }

    for len in 2..=n {
        for start in 0..(n - len + 1) {
            states[len][start] = states[len-1][start].merge(&states[len-1][start+1]);
        }
    }

    // for len in 1..=n {
    //     for start in 0..(n - len + 1) {
    //         let State{curr , max} = states[len][start];

    //         print!("{curr}-{max} ");
    //     }
    //     println!()
    // }

    let mut result = Vec::new();
    for q in queries {
        let start = q[0] as usize;
        let end = q[1] as usize;

        result.push(states[end - start +1][start].max);
    }

    result
}

#[derive(Debug , Clone , Copy)]
struct State {
    curr: i32,
    max: i32,
}

impl State {
    fn new(v: i32) -> Self {
        State { curr: v, max: v }
    }

    fn merge(&self, other: &Self) -> Self {
        let curr = self.curr ^ other.curr;
        let max = self.max.max(other.max).max(curr);
        State { curr, max }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0(){
        let nums = vec![2,8,4];
        let queries = [[0,2]];
        let queries = queries.iter().map(|v| v.to_vec()).collect();
        let result = maximum_subarray_xor(nums, queries);
        assert_eq!(result, vec![12]);
    }


    #[test]
    fn test1(){
        let nums = vec![2,8,4,32,16,1];
        let queries = [[0,2],[1,4],[0,5]];
        let queries = queries.iter().map(|v| v.to_vec()).collect();
        let result = maximum_subarray_xor(nums, queries);
        assert_eq!(result, vec![12,60,60]);
    }

    #[test]
    fn test_demo() {
        for n in 1..10 {
            let mut sums = Vec::new();
            let mut curr = 1;
            for _ in 0..n {
                sums.push(curr);
                curr <<= 1;
            }

            println!("new start with {n}");
            print_vec(&sums);
            while sums.len() > 1 {
                for i in 0..(sums.len() - 1) {
                    sums[i] = sums[i] ^ sums[i + 1];
                }
                sums.pop();
                print_vec(&sums);
            }
        }
    }

    fn print_vec(v: &[i32]) {
        println!();

        for i in v.iter() {
            print_v(*i);
            print!("\t");
        }

        println!();
    }

    fn print_v(v: i32) {
        let mut i = 1;
        let mut curr = 0;
        while i <= v {
            if (v & i) != 0 {
                print!("{curr}-");
            }
            i <<= 1;
            curr += 1;
        }
    }
}
