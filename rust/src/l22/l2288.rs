pub fn discount_prices(sentence: String, discount: i32) -> String {
    sentence
        .split_ascii_whitespace()
        .map(|v| {
            if !v.starts_with('$') {
                return v.to_string();
            }

            const INVALID_FLOAT: [&str; 5] = ["E", "e" , "inf" , "infinity" , "NaN"];
            for i in &INVALID_FLOAT {
                if v.contains(i) {
                    return v.to_string();
                }
            }

            if let Ok(price) = v[1..].parse::<f64>() {
                let np = price * (100 - discount) as f64 / 100f64;
                return format!("${np:.2}");
            }

            v.to_string()
        })
        .fold("".to_owned(), |mut acc, v| {
            if acc.is_empty() {
                acc.push_str(&v);
                return acc;
            }
            acc.push(' ');
            acc.push_str(&v);
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let sentence = "there are $1 $2 and 5$ candies in the shop".to_string();
        let discount = 50;
        let res = discount_prices(sentence, discount);
        assert_eq!(res, "there are $0.50 $1.00 and 5$ candies in the shop");
    }
}
