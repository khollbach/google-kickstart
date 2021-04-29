use std::io::{self, prelude::*};
use std::error::Error;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
struct Edge {
    node: usize,
    other: usize,
    limit: u32,
    amount: u64,
}

#[derive(Debug, Copy, Clone, Default)]
struct TreeEdge {
    parent: usize,
    limit: u32,
    amount: u64,
}

#[derive(Debug, Clone, Default)]
struct GraphNode {
    nbrs: Vec<TreeEdge>,
}

type Graph = Vec<GraphNode>;
type Tree = Vec<TreeEdge>;

fn build_graph(n: usize, edges: impl Iterator<Item = Edge>) -> Graph {
    // 1-indexed.
    let mut graph = vec![GraphNode::default(); n + 1];

    for Edge { node, other, limit, amount } in edges {
        graph[node].nbrs.push(TreeEdge { parent: other, limit, amount });
        graph[other].nbrs.push(TreeEdge { parent: node, limit, amount });
    }

    graph
}

fn build_tree(graph: &Graph) -> Tree {
    let n = graph.len() - 1;

    // 1-indexed.
    let mut tree = vec![TreeEdge::default(); n + 1];

    // DFS from 1 (root).
    // We put a self-loop parent pointer at the root.
    let mut stack = vec![(1, TreeEdge { parent: 1, ..TreeEdge::default() })];
    let mut seen = HashSet::new();
    seen.insert(1);
    while let Some((parent, edge)) = stack.pop() {
        let node = edge.parent;
        tree[node] = TreeEdge { parent, ..edge };

        // Explore neighbours (i.e. children).
        for &e in &graph[node].nbrs {
            let child = e.parent;
            if !seen.contains(&child) {
                seen.insert(child);
                stack.push((node, e));
            }
        }
    }

    tree
}

#[derive(Debug, Copy, Clone)]
struct Query {
    node: usize,
    weight: u32,
}

/*
- query:
- walk path to root, collecting edges according to thresh
- take gcd of edges collected
*/
fn answer_query(tree: &Tree, q: Query) -> u64 {
    let mut node = q.node;
    let mut payments = Vec::with_capacity(tree.len() - 2);
    while node != 1 {
        let edge = tree[node];
        if q.weight >= edge.limit {
            payments.push(edge.amount);
        }
        node = edge.parent;
    }
    gcd(&payments)
}

fn gcd(nums: &[u64]) -> u64 {
    if nums.is_empty() {
        return 0;
    }

    nums.iter().copied().fold(nums[0], gcd_2)
}

fn gcd_2(mut a: u64, mut b: u64) -> u64 {
    debug_assert_ne!(a, 0);
    debug_assert_ne!(b, 0);

    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }

    a
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
        let (edges, queries) = read_test_input(&mut lines)?;
        //dbg!(&edges, &queries); // looks good!

        let graph = build_graph(edges.len() + 1, edges.into_iter());
        //dbg!(&graph); panic!(); // looks good too!

        let tree = build_tree(&graph);
        let ans = queries.into_iter().map(|q| answer_query(&tree, q));
        println!("Case #{}: {}", test_no, nums_to_string(ans));
    }
    assert!(lines.next().is_none());
    Ok(())
}

/// Panics on malformed input.
fn read_test_input(lines: &mut impl Iterator<Item = io::Result<String>>) -> Res<(Vec<Edge>, Vec<Query>)> {
    let line = lines.next().unwrap()?;
    let nums: Vec<usize> = line.split_whitespace().map(|word| word.parse().unwrap()).collect();
    assert_eq!(nums.len(), 2);
    let n = nums[0];
    let q = nums[1];

    let mut edges = Vec::with_capacity(n - 1);
    for _ in 0..n - 1 {
        let line = lines.next().unwrap()?;
        let nums: Vec<u64> = line.split_whitespace().map(|word| word.parse().unwrap()).collect();
        assert_eq!(nums.len(), 4, "{:?}", nums);
        let node = nums[0] as usize;
        let other = nums[1] as usize;
        let limit = nums[2] as u32;
        let amount = nums[3];

        edges.push(Edge { node, other, limit, amount });
    }

    let mut queries = Vec::with_capacity(q);
    for _ in 0..q {
        let line = lines.next().unwrap()?;
        let nums: Vec<usize> = line.split_whitespace().map(|word| word.parse().unwrap()).collect();
        assert_eq!(nums.len(), 2);
        let node = nums[0];
        let weight = nums[1] as u32;

        queries.push(Query { node, weight });
    }

    Ok((edges, queries))
}

fn nums_to_string(nums: impl Iterator<Item = u64>) -> String {
    let mut res = String::new();
    for n in nums {
        if !res.is_empty() {
            res.push(' ');
        }
        res.push_str(&n.to_string());
    }
    res
}