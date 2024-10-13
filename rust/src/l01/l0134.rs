pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let gain = gas.iter().zip(cost.iter()).map(|(g, c)| g - c).collect::<Vec<i32>>();
    let n = gain.len();

    let mut start = 0;
    while start < n {
        let mut remain = 0;
        let mut end = start;
        while remain >= 0 && end < start + n {
            remain += gain[end % n];
            end += 1;
        }

        if end == start + n && remain >= 0 {
            return start as i32;
        }

        start = end;
    }

    -1
}