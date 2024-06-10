pub fn number_of_child(n: i32, k: i32) -> i32 {
    let mut curr = 0;
    let mut dis = 1;

    for c in 1..=k {
        curr += dis;
        if curr == n-1 {
            dis = -1;
        }
        if curr == 0 {
            dis = 1
        }
    }

    curr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(number_of_child(3, 5), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(number_of_child(5, 6), 2);
    }

    #[test]
    fn test3() {
        assert_eq!(number_of_child(4, 2), 2);
    }
}
