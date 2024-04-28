pub fn can_make_square(grid: Vec<Vec<char>>) -> bool {
    for si in 0..=1{
        for sj in 0..=1{
            let mut count =0;
            for i in 0..=1{
                for j in 0..=1{
                    if grid[si+i][sj+j] == 'W'{
                        count += 1;
                    }
                }
            }

            dbg!(si , sj , count);

            if count == 0 || count == 1 || count == 3 || count == 4{
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1(){
        let input = vec![
            vec!['B', 'W', 'B'],
            vec!['W', 'B', 'W'],
            vec!['B', 'W', 'B'],
        ];
        assert!(!can_make_square(input))
    }
}