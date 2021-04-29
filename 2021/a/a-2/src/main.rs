#![allow(non_snake_case)]

use std::io::{self, prelude::*};
use std::error::Error;
use std::cmp::min;

/// Must be non-empty.
type Grid = Vec<Vec<bool>>;
type Data = Vec<Vec<LOS>>;

/// Line of sight, from a given cell.
/// All 0 if the cell is false; else all at least 1.
#[derive(Debug, Clone, Copy, Default)]
struct LOS {
    up: u32,
    down: u32,
    left: u32,
    right: u32,
}

fn total_Ls(grid: &Grid) -> u32 {
    let data = compute_los(grid);

    let mut total = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            total += cell_Ls(&data, i, j);
        }
    }
    total
}

fn cell_Ls(data: &Data, r: usize, c: usize) -> u32 {
    let data = data[r][c];

    arm_Ls(data.up, data.left) +
    arm_Ls(data.up, data.right) +
    arm_Ls(data.down, data.left) +
    arm_Ls(data.down, data.right)
}

fn arm_Ls(a: u32, b: u32) -> u32 {
    flat_sub(min(a, b / 2), 1) +
    flat_sub(min(b, a / 2), 1)
}

fn flat_sub(a: u32, b: u32) -> u32 {
    if a >= b {
        a - b
    } else {
        0
    }
}

fn compute_los(grid: &Grid) -> Data {
    let mut data = vec![vec![LOS::default(); grid[0].len()]; grid.len()];

    // left
    for r in 0..grid.len() {
        let mut count = 0;
        for c in 0..grid[r].len() {
            if grid[r][c] {
                count += 1;
            } else {
                count = 0;
            }
            data[r][c].left = count;
        }
    }

    // right
    for r in 0..grid.len() {
        let mut count = 0;
        for c in (0..grid[r].len()).rev() {
            if grid[r][c] {
                count += 1;
            } else {
                count = 0;
            }
            data[r][c].right = count;
        }
    }

    // up
    for c in 0..grid[0].len() {
        let mut count = 0;
        for r in 0..grid.len() {
            if grid[r][c] {
                count += 1;
            } else {
                count = 0;
            }
            data[r][c].up = count;
        }
    }

    // down
    for c in 0..grid[0].len() {
        let mut count = 0;
        for r in (0..grid.len()).rev() {
            if grid[r][c] {
                count += 1;
            } else {
                count = 0;
            }
            data[r][c].down = count;
        }
    }

    data
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
        let grid = read_test_input(&mut lines)?;
        let ans = total_Ls(&grid);
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
        let row: Vec<_> = line.split_whitespace().map(|s| match s {
            "0" => false,
            "1" => true,
            _ => panic!(),
        }).collect();
        assert_eq!(row.len(), c);

        grid.push(row);
    }

    Ok(grid)
}
