
/*
n: 10
k: 3
k_cd: 3 2 1
[ 5 4 3 2 1 3 2 3 3 1 ]
                  ^
 */

use std::io::{self, prelude::*};
use std::error::Error;

/// We could re-write this to pass through the array only once. Right now, the worst case is
/// when `arr.len() == k`, where we do two whole passes.
fn num_k_countdowns(arr: &[u32], k: u32) -> usize {
    assert_ne!(k, 0);
    let n = arr.len();
    if k as usize > n {
        return 0;
    }
    let k_countdown = (1..=k).rev();

    let is_k_countdown = |i| -> bool {
        arr[i..i + k as usize].iter().copied().eq(k_countdown.clone())
    };

    (0..arr.len() - k as usize + 1).filter(|&i| is_k_countdown(i)).count()
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
        let ans = one_test(&mut lines)?;
        println!("Case #{}: {}", test_no, ans);
    }
    Ok(())
}

/// Panics on malformed input.
fn one_test(lines: &mut impl Iterator<Item = io::Result<String>>) -> Res<usize> {
    let line = lines.next().unwrap()?;
    let mut words = line.split_whitespace();
    let n: usize = words.next().unwrap().parse()?;
    let k: u32 = words.next().unwrap().parse()?;
    assert!(words.next().is_none());

    let line = lines.next().unwrap()?;
    let words = line.split_whitespace();
    let arr: Vec<_> = words.map(|w| w.parse::<u32>()).collect::<Result<_, _>>()?;
    assert_eq!(arr.len(), n);

    Ok(num_k_countdowns(&arr, k))
}
