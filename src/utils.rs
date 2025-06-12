pub const fn prev_power_of_two(n: u32) -> u32 {
    // n = 0 gives highest_bit_set_idx = 0.
    let highest_bit_set_idx = 31 - (n | 1).leading_zeros();
    // Binary AND of highest bit with n is a no-op, except zero gets wiped.
    (1 << highest_bit_set_idx) & n
}
