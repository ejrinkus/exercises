pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 1;
    }
    while b != 0 {
        if b < a {
            std::mem::swap(&mut b, &mut a);
        }
        b %= a;
    }
    a
}

pub fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}
