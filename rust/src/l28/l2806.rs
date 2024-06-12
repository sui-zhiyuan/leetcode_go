pub fn account_balance_after_purchase(purchase_amount: i32) -> i32 {
    let cost = purchase_amount / 10 * 10 + if purchase_amount % 10 >=5 { 10 } else { 0 };
    100 - cost
}