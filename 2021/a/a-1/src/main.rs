use std::io::{self, prelude::*};
use std::error::Error;
use std::mem;

/*
n: 6
CAB ABC
g: 2
t: 0
*/

fn goodness_difference(s: &[u8], target: usize) -> usize {
    let n = s.len();
    assert!(target <= n / 2);

    let goodness = (0..n / 2).filter(|&i| s[i] != s[n - i - 1]).count();

    abs_diff(goodness, target)
}

fn abs_diff(mut a: usize, mut b: usize) -> usize {
    if a > b {
        mem::swap(&mut a, &mut b);
    }
    b - a
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
        let (s, k)  = read_test_input(&mut lines)?;
        let ans = goodness_difference(&s.as_bytes(), k);
        println!("Case #{}: {}", test_no, ans);
    }
    assert!(lines.next().is_none());
    Ok(())
}

/// Panics on malformed input.
fn read_test_input(lines: &mut impl Iterator<Item = io::Result<String>>) -> Res<(String, usize)> {
    let line = lines.next().unwrap()?;
    let mut words = line.split_whitespace();
    let n: usize = words.next().unwrap().parse()?;
    let k: usize = words.next().unwrap().parse()?;
    assert!(words.next().is_none());

    let s = lines.next().unwrap()?;
    assert_eq!(s.len(), n);

    Ok((s, k))
}
