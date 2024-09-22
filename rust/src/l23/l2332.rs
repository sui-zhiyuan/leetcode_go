pub fn latest_time_catch_the_bus(buses: Vec<i32>, passengers: Vec<i32>, capacity: i32) -> i32 {
    let mut buses = buses;
    let mut passengers = passengers;
    buses.sort();
    passengers.sort();

    let mut bus = 0;
    let mut remain = capacity;
    let mut last_passenger = 0;

    for &p in passengers.iter() {
        if remain == 0 {
            bus += 1;
            if bus < buses.len() {
                remain = capacity;
            };
        }
        while bus < buses.len() && p > buses[bus] {
            bus += 1;
            if bus < buses.len() {
                remain = capacity;
            };
        }
        if bus >= buses.len() {
            break;
        }
        last_passenger = p;
        remain -= 1;
    }
    let mut pos = last_passenger - 1;
    if remain > 0 || bus < buses.len() - 1 {
        pos = buses[buses.len() - 1];
    }
    let mut i = passengers.len() - 1;
    while pos <= passengers[i] {
        if pos == passengers[i] {
            pos -= 1;
        }
        if i == 0 {
            break;
        }
        i -= 1;
    }
    pos
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let buses = vec![20, 30, 10];
        let passengers = vec![19, 13, 26, 4, 25, 11, 21];
        let capacity = 2;
        assert_eq!(latest_time_catch_the_bus(buses, passengers, capacity), 20);
    }

    #[test]
    fn test2() {
        let buses = vec![3];
        let passengers = vec![2, 4];
        let capacity = 2;
        assert_eq!(latest_time_catch_the_bus(buses, passengers, capacity), 3);
    }

    #[test]
    fn test3() {
        let buses = vec![6, 8, 18, 17];
        let passengers = vec![6, 8, 17];
        let capacity = 3;
        assert_eq!(latest_time_catch_the_bus(buses, passengers, capacity), 18);
    }
}
