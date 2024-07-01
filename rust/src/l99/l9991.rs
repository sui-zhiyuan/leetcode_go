pub fn max_height_of_triangle(red: i32, blue: i32) -> i32 {
    let mut curr = red;
    let mut next = blue;

    let max = red.max(blue);

    let mut result = 0;
    for i in 1..=(max + 1){
        if curr < i {
            result = result.max(i-1);
            break;
        }

        curr -= i;
        (next , curr) = (curr, next);
    }

    let mut curr = blue;
    let mut next = red;

    for i in 1..=(max + 1){
        if curr < i {
            result = result.max(i-1);
            break;
        }

        curr -= i;
        (next , curr) = (curr, next);
    }

    result
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1(){
        let red = 2;
        let blue = 4;
        let result = max_height_of_triangle(red, blue);
        assert_eq!(result, 3);
    }
    
    #[test]
    fn test2(){
        let red = 2;
        let blue = 1;
        let result = max_height_of_triangle(red, blue);
        assert_eq!(result, 2);
    }
}