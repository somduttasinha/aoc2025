use std::collections::HashSet;

use crate::common;
use crate::common::Part;

type Range = (usize, usize);

pub fn day5(part: Part) {
    let path = "src/resources/day5.txt";
    let raw = common::read(path);
    let res = run(raw, part);
    println!("res: {}", res);
}

fn run(raw: String, part: Part) -> usize {
    let split = raw.find("\n\n").expect("expected separator"); // find the empty line

    let mut range = raw[..split]
        .lines()
        .map(|l| {
            (
                l.split('-').nth(0).unwrap().parse::<usize>().unwrap(),
                l.split('-').nth(1).unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<(usize, usize)>>();

    range.sort_by(|x, y| x.0.cmp(&y.0));

    let mut ingredients = raw[split..]
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    ingredients.sort();

    match part {
        Part::Part1 => get_fresh_ingredients(&range, &ingredients).len(),
        Part::Part2 => get_all_fresh_ingredients(&range),
    }
}

fn get_all_fresh_ingredients(ranges: &Vec<Range>) -> usize {
    // let raw = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";
    // [3, 5], [10, 14], [12, 18], [16, 20]
    // [3, 5]
    // last = [3, 5]
    // range = [10, 14]
    //
    // [3, 5], [10, 14]
    // last = [10, 14] -> [10, 18]
    // range = [12, 18]
    //
    // [3, 5], [10, 18]
    // last = [10, 18] -> [10, 20]
    // range = [16, 20]
    // [3, 5], [10, 20]
    let mut effective_ranges: Vec<Range> = Vec::new();

    for range in ranges {
        if let Some(last) = effective_ranges.last_mut() {
            if range.0 <= last.1 + 1 {
                last.1 = last.1.max(range.1);
                continue;
            }
        }
        effective_ranges.push(range.clone());
    }

    let mut total = 0;

    for range in effective_ranges {
        total += range.1 - range.0 + 1;
    }
    total
}

fn get_fresh_ingredients(ranges: &Vec<Range>, ingredients: &[usize]) -> Vec<usize> {
    let mut fresh_ingredients = Vec::<usize>::new();

    for ingredient in ingredients {
        for range in ranges {
            if *ingredient < range.0 {
                continue;
            }

            if *ingredient >= range.0 && *ingredient <= range.1 {
                fresh_ingredients.push(*ingredient);
                break;
            }
        }
    }

    return fresh_ingredients;
}

#[test]
fn test_example_part_1() {
    let raw = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";
    let res = run(raw.to_string(), Part::Part1);
    assert_eq!(res, 3)
}

#[test]
fn test_example_part_2() {
    let raw = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";
    let res = run(raw.to_string(), Part::Part2);
    assert_eq!(res, 14)
}

#[test]
fn test_connecting_ranges() {
    let raw = "3-5\n10-14\n15-20\n\n14";
    let res = run(raw.to_string(), Part::Part1);

    assert_eq!(res, 1);

    let raw = "3-5\n10-13\n15-20\n\n14";
    let res = run(raw.to_string(), Part::Part1);
    assert_eq!(res, 0);

    let raw = "3-16\n10-13\n15-20\n\n14";
    let res = run(raw.to_string(), Part::Part1);
    assert_eq!(res, 1);
}
