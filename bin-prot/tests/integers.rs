///
/// These tests aim to follow the Jane Street OCaml tests as closely as possible
/// The tests contain a lookup table for auto-generated integer encoding test cases
///
/// Note the bytes are little-endian encoded.
///
/// These tests can be parsed and executed directly from their source
use std::collections::HashSet;
mod common;

type Points = HashSet<i64>;

const TEST_WINDOW_LEN: i64 = 16;

const INT_MIN: i64 = -2_i64.pow(62);
const INT_MAX: i64 = 2_i64.pow(62) - 1;

struct TestCase {
    min: i64,
    max: i64,
}

fn find_interesting_points(case: TestCase) -> Points {
    let mut points = HashSet::new();
    points.insert(0);
    points.insert(case.min);
    points.insert(case.max);
    points = points.union(&valid_powers_of_two(&case)).cloned().collect();
    add_windows_around_points(case, points)
}

// { 2 ^ n | 0 <= n <= 63 } \/ { -(2 ^ n) | 0 <= n <= 63 }
fn powers_of_two() -> Points {
    let mut acc = Points::new();
    for i in 0..62 {
        acc.insert(2_i64.pow(i));
        acc.insert(-2_i64.pow(i));
    }
    acc
}

fn valid_powers_of_two(case: &TestCase) -> Points {
    powers_of_two()
        .into_iter()
        .filter(|i| i >= &case.min && i <= &case.max)
        .collect()
}

fn add_windows_around_points(TestCase { min, max }: TestCase, points: Points) -> Points {
    // add all point between a and b inclusive into an accumulator
    fn add_between(a: i64, b: i64, mut acc: Points) -> Points {
        println!("adding between {} and {}", a, b);
        for i in a..b {
            acc.insert(i.clone());
        }
        acc
    }

    points.into_iter().fold(Points::new(), |acc, i| {
        let d = TEST_WINDOW_LEN / 2;
        let a = if i <= (min + d) { min } else { i - d };
        let b = if i >= (max - d) { max } else { i + d };
        add_between(a, b, acc)
    })
}

/// Test the variable size integer encoding
#[test]
fn test_roundtrip_integers() {
    let int_test = TestCase {
        min: INT_MIN,
        max: INT_MAX,
    };

    for val in find_interesting_points(int_test) {
        common::roundtrip_test(val);
    }
}
