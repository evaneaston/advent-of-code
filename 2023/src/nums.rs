pub(crate) fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub(crate) fn lcm_of_two(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

pub(crate) fn lcm_of_multiple(numbers: &[u64]) -> u64 {
    numbers.iter().cloned().fold(1, lcm_of_two)
}
