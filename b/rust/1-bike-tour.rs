#![allow(non_snake_case)]

/// Read a line from stdin.
/// Strip the trailing newline if any.
fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    #[allow(deprecated)]
    let new_len = buf.trim_right_matches('\n').len();
    buf.truncate(new_len);
    buf
}

fn main() {
    let T: u32 = input().parse().unwrap();
    for t in 1..T + 1 {
        let N: usize = input().parse().unwrap();
        let H: Vec<u32> = input()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        assert_eq!(N, H.len());
        let result = num_peaks(&H);
        println!("Case #{}: {}", t, result);
    }
}

use std::cmp::max;

fn num_peaks(heights: &[u32]) -> u32 {
    let n = heights.len();
    let mut count = 0;
    for i in 1..n - 1 {
        if heights[i] > max(heights[i - 1], heights[i + 1]) {
            count += 1;
        }
    }
    count
}
