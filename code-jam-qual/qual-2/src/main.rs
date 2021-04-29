use std::io::{self, prelude::*};
use std::error::Error;
use Symbol::{C, J};

/*
Observation: can just ignore ?s

- read in list of Cs and Js
- X is cost of CJ, Y is cost of JC

- compute number of CJs and JCs
    - one-pass, count up-edges and down-edges
- dot-product with costs, and return

todo: solve the extra-credit one, which may actually involve optimization...
*/

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Symbol {
    C,
    J,
}

fn total_cost(symbols: &[Symbol], cj_cost: i32, jc_cost: i32) -> i32 {
    if cj_cost < 0 || jc_cost < 0 {
        return -1; // todo
    }

    let (cj, jc) = count_cj_jc(symbols);
    cj * cj_cost + jc * jc_cost
}

fn count_cj_jc(symbols: &[Symbol]) -> (i32, i32) {
    let mut cj = 0;
    let mut jc = 0;
    for window in symbols.windows(2) {
        match window {
            [C, J] => {
                cj += 1;
            }
            [J, C] => {
                jc += 1;
            }
            _ => (),
        };
    }
    (cj, jc)
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
        let (symbols, x, y) = read_test_input(&mut lines)?;
        let ans = total_cost(&symbols, x, y);
        println!("Case #{}: {}", test_no, ans);
    }
    assert!(lines.next().is_none());
    Ok(())
}

/// Panics on malformed input.
fn read_test_input(lines: &mut impl Iterator<Item = io::Result<String>>) -> Res<(Vec<Symbol>, i32, i32)> {
    let line = lines.next().unwrap()?;
    let mut words = line.split_whitespace();
    let x: i32 = words.next().unwrap().parse()?;
    let y: i32 = words.next().unwrap().parse()?;
    let s = words.next().unwrap();
    assert!(words.next().is_none());

    let symbols: Vec<_> = s.chars().filter_map(|c| match c {
        'C' => Some(C),
        'J' => Some(J),
        '?' => None,
        _ => panic!("Unexpected character in s: {}", c),
    }).collect();

    Ok((symbols, x, y))
}
