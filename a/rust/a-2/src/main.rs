use std::error::Error;
use std::io::{self, prelude::*};

/// What is the best beauty score possible, when taking `num_plates` from `stacks`?
///
/// `stacks` must be a non-empty square matrix.
fn best_beauty_score(stacks: &Vec<Vec<u32>>, num_plates: usize) -> u32 {
    let n = stacks.len();
    assert_ne!(n, 0);
    let k = stacks[0].len();
    assert_ne!(k, 0);

    // Otherwise, the problem is impossible.
    assert!(num_plates <= n * k);

    // Indexed as `best[i in 0..=n][p in 0..=num_plates]`.
    // This represents the best score in the subproblem that needs `p` plates,
    // but is restricted to `stacks[i..]`.
    //
    // We use f64 instead of u32 so we can represent `-infty`, which means
    // that a particular sub-problem is impossible to solve.
    let mut best = vec![vec![0.0; num_plates + 1]; n + 1];

    // Base cases.
    best[n][0] = 0.0;
    for p in 1..=num_plates {
        best[n][p] = f64::NEG_INFINITY;
    }

    for i in (0..=n - 1).rev() {
        for p in 0..=num_plates {
            let low = if p >= k { p - k } else { 0 };
            best[i][p] = (low..=p)
                .map(|j| stacks[i][..p - j].iter().sum::<u32>() as f64 + best[i + 1][j])
                .fold(f64::NEG_INFINITY, f64::max);
        }
    }

    let ans = best[0][num_plates];
    // This follows from the assumption that `num_plates <= n * k`.
    assert!(ans.is_finite());
    ans as u32
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
        let P: usize = words.next().unwrap().parse()?;
        assert!(words.next().is_none());

        let mut stacks = Vec::with_capacity(N);
        for _ in 0..N {
            let line = lines.next().unwrap()?;
            let stack: Vec<_> = line
                .split_whitespace()
                .map(|w| w.parse::<u32>())
                .collect::<Result<_, _>>()?;
            assert_eq!(stack.len(), K);
            stacks.push(stack);
        }

        let ans = best_beauty_score(&stacks, P);
        println!("Case #{}: {}", t, ans);
    }
    assert!(lines.next().is_none());

    Ok(())
}
