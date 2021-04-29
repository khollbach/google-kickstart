use std::io::{self, prelude::*};
use std::error::Error;

fn pogo_jump(mut x: i32, mut y: i32) -> String {
    assert_ne!((x, y), (0, 0));

    dbg!(x, y);

    let mut res = String::new();

    while (x, y) != (0, 0) {
        let px = x.abs() % 2; // parity
        let py = y.abs() % 2;
        if px == py {
            return String::from("IMPOSSIBLE");
        } else if px == 1 {
            if x > 0 {
                res.push('E');
            } else {
                res.push('W');
            }
        } else {
            debug_assert_eq!(py, 1);
            if y > 0 {
                res.push('N');
            } else {
                res.push('S');
            }
        }

        x /= 2;
        y /= 2;
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
        let (x, y) = read_test_input(&mut lines)?;
        let ans = pogo_jump(x, y);
        println!("Case #{}: {}", test_no, ans);
        panic!();
    }
    assert!(lines.next().is_none());
    Ok(())
}

/// Panics on malformed input.
fn read_test_input(lines: &mut impl Iterator<Item = io::Result<String>>) -> Res<(i32, i32)> {
    let line = lines.next().unwrap()?;
    let vals: Vec<i32> = line.split_whitespace().map(|word| word.parse().unwrap()).collect();
    assert_eq!(vals.len(), 2);
    Ok((vals[0], vals[1]))
}