use primes::{Sieve, PrimeSet};


fn main() {
    // What is the 10 001st prime number?
    let n = 10001;
    let mut psieve = Sieve::new();
    let nth_prime: u64 = psieve.iter().skip(n-1).take(1).next().unwrap();
    println!("{n}th prime is: {nth_prime}");
}