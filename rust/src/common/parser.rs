use crate::common::{Error, Result};
use std::str::FromStr;

pub fn parser_vec<T>(input: &str) -> Result<Vec<T>>
where
    T: FromStr,
    Error: From<<T as FromStr>::Err>,
{
    let result = input
        .strip_prefix('[')
        .ok_or(Error::NoPrefixOfSlice)?
        .strip_suffix(']')
        .ok_or(Error::NoAppendixOfSlice)?
        .split(',')
        .map(|s| s.trim().parse::<T>())
        .collect::<std::result::Result<Vec<_>, _>>();
    Ok(result?)
}

pub fn parse_grid<T>(input: &str) -> Result<Vec<Vec<T>>>
where
    T: FromStr,
    Error: From<<T as FromStr>::Err>,
{
    if input == "[]" {
        return Ok(vec![]);
    }
    let result = input
        .strip_prefix("[[")
        .ok_or(Error::NoPrefixOfSlice)?
        .strip_suffix("]]")
        .ok_or(Error::NoAppendixOfSlice)?
        .split("],[")
        .map(|s| parser_vec::<T>(format!("[{}]", s.trim()).as_str()))
        .collect::<std::result::Result<Vec<_>, _>>();
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser_vec() {
        let input = "[1,2,3,4,5]";
        let result = parser_vec::<i32>(input);
        assert_eq!(result, Ok(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_parse_grid() {
        let input = "[[1,2,3],[4,5,6],[7,8,9]]";
        let result = parse_grid::<i32>(input);
        assert_eq!(
            result,
            Ok(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
        );
    }
}
