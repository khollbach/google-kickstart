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
        let program = read_line();
        let (x, y) = evaluate(program.as_bytes());
        println!("Case #{}: {} {}", t, x + 1, y + 1);
    }
}

/// Returns values in the range [0, 10^9).
fn evaluate(program: &[u8]) -> (u64, u64) {
    match program.split_first() {
        None => (0, 0),
        Some((&head, rest)) => {
            if b"NESW".contains(&head) {
                let (dx, dy) = cardinal(head);
                let (x, y) = evaluate(rest);
                modulus::wrap(dx + x as i64, dy + y as i64)
            } else {
                let head = (head as char).to_digit(10).unwrap() as u64;
                let (inner, rest) = get_subprogram(rest);
                let (x1, y1) = evaluate(inner);
                let (x2, y2) = evaluate(rest);
                modulus::wrap((head * x1 + x2) as i64, (head * y1 + y2) as i64)
            }
        }
    }
}

fn get_subprogram(rest: &[u8]) -> (&[u8], &[u8]) {
    assert!(!rest.is_empty() && rest[0] == b'(');
    let mut depth = 0;
    for (i, &b) in rest.iter().enumerate() {
        if b == b'(' {
            depth += 1;
        } else if b == b')' {
            depth -= 1;
        }

        if depth == 0 {
            return (&rest[1..i], &rest[i + 1..]);
        }
    }
    unreachable!();
}

fn cardinal(d: u8) -> (i64, i64) {
    match d {
        b'N' => (0, -1),
        b'E' => (1, 0),
        b'S' => (0, 1),
        b'W' => (-1, 0),
        _ => panic!("Invalid cardinal direction: {}", d),
    }
}

mod modulus {
    const BILLION: u64 = 1_000_000_000;

    /// Take an integer `a`, and wrap it according to a modulus `b`.
    fn modulo(a: i64, b: u64) -> u64 {
        let b = b as i64;

        let mut result = a % b;
        if result < 0 {
            result += b;
        }
        result as u64
    }

    /// Wrap two integers to be in the range [0, 10^9).
    pub fn wrap(x: i64, y: i64) -> (u64, u64) {
        (modulo(x, BILLION), modulo(y, BILLION))
    }
}
