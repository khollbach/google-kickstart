use std::io::{self, prelude::*};
use std::error::Error;























type Res<T> = Result<T, Box<dyn Error>>;

fn main() -> Res<()> {
    run_tests(io::stdin().lock().lines())
}

/// Panics on malformed input.
fn run_tests(mut lines: impl Iterator<Item = io::Result<String>>) -> Res<()> {
    let line = lines.next().unwrap()?;
    let t: u32 = line.parse()?;
    for test_no in 1..=t {
        let mut vals = read_test_input(&mut lines)?;
        let ans = append_sort(&mut vals);
        println!("Case #{}: {}", test_no, ans);
    }
    assert!(lines.next().is_none());
    Ok(())
}

/// Panics on malformed input.
fn read_test_input(lines: &mut impl Iterator<Item = io::Result<String>>) -> Res<Vec<u64>> {
    let line = lines.next().unwrap()?;
    let n: usize = line.parse()?;

    let line = lines.next().unwrap()?;
    let vals: Vec<u64> = line.split_whitespace().map(|word| word.parse().unwrap()).collect();
    assert_eq!(vals.len(), n);
    Ok(vals)
}