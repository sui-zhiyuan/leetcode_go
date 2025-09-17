use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

#[derive(Default)]
pub struct NumberContainers {
    numbers: HashMap<usize, i32>,
    locations: HashMap<i32, BinaryHeap<Reverse<usize>>>,
}

impl NumberContainers {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn change(&mut self, index: i32, number: i32) {
        let index = index as usize;
        self.numbers.insert(index, number);
        let los = &mut *self.locations.entry(number).or_default();
        los.push(Reverse(index));
    }

    pub fn find(&mut self, number: i32) -> i32 {
        loop {
            let Some(los) = self.locations.get_mut(&number) else {
                return -1;
            };

            let Some(&Reverse(top)) = los.peek() else {
                return -1;
            };

            if self.numbers[&top] == number {
                return top as i32;
            }

            los.pop();
        }
    }
}
