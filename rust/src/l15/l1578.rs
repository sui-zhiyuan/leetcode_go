pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
    let ballons = colors
        .chars()
        .zip(needed_time.iter())
        .map(|(c, &t)| Ballon { color: c, time: t })
        .collect::<Vec<_>>();

    let mut curr = 0;
    let mut result = 0;
    while curr < ballons.len() {
        let curr_color = ballons[curr].color;
        let mut sum_time = 0;
        let mut max_time = 0;

        while curr < ballons.len() && ballons[curr].color == curr_color {
            sum_time += ballons[curr].time;
            max_time = max_time.max(ballons[curr].time);
            curr += 1;
        }

        result += sum_time - max_time;
    }

    result
}

struct Ballon {
    color: char,
    time: i32,
}
