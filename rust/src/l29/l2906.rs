pub fn count_tested_devices(battery_percentages: Vec<i32>) -> i32 {
    let mut checked = 0;

    for v in battery_percentages {
        if v > checked {
            checked += 1;
        }
    }

    checked
}


