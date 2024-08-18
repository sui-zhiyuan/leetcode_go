pub fn check_record(s: String) -> bool {
    let absent = s.chars().filter(|v| *v == 'A').count();
    if absent >= 2 {
        return false;
    }

    let (_, late) = s
        .chars()
        .fold((0, 0), |(mut count, mut max_count), next| {
            if next == 'L' {
                count += 1;
                max_count = max_count.max(count);
            } else {
                count = 0;
            }
            (count, max_count)
        });

    if late >= 3 {
        return false;
    }

    true
}
