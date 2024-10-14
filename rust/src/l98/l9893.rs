use std::cmp;

pub fn count_good_integers(n: i32, k: i32) -> i64 {
    let n = n as usize;
    let k = k as usize;
    let mut md = vec![1 % k];

    for _ in 1..n {
        let last = md.last().unwrap();
        let next = (last * 10) % k;
        md.push(next);
    }

    let digits = n / 2 + n % 2;
    for i in 0..(n / 2) {
        md[i] += md[n - i - 1];
    }

    let min = 10usize.pow((digits - 1) as u32);
    let max = 10usize.pow((digits) as u32);
    let mut result = 0;
    let mut take = vec![false; max];
    for cv in min..max {
        let mut digs = Vec::new();
        let mut cv = cv;
        while cv > 0 {
            digs.push((cv % 10) as u8);
            cv /= 10;
        }

        let mut tmd = 0;
        for (i, &v) in digs.iter().enumerate() {
            tmd += v as usize * md[i]
        }
        if tmd % k != 0 {
            continue;
        }

        let vv = Value::new(digs, n);
        let vh = vv.hash_value();
        if take[vh] {
            continue;
        }
        take[vh] = true;

        result += vv.count_total()
    }

    result
}

struct Value {
    v: Vec<u8>,
    n: usize,
}

impl Value {
    fn new(v: Vec<u8>, n: usize) -> Self {
        let mut v = v;
        v.sort_unstable_by_key(|x| cmp::Reverse(*x));
        Self { v, n }
    }
    fn hash_value(&self) -> usize {
        let mut r = 0;
        for &vv in self.v.iter() {
            r = r * 10 + vv as usize;
        }
        r
    }

    fn count_total(&self) -> i64 {
        let mid_pos = if self.n % 2 == 1 {
            self.n / 2
        } else {
            usize::MAX // max valid value is 4
        };
        let mut curr = self.v[0];
        let mut counts = vec![if 0 == mid_pos { 1usize } else { 2usize }];
        let mut zero_index = self.v.len(); // invalid value
        if curr == 0 {
            zero_index = 0;
        }
        for (i, &cv) in self.v.iter().enumerate().skip(1) {
            let adder = if i == mid_pos { 1 } else { 2 };
            if cv == curr {
                *counts.last_mut().unwrap() += adder;
            } else {
                curr = cv;
                counts.push(adder);
                if cv == 0 {
                    zero_index = counts.len() - 1;
                }
            }
        }

        let a = (1..=self.n).collect::<Vec<_>>();
        let mut b = Vec::new();
        for &c in counts.iter() {
            b.extend(1..=c);
        }
        let sum1 = cal_sv(a.clone(), b);
        
        let sum2 = 0;
        if zero_index < counts.len(){
            // let a_nz = 1..=(self.n - counts[zero_index]);
            let mut b_nz = Vec::new();
            for (i, &c) in counts.iter().enumerate() {
                if i == zero_index {
                    continue;
                }
                b_nz.extend(1..=c);
            }
    
            // let sum2 = cal_sv(a, b_nz);
        }

        sum1 - sum2
    }
}

fn cal_sv(a: Vec<usize>, b: Vec<usize>) -> i64 {
    let mut a = a;
    let mut b = b;

    a.sort();
    b.sort();
    let mut result = 1;
    let mut curr = 0;
    for &v in a.iter() {
        result *= v;
        while curr < b.len() && result % b[curr] == 0 {
            result /= b[curr];
            curr += 1;
        }
    }
    assert_eq!(curr, b.len());
    result as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(count_good_integers(3, 5), 27);
    }

    #[test]
    fn test2() {
        assert_eq!(count_good_integers(1, 4), 2);
    }

    #[test]
    fn test3() {
        assert_eq!(count_good_integers(5, 6), 2468);
    }

    #[test]
    fn test_calc() {
        for k in 1..10 {
            let mut r = vec![1 % k];

            for _ in 1..10 {
                let last = r.last().unwrap();
                let next = (last * 10) % k;
                r.push(next);
            }

            println!("{k}:{r:?}");
        }
    }
}
