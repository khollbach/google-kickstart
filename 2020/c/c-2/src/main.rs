use std::io::{self, prelude::*};
use std::error::Error;
use std::collections::{HashMap, HashSet};

/*
- read input into a rectangular matrix of chars (A-Z) -- or nums < 26
- building a graph; deps map[char -> char] (can be arr. w len 26)
- for each cell "me" in the grid (except the bottom row),
    - the cell below it (if different than me):
        add a dep: cell -> me
    - I repeat; DO NOT add self-loops
 - make sure all nodes appear as keys

- WTF a total order (if exists)
- compute indegree of each node -- map[char -> u32]
- keep a queue of nodes w indegree 0
- while q:
    - pop "me"; decrement indegree's of nbrs (i.e. delete me)
        - (push to an accum. list to remember the order)
    - for each nbr with indeg 0,
        - push it
- if acc has len = n, we win; else ret -1
*/

type Grid = Vec<Vec<char>>;

fn find_insertion_order(grid: &Grid) -> Option<Vec<char>> {
    let (graph, mut indegrees) = build_graph(grid);
    let mut order = Vec::with_capacity(graph.len());

    // Keep track of nodes with indegree zero.
    let mut exposed: Vec<_> = indegrees.iter().filter_map(|(&k, &deg)| {
        if deg == 0 { Some(k) } else { None }
    }).collect();

    while let Some(k) = exposed.pop() {
        order.push(k);

        for nbr in &graph[&k] {
            *indegrees.get_mut(nbr).unwrap() -= 1;
            if indegrees[nbr] == 0 {
                exposed.push(*nbr);
            }
        }
    }

    if order.len() == graph.len() {
        Some(order)
    } else {
        None
    }
}

/// Build the graph of the `happens-before` relation.
/// Also compute the in-degrees of each node.
fn build_graph(grid: &Grid) -> (HashMap<char, HashSet<char>>, HashMap<char, u32>) {
    let mut graph = HashMap::<char, HashSet<char>>::new();
    let mut indegrees = HashMap::new();
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            let ch = grid[r][c];

            // Make sure every node appears as a key in both maps,
            // no matter how many in/out edges it has.
            graph.entry(ch).or_default();
            indegrees.entry(ch).or_default();

            if r == grid.len() - 1 { continue; } // Skip the last row.
            let below = grid[r + 1][c];
            if ch != below {
                // `below` happens before `ch`.
                // Add the edge: `below -> ch`.
                if graph.entry(below).or_default().insert(ch) {
                    // Only update once per edge; this isn't a multi-graph.
                    *indegrees.entry(ch).or_default() += 1;
                }
            }
        }
    }
    (graph, indegrees)
}

/// Panics on malformed input.
fn read_grid(lines: &mut impl Iterator<Item = io::Result<String>>, r: usize, c: usize) -> Res<Grid> {
    let mut grid: Grid = Vec::with_capacity(r);
    for i in 0..r {
        let line = lines.next().unwrap()?;
        grid.push(line.chars().collect());
        assert_eq!(grid[i].len(), c);
    }
    assert_eq!(grid.len(), r);
    Ok(grid)
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
        let ans: String = match find_insertion_order(&grid) {
            Some(order) => order.into_iter().collect(),
            None => "-1".into(),
        };
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

    read_grid(lines, r, c)
}
