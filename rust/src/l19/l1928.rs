use crate::common::OrderKeyTrait;

pub fn min_cost(max_time: i32, edges: Vec<Vec<i32>>, passing_fees: Vec<i32>) -> i32 {
    todo!()
}

struct Arrive{
    cost : i32,
    time : i32
}

impl OrderKeyTrait for Arrive{
    type TKey = i32; 





    fn order_key(&self) -> Self::TKey {
        self.cost
    }
}