pub fn minimum_refill(plants: Vec<i32>, capacity_a: i32, capacity_b: i32) -> i32 {
    let mut refill = 0;

    let mut pa = 0;
    let mut pb= plants.len()-1;
    let mut ra = capacity_a;
    let mut rb = capacity_b;

    while pa < pb {
        if ra < plants[pa] {
            ra = capacity_a;
            refill += 1;
        }
        if rb < plants[pb] {
            rb = capacity_b;
            refill += 1;
        }
        ra -= plants[pa];
        rb -= plants[pb];
        pa += 1;
        pb -= 1;
    }

    if pa == pb && ra.max(rb) < plants[pa] {
        refill += 1;
    }

    refill
}