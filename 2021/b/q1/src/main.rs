use std::io::{self, prelude::*};
use std::error::Error;

fn longest_increasing(s: &str) -> Vec<u32> {
    let s = s.as_bytes();
    if s.is_empty() {
        return vec![];
    }

    let mut res = vec![0; s.len()];
    res[0] = 1;
    for i in 1..s.len() {
        if s[i-1] < s[i] {
            res[i] = 1 + res[i-1];
        } else {
            res[i] = 1;
        }
    }
    res
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
        let s = read_test_input(&mut lines)?;
        let ans = longest_increasing(&s);
        println!("Case #{}: {}", test_no, nums_to_string(&ans));
    }
    assert!(lines.next().is_none());
    Ok(())
}

/// Panics on malformed input.
fn read_test_input(lines: &mut impl Iterator<Item = io::Result<String>>) -> Res<String> {
    let line = lines.next().unwrap()?;
    let len: usize = line.parse()?;

    let line = lines.next().unwrap()?;
    assert_eq!(line.len(), len);
    assert!(line.chars().all(|c| c.is_ascii_uppercase()), len);
    Ok(line)
}

fn nums_to_string(nums: &[u32]) -> String {
    let mut res = String::new();
    for n in nums {
        if !res.is_empty() {
            res.push(' ');
        }
        res.push_str(&n.to_string());
    }
    res
}