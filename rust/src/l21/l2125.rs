pub fn number_of_beams(bank: Vec<String>) -> i32 {
    let mut total_beams = 0;
    let mut last_device_count = 0;

    for row in bank {
        let device_count = row.chars().filter(|&c| c == '1').count();

        if device_count == 0 {
            continue;
        }

        total_beams += device_count * last_device_count;
        last_device_count = device_count;
    }

    total_beams as i32
}
