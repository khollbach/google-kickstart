use std::io::{self, prelude::*};
use std::error::Error;

/// Greedily append digits to make `vals` strictly increasing.
/// Return the number of digits appended.
///
/// `vals` must not contain the value `0`.
fn append_sort(vals: &mut [u64]) -> u32 {
    let old: Vec<_> = vals.iter().copied().collect();

    for i in 1..vals.len() {
        fix(vals[i - 1], &mut vals[i]);
    }

    vals.iter().zip(old.iter()).map(|(&new, &old)| {
        int_len(new) - int_len(old)
    }).sum()
}

/// Make curr strictly larger than prev, but as small as possible,
/// by appending digits.
fn fix(prev: u64, curr: &mut u64) {
    assert_ne!(*curr, 0);
    assert_ne!(prev, 0);

    let curr_len = int_len(*curr);
    let prev_len = int_len(prev);

    // Try padding zeros.
    let suffix_len = if prev_len > curr_len { prev_len - curr_len } else { 0 };
    let zero_padded = *curr * 10u64.pow(suffix_len);
    if zero_padded > prev {
        *curr = zero_padded;
        return;
    }

    // Try adding 1 to the suffix.
    let suffix = prev % 10u64.pow(suffix_len);
    if suffix != 10u64.pow(suffix_len) - 1 { // Ignore "all-9s".
        let suffix_padded = zero_padded + suffix + 1;
        if suffix_padded > prev {
            *curr = suffix_padded;
            return;
        }
    }

    // Resort to padding an extra zero.
    *curr = zero_padded * 10;
}

/// todo: test me.
fn int_len(val: u64) -> u32 {
    assert_ne!(val, 0);

    // I have no idea whether this is guaranteed to work.
    (val as f64).log10().floor() as u32
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