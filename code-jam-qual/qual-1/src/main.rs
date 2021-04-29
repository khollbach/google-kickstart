use std::io::{self, prelude::*};
use std::error::Error;

/*
- simulate algo on input
- each reverse, add that length to the count
*/

fn reversort(vals: &mut [u32]) -> usize {
    let n = vals.len();

    let mut cost = 0;
    for i in 0..n-1 {
        let (j, _) = vals.iter().enumerate().skip(i).min_by_key(|&(_, &val)| val).unwrap();
        cost += reverse(&mut vals[i..=j]);
    }
    cost
}

fn reverse(vals: &mut [u32]) -> usize {
    vals.reverse();
    vals.len()
}

type Res<T> = Result<T, Box<dyn Error>>;

fn main() -> Res<()> {
    run_tests(io::stdin().lock().lines())
}

/// Panics on malformed input.
fn run_tests(mut lines: impl Iterator<Item = io::Result<String>>) -> Res<()> {
    let line = lines.next().unwrap()?;
    let t = line.parse()?;
    for test_no in 1..=t {
        let mut vals = read_test_input(&mut lines)?;
        let ans = reversort(&mut vals);
        println!("Case #{}: {}", test_no, ans);
    }
    assert!(lines.next().is_none());
    Ok(())
}

/// Panics on malformed input.
fn read_test_input(lines: &mut impl Iterator<Item = io::Result<String>>) -> Res<Vec<u32>> {
    let line = lines.next().unwrap()?;
    let n: usize = line.parse()?;

    let line = lines.next().unwrap()?;
    let vals: Vec<_> = line
        .split_whitespace()
        .map(|word| word.parse::<u32>())
        .collect::<Result<_, _>>()?;
    assert_eq!(vals.len(), n);

    Ok(vals)
}
