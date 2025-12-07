use std::collections::VecDeque;

use crate::common::{self, Part};

pub fn day2(part: Part) {
    let path = "src/resources/day2.txt";
    let raw = common::read(path);
    let raw = raw.strip_suffix('\n').expect("\\n at the end").to_string();

    let res = run(raw, part);
    println!("res: {}", res);
}

fn run(raw: String, part: Part) -> usize {
    // delimit by ,
    let ranges = raw.split(",");

    let mut sum = 0;
    for range in ranges {
        let splits = range.split_at(range.find('-').expect("expected a -"));
        let low = splits.0;
        let high = splits.1.strip_prefix("-").expect("expected leading -");

        let low: usize = str::parse(low).expect("Expected number");
        let high: usize = str::parse(high).expect("Expected number");

        for int in low..high + 1 {
            let is_valid = match part {
                Part::Part1 => is_valid(int),
                Part::Part2 => is_valid_part_2(int),
            };
            if !is_valid {
                sum += int;
            }
        }
    }
    sum
}

fn is_valid(number: usize) -> bool {
    let num_str = number.to_string();
    if num_str.len() % 2 == 1 {
        return true;
    }
    let chars = num_str.chars();

    let mut buf: VecDeque<char> = VecDeque::new();
    let mut cur_idx: usize = 0;
    let half_way = num_str.len() / 2;
    for char in chars {
        if cur_idx >= half_way {
            let top = buf.pop_back();
            if top.is_none() {
                return false;
            }

            if char != top.unwrap() {
                return true;
            }
        } else {
            buf.push_front(char);
        }
        cur_idx += 1;
    }
    if buf.len() > 0 {
        return true;
    }

    false
}

fn check_pattern_repeats(pattern: &str, string: &String) -> bool {
    if string.len() % pattern.len() != 0 {
        return false;
    }
    for (i, char) in string.char_indices() {
        let j = i % (pattern.len());
        let jth = pattern.chars().nth(j).unwrap();
        if jth != char {
            return false;
        }
    }
    true
}

fn is_valid_part_2(number: usize) -> bool {
    let num_str = number.to_string();

    if num_str.len() <= 1 {
        return true;
    }

    for size in 1..(num_str.len() / 2) + 1 {
        let pattern = &num_str[0..size];
        if check_pattern_repeats(pattern, &num_str) {
            return false;
        }
    }
    true
}

#[test]
fn test_validity_check() {
    assert!(is_valid(1234));
    assert!(!is_valid(1212));
    let raw = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    let res = run(raw.to_string(), Part::Part1);
    assert_eq!(res, 1227775554);
}

#[test]
fn test_repeating_patterns() {
    assert!(check_pattern_repeats("1", &"11".to_string()));
    assert!(check_pattern_repeats("12", &"1212".to_string()));
    assert!(!check_pattern_repeats("12", &"12121".to_string()));
    assert!(!check_pattern_repeats("12", &"12122".to_string()));
}

#[test]
fn test_validity_part_2() {
    assert!(!is_valid_part_2(11));
    assert!(is_valid_part_2(121));
    assert!(!is_valid_part_2(1111));
    assert!(!is_valid_part_2(99));
}

#[test]
fn test_part2_example() {
    let raw = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    let res = run(raw.to_string(), Part::Part2);
    assert_eq!(res, 4174379265)
}
