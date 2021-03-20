u2se std::error::Error;
use std::io::{self, prelude::*};
use std::collections::BinaryHeap;

/// Return the minimum "difficulty score" of any workout.
///
/// We are allowed to insert up to `k` extra sessions.
fn min_workout(minutes: &[u32], k: usize) -> u32 {
    let n = minutes.len();
    assert!(n >= 2);

    // Max-heap, entries are triples: (ceil_div(d, num_copies), d, num_copies).
    let mut heap = BinaryHeap::with_capacity(n - 1);
    for i in 1..n {
        let delta = minutes[i] - minutes[i - 1];
        heap.push((delta, delta, 1));
    }

    // Greedily allocate all k extra sessions.
    for _ in 0..k {
        let (_, d, num_copies) = heap.pop().unwrap();
        let num_copies = num_copies + 1;
        heap.push((ceil_div(d, num_copies), d, num_copies));
    }

    let (difficulty, _, _) = heap.pop().unwrap();
    difficulty
}

/// Divide and round up.
fn ceil_div(a: u32, b: u32) -> u32 {
    assert_ne!(b, 0);

    let extra = if a % b != 0 { 1 } else { 0 };
    a / b + extra
}

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
        let K: usize = words.next().unwrap().parse()?;
        assert!(words.next().is_none());

        let line = lines.next().unwrap()?;
        let minutes: Vec<_> = line
            .split_whitespace()
            .map(|w| w.parse::<u32>())
            .collect::<Result<_, _>>()?;
        assert_eq!(minutes.len(), N);

        let ans = min_workout(&minutes, K);
        println!("Case #{}: {}", t, ans);
    }
    assert!(lines.next().is_none());

    Ok(())
}
