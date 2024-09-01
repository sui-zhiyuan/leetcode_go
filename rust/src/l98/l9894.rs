use std::cmp::Ordering;

pub fn min_damage(power: i32, damage: Vec<i32>, health: Vec<i32>) -> i64 {
    let mut enemies = damage
        .iter()
        .zip(health.iter())
        .map(|(d, h)| Enemy {
            time: (h + power - 1) / power,
            damage: *d,
        })
        .collect::<Vec<_>>();
    enemies.sort_unstable();

    let mut time = enemies.iter().map(|e| e.time).sum::<i32>() as i64;
    let mut result = 0i64;
    for e in enemies {
        result += e.damage as i64 * time;
        time -= e.time as i64;
    }
    result
}

#[derive(Debug, PartialEq, Eq)]
struct Enemy {
    time: i32,
    damage: i32,
}

impl PartialOrd for Enemy {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ord = (self.damage * other.time).cmp(&(other.damage * self.time));
        if ord == Ordering::Equal {
            Some(self.time.cmp(&other.time))
        } else {
            Some(ord)
        }
    }
}

impl Ord for Enemy {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
