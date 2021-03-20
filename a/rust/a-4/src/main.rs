use std::error::Error;
use std::io::{self, prelude::*};
use std::ops::AddAssign;
use trie::{TrieNode, build_trie};

mod trie;

/// Return the total score of the best partition into groups of size k.
pub fn best_partition(words: &[String], k: u32) -> u32 {
    let n = words.len() as u32;
    assert_eq!(n % k, 0);
    let num_groups = n / k;

    if n == 0 { return 0; }
    let trie = build_trie(words.iter().map(|s| s.as_str())).unwrap();

    // We adjust for the fact that all words share the empty prefix "", which causes
    // our recursive code to over-count the score of each group by 1.
    let mut ret = best_score(&trie, k);
    ret.total_score -= ret.num_groups;

    // Sanity check; should always pass.
    assert_eq!(ret.num_groups, num_groups);
    assert_eq!(ret.num_leftovers, 0);

    ret.total_score
}

fn best_score(trie: &TrieNode, k: u32) -> BestScoreRet {
    let mut total = BestScoreRet::default();
    for child in &trie.children {
        if let Some(node) = child {
            total += best_score(node, k);
        }
    }
    total.num_leftovers += trie.num_leaves;

    // Combine leftovers into new groups if possible.
    total.num_groups += total.num_leftovers / k;
    total.num_leftovers %= k;

    // Add 1 score to each group, since they share the char at this level.
    total.total_score += total.num_groups;

    total
}

#[derive(Debug, Clone, Copy, Default)]
struct BestScoreRet {
    /// How many groups of size `k`.
    num_groups: u32,
    /// Sum of the scores of the above groups.
    total_score: u32,
    /// Any extra words that didn't make up a full group.
    num_leftovers: u32,
}

impl AddAssign<Self> for BestScoreRet {
    fn add_assign(&mut self, other: Self) {
        self.num_groups += other.num_groups;
        self.total_score += other.total_score;
        self.num_leftovers += other.num_leftovers;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    run_tests(io::stdin().lock())
}

/// Panics if the input isn't correctly formatted.
#[allow(non_snake_case)]
fn run_tests(input: impl BufRead) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines();

    let T: u32 = lines.next().unwrap()?.parse()?;
    for t in 1..=T {
        let line = lines.next().unwrap()?;
        let mut words = line.split_whitespace();
        let N: usize = words.next().unwrap().parse()?;
        let K: u32 = words.next().unwrap().parse()?;
        assert!(words.next().is_none());

        let mut words = Vec::with_capacity(N);
        for _ in 0..N {
            words.push(lines.next().unwrap()?);
        }
        assert_eq!(words.len(), N);

        let ans = best_partition(&words, K);
        println!("Case #{}: {}", t, ans);
    }
    assert!(lines.next().is_none());

    Ok(())
}
