use std::io::{self, prelude::*};
use std::collections::BinaryHeap;
use std::error::Error;
use std::collections::BinaryHeap;

/*
- read grid
- fill max-heap "peaks" with:
    [val, i, j] for val > 1

- while heap, pop "me":
    if my value in my cell is larger than me,
        ignore; since Ive already been dequeued
    for each of my 4 nbrs,
        if theyre less than (me - 1):
            add the delta to total
            re-add them to the heap (if theyre > 1)
*/

type Grid = Vec<Vec<u32>>;

fn boxes_to_add(grid: &mut Grid) -> u64 {
    let mut heap = BinaryHeap::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let val = grid[i][j];
            if val >= 2 {
                heap.push((val, i, j));
            }
        }
    }

    let mut total_delta = 0u64;
    while let Some((val, i, j)) = heap.pop() {
        if grid[i][j] > val { continue; } // Already been here.

        for (x, y) in nbrs(grid, i as isize, j as isize) {
            let nbr = &mut grid[x][y];
            if *nbr < val - 1 {
                let delta = val - 1 - *nbr;
                total_delta += delta as u64;

                *nbr = val - 1;
                if *nbr >= 2 {
                    heap.push((*nbr, x, y));
                }
            }
        }
    }
    total_delta
}

fn nbrs(grid: &Grid, i: isize, j: isize) -> impl Iterator<Item = (usize, usize)> {
    let n = grid.len() as isize;
    let m = grid[0].len() as isize;

    [(0, -1), (0, 1), (-1, 0), (1, 0)].iter().filter_map(move |&(di, dj)| {
        let x = i + di;
        let y = j + dj;
        if 0 <= x && x < n && 0 <= y && y < m {
            Some((x as usize, y as usize))
        } else {
            None
        }
    })
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
