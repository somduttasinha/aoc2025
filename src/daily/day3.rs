use std::cmp::Ordering;

use crate::common::{self, Part};

pub fn day3(part: Part) {
    let path = "src/resources/day3.txt";
    let raw = common::read(path);
    let res = run(raw, part);
    println!("res: {}", res);
}

fn run(raw: String, part: Part) -> usize {
    let lines = raw.lines();
    let mut sum: usize = 0;
    for line in lines {
        let num_batteries = match part {
            Part::Part1 => 2,
            Part::Part2 => 12,
        };
        let largest = largest_joltage(line, num_batteries);
        sum += largest;
    }
    return sum;
}

fn largest_joltage(line: &str, num_batteries: usize) -> usize {
    let mut total: usize = 0;
    let mut starting_ix: isize = -1;

    for i in (0..num_batteries).rev() {
        let x = &line[..(line.len() - i)];
        let (ix, largest) = x
            .char_indices()
            .skip((starting_ix + 1) as usize)
            .map(|(i, v)| (i, v.to_digit(10).expect("expected digit")))
            .max_by(|x, y| {
                let value_order = x.1.cmp(&y.1);
                if value_order == Ordering::Equal {
                    return y.0.cmp(&x.0);
                };
                value_order
            })
            .expect("expected a maximum value");
        total = (total * 10) + (largest as usize);
        starting_ix = ix as isize;
    }

    return total;
}

#[test]
fn test_example_part_1() {
    let raw = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
    let res = run(raw.to_string(), Part::Part1);
    assert_eq!(res, 357);
}

#[test]
fn test_example_part_2() {
    let raw = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
    let res = run(raw.to_string(), Part::Part2);
    assert_eq!(res, 3121910778619);
}
#[test]
fn test_largest_joltage() {
    assert_eq!(largest_joltage("111119", 2), 19);
    assert_eq!(largest_joltage("911111", 2), 91);
    assert_eq!(largest_joltage("911119", 2), 99);
    assert_eq!(largest_joltage("211119", 2), 29);
    assert_eq!(largest_joltage("011119", 2), 19);
    assert_eq!(largest_joltage("911119", 2), 99);
    assert_eq!(largest_joltage("98173", 3), 987);
    assert_eq!(largest_joltage("991", 2), 99);
}

