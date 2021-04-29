use std::io::{self, prelude::*};
use std::error::Error;
use std::cmp::min;

fn reversort(vals: &mut [u32]) -> usize {
    let n = vals.len();
    let mut cost = 0;

    for i in 0..n - 1 {
        let (j, _) = vals.iter().enumerate().skip(i).min_by_key(|&(_, &val)| val).unwrap();

        let slice = &mut vals[i..=j];
        slice.reverse();
        cost += slice.len();
    }

    cost
}

fn array_with_cost(n: usize, target: u32) -> Option<Vec<u32>> {
    assert!(n >= 2);
    if (target as usize) < n - 1 {
        return None;
    }

    let mut step_costs = distribute_cost_increasing(n - 1, target - (n as u32 - 1))?;
    for c in &mut step_costs {
        *c += 1;
    }
    step_costs.reverse();

    let result = array_with_reversort_costs(&step_costs);

    // Sanity check.
    if cfg!(debug_assertions) {
        let mut sorted = result.clone();
        let cost = reversort(&mut sorted);
        assert_eq!(cost, target as usize);
    }

    Some(result)
}

/// Return a vector of `n` numbers that sum to `total_cost`, such that
/// `0 <= result[i] <= i + 1`.
///
/// If impossible, return None.
fn distribute_cost_increasing(n: usize, mut total_cost: u32) -> Option<Vec<u32>> {
    let mut costs = Vec::with_capacity(n);

    for i in 0..n as u32 {
        let amount = min(i + 1, total_cost);
        total_cost -= amount;

        costs.push(amount);
    }

    if total_cost == 0 {
        Some(costs)
    } else {
        None
    }
}

/// Output an array with these exact reversort costs at each iteration of the algorithm.
fn array_with_reversort_costs(step_costs: &[u32]) -> Vec<u32> {
    let n = step_costs.len() + 1;

    let mut result: Vec<_> = (1..=n as u32).collect();

    for i in (0..n - 1).rev() {
        result[i..i + step_costs[i] as usize].reverse();
    }

    result
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
        let (n, c) = read_test_input(&mut lines)?;
        let ans = match array_with_cost(n, c) {
            Some(nums) => display_nums(&nums),
            None => String::from("IMPOSSIBLE"),
        };
        println!("Case #{}: {}", test_no, ans);
    }
    assert!(lines.next().is_none());
    Ok(())
}

/// Panics on malformed input.
fn read_test_input(lines: &mut impl Iterator<Item = io::Result<String>>) -> Res<(usize, u32)> {
    let line = lines.next().unwrap()?;
    let mut words = line.split_whitespace();
    let n: usize = words.next().unwrap().parse()?;
    let c: u32 = words.next().unwrap().parse()?;
    assert!(words.next().is_none());

    Ok((n, c))
}

/// Space-separated.
fn display_nums(nums: &[u32]) -> String {
    let mut result = String::new();
    for (i, n) in nums.iter().enumerate() {
        if i != 0 {
            result.push(' ');
        }
        result.push_str(&n.to_string());
    }
    result
}
