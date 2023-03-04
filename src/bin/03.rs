use primes::{Sieve, PrimeSet};
use num_integer::{div_rem, Roots};

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false
    }

    for i in 2..n.sqrt() as u64 + 1 {
        if n % i == 0 {
            return false
        }
    }
    true
}

fn prime_factorization(mut num: u64) -> Vec<u64> {

    if is_prime(num) {
        return vec![num];
    }

    let mut factorials = Vec::new();
    let mut prime_sieve = Sieve::new();
    let mut pset = prime_sieve.iter();
    let mut prime = pset.next().unwrap();
    loop {
        let (div, remainder) = div_rem(num, prime);
        if remainder == 0 {
            factorials.push(prime.clone());
            if is_prime(div) {
                factorials.push(div);
                break;
            }
            num = div;
        } else {
            prime = pset.next().unwrap();
        }
    }
    factorials
}

fn main() {
    let fact = prime_factorization(600851475143);

    fact.iter().for_each(|f| print!("{} ", f));
    println!("");
}    
