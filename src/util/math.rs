/// Returns the floor division of `a` by `b`.
/// This is equivalent to `a.div_euclid(b)` in Rust.
///
/// # Panics
/// Panics if `b` is zero.
#[inline]
pub fn div_floor(a: i64, b: i64) -> i64 {
    a.div_euclid(b)
}

/// Returns the modulus after floor division of `a` by `b`.
/// This is equivalent to `a.rem_euclid(b)` in Rust.
///
/// # Panics
/// Panics if `b` is zero.
#[inline]
pub fn mod_floor(a: i64, b: i64) -> i64 {
    a.rem_euclid(b)
}
