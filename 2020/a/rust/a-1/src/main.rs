use std::error::Error;
use std::io::{self, prelude::*};

fn main() -> Result<(), Box<dyn Error>> {
    run_tests(io::stdin().lock())
}

/// Panics if the input isn't correctly formatted.
#[allow(non_snake_case)]
fn run_tests(input: impl BufRead) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines();

    let T: u32 = lines.next().unwrap()?.parse()?;
    for t in 1..=T {
        let line = lines.next().unwrap()?;
        let mut words = line.split_whitespace();
        let N: usize = words.next().unwrap().parse()?;
        let B: u32 = words.next().unwrap().parse()?;
        assert!(words.next().is_none());

        let line = lines.next().unwrap()?;
        let A: Vec<_> = line
            .split_whitespace()
            .map(|w| w.parse::<u32>())
            .collect::<Result<_, _>>()?;
        assert_eq!(A.len(), N);

        let ans = max_num_purchases(A, B);
        println!("Case #{}: {}", t, ans);
    }
    assert!(lines.next().is_none());

    Ok(())
}

fn max_num_purchases(mut prices: Vec<u32>, mut budget: u32) -> usize {
    let n = prices.len();
    prices.sort_unstable();

    for (i, p) in prices.into_iter().enumerate() {
        if p <= budget {
            budget -= p;
        } else {
            return i;
        }
    }

    n
}
