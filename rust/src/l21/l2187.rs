use crate::common::binary_search;

pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
    binary_search(0, time[0] as i64 * total_trips as i64 , |x| {
        finished_tripes(&time, x) >= total_trips as i64
    })
}

fn finished_tripes(time: &[i32] , time_limit:i64) -> i64 {
    let mut trips = 0;

    for &t in time {
        trips += time_limit / t as i64;
    }

    trips
}
