use std::io::{self, prelude::*};
use std::error::Error;






















type Res<T> = Result<T, Box<dyn Error>>;

fn main() -> Res<()> {
    run_tests(io::stdin().lock().lines())
}

/// Panics on malformed input.
fn run_tests(mut lines: impl Iterator<Item = io::Result<String>>) -> Res<()> {
    let line = lines.next().unwrap()?;
    let t = line.parse()?;
    for test_no in 1..=t {
        let mut grid = read_test_input(&mut lines)?;
        let ans = boxes_to_add(&mut grid);
        println!("Case #{}: {}", test_no, ans);
    }
    assert!(lines.next().is_none());
    Ok(())
}

/// Panics on malformed input.
fn read_test_input(lines: &mut impl Iterator<Item = io::Result<String>>) -> Res<Grid> {
    let line = lines.next().unwrap()?;
    let mut words = line.split_whitespace();
    let r: usize = words.next().unwrap().parse()?;
    let c: usize = words.next().unwrap().parse()?;
    assert!(words.next().is_none());

    let mut grid = Vec::with_capacity(r);
    for _ in 0..r {
        let line = lines.next().unwrap()?;
        let row: Vec<_> = line.split_whitespace().map(|w| w.parse::<u32>().unwrap()).collect();
        assert_eq!(row.len(), c);

        grid.push(row);
    }

    Ok(grid)
}
