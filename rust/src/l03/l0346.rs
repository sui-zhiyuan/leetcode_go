pub struct MovingAverage {
    cache: Vec<i32>,
    curr:usize,
    sum:i32,
    count:i32,
}

impl MovingAverage {
    pub fn new(size: i32) -> Self {
        MovingAverage {
            cache: vec![0; size as usize],
            curr:0,
            sum:0,
            count:0,
        }
    }

    pub fn next(&mut self, val: i32) -> f64 {
        self.sum -= self.cache[self.curr];
        self.sum  += val;
        self.cache[self.curr] = val;
        if self.count < self.cache.len() as i32 {
            self.count += 1;
        }
        self.curr = (self.curr + 1) % self.cache.len();
        self.sum as f64 / self.count as f64
    }
}
