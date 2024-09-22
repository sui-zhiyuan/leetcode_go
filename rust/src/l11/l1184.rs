use std::iter::once;

pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
    let dis = distance.iter().scan(0, |acc, &x| {
        *acc += x;
        Some(*acc)
    });

    let dis = once(0).chain(dis).collect::<Vec<_>>();
    let up_dis = if start < destination {
        dis[destination as usize] - dis[start as usize]
    } else {
        dis[start as usize] - dis[destination as usize]
    };

    let down_dis = dis.last().unwrap() - up_dis;
    up_dis.min(down_dis)
}