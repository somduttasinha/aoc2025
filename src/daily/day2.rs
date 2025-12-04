use std::collections::VecDeque;

use crate::daily::common;

pub fn day2() {
    let path = "src/resources/day2.txt";
    let raw = common::read(path);
    let raw = raw.strip_suffix('\n').expect("\\n at the end");
    let res = run(raw.to_string());
    println!("res: {}", res);
}

fn run(raw: String) -> usize {
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
            if !is_valid(int) {
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

#[test]
fn test_validity_check() {
    assert!(is_valid(1234));
    assert!(!is_valid(1212));
    let raw = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    let res = run(raw.to_string());
    assert_eq!(res, 1227775554);
}
