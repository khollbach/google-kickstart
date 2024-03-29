#![allow(non_snake_case)]

/// Read a line from stdin.
/// Strip the trailing newline if any.
fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    #[allow(deprecated)]
    let new_len = buf.trim_right_matches('\n').len();
    buf.truncate(new_len);
    buf
}

/// Read a line and parse it as a single value.
pub fn input<T>() -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    read_line().parse().unwrap()
}

/// Read a line and parse it as whitespace-separated values.
pub fn inputs<T>() -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    read_line()
        .split_whitespace()
        .map(|word| word.parse().unwrap())
        .collect()
}

fn main() {
    let T: u32 = input();
    for t in 1..T + 1 {
        let vals = inputs();
        let W = vals[0];
        let H = vals[1];
        let L = vals[2];
        let U = vals[3];
        let R = vals[4];
        let D = vals[5];
        let result = success_chance(W, H, L, U, R, D);
        println!("Case #{}: {}", t, result);
    }
}

fn success_chance(W: usize, H: usize, L: usize, U: usize, R: usize, D: usize) -> f64 {
    // Zero-indexed, please! (But still left/right inclusive.)
    let (L, U, R, D) = (L - 1, U - 1, R - 1, D - 1);

    // todo: instead, compute using ints and then divide out only at the end!
    // This should be possible, since I'm pretty sure the final probability is
    // just a function of the number of distinct routes to the destination,
    // and the dimensions of the grid.
    //
    // Problem: the numbers would be exponentially large. We're computing
    // Pascal's triangle here, and (n choose n/2) is supposedly
    // Theta(2^n / sqrt(n)), so that's not gonna work. (Where n = W + H.)
    //let mut grid = vec![0u64; W * 2];

    // H by W grid.
    // Row indices wrap mod 2!
    // Laid out in memory *transposed*, for cache locality:
    // - The 2 columns of memory are rows of the grid.
    // - The W rows of memory are columns of the grid.
    // Access grid[h][w] as grid[w*2 + h%2].
    let mut grid = vec![0.0; W * 2];

    for h in 0..H {
        for w in 0..W {
            grid[w * 2 + h % 2] = if h == 0 && w == 0 {
                1.0
            } else if U <= h && h <= D && L <= w && w <= R {
                0.0
            } else {
                let mut above = if h != 0 {
                    grid[w * 2 + (h - 1) % 2]
                } else {
                    0.0
                };
                above *= if w == W - 1 { 2.0 } else { 1.0 };

                let mut left = if w != 0 {
                    grid[(w - 1) * 2 + h % 2]
                } else {
                    0.0
                };
                left *= if h == H - 1 { 2.0 } else { 1.0 };

                (above + left) / 2.0
            }
        }
    }

    grid[(W - 1) * 2 + (H - 1) % 2]
}
