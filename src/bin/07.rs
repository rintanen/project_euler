use primes::{Sieve, PrimeSet};


fn main() {
    // What is the 10 001st prime number?
    let n = 10001;
    let nth_prime: u64 = Sieve::new().iter().nth(n).unwrap();
    println!("{n}th prime is: {nth_prime}");
}