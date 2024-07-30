pub fn cal_points(operations: Vec<String>) -> i32 {
    let mut scores = vec![];

    for o in operations{
        match o.as_str() {
            "C" => {
                scores.pop();
            },
            "D" => {
                let last = scores.last().unwrap();
                scores.push(last * 2);
            },
            "+" => {
                let last = scores.last().unwrap();
                let last2 = scores[scores.len()-2];
                scores.push(last + last2);
            },
            _ => {
                scores.push(o.parse::<i32>().unwrap());
            }
        }
    }

    scores.iter().sum()
}