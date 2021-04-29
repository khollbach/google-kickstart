use std::io::{self, prelude::*};
use std::error::Error;
use std::cmp::max;

fn best_score_with_edits(nums: &mut [i32]) -> usize {
    let n = nums.len();
    assert!(n >= 2);

    let mut best = 2;

    let mut check = |i: usize, val: i32, nums: &mut [i32]| {
        let tmp = nums[i];
        nums[i] = val;
        let score = best_score(&nums);
        nums[i] = tmp;
        best = max(best, score);
    };

    for i in 0..n {
        if i >= 2 {
            let slope = nums[i - 1] - nums[i - 2];
            check(i, nums[i - 1] + slope, nums);
        }
        if i < n - 2 {
            let slope = nums[i + 2] - nums[i + 1];
            check(i, nums[i + 1] - slope, nums);
        }
        if i != 0 && i != n - 1 {
            let slope_2 = nums[i + 1] - nums[i - 1];
            if slope_2 % 2 == 0 {
                check(i, nums[i - 1] + slope_2 / 2, nums);
            }
        }
    }
    best
}

fn best_score(nums: &[i32]) -> usize {
    let n = nums.len();
    assert!(n >= 2);

    let mut prev_slope = nums[1] - nums[0];
    let mut curr_score = 2;
    let mut best_score = 2;
    for i in 2..n {
        let curr_slope = nums[i] - nums[i-1];
        if curr_slope == prev_slope {
            curr_score += 1;
            best_score = max(best_score, curr_score);
        } else {
            curr_score = 2;
        }
        prev_slope = curr_slope;
    }
    best_score
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
        let mut nums = read_test_input(&mut lines)?;
        let ans = best_score_with_edits(&mut nums);
        println!("Case #{}: {}", test_no, ans);
    }
    assert!(lines.next().is_none());
    Ok(())
}

/// Panics on malformed input.
fn read_test_input(lines: &mut impl Iterator<Item = io::Result<String>>) -> Res<Vec<i32>> {
    let line = lines.next().unwrap()?;
    let n: usize = line.parse()?;

    let line = lines.next().unwrap()?;
    let nums: Vec<i32> = line.split_whitespace().map(|word| word.parse().unwrap()).collect();
    assert_eq!(nums.len(), n);
    Ok(nums)
}