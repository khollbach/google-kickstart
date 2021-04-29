use std::io::{self, prelude::*};
use std::error::Error;
use QueryResult::{Left, Middle, Right};

/*
idea: insertion sort using 3-way "binary" search

- first query sets `sorted` to some: (we get to choose how to break the tie)
    [i, j, k]

- for i in 3..n {
    - call insert(sorted, i)  --- returns an index to insert *before*
    insert into that position    -- in [0..=currlen]
}

insert:
- n = len(sorted)
- bc: n == 0  -> return
- ec: n == 1  -> panic; unreachable
- divide up n into 3 equal parts;
    - perform the query; match on three cases:
    - in each case, handle edge-case of length 1 before recursive call
        - that way, we never hit the ec
 */

fn median_sort(n: u32) -> Vec<u32> {
    if n < 3 {
        return (1..=n).collect();
    }

    let initial = match median_query(1, 2, 3) {
        Left => [2, 1, 3],
        Middle => [1, 2, 3],
        Right => [1, 3, 2],
    };

    let mut sorted = Vec::with_capacity(n as usize);
    sorted.extend_from_slice(&initial);

    for val in 4..=n {
        let idx = insertion_index(&sorted, val);
        sorted.insert(idx, val);
    }

    sorted
}

/// Panics if sorted has length 1.
fn insertion_index(sorted: &[u32], val: u32) -> usize {
    match sorted.len() {
        0 => 0,
        1 => panic!("Should never hit this case; there's a bug in our code."),
        n => {
            let mid1 = sorted[n / 3];
            let mid2 = sorted[n * 2 / 3];

            // Each of the following cases has a built-in edge-case
            // to avoid slices of length exactly 1.
            match median_query(mid1, val, mid2) {
                Left => {
                    let mut upper = n / 3;
                    if upper == 1 {
                        upper += 1;
                    }

                    insertion_index(&sorted[..upper], val)
                }
                Middle => {
                    let mut lower = (n / 3) + 1;
                    let upper = n * 2 / 3;
                    if upper - lower == 1 {
                        lower -= 1;
                    }

                    lower + insertion_index(&sorted[lower..upper], val)
                }
                Right => {
                    let mut lower = (n * 2 / 3) + 1;
                    if n - lower == 1 {
                        lower -= 1;
                    }

                    lower + insertion_index(&sorted[lower..], val)
                }
            }
        }
    }
}

enum QueryResult {
    Left,
    Middle,
    Right,
}

/// Accepts values in `1..=n`.
///
/// Performs the query using stdio.
///
/// Panics if the query fails.
fn median_query(i: u32, j: u32, k: u32) -> QueryResult {
    println!("{} {} {}", i, j, k);

    let line = io::stdin().lock().lines().next().unwrap().unwrap();
    let med: i32 = line.parse().unwrap();
    if med < 0 {
        panic!("Negative query response to {} {} {}: {}", i, j, k, med);
    }
    let med = med as u32;

    match med {
        _ if med == i => Left,
        _ if med == j => Middle,
        _ if med == k => Right,
        _ => panic!("Query response not one of the three inputs {} {} {}: {}", i, j, k, med),
    }
}

type Res<T> = Result<T, Box<dyn Error>>;

fn main() -> Res<()> {
    run_tests()
}

/// Panics on malformed input.
fn run_tests() -> Res<()> {
    let line = io::stdin().lock().lines().next().unwrap()?;
    let mut words = line.split_whitespace();
    let t: usize = words.next().unwrap().parse()?;
    let n: u32 = words.next().unwrap().parse()?;
    let _q: u32 = words.next().unwrap().parse()?;
    assert!(words.next().is_none());

    for _test_no in 1..=t {
        let sorted = median_sort(n);
        let ans = nums_to_string(&sorted);
        println!("{}", ans);

        let line = io::stdin().lock().lines().next().unwrap()?;
        assert_eq!(line, "1");
    }

    Ok(())
}

/// Space-separated.
#[must_use]
fn nums_to_string(nums: &[u32]) -> String {
    let mut result = String::new();
    for (i, n) in nums.iter().enumerate() {
        if i != 0 {
            result.push(' ');
        }
        result.push_str(&n.to_string());
    }
    result
}
