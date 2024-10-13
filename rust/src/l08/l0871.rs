use std::collections::BinaryHeap;

pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
    let mut stations: Vec<Station> = stations
        .iter()
        .map(|s| Station {
            distance: s[0],
            fuel: s[1],
        })
        .filter(|s| s.distance < target)
        .chain([Station { distance: target, fuel: 0 }])
        .collect();
    stations.sort_by_key(|s| s.distance);

    let mut heap = BinaryHeap::new();
    let mut distance = start_fuel;
    let mut stopped = 0;
    for station in stations.iter() {
        while distance < station.distance {
            let Some(fuel) = heap.pop() else {
                return -1;
            };
            distance += fuel;
            stopped += 1;
        }

        if station.distance <= distance {
            heap.push(station.fuel);
            continue;
        }
    }

    stopped
}

struct Station {
    distance: i32,
    fuel: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let target = 1;
        let start_fuel = 1;
        let stations = [[]; 0];
        let stations = stations.iter().map(|s| s.to_vec()).collect();
        assert_eq!(min_refuel_stops(target, start_fuel, stations), 0);
    }

    #[test]
    fn test2() {
        let target = 100;
        let start_fuel = 1;
        let stations = [[10, 100]];
        let stations = stations.iter().map(|s| s.to_vec()).collect();
        assert_eq!(min_refuel_stops(target, start_fuel, stations), -1);
    }

    #[test]
    fn test3() {
        let target = 100;
        let start_fuel = 10;
        let stations = [[10, 60], [20, 30], [30, 30], [60, 40]];
        let stations = stations.iter().map(|s| s.to_vec()).collect();
        assert_eq!(min_refuel_stops(target, start_fuel, stations), 2);
    }
}
