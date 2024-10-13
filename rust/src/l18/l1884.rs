use crate::common::binary_search;

pub fn two_egg_drop(n: i32) -> i32 {
    binary_search(1, 1000, |x| {
        maximum_level(x) >= n
    })
}

fn maximum_level(tries:i32) -> i32 {
    tries * (tries + 1) / 2
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn print(){
        for i in 1..30{
            println!("{}: {}", i, maximum_level(i));
        }
    }
}