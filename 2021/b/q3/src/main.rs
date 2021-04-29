use std::io::{self, prelude::*};
use std::error::Error;

// todo: implementing isqrt so we can modify our existing code to use u64 instead of u32.
//  (I expect it would just get TLE anyways, but I'm still curious to see it for myself.)
fn isqrt(x: u64) -> u64 {
    const MAX: u64 = u32::MAX as u64; // inclusive

    let mut low = 0;
    let mut high = MAX;
    while low < high {
        let mid = low + high / 2;
        if mid * mid <= x {

        } else {

        }
    }
    low
}

/*
- consider sqrt(z)
- find largest prime <= it
- find smallest prime > it
- if their product is <= z, return
- else find largest prime <= the first prime
- and return their product instead
*/
fn largest_product_of_primes(z: u64) -> u64 {
    let guess = (z as f64).sqrt().floor() as u32;

    let mut i = guess;
    let p1 = loop {
        if is_prime(i) {
            break i;
        }
        i -= 1;
    };

    let mut i = guess + 1;
    let p2 = loop {
        if is_prime(i) {
            break i;
        }
        i += 1;
    };

    if p1 * p2 <= z {
        return p1 * p2;
    }

    let mut i = p1 - 1;
    let p0 = loop {
        if is_prime(i) {
            break i;
        }
        i -= 1;
    };

    p0 * p1
}

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        false;
    }

    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }

        i += 1;
    }
    true
}

type Res<T> = Result<T, Box<dyn Error>>;

fn main() -> Res<()> {
    run_tests(io::stdin().lock().lines())
}

/// Panics on malformed input.
fn run_tests(mut lines: impl Iterator<Item = io::Result<String>>) -> Res<()> {
    let line = lines.next().unwrap()?;
    let t: u32 = line.parse()?;
    for test_no in 1..=t {
        let z = read_test_input(&mut lines)?;
        let ans = largest_product_of_primes(z);
        println!("Case #{}: {}", test_no, ans);
    }
    assert!(lines.next().is_none());
    Ok(())
}

/// Panics on malformed input.
fn read_test_input(lines: &mut impl Iterator<Item = io::Result<String>>) -> Res<u32> {
    let line = lines.next().unwrap()?;
    let z: u32 = line.parse()?;
    Ok(z)
}