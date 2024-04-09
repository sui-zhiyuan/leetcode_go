impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let chars = s.chars().collect::<Vec<_>>();
        let mut result = Vec::new();
        if chars.len() < 4 || chars.len() > 12 {
            return result;
        }
        for i1 in 1..chars.len() - 2 {
            for i2 in i1 + 1..chars.len() - 1 {
                for i3 in i2 + 1..chars.len() {
                    let values = vec![
                        chars[0..i1].iter().collect::<String>(),
                        chars[i1..i2].iter().collect::<String>(),
                        chars[i2..i3].iter().collect::<String>(),
                        chars[i3..].iter().collect::<String>(),
                    ];
                    if values.iter().all(|v| Self::valid_ip_part(v)) {
                        result.push(values.join("."));
                    }
                }
            }
        }
        result
    }

    fn valid_ip_part(s: &str) -> bool {
        assert!(!s.is_empty());
        if s.len() > 1 && s.chars().nth(0).unwrap() == '0' {
            return false;
        }
        let v = match s.parse::<u32>() {
            Ok(v) => v,
            Err(_) => return false,
        };
        v <= 255
    }
}

pub struct Solution();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::restore_ip_addresses("0".to_string()),
            Vec::<String>::new()
        );
        assert_eq!(
            Solution::restore_ip_addresses("25525511135".to_string()),
            vec!["255.255.11.135", "255.255.111.35"]
        );

        assert_eq!(
            Solution::restore_ip_addresses("0000".to_string()),
            vec!["0.0.0.0"]
        );

        assert_eq!(
            Solution::restore_ip_addresses("101023".to_string()),
            vec![
                "1.0.10.23",
                "1.0.102.3",
                "10.1.0.23",
                "10.10.2.3",
                "101.0.2.3"
            ]
        );
    }
}
