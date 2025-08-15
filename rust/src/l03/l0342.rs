pub fn is_power_of_four(n: i32) -> bool {
    const ODD_BITS: i32 = 0x5555_5555;

    n >= 0 && n.count_ones() == 1 && n & ODD_BITS > 0
}
