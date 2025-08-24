#[inline]
pub fn div_floor(a: i64, b: i64) -> i64 {
    let q = a / b;
    let r = a % b;
    if (r != 0) && ((r > 0) != (b > 0)) { q - 1 } else { q }
}
#[inline]
pub fn mod_floor(a: i64, b: i64) -> i64 {
    let r = a % b;
    if (r < 0 && b > 0) || (r > 0 && b < 0) { r + b } else { r }
}

