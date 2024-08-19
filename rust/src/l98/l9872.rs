pub fn max_energy_boost(energy_drink_a: Vec<i32>, energy_drink_b: Vec<i32>) -> i64 {
    let n = energy_drink_a.len();
    let mut state = vec![
        State {
            drink_a: 0,
            drink_b: 0
        };
        n
    ];
    state[0] = State {
        drink_a: energy_drink_a[0] as i64,
        drink_b: energy_drink_b[0] as i64,
    };
    state[1] = State {
        drink_a: energy_drink_a[0] as i64 + energy_drink_a[1] as i64,
        drink_b: energy_drink_b[0] as i64 + energy_drink_b[1] as i64,
    };

    for i in 2..n {
        state[i].drink_a = (state[i-1].drink_a + energy_drink_a[i] as i64).max(state[i-2].drink_b + energy_drink_a[i] as i64);
        state[i].drink_b = (state[i-1].drink_b + energy_drink_b[i] as i64).max(state[i-2].drink_a + energy_drink_b[i] as i64);
    }
    
    state[n-1].drink_a.max(state[n-1].drink_b)
}

#[derive(Debug, Clone, Copy)]
struct State {
    drink_a: i64,
    drink_b: i64,
}
