// Problem 3

/// Find all prime numbers less than `n`.
/// For example, `sieve(7)` should return `[2, 3, 5]`
pub fn sieve(n: u32) -> Vec<u32> {
    let mut numbers: Vec<u32> = (2..n).collect();
    let mut primes = Vec::new();

    while !numbers.is_empty() {
        let p = numbers[0];
        primes.push(p);
        numbers.retain(|&x| x % p != 0);
    }

    primes
}
