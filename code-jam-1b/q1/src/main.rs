use std::io::{self, prelude::*};
use std::error::Error;
use std::cmp::Ordering;

fn find_time_unknown_hands(a: Ticks, b: Ticks, c: Ticks) -> Option<Time> {
    // Pick hour and minute hands.
    let positions = vec![a, b, c];
    for (h, m, s) in permutations_3() {
        if let Some(t) = find_time(positions[h], positions[m], positions[s]) {
            return Some(t);
        }
    }
    None
}

fn permutations_3() -> Vec<(usize, usize, usize)> {
    let mut res = vec![];
    for h in 0..3 {
        for m in 0..3 {
            if m != h {
                for s in 0..3 {
                    if s != h && s != m {
                        res.push((h, m, s));
                    }
                }
            }
        }
    }
    res
}

// A measure of rotation.
type Ticks = u64;

// Number of ticks in 360 degrees.
const BASE: u64 = 360 * 12 * 10_000_000_000;

struct Time {
    hours: u64,
    minutes: u64,
    seconds: u64,
    nanoseconds: u64,
}

fn find_time(h: Ticks, m: Ticks, _s: Ticks) -> Option<Time> {
    let target_angle = wrap_minus(m, h) as i64;

    for hour in 0..6 {
        // bin search to find closest hour-hand tick in that hour
        // to try to make an angle equal to the target.

        let lo = 0 / 12;
        let hi = BASE / 12;

        let offset = hour * BASE / 12;

        let lo_range = min_hand(offset) as i64 - offset as i64;
        let hi_range = lo_range + (BASE * 11 / 12) as i64;

        let adjusted_target = if (lo_range..=hi_range).contains(&target_angle) {
            target_angle
        } else {
            target_angle - BASE as i64
        };

        dbg!(hour, lo, hi, offset, lo_range, hi_range, target_angle, adjusted_target);

        if let Ok(mut hour_hand) = dbg!(bin_search(lo, hi, adjusted_target)) {
            hour_hand += offset;

            // Compute time and return it.
            return Some(nanosecs_to_time(hour_hand));
        }
    }

    None
}

fn nanosecs_to_time(mut nanoseconds: u64) -> Time {
    let mut seconds = nanoseconds / 1000;
    nanoseconds %= 1000;

    let mut minutes = seconds / 60;
    seconds %= 60;

    let hours = minutes / 60;
    minutes %= 60;

    Time { hours, minutes, seconds, nanoseconds }
}

// hi is exclusive.
fn bin_search(lo: Ticks, hi: Ticks, target_angle: i64) -> Result<Ticks, Ticks> {
    if lo >= hi {
        return Err(lo);
    }

    // Hour hand position.
    let mid = (lo + hi) / 2;

    let angle = min_hand(mid) as i64 - mid as i64;

    use Ordering::*;
    match angle.cmp(&target_angle) {
        Less => bin_search(lo, mid, target_angle),
        Greater => match bin_search(mid + 1, hi, target_angle) {
            Ok(t) => Ok(mid + 1 + t),
            Err(t) => Err(mid + 1 + t),
        }
        Equal => Ok(mid),
    }
}

fn wrap_minus(a: Ticks, b: Ticks) -> Ticks {
    debug_assert!(a < BASE && b < BASE);
    (a + BASE - b) % BASE
}

// What would the min hand be for this hour hand?
// Should probably unit test this for sanity, but we don't have time...
fn min_hand(hour_hand: Ticks) -> Ticks {
    (hour_hand * 12) % BASE
}

#[test]
fn test() {
    for (hour_hand, expected) in vec![
        (0, 0),
        (BASE / 12, 0),
        (BASE / 24, BASE / 2),
    ] {
        let actual = min_hand(hour_hand);
        assert_eq!(expected, actual);
    }
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
        let (a, b, c) = read_test_input(&mut lines)?;
        let ans = find_time_unknown_hands(a, b, c).unwrap();
        println!("Case #{}: {} {} {} {}", test_no, ans.hours, ans.minutes, ans.seconds, ans.nanoseconds);
    }
    assert!(lines.next().is_none());
    Ok(())
}

/// Panics on malformed input.
fn read_test_input(lines: &mut impl Iterator<Item = io::Result<String>>) -> Res<(Ticks, Ticks, Ticks)> {
    let line = lines.next().unwrap()?;
    let words: Vec<_> = line.split_whitespace().map(|w| w.parse().unwrap()).collect();
    assert_eq!(words.len(), 3);

    Ok((words[0], words[1], words[2]))
}
