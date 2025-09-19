/// Returns the factorial of a number.
/// # Examples
/// ```
/// use MathUtils::factorial;
/// let result = factorial(5);
/// assert_eq!(result, 120);
/// ```
pub fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

/// Returns the maximum common divisor of two numbers.
/// # Examples
/// ```
/// use MathUtils::max_com_div;
/// let result = max_com_div(12, 15);
/// assert_eq!(result, 3);
/// ```
pub fn max_com_div(a: u64, b: u64) -> u64 {
    let mut divisors_a = Vec::new();
    let mut divisors_b = Vec::new();
    for i in 1..=a{
        let rem = a % i;
        match rem {
            0 => divisors_a.push(i),
            _ => continue,
        }
    }
    for i in 1..=b{
        let rem = b % i;
        match rem {
            0 => divisors_b.push(i),
            _ => continue,
        }
    }
    let mut common_divisors = Vec::new();
    for i in divisors_a {
        if divisors_b.contains(&i) {
            common_divisors.push(i);
        }
    }
    *common_divisors.iter().max().unwrap()
}

