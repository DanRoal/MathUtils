//! Operations module of u128 type numbers.

/// Returns the integer square root of a u128 number using binary search.
/// # Examples
/// ```
/// use MathUtils::operations::sqrt_u128;
/// let result = sqrt_u128(17);
/// assert_eq!(result, 4);
/// ```
pub fn sqrt_u128(n: u128) -> u128 {
    if n < 2 {
        return n;
    }
    let mut left = 1;
    let mut right = n / 2;
    while left <= right {
        let mid = left + (right - left) / 2;
        let mid_sq = mid * mid;
        if mid_sq == n {
            return mid;
        } else if mid_sq < n {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    right
}