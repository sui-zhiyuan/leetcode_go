pub fn merge(input: Vec::<Vec<i32>>) -> Vec::<Vec<i32>>{
    let mut intervals = input.iter().map(|v| Interval{start: v[0] , end:v[1]}).collect::<Vec<_>>();

    intervals.sort_by_key(|x| x.start);

    let mut result = Vec::<Interval>::new();

    let mut curr = intervals[0];
    for x in intervals.into_iter().skip(1){
        if x.start <= curr.end{
            curr.end = curr.end.max(x.end);
            continue;
        }

        result.push(curr);
        curr= x;
    }

    result.push(curr);
    
    result.into_iter().map(|v| vec![v.start ,v.end]).collect()
}


#[derive(Debug , Clone, Copy)]
struct Interval {
    start : i32,
    end :i32,
}

#[cfg(test)]
mod tests{
    use crate::common;

    use super::*;

    #[test]
    fn test1(){
        let input = common::parse_grid::<i32>("[[1,3],[2,6],[8,10],[15,18]]").unwrap();
        let output = common::parse_grid::<i32>("[[1,6],[8,10],[15,18]]").unwrap();
        assert_eq!(merge(input) , output)
    }

    #[test]
    fn test2(){
        let input = common::parse_grid::<i32>("[[1,4],[4,5]]").unwrap();
        let output = common::parse_grid::<i32>("[[1,5]]").unwrap();
        assert_eq!(merge(input) , output)
    }

    #[test]
    fn test3(){
        let input = common::parse_grid::<i32>("[[1,100]]").unwrap();
        let output = common::parse_grid::<i32>("[[1,100]]").unwrap();
        assert_eq!(merge(input) , output)
    }

    #[test]
    fn test4(){
        let input = common::parse_grid::<i32>("[[1,20],[25,30],[19,26]]").unwrap();
        let output = common::parse_grid::<i32>("[[1,30]]").unwrap();
        assert_eq!(merge(input) , output)
    }

    #[test]
    fn test5(){
        let input = common::parse_grid::<i32>("[[1,1],[1,2],[2,3],[3,4],[9,9],[9,9]]").unwrap();
        let output = common::parse_grid::<i32>("[[1,4],[9,9]]").unwrap();
        assert_eq!(merge(input) , output)
    }
}