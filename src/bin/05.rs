use primes::{Sieve, PrimeSet};
use num_integer::{div_rem, Roots};
use std::collections::{HashMap, HashSet};


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
    if num == 1 || is_prime(num) {
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


// how many occurances of n in v
fn n_count(v: &Vec<u64>, n: u64) -> usize {
    v.iter().filter(|&&i| i == n).count()
}


fn main() {
    // What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
    // i.e. least common multiple, do the prime factorization method
    /*
    https://www.cuemath.com/numbers/lcm-least-common-multiple/
    Step 1: Find the prime factors of the given numbers by repeated division method.
    Step 2: Write the numbers in their exponent form. Find the product of only those prime factors that have the highest power.
    Step 3: The product of these factors with the highest powers is the LCM of the given numbers.
    */

    let numbers: Vec<u64> = (1..=20).collect();
    let mut prime_factorizations: Vec<Vec<u64>> = vec![];
    
    // 1. prime factorize each number
    for n in numbers {
        prime_factorizations.push(prime_factorization(n.clone()));
    }

    // 2. 
    // collect maximum power of each prime number over all the factorizations 
    let mut powers_count: HashMap<u64, usize> = HashMap::new();
    for pf in prime_factorizations {
        // find which prime numbers appear in each factorization 
        let unique = pf.iter().cloned().collect::<HashSet<u64>>();

        // update powers_count if any prime number occurs more times in this factorization than in the previous
        for u in unique {
            powers_count
                .entry(u)
                .and_modify(|v| {
                    let u_count = n_count(&pf, u);
                    if *v < u_count {
                        *v = u_count;
                    }
                })
                .or_insert(n_count(&pf, u));
        }
    }

    // 3. Product of the prime factors with the highest powers is the LCM 
    let mut lcm = 1;
    for (key, value) in powers_count.into_iter() {
        lcm *= key.pow(value as u32);
    }

    println!("LCM = {}", lcm);
}