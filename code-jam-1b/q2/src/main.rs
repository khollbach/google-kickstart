use std::io::{self, prelude::*};
use std::error::Error;
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

/// Generate the first few primes.
fn gen_primes() -> [u32; NUM_PRIMES] {
    let mut is_prime = [true; LARGEST_PRIME as usize + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for p in 2..=LARGEST_PRIME {
        if is_prime[p as usize] {
            // Wipe out all multiples of p.
            for mult in 2.. {
                if mult * p > LARGEST_PRIME {
                    break;
                }
                is_prime[(mult * p) as usize] = false;
            }
        }
    }

    let primes: Vec<_> = (0..=LARGEST_PRIME).filter(|&p| is_prime[p as usize]).collect();
    assert_eq!(primes.len(), NUM_PRIMES);

    let mut primes_arr = [0; NUM_PRIMES];
    primes_arr.copy_from_slice(&primes);
    primes_arr
}

const LARGEST_PRIME: u32 = 499;
const NUM_PRIMES: usize = 95;

#[derive(Debug, Copy, Clone)]
struct Factorization {
    multiplicity: [u32; NUM_PRIMES],
}

impl Factorization {
    fn empty() -> Self {
        Self {
            multiplicity: [0; NUM_PRIMES],
        }
    }

    fn primes(&self, _primes: &[u32; NUM_PRIMES]) -> impl Iterator<Item = usize> {
        //(0..NUM_PRIMES).filter(|i| self.multiplicity[i] != 0).map(|i| primes[i])
        let m = self.multiplicity;
        (0..NUM_PRIMES).filter(move |&i| m[i] != 0)
    }

    fn sum_values(&self, primes: &[u32; NUM_PRIMES]) -> u32 { // todo: overflows?
        self.multiplicity.iter().enumerate().map(|(i, m)| m * primes[i]).sum()
    }

    fn minus(&self, other: Self) -> Self {
        let mut multiplicity = self.multiplicity;
        for i in 0..NUM_PRIMES {
            multiplicity[i] -= other.multiplicity[i];
        }

        Self {
            multiplicity
        }
    }
}

fn best_score(primes: Factorization) -> u32 {
    #[allow(non_snake_case)]
    let PRIMES = gen_primes();

    let mut q = BinaryHeap::new();
    q.push(Reverse(1));

    let mut factors = HashMap::<u32, Factorization>::new();
    factors.insert(1, Factorization::empty());

    let upper_bound = primes.sum_values(&PRIMES);

    let mut best = 0;

    while let Some(Reverse(n)) = q.pop() {
        if n > upper_bound {
            break;
        }

        let my_factors = factors[&n];

        let remaining = primes.minus(my_factors);
        let remaining_sum = remaining.sum_values(&PRIMES);
        if remaining_sum == n && n > best {
            best = n;
        }

        for i in remaining.primes(&PRIMES) {
            let new = n * PRIMES[i];
            if !factors.contains_key(&new) {
                q.push(Reverse(new));

                let mut new_factors = my_factors;
                new_factors.multiplicity[i] += 1;
                factors.insert(new, new_factors);
            }
        }
    }

    best
}

type Res<T> = Result<T, Box<dyn Error>>;

fn main() -> Res<()> {
    let primes = gen_primes();
    run_tests(io::stdin().lock().lines(), &primes)
}

/// Panics on malformed input.
fn run_tests(mut lines: impl Iterator<Item = io::Result<String>>, primes: &[u32; NUM_PRIMES]) -> Res<()> {
    let line = lines.next().unwrap()?;
    let t: u32 = line.parse()?;
    for test_no in 1..=t {
        let primes = read_test_input(&mut lines, primes)?;
        let ans = best_score(primes);
        println!("Case #{}: {}", test_no, ans);
    }
    assert!(lines.next().is_none());
    Ok(())
}

/// Panics on malformed input.
fn read_test_input(lines: &mut impl Iterator<Item = io::Result<String>>, primes: &[u32; NUM_PRIMES]) -> Res<Factorization> {
    let line = lines.next().unwrap()?;
    let m: usize = line.parse()?;

    let mut fact = Factorization::empty();
    for _ in 0..m {
        let line = lines.next().unwrap()?;
        let mut words = line.split_whitespace();

        let p: u32 = words.next().unwrap().parse()?;
        let mult: u32 = words.next().unwrap().parse()?;

        let idx = primes.iter().position(|&p2| p2 == p).unwrap();
        fact.multiplicity[idx] += mult;
    }
    Ok(fact)
}
