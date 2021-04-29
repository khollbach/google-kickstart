use std::io::{self, prelude::*};
use std::error::Error;

const NUM_STUDENTS: usize = 100;
const NUM_QUESTIONS: usize = 10_000;

const HARD_Q_PCT_THRESH: f64 = 0.20; // Anything at or below this is "hard".

fn find_cheater(answers: Vec<Vec<bool>>) -> usize {
    let mut questions = Vec::with_capacity(NUM_QUESTIONS);
    for q in 0..NUM_QUESTIONS {
        let avg = question_avg((0..NUM_STUDENTS).map(|s| answers[s][q]), NUM_STUDENTS);
        questions.push(avg);
    }
    let hard_questions: Vec<usize> = (0..NUM_QUESTIONS).filter(|&i| questions[i] <= HARD_Q_PCT_THRESH).collect();

    let hard_qs_correct = |s: usize| {
        hard_questions.iter().filter(|&&q| answers[s][q]).count()
    };


    let mut students = Vec::with_capacity(NUM_STUDENTS);
    for i in 0..NUM_STUDENTS {
        let avg = student_avg(&answers[i]);
        students.push(avg);
    }
    let expected = |s: usize| {
        //500 + (1000 as f64 * students[s]) as usize
        //(600 as f64 * students[s]) as usize
        0
    };

    let mut num_hard: Vec<_> = (0..NUM_STUDENTS).map(|s| (hard_qs_correct(s) - expected(s), s)).collect();
    num_hard.sort();

    // dbg!(&num_hard[num_hard.len() - 5..]);
    // dbg!(&num_hard[..5]);

    let (_, s) = num_hard.into_iter().max().unwrap();
    s
}

fn student_avg(row: &[bool]) -> f64 {
    let num_correct = row.iter().filter(|&&b| b).count();
    num_correct as f64 / row.len() as f64
}

fn question_avg(col: impl Iterator<Item = bool>, col_len: usize) -> f64 {
    let num_correct = col.filter(|&b| b).count();
    num_correct as f64 / col_len as f64
}

type Res<T> = Result<T, Box<dyn Error>>;

fn main() -> Res<()> {
    run_tests(io::stdin().lock().lines())
}


/// Panics on malformed input.
fn run_tests(mut lines: impl Iterator<Item = io::Result<String>>) -> Res<()> {
    let line = lines.next().unwrap()?;
    let t: u32 = line.parse()?;
    let line = lines.next().unwrap()?;
    let _p: u32 = line.parse()?;
    for test_no in 1..=t {
        let answers = read_test_input(&mut lines)?;
        let ans = find_cheater(answers) + 1; // 1-indexed answer.
        println!("Case #{}: {}", test_no, ans);
    }
    assert!(lines.next().is_none());
    Ok(())
}

/// Panics on malformed input.
fn read_test_input(lines: &mut impl Iterator<Item = io::Result<String>>) -> Res<Vec<Vec<bool>>> {
    let mut answers = Vec::with_capacity(NUM_STUDENTS);

    for _ in 0..NUM_STUDENTS {
        let line = lines.next().unwrap()?;
        assert_eq!(line.len(), NUM_QUESTIONS);

        let row: Vec<_> = line.chars().map(|c| match c {
            '0' => false,
            '1' => true,
            _ => panic!("Unexpected student answer (should be 0 or 1): {}", c),
        }).collect();
        assert_eq!(row.len(), NUM_QUESTIONS);

        answers.push(row);
    }
    assert_eq!(answers.len(), NUM_STUDENTS);

    Ok(answers)
}

/* An old  version follows. there's a 10KiB (?) code size limit :(((

use std::io::{self, prelude::*};
use std::error::Error;

const NUM_STUDENTS: usize = 100;
const NUM_QUESTIONS: usize = 10_000;

const HARD_Q_PCT_THRESH: f64 = 0.20; // Anything at or below this is "hard".

fn find_cheater(answers: Vec<Vec<bool>>) -> usize {
    let mut questions = Vec::with_capacity(NUM_QUESTIONS);
    for q in 0..NUM_QUESTIONS {
        let avg = question_avg((0..NUM_STUDENTS).map(|s| answers[s][q]), NUM_STUDENTS);
        questions.push(avg);
    }
    let hard_questions: Vec<usize> = (0..NUM_QUESTIONS).filter(|&i| questions[i] <= HARD_Q_PCT_THRESH).collect();

    let hard_qs_correct = |s: usize| {
        hard_questions.iter().filter(|&&q| answers[s][q]).count()
    };

    let (_, s) = (0..NUM_STUDENTS).map(|s| (hard_qs_correct(s), s)).max().unwrap();
    s
}

fn question_avg(col: impl Iterator<Item = bool>, col_len: usize) -> f64 {
    let num_correct = col.filter(|&b| b).count();
    num_correct as f64 / col_len as f64
}

type Res<T> = Result<T, Box<dyn Error>>;

fn main() -> Res<()> {
    run_tests(io::stdin().lock().lines())?
}


/// Panics on malformed input.
fn run_tests(mut lines: impl Iterator<Item = io::Result<String>>) -> Res<()> {
    let line = lines.next().unwrap()?;
    let t: u32 = line.parse()?;
    let line = lines.next().unwrap()?;
    let _p: u32 = line.parse()?;
    for test_no in 1..=t {
        let answers = read_test_input(&mut lines)?;
        let ans = find_cheater(answers) + 1; // 1-indexed answer.
        println!("Case #{}: {}", test_no, ans);
    }
    assert!(lines.next().is_none()); // For now, while I generate my own tests.
    Ok(())
}

/// Panics on malformed input.
fn read_test_input(lines: &mut impl Iterator<Item = io::Result<String>>) -> Res<Vec<Vec<bool>>> {
    let mut answers = Vec::with_capacity(NUM_STUDENTS);

    for _ in 0..NUM_STUDENTS {
        let line = lines.next().unwrap()?;
        assert_eq!(line.len(), NUM_QUESTIONS);

        let row: Vec<_> = line.chars().map(|c| match c {
            '0' => false,
            '1' => true,
            _ => panic!("Unexpected student answer (should be 0 or 1): {}", c),
        }).collect();
        assert_eq!(row.len(), NUM_QUESTIONS);

        answers.push(row);
    }
    assert_eq!(answers.len(), NUM_STUDENTS);

    Ok(answers)
}

fn random_test_input(fudge_s_scores: bool, all_cheat: bool) -> (Vec<Vec<bool>>, Vec<f64>, Vec<f64>) {
    let mut rng = rand::thread_rng();
    let students: Vec<_> = if fudge_s_scores { // terrible hack, for now
        let mut students = vec![];
        let mut score = -3.;
        while score <= 3. {
            students.push(score);
            score += 0.1;
        }
        students
    } else {
        (0..NUM_STUDENTS).map(|_| random_range(&mut rng, -3., 3.)).collect()
    };
    let questions: Vec<_> = (0..NUM_QUESTIONS).map(|_| random_range(&mut rng, -3., 3.)).collect();

    let mut answers = Vec::with_capacity(NUM_STUDENTS);
    for &stud_score in &students {
        let mut row = Vec::with_capacity(NUM_QUESTIONS);
        for &q_score in &questions {
            if all_cheat {
                row.push(cheater_gets_correct(&mut rng, stud_score, q_score));
            } else {
                row.push(gets_correct(&mut rng, stud_score, q_score));
            }
        }
        answers.push(row)
    }

    (answers, students, questions)
}

fn random_range(rng: &mut impl Rng, low: f64, high: f64) -> f64 {
    debug_assert!(low <= high);
    let zero_to_one: f64 = rng.gen();
    low + zero_to_one * (high - low)
}

fn gets_correct(rng: &mut impl Rng, student_score: f64, question_score: f64) -> bool {
    debug_assert!(-3. <= student_score && student_score <= 3.);
    debug_assert!(-3. <= question_score && question_score <= 3.);

    let x = student_score - question_score;
    let fx = sigmoid(x);
    rng.gen_bool(fx)
}

fn sigmoid(x: f64) -> f64 {
    1. / (1. + E.powf(-x))
}

fn cheater_gets_correct(rng: &mut impl Rng, student_score: f64, question_score: f64) -> bool {
    if rng.gen_bool(0.5) {
        true
    } else {
        gets_correct(rng, student_score, question_score)
    }
}

 */