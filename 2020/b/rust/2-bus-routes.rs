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
fn input<T>() -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    read_line().parse().unwrap()
}

/// Read a line and parse it as whitespace-separated values.
fn inputs<T>() -> Vec<T>
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
        let (N, D) = (vals[0], vals[1]);
        let X: Vec<u64> = inputs();
        assert_eq!(N as usize, X.len());
        assert!(X.iter().all(|&x| x >= 1));
        assert!(D >= 1);
        let result = latest_start(&X, D);
        println!("Case #{}: {}", t, result);
    }
}

/// Days are numbered from 1 to end_day inclusive.
/// If bus_freqs[-1] is 4, then the last bus only runs on days
/// that are multiples of 4: 4, 8, 12, etc...
///
/// Find the latest day to take bus[0] s.t. you can schedule all
/// busses one after the other (two on the same day is allowed) and
/// be done on or before end_day.
///
/// - take last bus; find largest # divisible by it and <= end_day
///   - if that # is 0, throw (promise violated)
/// - recurse on list[:-1] and that #  (return the result)
/// - base case: empty list; return end_day
fn latest_start(bus_freqs: &[u64], end_day: u64) -> u64 {
    assert!(end_day != 0);
    match bus_freqs.split_last() {
        None => end_day,
        Some((&last, rest)) => {
            // Schedule the last bus as late as possible.
            let d = largest_multiple_le(last, end_day);

            // Consider the subproblem of bus_freqs[..-1].
            latest_start(rest, d)
        }
    }
}

/// Return the largest multiple of x that is <= t.
fn largest_multiple_le(x: u64, t: u64) -> u64 {
    match x {
        0 => 0,
        _ => x * (t / x),
    }
}
